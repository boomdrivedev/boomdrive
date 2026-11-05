//! A partition is enrolled once (the hosted-control-plane spec's "A
//! partition is enrolled once by flywheel enroll" and "flywheel enroll
//! refuses what it cannot enrol"; design Decision 4; record 0006): one
//! `POST runner/enrol` bearing the token, its answer checked, and the key
//! kept only once `enrolled` answers, over the fake hosted control plane.

mod common;

use common::fake::{Fake, Taken};
use ed25519_dalek::SigningKey;
use flywheel_hosted::enrolment::{enrol, Enrolled, Refusal};
use flywheel_hosted::key::{KeyName, Memory, Store, SystemdCreds};
use flywheel_hosted::table::Table;
use serde_json::json;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::sync::Arc;

/// The enrolment token of these tests, their own.
const TOKEN: &str = "test-enrolment-token";

/// The hosted table naming `url`, with partitions `home` and `work`.
fn table(url: &str) -> Table {
    let text = format!(
        "url = {url:?}\n\
         [[partitions]]\nlabel = \"home\"\nrepositories = [\"example/app\"]\n\
         [[partitions]]\nlabel = \"work\"\nrepositories = [\"example/service\", \"example/app\"]\n"
    );
    Table::read(
        &toml::from_str(&text).expect("a TOML table"),
        Path::new("/h/.config/flywheel/config.toml"),
    )
    .expect("the table read")
}

/// The token `token`, given when it is asked for.
fn given(token: &str) -> impl FnOnce() -> Result<String, String> + '_ {
    move || Ok(token.to_string())
}

/// A token that is never asked for: nothing is sent, so no token is
/// read.
fn never_asked() -> Result<String, String> {
    panic!("the token was asked for")
}

/// A store in memory, shared with the test.
fn memory() -> (Arc<Memory>, impl FnOnce() -> Result<Box<dyn Store>, String>) {
    let memory = Arc::new(Memory::default());
    let kept = memory.clone();
    (memory, move || Ok(Box::new(kept) as Box<dyn Store>))
}

/// A loopback address with nothing listening on it.
fn loopback_with_nothing_on_it() -> String {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("a loopback port");
    let port = listener.local_addr().expect("its address").port();
    drop(listener);
    format!("http://127.0.0.1:{port}/flywheel")
}

#[test]
fn a_partition_enrolled() {
    let fake = Fake::start();
    fake.issue("home", TOKEN, "h-1");
    let table = table(&fake.url());
    let (memory, store) = memory();

    let enrolled = enrol(&table, "home", given(TOKEN), store, "host-a");

    assert_eq!(
        enrolled,
        Ok(Enrolled {
            partition: "home".to_string(),
            host: "h-1".to_string(),
        })
    );
    let taken = fake.taken();
    let [Taken::Enrol {
        bearer,
        message,
        refused,
        signed,
        status,
    }] = taken.as_slice()
    else {
        panic!("one enrolment taken: {taken:#?}");
    };
    assert_eq!(bearer.as_deref(), Some(TOKEN), "the request bore the token");
    assert_eq!(refused, &None, "its enrol passed version 1");
    assert!(signed, "its signature checked");
    assert_eq!(*status, 200);
    assert_eq!(message["body"]["enrol"]["machine"], json!("host-a"));
    assert!(
        !message.to_string().contains(TOKEN),
        "no message holds the token"
    );

    let kept = memory
        .read(&table.key_name("home"))
        .expect("the store read")
        .expect("the key is in the store under `home`");
    assert_eq!(
        fake.keys("home"),
        [kept.verifying_key().to_bytes()],
        "the key kept is the key enrolled"
    );
    assert_eq!(memory.accounts(), ["home@127.0.0.1"]);
}

#[test]
fn a_lapsed_token() {
    let fake = Fake::start();
    fake.issue("home", TOKEN, "h-1");
    fake.refuse_token(TOKEN, "the token has lapsed");
    let (memory, store) = memory();

    let refusal = enrol(&table(&fake.url()), "home", given(TOKEN), store, "host-a");

    assert_eq!(
        refusal,
        Err(Refusal {
            words: "the token has lapsed".to_string(),
            host_to_fix: false,
        })
    );
    assert_eq!(
        memory.accounts(),
        Vec::<String>::new(),
        "the store is as it was"
    );
    assert!(fake.keys("home").is_empty());
}

#[test]
fn a_spent_token() {
    let fake = Fake::start();
    fake.issue("home", TOKEN, "h-1");
    let table = table(&fake.url());
    let (_, store) = memory();
    enrol(&table, "home", given(TOKEN), store, "host-a").expect("the first enrolment");
    let (memory, store) = memory();

    let refusal = enrol(&table, "home", given(TOKEN), store, "host-a").expect_err("a token spent");

    assert_eq!(refusal.words, "the token was spent");
    assert!(!refusal.host_to_fix);
    assert!(memory.accounts().is_empty());
}

#[test]
fn an_old_systemd_makes_no_request() {
    let fake = Fake::start();
    fake.issue("home", TOKEN, "h-1");
    let scratch = tempfile::tempdir().expect("a scratch directory");
    let program = scratch.path().join("systemd-creds");
    std::fs::write(&program, "#!/bin/sh\nprintf 'systemd 255 (255-stub)\\n'\n").expect("the stub");
    std::fs::set_permissions(&program, std::fs::Permissions::from_mode(0o755))
        .expect("the stub runs");
    let credstore_dir = scratch.path().join("credstore.encrypted");

    let refusal = enrol(
        &table(&fake.url()),
        "home",
        never_asked,
        || {
            SystemdCreds::open(program, credstore_dir)
                .map(|store| Box::new(store) as Box<dyn Store>)
        },
        "host-a",
    )
    .expect_err("systemd 255 is refused");

    assert!(refusal.host_to_fix, "the host's to fix");
    assert!(
        refusal.words.contains("takes systemd 256 or later"),
        "{}",
        refusal.words
    );
    assert!(fake.taken().is_empty(), "no request was made");
}

#[test]
fn an_unknown_label() {
    let fake = Fake::start();
    let (_, store) = memory();
    let refusal = enrol(&table(&fake.url()), "garden", never_asked, store, "host-a");
    assert_eq!(
        refusal,
        Err(Refusal {
            words: "[control_plane.hosted] names no partition `garden`: it names `home`, `work`"
                .to_string(),
            host_to_fix: true,
        })
    );
    assert!(fake.taken().is_empty());
}

#[test]
fn an_empty_token_or_one_no_header_carries() {
    let fake = Fake::start();
    for token in ["", "two words", "line\nbreak"] {
        let (memory, store) = memory();
        let refusal = enrol(&table(&fake.url()), "home", given(token), store, "host-a")
            .expect_err("the token is refused");
        assert!(refusal.host_to_fix, "{token:?}");
        assert!(
            refusal.words.starts_with("the enrolment token "),
            "{}",
            refusal.words
        );
        assert!(memory.accounts().is_empty());
    }
    assert!(fake.taken().is_empty(), "no request was made");
}

#[test]
fn nothing_listening() {
    let url = loopback_with_nothing_on_it();
    let (memory, store) = memory();

    let refusal =
        enrol(&table(&url), "home", given(TOKEN), store, "host-a").expect_err("unreachable");

    assert!(
        !refusal.host_to_fix,
        "a control plane unreachable is not the host's to fix"
    );
    assert!(
        refusal.words.starts_with(&format!(
            "the control plane at {} is unreachable: ",
            table(&url).origin()
        )),
        "{}",
        refusal.words
    );
    assert!(!refusal.words.contains(TOKEN), "{}", refusal.words);
    assert!(memory.accounts().is_empty(), "no key was written");
}

#[test]
fn the_enrolments_message_refused() {
    let fake = Fake::start();
    fake.answer_enrolment(
        400,
        json!({
            "protocol_version": 1,
            "id": "plane:1",
            "body": { "message_refused": { "message": "runner:1", "words": "/body/enrol/machine is too long" } }
        }),
    );
    let (memory, store) = memory();

    let refusal = enrol(&table(&fake.url()), "home", given(TOKEN), store, "host-a");

    assert_eq!(
        refusal,
        Err(Refusal {
            words:
                "the control plane refused the enrolment's message: /body/enrol/machine is too long"
                    .to_string(),
            host_to_fix: false,
        })
    );
    assert!(memory.accounts().is_empty());
}

#[test]
fn an_answer_that_fails_version_1() {
    let fake = Fake::start();
    fake.answer_enrolment(
        200,
        json!({
            "protocol_version": 1,
            "id": "plane:1",
            "body": { "enrolled": { "host": "h-1", "certificate": "n0t-r34d" } }
        }),
    );
    let (memory, store) = memory();

    let refusal = enrol(&table(&fake.url()), "home", given(TOKEN), store, "host-a")
        .expect_err("refused whole");

    assert!(!refusal.host_to_fix);
    assert!(
        refusal
            .words
            .starts_with("the control plane's answer, 200, fails version 1's check: "),
        "{}",
        refusal.words
    );
    assert!(!refusal.words.contains("n0t-r34d"), "{}", refusal.words);
    assert!(memory.accounts().is_empty(), "the store is as it was");
}

#[test]
fn an_answer_of_another_kind() {
    let fake = Fake::start();
    fake.answer_enrolment(
        200,
        json!({
            "protocol_version": 1,
            "id": "plane:1",
            "body": { "pulled": { "wants": [], "report": null } }
        }),
    );
    let (memory, store) = memory();

    let refusal = enrol(&table(&fake.url()), "home", given(TOKEN), store, "host-a");

    assert_eq!(
        refusal,
        Err(Refusal {
            words: "the control plane answered 200 with `pulled`, which is no answer to an \
                    enrolment"
                .to_string(),
            host_to_fix: false,
        })
    );
    assert!(memory.accounts().is_empty());
}

/// A store that keeps nothing.
struct Refusing;

impl Store for Refusing {
    fn read(&self, _: &KeyName) -> Result<Option<SigningKey>, String> {
        Ok(None)
    }

    fn write(&self, _: &KeyName, _: &SigningKey) -> Result<(), String> {
        Err("the store is locked".to_string())
    }
}

#[test]
fn a_key_enrolled_and_not_kept() {
    let fake = Fake::start();
    fake.issue("home", TOKEN, "h-1");

    let refusal = enrol(
        &table(&fake.url()),
        "home",
        given(TOKEN),
        || Ok(Box::new(Refusing) as Box<dyn Store>),
        "host-a",
    )
    .expect_err("a key not kept");

    assert!(refusal.host_to_fix);
    assert_eq!(
        refusal.words,
        "the control plane enrolled `home` as host h-1, but its key could not be kept: the store \
         is locked. The control plane holds a key this host does not, and a new token enrols the \
         partition again"
    );
}
