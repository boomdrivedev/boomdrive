//! A fake of GitHub's API on the loopback address (design Decision 9): a
//! listener on `127.0.0.1` of the test's own, answering one HTTP/1.1
//! request for each connection on a thread of its own. It finds the App's
//! installation on the repositories it was told of, mints a fresh token
//! each time it is asked for one, gives an answer it was told to in place
//! of either, or holds every connection silent. It verifies each JWT
//! against the public halves of the tests' keys, and keeps each request.
//! No test reaches the network.
//!
//! The tests' keys are made by `rsa` once for each test binary and
//! written at mode 0600 into the scratch home, since no key is committed
//! (core.4).

#![allow(dead_code)]

use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use rsa::pkcs1::EncodeRsaPrivateKey;
use rsa::pkcs8::{EncodePublicKey, LineEnding};
use rsa::RsaPrivateKey;
use serde_json::{json, Value};
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

/// One of the tests' keys: `App` the one a test's `[forge]` names, and
/// `Other` a second, for an App whose key changed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Key {
    App,
    Other,
}

impl Key {
    /// The key, made once for the test binary.
    pub fn private(self) -> &'static RsaPrivateKey {
        static APP: OnceLock<RsaPrivateKey> = OnceLock::new();
        static OTHER: OnceLock<RsaPrivateKey> = OnceLock::new();
        let made = match self {
            Key::App => &APP,
            Key::Other => &OTHER,
        };
        made.get_or_init(|| {
            RsaPrivateKey::new(&mut rsa::rand_core::OsRng, 2048).expect("a test's key")
        })
    }

    /// The key as PEM, PKCS#1, as GitHub gives an App's key.
    pub fn pem(self) -> String {
        self.private()
            .to_pkcs1_pem(LineEnding::LF)
            .expect("the key as PEM")
            .to_string()
    }

    /// Writes the key as PEM, at `mode`, at `~/.config/boomdrive/<name>`
    /// under `home`, and gives its path.
    pub fn write(self, home: &Path, mode: u32) -> PathBuf {
        let name = match self {
            Key::App => "app.pem",
            Key::Other => "other.pem",
        };
        let file = home.join(".config/boomdrive").join(name);
        std::fs::create_dir_all(file.parent().expect("the key's folder"))
            .expect("the key's folder");
        std::fs::write(&file, self.pem()).expect("the key written");
        std::fs::set_permissions(&file, std::fs::Permissions::from_mode(mode))
            .expect("the key's mode");
        file
    }

    /// The key's public half, which verifies a JWT it signed.
    fn public(self) -> DecodingKey {
        let pem = self
            .private()
            .to_public_key()
            .to_public_key_pem(LineEnding::LF)
            .expect("the public half as PEM");
        DecodingKey::from_rsa_pem(pem.as_bytes()).expect("the public half")
    }
}

/// A request the fake was sent.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub body: String,
    /// The JWT it carried as its bearer, as sent.
    pub jwt: String,
    /// The key whose public half verified the JWT, none where none did.
    pub signed_by: Option<Key>,
    /// The JWT's `iss`, where a key verified it.
    pub iss: Option<String>,
}

#[derive(serde::Deserialize)]
struct Claims {
    iss: String,
}

/// What the fake was told, and what it was sent.
#[derive(Default)]
struct State {
    /// Each repository the App is installed on, `owner/name` in lower
    /// case, with its installation's id.
    installed: Vec<(String, u64)>,
    /// How long after its minting a token ends: an hour where none.
    lasting: Option<Duration>,
    /// An answer given in place of every call's, its status and message.
    instead: Option<(u16, String)>,
    /// Whether every connection is held open without a byte.
    silent: bool,
    requests: Vec<Request>,
    tokens: Vec<String>,
}

/// The fake, listening until the test binary ends.
pub struct GitHub {
    /// The API's address, `http://127.0.0.1:<port>`.
    pub url: String,
    state: Arc<Mutex<State>>,
}

impl GitHub {
    /// A fake on a port of its own, on which the App is installed nowhere.
    pub fn start() -> GitHub {
        let listener = TcpListener::bind("127.0.0.1:0").expect("the fake's listener");
        let port = listener.local_addr().expect("its address").port();
        let state = Arc::new(Mutex::new(State::default()));
        let serving = Arc::clone(&state);
        std::thread::spawn(move || {
            for stream in listener.incoming().flatten() {
                let state = Arc::clone(&serving);
                std::thread::spawn(move || answer(stream, &state));
            }
        });
        GitHub {
            url: format!("http://127.0.0.1:{port}"),
            state,
        }
    }

    /// Has the App installed on `address`, as the installation `id`.
    pub fn install(&self, address: &str, id: u64) -> &GitHub {
        self.state
            .lock()
            .unwrap()
            .installed
            .push((address.to_ascii_lowercase(), id));
        self
    }

    /// Has each token it mints from now on end `lasting` after its
    /// minting.
    pub fn lasting(&self, lasting: Duration) -> &GitHub {
        self.state.lock().unwrap().lasting = Some(lasting);
        self
    }

    /// Answers every call with `status` and GitHub's `message`.
    pub fn answering(&self, status: u16, message: &str) -> &GitHub {
        self.state.lock().unwrap().instead = Some((status, message.to_string()));
        self
    }

    /// Holds every connection open without a byte.
    pub fn silent(&self) -> &GitHub {
        self.state.lock().unwrap().silent = true;
        self
    }

    /// Each request it was sent, in order.
    pub fn requests(&self) -> Vec<Request> {
        self.state.lock().unwrap().requests.clone()
    }

    /// Each request it was sent, as `<method> <path>`.
    pub fn calls(&self) -> Vec<String> {
        self.requests()
            .iter()
            .map(|request| format!("{} {}", request.method, request.path))
            .collect()
    }

    /// Each token it minted, in order.
    pub fn tokens(&self) -> Vec<String> {
        self.state.lock().unwrap().tokens.clone()
    }
}

/// Reads one request from `stream` and answers it, as `state` has it.
fn answer(stream: TcpStream, state: &Mutex<State>) {
    let mut reader = BufReader::new(stream.try_clone().expect("the stream"));
    if state.lock().unwrap().silent {
        // Held without a byte until the caller lets go.
        let _ = std::io::copy(&mut reader, &mut std::io::sink());
        return;
    }
    let mut line = String::new();
    if reader.read_line(&mut line).unwrap_or(0) == 0 {
        return;
    }
    let mut words = line.split_whitespace();
    let method = words.next().unwrap_or_default().to_string();
    let path = words.next().unwrap_or_default().to_string();
    let mut length = 0;
    let mut jwt = String::new();
    loop {
        let mut header = String::new();
        if reader.read_line(&mut header).unwrap_or(0) == 0 || header.trim().is_empty() {
            break;
        }
        let Some((name, value)) = header.split_once(':') else {
            continue;
        };
        let value = value.trim();
        if name.eq_ignore_ascii_case("content-length") {
            length = value.parse().unwrap_or(0);
        } else if name.eq_ignore_ascii_case("authorization") {
            jwt = value.strip_prefix("Bearer ").unwrap_or(value).to_string();
        }
    }
    let mut body = vec![0; length];
    let _ = reader.read_exact(&mut body);
    let body = String::from_utf8_lossy(&body).into_owned();

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_required_spec_claims(&["exp", "iat", "iss"]);
    let verified = [Key::App, Key::Other].into_iter().find_map(|key| {
        let claims = jsonwebtoken::decode::<Claims>(&jwt, &key.public(), &validation).ok()?;
        Some((key, claims.claims.iss))
    });

    let mut state = state.lock().unwrap();
    state.requests.push(Request {
        method: method.clone(),
        path: path.clone(),
        body,
        jwt,
        signed_by: verified.as_ref().map(|(key, _)| *key),
        iss: verified.as_ref().map(|(_, iss)| iss.clone()),
    });
    let (status, answer) = if let Some((status, message)) = &state.instead {
        (*status, json!({ "message": message }))
    } else if verified.is_none() {
        (
            401,
            json!({ "message": "A JSON web token could not be decoded" }),
        )
    } else {
        route(&mut state, &method, &path)
    };
    drop(state);
    reply(stream, status, &answer);
}

/// GitHub's answer to `method` at `path`.
fn route(state: &mut State, method: &str, path: &str) -> (u16, Value) {
    let not_found = (404, json!({ "message": "Not Found" }));
    let segments: Vec<&str> = path.trim_start_matches('/').split('/').collect();
    match (method, segments.as_slice()) {
        ("GET", ["repos", owner, name, "installation"]) => {
            let address = format!("{owner}/{name}").to_ascii_lowercase();
            match state.installed.iter().find(|(kept, _)| *kept == address) {
                Some((_, id)) => (200, json!({ "id": id })),
                None => not_found,
            }
        }
        ("POST", ["app", "installations", id, "access_tokens"]) => {
            if !state
                .installed
                .iter()
                .any(|(_, installed)| installed.to_string() == *id)
            {
                return not_found;
            }
            let token = format!(
                "ghs_fake{:04}x{}",
                state.tokens.len() + 1,
                OffsetDateTime::now_utc().unix_timestamp_nanos()
            );
            state.tokens.push(token.clone());
            let lasting = state.lasting.unwrap_or(Duration::from_secs(60 * 60));
            let ends = (OffsetDateTime::now_utc() + lasting)
                .replace_nanosecond(0)
                .expect("a whole second")
                .format(&Rfc3339)
                .expect("an end in RFC 3339");
            (201, json!({ "token": token, "expires_at": ends }))
        }
        _ => not_found,
    }
}

/// Writes `answer` with `status` to `stream`, and closes it.
fn reply(mut stream: TcpStream, status: u16, answer: &Value) {
    let body = answer.to_string();
    let reason = match status {
        200 => "OK",
        201 => "Created",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        _ => "Answered",
    };
    let _ = write!(
        stream,
        "HTTP/1.1 {status} {reason}\r\nContent-Type: application/json\r\n\
         Content-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
    let _ = stream.flush();
}
