//! Flume's command line (design Decision 9), the one way the runner
//! reaches Flume (record 0001). Every call puts `--json` first and runs
//! with `FLUME_RECIPIENT` unset, so a place's recipients are only those
//! its launch names, and none passes `--force`. A launch that names
//! recipients names the runner one more, [`RECIPIENT`], so Flume owes it
//! each end of the place, holding the place's record past its tidy, until
//! the runner confirms each by `flume confirm` once its control plane has
//! taken it (core.54; record 0030 as amended on 2026-11-09). A launch
//! passes its kind to `--kind` as the control plane gave it and never
//! `--kinds`, since where a host's kinds come from is its Flume
//! configuration's (core.5), and every other flag only where the launch
//! gives it. Each answer is read by Flume's exit and kept as Flume gave
//! it: the runner rewrites, summarises and judges none (core.1).

use crate::settle;
use serde::de::IgnoredAny;
use serde::Deserialize;
use std::ffi::OsString;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::time::Duration;

/// The least Flume the runner launches and confirms with: 0.21.0, the
/// first whose ends a recipient confirms by key, `flume confirm <key>…
/// --recipient <name>`, each owed until it is (core.38; record 0030 as
/// amended on 2026-11-09; Flume record 0061).
pub const LEAST: (u32, u32, u32) = (0, 21, 0);

/// The runner's own name as a recipient of the places it launches: fixed
/// and the same on every host, since one runner serves a host. No pane
/// holds it, so Flume sends it no notice and wakes nothing for it, and
/// owes it each end of the place until it confirms them (core.54; record
/// 0030 as amended on 2026-11-09; Flume records 0060 and 0061).
pub const RECIPIENT: &str = "boomdrive";

/// The recipients a launch naming `given` is launched with: `given`, as
/// the launch gives them, then the runner's own, [`RECIPIENT`], where
/// `given` names any and not it already. A launch naming none is given
/// none, so Flume's default is its recipient, which naming any would drop
/// (Flume's 7; core.15, core.54).
pub fn recipients(given: &[String]) -> Vec<String> {
    let mut named = given.to_vec();
    if !given.is_empty() && !given.iter().any(|recipient| recipient == RECIPIENT) {
        named.push(RECIPIENT.to_string());
    }
    named
}

/// The version `said`, the first line of `flume --version`, names:
/// `flume <major>.<minor>.<patch>`, a suffix after the patch ignored, as
/// [`crate::git::version`] reads git's.
pub fn version(said: &str) -> Option<(u32, u32, u32)> {
    let number = said
        .trim()
        .strip_prefix("flume ")?
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

/// Refused, naming what Flume said and the least the runner launches
/// with, where `said`, the first line of `flume --version`, is no version
/// or one older than [`LEAST`] (core.38).
pub fn runs_with(said: &str) -> Result<(), String> {
    let (major, minor, patch) = LEAST;
    let least = format!(
        "boomdrive launches with Flume {major}.{minor}.{patch} or later, \
         the first whose ends a recipient confirms by key"
    );
    let said = said.trim();
    match version(said) {
        None => Err(format!(
            "flume --version says {said:?}, which names no version: {least}"
        )),
        Some(found) if found < LEAST => Err(format!("flume --version says {said:?}: {least}")),
        Some(_) => Ok(()),
    }
}

/// What one `flume launch` is given, each field the flag of its name.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LaunchArgs {
    /// The clone's container, `--repository`.
    pub container_dir: PathBuf,
    /// Flume's kind, a name or an identity `<source>/<name>@<version>`, as
    /// the control plane gave it (design Decision 10).
    pub kind: String,
    pub name: String,
    pub base: Option<String>,
    /// Flume's stream, where the launch names one.
    pub stream: Option<String>,
    pub priority: Option<i64>,
    pub intent: Option<String>,
    pub recipients: Vec<String>,
    /// Each stage on hold with its words, `--hold <stage>=<words>`.
    pub holds: Vec<(String, String)>,
    pub hold_submission: bool,
    /// The brief's whole text, written to a file for `--brief-file`.
    pub brief: Option<String>,
    /// Each agent setting, a field and its value, `--agent <field>=<value>`.
    pub agent_settings: Vec<(String, String)>,
    /// The runner's own mark, `boomdrive:<id>` (design Decision 11).
    pub mark: String,
}

/// An act on a place: Flume's own command of the same name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Act {
    Retry {
        stage: Option<String>,
        words: Option<String>,
        submission: bool,
    },
    Drop,
    Priority(i64),
    Done,
}

/// Flume's answer, read by its exit.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Reply {
    /// Exit 0 with JSON on standard output: that output as Flume printed
    /// it.
    Given(String),
    /// Exit 1: the `reason` of Flume's `refused` answer, as given. Any
    /// other exit, or an answer that is not JSON or names no reason:
    /// Flume's standard error, as given.
    Refused(String),
}

/// Why `flume` gave no answer at all: it could not be run, or a signal
/// ended it. Nothing is kept of it, and the next pass asks again.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unrun(pub String);

impl fmt::Display for Unrun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Flume as the runner reaches it: `Cli` on a host, a fake in a test of
/// the pass.
pub trait Flume {
    /// One `flume launch`, its brief written to a file under `brief_dir`
    /// and removed once Flume answers.
    fn launch(&self, args: &LaunchArgs, brief_dir: &Path) -> Result<Reply, Unrun>;
    /// `act` on the place at `place`.
    fn act(&self, place: &str, act: &Act) -> Result<Reply, Unrun>;
    /// Every place and the host's part, `flume --json status`.
    fn status(&self) -> Result<Reply, Unrun>;
    /// The place at `place`, in the repository at `container_dir` where
    /// the runner keeps its clone, else by its location alone.
    fn status_of(&self, container_dir: Option<&Path>, place: &str) -> Result<Reply, Unrun>;
    /// Each end under `keys`, its delivery keys in the order given,
    /// confirmed as `recipient`: `flume confirm <key>… --recipient
    /// <recipient>`, naming no repository. The runner confirms as
    /// [`RECIPIENT`], only once its control plane has taken the status
    /// that gave the ends (design Decision 5; core.54; record 0030 as
    /// amended on 2026-11-09; Flume record 0061).
    fn confirm(&self, recipient: &str, keys: &[String]) -> Result<Reply, Unrun>;
}

/// Flume's command line, by the program at `program`: `flume` on the
/// host's `PATH` in the binary, a stub in a test.
#[derive(Clone, Debug)]
pub struct Cli {
    program: PathBuf,
}

impl Cli {
    pub fn new(program: impl Into<PathBuf>) -> Cli {
        Cli {
            program: program.into(),
        }
    }

    /// Flume's answer to `args`, after `--json`.
    fn run(&self, args: Vec<OsString>) -> Result<Reply, Unrun> {
        let output = Command::new(&self.program)
            .arg("--json")
            .args(args)
            .env_remove("FLUME_RECIPIENT")
            .stdin(Stdio::null())
            .output()
            .map_err(|error| Unrun(format!("flume could not run: {error}")))?;
        read(&output)
    }

    /// Flume's settle of every place as a caller's watch over the host,
    /// `flume --json settle --every <bound>`, its bound in whole seconds
    /// (core.13; record 0029). It is `Cli`'s own and on no trait: only the
    /// runner's loop runs it, and the loop runs `Cli`.
    pub fn settle_every(&self, bound: Duration) -> Result<Reply, Unrun> {
        self.run(vec![
            "settle".into(),
            "--every".into(),
            settle::every(bound).into(),
        ])
    }
}

impl Flume for Cli {
    fn launch(&self, args: &LaunchArgs, brief_dir: &Path) -> Result<Reply, Unrun> {
        let Some(brief) = &args.brief else {
            return self.run(launch_args(args, None));
        };
        let file = brief_dir.join(format!("{}.md", escaped(&args.mark)));
        fs::create_dir_all(brief_dir)
            .and_then(|()| fs::write(&file, brief))
            .map_err(|error| {
                Unrun(format!(
                    "the brief could not be written to {}: {error}",
                    file.display()
                ))
            })?;
        let reply = self.run(launch_args(args, Some(&file)));
        // The brief is the control plane's, and no copy of it stays on the
        // host once Flume has read it (core.11).
        let _ = fs::remove_file(&file);
        reply
    }

    fn act(&self, place: &str, act: &Act) -> Result<Reply, Unrun> {
        self.run(act_args(place, act))
    }

    fn status(&self) -> Result<Reply, Unrun> {
        self.run(vec!["status".into()])
    }

    fn status_of(&self, container_dir: Option<&Path>, place: &str) -> Result<Reply, Unrun> {
        let mut args: Vec<OsString> = vec!["status".into()];
        if let Some(container_dir) = container_dir {
            args.extend(["--repository".into(), container_dir.into()]);
        }
        args.push(place.into());
        self.run(args)
    }

    fn confirm(&self, recipient: &str, keys: &[String]) -> Result<Reply, Unrun> {
        let mut args: Vec<OsString> = vec!["confirm".into()];
        args.extend(keys.iter().map(OsString::from));
        args.extend(["--recipient".into(), recipient.into()]);
        self.run(args)
    }
}

/// `flume launch`'s arguments, in design Decision 9's order, the
/// runner's own name after the launch's recipients, and the brief named
/// by `brief_file`.
fn launch_args(args: &LaunchArgs, brief_file: Option<&Path>) -> Vec<OsString> {
    let mut words: Vec<OsString> = vec![
        "launch".into(),
        "--repository".into(),
        args.container_dir.clone().into(),
        "--kind".into(),
        args.kind.clone().into(),
        "--name".into(),
        args.name.clone().into(),
    ];
    let mut flag = |name: &str, value: String| {
        words.push(name.into());
        words.push(value.into());
    };
    if let Some(base) = &args.base {
        flag("--base", base.clone());
    }
    if let Some(stream) = &args.stream {
        flag("--stream", stream.clone());
    }
    if let Some(priority) = args.priority {
        flag("--priority", priority.to_string());
    }
    if let Some(intent) = &args.intent {
        flag("--intent", intent.clone());
    }
    for recipient in recipients(&args.recipients) {
        flag("--recipient", recipient);
    }
    for (stage, hold) in &args.holds {
        flag("--hold", format!("{stage}={hold}"));
    }
    if args.hold_submission {
        words.push("--hold-submission".into());
    }
    if let Some(file) = brief_file {
        words.push("--brief-file".into());
        words.push(file.into());
    }
    for (field, value) in &args.agent_settings {
        words.push("--agent".into());
        words.push(format!("{field}={value}").into());
    }
    words.push("--mark".into());
    words.push(args.mark.clone().into());
    words
}

/// The arguments of `act` on the place at `place`.
fn act_args(place: &str, act: &Act) -> Vec<OsString> {
    match act {
        Act::Retry {
            stage,
            words,
            submission,
        } => {
            let mut args: Vec<OsString> = vec!["retry".into(), place.into()];
            if let Some(stage) = stage {
                args.extend(["--stage".into(), stage.into()]);
            }
            if let Some(words) = words {
                args.extend(["--words".into(), words.into()]);
            }
            if *submission {
                args.push("--submission".into());
            }
            args
        }
        Act::Drop => vec!["drop".into(), place.into()],
        Act::Priority(priority) => {
            vec!["priority".into(), place.into(), priority.to_string().into()]
        }
        Act::Done => vec!["done".into(), place.into()],
    }
}

/// Flume's `refused` answer, as it prints one on exit 1.
#[derive(Deserialize)]
struct Refused {
    refused: Refusal,
}

#[derive(Deserialize)]
struct Refusal {
    refusal: Reason,
}

#[derive(Deserialize)]
struct Reason {
    reason: String,
}

/// Flume's answer, read by its exit (design Decision 9), or none where a
/// signal ended it.
fn read(output: &Output) -> Result<Reply, Unrun> {
    let Some(code) = output.status.code() else {
        return Err(Unrun(format!("flume was ended: {}", output.status)));
    };
    let printed = std::str::from_utf8(&output.stdout).ok();
    let stderr = || String::from_utf8_lossy(&output.stderr).into_owned();
    Ok(match (code, printed) {
        (0, Some(text)) if serde_json::from_str::<IgnoredAny>(text).is_ok() => {
            Reply::Given(text.to_string())
        }
        (1, Some(text)) => match serde_json::from_str::<Refused>(text) {
            Ok(refused) => Reply::Refused(refused.refused.refusal.reason),
            Err(_) => Reply::Refused(stderr()),
        },
        _ => Reply::Refused(stderr()),
    })
}

/// `mark` as one file name: every byte but a letter, a digit, `.`, `_`
/// and `-` written `%XX`, so two marks never name one file.
fn escaped(mark: &str) -> String {
    mark.bytes()
        .map(|byte| match byte {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'.' | b'_' | b'-' => {
                char::from(byte).to_string()
            }
            _ => format!("%{byte:02X}"),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn named(recipients: &[&str]) -> Vec<String> {
        recipients.iter().map(|named| named.to_string()).collect()
    }

    #[test]
    fn the_runner_is_named_after_a_launchs_recipients() {
        assert_eq!(
            recipients(&named(&["home/planner"])),
            named(&["home/planner", "boomdrive"])
        );
        assert_eq!(
            recipients(&named(&["home/planner", "home/reviewer"])),
            named(&["home/planner", "home/reviewer", "boomdrive"])
        );
        assert_eq!(recipients(&[]), named(&[]), "none named, none added");
        assert_eq!(
            recipients(&named(&["boomdrive", "home/planner"])),
            named(&["boomdrive", "home/planner"]),
            "the runner named once"
        );
    }

    #[test]
    fn versions_taken() {
        for said in [
            "flume 0.21.0",
            "flume 0.21.0\n",
            "flume 0.21.1 (77d5b93)",
            "flume 0.22.0-dev",
            "flume 0.21",
            "flume 1.0.0",
        ] {
            assert_eq!(runs_with(said), Ok(()), "{said:?}");
        }
        assert_eq!(version("flume 0.21.1 (77d5b93)"), Some((0, 21, 1)));
    }

    #[test]
    fn a_flume_older_than_the_least_is_refused() {
        let least = "boomdrive launches with Flume 0.21.0 or later, \
                     the first whose ends a recipient confirms by key";
        assert_eq!(
            runs_with("flume 0.20.0\n"),
            Err(format!("flume --version says \"flume 0.20.0\": {least}"))
        );
        assert_eq!(
            runs_with("flume 0.20.9"),
            Err(format!("flume --version says \"flume 0.20.9\": {least}"))
        );
        for said in ["flume", "", "sluice 0.20.0", "flume x.y.z", "flume 0"] {
            assert_eq!(
                runs_with(said),
                Err(format!(
                    "flume --version says {:?}, which names no version: {least}",
                    said.trim()
                )),
                "{said:?}"
            );
        }
    }

    #[test]
    fn a_mark_names_one_file() {
        assert_eq!(escaped("boomdrive:home/12"), "boomdrive%3Ahome%2F12");
        assert_eq!(escaped("a%2F"), "a%252F");
        assert_ne!(escaped("a/b"), escaped("a%2Fb"));
    }
}
