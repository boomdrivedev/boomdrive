//! The runner's report (design Decision 2): what it answered of each
//! launch, act, command and inbox, by id, with each tool's answer carried
//! as the tool printed it (core.11). It is sent whole at every report and
//! handed whole to every pull.

use crate::want::{Id, Lease};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::BTreeMap;
use std::fmt;
use time::format_description::well_known::Rfc3339;
use time::{OffsetDateTime, UtcOffset};

/// The runner's whole report, each part by id.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Report {
    pub launches: BTreeMap<Id, Launched>,
    pub acts: BTreeMap<Id, Acted>,
    pub commands: BTreeMap<Id, Commanded>,
    pub inboxes: BTreeMap<Id, InboxCarried>,
}

/// A launch the runner answered.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Launched {
    pub lease: Lease,
    pub answer: Answer,
    /// Flume's status of its place.
    pub status: Option<Carried>,
    /// sluice's entry of its submission.
    pub entry: Option<Carried>,
}

/// The runner's answer to an offer.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Answer {
    pub outcome: Outcome,
    /// The place Flume made, as Flume's answer names it.
    pub place_dir: Option<String>,
    pub words: Option<String>,
    pub at: At,
}

/// How the runner answered. An act or a command is only `Done` or
/// `Refused`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Outcome {
    Held,
    Launched,
    Done,
    Refused,
}

/// A tool's answer, carried as the tool printed it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Carried {
    pub text: String,
    /// When the runner first read it as it now stands.
    pub at: At,
    /// Whether the tool's last listing held it.
    pub listed: bool,
}

/// Which of a launch's carried answers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Carrying {
    Status,
    Entry,
}

/// An act the runner carried out or refused.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Acted {
    /// The launch acted on, by id.
    pub launch: Id,
    /// The launch's lease number.
    pub lease: u64,
    pub outcome: Outcome,
    pub words: Option<String>,
    pub at: At,
}

/// A command the runner carried out or refused.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Commanded {
    pub lease: Lease,
    pub outcome: Outcome,
    /// The tool's answer as given, where a tool ran.
    pub text: Option<String>,
    pub words: Option<String>,
    /// The commit a branch command's branch stood at on origin, or none
    /// where origin held no such branch. No command this release answers
    /// fills it: it is kept for a branch command an earlier release
    /// carried out, which the release before reads in `report.json`
    /// (core.36), and version 1 of the hosted protocol carries it
    /// (core.42).
    pub origin_commit: Option<String>,
    pub at: At,
}

/// An inbox as the runner last carried it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InboxCarried {
    /// The commit of the branch's tip it was read at, where one was read.
    pub tip: Option<String>,
    /// Each file under `inbox/` by its path.
    pub files: BTreeMap<String, InboxFile>,
    pub words: Option<String>,
    pub at: At,
}

/// A file of an inbox, carried as the tip holds it or by its digest.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InboxFile {
    Text(String),
    /// `sha256:<hex>` of its bytes.
    Digest(String),
}

impl Report {
    /// Gives launch `id`, under `lease`, the answer `answer`. Where its
    /// outcome, place and words are as they stood, the answer's `at` is
    /// kept, so it moves only when the answer's text does. Says whether
    /// the report changed.
    pub fn answer(&mut self, id: &Id, lease: Lease, answer: Answer) -> bool {
        let Some(launched) = self.launches.get_mut(id) else {
            self.launches.insert(
                id.clone(),
                Launched {
                    lease,
                    answer,
                    status: None,
                    entry: None,
                },
            );
            return true;
        };
        let renewed = launched.lease != lease;
        launched.lease = lease;
        let held = &launched.answer;
        if held.outcome == answer.outcome
            && held.place_dir == answer.place_dir
            && held.words == answer.words
        {
            return renewed;
        }
        launched.answer = answer;
        true
    }

    /// Carries `text` as launch `id`'s status or entry, read at `at`, the
    /// tool's last listing holding it or not. The carried `at` moves only
    /// where the text differs from what was carried. A launch the report
    /// does not hold carries nothing. Says whether the report changed.
    pub fn carry(&mut self, id: &Id, what: Carrying, text: &str, listed: bool, at: At) -> bool {
        let Some(launched) = self.launches.get_mut(id) else {
            return false;
        };
        let slot = match what {
            Carrying::Status => &mut launched.status,
            Carrying::Entry => &mut launched.entry,
        };
        match slot {
            Some(carried) if carried.text == text => {
                let changed = carried.listed != listed;
                carried.listed = listed;
                changed
            }
            _ => {
                *slot = Some(Carried {
                    text: text.to_string(),
                    at,
                    listed,
                });
                true
            }
        }
    }

    /// Each launch's and each command's id with the lease number held.
    pub fn leases(&self) -> BTreeMap<Id, u64> {
        let launches = self.launches.iter().map(|(id, l)| (id, l.lease.number));
        let commands = self.commands.iter().map(|(id, c)| (id, c.lease.number));
        launches
            .chain(commands)
            .map(|(id, number)| (id.clone(), number))
            .collect()
    }
}

/// A time, in UTC to the second, written as RFC 3339.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct At(OffsetDateTime);

impl At {
    /// Now.
    pub fn now() -> At {
        At::from(OffsetDateTime::now_utc())
    }

    /// The time `text` writes in RFC 3339, or why it is none.
    pub fn parse(text: &str) -> Result<At, time::error::Parse> {
        OffsetDateTime::parse(text, &Rfc3339).map(At::from)
    }

    /// The time as `time` holds it.
    pub fn time(self) -> OffsetDateTime {
        self.0
    }
}

impl From<OffsetDateTime> for At {
    /// `time` in UTC, its fraction of a second dropped.
    fn from(time: OffsetDateTime) -> At {
        let utc = time.to_offset(UtcOffset::UTC);
        At(utc.replace_nanosecond(0).unwrap_or(utc))
    }
}

impl fmt::Display for At {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = self.0.format(&Rfc3339).map_err(|_| fmt::Error)?;
        f.write_str(&text)
    }
}

impl Serialize for At {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for At {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<At, D::Error> {
        let text = String::deserialize(deserializer)?;
        At::parse(&text).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(text: &str) -> At {
        At::parse(text).unwrap()
    }

    fn lease(number: u64) -> Lease {
        Lease {
            number,
            expires: None,
        }
    }

    fn held(words: &str, when: &str) -> Answer {
        Answer {
            outcome: Outcome::Held,
            place_dir: None,
            words: Some(words.to_string()),
            at: at(when),
        }
    }

    fn launched(report: &Report, id: &Id) -> Launched {
        report.launches[id].clone()
    }

    #[test]
    fn an_answer_is_given_to_a_launch_the_report_lacks() {
        let mut report = Report::default();
        let id = Id::new("a");
        assert!(report.answer(&id, lease(3), held("waits on b", "2026-11-01T10:00:00Z")));
        let a = launched(&report, &id);
        assert_eq!(a.lease, lease(3));
        assert_eq!(a.answer, held("waits on b", "2026-11-01T10:00:00Z"));
        assert_eq!((a.status, a.entry), (None, None));
    }

    #[test]
    fn an_answers_time_stands_while_its_words_do() {
        let mut report = Report::default();
        let id = Id::new("a");
        report.answer(&id, lease(1), held("waits on b", "2026-11-01T10:00:00Z"));
        assert!(!report.answer(&id, lease(1), held("waits on b", "2026-11-01T10:05:00Z")));
        assert_eq!(launched(&report, &id).answer.at, at("2026-11-01T10:00:00Z"));
    }

    #[test]
    fn an_answers_time_moves_with_its_words() {
        let mut report = Report::default();
        let id = Id::new("a");
        report.answer(&id, lease(1), held("waits on b", "2026-11-01T10:00:00Z"));
        assert!(report.answer(
            &id,
            lease(1),
            held("waits for room", "2026-11-01T10:05:00Z")
        ));
        assert_eq!(
            launched(&report, &id).answer,
            held("waits for room", "2026-11-01T10:05:00Z")
        );
    }

    #[test]
    fn an_answers_time_moves_with_its_outcome_and_place() {
        let mut report = Report::default();
        let id = Id::new("a");
        report.answer(&id, lease(1), held("waits on b", "2026-11-01T10:00:00Z"));
        let place = Answer {
            outcome: Outcome::Launched,
            place_dir: Some("/places/a".to_string()),
            words: None,
            at: at("2026-11-01T10:05:00Z"),
        };
        assert!(report.answer(&id, lease(1), place.clone()));
        assert_eq!(launched(&report, &id).answer, place);
    }

    #[test]
    fn an_answer_keeps_what_was_carried_and_takes_the_lease() {
        let mut report = Report::default();
        let id = Id::new("a");
        report.answer(&id, lease(1), held("waits on b", "2026-11-01T10:00:00Z"));
        report.carry(
            &id,
            Carrying::Status,
            "{}",
            true,
            at("2026-11-01T10:00:00Z"),
        );
        assert!(report.answer(&id, lease(2), held("waits on b", "2026-11-01T10:05:00Z")));
        let a = launched(&report, &id);
        assert_eq!(a.lease, lease(2));
        assert_eq!(a.answer.at, at("2026-11-01T10:00:00Z"));
        assert!(a.status.is_some());
    }

    #[test]
    fn a_carried_texts_time_stands_while_its_text_does() {
        let mut report = Report::default();
        let id = Id::new("a");
        report.answer(&id, lease(1), held("waits on b", "2026-11-01T10:00:00Z"));
        let first = r#"{"status":{"places":[]}}"#;
        assert!(report.carry(
            &id,
            Carrying::Status,
            first,
            true,
            at("2026-11-01T10:00:00Z")
        ));
        assert!(!report.carry(
            &id,
            Carrying::Status,
            first,
            true,
            at("2026-11-01T10:05:00Z")
        ));
        let status = launched(&report, &id).status.unwrap();
        assert_eq!(status.text, first);
        assert_eq!(status.at, at("2026-11-01T10:00:00Z"));
    }

    #[test]
    fn a_carried_texts_time_moves_with_its_text() {
        let mut report = Report::default();
        let id = Id::new("a");
        report.answer(&id, lease(1), held("waits on b", "2026-11-01T10:00:00Z"));
        report.carry(
            &id,
            Carrying::Entry,
            r#"{"state":"queued"}"#,
            true,
            at("2026-11-01T10:00:00Z"),
        );
        let next = r#"{"state":"landed"}"#;
        assert!(report.carry(&id, Carrying::Entry, next, true, at("2026-11-01T10:05:00Z")));
        let entry = launched(&report, &id).entry.unwrap();
        assert_eq!(
            (entry.text.as_str(), entry.at),
            (next, at("2026-11-01T10:05:00Z"))
        );
    }

    #[test]
    fn a_carried_text_no_longer_listed_keeps_its_time() {
        let mut report = Report::default();
        let id = Id::new("a");
        report.answer(&id, lease(1), held("waits on b", "2026-11-01T10:00:00Z"));
        report.carry(
            &id,
            Carrying::Status,
            "{}",
            true,
            at("2026-11-01T10:00:00Z"),
        );
        assert!(report.carry(
            &id,
            Carrying::Status,
            "{}",
            false,
            at("2026-11-01T10:05:00Z")
        ));
        let status = launched(&report, &id).status.unwrap();
        assert_eq!(
            (status.listed, status.at),
            (false, at("2026-11-01T10:00:00Z"))
        );
    }

    #[test]
    fn a_launch_the_report_lacks_carries_nothing() {
        let mut report = Report::default();
        let id = Id::new("a");
        assert!(!report.carry(&id, Carrying::Status, "{}", true, At::now()));
        assert_eq!(report, Report::default());
    }

    #[test]
    fn the_leases_are_each_launchs_and_each_commands() {
        let mut report = Report::default();
        report.answer(
            &Id::new("a"),
            lease(3),
            held("waits", "2026-11-01T10:00:00Z"),
        );
        report.commands.insert(
            Id::new("c"),
            Commanded {
                lease: lease(7),
                outcome: Outcome::Done,
                text: None,
                words: None,
                origin_commit: None,
                at: at("2026-11-01T10:00:00Z"),
            },
        );
        let leases: Vec<(String, u64)> = report
            .leases()
            .into_iter()
            .map(|(id, number)| (id.to_string(), number))
            .collect();
        assert_eq!(leases, [("a".to_string(), 3), ("c".to_string(), 7)]);
    }

    #[test]
    fn a_time_is_utc_to_the_second() {
        let time = time::macros::datetime!(2026-11-01 12:30:15.75 +02:00);
        assert_eq!(At::from(time).to_string(), "2026-11-01T10:30:15Z");
        assert_eq!(
            at("2026-11-01T12:30:15+02:00").to_string(),
            "2026-11-01T10:30:15Z"
        );
        assert!(At::parse("yesterday").is_err());
        assert_eq!(At::now().time().nanosecond(), 0);
    }
}
