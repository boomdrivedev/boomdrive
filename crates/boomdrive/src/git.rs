//! Every git call the runner makes (design 5, 10), by git's command line
//! and never through a library (record 0001). Each runs at a bare
//! repository or a folder, never asks anyone, and holds no credential of
//! its own on its command line (core.4). A failure is named by the last
//! line git wrote on standard error.
//!
//! A call to a remote is given a [`Credential`] (record 0007 as amended on
//! 2026-11-11; core.34): the host's own credential helpers, as the host's
//! git configuration names them; or, where the configuration names the
//! Boomdrive App, the App's token alone, every helper of the host's emptied
//! for that call and its one helper, [`TOKEN_HELPER`], reading the token
//! from that call's environment, so no process listing shows it; or no
//! credential, every helper of the host's emptied.
//!
//! A call that reaches a remote, a clone's fetch, is bounded: it runs in a
//! process group of its own and is stopped at the git's limit, thirty
//! seconds, with everything it started, failing with words that name its
//! command, its remote and the limit. weir's crate `weir-bounded` holds the
//! limit, the run and the words (weir record 0008; record 0042 as amended
//! on 2026-11-11; core.58). Every other call reaches no remote and runs to
//! its end.

use std::ffi::{OsStr, OsString};
use std::fmt;
use std::path::Path;
use std::process::{Command, Output, Stdio};
use std::time::Duration;

use weir_bounded::TRY_FOR;

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

/// The variable the call's own helper reads the App's token from, set in
/// that call's environment alone.
const TOKEN_VAR: &str = "BOOMDRIVE_FORGE_TOKEN";

/// The call's own credential helper, in git's shell form: it reads what git
/// sends to its end, answers a `get` with the App's token from the call's
/// environment, and answers nothing else, `store` and `erase` among them
/// (design Decision 5).
pub const TOKEN_HELPER: &str = "!f() { cat >/dev/null; test \"$1\" != get || printf \
     'username=x-access-token\\npassword=%s\\n' \"$BOOMDRIVE_FORGE_TOKEN\"; }; f";

/// What a call to a remote is given to answer git's ask for a credential
/// (design Decision 5). Its `Debug` never shows a token (core.4).
#[derive(Clone, PartialEq, Eq)]
pub enum Credential {
    /// The host's own helpers, as the host's git configuration names
    /// them: no argument names a credential.
    Host,
    /// The Boomdrive App's token alone: `-c credential.helper=` empties
    /// every helper the host's configuration names, its URL-scoped ones
    /// among them, then `-c credential.helper=<TOKEN_HELPER>` reads the
    /// token from the call's environment.
    Token(String),
    /// No credential: `-c credential.helper=` alone, so git asks no helper
    /// and, never prompting, goes without one.
    Nothing,
}

impl fmt::Debug for Credential {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Credential::Host => "Host",
            Credential::Token(_) => "Token(..)",
            Credential::Nothing => "Nothing",
        })
    }
}

/// Where a call runs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum At<'a> {
    /// A bare repository, by `--git-dir`.
    GitDir(&'a Path),
    /// A folder, by `-C`: a worktree, or where a repository is made.
    Dir(&'a Path),
}

impl At<'_> {
    /// Names where `command` runs.
    fn set(self, command: &mut Command) {
        match self {
            At::GitDir(dir) => command.arg("--git-dir").arg(dir),
            At::Dir(dir) => command.arg("-C").arg(dir),
        };
    }
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
/// configuration in a test (design 11), its call to a remote stopped at
/// its limit.
#[derive(Clone, Debug)]
pub struct Git {
    overlay: Vec<(OsString, OsString)>,
    /// How long one try of a call to a remote may take: [`TRY_FOR`], or a
    /// test's (record 0042).
    try_for: Duration,
}

impl Default for Git {
    fn default() -> Git {
        Git::new(Vec::new())
    }
}

impl Git {
    /// git with `overlay` added to its environment, its call to a remote
    /// stopped at [`TRY_FOR`].
    pub fn new(overlay: Vec<(OsString, OsString)>) -> Git {
        Git {
            overlay,
            try_for: TRY_FOR,
        }
    }

    /// This git, its call to a remote stopped at `limit`: a test's
    /// shorter limit, which no host sets (record 0042).
    pub fn trying_for(self, limit: Duration) -> Git {
        Git {
            try_for: limit,
            ..self
        }
    }

    /// git's standard output from `args` at `at`, a call that reaches
    /// `remote` given `credential`, or its words where it failed. It runs
    /// in a process group of its own and is stopped at this git's limit
    /// with everything it started; a call stopped so fails naming its
    /// command, `remote` and the limit, after git's last line on standard
    /// error where git wrote one, and its standard output is not read
    /// (record 0042; core.58).
    pub fn call_remote<I, S>(
        &self,
        at: At,
        remote: &str,
        args: I,
        credential: &Credential,
    ) -> Result<String, Words>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let args: Vec<OsString> = args
            .into_iter()
            .map(|arg| arg.as_ref().to_os_string())
            .collect();
        let call = match args.first() {
            Some(verb) => format!("git {}", verb.to_string_lossy()),
            None => "git".to_string(),
        };
        let command = self.remote_command(at, &args, credential);
        bounded(command, &call, remote, self.try_for)
    }

    /// The call to a remote `args` at `at`, given `credential`, would
    /// make: its helpers set before the call names where it runs, and the
    /// App's token in its environment alone.
    fn remote_command<I, S>(&self, at: At, args: I, credential: &Credential) -> Command
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut command = self.bare_command();
        command.env_remove(TOKEN_VAR);
        match credential {
            Credential::Host => {}
            Credential::Token(token) => {
                command
                    .args(["-c", "credential.helper="])
                    .arg("-c")
                    .arg(format!("credential.helper={TOKEN_HELPER}"))
                    .env(TOKEN_VAR, token);
            }
            Credential::Nothing => {
                command.args(["-c", "credential.helper="]);
            }
        }
        at.set(&mut command);
        command.args(args);
        command
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
        at.set(&mut command);
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

/// `command`, the call `call` to `remote`, run to `limit`: its standard
/// output where it ended by itself and succeeded, its words otherwise, a
/// stopped call's naming `call`, `remote` and `limit`, run by
/// `weir-bounded` (record 0042 as amended on 2026-11-11).
fn bounded(
    mut command: Command,
    call: &str,
    remote: &str,
    limit: Duration,
) -> Result<String, Words> {
    let ended = weir_bounded::output(&mut command, limit)
        .map_err(|error| Words(format!("git could not run: {error}")))?;
    if ended.stopped {
        let stopped = weir_bounded::stopped(call, remote, limit);
        let stderr = String::from_utf8_lossy(&ended.stderr);
        return Err(Words(match last_line(&stderr) {
            Some(line) => format!("{line}; {stopped}"),
            None => stopped,
        }));
    }
    let output = Output {
        status: ended.status,
        stdout: ended.stdout,
        stderr: ended.stderr,
    };
    if output.status.success() {
        return Ok(String::from_utf8_lossy(&output.stdout).into_owned());
    }
    Err(failure(&output))
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
        "boomdrive runs with git {major}.{minor} or later, whose fetch keeps origin's HEAD current"
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
        let remote = git.remote_command(
            At::GitDir(Path::new("/d/r.git")),
            ["fetch", "--prune"],
            &Credential::Host,
        );
        assert_eq!(
            remote.get_args().collect::<Vec<_>>(),
            ["--git-dir", "/d/r.git", "fetch", "--prune"]
        );
        assert_eq!(set(&remote, "GIT_TERMINAL_PROMPT"), Some(Some("0".into())));
        assert_eq!(set(&remote, TOKEN_VAR), Some(None));
        for arg in command.get_args().chain(remote.get_args()) {
            assert!(
                !arg.to_string_lossy().contains("credential"),
                "an argument names a credential: {arg:?}"
            );
        }
    }

    /// A scratch folder and a global git configuration whose helpers, one
    /// for every URL and one for GitHub's, answer with the password
    /// `host`, and git under it, reading no system configuration.
    fn with_a_host_helper() -> (tempfile::TempDir, Git) {
        let dir = tempfile::tempdir().expect("a scratch folder");
        let gitconfig = dir.path().join("gitconfig");
        std::fs::write(
            &gitconfig,
            "[credential]\n\
             \thelper = \"!f() { cat >/dev/null; echo username=host; echo password=host; }; f\"\n\
             [credential \"https://github.com\"]\n\
             \thelper = \"!f() { cat >/dev/null; echo username=host; echo password=host; }; f\"\n",
        )
        .expect("the scratch git configuration");
        let git = Git::new(vec![
            ("GIT_CONFIG_GLOBAL".into(), gitconfig.into()),
            ("GIT_CONFIG_NOSYSTEM".into(), "1".into()),
            ("HOME".into(), dir.path().into()),
        ]);
        (dir, git)
    }

    /// `git credential fill` for `https://github.com/agentplot/flume.git`,
    /// run through the call's own command given `credential`: its exit,
    /// its standard output and the command's arguments.
    fn fill(git: &Git, dir: &Path, credential: &Credential) -> (bool, String, Vec<String>) {
        let mut command = git.remote_command(At::Dir(dir), ["credential", "fill"], credential);
        let args = command
            .get_args()
            .map(|arg| arg.to_string_lossy().into_owned())
            .collect();
        let mut child = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("git run");
        std::io::Write::write_all(
            child.stdin.as_mut().expect("its standard input"),
            b"protocol=https\nhost=github.com\npath=agentplot/flume.git\n\n",
        )
        .expect("the request written");
        let output = child.wait_with_output().expect("git's answer");
        (
            output.status.success(),
            String::from_utf8_lossy(&output.stdout).into_owned(),
            args,
        )
    }

    /// The clones spec's "Git is given the App's token alone".
    #[test]
    fn a_call_given_a_token_gives_git_that_token_alone() {
        let (dir, git) = with_a_host_helper();
        let (_, by_host, _) = fill(&git, dir.path(), &Credential::Host);
        assert!(by_host.contains("password=host\n"), "{by_host}");

        let token = "ghs_n0t4r34lt0k3n";
        let (filled, given, args) = fill(&git, dir.path(), &Credential::Token(token.into()));
        assert!(filled, "{given}");
        assert!(given.contains("username=x-access-token\n"), "{given}");
        assert!(given.contains(&format!("password={token}\n")), "{given}");
        assert!(!given.contains("host\n"), "{given}");
        for arg in &args {
            assert!(!arg.contains(token), "an argument holds the token: {arg}");
        }
    }

    /// The clones spec's "Git is given no credential".
    #[test]
    fn a_call_given_nothing_gives_git_no_credential() {
        let (dir, git) = with_a_host_helper();
        let (filled, given, args) = fill(&git, dir.path(), &Credential::Nothing);
        assert!(!filled, "{given}");
        assert!(!given.contains("password="), "{given}");
        assert_eq!(&args[..2], ["-c", "credential.helper="]);
    }

    #[test]
    fn a_credentials_debug_holds_no_token() {
        let shown = format!(
            "{:?} {:?} {:?}",
            Credential::Token("ghs_n0t4r34lt0k3n".into()),
            Credential::Host,
            Credential::Nothing
        );
        assert_eq!(shown, "Token(..) Host Nothing");
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

    /// The clones spec's "The limit is thirty seconds": a git tries a call
    /// to a remote for the family's limit, and a test's for its own.
    #[test]
    fn a_git_tries_for_the_familys_limit() {
        assert_eq!(Git::default().try_for, TRY_FOR);
        assert_eq!(Git::new(Vec::new()).try_for, TRY_FOR);
        let short = Duration::from_millis(300);
        assert_eq!(Git::default().trying_for(short).try_for, short);
    }

    /// A command that writes `stderr` and then never ends, standing in
    /// for git whose remote never answers.
    fn hanging(stderr: &str) -> Command {
        let mut command = Command::new("sh");
        command
            .args(["-c", &format!("printf '%b' '{stderr}' >&2; sleep 60")])
            .stdin(Stdio::null());
        command
    }

    /// Record 0042's design Decision 3: a call to a remote stopped at its
    /// limit fails naming its command, its remote and the limit, after
    /// git's last line where git wrote one; one that ends by itself is
    /// read as `run`'s is.
    #[test]
    fn a_call_to_a_remote_stopped_at_its_limit() {
        let limit = Duration::from_millis(300);
        let ample = Duration::from_secs(10);
        let stopped = "git fetch with origin was still going after 300ms, the most a try may \
                       take, and was stopped";
        assert_eq!(
            bounded(hanging("fatal: half\\n"), "git fetch", "origin", limit),
            Err(Words(format!("fatal: half; {stopped}")))
        );
        assert_eq!(
            bounded(hanging(""), "git fetch", "origin", limit),
            Err(Words(stopped.to_string()))
        );
        assert_eq!(
            bounded(saying("warning: kept\\n", 0), "git fetch", "origin", ample),
            Ok("out\n".into())
        );
        assert_eq!(
            bounded(
                saying("fatal: refused\\n", 128),
                "git fetch",
                "origin",
                ample
            ),
            Err(Words("fatal: refused".into()))
        );
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
            Err("git is 2.47.1: boomdrive runs with git 2.48 or later, \
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
