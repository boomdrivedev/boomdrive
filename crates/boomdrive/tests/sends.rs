//! A pass past due sends its answers as it makes them (record 0002 as
//! amended on 2026-11-14, and on taking turns; core.50; the
//! work-into-flume spec's "A pass past due sends each answer as it makes
//! it"): once the next control pass is due, a pass sends its report,
//! wherever an answer changed it since its last send, before each tool's
//! command it runs for a want, and once it has answered its last want, so
//! no answer waits on a command; a pass that ends before the next is due
//! sends once, at its end. What a send lets go or refuses is taken no more
//! that pass, nor an act on a launch it let go or refused (core.10).

mod fakes;

use boomdrive_control::fake::Fake;
use boomdrive_control::{
    ActSort, ControlPlane, Hello, Id, Outcome, Pulled, Report, Reported, Unread, Unsent, Want,
};
use fakes::{act, offer, place_of, Bench, Call, FakeFlume, PART};
use std::cell::Cell;

/// A fake control plane that notes, at each report it is sent, how many
/// launches Flume had been asked for by then, and which launches the
/// report answers `launched`, and, beside it, how many acts Flume had been
/// asked for by then, with the report itself.
struct Watching<'f> {
    fake: Fake,
    flume: &'f FakeFlume,
    sent: Vec<(usize, Vec<String>)>,
    seen: Vec<Seen>,
}

/// A report a [`Watching`] control plane took, with how many launches and
/// acts Flume had been asked for when it was sent.
struct Seen {
    launches: usize,
    acts: usize,
    report: Report,
}

impl Seen {
    /// The launches the report answers `outcome`, by id.
    fn launches(&self, outcome: Outcome) -> Vec<&str> {
        self.report
            .launches
            .iter()
            .filter(|(_, launched)| launched.answer.outcome == outcome)
            .map(|(id, _)| id.as_str())
            .collect()
    }

    /// The acts the report answers, by id.
    fn acts(&self) -> Vec<&str> {
        self.report.acts.keys().map(Id::as_str).collect()
    }
}

impl<'f> Watching<'f> {
    fn new(flume: &'f FakeFlume, fake: Fake) -> Watching<'f> {
        Watching {
            fake,
            flume,
            sent: Vec::new(),
            seen: Vec::new(),
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
            let launches = self.flume.launches().len();
            self.sent.push((launches, launched));
            self.seen.push(Seen {
                launches,
                acts: self.flume.acts().len(),
                report: report.clone(),
            });
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

/// The offers `names`, each `id` and `name` alike.
fn offers(names: &[&str]) -> Vec<Want> {
    names
        .iter()
        .map(|name| Want::Offer(offer(name, name)))
        .collect()
}

/// The spec's "A pass past due sends a launch's answer before its act
/// runs": the runner holds `x` at a place Flume stands, and the pass, due
/// from its start, is given a priority of `x`, then offers `a`, `b` and
/// `c`. One report, holding `a` launched and `b` and `c` held, goes up
/// after `a`'s launch and before `x`'s priority is run; one holding the
/// priority done once its answer is made; and the pass's end sends once
/// more.
#[test]
fn a_pass_past_due_sends_a_launchs_answer_before_its_act_runs() {
    let bench = Bench::new();
    let mut plane = Watching::new(&bench.flume, Fake::holding(offers(&["x"])));
    bench.pass_of(PART, &bench.flume, &mut plane, &|| false);
    let before = plane.seen.len();
    let mut wants = vec![act("x/priority", "x", ActSort::Priority(3))];
    wants.extend(offers(&["a", "b", "c"]));
    plane.fake.set_wants(wants);
    let (passed, _) = bench.pass_of(PART, &bench.flume, &mut plane, &|| true);
    assert!(passed.reported);
    let seen = &plane.seen[before..];
    assert_eq!(
        seen.len(),
        3,
        "before the act, once it is answered, at the end"
    );
    assert_eq!(
        (seen[0].launches, seen[0].acts),
        (2, 0),
        "after a, before x's priority"
    );
    assert_eq!(seen[0].launches(Outcome::Launched), ["a", "x"]);
    assert_eq!(seen[0].launches(Outcome::Held), ["b", "c"]);
    assert_eq!(seen[0].acts(), [] as [&str; 0]);
    for later in &seen[1..] {
        assert_eq!((later.launches, later.acts), (2, 1));
        assert_eq!(later.acts(), ["x/priority"]);
        assert_eq!(
            later.report.acts[&Id::new("x/priority")].outcome,
            Outcome::Done
        );
    }
}

/// The spec's "A pass past due sends each launch's answer before its next
/// launch": each pass is due from its start, and each pass's pull offers
/// `a`, `b` and `c`. The first pass launches `a`, the second `b` and the
/// third `c`; each sends a report holding its launch once its last want is
/// answered, before the next pass's launch is asked of Flume, and its end
/// sends once more.
#[test]
fn a_pass_past_due_sends_each_launchs_answer_before_its_next_launch() {
    let bench = Bench::new();
    let mut plane = Watching::new(&bench.flume, Fake::holding(offers(&["a", "b", "c"])));
    let mut said = Vec::new();
    for _ in 0..3 {
        let (passed, lines) = bench.pass_of(PART, &bench.flume, &mut plane, &|| true);
        assert!(passed.reported);
        said.extend(lines.into_iter().filter(|line| line.contains("launched")));
    }
    assert_eq!(
        said,
        ["a", "b", "c"].map(|name| format!("{name}: launched at {}", place_of(name)))
    );
    assert_eq!(
        plane.sent,
        [
            (1, ids(&["a"])),
            (1, ids(&["a"])),
            (2, ids(&["a", "b"])),
            (2, ids(&["a", "b"])),
            (3, ids(&["a", "b", "c"])),
            (3, ids(&["a", "b", "c"])),
        ],
        "each pass's report once its last want is answered, then its end's"
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
/// offers `c` in a repository the runner does not keep, then `b`, then
/// `a`. `c` is refused; the report sent before `b`'s launch holds that
/// refusal, and its answer has `a` refused, so `a` leaves the report, named
/// on a line, and its offer, later in the same pull, is neither launched
/// nor answered.
#[test]
fn a_want_refused_at_a_send_of_the_pass_is_taken_no_more() {
    let bench = Bench::new();
    let mut plane = Watching::new(&bench.flume, Fake::holding(offers(&["a"])));
    bench.pass_of(PART, &bench.flume, &mut plane, &|| false);
    let pa = place_of("a");
    bench.flume.tidy(&pa);
    bench.flume.take_calls();
    let before = plane.seen.len();
    plane
        .fake
        .refuse(Id::new("a"), "its lease is another runner's");
    let mut elsewhere = offer("c", "c");
    elsewhere.launch.repository = "example/elsewhere".to_string();
    let mut wants = vec![Want::Offer(elsewhere)];
    wants.extend(offers(&["b", "a"]));
    plane.fake.set_wants(wants);
    let (_, lines) = bench.pass_of(PART, &bench.flume, &mut plane, &|| true);
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
    let first = &plane.seen[before];
    assert_eq!(first.launches, 0, "sent before b's launch");
    assert_eq!(first.launches(Outcome::Refused), ["c"]);
    assert_eq!(first.launches(Outcome::Launched), ["a"]);
    let refused = format!(
        "a: let go, its lease no longer this runner's: its lease is another runner's: \
         its place {pa} is left on this host as Flume holds it"
    );
    assert_eq!(
        lines,
        [
            "c: refused: this host's configuration does not keep example/elsewhere".to_string(),
            refused,
            format!("b: launched at {}", place_of("b")),
        ]
    );
    let report = bench.report();
    assert!(!report.launches.contains_key(&Id::new("a")));
    assert_eq!(
        report.launches[&Id::new("b")].answer.outcome,
        Outcome::Launched
    );
}

/// The spec's "An act whose launch is let go at the send before it runs
/// nothing": the runner holds `x` at a place Flume stands, and the control
/// plane refuses `x`'s lease. The pass is due from its start, and its pull
/// offers `b`, then gives a priority of `x`, then a drop of `x`. `b` is
/// launched; the report sent before `x`'s priority would run has `x`
/// refused, so `x` leaves the report; no `flume` command is run on `x`'s
/// place, and neither act is answered that pass.
#[test]
fn an_act_whose_launch_is_let_go_at_the_send_before_it_runs_nothing() {
    let bench = Bench::new();
    let mut plane = Watching::new(&bench.flume, Fake::holding(offers(&["x"])));
    bench.pass_of(PART, &bench.flume, &mut plane, &|| false);
    let px = place_of("x");
    bench.flume.take_calls();
    let before = plane.seen.len();
    plane
        .fake
        .refuse(Id::new("x"), "its lease is another runner's");
    let mut wants = offers(&["b"]);
    wants.push(act("x/priority", "x", ActSort::Priority(3)));
    wants.push(act("x/drop", "x", ActSort::Drop));
    plane.fake.set_wants(wants);
    let (_, lines) = bench.pass_of(PART, &bench.flume, &mut plane, &|| true);
    let launched: Vec<String> = bench
        .flume
        .launches()
        .into_iter()
        .map(|args| args.name)
        .collect();
    assert_eq!(launched, ["b"]);
    let first = &plane.seen[before];
    assert_eq!(
        (first.launches, first.acts),
        (1, 0),
        "sent before x's priority"
    );
    assert_eq!(first.launches(Outcome::Launched), ["b", "x"]);
    let on_x: Vec<Call> = bench
        .flume
        .commands()
        .into_iter()
        .filter(|call| match call {
            Call::Act(place, _) => *place == px,
            Call::StatusOf(_, place) => *place == px,
            _ => false,
        })
        .collect();
    assert_eq!(on_x, [], "no flume command on x's place");
    assert_eq!(
        lines,
        [
            format!("b: launched at {}", place_of("b")),
            format!(
                "x: let go, its lease no longer this runner's: its lease is another runner's: \
                 its place {px} is left on this host as Flume holds it"
            ),
        ]
    );
    let report = bench.report();
    assert!(!report.launches.contains_key(&Id::new("x")));
    assert!(!report.acts.contains_key(&Id::new("x/priority")));
    assert!(!report.acts.contains_key(&Id::new("x/drop")));
    let last = plane.seen.last().expect("the pass's end was sent");
    assert_eq!(last.acts(), [] as [&str; 0], "neither act answered");
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
