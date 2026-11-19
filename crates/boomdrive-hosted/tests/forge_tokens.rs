//! The hosted control plane's forge tokens (the hosted-control-plane
//! spec's "A served repository's token is asked by one request of version
//! 4", "Each partition served the repository is asked in the table's
//! order", "An ask's answer is checked against version 4 before it is
//! read" and "A host with no partition's key must be fixed"; design
//! Decisions 2 and 7 of a-cloud-host-gets-its-github-tokens-from-the-cloud;
//! record 0053), over the fake hosted control plane on the loopback
//! address, each partition's key in a store in memory.

mod common;

use boomdrive_control::{At, ForgeTokens, TokenAsked};
use boomdrive_hosted::forge_token::Tokens;
use boomdrive_hosted::key::{Memory, Store};
use boomdrive_hosted::table::Table;
use common::fake::{self, Fake, Taken};
use ed25519_dalek::SigningKey;
use serde_json::json;
use std::path::Path;
use std::time::{Duration, Instant};

/// A partition's key, fixed bytes of the test's own.
fn key_of(partition: &str) -> SigningKey {
    let byte = match partition {
        "home" => 5,
        _ => 6,
    };
    SigningKey::from_bytes(&[byte; 32])
}

/// The hosted table naming `fake`, with each of `partitions`, a label and
/// the repositories it is served, in that order.
fn table(fake: &Fake, partitions: &[(&str, &[&str])]) -> Table {
    let mut text = format!("url = {:?}\n", fake.url());
    for (label, repositories) in partitions {
        text.push_str(&format!(
            "[[partitions]]\nlabel = {label:?}\nrepositories = {repositories:?}\n"
        ));
    }
    Table::read(
        &toml::from_str(&text).expect("a TOML table"),
        Path::new("/h/.config/boomdrive/config.toml"),
    )
    .expect("the table read")
}

/// The forge tokens of `table`, each of `keyed`'s keys in a store in
/// memory and enrolled with `fake`.
fn tokens(fake: &Fake, table: Table, keyed: &[&str]) -> Tokens {
    let memory = Memory::default();
    for label in keyed {
        let key = key_of(label);
        memory.write(&table.key_name(label), &key).expect("kept");
        fake.enrolled(label, &key.verifying_key());
    }
    Tokens::new(table, Ok(Box::new(memory)))
}

/// Each forge token's ask the fake took, in order.
fn asks(taken: &[Taken]) -> Vec<&Taken> {
    taken
        .iter()
        .filter(|t| matches!(t, Taken::ForgeToken { .. }))
        .collect()
}

/// The words of a refusal, or the test fails naming what it gave.
fn refusal(asked: &TokenAsked) -> (&str, bool) {
    match asked {
        TokenAsked::Refused { words, host_to_fix } => (words, *host_to_fix),
        other => panic!("not refused: {other:?}"),
    }
}

/// The hosted-control-plane spec's "A token given": the ask's body passed
/// version 4 and named `example/app`, its statement checks over the
/// context `forge token` with `home`'s key and `example/app`, and the
/// answer is the fake's token, ending at its time.
#[test]
fn a_token_given() {
    let fake = Fake::start();
    let table = table(&fake, &[("home", &["example/app"])]);
    let tokens = tokens(&fake, table, &["home"]);
    fake.answer_forge_token(
        "home",
        "example/app",
        fake::given("ghs_example", "2026-11-18T13:00:00Z"),
    );

    let asked = tokens.token("example/app");

    let TokenAsked::Token(token) = asked else {
        panic!("no token: {asked:?}");
    };
    assert_eq!(token.secret(), "ghs_example");
    assert_eq!(
        token.ends(),
        At::parse("2026-11-18T13:00:00Z").unwrap().time()
    );
    let taken = fake.taken();
    let asks = asks(&taken);
    assert_eq!(asks.len(), 1, "{taken:#?}");
    let Taken::ForgeToken {
        partition,
        message,
        refused,
        signed,
        status,
        ..
    } = asks[0]
    else {
        unreachable!("an ask");
    };
    assert_eq!(partition.as_deref(), Some("home"));
    assert_eq!(refused, &None, "the ask passed version 4");
    assert_eq!(message["protocol_version"], json!(4));
    assert_eq!(
        message["body"],
        json!({ "forge_token": { "repository": "example/app" } })
    );
    assert!(*signed, "the statement checks over its context");
    assert_eq!(*status, Some(200));
}

/// The hosted-control-plane spec's "Each ask signed afresh": two asks'
/// statements hold nonces that differ, and no upgrade of `runner/connect`
/// is taken.
#[test]
fn each_ask_signed_afresh() {
    let fake = Fake::start();
    let table = table(&fake, &[("home", &["example/app"])]);
    let tokens = tokens(&fake, table, &["home"]);
    fake.answer_forge_token(
        "home",
        "example/app",
        fake::given("ghs_example", "2026-11-18T13:00:00Z"),
    );

    for _ in 0..2 {
        assert!(matches!(tokens.token("example/app"), TokenAsked::Token(_)));
    }

    let taken = fake.taken();
    let nonces: Vec<&Vec<u8>> = asks(&taken)
        .into_iter()
        .map(|ask| match ask {
            Taken::ForgeToken {
                nonce: Some(nonce),
                signed: true,
                ..
            } => nonce,
            other => panic!("an ask with no signed statement: {other:?}"),
        })
        .collect();
    assert_eq!(nonces.len(), 2, "{taken:#?}");
    assert_ne!(nonces[0], nonces[1], "each ask's nonce is its own");
    assert!(
        taken.iter().all(|t| !matches!(t, Taken::Upgrade { .. })),
        "{taken:#?}"
    );
}

/// The hosted-control-plane spec's "A control plane that never answers":
/// an ask the fake takes and never answers is refused at the asks' limit,
/// 500 ms here, well within 5 seconds, its words naming
/// `runner/forge-token`.
#[test]
fn a_control_plane_that_never_answers() {
    let fake = Fake::start();
    let table = table(&fake, &[("home", &["example/app"])]);
    let tokens = tokens(&fake, table, &["home"]).asking_for(Duration::from_millis(500));
    fake.answer_forge_token("home", "example/app", fake::never());

    let began = Instant::now();
    let asked = tokens.token("example/app");

    assert!(
        began.elapsed() < Duration::from_secs(5),
        "{:?}",
        began.elapsed()
    );
    let (words, host_to_fix) = refusal(&asked);
    assert!(words.contains("runner/forge-token"), "{words}");
    assert!(!host_to_fix, "{words}");
    let taken = fake.taken();
    assert!(
        matches!(asks(&taken)[..], [Taken::ForgeToken { status: None, .. }]),
        "{taken:#?}"
    );
}

/// The hosted-control-plane spec's "No key of the partition": `home` is
/// served `example/app` and the store holds no key of it, so the ask is
/// the host's to fix, naming `boomdrive enroll home`, and no request is
/// made.
#[test]
fn no_key_of_the_partition() {
    let fake = Fake::start();
    let table = table(&fake, &[("home", &["example/app"])]);
    let tokens = tokens(&fake, table, &[]);

    let asked = tokens.token("example/app");

    let (words, host_to_fix) = refusal(&asked);
    assert!(host_to_fix, "{words}");
    assert!(words.contains("`boomdrive enroll home`"), "{words}");
    assert!(fake.taken().is_empty(), "{:#?}", fake.taken());
}

/// The words `asked` gives, of any answer but a token.
#[allow(dead_code)]
fn words(asked: &TokenAsked) -> &str {
    match asked {
        TokenAsked::NotInstalled(words) | TokenAsked::Refused { words, .. } => words,
        TokenAsked::Token(_) => panic!("a token"),
    }
}
