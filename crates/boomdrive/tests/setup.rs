//! `boomdrive setup` through the binary, over stand-in programs (the
//! setup spec; the design of the change a-persons-drive-is-set-up-by-one-command).
//! Every test runs on a `Scratch` home under core.41's neutral names, and
//! no test runs the host's `crew`, `gh`, `herdr`, `hostname`, `launchctl`,
//! `claude` or `pendingyou`: each is a stand-in `sh` script in the scratch
//! `stand` folder, first on the binary's `PATH`, which appends each call's
//! arguments to a calls file and answers what the test has put beside it.
//! Nothing reaches the network. They run on macOS, where setup runs and
//! the gate runs.

#![cfg(target_os = "macos")]

mod common;

use common::Scratch;
use serde_json::{json, Value};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Output;

/// A stand-in program's script: its arguments appended to `<name>.calls`
/// beside it, one call a line, each `HERDR_` variable it was given to
/// `<name>.env`, and the answer put for its first two arguments,
/// `<name>.<first>-<second>.*`, the second up to its first `/`, else its
/// own, `<name>.*`: `.out` on
/// standard output, `.err` on standard error and `.exit` its exit, 0 where
/// none is put.
const SCRIPT: &str = r#"#!/bin/sh
d=$(dirname "$0")
n=$(basename "$0")
printf '%s\n' "$*" >> "$d/$n.calls"
env | grep '^HERDR_' >> "$d/$n.env"
k="$n.$1-${2%%/*}"
[ -e "$d/$k.exit" ] || k="$n"
[ -e "$d/$k.out" ] && cat "$d/$k.out"
[ -e "$d/$k.err" ] && cat "$d/$k.err" >&2
exit "$(cat "$d/$k.exit" 2>/dev/null || echo 0)"
"#;

/// crew's answer to `drive set`, as the stand-in prints it.
const CREWS: &str = r#"{"label":"home","teams":{"outcome":"written"},"state":{"outcome":"made"}}"#;

/// Every answer of the drive `home` of `own`, keeping `agentplot/flume`
/// and `agentplot/sluice`, with `--json`.
const EVERY: [&str; 11] = [
    "--account",
    "own",
    "--drive",
    "home",
    "--state-repository",
    "example/state",
    "--repository",
    "agentplot/flume",
    "--repository",
    "agentplot/sluice",
    "--json",
];

/// A scratch host whose stand-ins answer as a Mac ready for setup:
/// `hostname` `host-a`, gh signed in and no state repository, crew
/// setting the drive, herdr listing no plugin, `launchctl` finding no
/// agent, and Claude Code and Pending You signed in; its profile holds
/// both of the set's plugins.
struct Stand {
    scratch: Scratch,
}

impl Stand {
    fn new() -> Stand {
        let stand = Stand {
            scratch: Scratch::new(),
        };
        std::fs::create_dir_all(stand.folder()).unwrap();
        for program in [
            "hostname",
            "gh",
            "crew",
            "herdr",
            "launchctl",
            "claude",
            "pendingyou",
        ] {
            stand.program(program);
        }
        stand.answer("hostname", None, "host-a\n", "", 0);
        stand.answer(
            "gh",
            Some("auth-status"),
            "",
            "Logged in to github.com\n",
            0,
        );
        stand.answer(
            "gh",
            Some("repo-view"),
            "",
            "GraphQL: Could not resolve to a Repository with the name 'example/state'. (repository)\n",
            1,
        );
        stand.answer(
            "gh",
            Some("repo-create"),
            "https://github.com/example/state\n",
            "",
            0,
        );
        stand.answer("crew", None, &format!("{CREWS}\n"), "", 0);
        stand.listing(&[]);
        stand.answer("herdr", Some("plugin-link"), "linked\n", "", 0);
        stand.answer("launchctl", Some("print-gui"), "", "", 113);
        stand.answer("launchctl", None, "", "", 0);
        stand.answer("claude", None, "{\"loggedIn\":true}\n", "", 0);
        stand.answer(
            "pendingyou",
            None,
            "Pending You for Claude Code\n  ok      Sign-in: this computer's own connection\n",
            "",
            0,
        );
        for plugin in ["flume", "sluice"] {
            std::fs::create_dir_all(stand.plugin(plugin)).unwrap();
        }
        stand
    }

    fn home(&self) -> &Path {
        &self.scratch.home
    }

    /// The stand-ins' folder.
    fn folder(&self) -> PathBuf {
        self.scratch.root.join("stand")
    }

    /// The profile's folder of the plugin `name`.
    fn plugin(&self, name: &str) -> PathBuf {
        self.home()
            .join(".nix-profile/share/herdr/plugins")
            .join(name)
    }

    fn config(&self) -> PathBuf {
        self.scratch.config_file()
    }

    fn aside(&self) -> PathBuf {
        self.config().with_file_name("config.toml.before-setup")
    }

    fn plist(&self) -> PathBuf {
        self.home()
            .join("Library/LaunchAgents/io.github.agentplot.boomdrive.plist")
    }

    fn teams(&self) -> PathBuf {
        self.home().join(".config/crew/teams.json")
    }

    /// Writes the stand-in `name`.
    fn program(&self, name: &str) {
        let path = self.folder().join(name);
        std::fs::write(&path, SCRIPT).unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
    }

    /// Removes the stand-in `name`, so it is not found.
    fn remove(&self, name: &str) {
        std::fs::remove_file(self.folder().join(name)).unwrap();
    }

    /// Puts `name`'s answer for its first two arguments `key`, as
    /// `<first>-<second>`, or its own where `key` is none.
    fn answer(&self, name: &str, key: Option<&str>, out: &str, err: &str, exit: i32) {
        let stem = match key {
            Some(key) => format!("{name}.{key}"),
            None => name.to_string(),
        };
        let folder = self.folder();
        std::fs::write(folder.join(format!("{stem}.out")), out).unwrap();
        std::fs::write(folder.join(format!("{stem}.err")), err).unwrap();
        std::fs::write(folder.join(format!("{stem}.exit")), exit.to_string()).unwrap();
    }

    /// herdr's `plugin list --json` giving a plugin at each of `roots`.
    fn listing(&self, roots: &[&Path]) {
        let plugins: Vec<Value> = roots
            .iter()
            .map(|root| json!({"plugin_id": "a-plugin", "plugin_root": root}))
            .collect();
        let listed = json!({"id": "cli:plugin", "result": {"plugins": plugins}});
        self.answer("herdr", Some("plugin-list"), &format!("{listed}\n"), "", 0);
    }

    /// `launchctl print` finding the agent.
    fn loaded(&self) {
        self.answer("launchctl", Some("print-gui"), "state = running\n", "", 0);
    }

    /// Each call of `name`, its arguments joined by spaces.
    fn calls(&self, name: &str) -> Vec<String> {
        match std::fs::read_to_string(self.folder().join(format!("{name}.calls"))) {
            Ok(text) => text.lines().map(str::to_string).collect(),
            Err(_) => Vec::new(),
        }
    }

    /// Forgets every call made so far.
    fn forget(&self) {
        for entry in std::fs::read_dir(self.folder()).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_some_and(|e| e == "calls" || e == "env") {
                std::fs::remove_file(path).unwrap();
            }
        }
    }

    /// Whether any stand-in was run.
    fn ran_any(&self) -> bool {
        std::fs::read_dir(self.folder()).unwrap().any(|entry| {
            entry
                .unwrap()
                .path()
                .extension()
                .is_some_and(|e| e == "calls")
        })
    }

    /// The binary's `setup` with `args`, on the scratch home, its `PATH`
    /// the stand-ins, `/usr/bin` and `/bin`, and a herdr's variables set,
    /// which setup takes from herdr.
    fn setup(&self, args: &[&str]) -> Output {
        let mut path = self.folder().into_os_string();
        path.push(":/usr/bin:/bin");
        self.scratch
            .command(env!("CARGO_BIN_EXE_boomdrive"))
            .arg("setup")
            .args(args)
            .env("PATH", path)
            .env("HERDR_ENV", "1")
            .env("HERDR_SOCKET_PATH", "/stand-in/herdr.sock")
            .output()
            .unwrap()
    }

    /// The `{"set_up": …}` of a setup that exited 0.
    fn set_up(&self, args: &[&str]) -> Value {
        let output = self.setup(args);
        assert_eq!(output.status.code(), Some(0), "{}", said(&output));
        let answer: Value = serde_json::from_slice(&output.stdout).unwrap();
        answer["set_up"].clone()
    }
}

/// What a run said, for a failing assertion.
fn said(output: &Output) -> String {
    format!(
        "stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

/// The reason of a refusal in JSON, its exit `exit`.
fn refused(output: &Output, exit: i32) -> String {
    assert_eq!(output.status.code(), Some(exit), "{}", said(output));
    let answer: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(answer.as_object().map(|o| o.len()), Some(1), "{answer}");
    answer["refused"]["refusal"]["reason"]
        .as_str()
        .unwrap_or_else(|| panic!("no reason: {answer}"))
        .to_string()
}

/// The login's uid, as `id -u` gives it.
fn uid() -> String {
    let output = std::process::Command::new("id").arg("-u").output().unwrap();
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}

/// Asserts that no file setup writes is there, and no state repository
/// was made and no crew run.
fn nothing_written(stand: &Stand) {
    assert!(!stand.config().exists(), "the runner's file is there");
    assert!(!stand.plist().exists(), "the agent's plist is there");
    assert!(!stand.teams().exists(), "crew's file is there");
    assert!(
        !stand
            .calls("gh")
            .iter()
            .any(|call| call.starts_with("repo")),
        "{:?}",
        stand.calls("gh")
    );
    assert!(stand.calls("crew").is_empty(), "{:?}", stand.calls("crew"));
    assert!(stand.calls("launchctl").is_empty());
}

/// The text of a file past its first line.
fn past_first_line(text: &str) -> &str {
    text.split_once('\n').map_or("", |(_, rest)| rest)
}

/// The setup spec's "The runner's file is Nix's".
#[test]
fn the_runners_file_is_nixs() {
    let stand = Stand::new();
    let target = stand.scratch.root.join("store-config.toml");
    std::fs::write(&target, "[accounts]\n").unwrap();
    std::fs::create_dir_all(stand.config().parent().unwrap()).unwrap();
    std::os::unix::fs::symlink(&target, stand.config()).unwrap();
    let reason = refused(&stand.setup(&EVERY), 1);
    let file = stand.config().display().to_string();
    for words in [file.as_str(), "`services.boomdrive.accounts`"] {
        assert!(reason.contains(words), "{reason}");
    }
    assert_eq!(std::fs::read_link(stand.config()).unwrap(), target);
    assert!(!stand.teams().exists());
    assert!(!stand.home().join("Library/LaunchAgents").exists());
    assert!(!stand.ran_any(), "a program was run");
}

/// The setup spec's "crew missing".
#[test]
fn crew_missing() {
    let stand = Stand::new();
    stand.remove("crew");
    let reason = refused(&stand.setup(&EVERY), 2);
    for words in ["`crew`", "github:agentplot/boomdrive#crew"] {
        assert!(reason.contains(words), "{reason}");
    }
    assert!(stand.calls("gh").is_empty(), "{:?}", stand.calls("gh"));
    nothing_written(&stand);
}

/// The setup spec's "GitHub not signed in".
#[test]
fn github_not_signed_in() {
    let stand = Stand::new();
    stand.answer(
        "gh",
        Some("auth-status"),
        "",
        "You are not logged into any GitHub hosts.\n",
        1,
    );
    let reason = refused(&stand.setup(&EVERY), 2);
    for words in ["GitHub's sign-in", "`gh auth login`"] {
        assert!(reason.contains(words), "{reason}");
    }
    assert_eq!(stand.calls("gh"), ["auth status --hostname github.com"]);
    nothing_written(&stand);
}

/// The setup spec's "Both files there".
#[test]
fn both_files_there() {
    let stand = Stand::new();
    let own = "fetch_interval = \"5m\"\n[accounts]\n";
    let before = "[accounts]\n";
    stand.scratch.configure(own);
    std::fs::write(stand.aside(), before).unwrap();
    let reason = refused(&stand.setup(&EVERY), 2);
    let (file, aside) = (
        stand.config().display().to_string(),
        stand.aside().display().to_string(),
    );
    for words in [file.as_str(), aside.as_str()] {
        assert!(reason.contains(words), "{reason}");
    }
    assert_eq!(std::fs::read_to_string(stand.config()).unwrap(), own);
    assert_eq!(std::fs::read_to_string(stand.aside()).unwrap(), before);
    assert!(stand.calls("gh").is_empty(), "{:?}", stand.calls("gh"));
    assert!(!stand.plist().exists());
    assert!(stand.calls("crew").is_empty());
}

/// The setup spec's "Claude Code not signed in".
#[test]
fn claude_code_not_signed_in() {
    let stand = Stand::new();
    stand.answer("claude", None, "{\"loggedIn\":false}\n", "", 1);
    let answer = stand.set_up(&EVERY);
    assert_eq!(
        answer["missing"],
        json!([{"what": "Claude Code's sign-in", "fix": "claude auth login"}])
    );
    assert_eq!(stand.calls("claude"), ["auth status --json"]);
}

/// The setup spec's "Pending You not installed".
#[test]
fn pending_you_not_installed() {
    let stand = Stand::new();
    stand.remove("pendingyou");
    let answer = stand.set_up(&EVERY);
    assert_eq!(
        answer["missing"],
        json!([{"what": "Pending You's sign-in", "fix": "npx -y pendingyou@latest init"}])
    );
}

/// The setup spec's "One drive under crew": the runner's file, past its
/// first line, is the file Nix's module renders for the same drive.
#[test]
fn one_drive_under_crew_is_the_modules_file() {
    let stand = Stand::new();
    stand.set_up(&EVERY);
    let written = std::fs::read_to_string(stand.config()).unwrap();
    let modules = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../nix/tests/files/setup.toml"),
    )
    .unwrap();
    assert_eq!(
        written.lines().next(),
        Some("# Written by boomdrive setup; a change goes through boomdrive setup.")
    );
    assert_eq!(past_first_line(&written), past_first_line(&modules));
}

/// The setup spec's "A second drive".
#[test]
fn a_second_drive_keeps_the_first() {
    let stand = Stand::new();
    stand.set_up(&EVERY);
    stand.set_up(&[
        "--account",
        "own",
        "--drive",
        "work",
        "--state-repository",
        "example/work",
        "--repository",
        "example/app",
        "--json",
    ]);
    let file: toml::Table =
        toml::from_str(&std::fs::read_to_string(stand.config()).unwrap()).unwrap();
    let want: toml::Table = toml::from_str(
        "[accounts.own.drives.home]\n\
         repositories = [\"agentplot/flume\", \"agentplot/sluice\"]\n\
         [accounts.own.drives.work]\n\
         repositories = [\"example/app\"]\n\
         [control_plane]\n\
         type = \"crew\"\n\
         [control_plane.crew]\n\
         host = \"host-a\"\n\
         [[control_plane.crew.partitions]]\n\
         label = \"home\"\n\
         state_repository = \"example/state\"\n\
         [[control_plane.crew.partitions]]\n\
         label = \"work\"\n\
         state_repository = \"example/work\"\n",
    )
    .unwrap();
    assert_eq!(file, want);
}

/// The setup spec's "Nothing changed".
#[test]
fn nothing_changed_writes_nothing() {
    let stand = Stand::new();
    stand.set_up(&EVERY);
    let modified = std::fs::metadata(stand.config())
        .unwrap()
        .modified()
        .unwrap();
    let text = std::fs::read_to_string(stand.config()).unwrap();
    let answer = stand.set_up(&EVERY);
    assert_eq!(answer["config"]["outcome"], json!("unchanged"));
    assert_eq!(
        std::fs::metadata(stand.config())
            .unwrap()
            .modified()
            .unwrap(),
        modified
    );
    assert_eq!(std::fs::read_to_string(stand.config()).unwrap(), text);
}

/// The setup spec's "A local layer beside it".
#[test]
fn a_local_layer_beside_it_is_left() {
    let stand = Stand::new();
    let local = "[accounts.own.drives.home]\nrepositories = [\"example/app\"]\n";
    stand.scratch.configure_local(local);
    stand.set_up(&EVERY);
    assert_eq!(
        std::fs::read_to_string(stand.scratch.local_file()).unwrap(),
        local
    );
}

/// The setup spec's "A person's own file".
#[test]
fn a_persons_own_file_is_moved_aside() {
    let stand = Stand::new();
    let own = "fetch_interval = \"5m\"\n[accounts]\n";
    stand.scratch.configure(own);
    let answer = stand.set_up(&EVERY);
    assert_eq!(std::fs::read_to_string(stand.aside()).unwrap(), own);
    let written = std::fs::read_to_string(stand.config()).unwrap();
    assert!(
        written.starts_with("# Written by boomdrive setup;"),
        "{written}"
    );
    assert_eq!(
        answer["config"],
        json!({
            "file": stand.config().display().to_string(),
            "outcome": "written",
            "aside": stand.aside().display().to_string(),
        })
    );
}

/// The setup spec's "crew's flags", through the binary.
#[test]
fn crews_flags_through_setup() {
    let stand = Stand::new();
    stand.set_up(&EVERY);
    let home = stand.home().display().to_string();
    let mut want = format!(
        "drive set home --json --state example/state --blueprints agentplot/flume \
         --machine host-a --hosts host-a host-a darwin --session home --folder {home}/Code \
         --repos agentplot/flume --repos agentplot/sluice"
    );
    for role in ["design", "planner", "reviewer", "ops", "operator"] {
        want.push_str(&format!(" --accounts {role} own"));
    }
    for role in ["design", "planner", "reviewer", "ops", "operator"] {
        want.push_str(&format!(" --runs {role} standing"));
    }
    want.push_str(" --review 5 4");
    for repository in ["agentplot/flume", "agentplot/sluice"] {
        want.push_str(&format!(
            " --repositories {repository} {home}/.local/share/boomdrive/repositories/{repository} \
             {home}/Code/{repository}"
        ));
    }
    assert_eq!(stand.calls("crew"), [want]);
    for flag in ["--flume", "--roles", "--console"] {
        assert!(!stand.calls("crew")[0].contains(flag));
    }
}

/// The setup spec's "crew refuses".
#[test]
fn crews_refusal_writes_no_runner_file() {
    let stand = Stand::new();
    stand.answer(
        "crew",
        None,
        "",
        "the teams file crew drive set would write is refused\n",
        1,
    );
    let reason = refused(&stand.setup(&EVERY), 1);
    assert!(
        reason.contains("the teams file crew drive set would write is refused"),
        "{reason}"
    );
    assert!(!stand.config().exists());
    assert!(!stand.plist().exists());
    assert!(stand.calls("launchctl").is_empty());
    assert!(stand.calls("herdr").is_empty());
}

/// The setup spec's "Both plugins linked", herdr given no `HERDR_`
/// variable.
#[test]
fn both_plugins_linked() {
    let stand = Stand::new();
    let answer = stand.set_up(&EVERY);
    let (flume, sluice) = (stand.plugin("flume"), stand.plugin("sluice"));
    assert_eq!(
        stand.calls("herdr"),
        [
            "plugin list --json".to_string(),
            format!("plugin link {}", flume.display()),
            format!("plugin link {}", sluice.display()),
        ]
    );
    assert_eq!(
        answer["plugins"],
        json!([
            {"plugin": "flume", "folder": flume.display().to_string(), "outcome": "linked"},
            {"plugin": "sluice", "folder": sluice.display().to_string(), "outcome": "linked"},
        ])
    );
    let seen = std::fs::read_to_string(stand.folder().join("herdr.env")).unwrap_or_default();
    assert_eq!(seen, "", "herdr was given {seen}");
    assert_eq!(answer["missing"], json!([]));
}

/// The setup spec's "A plugin already linked", and a link herdr refuses
/// named with its words.
#[test]
fn a_plugin_already_linked_is_left() {
    let stand = Stand::new();
    stand.listing(&[&stand.plugin("flume")]);
    stand.answer(
        "herdr",
        Some("plugin-link"),
        "",
        "plugin sluice needs herdr 0.9\n",
        1,
    );
    let answer = stand.set_up(&EVERY);
    let sluice = stand.plugin("sluice");
    assert_eq!(
        stand.calls("herdr"),
        [
            "plugin list --json".to_string(),
            format!("plugin link {}", sluice.display()),
        ]
    );
    assert_eq!(answer["plugins"][0]["outcome"], json!("there"));
    assert_eq!(answer["plugins"][1]["outcome"], json!("refused"));
    assert_eq!(
        answer["plugins"][1]["words"],
        json!("plugin sluice needs herdr 0.9")
    );
}

/// The setup spec's "No plugin in the profile".
#[test]
fn no_plugin_in_the_profile() {
    let stand = Stand::new();
    std::fs::remove_dir_all(stand.home().join(".nix-profile")).unwrap();
    let answer = stand.set_up(&EVERY);
    assert!(
        !stand
            .calls("herdr")
            .iter()
            .any(|call| call.starts_with("plugin link")),
        "{:?}",
        stand.calls("herdr")
    );
    assert_eq!(answer["plugins"], json!([]));
    assert_eq!(
        answer["missing"],
        json!([{
            "what": "the set's herdr plugins",
            "fix": "nix profile add github:agentplot/boomdrive#herdr-plugins"
        }])
    );
}

/// The setup spec's "First loaded".
#[test]
fn first_loaded() {
    let stand = Stand::new();
    let answer = stand.set_up(&EVERY);
    let uid = uid();
    assert_eq!(
        stand.calls("launchctl"),
        [
            format!("print gui/{uid}/io.github.agentplot.boomdrive"),
            format!("bootstrap gui/{uid} {}", stand.plist().display()),
        ]
    );
    assert_eq!(
        answer["service"],
        json!({
            "file": stand.plist().display().to_string(),
            "plist": "written",
            "outcome": "loaded",
        })
    );
    let plist = std::fs::read_to_string(stand.plist()).unwrap();
    assert!(plist.contains("<string>io.github.agentplot.boomdrive</string>"));
}

/// The setup spec's "Loaded and unchanged".
#[test]
fn loaded_and_unchanged() {
    let stand = Stand::new();
    stand.set_up(&EVERY);
    let modified = std::fs::metadata(stand.plist())
        .unwrap()
        .modified()
        .unwrap();
    stand.loaded();
    stand.forget();
    let answer = stand.set_up(&EVERY);
    assert_eq!(
        stand.calls("launchctl"),
        [format!("print gui/{}/io.github.agentplot.boomdrive", uid())]
    );
    assert_eq!(answer["service"]["plist"], json!("unchanged"));
    assert_eq!(answer["service"]["outcome"], json!("running"));
    assert_eq!(
        std::fs::metadata(stand.plist())
            .unwrap()
            .modified()
            .unwrap(),
        modified
    );
}

/// The setup spec's "Changed and loaded", and a `launchctl` refusal
/// carried with its words.
#[test]
fn a_changed_plist_is_booted_out_first() {
    let stand = Stand::new();
    std::fs::create_dir_all(stand.plist().parent().unwrap()).unwrap();
    std::fs::write(stand.plist(), "an older agent\n").unwrap();
    stand.loaded();
    let answer = stand.set_up(&EVERY);
    let uid = uid();
    assert_eq!(
        stand.calls("launchctl"),
        [
            format!("print gui/{uid}/io.github.agentplot.boomdrive"),
            format!("bootout gui/{uid}/io.github.agentplot.boomdrive"),
            format!("bootstrap gui/{uid} {}", stand.plist().display()),
        ]
    );
    assert_eq!(answer["service"]["outcome"], json!("reloaded"));

    let stand = Stand::new();
    stand.answer(
        "launchctl",
        Some("bootstrap-gui"),
        "",
        "Bootstrap failed: 5: Input/output error\n",
        5,
    );
    let reason = refused(&stand.setup(&EVERY), 1);
    assert!(reason.contains("Input/output error"), "{reason}");
}

/// The setup spec's "The answer", and its words without `--json`, ending
/// with the command that starts crew's agents.
#[test]
fn the_answer() {
    let stand = Stand::new();
    let answer = stand.set_up(&EVERY);
    let crews: Value = serde_json::from_str(CREWS).unwrap();
    assert_eq!(
        answer,
        json!({
            "account": "own",
            "drive": "home",
            "repositories": ["agentplot/flume", "agentplot/sluice"],
            "control_plane": {
                "state_repository": {"repository": "example/state", "outcome": "made"},
                "drive": crews,
                "start": "crew main up home",
            },
            "config": {
                "file": stand.config().display().to_string(),
                "outcome": "written",
                "aside": null,
            },
            "service": {
                "file": stand.plist().display().to_string(),
                "plist": "written",
                "outcome": "loaded",
            },
            "plugins": [
                {"plugin": "flume", "folder": stand.plugin("flume").display().to_string(), "outcome": "linked"},
                {"plugin": "sluice", "folder": stand.plugin("sluice").display().to_string(), "outcome": "linked"},
            ],
            "missing": [],
        })
    );

    let stand = Stand::new();
    let output = stand.setup(&EVERY[..EVERY.len() - 1]);
    assert_eq!(output.status.code(), Some(0), "{}", said(&output));
    let words = String::from_utf8(output.stdout).unwrap();
    assert!(words.lines().count() > 3, "{words}");
    assert_eq!(words.lines().last(), Some("start: crew main up home"));
}

/// The setup spec's "Run again".
#[test]
fn run_again_changes_nothing() {
    let stand = Stand::new();
    stand.set_up(&EVERY);
    let first_crew = stand.calls("crew");
    let modified = |path: PathBuf| std::fs::metadata(path).unwrap().modified().unwrap();
    let (config, plist) = (modified(stand.config()), modified(stand.plist()));
    stand.answer("gh", Some("repo-view"), "{\"name\":\"state\"}\n", "", 0);
    stand.listing(&[&stand.plugin("flume"), &stand.plugin("sluice")]);
    stand.loaded();
    stand.forget();
    let answer = stand.set_up(&EVERY);
    assert_eq!(
        stand.calls("gh"),
        [
            "auth status --hostname github.com",
            "repo view example/state --json name"
        ]
    );
    assert_eq!(stand.calls("crew"), first_crew);
    assert_eq!(stand.calls("herdr"), ["plugin list --json"]);
    assert_eq!(
        stand.calls("launchctl"),
        [format!("print gui/{}/io.github.agentplot.boomdrive", uid())]
    );
    assert_eq!(modified(stand.config()), config);
    assert_eq!(modified(stand.plist()), plist);
    assert_eq!(
        answer["control_plane"]["state_repository"]["outcome"],
        json!("there")
    );
    assert_eq!(answer["config"]["outcome"], json!("unchanged"));
    assert_eq!(answer["service"]["outcome"], json!("running"));
}

/// The setup spec's "A refusal in JSON".
#[test]
fn a_refusal_in_json() {
    let stand = Stand::new();
    let reason = refused(
        &stand.setup(&[
            "--json",
            "--account",
            "Own",
            "--drive",
            "home",
            "--state-repository",
            "example/state",
            "--repository",
            "agentplot/flume",
        ]),
        2,
    );
    assert!(reason.contains("Own"), "{reason}");
    assert!(!stand.ran_any());
}

/// The setup spec's "Every answer given".
#[test]
fn every_answer_given_asks_nothing() {
    let stand = Stand::new();
    let output = stand.setup(&EVERY);
    assert_eq!(output.status.code(), Some(0), "{}", said(&output));
    assert_eq!(String::from_utf8_lossy(&output.stderr), "");
    let answer: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(answer["set_up"]["account"], json!("own"));
    assert_eq!(answer["set_up"]["drive"], json!("home"));
    assert_eq!(
        answer["set_up"]["repositories"],
        json!(["agentplot/flume", "agentplot/sluice"])
    );
}

/// The setup spec's "A repository added".
#[test]
fn a_repository_added_is_kept_and_given_crew() {
    let stand = Stand::new();
    stand.set_up(&EVERY);
    stand.forget();
    let answer = stand.set_up(&["--add-repository", "agentplot/crew", "--json"]);
    assert_eq!(
        answer["repositories"],
        json!(["agentplot/flume", "agentplot/sluice", "agentplot/crew"])
    );
    let crew = stand.calls("crew");
    assert_eq!(crew.len(), 1);
    for repository in ["agentplot/flume", "agentplot/sluice", "agentplot/crew"] {
        assert!(
            crew[0].contains(&format!("--repos {repository} ")),
            "{}",
            crew[0]
        );
        assert!(
            crew[0].contains(&format!("--repositories {repository} ")),
            "{}",
            crew[0]
        );
    }
    assert!(crew[0].contains("--state example/state "), "{}", crew[0]);
    let file = std::fs::read_to_string(stand.config()).unwrap();
    assert!(
        file.contains(
            "repositories = [\"agentplot/flume\", \"agentplot/sluice\", \"agentplot/crew\"]"
        ),
        "{file}"
    );
}

/// The setup spec's "A repository removed": crew and the runner's file
/// keep the one left, and the removed one's clone and view are left for
/// the runner to let go by its own rules.
#[test]
fn a_repository_removed_leaves_both() {
    let stand = Stand::new();
    stand.set_up(&EVERY);
    let clone = stand.scratch.container("agentplot/sluice");
    let view = stand.home().join("Code/agentplot/sluice");
    for folder in [&clone, &view] {
        std::fs::create_dir_all(folder).unwrap();
    }
    stand.forget();
    let answer = stand.set_up(&["--remove-repository", "agentplot/sluice", "--json"]);
    assert_eq!(answer["repositories"], json!(["agentplot/flume"]));
    let crew = stand.calls("crew");
    assert_eq!(crew.len(), 1);
    assert!(crew[0].contains("--repos agentplot/flume "), "{}", crew[0]);
    assert!(!crew[0].contains("agentplot/sluice"), "{}", crew[0]);
    let file = std::fs::read_to_string(stand.config()).unwrap();
    assert!(
        file.contains("[accounts.own.drives.home]\nrepositories = [\"agentplot/flume\"]\n"),
        "{file}"
    );
    assert!(clone.is_dir() && view.is_dir());
}
