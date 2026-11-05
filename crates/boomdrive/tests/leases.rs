//! Every report names each lease the runner holds with its work, and so
//! renews it; the control pass is due before a lease not yet lapsed
//! lapses, and only after a pass that reported; work under a lapsed lease
//! waits for a live one; and a launch whose report is refused, its lease
//! no longer the runner's, is let go, its place left as Flume holds it
//! (design Decisions 11 and 13; core.10, core.22, core.24; the
//! work-into-flume spec's "A lease is renewed by every report and let go
//! once refused" and "Work under a lapsed lease waits for a live one";
//! the running spec's "boomdrive run makes control passes on a timer of
//! their own"; the control-plane-interface spec's "A launch let go"; Task
//! notes 1.1 and 1.3). The timing function's own tests are in `carry.rs`.

mod fakes;

use boomdrive_control::fake::Fake;
use boomdrive_control::{
    ActSort, At, Command, CommandSort, Id, Lease, Offer, Outcome, Report, Want,
};
use fakes::{act, offer, place_of, Bench, Call, KEPT};
use std::collections::BTreeMap;
use std::time::{Duration, Instant};

/// The poll interval the loop's timing is read at.
const MINUTE: Duration = Duration::from_secs(60);

/// The time `seconds` after now.
fn after(seconds: u64) -> At {
    At::from(At::now().time() + Duration::from_secs(seconds))
}

/// The time `seconds` before now.
fn before(seconds: u64) -> At {
    At::from(At::now().time() - Duration::from_secs(seconds))
}

/// An offer `id` named `name` under lease `number`, expiring at
/// `expires` where given.
fn leased(id: &str, name: &str, number: u64, expires: Option<At>) -> Offer {
    let mut offer = offer(id, name);
    offer.lease = Lease { number, expires };
    offer
}

/// The command `branch/1`, asking for `bolt/x` from `main` in [`KEPT`],
/// under lease `number` expiring at `expires`.
fn branch(number: u64, expires: At) -> Want {
    Want::Command(Command {
        id: Id::new("branch/1"),
        lease: Lease {
            number,
            expires: Some(expires),
        },
        what: CommandSort::Branch {
            repository: KEPT.to_string(),
            branch: "bolt/x".to_string(),
            from: "main".to_string(),
        },
    })
}

/// A retry of launch `launch`'s place, as act `<launch>/retry`.
fn retry(launch: &str) -> Want {
    let what = ActSort::Retry {
        stage: None,
        words: None,
        submission: false,
    };
    act(&format!("{launch}/retry"), launch, what)
}

/// Each call Flume was asked that names the place at `place`.
fn naming(calls: &[Call], place: &str) -> Vec<Call> {
    calls
        .iter()
        .filter(|call| match call {
            Call::Act(at, _) | Call::StatusOf(_, at) => at == place,
            Call::Launch(_) | Call::Status => false,
        })
        .cloned()
        .collect()
}

/// The report's lease numbers are named in the hello and in the report
/// sent, a held launch's and an act's among them.
#[test]
fn each_lease_named_in_the_hello_and_the_report() {
    let bench = Bench::new();
    let mut b = leased("b", "b", 5, None);
    b.launch.after = vec![Id::new("a")];
    let mut plane = Fake::holding(vec![
        Want::Offer(leased("a", "a", 3, None)),
        Want::Offer(b),
        act("a/priority", "a", ActSort::Priority(2)),
    ]);
    bench.pass(&mut plane);
    bench.pass(&mut plane);
    let leases = BTreeMap::from([(Id::new("a"), 3), (Id::new("b"), 5)]);
    assert_eq!(plane.hellos()[1].leases, leases);
    for report in plane.reports() {
        assert_eq!(report.launches[&Id::new("a")].lease.number, 3);
        assert_eq!(report.launches[&Id::new("b")].lease.number, 5);
        assert_eq!(report.launches[&Id::new("b")].answer.outcome, Outcome::Held);
        assert_eq!(report.acts[&Id::new("a/priority")].lease, 1);
    }
}

/// A lease the control plane gives anew with an offer is the one the
/// report holds, and the pass leaves its expiry for the loop.
#[test]
fn a_lease_given_anew() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![Want::Offer(leased("a", "a", 1, None))]);
    let (passed, _) = bench.pass(&mut plane);
    assert_eq!(passed.expires, None);
    let expiry = after(40);
    plane.set_wants(vec![
        Want::Offer(leased("a", "a", 2, Some(expiry))),
        Want::Offer(leased("b", "b", 4, Some(after(300)))),
    ]);
    let (passed, _) = bench.pass(&mut plane);
    let held = bench.report().launches[&Id::new("a")].lease;
    assert_eq!(
        held,
        Lease {
            number: 2,
            expires: Some(expiry)
        }
    );
    assert_eq!(passed.expires, Some(expiry));
    assert_eq!(bench.flume.launches().len(), 2, "a is not launched again");
}

/// The work-into-flume spec's "Renewed before it lapses", from a pass's
/// own expiry: a lease expiring 40 seconds after the pass ended has the
/// next pass due no later than 20 seconds after it.
#[test]
fn renewed_before_it_lapses() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![Want::Offer(leased("a", "a", 1, Some(after(40))))]);
    let began = Instant::now();
    let (passed, _) = bench.pass(&mut plane);
    let ended = Instant::now();
    assert!(passed.reported);
    assert!(passed.next_due(began, ended, MINUTE) <= ended + Duration::from_secs(20));
}

/// The running spec's "A lapsed lease": the runner holds a lease that
/// lapsed before the pass, and the control plane cannot be reached, so
/// the next pass is `poll_interval` after this one began, and none comes
/// between. A pass that reported is hastened by no lapsed lease either.
#[test]
fn a_lapsed_lease() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![Want::Offer(leased("a", "a", 1, None))]);
    bench.pass(&mut plane);
    plane.set_wants(vec![Want::Offer(leased("a", "a", 2, Some(before(60))))]);
    let began = Instant::now();
    let (passed, _) = bench.pass(&mut plane);
    let ended = Instant::now();
    assert_eq!(bench.report().launches[&Id::new("a")].lease.number, 2);
    assert!(passed.reported);
    assert_eq!(passed.expires, None);
    assert_eq!(passed.next_due(began, ended, MINUTE), began + MINUTE);

    plane.refuse_pulls("no network");
    let began = Instant::now();
    let (passed, _) = bench.pass(&mut plane);
    let ended = Instant::now();
    assert!(!passed.reported);
    assert_eq!(passed.expires, None);
    assert_eq!(passed.next_due(began, ended, MINUTE), began + MINUTE);
}

/// The running spec's "A pass that did not report": the runner holds a
/// lease expiring 40 seconds after the pass, and the pass's report was
/// not sent, so the next pass is `poll_interval` after this one began.
#[test]
fn a_pass_that_did_not_report() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![Want::Offer(leased("a", "a", 1, Some(after(40))))]);
    plane.refuse_reports("no network");
    let began = Instant::now();
    let (passed, _) = bench.pass(&mut plane);
    let ended = Instant::now();
    assert!(!passed.reported);
    assert!(passed.expires.is_some());
    assert_eq!(passed.next_due(began, ended, MINUTE), began + MINUTE);
}

/// The work-into-flume spec's "An offer whose lease lapsed": offers
/// under a lease that expired a minute before the pass are neither
/// launched, held nor refused, one in a repository the host does not
/// keep and one waiting on another among them; once a pull offers one
/// under a live lease, it is launched.
#[test]
fn an_offer_whose_lease_lapsed() {
    let bench = Bench::new();
    let lapsed = Some(before(60));
    let mut unkept = leased("b", "b", 1, lapsed);
    unkept.launch.repository = "example/service".to_string();
    let mut waiting = leased("c", "c", 1, lapsed);
    waiting.launch.after = vec![Id::new("x")];
    let mut plane = Fake::holding(vec![
        Want::Offer(leased("a", "a", 1, lapsed)),
        Want::Offer(unkept),
        Want::Offer(waiting),
    ]);
    let (passed, lines) = bench.pass(&mut plane);
    assert!(passed.reported);
    assert_eq!(lines, Vec::<String>::new());
    assert!(bench.flume.launches().is_empty());
    assert_eq!(plane.reports(), [Report::default()]);

    plane.set_wants(vec![Want::Offer(leased("a", "a", 2, Some(after(300))))]);
    bench.pass(&mut plane);
    assert_eq!(bench.flume.launches().len(), 1);
    let launched = &bench.report().launches[&Id::new("a")];
    assert_eq!(launched.answer.outcome, Outcome::Launched);
}

/// The work-into-flume spec's "An act on a lapsed launch": the runner
/// holds launch `a` at its place under a lease that has expired, and a
/// retry of `a` runs no Flume command naming the place, and has no answer
/// in the report, while `a`'s own answer stands; once a pull gives `a`
/// under a live lease, the retry is carried out.
#[test]
fn an_act_on_a_lapsed_launch() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![Want::Offer(leased("a", "a", 1, None))]);
    bench.pass(&mut plane);
    let p = place_of("a");
    bench.flume.take_calls();

    plane.set_wants(vec![
        Want::Offer(leased("a", "a", 2, Some(before(60)))),
        retry("a"),
    ]);
    let (_, lines) = bench.pass(&mut plane);
    assert_eq!(lines, Vec::<String>::new());
    assert_eq!(naming(&bench.flume.calls(), &p), []);
    let report = bench.report();
    assert!(!report.acts.contains_key(&Id::new("a/retry")));
    let launched = &report.launches[&Id::new("a")];
    assert_eq!(
        (
            launched.answer.outcome,
            launched.answer.place_dir.as_deref()
        ),
        (Outcome::Launched, Some(p.as_str())),
        "the launch's answer stands"
    );

    plane.set_wants(vec![
        Want::Offer(leased("a", "a", 3, Some(after(300)))),
        retry("a"),
    ]);
    bench.pass(&mut plane);
    assert_eq!(bench.flume.acts().len(), 1);
    let acted = &bench.report().acts[&Id::new("a/retry")];
    assert_eq!(acted.outcome, Outcome::Done);
}

/// The work-into-flume spec's "A command whose lease lapsed": a branch
/// command under a lease that has expired runs no `sluice` command, and
/// has no answer in the report; once a pull gives it under a live lease,
/// sluice is run.
#[test]
fn a_command_whose_lease_lapsed() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![branch(1, before(60))]);
    let (passed, lines) = bench.pass(&mut plane);
    assert!(passed.reported);
    assert_eq!(lines, Vec::<String>::new());
    assert_eq!(bench.sluice.calls(), Vec::<Vec<String>>::new());
    assert_eq!(plane.reports(), [Report::default()]);

    plane.set_wants(vec![branch(2, after(300))]);
    bench.pass(&mut plane);
    assert_eq!(bench.sluice.calls().len(), 1);
}

/// The work-into-flume spec's "A lease refused".
#[test]
fn a_lease_refused() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![Want::Offer(offer("a", "a"))]);
    plane.refuse(Id::new("a"), "lease 2 of a is vega's");
    let (_, lines) = bench.pass(&mut plane);
    let p = place_of("a");
    assert!(!bench.report().launches.contains_key(&Id::new("a")));
    assert_eq!(
        lines.last().unwrap(),
        &format!(
            "a: let go, its lease no longer this runner's: lease 2 of a is vega's: \
             its place {p} is left on this host as Flume holds it"
        )
    );
    bench.flume.take_calls();
    plane.set_wants(vec![act("a/drop", "a", ActSort::Drop)]);
    bench.pass(&mut plane);
    let acted = &bench.report().acts[&Id::new("a/drop")];
    assert_eq!(
        (acted.outcome, acted.words.as_deref()),
        (Outcome::Refused, Some("a is no launch of this runner's"))
    );
    assert_eq!(naming(&bench.flume.calls(), &p), []);
    assert!(
        bench.flume.entry(&p).is_some(),
        "the place is left standing"
    );
    let last = plane.reports().last().unwrap();
    assert!(!last.launches.contains_key(&Id::new("a")));
}

/// The control-plane-interface spec's "A launch let go".
#[test]
fn a_launch_let_go() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![
        Want::Offer(offer("a", "a")),
        Want::Offer(offer("b", "b")),
    ]);
    bench.pass(&mut plane);
    plane.let_go(Id::new("a"));
    let (_, lines) = bench.pass(&mut plane);
    assert_eq!(lines, ["a: let go by the control plane"]);
    assert!(!bench.report().launches.contains_key(&Id::new("a")));
    bench.pass(&mut plane);
    let last = plane.reports().last().unwrap();
    assert!(!last.launches.contains_key(&Id::new("a")));
    assert!(last.launches.contains_key(&Id::new("b")));
    assert_eq!(bench.flume.launches().len(), 2);
}
