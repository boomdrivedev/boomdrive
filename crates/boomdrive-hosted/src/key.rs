//! The host's key (design Decision 3; core.31, core.32): each partition's
//! Ed25519 key pair, made on the host from the operating system's
//! randomness, its private key never leaving it; the stores it is kept in,
//! the macOS keychain or a user's credential encrypted by `systemd-creds`,
//! and never a file in the clear; and the contexts it signs, as the
//! protocol's README gives their bytes.

use crate::protocol::v2::SUBPROTOCOL;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
use keyring_core::{CredentialStore, Error};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::io::Write;
use std::os::unix::fs::DirBuilderExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use zeroize::Zeroizing;

/// The bytes of a key's secret half, and of its public half.
const KEY_LENGTH: usize = 32;

/// The bytes of a connection's nonce.
pub const NONCE_LENGTH: usize = 16;

/// What a partition's key is kept under: its label, and the host of its
/// control plane's `url`, so a host enrolled with two control planes keeps
/// a key for each and never presents one to the other.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyName {
    label: String,
    plane: String,
}

impl KeyName {
    pub fn new(label: &str, plane: &str) -> KeyName {
        KeyName {
            label: label.to_string(),
            plane: plane.to_string(),
        }
    }

    /// The partition's label.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// The account of its keychain item, `<label>@<host>`.
    pub fn account(&self) -> String {
        format!("{}@{}", self.label, self.plane)
    }

    /// The name of its systemd credential, `boomdrive.<label>.<host>`.
    pub fn credential(&self) -> String {
        format!("boomdrive.{}.{}", self.label, self.plane)
    }
}

/// A credential store a partition's key is kept in, read again at every
/// connection attempt and never cached across one.
pub trait Store: Send + Sync {
    /// The key kept as `name`'s, none where none is, or why it could not
    /// be read.
    fn read(&self, name: &KeyName) -> Result<Option<SigningKey>, String>;
    /// Keeps `key` as `name`'s, replacing one kept there, or says why it
    /// could not.
    fn write(&self, name: &KeyName, key: &SigningKey) -> Result<(), String>;
}

/// A store shared, as every partition's connection shares the host's.
impl<S: Store + ?Sized> Store for Arc<S> {
    fn read(&self, name: &KeyName) -> Result<Option<SigningKey>, String> {
        (**self).read(name)
    }

    fn write(&self, name: &KeyName, key: &SigningKey) -> Result<(), String> {
        (**self).write(name, key)
    }
}

/// The host's own store, on macOS the user's login keychain. Nothing
/// falls back to a file (core.32).
#[cfg(target_os = "macos")]
pub fn store_for_host() -> Result<Box<dyn Store>, String> {
    Ok(Box::new(Keychain::login()?))
}

/// The host's own store, on Linux the user's `systemd-creds` store, 256
/// or later; or what the host lacks, which is the host's to fix. Nothing
/// falls back to a file (core.32).
#[cfg(target_os = "linux")]
pub fn store_for_host() -> Result<Box<dyn Store>, String> {
    Ok(Box::new(SystemdCreds::for_user()?))
}

/// A host with neither store, which is the host's to fix: nothing falls
/// back to a file (core.32).
#[cfg(not(any(target_os = "macos", target_os = "linux")))]
pub fn store_for_host() -> Result<Box<dyn Store>, String> {
    Err(
        "this host has neither the macOS keychain nor `systemd-creds`, the stores a \
         partition's key is kept in, and its key is kept in no file"
            .to_string(),
    )
}

/// The macOS keychain's store, through keyring's: each key a generic
/// password whose service is [`Keychain::SERVICE`] and whose account is
/// its [`KeyName::account`].
pub struct Keychain {
    store: Arc<CredentialStore>,
}

impl Keychain {
    /// The service each key's keychain item is under.
    pub const SERVICE: &'static str = "boomdrive";

    /// The user's login keychain, by keyring v4's macOS store.
    #[cfg(target_os = "macos")]
    pub fn login() -> Result<Keychain, String> {
        let store = apple_native_keyring_store::keychain::Store::new()
            .map_err(|e| format!("the macOS keychain could not be opened: {}", words(&e)))?;
        Ok(Keychain::over(store))
    }

    /// Keys kept through `store`, one of keyring's: the macOS keychain's,
    /// or keyring's mock in a test.
    pub fn over(store: Arc<CredentialStore>) -> Keychain {
        Keychain { store }
    }
}

impl Store for Keychain {
    fn read(&self, name: &KeyName) -> Result<Option<SigningKey>, String> {
        let entry = self
            .store
            .build(Keychain::SERVICE, &name.account(), None)
            .map_err(|e| keychains(name, &e))?;
        match entry.get_secret() {
            Ok(bytes) => signing_key(Zeroizing::new(bytes))
                .map(Some)
                .map_err(|words| format!("the keychain's item for `{}` {words}", name.label)),
            Err(Error::NoEntry) => Ok(None),
            Err(e) => Err(keychains(name, &e)),
        }
    }

    fn write(&self, name: &KeyName, key: &SigningKey) -> Result<(), String> {
        let entry = self
            .store
            .build(Keychain::SERVICE, &name.account(), None)
            .map_err(|e| keychains(name, &e))?;
        entry
            .set_secret(key.as_bytes())
            .map_err(|e| keychains(name, &e))
    }
}

/// The keychain's words for `name`'s item.
fn keychains(name: &KeyName, error: &Error) -> String {
    format!(
        "the keychain could not keep `{}`'s key: {}",
        name.label,
        words(error)
    )
}

/// keyring's words for `error`, holding none of a secret's bytes.
fn words(error: &Error) -> String {
    match error {
        Error::BadEncoding(_) | Error::BadDataFormat(_, _) => {
            "its item is not a key the runner made".to_string()
        }
        error => error.to_string(),
    }
}

/// The store of a user's credentials encrypted by `systemd-creds`, 256 or
/// later, which binds each to the host's own key and to its TPM where it
/// has one (record 0006). The key's bytes reach the program on standard
/// input and come back on its standard output, never in an argument, and
/// the program writes the credential itself.
#[derive(Clone, Debug)]
pub struct SystemdCreds {
    program: PathBuf,
    credstore_dir: PathBuf,
}

impl SystemdCreds {
    /// The least systemd whose `systemd-creds` keeps a user's credential.
    pub const LEAST: u32 = 256;

    /// The user's: `systemd-creds` from `PATH`, keeping each credential in
    /// the user's encrypted credential store,
    /// `$XDG_CONFIG_HOME/credstore.encrypted`, which the user's service
    /// manager searches.
    pub fn for_user() -> Result<SystemdCreds, String> {
        let config_dir = match (
            std::env::var_os("XDG_CONFIG_HOME"),
            std::env::var_os("HOME"),
        ) {
            (Some(dir), _) if !dir.is_empty() => PathBuf::from(dir),
            (_, Some(home)) if !home.is_empty() => PathBuf::from(home).join(".config"),
            _ => {
                return Err(
                    "neither `XDG_CONFIG_HOME` nor `HOME` is set, so the user's \
                            credential store is not found"
                        .to_string(),
                )
            }
        };
        SystemdCreds::open(
            PathBuf::from("systemd-creds"),
            config_dir.join("credstore.encrypted"),
        )
    }

    /// `program`, keeping each credential under `credstore_dir`, once its
    /// own `--version` says it is systemd [`SystemdCreds::LEAST`] or
    /// later; or what the host lacks.
    pub fn open(program: PathBuf, credstore_dir: PathBuf) -> Result<SystemdCreds, String> {
        let output = Command::new(&program)
            .arg("--version")
            .stdin(Stdio::null())
            .output()
            .map_err(|e| {
                format!(
                    "`{}` could not be run ({e}): this host keeps each partition's key with \
                     `systemd-creds` of systemd {} or later, or in the macOS keychain",
                    program.display(),
                    SystemdCreds::LEAST
                )
            })?;
        let text = String::from_utf8_lossy(&output.stdout);
        let version = text
            .lines()
            .next()
            .and_then(|line| line.strip_prefix("systemd "))
            .and_then(|rest| rest.split_whitespace().next())
            .and_then(|number| number.parse::<u32>().ok());
        match version {
            Some(version) if version >= SystemdCreds::LEAST => Ok(SystemdCreds {
                program,
                credstore_dir,
            }),
            Some(version) => Err(format!(
                "`systemd-creds` is systemd {version}'s: a partition's key is kept as a user's \
                 credential, which takes systemd {} or later",
                SystemdCreds::LEAST
            )),
            None => Err(format!(
                "`{} --version` names no systemd version: a partition's key is kept as a \
                 user's credential, which takes systemd {} or later",
                program.display(),
                SystemdCreds::LEAST
            )),
        }
    }

    /// Where `name`'s credential is.
    fn credential_file(&self, name: &KeyName) -> PathBuf {
        self.credstore_dir.join(name.credential())
    }

    /// `systemd-creds` run on `arguments`, handed `input` on standard
    /// input, its standard output, or its words where it failed.
    fn run(&self, arguments: &[&str], input: &[u8]) -> Result<Zeroizing<Vec<u8>>, String> {
        let mut child = Command::new(&self.program)
            .args(arguments)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("`{}` could not be run: {e}", self.program.display()))?;
        let written = child
            .stdin
            .take()
            .map(|mut stdin| stdin.write_all(input))
            .unwrap_or(Ok(()));
        let output = child
            .wait_with_output()
            .map_err(|e| format!("`{}` could not be run: {e}", self.program.display()))?;
        let stdout = Zeroizing::new(output.stdout);
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!(
                "`systemd-creds {}` failed: {}",
                arguments[0],
                stderr.trim()
            ));
        }
        written.map_err(|e| format!("`systemd-creds {}` took no key: {e}", arguments[0]))?;
        Ok(stdout)
    }
}

impl Store for SystemdCreds {
    fn read(&self, name: &KeyName) -> Result<Option<SigningKey>, String> {
        let file = self.credential_file(name);
        if !file.exists() {
            return Ok(None);
        }
        let named = format!("--name={}", name.credential());
        let bytes = self
            .run(&["decrypt", "--user", &named, &path(&file)?, "-"], &[])
            .map_err(|words| format!("`{}`'s key could not be read: {words}", name.label))?;
        signing_key(bytes)
            .map(Some)
            .map_err(|words| format!("`{}`'s credential {words}", name.label))
    }

    fn write(&self, name: &KeyName, key: &SigningKey) -> Result<(), String> {
        // The user's alone, where the runner makes it.
        std::fs::DirBuilder::new()
            .recursive(true)
            .mode(0o700)
            .create(&self.credstore_dir)
            .map_err(|e| {
                format!(
                    "the user's credential store, {}, could not be made: {e}",
                    self.credstore_dir.display()
                )
            })?;
        let named = format!("--name={}", name.credential());
        let file = self.credential_file(name);
        self.run(
            &["encrypt", "--user", &named, "-", &path(&file)?],
            key.as_bytes(),
        )
        .map(|_| ())
        .map_err(|words| format!("`{}`'s key could not be kept: {words}", name.label))
    }
}

/// `file` as an argument.
fn path(file: &Path) -> Result<String, String> {
    file.to_str()
        .map(str::to_string)
        .ok_or_else(|| format!("{} is not UTF-8", file.display()))
}

/// The keys of a test, kept in memory alone.
#[derive(Default)]
pub struct Memory {
    keys: Mutex<BTreeMap<String, Zeroizing<[u8; KEY_LENGTH]>>>,
}

impl Memory {
    /// The account of each key kept, `<label>@<host>`.
    pub fn accounts(&self) -> Vec<String> {
        let keys = self.keys.lock().unwrap_or_else(|e| e.into_inner());
        keys.keys().cloned().collect()
    }
}

impl Store for Memory {
    fn read(&self, name: &KeyName) -> Result<Option<SigningKey>, String> {
        let keys = self.keys.lock().unwrap_or_else(|e| e.into_inner());
        Ok(keys
            .get(&name.account())
            .map(|bytes| SigningKey::from_bytes(bytes)))
    }

    fn write(&self, name: &KeyName, key: &SigningKey) -> Result<(), String> {
        let mut keys = self.keys.lock().unwrap_or_else(|e| e.into_inner());
        keys.insert(name.account(), Zeroizing::new(key.to_bytes()));
        Ok(())
    }
}

/// The key whose secret half is `bytes`, or why they are none.
fn signing_key(bytes: Zeroizing<Vec<u8>>) -> Result<SigningKey, String> {
    let secret: Zeroizing<[u8; KEY_LENGTH]> = Zeroizing::new(
        bytes
            .as_slice()
            .try_into()
            .map_err(|_| format!("holds {} bytes, not a key's {KEY_LENGTH}", bytes.len()))?,
    );
    Ok(SigningKey::from_bytes(&secret))
}

/// A new key pair, made from the operating system's randomness.
pub fn made() -> Result<SigningKey, String> {
    let mut secret = Zeroizing::new([0u8; KEY_LENGTH]);
    getrandom::fill(secret.as_mut())
        .map_err(|e| format!("the host's randomness could not be read: {e}"))?;
    Ok(SigningKey::from_bytes(&secret))
}

/// A new nonce for a connection, made from the operating system's
/// randomness.
pub fn nonce() -> Result<[u8; NONCE_LENGTH], String> {
    let mut nonce = [0u8; NONCE_LENGTH];
    getrandom::fill(&mut nonce)
        .map_err(|e| format!("the host's randomness could not be read: {e}"))?;
    Ok(nonce)
}

/// `bytes` in base64url without padding, as version 2 carries bytes.
pub fn base64url(bytes: &[u8]) -> String {
    URL_SAFE_NO_PAD.encode(bytes)
}

/// The context `name` over `fields`: the UTF-8 bytes of
/// `boomdrive.v2`, a space and `name`, then each field after a NUL
/// byte, with nothing after the last (the protocol's README, *The
/// signatures*).
pub fn context(name: &str, fields: &[&[u8]]) -> Vec<u8> {
    let mut bytes = format!("{SUBPROTOCOL} {name}").into_bytes();
    for field in fields {
        bytes.push(0);
        bytes.extend_from_slice(field);
    }
    bytes
}

/// The digest an enrollment binds its token by: `sha256:` and the
/// lowercase hex of the SHA-256 digest of the token's bytes.
pub fn token_digest(token: &str) -> String {
    let digest = Sha256::digest(token.as_bytes());
    let hex: String = digest.iter().map(|byte| format!("{byte:02x}")).collect();
    format!("sha256:{hex}")
}

/// The enrollment's context: the control plane's `origin`, the token's
/// digest and the public key's bytes.
pub fn enrollment_context(origin: &str, token: &str, key: &VerifyingKey) -> Vec<u8> {
    context(
        "enroll",
        &[
            origin.as_bytes(),
            token_digest(token).as_bytes(),
            key.as_bytes(),
        ],
    )
}

/// A connection's context: the control plane's `origin`, the public
/// key's bytes, `time`'s decimal digits and the nonce's bytes.
pub fn connection_context(
    origin: &str,
    key: &VerifyingKey,
    time: u64,
    nonce: &[u8; NONCE_LENGTH],
) -> Vec<u8> {
    context(
        "connect",
        &[
            origin.as_bytes(),
            key.as_bytes(),
            time.to_string().as_bytes(),
            nonce,
        ],
    )
}

/// `key`'s signature over `context`, in base64url.
pub fn signature(key: &SigningKey, context: &[u8]) -> String {
    base64url(&key.sign(context).to_bytes())
}

/// The `Authorization` header's value for one connection attempt to the
/// control plane at `origin`, at `time` in Unix seconds with `nonce`:
/// `BoomdriveHost <key>.<time>.<nonce>.<signature>` (design Decision 5).
pub fn statement(key: &SigningKey, origin: &str, time: u64, nonce: &[u8; NONCE_LENGTH]) -> String {
    let public = key.verifying_key();
    let signed = signature(key, &connection_context(origin, &public, time, nonce));
    format!(
        "BoomdriveHost {}.{time}.{}.{signed}",
        base64url(public.as_bytes()),
        base64url(nonce)
    )
}
