//! millpond's tick, `millpond tick --inbox <name> --json`, the runner's
//! second housekeeping beside Flume's settle: each pass, keeping or a
//! part's control pass, runs it once for each inbox the configuration
//! names, awaited on the settle's thread, so the host keeps one clock
//! (record 0033; core.13). millpond decides what is due and what a tick
//! runs (core.1). The runner reads nothing of a tick's answer but whether
//! millpond gave one: it is carried to no control plane and kept nowhere
//! (core.11), and a tick that fails stops no pass. What the runner says of
//! an inbox's ticks it says only when that changes, as it says its
//! settles (core.23).

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};

/// millpond's answer to a tick: its JSON where it exited 0, else its
/// words.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Reply {
    Given(String),
    Refused(String),
}

/// Why millpond gave no answer: it could not be run, or a signal ended
/// it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unrun(pub String);

/// millpond's command line, by the program at `program`: `millpond` on
/// the host's `PATH` in the binary, a stub in a test.
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

    /// One tick of the inbox named `inbox` in millpond's host
    /// configuration, `millpond tick --inbox <inbox> --json`, awaited.
    pub fn tick(&self, inbox: &str) -> Result<Reply, Unrun> {
        let output = Command::new(&self.program)
            .args(["tick", "--inbox", inbox, "--json"])
            .stdin(Stdio::null())
            .output()
            .map_err(|error| Unrun(format!("millpond could not run: {error}")))?;
        read(&output)
    }
}

/// millpond's answer as its exit gives it: its standard output where it
/// exited 0, its words on standard error where it exited otherwise, with
/// its prefix `millpond: ` left off, and no answer where a signal ended
/// it.
fn read(output: &Output) -> Result<Reply, Unrun> {
    let Some(code) = output.status.code() else {
        return Err(Unrun(format!("millpond was ended: {}", output.status)));
    };
    if code == 0 {
        return Ok(Reply::Given(
            String::from_utf8_lossy(&output.stdout).into_owned(),
        ));
    }
    let said = String::from_utf8_lossy(&output.stderr);
    let said = said.trim();
    let words = said.strip_prefix("millpond: ").unwrap_or(said);
    Ok(Reply::Refused(if words.is_empty() {
        format!("millpond exited {code}")
    } else {
        words.to_string()
    }))
}

/// Which inboxes' last ticks were refused or gave no answer, kept across
/// the turns of the runner's loop.
#[derive(Debug, Default)]
pub struct Ticks {
    unanswered: BTreeSet<String>,
}

impl Ticks {
    /// Takes the tick of `inbox`'s `answer`, saying a refusal or no
    /// answer only where the inbox's tick before was answered or this is
    /// its first, and that millpond ticks it again at the first answer
    /// after one that was not. millpond's words, and why it gave none, are
    /// said as given (core.1).
    pub fn heard(
        &mut self,
        inbox: &str,
        answer: Result<Reply, Unrun>,
        say: &mut dyn FnMut(String),
    ) {
        let line = match answer {
            Ok(Reply::Given(_)) => {
                if self.unanswered.remove(inbox) {
                    say(format!("boomdrive: millpond ticks the inbox {inbox} again"));
                }
                return;
            }
            Ok(Reply::Refused(words)) => {
                format!("boomdrive: millpond refused the tick of the inbox {inbox}: {words}")
            }
            Err(Unrun(why)) => {
                format!(
                    "boomdrive: millpond gave no answer to the tick of the inbox {inbox}: {why}"
                )
            }
        };
        if self.unanswered.insert(inbox.to_string()) {
            say(line);
        }
    }

    /// Forgets each inbox but those of `kept`, so one the configuration
    /// names again is said as at its first tick.
    pub fn keep(&mut self, kept: &[String]) {
        self.unanswered.retain(|inbox| kept.contains(inbox));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Each tick's lines, said in turn, each of `(inbox, answer)`.
    fn said(answers: Vec<(&str, Result<Reply, Unrun>)>) -> Vec<Vec<String>> {
        let mut ticks = Ticks::default();
        answers
            .into_iter()
            .map(|(inbox, answer)| {
                let mut lines = Vec::new();
                ticks.heard(inbox, answer, &mut |line| lines.push(line));
                lines
            })
            .collect()
    }

    /// The running spec's "A refused tick said once until millpond ticks
    /// again".
    #[test]
    fn a_refused_tick_is_said_once_until_millpond_ticks_again() {
        let refused = || {
            Ok(Reply::Refused(
                "this host keeps no inbox named `home`".into(),
            ))
        };
        let lines = said(vec![
            ("home", refused()),
            ("home", refused()),
            ("home", Ok(Reply::Given("{}".into()))),
            ("home", Ok(Reply::Given("{}".into()))),
        ]);
        assert_eq!(
            lines,
            vec![
                vec!["boomdrive: millpond refused the tick of the inbox home: \
                     this host keeps no inbox named `home`"
                    .to_string()],
                vec![],
                vec!["boomdrive: millpond ticks the inbox home again".to_string()],
                vec![],
            ]
        );
    }

    /// Each inbox's ticks are said apart.
    #[test]
    fn each_inbox_is_said_apart() {
        let unrun = || Err(Unrun("millpond could not run: not found".into()));
        let lines = said(vec![
            ("home", unrun()),
            ("work", unrun()),
            ("home", unrun()),
        ]);
        assert_eq!(
            lines,
            vec![
                vec![
                    "boomdrive: millpond gave no answer to the tick of the inbox home: \
                     millpond could not run: not found"
                        .to_string()
                ],
                vec![
                    "boomdrive: millpond gave no answer to the tick of the inbox work: \
                     millpond could not run: not found"
                        .to_string()
                ],
                vec![],
            ]
        );
    }

    /// An answered tick, the first among them, says nothing.
    #[test]
    fn an_answered_tick_says_nothing() {
        let given = || Ok(Reply::Given("{}".into()));
        assert_eq!(
            said(vec![("home", given()), ("home", given())]),
            vec![Vec::<String>::new(), vec![]]
        );
    }
}
