//! The host's key is kept only in its credential store (the
//! hosted-control-plane spec's "The host's key is kept only in its
//! credential store"; design Decision 3; core.32): the `systemd-creds`
//! store over a stub program the test writes, and the keychain's over
//! keyring's mock store. No test runs a real `systemd-creds` or touches a
//! keychain (design Decision 14).

use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use base64::Engine;
use ed25519_dalek::SigningKey;
use flywheel_hosted::key::{self, KeyName, Keychain, Memory, Store, SystemdCreds};
use keyring_core::api::CredentialStoreApi;
use keyring_core::{mock, CredentialStore};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// A stub of `systemd-creds`, the test's own, reporting systemd
/// `version`. It writes each call's arguments as a line of `calls`, and
/// what `encrypt` is handed on standard input to `stdin`. `encrypt` keeps
/// its input as the credential file its last argument names, and
/// `decrypt` gives the file its fourth argument names: the stub seals
/// nothing, which is the real program's to do.
fn stub(dir: &Path, version: u32) -> PathBuf {
    let program = dir.join("systemd-creds");
    let script = format!(
        "#!/bin/sh\n\
         dir=\"$(dirname \"$0\")\"\n\
         printf '%s\\n' \"$*\" >> \"$dir/calls\"\n\
         case \"$1\" in\n\
         --version) printf 'systemd {version} ({version}-stub)\\n+PAM +AUDIT\\n' ;;\n\
         encrypt) tee \"$dir/stdin\" > \"$5\" ;;\n\
         decrypt) cat \"$4\" ;;\n\
         *) echo \"no such verb: $1\" >&2; exit 1 ;;\n\
         esac\n"
    );
    std::fs::write(&program, script).expect("the stub");
    std::fs::set_permissions(&program, std::fs::Permissions::from_mode(0o755))
        .expect("the stub runs");
    program
}

/// A key of the test's own, fixed bytes.
fn test_key() -> SigningKey {
    SigningKey::from_bytes(&[7u8; 32])
}

fn home() -> KeyName {
    KeyName::new("home", "control.example")
}

#[test]
fn kept_by_systemd_creds() {
    let scratch = tempfile::tempdir().expect("a scratch directory");
    let program = stub(scratch.path(), 256);
    let credstore_dir = scratch.path().join("config/credstore.encrypted");
    let store = SystemdCreds::open(program, credstore_dir.clone()).expect("systemd 256 is taken");
    let key = test_key();

    assert!(matches!(store.read(&home()), Ok(None)));
    store.write(&home(), &key).expect("the key kept");
    let made = std::fs::metadata(&credstore_dir).expect("the store made");
    assert_eq!(made.permissions().mode() & 0o777, 0o700, "the user's alone");

    let calls = std::fs::read(scratch.path().join("calls")).expect("the stub's calls");
    let calls_text = String::from_utf8_lossy(&calls);
    let encrypt = calls_text
        .lines()
        .find(|line| line.starts_with("encrypt"))
        .expect("an encrypt");
    let file = credstore_dir.join("flywheel.home.control.example");
    assert_eq!(
        encrypt,
        format!(
            "encrypt --user --name=flywheel.home.control.example - {}",
            file.display()
        )
    );
    assert_eq!(
        std::fs::read(scratch.path().join("stdin")).expect("what it was handed"),
        key.to_bytes(),
        "the key's bytes reach it on standard input"
    );
    let hex: String = key.to_bytes().iter().map(|b| format!("{b:02x}")).collect();
    for shown in [
        hex,
        STANDARD.encode(key.to_bytes()),
        URL_SAFE_NO_PAD.encode(key.to_bytes()),
    ] {
        assert!(
            !calls_text.contains(&shown),
            "an argument holds the key: {calls_text}"
        );
    }
    assert!(
        !calls.windows(32).any(|window| window == key.to_bytes()),
        "an argument holds the key's bytes"
    );

    let read = store
        .read(&home())
        .expect("the key read")
        .expect("a key kept");
    assert_eq!(read.to_bytes(), key.to_bytes());
    let decrypt = String::from_utf8_lossy(
        &std::fs::read(scratch.path().join("calls")).expect("the stub's calls"),
    )
    .lines()
    .find(|line| line.starts_with("decrypt"))
    .map(str::to_string)
    .expect("a decrypt");
    assert_eq!(
        decrypt,
        format!(
            "decrypt --user --name=flywheel.home.control.example {} -",
            file.display()
        )
    );
}

#[test]
fn an_old_systemd() {
    let scratch = tempfile::tempdir().expect("a scratch directory");
    let program = stub(scratch.path(), 255);
    let refusal = SystemdCreds::open(program, scratch.path().join("credstore.encrypted"))
        .expect_err("systemd 255 is refused");
    assert_eq!(
        refusal,
        "`systemd-creds` is systemd 255's: a partition's key is kept as a user's credential, \
         which takes systemd 256 or later"
    );
    let calls = std::fs::read_to_string(scratch.path().join("calls")).expect("the stub's calls");
    assert_eq!(calls, "--version\n", "nothing but its version is asked");
}

#[test]
fn no_systemd_creds() {
    let scratch = tempfile::tempdir().expect("a scratch directory");
    let refusal = SystemdCreds::open(
        scratch.path().join("systemd-creds"),
        scratch.path().join("credstore.encrypted"),
    )
    .expect_err("a program that is not there is refused");
    assert!(
        refusal.contains("this host keeps each partition's key with `systemd-creds` of systemd 256 or later, or in the macOS keychain"),
        "{refusal}"
    );
}

#[test]
fn a_credential_that_is_no_key() {
    let scratch = tempfile::tempdir().expect("a scratch directory");
    let program = stub(scratch.path(), 257);
    let credstore_dir = scratch.path().join("credstore.encrypted");
    std::fs::create_dir_all(&credstore_dir).expect("the store");
    std::fs::write(
        credstore_dir.join("flywheel.home.control.example"),
        b"short",
    )
    .expect("a credential");
    let store = SystemdCreds::open(program, credstore_dir).expect("systemd 257 is taken");
    let refusal = store.read(&home()).expect_err("five bytes are no key");
    assert_eq!(refusal, "`home`'s credential holds 5 bytes, not a key's 32");
}

#[test]
fn kept_in_the_keychain() {
    let mock = mock::Store::new().expect("keyring's mock store");
    let keychain = Keychain::over(mock.clone() as Arc<CredentialStore>);
    let key = test_key();

    assert!(matches!(keychain.read(&home()), Ok(None)));
    keychain.write(&home(), &key).expect("the key kept");
    let read = keychain
        .read(&home())
        .expect("the key read")
        .expect("a key kept");
    assert_eq!(read.to_bytes(), key.to_bytes());

    let item = mock
        .build("flywheel", "home@control.example", None)
        .expect("the item");
    assert_eq!(item.get_secret().expect("its secret"), key.to_bytes());
    let other = mock
        .build("flywheel", "home@staging.control.example", None)
        .expect("another control plane's item");
    assert!(
        other.get_secret().is_err(),
        "a key is kept for its control plane alone"
    );
}

#[test]
fn a_key_replaced() {
    let keychain = Keychain::over(mock::Store::new().expect("keyring's mock store"));
    keychain.write(&home(), &test_key()).expect("the key kept");
    let again = SigningKey::from_bytes(&[9u8; 32]);
    keychain.write(&home(), &again).expect("the key kept again");
    let read = keychain.read(&home()).unwrap().unwrap();
    assert_eq!(read.to_bytes(), again.to_bytes());
}

#[test]
fn kept_in_memory() {
    let memory = Memory::default();
    assert!(matches!(memory.read(&home()), Ok(None)));
    memory.write(&home(), &test_key()).unwrap();
    assert_eq!(memory.accounts(), ["home@control.example"]);
    assert_eq!(
        memory.read(&home()).unwrap().unwrap().to_bytes(),
        test_key().to_bytes()
    );
}

#[test]
fn each_key_made_afresh() {
    let one = key::made().expect("a key");
    let two = key::made().expect("a key");
    assert_ne!(one.to_bytes(), two.to_bytes());
    assert_ne!(key::nonce().unwrap(), key::nonce().unwrap());
}
