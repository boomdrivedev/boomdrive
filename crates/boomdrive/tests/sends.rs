//! The runner carries its work side by side (record 0002 as amended on
//! 2026-11-19; core.50; the work-into-flume spec's "Each want is carried
//! on a thread of its own" and "Each answer goes up once it is made"):
//! each launch, act and preview runs its command on a thread of its own as
//! soon as the pull gives it, so a slow launch holds no other launch and no
//! land; an act on a launch waits only until that launch is made; each
//! answer comes back over the part's channel and goes up in the part's
//! next send, between its passes; a want in flight is not taken again from
//! a later pull; and an answer for a lease the control plane let go or
//! refused while its command ran is not reported (core.10).

mod fakes;

use boomdrive::carry::{Carrier, Passed};
use boomdrive::flume::Act;
use boomdrive::hands::{self, Underway};
use boomdrive_control::fake::Fake;
use boomdrive_control::{ActSort, Id, Outcome, Report, Want};
use fakes::{act, offer, place_of, Bench, Call, PART};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver};
use std::sync::Arc;
use std::time::Duration;

/// The offers `names`, each `id` and `name` alike.
fn offers(names: &[&str]) -> Vec<Want> {
    names
        .iter()
        .map(|name| Want::Offer(offer(name, name)))
        .collect()
}

/// Whether `calls` holds a launch named `name`.
fn launched(calls: &[Call], name: &str) -> bool {
    calls
        .iter()
        .any(|call| matches!(call, Call::Launch(args) if args.name == name))
}

/// The launches `report` answers `outcome`, by id.
fn answered(report: Option<&Report>, outcome: Outcome) -> Vec<String> {
    report
        .map(|report| {
            report
                .launches
                .iter()
                .filter(|(_, launched)| launched.answer.outcome == outcome)
                .map(|(id, _)| id.to_string())
                .collect()
        })
        .unwrap_or_default()
}

/// A gate a held launch or act waits on, opened by the test.
fn gate() -> (Arc<AtomicBool>, impl Fn(&[Call]) -> bool + Send + 'static) {
    let open = Arc::new(AtomicBool::new(false));
    let opened = Arc::clone(&open);
    (open, move |_: &[Call]| opened.load(Ordering::SeqCst))
}

/// A part carried across its passes, as `boomdrive run` carries one: its
/// carrier, the commands it has under way, and the wakes their answers
/// ring.
struct Part<'a> {
    carrier: &'a Carrier<'a>,
    underway: &'a Underway<'a>,
    woke: &'a Receiver<()>,
}

impl Part<'_> {
    /// One control pass, which waits on no command, and its lines.
    fn pass(&self, plane: &mut Fake) -> (Passed, Vec<String>) {
        let mut lines = Vec::new();
        let passed = self
            .carrier
            .carry_pass(plane, self.underway, &mut |line| lines.push(line));
        (passed, lines)
    }

    /// Takes the answers that came back, as the part does between its
    /// passes each time one rings, until `done` reads the control plane
    /// as it wants it, and the lines said.
    fn collect_until(&self, plane: &mut Fake, done: impl Fn(&Fake) -> bool) -> Vec<String> {
        let mut lines = Vec::new();
        while !done(plane) {
            self.woke
                .recv_timeout(Duration::from_secs(10))
                .expect("an answer comes back");
            self.carrier
                .collect(plane, self.underway, &mut |line| lines.push(line));
        }
        lines
    }
}

/// Runs `body` with `bench`'s part carried across its passes.
fn carried(bench: &Bench, body: impl FnOnce(&Part)) {
    let sluice = bench.sluice_cli();
    let control_dir = bench.control_dir();
    let carrier = bench.carrier(PART, &bench.flume, &sluice, &control_dir);
    let (rang, woke) = mpsc::channel();
    let wake = move || {
        let _ = rang.send(());
    };
    hands::carried(&bench.flume, &sluice, &bench.locks, &wake, |underway| {
        body(&Part {
            carrier: &carrier,
            underway,
            woke: &woke,
        });
    });
}

/// The work-into-flume spec's "A slow launch holds no other launch": the
/// pull offers `a`, whose launch is readied only once `b`'s has been asked
/// of Flume, then `b`. Both are launched in the one pass, `b` while `a`'s
/// launch runs.
#[test]
fn a_slow_launch_holds_no_other_launch() {
    let bench = Bench::new();
    bench.flume.hold("a", |calls| launched(calls, "b"));
    let mut plane = Fake::holding(offers(&["a", "b"]));
    let (passed, _) = bench.pass(&mut plane);
    assert!(passed.reported);
    assert_eq!(
        bench.flume.let_go_by("a"),
        Some(true),
        "b was asked while a ran"
    );
    let sent = plane.reports().last();
    assert_eq!(answered(sent, Outcome::Launched), ["a", "b"]);
}

/// The work-into-flume spec's "A slow launch holds no land": the runner
/// holds `x` at a place Flume stands, and the pull offers `x` and `a`,
/// whose launch is readied only once `x`'s land has been asked of Flume,
/// then gives a land of `x`. The land runs while `a`'s launch does.
#[test]
fn a_slow_launch_holds_no_land() {
    let bench = Bench::new();
    let mut plane = Fake::holding(offers(&["x"]));
    bench.pass(&mut plane);
    let px = place_of("x");
    let landed = px.clone();
    bench.flume.hold("a", move |calls| {
        calls.iter().any(|call| {
            matches!(call, Call::Act(place, Act::Retry { submission: true, .. }) if *place == landed)
        })
    });
    plane.set_wants(vec![
        Want::Offer(offer("x", "x")),
        Want::Offer(offer("a", "a")),
        act("x/land", "x", ActSort::Land),
    ]);
    let (_, lines) = bench.pass(&mut plane);
    assert_eq!(
        bench.flume.let_go_by("a"),
        Some(true),
        "the land ran while a did"
    );
    assert!(
        lines.contains(&format!("x/land: land of x at {px}: done")),
        "{lines:?}"
    );
    assert_eq!(
        bench.report().acts[&Id::new("x/land")].outcome,
        Outcome::Done
    );
}

/// The work-into-flume spec's "Each answer goes up once it is made": a
/// part's pass starts `a`'s launch, which is slow, and `b`'s, and ends
/// without waiting on either; `b`'s answer goes up as soon as it comes
/// back, while `a`'s launch still runs, and `a`'s once it is made, each
/// between the part's passes. A later pass while `a`'s launch runs takes
/// it no more.
#[test]
fn each_answer_goes_up_once_it_is_made() {
    let bench = Bench::new();
    let (open, until) = gate();
    bench.flume.hold("a", until);
    let mut plane = Fake::holding(offers(&["a", "b"]));
    carried(&bench, |part| {
        let (passed, _) = part.pass(&mut plane);
        assert!(passed.reported);
        part.collect_until(&mut plane, |plane| {
            answered(plane.reports().last(), Outcome::Launched) == ["b"]
        });
        let (_, lines) = part.pass(&mut plane);
        assert_eq!(lines, [] as [String; 0], "a is not taken again");
        assert_eq!(bench.flume.launches().len(), 2);
        open.store(true, Ordering::SeqCst);
        let lines = part.collect_until(&mut plane, |plane| {
            answered(plane.reports().last(), Outcome::Launched) == ["a", "b"]
        });
        assert_eq!(lines, [format!("a: launched at {}", place_of("a"))]);
        assert_eq!(bench.flume.launches().len(), 2, "no launch made twice");
    });
}

/// The work-into-flume spec's "An act waits only until its launch is
/// made": a pull gives the offer `a`, whose launch is slow, then a
/// priority of `a`. The priority is neither run nor refused while `a`'s
/// launch runs, at that pass or the next; once `a` is made, it is run on
/// its place and its answer goes up, between the part's passes.
#[test]
fn an_act_waits_only_until_its_launch_is_made() {
    let bench = Bench::new();
    let (open, until) = gate();
    bench.flume.hold("a", until);
    let mut plane = Fake::holding(vec![
        Want::Offer(offer("a", "a")),
        act("a/priority", "a", ActSort::Priority(3)),
    ]);
    carried(&bench, |part| {
        part.pass(&mut plane);
        part.pass(&mut plane);
        assert_eq!(bench.flume.acts(), []);
        let held = boomdrive::report::read(&bench.report_file()).unwrap();
        assert!(held.is_none_or(|report| !report.acts.contains_key(&Id::new("a/priority"))));
        open.store(true, Ordering::SeqCst);
        let lines = part.collect_until(&mut plane, |plane| {
            plane
                .reports()
                .last()
                .is_some_and(|report| report.acts.contains_key(&Id::new("a/priority")))
        });
        let pa = place_of("a");
        assert_eq!(bench.flume.acts(), [(pa.clone(), Act::Priority(3))]);
        assert!(
            lines.contains(&format!("a/priority: priority 3 of a at {pa}: done")),
            "{lines:?}"
        );
    });
}

/// The work-into-flume spec's "An answer for a lease let go while its
/// command ran": the runner holds `x` at a place Flume stands, a land of
/// `x` runs slowly, and the control plane refuses `x`'s lease at the
/// part's next pass. Once the land returns, its answer is not reported,
/// said in one line.
#[test]
fn an_answer_for_a_lease_refused_while_its_command_ran() {
    let bench = Bench::new();
    let mut plane = Fake::holding(offers(&["x"]));
    bench.pass(&mut plane);
    let (open, until) = gate();
    bench.flume.hold(&place_of("x"), until);
    plane.set_wants(vec![
        Want::Offer(offer("x", "x")),
        act("x/land", "x", ActSort::Land),
    ]);
    carried(&bench, |part| {
        part.pass(&mut plane);
        plane.refuse(Id::new("x"), "lease 2 of x is host-b's");
        part.pass(&mut plane);
        assert!(!bench.report().launches.contains_key(&Id::new("x")));
        let sent = plane.reports().len();
        open.store(true, Ordering::SeqCst);
        let lines = part.collect_until(&mut plane, |_| part.underway.idle());
        assert_eq!(
            lines,
            ["x/land: let go by the control plane while its command ran: its answer is not reported"]
        );
        assert!(!bench.report().acts.contains_key(&Id::new("x/land")));
        assert_eq!(plane.reports().len(), sent, "nothing more is sent");
    });
}

/// The work-into-flume spec's "A command in flight when the runner
/// stops": a part's pass starts `a`'s slow launch, and the runner stops;
/// the part waits for the launch to end, takes its answer and sends it.
#[test]
fn a_command_in_flight_when_the_runner_stops() {
    let bench = Bench::new();
    let (open, until) = gate();
    bench.flume.hold("a", until);
    let mut plane = Fake::holding(offers(&["a"]));
    carried(&bench, |part| {
        part.pass(&mut plane);
        assert_eq!(
            answered(plane.reports().last(), Outcome::Launched),
            [] as [String; 0]
        );
        let opener = std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(100));
            open.store(true, Ordering::SeqCst);
        });
        let mut lines = Vec::new();
        part.carrier
            .finish(&mut plane, part.underway, &mut |line| lines.push(line));
        opener.join().unwrap();
        assert!(part.underway.idle());
        assert_eq!(answered(plane.reports().last(), Outcome::Launched), ["a"]);
        assert_eq!(lines, [format!("a: launched at {}", place_of("a"))]);
    });
}

/// The work-into-flume spec's "A slow launch keeps its room": stream
/// `work` has room for one, a part's pass starts `a`'s slow launch into
/// it, and its next pass, whose listing does not hold `a`'s place yet, is
/// offered `b` into `work`. `b` is not launched while `a`'s launch runs.
#[test]
fn a_slow_launch_keeps_its_room() {
    let bench = Bench::new();
    bench.flume.host(serde_json::json!({
        "running": 0, "bound": null, "room": null, "queue": [],
        "streams": [{ "name": "work", "priority": 0, "running": 0, "bound": 1, "room": 1, "queue": [] }],
    }));
    let (open, until) = gate();
    bench.flume.hold("a", until);
    let in_work = |id: &str| {
        let mut offered = offer(id, id);
        offered.launch.stream = Some("work".into());
        Want::Offer(offered)
    };
    let mut plane = Fake::holding(vec![in_work("a")]);
    carried(&bench, |part| {
        part.pass(&mut plane);
        plane.set_wants(vec![in_work("a"), in_work("b")]);
        part.pass(&mut plane);
        assert!(!launched(&bench.flume.calls(), "b"), "b waits for room");
        open.store(true, Ordering::SeqCst);
        part.collect_until(&mut plane, |_| part.underway.idle());
    });
}
