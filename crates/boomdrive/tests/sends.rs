//! A pass past due sends its answers as it makes them (record 0002 as
//! amended on 2026-11-14; core.50; the work-into-flume spec's "A pass past
//! due sends each answer as it makes it"): once the next control pass is
//! due, a pass sends its report after each launch, act, command or preview
//! it carries out, so no answer waits on the rest of the pass; a pass that
//! ends before the next is due sends once, at its end.

mod fakes;

use boomdrive_control::fake::Fake;
use boomdrive_control::{
    ActSort, ControlPlane, Hello, Id, Outcome, Pulled, Report, Reported, Unread, Unsent, Want,
};
use fakes::{act, offer, place_of, Bench, Call, FakeFlume, PART};
use std::cell::Cell;

/// A fake control plane that notes, at each report it is sent, how many
/// launches Flume had been asked for by then, and which launches the
/// report answers `launched`.
struct Watching<'f> {
    fake: Fake,
    flume: &'f FakeFlume,
    sent: Vec<(usize, Vec<String>)>,
}

impl<'f> Watching<'f> {
    fn new(flume: &'f FakeFlume, fake: Fake) -> Watching<'f> {
        Watching {
            fake,
            flume,
            sent: Vec::new(),
        }
    }
}

impl ControlPlane for Watching<'_> {
    fn pull(&mut self, hello: &Hello, report: &Report) -> Result<Pulled, Unread> {
        self.fake.pull(hello, report)
    }

    fn report(&mut self, report: &Report) -> Result<Reported, Unsent> {
        let launched = report
            .launches
            .iter()
            .filter(|(_, launched)| launched.answer.outcome == Outcome::Launched)
            .map(|(id, _)| id.to_string())
            .collect();
        let reported = self.fake.report(report);
        if reported.is_ok() {
            self.sent.push((self.flume.launches().len(), launched));
        }
        reported
    }

    fn owns(&self, id: &Id) -> bool {
        self.fake.owns(id)
    }
}

/// `ids` as owned strings.
fn ids(ids: &[&str]) -> Vec<String> {
    ids.iter().map(|id| id.to_string()).collect()
}

/// The spec's "A pass past due sends each launch's answer before its next
/// launch": the pass is due from its start, and the pull offers `a`, `b`
/// and `c`. Each report goes up after a launch returns and before the next
/// is asked of Flume, holding every launch made so far, and the pass's end
/// sends once more.
#[test]
fn a_pass_past_due_sends_each_launchs_answer_before_its_next_launch() {
    let bench = Bench::new();
    let wants = ["a", "b", "c"]
        .map(|name| Want::Offer(offer(name, name)))
        .to_vec();
    let mut plane = Watching::new(&bench.flume, Fake::holding(wants));
    let (passed, lines) = bench.pass_of(PART, &bench.flume, &mut plane, &|| true);
    assert!(passed.reported);
    assert_eq!(
        lines,
        ["a", "b", "c"].map(|name| format!("{name}: launched at {}", place_of(name)))
    );
    assert_eq!(
        plane.sent,
        [
            (1, ids(&["a"])),
            (2, ids(&["a", "b"])),
            (3, ids(&["a", "b", "c"])),
            (3, ids(&["a", "b", "c"])),
        ],
        "a report after each launch, then the pass's own at its end"
    );
}

/// The spec's "A pass not yet due sends once": the next pass never falls
/// due while the pull's three launches are made, so the pass sends one
/// report, at its end, holding all three.
#[test]
fn a_pass_not_yet_due_sends_once_at_its_end() {
    let bench = Bench::new();
    let wants = ["a", "b", "c"]
        .map(|name| Want::Offer(offer(name, name)))
        .to_vec();
    let mut plane = Watching::new(&bench.flume, Fake::holding(wants));
    let (passed, _) = bench.pass_of(PART, &bench.flume, &mut plane, &|| false);
    assert!(passed.reported);
    assert_eq!(plane.sent, [(3, ids(&["a", "b", "c"]))]);
}

/// The spec's "An act's answer goes up before the next act runs": the
/// runner holds `a` and `b` at places Flume stands, Flume no longer stands
/// `c`'s, and the pass falls due once one act's command has run. The pull
/// gives a priority of `c`, then of `a`, then of `b`. Flume's refusal of
/// `c`'s place, read before any command runs, waits for the first send;
/// `a`'s priority is run and sent before `b`'s is left for the next pass.
#[test]
fn an_acts_answer_goes_up_before_the_next_act_runs() {
    let bench = Bench::new();
    let offers: Vec<Want> = ["a", "b", "c"]
        .map(|name| Want::Offer(offer(name, name)))
        .to_vec();
    let mut plane = Fake::holding(offers.clone());
    bench.pass(&mut plane);
    let pc = place_of("c");
    bench.flume.tidy(&pc);
    let mut wants = vec![
        act("c/priority", "c", ActSort::Priority(2)),
        act("a/priority", "a", ActSort::Priority(3)),
        act("b/priority", "b", ActSort::Priority(4)),
    ];
    wants.extend(offers);
    plane.set_wants(wants);
    let before = plane.reports().len();
    let due = || !bench.flume.acts().is_empty();
    bench.pass_due(&bench.flume, &mut plane, &due);
    let sent: Vec<Vec<&str>> = plane.reports()[before..]
        .iter()
        .map(|report| report.acts.keys().map(Id::as_str).collect())
        .collect();
    assert_eq!(
        sent,
        [
            vec!["a/priority", "c/priority"],
            vec!["a/priority", "c/priority"],
        ],
        "the first act's answer, with the refusal before it, then the pass's end"
    );
    let refused = &bench.report().acts[&Id::new("c/priority")];
    assert_eq!(
        (refused.outcome, refused.words.as_deref()),
        (
            Outcome::Refused,
            Some(format!("no place {pc} stands on this host").as_str())
        )
    );
}

/// The spec's "A want let go at a send of the pass is taken no more": the
/// runner holds `a` at a place Flume has since tidied, and the control
/// plane refuses `a`'s lease. The pass is due from its start, and its pull
/// offers `b`, then `a`. The send after `b`'s launch has `a` refused, so
/// `a` leaves the report, and its offer, later in the same pull, is not
/// launched again.
#[test]
fn a_want_refused_at_a_send_of_the_pass_is_taken_no_more() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![Want::Offer(offer("a", "a"))]);
    bench.pass(&mut plane);
    let pa = place_of("a");
    bench.flume.tidy(&pa);
    bench.flume.take_calls();
    plane.refuse(Id::new("a"), "its lease is another runner's");
    plane.set_wants(vec![
        Want::Offer(offer("b", "b")),
        Want::Offer(offer("a", "a")),
    ]);
    let (_, lines) = bench.pass_due(&bench.flume, &mut plane, &|| true);
    let launched: Vec<String> = bench
        .flume
        .calls()
        .into_iter()
        .filter_map(|call| match call {
            Call::Launch(args) => Some(args.name),
            _ => None,
        })
        .collect();
    assert_eq!(launched, ["b"], "a, refused, is launched no more");
    assert!(lines.contains(&format!(
        "a: let go, its lease no longer this runner's: its lease is another runner's: \
         its place {pa} is left on this host as Flume holds it"
    )));
    assert!(!bench.report().launches.contains_key(&Id::new("a")));
}

/// The spec's "A send that fails leaves the pass's answers to its end": the
/// pass is due from its start, and the control plane cannot take a report
/// until the pass's last launch is made. The send after the first launch
/// fails and is said once; no send is tried after the second; the pass's
/// end sends both.
#[test]
fn a_send_that_fails_leaves_the_passs_answers_to_its_end() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![
        Want::Offer(offer("a", "a")),
        Want::Offer(offer("b", "b")),
    ]);
    plane.refuse_reports("the state repository cannot be reached");
    let tries = Cell::new(0);
    let mut refusing = RefusingUntil {
        fake: &mut plane,
        flume: &bench.flume,
        launches: 2,
        tries: &tries,
    };
    let (passed, lines) = bench.pass_of(PART, &bench.flume, &mut refusing, &|| true);
    assert!(passed.reported);
    assert_eq!(
        tries.get(),
        2,
        "one send after the first launch, one at the end"
    );
    assert_eq!(
        lines
            .iter()
            .filter(|line| line.contains("could not be sent"))
            .collect::<Vec<_>>(),
        ["boomdrive: home: the report could not be sent: \
          the state repository cannot be reached: it is sent at the pass's end"]
    );
    let held = plane.held().expect("the pass's end was taken");
    assert_eq!(
        held.launches.keys().map(Id::as_str).collect::<Vec<_>>(),
        ["a", "b"]
    );
}

/// A fake control plane refusing every report until Flume has been asked
/// for `launches` launches, counting each send tried.
struct RefusingUntil<'p> {
    fake: &'p mut Fake,
    flume: &'p FakeFlume,
    launches: usize,
    tries: &'p Cell<usize>,
}

impl ControlPlane for RefusingUntil<'_> {
    fn pull(&mut self, hello: &Hello, report: &Report) -> Result<Pulled, Unread> {
        self.fake.pull(hello, report)
    }

    fn report(&mut self, report: &Report) -> Result<Reported, Unsent> {
        self.tries.set(self.tries.get() + 1);
        if self.flume.launches().len() >= self.launches {
            self.fake.take_reports();
        }
        self.fake.report(report)
    }

    fn owns(&self, id: &Id) -> bool {
        self.fake.owns(id)
    }
}
