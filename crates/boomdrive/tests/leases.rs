//! Every report names each lease the runner holds with its work, and so
//! renews it; the control pass is due before a lease lapses; and a launch
//! whose report is refused, its lease no longer the runner's, is let go,
//! its place left as Flume holds it (design Decision 11; core.10; the
//! work-into-flume spec's "A lease is renewed by every report and let go
//! once refused"; the control-plane-interface spec's "A launch let go";
//! Task notes 4.6). The timing function's own test is in `carry.rs`.

mod fakes;

use boomdrive::carry;
use boomdrive_control::fake::Fake;
use boomdrive_control::{ActSort, At, Id, Lease, Offer, Outcome, Want};
use fakes::{act, offer, place_of, Bench, Call};
use std::collections::BTreeMap;
use std::time::Duration;

/// An offer `id` named `name` under lease `number`, expiring at
/// `expires` where given.
fn leased(id: &str, name: &str, number: u64, expires: Option<&str>) -> Offer {
    let mut offer = offer(id, name);
    offer.lease = Lease {
        number,
        expires: expires.map(|text| At::parse(text).unwrap()),
    };
    offer
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
    let expiry = "2026-11-01T10:00:40Z";
    plane.set_wants(vec![
        Want::Offer(leased("a", "a", 2, Some(expiry))),
        Want::Offer(leased("b", "b", 4, Some("2026-11-01T10:05:00Z"))),
    ]);
    let (passed, _) = bench.pass(&mut plane);
    let held = bench.report().launches[&Id::new("a")].lease;
    assert_eq!(
        held,
        Lease {
            number: 2,
            expires: Some(At::parse(expiry).unwrap())
        }
    );
    assert_eq!(passed.expires, Some(At::parse(expiry).unwrap()));
    assert_eq!(bench.flume.launches().len(), 2, "a is not launched again");
}

/// The work-into-flume spec's "Renewed before it lapses", from a pass's
/// own expiry: a lease expiring 40 seconds after the pass ended has the
/// next pass due 20 seconds after it.
#[test]
fn renewed_before_it_lapses() {
    let ended = At::parse("2026-11-01T10:00:00Z").unwrap();
    let expires = At::parse("2026-11-01T10:00:40Z").unwrap();
    let left = carry::left(expires, ended);
    let began = std::time::Instant::now();
    let finished = began + Duration::from_secs(3);
    let due = carry::next_due(began, finished, Duration::from_secs(60), Some(left));
    assert!(due <= finished + Duration::from_secs(20));
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
