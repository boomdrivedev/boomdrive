//! The hosted control plane's forge tokens (the hosted-control-plane
//! spec's "A served repository's token is asked by one request of version
//! 4", "Each partition served the repository is asked in the table's
//! order", "An ask's answer is checked against version 5 before it is
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
use serde_json::{json, Value};
use std::path::Path;
use std::sync::Arc;
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
    let memory = Arc::new(Memory::default());
    for label in keyed {
        let key = key_of(label);
        memory.write(&table.key_name(label), &key).expect("kept");
        fake.enrolled(label, &key.verifying_key());
    }
    Tokens::new(
        table,
        Box::new(move || Ok(Box::new(memory.clone()) as Box<dyn Store>)),
    )
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
/// version 5 and named `example/app`, its statement checks over the
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
    assert_eq!(refused, &None, "the ask passed version 5");
    assert_eq!(message["protocol_version"], json!(5));
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
fn words(asked: &TokenAsked) -> &str {
    match asked {
        TokenAsked::NotInstalled(words) | TokenAsked::Refused { words, .. } => words,
        TokenAsked::Token(_) => panic!("a token"),
    }
}

/// The partition each forge token's ask was proved by, in order.
fn askers(taken: &[Taken]) -> Vec<Option<String>> {
    asks(taken)
        .into_iter()
        .map(|ask| match ask {
            Taken::ForgeToken { partition, .. } => partition.clone(),
            _ => unreachable!("an ask"),
        })
        .collect()
}

/// The hosted-control-plane spec's "The second partition gives it":
/// `home` and `work` are each served `example/app`; `home` is asked first,
/// under its own key, and says the App is not installed, then `work`,
/// under its own, which gives the token.
#[test]
fn the_second_partition_gives_it() {
    let fake = Fake::start();
    let table = table(
        &fake,
        &[("home", &["example/app"]), ("work", &["example/app"])],
    );
    let tokens = tokens(&fake, table, &["home", "work"]);
    fake.answer_forge_token(
        "home",
        "example/app",
        fake::refused(403, true, "home's installation does not cover example/app"),
    );
    fake.answer_forge_token(
        "work",
        "example/app",
        fake::given("ghs_work", "2026-11-18T13:00:00Z"),
    );

    let asked = tokens.token("example/app");

    let TokenAsked::Token(token) = asked else {
        panic!("no token: {asked:?}");
    };
    assert_eq!(token.secret(), "ghs_work");
    let taken = fake.taken();
    assert_eq!(
        askers(&taken),
        [Some("home".to_string()), Some("work".to_string())],
        "{taken:#?}"
    );
}

/// The hosted-control-plane spec's "The App installed under no
/// partition": each of `home` and `work` says the App's installation does
/// not cover `example/app`, so the answer is the App's absence, its words
/// naming each partition with the fake's words for it.
#[test]
fn the_app_installed_under_no_partition() {
    let fake = Fake::start();
    let table = table(
        &fake,
        &[("home", &["example/app"]), ("work", &["example/app"])],
    );
    let tokens = tokens(&fake, table, &["home", "work"]);
    for (partition, words) in [
        ("home", "home's installation does not cover example/app"),
        ("work", "work's installation does not cover example/app"),
    ] {
        fake.answer_forge_token(partition, "example/app", fake::refused(403, true, words));
    }

    let asked = tokens.token("example/app");

    let TokenAsked::NotInstalled(words) = &asked else {
        panic!("not the App's absence: {asked:?}");
    };
    for named in [
        "`home`: home's installation does not cover example/app",
        "`work`: work's installation does not cover example/app",
    ] {
        assert!(words.contains(named), "{words}");
    }
    assert!(words.find("`home`") < words.find("`work`"), "{words}");
    assert_eq!(askers(&fake.taken()).len(), 2);
}

/// The hosted-control-plane spec's "A refusal carried as given": the fake
/// refuses `home`'s ask with `not_installed` false, so the ask is refused,
/// its words naming `home` and holding the fake's words as given.
#[test]
fn a_refusal_carried_as_given() {
    let fake = Fake::start();
    let table = table(&fake, &[("home", &["example/app"])]);
    let tokens = tokens(&fake, table, &["home"]);
    let given = "example/app is registered to no drive this host serves";
    fake.answer_forge_token("home", "example/app", fake::refused(403, false, given));

    let asked = tokens.token("example/app");

    let (words, host_to_fix) = refusal(&asked);
    assert!(!host_to_fix, "{words}");
    assert!(words.contains("`home`"), "{words}");
    assert!(words.contains(given), "{words}");
    assert_eq!(self::words(&asked), words);
}

/// The hosted-control-plane spec's "A repository no partition is served":
/// a token asked for `example/service`, which no partition names, is
/// refused naming it, and the fake takes no request.
#[test]
fn a_repository_no_partition_is_served() {
    let fake = Fake::start();
    let table = table(&fake, &[("home", &["example/app"])]);
    let tokens = tokens(&fake, table, &["home"]);

    let asked = tokens.token("example/service");

    let (words, _) = refusal(&asked);
    assert!(words.contains("example/service"), "{words}");
    assert!(fake.taken().is_empty(), "{:#?}", fake.taken());
}

/// The hosted-control-plane spec's "A control plane that speaks version
/// 3": a control plane with no `runner/forge-token` answers 404 with no
/// body, so the ask is refused, its words naming `runner/forge-token` and
/// 404.
#[test]
fn a_control_plane_that_speaks_version_3() {
    let fake = Fake::start();
    let table = table(&fake, &[("home", &["example/app"])]);
    let tokens = tokens(&fake, table, &["home"]);
    fake.answer_forge_token("home", "example/app", fake::answered(404, ""));

    let asked = tokens.token("example/app");

    let (words, host_to_fix) = refusal(&asked);
    assert!(!host_to_fix, "{words}");
    assert!(words.contains("runner/forge-token"), "{words}");
    assert!(words.contains("404"), "{words}");
}

/// An answer giving `token` for the ask, its body changed by `alter`.
fn given_altered(
    token: &str,
    alter: impl Fn(&mut Value) + Send + Sync + 'static,
) -> fake::ForgeAnswer {
    let token = token.to_string();
    Arc::new(move |ask: &Value| {
        let mut body = json!({
            "forge_token": ask["id"],
            "repository": ask["body"]["forge_token"]["repository"],
            "token": token,
            "expires_at": "2026-11-18T13:00:00Z",
        });
        alter(&mut body);
        Some((
            200,
            fake::message_text("plane:given", "forge_token_given", body),
        ))
    })
}

/// The hosted-control-plane spec's "A token given with a field unnamed":
/// a `forge_token_given` holding `scope` fails version 5, so the ask is
/// refused, its words naming `runner/forge-token` and the schema's path,
/// and the token is in no words.
#[test]
fn a_token_given_with_a_field_unnamed() {
    let fake = Fake::start();
    let table = table(&fake, &[("home", &["example/app"])]);
    let tokens = tokens(&fake, table, &["home"]);
    fake.answer_forge_token(
        "home",
        "example/app",
        given_altered("ghs_n0t-sh0wn", |body| body["scope"] = json!("contents")),
    );

    let asked = tokens.token("example/app");

    let (words, _) = refusal(&asked);
    assert!(words.contains("runner/forge-token"), "{words}");
    assert!(words.contains("fails the schema's /"), "{words}");
    assert!(!words.contains("ghs_n0t-sh0wn"), "{words}");
}

/// The hosted-control-plane spec's "A token for another repository": a
/// `forge_token_given` for the ask of `example/app` naming
/// `example/other` is refused, its words naming `runner/forge-token` and
/// `example/other`, and the token is in no words.
#[test]
fn a_token_for_another_repository() {
    let fake = Fake::start();
    let table = table(&fake, &[("home", &["example/app"])]);
    let tokens = tokens(&fake, table, &["home"]);
    fake.answer_forge_token(
        "home",
        "example/app",
        given_altered("ghs_n0t-sh0wn", |body| {
            body["repository"] = json!("example/other")
        }),
    );

    let asked = tokens.token("example/app");

    let (words, _) = refusal(&asked);
    assert!(words.contains("runner/forge-token"), "{words}");
    assert!(words.contains("example/other"), "{words}");
    assert!(!words.contains("ghs_n0t-sh0wn"), "{words}");
}

/// The hosted-control-plane spec's "The ask refused as a message": the
/// fake answers 400 with `message_refused` naming the ask, with words, so
/// the ask is refused, its words naming `runner/forge-token` and holding
/// the fake's words as given.
#[test]
fn the_ask_refused_as_a_message() {
    let fake = Fake::start();
    let table = table(&fake, &[("home", &["example/app"])]);
    let tokens = tokens(&fake, table, &["home"]);
    let given = "/body/forge_token/repository is not one this control plane reads";
    fake.answer_forge_token(
        "home",
        "example/app",
        Arc::new(move |ask: &Value| {
            let body = json!({ "message": ask["id"], "words": given });
            Some((
                400,
                fake::message_text("plane:refused", "message_refused", body),
            ))
        }),
    );

    let asked = tokens.token("example/app");

    let (words, _) = refusal(&asked);
    assert!(words.contains("runner/forge-token"), "{words}");
    assert!(words.contains(given), "{words}");
}
