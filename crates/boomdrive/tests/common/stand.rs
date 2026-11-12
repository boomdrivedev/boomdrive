//! The stand-ins of a Mac's programs that `boomdrive setup` runs: `gh`,
//! `crew`, `herdr`, `launchctl`, `hostname`, `claude` and `pendingyou`,
//! each a `sh` script in the scratch `stand` folder, which notes each
//! call's arguments and answers what the test has put beside it (the
//! setup spec; design Decision 5 of the change
//! boomdrives-quick-start-takes-a-stranger-to-a-working-drive). setup's
//! tests and the quick start's write the same ones. No test runs the
//! host's own, and nothing reaches the network.

use super::Scratch;
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
pub const CREWS: &str =
    r#"{"label":"home","teams":{"outcome":"written"},"state":{"outcome":"made"}}"#;

/// A scratch host whose stand-ins answer as a Mac ready for setup:
/// `hostname` `host-a`, gh signed in and no state repository, crew
/// setting the drive, herdr listing no plugin, `launchctl` finding no
/// agent, and Claude Code and Pending You signed in; its profile holds
/// both of the set's plugins.
pub struct Stand {
    pub scratch: Scratch,
}

impl Stand {
    pub fn new() -> Stand {
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

    pub fn home(&self) -> &Path {
        &self.scratch.home
    }

    /// The stand-ins' folder.
    pub fn folder(&self) -> PathBuf {
        self.scratch.root.join("stand")
    }

    /// The profile's folder of the plugin `name`.
    pub fn plugin(&self, name: &str) -> PathBuf {
        self.home()
            .join(".nix-profile/share/herdr/plugins")
            .join(name)
    }

    pub fn config(&self) -> PathBuf {
        self.scratch.config_file()
    }

    pub fn aside(&self) -> PathBuf {
        self.config().with_file_name("config.toml.before-setup")
    }

    pub fn plist(&self) -> PathBuf {
        self.home()
            .join("Library/LaunchAgents/io.github.agentplot.boomdrive.plist")
    }

    pub fn teams(&self) -> PathBuf {
        self.home().join(".config/crew/teams.json")
    }

    /// Writes the stand-in `name`.
    pub fn program(&self, name: &str) {
        let path = self.folder().join(name);
        std::fs::write(&path, SCRIPT).unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
    }

    /// Removes the stand-in `name`, so it is not found.
    pub fn remove(&self, name: &str) {
        std::fs::remove_file(self.folder().join(name)).unwrap();
    }

    /// Puts `name`'s answer for its first two arguments `key`, as
    /// `<first>-<second>`, or its own where `key` is none.
    pub fn answer(&self, name: &str, key: Option<&str>, out: &str, err: &str, exit: i32) {
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
    pub fn listing(&self, roots: &[&Path]) {
        let plugins: Vec<Value> = roots
            .iter()
            .map(|root| json!({"plugin_id": "a-plugin", "plugin_root": root}))
            .collect();
        let listed = json!({"id": "cli:plugin", "result": {"plugins": plugins}});
        self.answer("herdr", Some("plugin-list"), &format!("{listed}\n"), "", 0);
    }

    /// `launchctl print` finding the agent.
    pub fn loaded(&self) {
        self.answer("launchctl", Some("print-gui"), "state = running\n", "", 0);
    }

    /// Each call of `name`, its arguments joined by spaces.
    pub fn calls(&self, name: &str) -> Vec<String> {
        match std::fs::read_to_string(self.folder().join(format!("{name}.calls"))) {
            Ok(text) => text.lines().map(str::to_string).collect(),
            Err(_) => Vec::new(),
        }
    }

    /// Forgets every call made so far.
    pub fn forget(&self) {
        for entry in std::fs::read_dir(self.folder()).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_some_and(|e| e == "calls" || e == "env") {
                std::fs::remove_file(path).unwrap();
            }
        }
    }

    /// Whether any stand-in was run.
    pub fn ran_any(&self) -> bool {
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
    pub fn setup(&self, args: &[&str]) -> Output {
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
    pub fn set_up(&self, args: &[&str]) -> Value {
        let output = self.setup(args);
        assert_eq!(output.status.code(), Some(0), "{}", said(&output));
        let answer: Value = serde_json::from_slice(&output.stdout).unwrap();
        answer["set_up"].clone()
    }
}

/// What a run said, for a failing assertion.
pub fn said(output: &Output) -> String {
    format!(
        "stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

/// The reason of a refusal in JSON, its exit `exit`.
pub fn refused(output: &Output, exit: i32) -> String {
    assert_eq!(output.status.code(), Some(exit), "{}", said(output));
    let answer: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(answer.as_object().map(|o| o.len()), Some(1), "{answer}");
    answer["refused"]["refusal"]["reason"]
        .as_str()
        .unwrap_or_else(|| panic!("no reason: {answer}"))
        .to_string()
}
