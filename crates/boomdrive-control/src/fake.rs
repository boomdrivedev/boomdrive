//! A control plane in memory, for tests (design Decision 21): it holds
//! the wants and terminals a test sets, refuses a pull or a report when
//! the test says so, records each hello and report, gives the report it
//! holds back at every pull, and lets go and refuses what the test names.
//! It takes each carried status of a report it takes, but those the test
//! leaves.
//! It owns every id, or, made by [`Fake::owning`], those under one prefix
//! alone, so a test makes as many parts as it needs. [`Tokens`] is a
//! fake control plane's forge tokens, which gives each repository the
//! answer a test sets and counts each ask.

use crate::want::{Id, Input, Link, Terminal, Want};
use crate::{
    ControlPlane, ForgeTokens, Hello, Pulled, Report, Reported, TokenAsked, Unread, Unsent,
};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// A control plane in memory.
#[derive(Debug, Default)]
pub struct Fake {
    wants: Vec<Want>,
    terminals: Vec<Terminal>,
    pull_refusal: Option<String>,
    report_refusal: Option<String>,
    hellos: Vec<Hello>,
    reports: Vec<Report>,
    held: Option<Report>,
    let_go: BTreeSet<Id>,
    refused: BTreeMap<Id, String>,
    /// The launches whose carried statuses it does not take.
    left_statuses: BTreeSet<Id>,
    /// The prefix of every id it owns, none where it owns every id.
    owning: Option<String>,
    /// Whether its readers reach the host only through the runner.
    through_the_runner: bool,
}

impl Fake {
    /// A control plane wanting nothing.
    pub fn new() -> Fake {
        Fake::default()
    }

    /// A control plane wanting `wants`, in that order.
    pub fn holding(wants: Vec<Want>) -> Fake {
        Fake {
            wants,
            ..Fake::default()
        }
    }

    /// A control plane wanting nothing, owning only the ids under
    /// `prefix`, such as `home/`: a part of its own among others.
    pub fn owning(prefix: &str) -> Fake {
        Fake {
            owning: Some(prefix.to_string()),
            ..Fake::default()
        }
    }

    /// This control plane, its readers reaching the host only through the
    /// runner, as a hosted control plane's do.
    pub fn reading_through_the_runner(self) -> Fake {
        Fake {
            through_the_runner: true,
            ..self
        }
    }

    /// Wants `wants`, in that order, from the next pull on.
    pub fn set_wants(&mut self, wants: Vec<Want>) {
        self.wants = wants;
    }

    /// Opens a terminal on launch `launch` under lease number `lease`,
    /// given at the next pull and only then. The test holds its far end.
    pub fn open_terminal(&mut self, id: Id, launch: Id, lease: u64) -> FarEnd {
        let (input, from_far_end) = mpsc::channel();
        let (to_far_end, sent) = mpsc::channel();
        self.terminals.push(Terminal {
            id,
            launch,
            lease,
            link: Box::new(FakeLink {
                input: from_far_end,
                sent: to_far_end,
            }),
        });
        FarEnd { input, sent }
    }

    /// Refuses every pull from now on, with `words`.
    pub fn refuse_pulls(&mut self, words: &str) {
        self.pull_refusal = Some(words.to_string());
    }

    /// Answers every pull from now on.
    pub fn answer_pulls(&mut self) {
        self.pull_refusal = None;
    }

    /// Refuses every report from now on, with `words`.
    pub fn refuse_reports(&mut self, words: &str) {
        self.report_refusal = Some(words.to_string());
    }

    /// Takes every report from now on.
    pub fn take_reports(&mut self) {
        self.report_refusal = None;
    }

    /// Carries `id` no more: a report naming it is answered with it let
    /// go, the fake holds it no more, and no pull gives a want under it.
    pub fn let_go(&mut self, id: Id) {
        self.let_go.insert(id);
    }

    /// Refuses the report of `id`, its lease no longer the runner's, with
    /// `words`: a report naming it is answered with it refused, and the
    /// fake holds nothing of it.
    pub fn refuse(&mut self, id: Id, words: &str) {
        self.refused.insert(id, words.to_string());
    }

    /// Leaves the carried status of launch `id`: an answer to a report
    /// holding it does not name it taken, until [`Fake::take_status`].
    pub fn leave_status(&mut self, id: Id) {
        self.left_statuses.insert(id);
    }

    /// Takes the carried status of launch `id` again, as it takes every
    /// other.
    pub fn take_status(&mut self, id: Id) {
        self.left_statuses.remove(&id);
    }

    /// Each hello a pull the fake answered was handed, in order.
    pub fn hellos(&self) -> &[Hello] {
        &self.hellos
    }

    /// Each report the fake took, in order, as the runner sent it.
    pub fn reports(&self) -> &[Report] {
        &self.reports
    }

    /// The runner's report as the fake holds it: the last it took, but
    /// for what it let go and refused.
    pub fn held(&self) -> Option<&Report> {
        self.held.as_ref()
    }
}

impl ControlPlane for Fake {
    fn pull(&mut self, hello: &Hello, _report: &Report) -> Result<Pulled, Unread> {
        if let Some(words) = &self.pull_refusal {
            return Err(Unread(words.clone()));
        }
        self.hellos.push(hello.clone());
        let wants = self
            .wants
            .iter()
            .filter(|want| !self.let_go.contains(want.id()))
            .cloned()
            .collect();
        Ok(Pulled {
            wants,
            terminals: std::mem::take(&mut self.terminals),
            report: self.held.clone(),
        })
    }

    fn report(&mut self, report: &Report) -> Result<Reported, Unsent> {
        if let Some(words) = &self.report_refusal {
            return Err(Unsent(words.clone()));
        }
        self.reports.push(report.clone());
        let ids: BTreeSet<&Id> = report
            .launches
            .keys()
            .chain(report.acts.keys())
            .chain(report.commands.keys())
            .chain(report.inboxes.keys())
            .chain(report.previews.keys())
            .collect();
        let statuses_taken = report
            .launches
            .iter()
            .filter(|(id, launched)| {
                launched.status.is_some()
                    && !self.left_statuses.contains(*id)
                    && !self.let_go.contains(*id)
                    && !self.refused.contains_key(*id)
            })
            .map(|(id, _)| id.clone())
            .collect();
        let reported = Reported {
            let_go: ids
                .iter()
                .filter(|id| self.let_go.contains(**id))
                .map(|id| (*id).clone())
                .collect(),
            refused: ids
                .iter()
                .filter_map(|id| Some(((*id).clone(), self.refused.get(*id)?.clone())))
                .collect(),
            statuses_taken,
        };
        let mut held = report.clone();
        for id in reported.let_go.iter().chain(reported.refused.keys()) {
            held.launches.remove(id);
            held.acts.remove(id);
            held.commands.remove(id);
            held.inboxes.remove(id);
            held.previews.remove(id);
        }
        self.held = Some(held);
        Ok(reported)
    }

    fn owns(&self, id: &Id) -> bool {
        self.owning
            .as_deref()
            .is_none_or(|prefix| id.as_str().starts_with(prefix))
    }

    fn read_through_the_runner(&self) -> bool {
        self.through_the_runner
    }
}

/// A fake terminal's far end, which the test holds.
#[derive(Debug)]
pub struct FarEnd {
    /// Sends input to the runner. Dropping it closes the link.
    pub input: Sender<Input>,
    /// What the runner sent. Dropping it stops the far end listening.
    pub sent: Receiver<Sent>,
}

/// What the runner sent on a fake terminal's link.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Sent {
    Screen(String),
    Closed(String),
}

/// A fake terminal's link: a pair of channels to its far end.
struct FakeLink {
    input: Receiver<Input>,
    sent: Sender<Sent>,
}

impl Link for FakeLink {
    fn input(&mut self, wait: Duration) -> Option<Input> {
        match self.input.recv_timeout(wait) {
            Ok(input) => Some(input),
            Err(RecvTimeoutError::Timeout) => None,
            Err(RecvTimeoutError::Disconnected) => Some(Input::Close),
        }
    }

    fn screen(&mut self, text: &str) -> bool {
        self.sent.send(Sent::Screen(text.to_string())).is_ok()
    }

    fn close(self: Box<Self>, words: &str) {
        // A far end that stopped listening has nothing to be told.
        let _ = self.sent.send(Sent::Closed(words.to_string()));
    }
}

/// A control plane's forge tokens in memory: each repository gives the
/// answer a test set for it, or a refusal naming it where none is set, and
/// each repository's asks are counted. Its clones share what they hold, so
/// a test keeps one while the runner is handed another (design Decision 6
/// of a-cloud-host-gets-its-github-tokens-from-the-cloud).
#[derive(Clone, Debug, Default)]
pub struct Tokens {
    held: Arc<Mutex<TokensHeld>>,
}

#[derive(Debug, Default)]
struct TokensHeld {
    answers: BTreeMap<String, TokenAsked>,
    asks: BTreeMap<String, usize>,
}

impl Tokens {
    /// Forge tokens giving nothing set.
    pub fn new() -> Tokens {
        Tokens::default()
    }

    /// Gives `answer` for `repository` from the next ask on.
    pub fn set(&self, repository: &str, answer: TokenAsked) {
        self.held().answers.insert(repository.to_string(), answer);
    }

    /// How many times `repository`'s token was asked.
    pub fn asks(&self, repository: &str) -> usize {
        self.held().asks.get(repository).copied().unwrap_or(0)
    }

    fn held(&self) -> std::sync::MutexGuard<'_, TokensHeld> {
        self.held.lock().unwrap_or_else(|e| e.into_inner())
    }
}

impl ForgeTokens for Tokens {
    fn token(&self, repository: &str) -> TokenAsked {
        let mut held = self.held();
        *held.asks.entry(repository.to_string()).or_default() += 1;
        held.answers
            .get(repository)
            .cloned()
            .unwrap_or_else(|| TokenAsked::Refused {
                words: format!("the fake gives no token for {repository}"),
                host_to_fix: false,
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The control-plane-interface spec's "The fake holds what it is
    /// handed": a share's unleased place is held with the report, and the
    /// answer lets go of nothing (record 0054).
    #[test]
    fn the_fake_holds_an_unleased_place() {
        let mut fake = Fake::new();
        let mut share = Report::default();
        share.unleased.insert(
            "/p/places/note--by-hand".to_string(),
            crate::Unleased {
                repository: "agentplot/flume".to_string(),
                status: crate::Carried {
                    text: "{}".to_string(),
                    at: crate::At::parse("2026-11-01T10:00:00Z").unwrap(),
                    listed: true,
                },
            },
        );
        let answer = fake.report(&share).expect("the fake takes the report");
        assert!(answer.let_go.is_empty());
        assert!(answer.refused.is_empty());
        assert!(fake.reports()[0]
            .unleased
            .contains_key("/p/places/note--by-hand"));
    }

    /// A fake's forge tokens give the answer set for a repository, a
    /// refusal for one with none set, and count each repository's asks,
    /// whichever clone was asked.
    #[test]
    fn the_fake_tokens_count_each_ask() {
        use crate::ForgeToken;
        use time::format_description::well_known::Rfc3339;
        use time::OffsetDateTime;

        let tokens = Tokens::new();
        let ends = OffsetDateTime::parse("2026-11-18T13:00:00Z", &Rfc3339).unwrap();
        let given = TokenAsked::Token(ForgeToken::new("ghs_example", ends));
        tokens.set("example/app", given.clone());
        let handed: Box<dyn ForgeTokens> = Box::new(tokens.clone());
        assert_eq!(handed.token("example/app"), given);
        assert_eq!(handed.token("example/app"), given);
        assert!(matches!(
            handed.token("example/service"),
            TokenAsked::Refused {
                host_to_fix: false,
                ..
            }
        ));
        assert_eq!(tokens.asks("example/app"), 2);
        assert_eq!(tokens.asks("example/service"), 1);
        assert_eq!(tokens.asks("example/other"), 0);
    }
}
