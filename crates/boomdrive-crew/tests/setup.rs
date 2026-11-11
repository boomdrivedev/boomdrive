//! crew's side of a drive's setup over stand-in programs (the setup spec;
//! design Decisions 4, 5 and 11 of the change
//! a-persons-drive-is-set-up-by-one-command): `hostname`, `gh` and `crew`
//! are `sh` scripts in a scratch `bin`, each appending its arguments to a
//! calls file and answering what the test put beside it, so no test runs
//! the host's programs or reaches the network. Every test runs on a
//! scratch home, under core.41's neutral names.

#![cfg(unix)]

use boomdrive_crew::setup::{self, Asked, Held, Kept, Prepared, Settled};
use serde_json::json;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use tempfile::TempDir;

/// A stand-in program's script: its arguments appended to `<name>.calls`
/// beside it, one call a line, and the answer put for its first two
/// arguments, `<name>.<first>-<second>.*`, else its own, `<name>.*`:
/// `.out` on standard output, `.err` on standard error and `.exit` its
/// exit, 0 where none is put.
const SCRIPT: &str = r#"#!/bin/sh
d=$(dirname "$0")
n=$(basename "$0")
printf '%s\n' "$*" >> "$d/$n.calls"
k="$n.$1-$2"
[ -e "$d/$k.exit" ] || k="$n"
[ -e "$d/$k.out" ] && cat "$d/$k.out"
[ -e "$d/$k.err" ] && cat "$d/$k.err" >&2
exit "$(cat "$d/$k.exit" 2>/dev/null || echo 0)"
"#;

/// A scratch home with a `bin` of stand-in programs.
struct Stand {
    dir: TempDir,
}

impl Stand {
    /// A scratch home whose `bin` holds `hostname`, answering `host-a`,
    /// `gh`, whose `repo view` finds the repository, and `crew`, answering
    /// crew's JSON.
    fn new() -> Stand {
        let stand = Stand {
            dir: tempfile::tempdir().unwrap(),
        };
        std::fs::create_dir_all(stand.bin()).unwrap();
        for program in ["hostname", "gh", "crew"] {
            stand.program(program);
        }
        stand.answer("hostname", None, "host-a\n", "", 0);
        stand.answer("gh", Some("repo-view"), "{\"name\":\"state\"}\n", "", 0);
        stand.answer(
            "crew",
            None,
            "{\"label\":\"home\",\"teams\":{\"outcome\":\"written\"},\"state\":{\"outcome\":\"made\"}}\n",
            "",
            0,
        );
        stand
    }

    fn home(&self) -> PathBuf {
        self.dir.path().join("home")
    }

    fn bin(&self) -> PathBuf {
        self.dir.path().join("bin")
    }

    /// Writes the stand-in `name`.
    fn program(&self, name: &str) {
        let path = self.bin().join(name);
        std::fs::write(&path, SCRIPT).unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
    }

    /// Removes the stand-in `name`, so it is not found.
    fn remove(&self, name: &str) {
        std::fs::remove_file(self.bin().join(name)).unwrap();
    }

    /// Puts `name`'s answer for its first two arguments `key`, as
    /// `<first>-<second>`, or its own where `key` is none.
    fn answer(&self, name: &str, key: Option<&str>, out: &str, err: &str, exit: i32) {
        let stem = match key {
            Some(key) => format!("{name}.{key}"),
            None => name.to_string(),
        };
        let bin = self.bin();
        std::fs::write(bin.join(format!("{stem}.out")), out).unwrap();
        std::fs::write(bin.join(format!("{stem}.err")), err).unwrap();
        std::fs::write(bin.join(format!("{stem}.exit")), exit.to_string()).unwrap();
    }

    /// Each call of `name`, its arguments joined by spaces.
    fn calls(&self, name: &str) -> Vec<String> {
        match std::fs::read_to_string(self.bin().join(format!("{name}.calls"))) {
            Ok(text) => text.lines().map(str::to_string).collect(),
            Err(_) => Vec::new(),
        }
    }

    /// The finder setup hands crew's side: a program in the scratch `bin`
    /// alone.
    fn find(&self) -> impl Fn(&str) -> Option<PathBuf> {
        let bin = self.bin();
        move |name| Some(bin.join(name)).filter(|path| path.is_file())
    }

    /// The drive `drive` keeping `repositories`, under the scratch home.
    fn kept(&self, repositories: &[&str]) -> Vec<Kept> {
        let home = self.home();
        repositories
            .iter()
            .map(|repository| Kept {
                repository: repository.to_string(),
                dir: home
                    .join(".local/share/boomdrive/repositories")
                    .join(repository),
                checkout: home.join("Code").join(repository),
            })
            .collect()
    }

    fn prepared(&self) -> Prepared {
        setup::prepare(&self.find()).unwrap()
    }

    /// The drive `drive` set up at the state repository `state`, keeping
    /// `repositories`, beside crew's table `table`.
    fn set_up(
        &self,
        drive: &str,
        state: &str,
        repositories: &[&str],
        table: Option<&toml::Table>,
    ) -> Result<setup::SetUp, setup::Refusal> {
        let settled = settled(state, repositories);
        setup::set_up(
            &self.prepared(),
            &settled,
            drive,
            "own",
            &self.home().join("Code"),
            &self.kept(repositories),
            table,
        )
    }
}

/// The answers of a first setup at `state`, keeping `repositories`.
fn settled(state: &str, repositories: &[&str]) -> Settled {
    let asked = Asked {
        state_repository: Some(state.to_string()),
        ..Asked::default()
    };
    let repositories: Vec<String> = repositories.iter().map(|r| r.to_string()).collect();
    setup::settle(&asked, &Held::default(), &repositories).unwrap()
}

/// gh saying it cannot resolve `example/state`.
fn not_there(stand: &Stand) {
    stand.answer(
        "gh",
        Some("repo-view"),
        "",
        "GraphQL: Could not resolve to a Repository with the name 'example/state'. (repository)\n",
        1,
    );
}

/// The setup spec's "No state repository".
#[test]
fn no_state_repository_is_made_private() {
    let stand = Stand::new();
    not_there(&stand);
    stand.answer(
        "gh",
        Some("repo-create"),
        "https://github.com/example/state\n",
        "",
        0,
    );
    let done = stand
        .set_up("home", "example/state", &["agentplot/flume"], None)
        .unwrap();
    assert_eq!(
        stand.calls("gh"),
        [
            "repo view example/state --json name",
            "repo create example/state --private"
        ]
    );
    assert_eq!(
        done.answer["state_repository"],
        json!({"repository": "example/state", "outcome": "made"})
    );
    assert_eq!(stand.calls("crew").len(), 1);
}

/// The setup spec's "The state repository there".
#[test]
fn a_state_repository_there_is_left() {
    let stand = Stand::new();
    let done = stand
        .set_up("home", "example/state", &["agentplot/flume"], None)
        .unwrap();
    assert_eq!(stand.calls("gh"), ["repo view example/state --json name"]);
    assert_eq!(
        done.answer["state_repository"],
        json!({"repository": "example/state", "outcome": "there"})
    );
}

/// The setup spec's "The forge refuses", and a view gh refuses for
/// another reason.
#[test]
fn the_forges_refusal_is_carried() {
    let stand = Stand::new();
    not_there(&stand);
    stand.answer(
        "gh",
        Some("repo-create"),
        "",
        "HTTP 403: Resource not accessible by integration\n",
        1,
    );
    let refusal = stand
        .set_up("home", "example/state", &["agentplot/flume"], None)
        .unwrap_err();
    assert!(!refusal.host_to_fix);
    assert!(refusal.words.contains("HTTP 403"), "{}", refusal.words);
    assert!(stand.calls("crew").is_empty());

    let stand = Stand::new();
    stand.answer(
        "gh",
        Some("repo-view"),
        "",
        "error connecting to api.github.com\n",
        1,
    );
    let refusal = stand
        .set_up("home", "example/state", &["agentplot/flume"], None)
        .unwrap_err();
    assert!(!refusal.host_to_fix);
    assert!(
        refusal.words.contains("error connecting"),
        "{}",
        refusal.words
    );
    assert_eq!(stand.calls("gh"), ["repo view example/state --json name"]);
    assert!(stand.calls("crew").is_empty());
}

/// The setup spec's "crew's answer carried", with the command that
/// starts crew's agents, and crew run once with task 3.2's arguments.
#[test]
fn crews_answer_is_carried_as_crew_printed_it() {
    let stand = Stand::new();
    let done = stand
        .set_up(
            "home",
            "example/state",
            &["agentplot/flume", "agentplot/sluice"],
            None,
        )
        .unwrap();
    assert_eq!(
        done.answer["drive"],
        json!({"label": "home", "teams": {"outcome": "written"}, "state": {"outcome": "made"}})
    );
    assert_eq!(done.answer["start"], json!("crew main up home"));
    assert_eq!(done.start, "crew main up home");
    let settled = settled("example/state", &["agentplot/flume", "agentplot/sluice"]);
    let args = setup::drive_set(
        &settled,
        "home",
        "own",
        "host-a",
        &stand.home().join("Code"),
        &stand.kept(&["agentplot/flume", "agentplot/sluice"]),
    );
    let args: Vec<&str> = args.iter().map(|arg| arg.to_str().unwrap()).collect();
    assert_eq!(stand.calls("crew"), [args.join(" ")]);
}

/// The setup spec's "crew refuses".
#[test]
fn crews_refusal_is_carried() {
    let stand = Stand::new();
    stand.answer(
        "crew",
        None,
        "",
        "the teams file crew drive set would write is refused\n",
        1,
    );
    let refusal = stand
        .set_up("home", "example/state", &["agentplot/flume"], None)
        .unwrap_err();
    assert!(!refusal.host_to_fix);
    assert!(
        refusal
            .words
            .contains("the teams file crew drive set would write is refused"),
        "{}",
        refusal.words
    );
}

/// The setup spec's "A host name crew refuses", and "crew missing" on
/// crew's side.
#[test]
fn a_host_name_crew_refuses() {
    let stand = Stand::new();
    stand.answer("hostname", None, "Pats-MacBook-Air\n", "", 0);
    let refusal = setup::prepare(&stand.find()).unwrap_err();
    assert!(refusal.host_to_fix);
    for words in ["Pats-MacBook-Air", "sudo scutil --set HostName"] {
        assert!(refusal.words.contains(words), "{}", refusal.words);
    }
    assert_eq!(stand.calls("hostname"), ["-s"]);
    assert!(stand.calls("gh").is_empty());
    assert!(stand.calls("crew").is_empty());

    let stand = Stand::new();
    stand.remove("crew");
    let refusal = setup::prepare(&stand.find()).unwrap_err();
    assert!(refusal.host_to_fix);
    for words in ["`crew`", "github:agentplot/boomdrive#crew"] {
        assert!(refusal.words.contains(words), "{}", refusal.words);
    }
    assert!(stand.calls("hostname").is_empty());
}

/// crew's table names each drive set up, in the order set up, with this
/// host, a drive set up again keeping its place.
#[test]
fn the_table_names_each_drive_set_up() {
    let stand = Stand::new();
    let home = stand
        .set_up("home", "example/state", &["agentplot/flume"], None)
        .unwrap();
    let work = stand
        .set_up("work", "example/work", &["example/app"], Some(&home.table))
        .unwrap();
    let again = stand
        .set_up(
            "home",
            "example/state",
            &["agentplot/flume"],
            Some(&work.table),
        )
        .unwrap();
    let want: toml::Table = toml::from_str(
        "host = \"host-a\"\n\
         [[partitions]]\nlabel = \"home\"\nstate_repository = \"example/state\"\n\
         [[partitions]]\nlabel = \"work\"\nstate_repository = \"example/work\"\n",
    )
    .unwrap();
    assert_eq!(work.table, want);
    assert_eq!(again.table, want);
    let first: toml::Table = toml::from_str(
        "host = \"host-a\"\n\
         [[partitions]]\nlabel = \"home\"\nstate_repository = \"example/state\"\n",
    )
    .unwrap();
    assert_eq!(home.table, first);
}
