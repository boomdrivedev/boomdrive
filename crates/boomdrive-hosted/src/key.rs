//! The host's key (design Decision 3; core.31, core.32): each partition's
//! Ed25519 key pair, made on the host from the operating system's
//! randomness, its private key never leaving it; the stores it is kept in,
//! the macOS keychain or a user's credential encrypted by `systemd-creds`,
//! and never a file in the clear; and the contexts it signs, as the
//! protocol's README gives their bytes.

use crate::protocol::Version;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
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
    Ok(Box::new(Keychain::login()))
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

/// The macOS keychain's store, through macOS's own `security` command
/// alone: each key a generic password of the user's keychain whose
/// service is [`Keychain::SERVICE`] and whose account is its
/// [`KeyName::account`]. macOS lets a program read an item without asking
/// only where the item's access names the program's partition, and it
/// gives code signed by no certificate of Apple's the partition of its own
/// build's hash, so every new build of the runner would be asked again.
/// `security` is Apple's, and its partition, `apple-tool:`, is the same at
/// every build of the runner, so an item it made, or one a person once let
/// it read, is read with no ask (record 0057). The key's bytes reach it on
/// standard input and come back on its standard output, never in an
/// argument.
#[derive(Clone, Debug)]
pub struct Keychain {
    program: PathBuf,
}

impl Keychain {
    /// The service each key's keychain item is under.
    pub const SERVICE: &'static str = "boomdrive";

    /// macOS's own `security`, by its path, so no program on `PATH` stands
    /// in for it.
    pub const PROGRAM: &'static str = "/usr/bin/security";

    /// The exit status of `security find-generic-password` where no item
    /// matches: the low byte of `errSecItemNotFound`.
    const NOT_FOUND: i32 = 44;

    /// The user's keychain, through macOS's own `security`.
    pub fn login() -> Keychain {
        Keychain::open(PathBuf::from(Keychain::PROGRAM))
    }

    /// The user's keychain, through `program`, which speaks `security`'s
    /// command line.
    pub fn open(program: PathBuf) -> Keychain {
        Keychain { program }
    }
}

impl Store for Keychain {
    fn read(&self, name: &KeyName) -> Result<Option<SigningKey>, String> {
        let output = Command::new(&self.program)
            .args([
                "find-generic-password",
                "-s",
                Keychain::SERVICE,
                "-a",
                &name.account(),
                "-w",
            ])
            .stdin(Stdio::null())
            .output()
            .map_err(|e| {
                keychains(
                    name,
                    &format!("`{}` could not be run: {e}", self.program.display()),
                )
            })?;
        let stdout = Zeroizing::new(output.stdout);
        if output.status.code() == Some(Keychain::NOT_FOUND) {
            return Ok(None);
        }
        if !output.status.success() {
            return Err(keychains(name, &failed(&output.stderr)));
        }
        item_bytes(&stdout)
            .and_then(signing_key)
            .map(Some)
            .map_err(|words| format!("the keychain's item for `{}` {words}", name.label))
    }

    fn write(&self, name: &KeyName, key: &SigningKey) -> Result<(), String> {
        let mut child = Command::new(&self.program)
            .arg("-i")
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| {
                keychains(
                    name,
                    &format!("`{}` could not be run: {e}", self.program.display()),
                )
            })?;
        let hex: Zeroizing<String> = Zeroizing::new(
            key.as_bytes()
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect(),
        );
        let line = Zeroizing::new(format!(
            "add-generic-password -U -s {} -a {} -X {}\n",
            Keychain::SERVICE,
            quoted(&name.account()),
            hex.as_str()
        ));
        let written = child
            .stdin
            .take()
            .map(|mut stdin| stdin.write_all(line.as_bytes()))
            .unwrap_or(Ok(()));
        let output = child.wait_with_output().map_err(|e| {
            keychains(
                name,
                &format!("`{}` could not be run: {e}", self.program.display()),
            )
        })?;
        // In `-i`, `security` says a command that failed as
        // `<command>: returned <status>` on standard error.
        if !output.status.success() || !output.stderr.is_empty() {
            return Err(keychains(name, &failed(&output.stderr)));
        }
        written.map_err(|e| keychains(name, &format!("`security` took no key: {e}")))
    }
}

/// The keychain's words for `name`'s item.
fn keychains(name: &KeyName, words: &str) -> String {
    format!(
        "the keychain could not keep `{}`'s key: {words}",
        name.label
    )
}

/// `security`'s words on standard error, which hold none of a key's bytes.
fn failed(stderr: &[u8]) -> String {
    let words = String::from_utf8_lossy(stderr);
    let words = words.trim();
    if words.is_empty() {
        "`security` failed and said nothing".to_string()
    } else {
        format!("`security` failed: {words}")
    }
}

/// `account` as one argument of a line of `security -i`, which splits a
/// line at spaces outside quotes and takes a backslash's next character as
/// it is.
fn quoted(account: &str) -> String {
    let mut quoted = String::from("'");
    for c in account.chars() {
        if c == '\\' || c == '\'' {
            quoted.push('\\');
        }
        quoted.push(c);
    }
    quoted.push('\'');
    quoted
}

/// The key's bytes from what `security find-generic-password -w` printed
/// of its item, without the newline after them: where each of the item's
/// bytes is printable, `security` prints the bytes themselves, and
/// otherwise each byte as two lowercase hex digits. A key's item holds 32
/// bytes, so 64 hex digits are its bytes in hex and 32 bytes are its bytes
/// themselves; anything else is no key's.
fn item_bytes(printed: &[u8]) -> Result<Zeroizing<Vec<u8>>, String> {
    let printed = printed.strip_suffix(b"\n").unwrap_or(printed);
    let digit = |byte: u8| match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        _ => None,
    };
    if printed.len() == 2 * KEY_LENGTH {
        let decoded: Option<Vec<u8>> = printed
            .chunks(2)
            .map(|pair| Some(digit(pair[0])? << 4 | digit(pair[1])?))
            .collect();
        if let Some(decoded) = decoded {
            return Ok(Zeroizing::new(decoded));
        }
    }
    if printed.len() == KEY_LENGTH {
        return Ok(Zeroizing::new(printed.to_vec()));
    }
    Err(format!("holds no key's {KEY_LENGTH} bytes"))
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

/// `bytes` in base64url without padding, as version 4 carries bytes.
pub fn base64url(bytes: &[u8]) -> String {
    URL_SAFE_NO_PAD.encode(bytes)
}

/// The context `name` of `version` over `fields`: the UTF-8 bytes of the
/// version's subprotocol, `boomdrive.v5` for version 5, a space and
/// `name`, then each field after a NUL byte, with nothing after the last
/// (the protocol's README, *The signatures*).
pub fn context(version: &Version, name: &str, fields: &[&[u8]]) -> Vec<u8> {
    let mut bytes = format!("{} {name}", version.subprotocol).into_bytes();
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

/// `version`'s enrollment context: the control plane's `origin`, the
/// token's digest and the public key's bytes.
pub fn enrollment_context(
    version: &Version,
    origin: &str,
    token: &str,
    key: &VerifyingKey,
) -> Vec<u8> {
    context(
        version,
        "enroll",
        &[
            origin.as_bytes(),
            token_digest(token).as_bytes(),
            key.as_bytes(),
        ],
    )
}

/// `version`'s connection context: the control plane's `origin`, the
/// public key's bytes, `time`'s decimal digits and the nonce's bytes.
pub fn connection_context(
    version: &Version,
    origin: &str,
    key: &VerifyingKey,
    time: u64,
    nonce: &[u8; NONCE_LENGTH],
) -> Vec<u8> {
    context(
        version,
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

/// The `Authorization` header's value for one connection attempt of
/// `version` to the control plane at `origin`, at `time` in Unix seconds
/// with `nonce`: `BoomdriveHost <key>.<time>.<nonce>.<signature>` (design
/// Decision 5).
pub fn statement(
    key: &SigningKey,
    version: &Version,
    origin: &str,
    time: u64,
    nonce: &[u8; NONCE_LENGTH],
) -> String {
    let public = key.verifying_key();
    let signed = signature(
        key,
        &connection_context(version, origin, &public, time, nonce),
    );
    format!(
        "BoomdriveHost {}.{time}.{}.{signed}",
        base64url(public.as_bytes()),
        base64url(nonce)
    )
}

/// `version`'s context of a forge token's ask: the control plane's `origin`, the
/// public key's bytes, `time`'s decimal digits, the nonce's bytes and the
/// repository's address, `owner/name`, as its UTF-8 bytes, so a statement
/// made for one repository is never taken for another, nor an ask's for a
/// connection's (design Decision 2 of
/// a-cloud-host-gets-its-github-tokens-from-the-cloud; record 0053).
pub fn forge_token_context(
    version: &Version,
    origin: &str,
    key: &VerifyingKey,
    time: u64,
    nonce: &[u8; NONCE_LENGTH],
    repository: &str,
) -> Vec<u8> {
    context(
        version,
        "forge token",
        &[
            origin.as_bytes(),
            key.as_bytes(),
            time.to_string().as_bytes(),
            nonce,
            repository.as_bytes(),
        ],
    )
}

/// The `Authorization` header's value for one ask of `repository`'s
/// forge token of the control plane at `origin`, at `time` in Unix
/// seconds with `nonce`: `BoomdriveHost <key>.<time>.<nonce>.<signature>`,
/// signed over [`forge_token_context`] (design Decision 2 of
/// a-cloud-host-gets-its-github-tokens-from-the-cloud).
pub fn forge_token_statement(
    key: &SigningKey,
    version: &Version,
    origin: &str,
    time: u64,
    nonce: &[u8; NONCE_LENGTH],
    repository: &str,
) -> String {
    let public = key.verifying_key();
    let context = forge_token_context(version, origin, &public, time, nonce, repository);
    format!(
        "BoomdriveHost {}.{time}.{}.{}",
        base64url(public.as_bytes()),
        base64url(nonce),
        signature(key, &context)
    )
}
