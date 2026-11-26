//! The host's key is kept only in its credential store (the
//! hosted-control-plane spec's "The host's key is kept only in its
//! credential store"; design Decision 3; core.32): the `systemd-creds`
//! store and the keychain's, each over a stub program the test writes. No
//! test runs a real `systemd-creds` or `security`, or touches a keychain
//! (design Decision 14).

use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use base64::Engine;
use boomdrive_hosted::key::{self, KeyName, Keychain, Memory, Store, SystemdCreds};
use ed25519_dalek::SigningKey;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

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
    let file = credstore_dir.join("boomdrive.home.control.example");
    assert_eq!(
        encrypt,
        format!(
            "encrypt --user --name=boomdrive.home.control.example - {}",
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
            "decrypt --user --name=boomdrive.home.control.example {} -",
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
        credstore_dir.join("boomdrive.home.control.example"),
        b"short",
    )
    .expect("a credential");
    let store = SystemdCreds::open(program, credstore_dir).expect("systemd 257 is taken");
    let refusal = store.read(&home()).expect_err("five bytes are no key");
    assert_eq!(refusal, "`home`'s credential holds 5 bytes, not a key's 32");
}

/// A stub of macOS's `security`, the test's own. It writes each call's
/// arguments as a line of `calls`, and what `-i` is handed on standard
/// input to `stdin`. `-i` keeps the hex after `-X` as the item of the
/// account its line quotes, in `item.<account>`, and says a refusal where
/// `refuse` is there, as `security -i` does; `find-generic-password` prints
/// that item's text and a newline, as `security` prints an item that holds
/// a byte it cannot print, or exits 44, `errSecItemNotFound`'s, where none
/// is, or 51, `errSecAuthFailed`'s, where `deny` is there. The stub keeps
/// no keychain, which the real program's is.
fn security_stub(dir: &Path) -> PathBuf {
    let program = dir.join("security");
    let script = "#!/bin/sh\n\
         dir=\"$(dirname \"$0\")\"\n\
         printf '%s\\n' \"$*\" >> \"$dir/calls\"\n\
         case \"$1\" in\n\
         -i)\n\
           line=\"$(cat)\"\n\
           printf '%s\\n' \"$line\" >> \"$dir/stdin\"\n\
           if [ -e \"$dir/refuse\" ]; then echo 'add-generic-password: returned 25' >&2; exit 25; fi\n\
           account=\"${line#*-a \\'}\"; account=\"${account%%\\'*}\"\n\
           printf '%s' \"${line##* -X }\" > \"$dir/item.$account\" ;;\n\
         find-generic-password)\n\
           if [ -e \"$dir/deny\" ]; then echo 'security: SecKeychainItemCopyContent: The user name or passphrase you entered is not correct.' >&2; exit 51; fi\n\
           if [ -e \"$dir/item.$5\" ]; then cat \"$dir/item.$5\"; echo; else echo 'security: SecKeychainSearchCopyNext: The specified item could not be found in the keychain.' >&2; exit 44; fi ;;\n\
         *) echo \"no such command: $1\" >&2; exit 2 ;;\n\
         esac\n";
    std::fs::write(&program, script).expect("the stub");
    std::fs::set_permissions(&program, std::fs::Permissions::from_mode(0o755))
        .expect("the stub runs");
    program
}

#[test]
fn kept_in_the_keychain() {
    let scratch = tempfile::tempdir().expect("a scratch directory");
    let keychain = Keychain::open(security_stub(scratch.path()));
    let key = test_key();

    assert!(matches!(keychain.read(&home()), Ok(None)));
    keychain.write(&home(), &key).expect("the key kept");
    let read = keychain
        .read(&home())
        .expect("the key read")
        .expect("a key kept");
    assert_eq!(read.to_bytes(), key.to_bytes());

    let calls = std::fs::read(scratch.path().join("calls")).expect("the stub's calls");
    let calls_text = String::from_utf8_lossy(&calls);
    assert_eq!(
        calls_text,
        "find-generic-password -s boomdrive -a home@control.example -w\n\
         -i\n\
         find-generic-password -s boomdrive -a home@control.example -w\n"
    );
    let hex: String = key.to_bytes().iter().map(|b| format!("{b:02x}")).collect();
    assert_eq!(
        std::fs::read_to_string(scratch.path().join("stdin")).expect("what it was handed"),
        format!("add-generic-password -U -s boomdrive -a 'home@control.example' -X {hex}\n"),
        "the key reaches it on standard input"
    );
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

    let other = KeyName::new("home", "staging.control.example");
    assert!(
        matches!(keychain.read(&other), Ok(None)),
        "a key is kept for its control plane alone"
    );
}

#[test]
fn a_key_replaced() {
    let scratch = tempfile::tempdir().expect("a scratch directory");
    let keychain = Keychain::open(security_stub(scratch.path()));
    keychain.write(&home(), &test_key()).expect("the key kept");
    let again = SigningKey::from_bytes(&[9u8; 32]);
    keychain.write(&home(), &again).expect("the key kept again");
    let read = keychain.read(&home()).unwrap().unwrap();
    assert_eq!(read.to_bytes(), again.to_bytes());
}

/// `security` prints an item whose every byte is printable as those bytes
/// themselves, and the runner reads them as the key, even where they are
/// hex digits; a key with a byte it cannot print it prints in hex.
#[test]
fn a_key_of_printable_bytes() {
    let scratch = tempfile::tempdir().expect("a scratch directory");
    let keychain = Keychain::open(security_stub(scratch.path()));
    for bytes in [*b"0123456789abcdef0123456789abcdef", [b'~'; 32]] {
        std::fs::write(scratch.path().join("item.home@control.example"), bytes).expect("an item");
        let read = keychain.read(&home()).unwrap().unwrap();
        assert_eq!(read.to_bytes(), bytes);
    }
}

#[test]
fn an_item_that_is_no_key() {
    let scratch = tempfile::tempdir().expect("a scratch directory");
    let keychain = Keychain::open(security_stub(scratch.path()));
    std::fs::write(scratch.path().join("item.home@control.example"), "0001ff").expect("an item");
    let refusal = keychain.read(&home()).expect_err("three bytes are no key");
    assert_eq!(
        refusal,
        "the keychain's item for `home` holds no key's 32 bytes"
    );
}

#[test]
fn the_keychain_refuses() {
    let scratch = tempfile::tempdir().expect("a scratch directory");
    let keychain = Keychain::open(security_stub(scratch.path()));
    std::fs::write(scratch.path().join("deny"), "").expect("a denial");
    assert_eq!(
        keychain.read(&home()).expect_err("a refused read"),
        "the keychain could not keep `home`'s key: `security` failed: security: \
         SecKeychainItemCopyContent: The user name or passphrase you entered is not correct."
    );
    std::fs::write(scratch.path().join("refuse"), "").expect("a refusal");
    assert_eq!(
        keychain
            .write(&home(), &test_key())
            .expect_err("a refused write"),
        "the keychain could not keep `home`'s key: `security` failed: add-generic-password: \
         returned 25"
    );
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
