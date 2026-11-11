//! The Boomdrive App's tokens (the forge-tokens spec; record 0007 as
//! amended on 2026-11-11; core.33): a mint from the App's key, over a fake
//! of GitHub's API on the loopback address (design Decision 9). No test
//! reaches the network, and each test binary makes its own key (core.4).

mod common;
mod github;

use boomdrive::config::{Address, ForgeApp};
use boomdrive::forge::{Forge, Minted, Refused};
use common::Scratch;
use github::{GitHub, Key};
use std::path::PathBuf;
use std::time::{Duration, Instant};

/// The App `1`, its key at `key_file`.
fn app(key_file: PathBuf) -> ForgeApp {
    ForgeApp {
        app_id: 1,
        key_file,
    }
}

fn address(text: &str) -> Address {
    Address::parse(text).expect("an address")
}

/// A forge calling `github`.
fn forge(github: &GitHub) -> Forge {
    Forge::at(&github.url).calling_for(Duration::from_secs(5))
}

/// The forge-tokens spec's "A token for one repository".
#[test]
fn a_token_is_bought_for_its_repository_alone() {
    let scratch = Scratch::new();
    let github = GitHub::start();
    github.install("agentplot/flume", 7);
    let before = time::OffsetDateTime::now_utc();

    let minted = forge(&github)
        .mint(
            &app(Key::App.write(&scratch.home, 0o600)),
            &address("agentplot/flume"),
        )
        .expect("a token");

    let Minted::Token(token) = minted else {
        panic!("no token: {minted:?}");
    };
    assert_eq!(github.tokens(), [token.secret().to_string()]);
    let lasting = token.ends() - before;
    assert!(
        lasting > time::Duration::minutes(59) && lasting <= time::Duration::minutes(61),
        "{token:?}"
    );
    let requests = github.requests();
    assert_eq!(
        github.calls(),
        [
            "GET /repos/agentplot/flume/installation",
            "POST /app/installations/7/access_tokens"
        ]
    );
    assert_eq!(requests[1].body, r#"{"repositories":["flume"]}"#);
    for request in &requests {
        assert_eq!(request.signed_by, Some(Key::App), "{request:?}");
        assert_eq!(request.iss.as_deref(), Some("1"), "{request:?}");
    }
}

/// The forge-tokens spec's "The App is not installed".
#[test]
fn the_app_not_installed_is_its_own_answer() {
    let scratch = Scratch::new();
    let github = GitHub::start();
    github.install("agentplot/sluice", 7);

    let minted = forge(&github).mint(
        &app(Key::App.write(&scratch.home, 0o600)),
        &address("agentplot/flume"),
    );

    assert_eq!(minted, Ok(Minted::NotInstalled));
    assert_eq!(github.calls(), ["GET /repos/agentplot/flume/installation"]);
    assert!(github.tokens().is_empty());
}

/// The forge-tokens spec's "A key others may read".
#[test]
fn a_key_others_may_read_calls_no_one() {
    let scratch = Scratch::new();
    let github = GitHub::start();
    github.install("agentplot/flume", 7);
    let key_file = Key::App.write(&scratch.home, 0o644);

    let refused = forge(&github)
        .mint(&app(key_file.clone()), &address("agentplot/flume"))
        .expect_err("a refusal");

    assert!(refused.host_to_fix, "{refused:?}");
    for named in [
        key_file.display().to_string(),
        "0644".to_string(),
        "chmod 600".to_string(),
    ] {
        assert!(refused.words.contains(&named), "{named}: {}", refused.words);
    }
    assert!(github.requests().is_empty());
}

/// The forge-tokens spec's "A key file that is no key".
#[test]
fn a_key_file_that_is_no_key_calls_no_one() {
    let scratch = Scratch::new();
    let github = GitHub::start();
    github.install("agentplot/flume", 7);
    let key_file = Key::App.write(&scratch.home, 0o600);
    std::fs::write(
        &key_file,
        "-----BEGIN NOTHING-----\nAAAA\n-----END NOTHING-----\n",
    )
    .expect("the key file overwritten");

    let refused = forge(&github).mint(&app(key_file.clone()), &address("agentplot/flume"));

    assert_eq!(
        refused,
        Err(Refused {
            words: format!(
                "[forge] key_file {} holds no RSA private key in PEM",
                key_file.display()
            ),
            host_to_fix: true,
        })
    );
    assert!(github.requests().is_empty());
}

/// The forge-tokens spec's "GitHub refuses the App": named by its call,
/// status and message, the words holding neither the JWT nor any line of
/// the key.
#[test]
fn a_refused_mint_names_github_and_holds_no_secret() {
    let scratch = Scratch::new();
    let github = GitHub::start();
    github
        .install("agentplot/flume", 7)
        .answering(401, "A JSON web token could not be decoded");

    let refused = forge(&github)
        .mint(
            &app(Key::App.write(&scratch.home, 0o600)),
            &address("agentplot/flume"),
        )
        .expect_err("a refusal");

    assert_eq!(
        refused,
        Refused {
            words: "GitHub answered 401 to finding app 1's installation on agentplot/flume: \
                    A JSON web token could not be decoded"
                .to_string(),
            host_to_fix: false,
        }
    );
    let requests = github.requests();
    assert_eq!(requests.len(), 1);
    for part in requests[0].jwt.split('.') {
        assert!(!refused.words.contains(part), "{}", refused.words);
    }
    for line in Key::App.pem().lines().filter(|line| !line.is_empty()) {
        assert!(!refused.words.contains(line), "{}", refused.words);
    }
}

/// The forge-tokens spec's "GitHub's API never answers".
#[test]
fn a_silent_api_is_stopped_at_its_limit() {
    let scratch = Scratch::new();
    let github = GitHub::start();
    github.install("agentplot/flume", 7).silent();
    let forge = Forge::at(&github.url).calling_for(Duration::from_millis(500));
    let started = Instant::now();

    let refused = forge
        .mint(
            &app(Key::App.write(&scratch.home, 0o600)),
            &address("agentplot/flume"),
        )
        .expect_err("a refusal");

    assert!(
        started.elapsed() < Duration::from_secs(5),
        "{:?}",
        started.elapsed()
    );
    assert!(!refused.host_to_fix, "{refused:?}");
    assert!(
        refused.words.starts_with(
            "GitHub could not be reached to finding app 1's installation on agentplot/flume: "
        ),
        "{}",
        refused.words
    );
}
