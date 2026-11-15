//! A terminal the control plane opens on a launch held at a place is
//! relayed through Flume's relay of that place, `flume --json relay
//! <place>`, on a thread of its own: Flume's screens out as Flume gives
//! them, the far end's input in as one line each, closed by closing
//! Flume's input or by Flume's end, and nothing of it is kept (the
//! terminal-relay spec; core.11, core.13 and core.29; record 0004 as
//! amended on 2026-11-15). Each runs against the fake control plane's
//! link and a stub `flume` that relays.

mod fakes;

use boomdrive_control::fake::{Fake, FarEnd, Sent};
use boomdrive_control::{Id, Want};
use boomdrive_control::{Offer, Outcome};
use fakes::{offer, place_of, Bench};
use std::thread;
use std::time::{Duration, Instant};

/// The control plane offering `o1`, a launch named `a`, after a pass that
/// launches it, so the report holds `o1` under lease 1 at
/// `place_of("a")`.
fn held(bench: &Bench) -> Fake {
    let mut plane = Fake::holding(vec![Want::Offer(offer("o1", "a"))]);
    bench.pass(&mut plane);
    plane
}

/// Terminal `t1` on `launch` under lease `lease`, opened at a pass, its
/// far end and the lines the pass said.
fn opened(bench: &Bench, plane: &mut Fake, launch: &str, lease: u64) -> (FarEnd, Vec<String>) {
    let far = plane.open_terminal(Id::new("t1"), Id::new(launch), lease);
    let (_, lines) = bench.pass(plane);
    (far, lines)
}

/// What the runner sent next on the link.
fn next(far: &FarEnd) -> Sent {
    far.sent
        .recv_timeout(Duration::from_secs(10))
        .expect("the runner sends on the link")
}

fn wait_until(what: &str, done: impl Fn() -> bool) {
    let deadline = Instant::now() + Duration::from_secs(10);
    while !done() {
        assert!(Instant::now() < deadline, "no sign that {what}");
        thread::sleep(Duration::from_millis(20));
    }
}

/// Waits until every terminal's thread has ended.
fn ended(bench: &Bench) {
    wait_until("the terminal's thread ends", || {
        bench.relays.relaying().is_empty()
    });
}

fn argv(words: &[&str]) -> Vec<String> {
    words.iter().map(|word| word.to_string()).collect()
}

/// The terminal-relay spec's "A launch not held": a terminal on a launch
/// the report does not hold, or holds under another lease number, is
/// closed with words naming the launch and the lease, and no `flume
/// relay` runs.
#[test]
fn a_launch_not_held() {
    let bench = Bench::new();
    let mut plane = held(&bench);
    let (far, _) = opened(&bench, &mut plane, "o9", 1);
    assert_eq!(
        next(&far),
        Sent::Closed("this runner holds no launch o9 under lease 1".to_string())
    );
    let (far, _) = opened(&bench, &mut plane, "o1", 2);
    assert_eq!(
        next(&far),
        Sent::Closed("this runner holds no launch o1 under lease 2".to_string())
    );
    assert!(bench.relays.relaying().is_empty());
    assert_eq!(bench.flume_relay.calls(), Vec::<Vec<String>>::new());
    assert!(bench.relay_lines().contains(
        &"t1: the terminal on o9 is not opened: this runner holds no launch o9 under lease 1"
            .to_string()
    ));
}

/// The terminal-relay spec's "A launch at no place": a terminal on a
/// launch the report holds unlaunched, waiting on its After, is closed
/// with words saying it has no place Flume named, and no `flume relay`
/// runs.
#[test]
fn a_launch_at_no_place() {
    let bench = Bench::new();
    let mut after: Offer = offer("o2", "b");
    after.launch.after = vec![Id::new("o1")];
    let mut plane = Fake::holding(vec![Want::Offer(offer("o1", "a")), Want::Offer(after)]);
    bench.pass(&mut plane);
    let report = bench.report();
    assert_eq!(
        report.launches[&Id::new("o1")].answer.place_dir.as_deref(),
        Some(place_of("a").as_str())
    );
    let held = &report.launches[&Id::new("o2")].answer;
    assert_eq!(
        (held.outcome, held.place_dir.as_deref()),
        (Outcome::Held, None)
    );
    let (far, _) = opened(&bench, &mut plane, "o2", 1);
    assert_eq!(
        next(&far),
        Sent::Closed("o2 has no place Flume named".to_string())
    );
    assert!(bench.relays.relaying().is_empty());
    assert_eq!(bench.flume_relay.calls(), Vec::<Vec<String>>::new());
}

/// The terminal-relay spec's "Flume refuses the relay": the stub `flume`
/// is run once as `flume --json relay <place>`, and its refusal closes
/// the link in its words.
#[test]
fn flumes_refusal_closes_the_terminal() {
    let bench = Bench::new();
    let refusal = "fix--a has no live agent: waiting for room";
    bench.flume_relay.refuse(refusal);
    let mut plane = held(&bench);
    let (far, _) = opened(&bench, &mut plane, "o1", 1);
    assert_eq!(next(&far), Sent::Closed(refusal.to_string()));
    ended(&bench);
    assert_eq!(
        bench.flume_relay.calls(),
        [argv(&["--json", "relay", &place_of("a")])]
    );
}

/// The terminal-relay spec's "A Flume with no relay": a Flume before
/// `relay` answers with its usage error on standard error, which closes
/// the link, its one final newline left off.
#[test]
fn a_flume_with_no_relay() {
    let bench = Bench::new();
    bench
        .flume_relay
        .usage("error: unrecognized subcommand 'relay'\n");
    let mut plane = held(&bench);
    let (far, _) = opened(&bench, &mut plane, "o1", 1);
    assert_eq!(
        next(&far),
        Sent::Closed("error: unrecognized subcommand 'relay'".to_string())
    );
    ended(&bench);
}
