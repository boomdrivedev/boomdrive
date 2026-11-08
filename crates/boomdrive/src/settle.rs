//! Flume's settle of every place, `flume settle --every <bound>`, which
//! ends each turn of the runner's passes and so keeps Flume's watch over
//! the host while the runner runs (core.13; record 0029; Flume's 208).
//! The runner reads nothing of the settle's answer but whether Flume gave
//! one: its places and tidy are Flume's, carried to no control plane and
//! kept nowhere (core.11, core.35). What it says of its settles it says
//! only when that changes, as it says its control plane's reach (core.23).

use crate::flume::{Reply, Unrun};
use std::time::Duration;

/// The settle's bound: `fetch_interval`, or the shorter of it and
/// `poll_interval` where the runner makes control passes, `poll_interval`
/// being `None` where it makes none (design Decision 2).
pub fn bound(fetch_interval: Duration, poll_interval: Option<Duration>) -> Duration {
    poll_interval.map_or(fetch_interval, |poll_interval| {
        fetch_interval.min(poll_interval)
    })
}

/// `bound` as `--every` takes it, `<n>s`: whole seconds, rounded up, and
/// never less than one, so Flume's refusal of a zero bound is out of
/// reach.
pub fn every(bound: Duration) -> String {
    let seconds = bound.as_secs() + u64::from(bound.subsec_nanos() > 0);
    format!("{}s", seconds.max(1))
}

/// Whether the runner's last settle was answered, none before the first,
/// kept across the turns of its loop.
#[derive(Debug, Default)]
pub struct Settles {
    /// Whether the last settle was refused or gave no answer.
    unanswered: bool,
}

impl Settles {
    /// Takes one settle's `answer`, saying a refusal or no answer only
    /// where the settle before was answered or this is the first, and
    /// that Flume settles again at the first answer after one that was
    /// not. Flume's words, and why it gave none, are said as given
    /// (core.1).
    pub fn heard(&mut self, answer: Result<Reply, Unrun>, say: &mut dyn FnMut(String)) {
        let line = match answer {
            Ok(Reply::Given(_)) => {
                if std::mem::replace(&mut self.unanswered, false) {
                    say("boomdrive: Flume settles every place again".to_string());
                }
                return;
            }
            Ok(Reply::Refused(words)) => {
                format!("boomdrive: Flume refused the settle of every place: {words}")
            }
            Err(Unrun(why)) => {
                format!("boomdrive: Flume gave no answer to the settle of every place: {why}")
            }
        };
        if !std::mem::replace(&mut self.unanswered, true) {
            say(line);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MINUTE: Duration = Duration::from_secs(60);

    /// Each settle's lines, said in turn.
    fn said(answers: Vec<Result<Reply, Unrun>>) -> Vec<Vec<String>> {
        let mut settles = Settles::default();
        answers
            .into_iter()
            .map(|answer| {
                let mut lines = Vec::new();
                settles.heard(answer, &mut |line| lines.push(line));
                lines
            })
            .collect()
    }

    #[test]
    fn the_bound_is_the_fetch_interval_with_no_control_pass() {
        assert_eq!(bound(2 * MINUTE, None), 2 * MINUTE);
    }

    /// The running spec's "The shorter interval".
    #[test]
    fn the_bound_is_the_shorter_interval() {
        assert_eq!(bound(2 * MINUTE, Some(MINUTE)), MINUTE);
        assert_eq!(bound(MINUTE, Some(2 * MINUTE)), MINUTE);
        assert_eq!(bound(MINUTE, Some(MINUTE)), MINUTE);
    }

    #[test]
    fn every_is_whole_seconds() {
        assert_eq!(every(2 * MINUTE), "120s");
        assert_eq!(every(Duration::from_secs(1)), "1s");
        assert_eq!(every(Duration::from_millis(1500)), "2s");
        assert_eq!(every(Duration::from_millis(1)), "1s");
        assert_eq!(every(Duration::ZERO), "1s");
    }

    /// The running spec's "A refusal said once until Flume settles again".
    #[test]
    fn a_refusal_is_said_once_until_flume_settles_again() {
        let refused = || Ok(Reply::Refused("no store".to_string()));
        let lines = said(vec![
            refused(),
            refused(),
            refused(),
            Ok(Reply::Given("{\"settled\":{}}\n".to_string())),
        ]);
        assert_eq!(
            lines,
            vec![
                vec!["boomdrive: Flume refused the settle of every place: no store".to_string()],
                vec![],
                vec![],
                vec!["boomdrive: Flume settles every place again".to_string()],
            ]
        );
    }

    /// The running spec's "A settle with no answer".
    #[test]
    fn a_settle_with_no_answer_is_said_once() {
        let unrun = || Err(Unrun("flume could not run: not found".to_string()));
        let lines = said(vec![unrun(), unrun()]);
        assert_eq!(
            lines,
            vec![
                vec![
                    "boomdrive: Flume gave no answer to the settle of every place: \
                     flume could not run: not found"
                        .to_string()
                ],
                vec![],
            ]
        );
    }

    /// An answered settle, the first among them, says nothing.
    #[test]
    fn an_answered_settle_says_nothing() {
        let given = || Ok(Reply::Given("{}".to_string()));
        assert_eq!(
            said(vec![given(), given()]),
            vec![Vec::<String>::new(), vec![]]
        );
    }
}
