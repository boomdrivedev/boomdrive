//! The Boomdrive App's tokens (record 0007 as amended on 2026-11-09 and
//! 2026-11-11; core.28, core.33): on a host whose control plane does not
//! hold the App, the runner mints a short-lived installation token for a
//! served repository from the App's key its configuration's `[forge]`
//! names. A mint reads the key, signs a JWT with it, RS256, finds the
//! App's installation on the repository and buys a token from it for that
//! repository alone, by two calls to GitHub's API, each stopped at
//! [`CALL_FOR`] (design Decision 2). GitHub's word that the App has no
//! installation there is an answer of its own.
//!
//! No token is kept but in a [`Forge`]'s memory, for the process that
//! minted it: nothing is written to a file, the data directory, the host's
//! credential store or a log, and no refusal's words hold a token, the JWT
//! or the key (core.4, core.35).

use crate::config::{Address, ForgeApp};
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use serde::Serialize;
use serde_json::Value;
use std::fmt;
use std::io::Read;
use std::os::unix::fs::PermissionsExt;
use std::time::Duration;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;
use ureq::tls::{RootCerts, TlsConfig};

/// GitHub's API, which [`Forge::new`] calls.
pub const API: &str = "https://api.github.com";

/// How long each of a mint's calls to GitHub's API may take, so a
/// `boomdrive forge-token get` answers within the thirty seconds sluice
/// gives a credential command (sluice record 0108), whatever GitHub does.
pub const CALL_FOR: Duration = Duration::from_secs(10);

/// How long before its end a kept token is minted again, as sluice's
/// `REFRESH` has it.
pub const RENEW_BEFORE: Duration = Duration::from_secs(5 * 60);

/// How far back a JWT is dated, for a host clock a little ahead.
const JWT_SKEW: i64 = 60;

/// How long after now a JWT lasts, within GitHub's ten minutes.
const JWT_LIFE: i64 = 9 * 60;

/// The most of GitHub's answer a mint reads.
const ANSWER_LIMIT: u64 = 64 * 1024;

/// An installation token for one repository, and its end. Its `Debug`
/// names its end and never its secret (core.4).
#[derive(Clone, PartialEq, Eq)]
pub struct Token {
    secret: String,
    ends: OffsetDateTime,
}

impl Token {
    /// The token's secret, which only a credential's answer is given.
    pub fn secret(&self) -> &str {
        &self.secret
    }

    /// When GitHub says the token ends.
    pub fn ends(&self) -> OffsetDateTime {
        self.ends
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ends = self
            .ends
            .format(&Rfc3339)
            .unwrap_or_else(|_| self.ends.unix_timestamp().to_string());
        f.debug_struct("Token")
            .field("ends", &ends)
            .finish_non_exhaustive()
    }
}

/// What a mint gives: a token, or GitHub's word that the App has no
/// installation on the repository.
#[derive(Debug, PartialEq, Eq)]
pub enum Minted {
    Token(Token),
    NotInstalled,
}

/// A mint refused: words saying why, which hold no token, JWT or key, and
/// whether the host must fix its key before a mint can go (core.39's exit
/// 2), or GitHub refused it or could not be reached (exit 1).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Refused {
    pub words: String,
    pub host_to_fix: bool,
}

impl Refused {
    /// A refusal the host must fix.
    fn host(words: String) -> Refused {
        Refused {
            words,
            host_to_fix: true,
        }
    }

    /// A refusal of GitHub's, or GitHub out of reach.
    fn github(words: String) -> Refused {
        Refused {
            words,
            host_to_fix: false,
        }
    }
}

/// What mints the App's tokens: the API it calls and how long each call
/// may take.
pub struct Forge {
    api: String,
    call_for: Duration,
}

impl Default for Forge {
    fn default() -> Forge {
        Forge::new()
    }
}

impl Forge {
    /// A forge calling GitHub's API, each call stopped at [`CALL_FOR`].
    pub fn new() -> Forge {
        Forge::at(API)
    }

    /// A forge calling the API at `api`, a test's fake, each call stopped
    /// at [`CALL_FOR`]. No configuration key, environment variable or flag
    /// sets it (core.17).
    pub fn at(api: &str) -> Forge {
        Forge {
            api: api.trim_end_matches('/').to_string(),
            call_for: CALL_FOR,
        }
    }

    /// `self`, each call stopped at `limit`, a test's shorter limit.
    pub fn calling_for(self, limit: Duration) -> Forge {
        Forge {
            call_for: limit,
            ..self
        }
    }

    /// Mints a token for `address` from the App `app`'s key (design
    /// Decision 2): the key read and the JWT signed, then GitHub asked for
    /// the App's installation on `address`, whose 404 gives
    /// [`Minted::NotInstalled`], and a token bought from it for `address`
    /// alone. Every other answer, and a call that could not be made or was
    /// stopped at its limit, refuses the mint, naming the call.
    pub fn mint(&self, app: &ForgeApp, address: &Address) -> Result<Minted, Refused> {
        let key = key(app)?;
        let jwt = jwt(app, &key, OffsetDateTime::now_utc())?;

        let finding = format!("finding app {}'s installation on {address}", app.app_id);
        let path = format!("/repos/{}/{}/installation", address.owner(), address.name());
        let installation = match self.call(&path, &jwt, None, &finding)? {
            (200, answer) => answer["id"].as_u64().ok_or_else(|| {
                Refused::github(format!(
                    "GitHub answered 200 to {finding} with no installation's id"
                ))
            })?,
            (404, _) => return Ok(Minted::NotInstalled),
            (status, answer) => return Err(answered(status, &answer, &finding)),
        };

        let buying = format!("buying a token for {address} from installation {installation}");
        let path = format!("/app/installations/{installation}/access_tokens");
        let body = serde_json::json!({ "repositories": [address.name()] }).to_string();
        let answer = match self.call(&path, &jwt, Some(&body), &buying)? {
            (201, answer) => answer,
            (status, answer) => return Err(answered(status, &answer, &buying)),
        };
        let secret = answer["token"].as_str().filter(|token| !token.is_empty());
        let ends = answer["expires_at"]
            .as_str()
            .and_then(|ends| OffsetDateTime::parse(ends, &Rfc3339).ok());
        match (secret, ends) {
            (Some(secret), Some(ends)) => Ok(Minted::Token(Token {
                secret: secret.to_string(),
                ends,
            })),
            _ => Err(Refused::github(format!(
                "GitHub answered 201 to {buying} with no token and its end"
            ))),
        }
    }

    /// One call to the API at `path`, with the JWT `jwt` as its bearer: a
    /// `POST` of `body` where there is one, else a `GET`; its status and
    /// its answer as JSON, null where it is none. A call that could not be
    /// made, or was stopped at the forge's limit, refuses, naming `call`.
    fn call(
        &self,
        path: &str,
        jwt: &str,
        body: Option<&str>,
        call: &str,
    ) -> Result<(u16, Value), Refused> {
        let agent: ureq::Agent = ureq::Agent::config_builder()
            .http_status_as_error(false)
            .max_redirects(0)
            .timeout_global(Some(self.call_for))
            .user_agent("boomdrive")
            .tls_config(
                TlsConfig::builder()
                    .root_certs(RootCerts::PlatformVerifier)
                    .build(),
            )
            .build()
            .into();
        let url = format!("{}{path}", self.api);
        let bearer = format!("Bearer {jwt}");
        let unreachable = |error: &dyn fmt::Display| {
            Refused::github(format!("GitHub could not be reached to {call}: {error}"))
        };
        let sent = match body {
            None => agent
                .get(&url)
                .header("Accept", "application/vnd.github+json")
                .header("X-GitHub-Api-Version", "2022-11-28")
                .header("Authorization", &bearer)
                .call(),
            Some(body) => agent
                .post(&url)
                .header("Accept", "application/vnd.github+json")
                .header("X-GitHub-Api-Version", "2022-11-28")
                .header("Authorization", &bearer)
                .content_type("application/json")
                .send(body),
        };
        let mut response = sent.map_err(|error| unreachable(&error))?;
        let status = response.status().as_u16();
        let mut text = String::new();
        response
            .body_mut()
            .as_reader()
            .take(ANSWER_LIMIT)
            .read_to_string(&mut text)
            .map_err(|error| unreachable(&error))?;
        Ok((status, serde_json::from_str(&text).unwrap_or(Value::Null)))
    }
}

/// GitHub's refusal of `call`, by its status and its answer's `message`.
fn answered(status: u16, answer: &Value, call: &str) -> Refused {
    Refused::github(match answer["message"].as_str() {
        Some(message) if !message.is_empty() => {
            format!("GitHub answered {status} to {call}: {message}")
        }
        _ => format!("GitHub answered {status} to {call}"),
    })
}

/// The App `app`'s key, read from its file for a mint: refused, as the
/// host's to fix and naming the file, where it cannot be read, where
/// anyone but its owner may read it, as sluice refuses a key, or where it
/// holds no RSA private key in PEM.
fn key(app: &ForgeApp) -> Result<EncodingKey, Refused> {
    let path = app.key_file.display();
    let unread = |error: std::io::Error| {
        Refused::host(format!("[forge] key_file {path} cannot be read: {error}"))
    };
    let mut file = std::fs::File::open(&app.key_file).map_err(unread)?;
    let mode = file.metadata().map_err(unread)?.permissions().mode() & 0o7777;
    if mode & 0o077 != 0 {
        return Err(Refused::host(format!(
            "[forge] key_file {path} has mode {mode:04o}, which lets others read it: give it \
             mode 0600 (chmod 600 {path})"
        )));
    }
    let mut pem = Vec::new();
    file.read_to_end(&mut pem).map_err(unread)?;
    EncodingKey::from_rsa_pem(&pem).map_err(|_| no_key(app))
}

/// The refusal of a key file that holds no RSA private key in PEM.
fn no_key(app: &ForgeApp) -> Refused {
    Refused::host(format!(
        "[forge] key_file {} holds no RSA private key in PEM",
        app.key_file.display()
    ))
}

/// A JWT's claims: the App's id, and when it starts and ends.
#[derive(Serialize)]
struct Claims {
    iss: String,
    iat: i64,
    exp: i64,
}

/// A JWT for the App `app`, signed RS256 with its key `key`, as sluice
/// signs one: issued a minute before `now`, for a clock a little ahead,
/// and lasting until nine minutes after it. A key the signature cannot
/// use refuses as one holding no RSA private key.
fn jwt(app: &ForgeApp, key: &EncodingKey, now: OffsetDateTime) -> Result<String, Refused> {
    let now = now.unix_timestamp();
    let claims = Claims {
        iss: app.app_id.to_string(),
        iat: now - JWT_SKEW,
        exp: now + JWT_LIFE,
    };
    jsonwebtoken::encode(&Header::new(Algorithm::RS256), &claims, key).map_err(|_| no_key(app))
}

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::{DecodingKey, Validation};
    use rsa::pkcs1::EncodeRsaPrivateKey;
    use rsa::pkcs8::{EncodePublicKey, LineEnding};
    use rsa::RsaPrivateKey;
    use std::path::{Path, PathBuf};
    use std::sync::OnceLock;

    /// The tests' key, made once, since no key is committed (core.4).
    fn the_key() -> &'static RsaPrivateKey {
        static KEY: OnceLock<RsaPrivateKey> = OnceLock::new();
        KEY.get_or_init(|| {
            RsaPrivateKey::new(&mut rsa::rand_core::OsRng, 2048).expect("the tests' key")
        })
    }

    /// The tests' key written as PEM at `mode` in `dir`.
    fn written(dir: &Path, mode: u32) -> PathBuf {
        let file = dir.join("app.pem");
        let pem = the_key()
            .to_pkcs1_pem(LineEnding::LF)
            .expect("the key as PEM");
        std::fs::write(&file, pem.as_bytes()).expect("the key written");
        std::fs::set_permissions(&file, std::fs::Permissions::from_mode(mode))
            .expect("the key's mode");
        file
    }

    fn app(key_file: PathBuf) -> ForgeApp {
        ForgeApp {
            app_id: 1,
            key_file,
        }
    }

    #[derive(serde::Deserialize)]
    struct Read {
        iss: String,
        iat: i64,
        exp: i64,
    }

    #[test]
    fn a_jwt_names_the_app_and_lasts_nine_minutes() {
        let dir = tempfile::tempdir().unwrap();
        let app = app(written(dir.path(), 0o600));
        let now = OffsetDateTime::now_utc();
        let jwt = jwt(&app, &key(&app).unwrap(), now).unwrap();

        let public = the_key()
            .to_public_key()
            .to_public_key_pem(LineEnding::LF)
            .unwrap();
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_issuer(&["1"]);
        let read = jsonwebtoken::decode::<Read>(
            &jwt,
            &DecodingKey::from_rsa_pem(public.as_bytes()).unwrap(),
            &validation,
        )
        .expect("a JWT the key's public half verifies");
        assert_eq!(read.claims.iss, "1");
        assert_eq!(read.claims.iat, now.unix_timestamp() - 60);
        assert_eq!(read.claims.exp, now.unix_timestamp() + 9 * 60);
    }

    #[test]
    fn a_key_others_may_read_is_refused() {
        let dir = tempfile::tempdir().unwrap();
        let file = written(dir.path(), 0o644);
        let refused = key(&app(file.clone())).err().expect("a refusal");
        assert_eq!(
            refused,
            Refused::host(format!(
                "[forge] key_file {path} has mode 0644, which lets others read it: give it \
                 mode 0600 (chmod 600 {path})",
                path = file.display()
            ))
        );
    }

    #[test]
    fn a_key_file_that_cannot_be_read_is_refused() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("missing.pem");
        let refused = key(&app(file.clone())).err().expect("a refusal");
        assert!(refused.host_to_fix);
        assert!(
            refused.words.starts_with(&format!(
                "[forge] key_file {} cannot be read: ",
                file.display()
            )),
            "{}",
            refused.words
        );
    }

    #[test]
    fn a_key_file_that_is_no_key_is_refused() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("app.pem");
        std::fs::write(&file, "not a key\n").unwrap();
        std::fs::set_permissions(&file, std::fs::Permissions::from_mode(0o600)).unwrap();
        assert_eq!(
            key(&app(file.clone())).err(),
            Some(Refused::host(format!(
                "[forge] key_file {} holds no RSA private key in PEM",
                file.display()
            )))
        );
    }

    #[test]
    fn a_tokens_debug_holds_no_secret() {
        let token = Token {
            secret: "ghs_n0t4r34lt0k3n".to_string(),
            ends: OffsetDateTime::from_unix_timestamp(1_800_000_000).unwrap(),
        };
        let shown = format!("{token:?} {:?}", Minted::Token(token.clone()));
        assert!(!shown.contains("ghs_n0t4r34lt0k3n"), "{shown}");
        assert!(shown.contains("2027-01-15T08:00:00Z"), "{shown}");
        assert_eq!(token.secret(), "ghs_n0t4r34lt0k3n");
    }
}
