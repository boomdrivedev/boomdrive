//! The Boomdrive App's tokens (the forge-tokens spec; record 0007 as
//! amended on 2026-11-11; core.33): a mint from the App's key, over a fake
//! of GitHub's API on the loopback address (design Decision 9). No test
//! reaches the network, and each test binary makes its own key (core.4).

mod common;
mod github;

use boomdrive::config::{Address, ForgeApp};
use boomdrive::forge::{Forge, Minted, Refused};
use boomdrive::forge_token::{self, Operation};
use common::{keeping, Scratch};
use github::{GitHub, Key};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Stdio;
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

/// A configuration keeping `agentplot/flume` in the drive `home` and
/// naming the App `1`, its key written at mode 0600 into the scratch home.
fn naming_the_app(scratch: &Scratch) -> String {
    Key::App.write(&scratch.home, 0o600);
    format!(
        "{}\n[forge]\napp_id = 1\nkey_file = \"~/.config/boomdrive/app.pem\"\n",
        keeping(&["agentplot/flume"])
    )
}

/// git's request for `path` on `https://github.com`, as git writes it,
/// with the keys the runner passes over.
fn for_path(path: &str) -> String {
    format!(
        "capability[]=authtype\nprotocol=https\nhost=github.com\npath={path}\n\
         wwwauth[]=Basic realm=\"GitHub\"\n\n"
    )
}

/// `boomdrive forge-token <operation>` answered in-process, over `github`,
/// with `request` as its standard input, under the scratch host's
/// configuration and its local layer: what it answered, and what it wrote
/// on its standard output.
fn ask(
    scratch: &Scratch,
    github: &GitHub,
    operation: Operation,
    request: &str,
) -> (Result<(), Refused>, String) {
    let config = boomdrive::config::read_layers(
        &scratch.config_file(),
        &scratch.local_file(),
        &scratch.home,
    )
    .expect("a configuration");
    let mut output = Vec::new();
    let answered = forge_token::answer(
        &config,
        operation,
        &mut request.as_bytes(),
        &mut output,
        &forge(github),
    );
    (answered, String::from_utf8(output).expect("words"))
}

/// The answer to a `get` that gave `token`, ending an hour from now.
fn the_answer(output: &str, token: &str) {
    let lines: Vec<&str> = output.lines().collect();
    assert_eq!(output.lines().count(), 3, "{output}");
    assert_eq!(lines[0], "username=x-access-token");
    assert_eq!(lines[1], format!("password={token}"));
    let ends: i64 = lines[2]
        .strip_prefix("password_expiry_utc=")
        .and_then(|ends| ends.parse().ok())
        .expect("an end in seconds");
    let lasting = ends - time::OffsetDateTime::now_utc().unix_timestamp();
    assert!((59 * 60..=60 * 60).contains(&lasting), "{output}");
    assert!(output.ends_with('\n'), "{output}");
}

/// The forge-tokens spec's "A served repository".
#[test]
fn a_get_gives_a_served_repositorys_token_on_standard_output_alone() {
    let scratch = Scratch::new();
    scratch.configure(&naming_the_app(&scratch));
    let github = GitHub::start();
    github.install("agentplot/flume", 7);

    let (answered, output) = ask(
        &scratch,
        &github,
        Operation::Get,
        &for_path("agentplot/flume.git"),
    );

    assert_eq!(answered, Ok(()));
    let tokens = github.tokens();
    assert_eq!(tokens.len(), 1);
    the_answer(&output, &tokens[0]);
    assert_eq!(
        github.calls(),
        [
            "GET /repos/agentplot/flume/installation",
            "POST /app/installations/7/access_tokens"
        ]
    );
}

/// The forge-tokens spec's "A repository the local layer adds".
#[test]
fn a_repository_the_local_layer_adds_is_served() {
    let scratch = Scratch::new();
    scratch.configure(&naming_the_app(&scratch));
    scratch.configure_local("accounts.own.drives.home.repositories = [\"agentplot/sluice\"]\n");
    let github = GitHub::start();
    github.install("agentplot/sluice", 9);

    let (answered, output) = ask(
        &scratch,
        &github,
        Operation::Get,
        &for_path("agentplot/sluice"),
    );

    assert_eq!(answered, Ok(()));
    the_answer(&output, &github.tokens()[0]);
    assert_eq!(
        github.calls(),
        [
            "GET /repos/agentplot/sluice/installation",
            "POST /app/installations/9/access_tokens"
        ]
    );
    assert_eq!(github.requests()[1].body, r#"{"repositories":["sluice"]}"#);
}

/// The forge-tokens spec's "Each get mints".
#[test]
fn each_get_mints() {
    let scratch = Scratch::new();
    scratch.configure(&naming_the_app(&scratch));
    let github = GitHub::start();
    github.install("agentplot/flume", 7);

    let (first, once) = ask(
        &scratch,
        &github,
        Operation::Get,
        &for_path("agentplot/flume"),
    );
    let (second, twice) = ask(
        &scratch,
        &github,
        Operation::Get,
        &for_path("agentplot/flume"),
    );

    assert_eq!((first, second), (Ok(()), Ok(())));
    let tokens = github.tokens();
    assert_eq!(tokens.len(), 2);
    assert_ne!(tokens[0], tokens[1]);
    the_answer(&once, &tokens[0]);
    the_answer(&twice, &tokens[1]);
}

/// A `get` refused with `words`, as a refusal of what was asked, with
/// nothing on standard output.
fn refused_with(asked: (Result<(), Refused>, String), words: &str) {
    assert_eq!(
        asked,
        (
            Err(Refused {
                words: words.to_string(),
                host_to_fix: false,
            }),
            String::new()
        )
    );
}

/// The forge-tokens spec's "A repository no drive serves".
#[test]
fn a_repository_no_drive_serves_is_refused() {
    let scratch = Scratch::new();
    scratch.configure(&naming_the_app(&scratch));
    let github = GitHub::start();
    github.install("someone/else", 7);

    refused_with(
        ask(
            &scratch,
            &github,
            Operation::Get,
            &for_path("someone/else.git"),
        ),
        "someone/else is a repository of no drive this host serves",
    );
    assert!(github.requests().is_empty());
}

/// The forge-tokens spec's "Another host", and another protocol.
#[test]
fn another_host_is_refused() {
    let scratch = Scratch::new();
    scratch.configure(&naming_the_app(&scratch));
    let github = GitHub::start();
    github.install("agentplot/flume", 7);

    refused_with(
        ask(
            &scratch,
            &github,
            Operation::Get,
            "protocol=https\nhost=example.com\npath=agentplot/flume\n\n",
        ),
        "git asked for https://example.com, and the runner gives tokens for https://github.com \
         alone",
    );
    refused_with(
        ask(
            &scratch,
            &github,
            Operation::Get,
            "protocol=http\nhost=github.com\npath=agentplot/flume\n",
        ),
        "git asked for http://github.com, and the runner gives tokens for https://github.com \
         alone",
    );
    let (answered, _) = ask(
        &scratch,
        &github,
        Operation::Get,
        "protocol=https\nhost=GitHub.com\npath=agentplot/flume\n",
    );
    assert_eq!(answered, Ok(()));
}

/// The forge-tokens spec's "No path", and a path that is no address.
#[test]
fn a_request_with_no_path_is_refused() {
    let scratch = Scratch::new();
    scratch.configure(&naming_the_app(&scratch));
    let github = GitHub::start();
    github.install("agentplot/flume", 7);

    refused_with(
        ask(
            &scratch,
            &github,
            Operation::Get,
            "protocol=https\nhost=github.com\n\n",
        ),
        "git named no repository: set credential.useHttpPath so git sends its path",
    );
    refused_with(
        ask(&scratch, &github, Operation::Get, &for_path("agentplot")),
        "agentplot is no repository's address",
    );
    assert!(github.requests().is_empty());
}

/// The forge-tokens spec's "The App is not installed on the repository".
#[test]
fn a_repository_the_app_is_not_installed_on_is_refused() {
    let scratch = Scratch::new();
    scratch.configure(&naming_the_app(&scratch));
    let github = GitHub::start();

    refused_with(
        ask(
            &scratch,
            &github,
            Operation::Get,
            &for_path("agentplot/flume"),
        ),
        "GitHub finds no installation of app 1 on agentplot/flume: install the app on it",
    );
    assert_eq!(github.calls(), ["GET /repos/agentplot/flume/installation"]);
}

/// The forge-tokens spec's "A token erased and asked for again".
#[test]
fn an_erase_revokes_nothing_and_the_next_get_mints_afresh() {
    let scratch = Scratch::new();
    scratch.configure(&naming_the_app(&scratch));
    let github = GitHub::start();
    github.install("agentplot/flume", 7);

    let (_, given) = ask(
        &scratch,
        &github,
        Operation::Get,
        &for_path("agentplot/flume"),
    );
    let erased = github.tokens()[0].clone();
    the_answer(&given, &erased);
    let (answered, output) = ask(
        &scratch,
        &github,
        Operation::Erase,
        &format!(
            "protocol=https\nhost=github.com\npath=agentplot/flume\n\
             username=x-access-token\npassword={erased}\n"
        ),
    );
    assert_eq!((answered, output), (Ok(()), String::new()));
    let (answered, again) = ask(
        &scratch,
        &github,
        Operation::Get,
        &for_path("agentplot/flume"),
    );

    assert_eq!(answered, Ok(()));
    let tokens = github.tokens();
    assert_eq!(tokens.len(), 2);
    assert_ne!(tokens[1], erased);
    the_answer(&again, &tokens[1]);
    assert_eq!(
        github.calls(),
        [
            "GET /repos/agentplot/flume/installation",
            "POST /app/installations/7/access_tokens",
            "GET /repos/agentplot/flume/installation",
            "POST /app/installations/7/access_tokens"
        ]
    );
}

/// Each file under `dir`, by its path beneath it, sorted.
fn files(dir: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let mut walking = vec![dir.to_path_buf()];
    while let Some(at) = walking.pop() {
        let Ok(entries) = std::fs::read_dir(&at) else {
            continue;
        };
        for entry in entries {
            let path = entry.expect("an entry").path();
            if path.is_dir() {
                walking.push(path);
            } else {
                found.push(path.strip_prefix(dir).expect("beneath").to_path_buf());
            }
        }
    }
    found.sort();
    found
}

/// The forge-tokens spec's "Nothing is left behind".
#[test]
fn nothing_is_left_behind() {
    let scratch = Scratch::new();
    scratch.configure(&naming_the_app(&scratch));
    let github = GitHub::start();
    github.install("agentplot/flume", 7);
    let config_dir = scratch.home.join(".config");
    let before = (files(&scratch.data()), files(&config_dir));

    let (answered, _) = ask(
        &scratch,
        &github,
        Operation::Get,
        &for_path("agentplot/flume"),
    );
    assert_eq!(answered, Ok(()));
    github.answering(403, "Resource not accessible by integration");
    let (answered, output) = ask(
        &scratch,
        &github,
        Operation::Get,
        &for_path("agentplot/flume"),
    );

    let refused = answered.expect_err("a refusal");
    assert_eq!(output, "");
    let token = &github.tokens()[0];
    assert!(!refused.words.contains(token.as_str()), "{}", refused.words);
    assert_eq!((files(&scratch.data()), files(&config_dir)), before);
}

/// The runner's binary run as `boomdrive forge-token <args>` on the scratch
/// host, `input` its standard input.
fn run(scratch: &Scratch, args: &[&str], input: &str) -> std::process::Output {
    let mut child = scratch
        .boomdrive()
        .arg("forge-token")
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the runner run");
    child
        .stdin
        .take()
        .expect("its standard input")
        .write_all(input.as_bytes())
        .expect("the request written");
    child.wait_with_output().expect("its answer")
}

/// What `output` exits with, its standard output, which every refusal
/// leaves empty, and its standard error.
fn refusal(output: &std::process::Output) -> (Option<i32>, String, String) {
    (
        output.status.code(),
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

/// The forge-tokens spec's "git asks it to store".
#[test]
fn a_store_is_a_usage_error() {
    let scratch = Scratch::new();
    scratch.configure(&naming_the_app(&scratch));

    let (code, stdout, stderr) = refusal(&run(&scratch, &["store"], &for_path("agentplot/flume")));

    assert_eq!((code, stdout.as_str()), (Some(2), ""));
    assert!(stderr.contains("Usage:"), "{stderr}");
    assert!(stderr.contains("forge-token"), "{stderr}");
}

/// The forge-tokens spec's "No [forge]".
#[test]
fn no_forge_must_be_fixed() {
    let scratch = Scratch::new();
    scratch.configure(&keeping(&["agentplot/flume"]));

    let (code, stdout, stderr) = refusal(&run(&scratch, &["get"], &for_path("agentplot/flume")));

    assert_eq!((code, stdout.as_str()), (Some(2), ""));
    assert_eq!(
        stderr,
        "boomdrive forge-token: the configuration names no [forge], the Boomdrive App's app_id \
         and key_file\n"
    );
}

/// The forge-tokens spec's "A configuration it cannot read".
#[test]
fn a_configuration_it_cannot_read_must_be_fixed() {
    let scratch = Scratch::new();
    let file = scratch.configure("accounts = [\n");

    let (code, stdout, stderr) = refusal(&run(&scratch, &["get"], &for_path("agentplot/flume")));

    assert_eq!((code, stdout.as_str()), (Some(2), ""));
    assert!(
        stderr.starts_with(&format!("boomdrive forge-token: {}", file.display())),
        "{stderr}"
    );
    assert_eq!(stderr.lines().count(), 1, "{stderr}");
}

/// A configuration naming the hosted control plane, keeping
/// `agentplot/flume` in the drive `home`.
fn hosted() -> String {
    format!(
        "{}\n[control_plane]\ntype = \"hosted\"\n\n[control_plane.hosted]\n\
         url = \"https://control.example\"\n\n[[control_plane.hosted.partitions]]\n\
         label = \"home\"\nrepositories = [\"agentplot/flume\"]\n",
        keeping(&["agentplot/flume"])
    )
}

/// The forge-tokens spec's "A get on a cloud host".
#[test]
fn a_get_on_a_cloud_host_is_refused() {
    let scratch = Scratch::new();
    scratch.configure(&hosted());

    let (code, stdout, stderr) = refusal(&run(&scratch, &["get"], &for_path("agentplot/flume")));

    assert_eq!(
        (code, stdout.as_str(), stderr),
        (
            Some(1),
            "",
            format!(
                "boomdrive forge-token: {}\n",
                boomdrive_hosted::NO_FORGE_TOKEN
            )
        )
    );
    assert_eq!(
        boomdrive_hosted::NO_FORGE_TOKEN,
        "the hosted control plane mints this host's forge tokens, and version 2 of its \
         protocol carries none yet"
    );
}

/// The forge-tokens spec's "An erase on a cloud host".
#[test]
fn an_erase_on_a_cloud_host_is_refused() {
    let scratch = Scratch::new();
    scratch.configure(&hosted());

    let (code, stdout, stderr) = refusal(&run(
        &scratch,
        &["erase"],
        "protocol=https\nhost=github.com\npath=agentplot/flume\nusername=x-access-token\n\
         password=ghs_n0t4r34lt0k3n\n",
    ));

    assert_eq!(
        (code, stdout.as_str(), stderr),
        (
            Some(1),
            "",
            format!(
                "boomdrive forge-token: {}\n",
                boomdrive_hosted::NO_FORGE_TOKEN
            )
        )
    );
}

/// The forge-tokens spec's "A repository no drive serves", by the binary,
/// whose `--config` names the file.
#[test]
fn the_binary_refuses_a_repository_no_drive_serves() {
    let scratch = Scratch::new();
    let file = scratch.root.join("elsewhere.toml");
    std::fs::write(&file, naming_the_app(&scratch)).expect("the configuration written");

    let (code, stdout, stderr) = refusal(&run(
        &scratch,
        &["--config", &file.to_string_lossy(), "get"],
        &for_path("someone/else.git"),
    ));

    assert_eq!(
        (code, stdout.as_str(), stderr.as_str()),
        (
            Some(1),
            "",
            "boomdrive forge-token: someone/else is a repository of no drive this host serves\n"
        )
    );
}
