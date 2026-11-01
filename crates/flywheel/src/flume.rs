//! Flume's command line (design Decision 9), the one way the runner
//! reaches Flume (record 0001). Every call puts `--json` first and runs
//! with `FLUME_RECIPIENT` unset, so a place's recipients are only those
//! its launch names, and none passes `--force`. A launch passes its kind
//! to `--kind` as the control plane gave it and never `--kinds`, since
//! where a host's kinds come from is its Flume configuration's (core.5),
//! and every other flag only where the launch gives it. Each answer is
//! read by Flume's exit and kept as Flume gave it: the runner rewrites,
//! summarises and judges none (core.1).

use serde::de::IgnoredAny;
use serde::Deserialize;
use std::ffi::OsString;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

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
    /// The runner's own mark, `flywheel:<id>` (design Decision 11).
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
    /// The place at `place` in the repository at `container_dir`.
    fn status_of(&self, container_dir: &Path, place: &str) -> Result<Reply, Unrun>;
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

    fn status_of(&self, container_dir: &Path, place: &str) -> Result<Reply, Unrun> {
        self.run(vec![
            "status".into(),
            "--repository".into(),
            container_dir.into(),
            place.into(),
        ])
    }
}

/// `flume launch`'s arguments, in design Decision 9's order, the brief
/// named by `brief_file`.
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
    for recipient in &args.recipients {
        flag("--recipient", recipient.clone());
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

    #[test]
    fn a_mark_names_one_file() {
        assert_eq!(escaped("flywheel:madswan/12"), "flywheel%3Amadswan%2F12");
        assert_eq!(escaped("a%2F"), "a%252F");
        assert_ne!(escaped("a/b"), escaped("a%2Fb"));
    }
}
