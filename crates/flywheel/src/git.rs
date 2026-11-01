//! Every git call the runner makes (design 5, 10), by git's command line
//! and never through a library (record 0001). Each runs at a bare
//! repository or a folder, never asks anyone, and is given no credential:
//! the host's own credential helper answers, or the call fails (core.4). A
//! failure is named by the last line git wrote on standard error.

use std::ffi::{OsStr, OsString};
use std::fmt;
use std::path::Path;
use std::process::{Command, Output, Stdio};

/// The least git the runner runs with: 2.48, the first whose fetch keeps
/// origin's HEAD current (`remote.origin.followRemoteHEAD`, design 6).
pub const LEAST: (u32, u32, u32) = (2, 48, 0);

/// The variables that would send a call somewhere other than where it
/// names, removed so each runs where its `--git-dir` or `-C` says.
const ELSEWHERE: [&str; 6] = [
    "GIT_DIR",
    "GIT_WORK_TREE",
    "GIT_COMMON_DIR",
    "GIT_INDEX_FILE",
    "GIT_OBJECT_DIRECTORY",
    "GIT_ALTERNATE_OBJECT_DIRECTORIES",
];

/// Where a call runs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum At<'a> {
    /// A bare repository, by `--git-dir`.
    GitDir(&'a Path),
    /// A folder, by `-C`: a worktree, or where a repository is made.
    Dir(&'a Path),
}

/// Why a call failed: git's last line on standard error, or why git did
/// not run.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Words(pub String);

impl fmt::Display for Words {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// git as the runner runs it: the host's, under the caller's environment
/// overlay, which is empty on a host and holds the scratch git
/// configuration in a test (design 11).
#[derive(Clone, Debug, Default)]
pub struct Git {
    overlay: Vec<(OsString, OsString)>,
}

impl Git {
    /// git with `overlay` added to its environment.
    pub fn new(overlay: Vec<(OsString, OsString)>) -> Git {
        Git { overlay }
    }

    /// git's standard output from `args` at `at`, or its words where it
    /// failed.
    pub fn run<I, S>(&self, at: At, args: I) -> Result<String, Words>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        output(self.command(at, args))
    }

    /// git's standard output from `args` at `at`, the bytes as git wrote
    /// them, or its words where it failed.
    pub fn bytes<I, S>(&self, at: At, args: I) -> Result<Vec<u8>, Words>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let output = spawn(self.command(at, args))?;
        if output.status.success() {
            return Ok(output.stdout);
        }
        Err(failure(&output))
    }

    /// git's standard output from `args` at `at` where git answers yes
    /// (exits 0), none where it answers no (exits 1), as `rev-parse
    /// --verify --quiet`, `symbolic-ref --quiet` and `config --get`
    /// answer; its words where it failed otherwise.
    pub fn ask<I, S>(&self, at: At, args: I) -> Result<Option<String>, Words>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        answer(self.command(at, args))
    }

    /// Refused where git cannot be run or is older than the least the
    /// runner runs with (design 10's start check 3).
    pub fn check_version(&self) -> Result<(), String> {
        let mut command = self.bare_command();
        command.arg("--version");
        let said = output(command).map_err(|Words(words)| words)?;
        starts_with(&said)
    }

    /// The call `args` at `at` would make.
    fn command<I, S>(&self, at: At, args: I) -> Command
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut command = self.bare_command();
        match at {
            At::GitDir(dir) => command.arg("--git-dir").arg(dir),
            At::Dir(dir) => command.arg("-C").arg(dir),
        };
        command.args(args);
        command
    }

    /// git, its environment set, at nowhere yet. The overlay goes first,
    /// so nothing in it can let git ask anyone.
    fn bare_command(&self) -> Command {
        let mut command = Command::new("git");
        for name in ELSEWHERE {
            command.env_remove(name);
        }
        command
            .envs(self.overlay.iter().map(|(name, value)| (name, value)))
            .env("GIT_TERMINAL_PROMPT", "0")
            .env_remove("GIT_ASKPASS")
            .env_remove("SSH_ASKPASS")
            .stdin(Stdio::null());
        command
    }
}

/// `command`'s standard output, or its words where it failed.
fn output(command: Command) -> Result<String, Words> {
    let output = spawn(command)?;
    if output.status.success() {
        return Ok(String::from_utf8_lossy(&output.stdout).into_owned());
    }
    Err(failure(&output))
}

/// `command`'s standard output where it exits 0, none where it exits 1,
/// or its words where it failed otherwise.
fn answer(command: Command) -> Result<Option<String>, Words> {
    let output = spawn(command)?;
    match output.status.code() {
        Some(0) => Ok(Some(String::from_utf8_lossy(&output.stdout).into_owned())),
        Some(1) => Ok(None),
        _ => Err(failure(&output)),
    }
}

/// `command` run to its end, or why it could not run.
fn spawn(mut command: Command) -> Result<Output, Words> {
    command
        .output()
        .map_err(|error| Words(format!("git could not run: {error}")))
}

/// The words of a call that failed: its last line on standard error.
fn failure(output: &Output) -> Words {
    let stderr = String::from_utf8_lossy(&output.stderr);
    Words(match last_line(&stderr) {
        Some(line) => line.to_string(),
        None => format!("git failed ({}) and said nothing", output.status),
    })
}

/// The last line `text` holds that is not blank, trimmed.
fn last_line(text: &str) -> Option<&str> {
    text.lines().map(str::trim).rfind(|line| !line.is_empty())
}

/// The version `git --version` says, as `(major, minor, patch)`. Anything
/// after the number is set aside, so Apple's `git version 2.50.1 (Apple
/// Git-155)` reads as 2.50.1.
pub fn version(said: &str) -> Option<(u32, u32, u32)> {
    let number = said
        .trim()
        .strip_prefix("git version ")?
        .split_whitespace()
        .next()?;
    let mut parts = number.split('.').map(|part| {
        let digits = part
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(part.len());
        part[..digits].parse::<u32>().ok()
    });
    let major = parts.next()??;
    let minor = parts.next()??;
    let patch = parts.next().unwrap_or(Some(0))?;
    Some((major, minor, patch))
}

/// Refused, naming what git said and the least the runner runs with,
/// where `said` is no version or one older than [`LEAST`].
pub fn starts_with(said: &str) -> Result<(), String> {
    let (major, minor, _) = LEAST;
    let least = format!(
        "flywheel runs with git {major}.{minor} or later, whose fetch keeps origin's HEAD current"
    );
    match version(said) {
        None => Err(format!(
            "git says {:?}, which names no version: {least}",
            said.trim()
        )),
        Some(found) if found < LEAST => {
            let (major, minor, patch) = found;
            Err(format!("git is {major}.{minor}.{patch}: {least}"))
        }
        Some(_) => Ok(()),
    }
}

/// Whether git would reach `url` by ssh: `ssh://` and its `git+ssh://`
/// spellings, or the scp form `git@github.com:owner/name.git`, a colon
/// before any slash. Any other scheme is not, a test's `file://` forge
/// among them (design 5).
pub fn is_ssh(url: &str) -> bool {
    if let Some((scheme, _)) = url.split_once("://") {
        return ["ssh", "git+ssh", "ssh+git"]
            .iter()
            .any(|ssh| scheme.eq_ignore_ascii_case(ssh));
    }
    match url.find(':') {
        Some(colon) => !url[..colon].contains('/'),
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A built command's environment, by name: `None` where it is removed.
    fn environment(command: &Command) -> Vec<(String, Option<String>)> {
        command
            .get_envs()
            .map(|(name, value)| {
                (
                    name.to_string_lossy().into_owned(),
                    value.map(|value| value.to_string_lossy().into_owned()),
                )
            })
            .collect()
    }

    fn set(command: &Command, name: &str) -> Option<Option<String>> {
        environment(command)
            .into_iter()
            .find(|(set, _)| set == name)
            .map(|(_, value)| value)
    }

    #[test]
    fn git_asks_no_one() {
        let git = Git::new(vec![
            ("GIT_CONFIG_GLOBAL".into(), "/s/gitconfig".into()),
            ("GIT_ASKPASS".into(), "/bin/ask".into()),
            ("GIT_TERMINAL_PROMPT".into(), "1".into()),
        ]);
        let command = git.command(At::GitDir(Path::new("/d/r.git")), ["fetch", "--prune"]);
        assert_eq!(set(&command, "GIT_TERMINAL_PROMPT"), Some(Some("0".into())));
        assert_eq!(set(&command, "GIT_ASKPASS"), Some(None));
        assert_eq!(set(&command, "SSH_ASKPASS"), Some(None));
        assert_eq!(
            set(&command, "GIT_CONFIG_GLOBAL"),
            Some(Some("/s/gitconfig".into()))
        );
        for arg in command.get_args() {
            assert!(
                !arg.to_string_lossy().contains("credential"),
                "an argument names a credential: {arg:?}"
            );
        }
    }

    #[test]
    fn a_call_runs_where_it_names() {
        let git = Git::default();
        let bare = git.command(At::GitDir(Path::new("/d/r.git")), ["fetch"]);
        assert_eq!(
            bare.get_args().collect::<Vec<_>>(),
            ["--git-dir", "/d/r.git", "fetch"]
        );
        let folder = git.command(At::Dir(Path::new("/v/o/r")), ["status"]);
        assert_eq!(
            folder.get_args().collect::<Vec<_>>(),
            ["-C", "/v/o/r", "status"]
        );
        for name in ELSEWHERE {
            assert_eq!(set(&folder, name), Some(None), "{name} is not removed");
        }
    }

    /// A command that writes `stderr` and exits with `status`, standing
    /// in for git.
    fn saying(stderr: &str, status: u8) -> Command {
        let mut command = Command::new("sh");
        command.args([
            "-c",
            &format!("printf '%b' '{stderr}' >&2; echo out; exit {status}"),
        ]);
        command
    }

    #[test]
    fn a_failure_is_named_by_gits_last_line() {
        assert_eq!(
            output(saying("hint: one\\nfatal: the last\\n\\n", 128)),
            Err(Words("fatal: the last".into()))
        );
        assert_eq!(output(saying("warning: kept\\n", 0)), Ok("out\n".into()));
        let silent = output(saying("", 1)).unwrap_err();
        assert!(silent.0.ends_with("and said nothing"), "{silent}");
    }

    #[test]
    fn an_answer_is_yes_no_or_a_failure() {
        assert_eq!(answer(saying("", 0)), Ok(Some("out\n".into())));
        assert_eq!(answer(saying("", 1)), Ok(None));
        assert_eq!(
            answer(saying("fatal: not a git repository\\n", 128)),
            Err(Words("fatal: not a git repository".into()))
        );
    }

    #[test]
    fn an_old_git() {
        assert_eq!(
            starts_with("git version 2.47.1\n"),
            Err("git is 2.47.1: flywheel runs with git 2.48 or later, \
                 whose fetch keeps origin's HEAD current"
                .to_string())
        );
    }

    #[test]
    fn versions_taken() {
        assert_eq!(starts_with("git version 2.48.0\n"), Ok(()));
        assert_eq!(starts_with("git version 2.50.1 (Apple Git-155)\n"), Ok(()));
        assert_eq!(
            version("git version 2.50.1 (Apple Git-155)"),
            Some((2, 50, 1))
        );
        assert_eq!(starts_with("git version 3.0.0\n"), Ok(()));
    }

    #[test]
    fn junk_is_no_version() {
        for said in [
            "",
            "junk",
            "git version",
            "git version x.y.z",
            "git version 2",
        ] {
            assert_eq!(version(said), None, "{said:?} read as a version");
            assert!(starts_with(said).is_err(), "{said:?} taken");
        }
    }

    #[test]
    fn ssh_urls() {
        for url in [
            "git@github.com:a/b.git",
            "ssh://git@github.com/a/b.git",
            "git+ssh://git@github.com/a/b.git",
            "github.com:a/b.git",
        ] {
            assert!(is_ssh(url), "{url} is not ssh");
        }
        for url in [
            "https://github.com/a/b.git",
            "file:///tmp/forge/a/b.git",
            "/tmp/forge/a/b.git",
        ] {
            assert!(!is_ssh(url), "{url} is ssh");
        }
    }
}
