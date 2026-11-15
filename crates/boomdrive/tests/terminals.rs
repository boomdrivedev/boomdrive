//! A terminal the control plane opens on a launch held at a place is
//! relayed through Flume's relay of that place, `flume --json relay
//! <place>`, on a thread of its own: Flume's screens out as Flume gives
//! them, the far end's input in as one line each, closed by closing
//! Flume's input or by Flume's end, and nothing of it is kept (the
//! terminal-relay spec; core.11, core.13 and core.29; record 0004 as
//! amended on 2026-11-15). Each runs against the fake control plane's
//! link and a stub `flume` that relays.

mod fakes;

use boomdrive::flume;
use boomdrive::terminal::Relays;
use boomdrive_control::fake::{Fake, FarEnd, Sent};
use boomdrive_control::shared;
use boomdrive_control::{ControlPlane, Id, Input, Offer, Outcome, Want};
use fakes::{offer, place_of, Bench};
use serde_json::json;
use std::sync::{Arc, Mutex};
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

/// The terminal-relay spec's "Screens as Flume gives them": the stub
/// writes three screens, the second and third the same, and the link
/// receives the three, in order, each as the stub wrote it; Flume ran
/// with `FLUME_RECIPIENT` unset, though the runner's own environment
/// names one.
#[test]
fn screens_as_flume_gives_them() {
    std::env::set_var("FLUME_RECIPIENT", "someone/else");
    let bench = Bench::new();
    let one = "\u{1b}[1mone\u{1b}[0m\r\n$ ";
    for screen in [one, "one", "one"] {
        bench.flume_relay.out(&json!({ "screen": screen }));
    }
    let mut plane = held(&bench);
    let (far, _) = opened(&bench, &mut plane, "o1", 1);
    assert_eq!(next(&far), Sent::Screen(one.to_string()));
    assert_eq!(next(&far), Sent::Screen("one".to_string()));
    assert_eq!(next(&far), Sent::Screen("one".to_string()));
    assert_eq!(bench.flume_relay.recipient(1).as_deref(), Some("unset"));
    assert_eq!(
        bench.flume_relay.calls(),
        [argv(&["--json", "relay", &place_of("a")])]
    );
}

/// The terminal-relay spec's "An unread read and an untyped input": an
/// unread read is said with Flume's words, an untyped input without
/// them, each naming the terminal, its launch and its place, and the
/// relay goes on to send the screen after them.
#[test]
fn an_unread_read_and_an_untyped_input() {
    let bench = Bench::new();
    let place = place_of("a");
    bench
        .flume_relay
        .out(&json!({ "unread": "herdr did not answer within 1s" }));
    bench
        .flume_relay
        .out(&json!({ "untyped": "secret-key-name is no key flume names" }));
    bench.flume_relay.out(&json!({ "screen": "after" }));
    let mut plane = held(&bench);
    let (far, _) = opened(&bench, &mut plane, "o1", 1);
    assert_eq!(next(&far), Sent::Screen("after".to_string()));
    let lines = bench.relay_lines();
    assert!(
        lines.contains(&format!(
            "t1: flume relay of {place} for o1 read nothing: herdr did not answer within 1s"
        )),
        "{lines:#?}"
    );
    let untyped: Vec<&String> = lines
        .iter()
        .filter(|line| line.contains("untyped"))
        .collect();
    assert_eq!(untyped.len(), 1, "{lines:#?}");
    let line = untyped[0];
    for named in ["t1", "o1", place.as_str()] {
        assert!(line.contains(named), "{line:?} names no {named}");
    }
    for line in &lines {
        assert!(!line.contains("secret-key-name"), "{line:?}");
    }
    assert!(!bench.relays.relaying().is_empty(), "the relay goes on");
}

/// The terminal-relay spec's "Input passed in": the text `yes` and then
/// the keys `ctrl+c` and `enter` are written to Flume's input, one line
/// each, as the link gave them.
#[test]
fn input_passed_in() {
    let bench = Bench::new();
    let mut plane = held(&bench);
    let (far, _) = opened(&bench, &mut plane, "o1", 1);
    let send = |input: Input| far.input.send(input).expect("the link takes input");
    send(Input::Text("yes".to_string()));
    send(Input::Keys(vec!["ctrl+c".to_string(), "enter".to_string()]));
    wait_until("both inputs are written", || {
        bench.flume_relay.inputs(1).len() == 2
    });
    assert_eq!(
        bench.flume_relay.inputs(1),
        [
            json!({ "text": "yes" }),
            json!({ "keys": ["ctrl+c", "enter"] })
        ]
    );
}

/// The words the link was closed with, past every screen sent before.
fn closed(far: &FarEnd) -> String {
    loop {
        match next(far) {
            Sent::Screen(_) => continue,
            Sent::Closed(words) => return words,
        }
    }
}

/// The terminal-relay spec's "The agent's terminal gone": Flume answers
/// that its relay ended `gone`, the runner having closed nothing, so the
/// link is closed in those words and the thread ends.
#[test]
fn the_agents_terminal_gone() {
    let bench = Bench::new();
    let mut plane = held(&bench);
    let (far, _) = opened(&bench, &mut plane, "o1", 1);
    wait_until("flume relays", || !bench.flume_relay.calls().is_empty());
    bench.flume_relay.gone();
    assert_eq!(closed(&far), "Flume's relay ended: gone");
    ended(&bench);
    assert!(bench.relay_lines().contains(&format!(
        "t1: the terminal on o1 at {} is closed: Flume's relay ended: gone",
        place_of("a")
    )));
}

/// The terminal-relay spec's "The link closed": the far end closes it, so
/// Flume's input reaches its end, Flume answers that its caller ended it
/// and exits, and the link is closed with `its link closed`.
#[test]
fn the_link_closed() {
    let bench = Bench::new();
    bench.flume_relay.out(&json!({ "screen": "screen" }));
    let mut plane = held(&bench);
    let (far, _) = opened(&bench, &mut plane, "o1", 1);
    assert_eq!(next(&far), Sent::Screen("screen".to_string()));
    far.input
        .send(Input::Close)
        .expect("the link takes its close");
    assert_eq!(closed(&far), "its link closed");
    ended(&bench);
    assert!(bench.flume_relay.closed(1), "flume's input reached its end");
    assert!(bench.relay_lines().contains(&format!(
        "t1: the terminal on o1 at {} is closed: its link closed",
        place_of("a")
    )));
}

/// The far end stops listening: the next screen Flume gives finds no one,
/// so the runner closes Flume's input and the terminal.
#[test]
fn the_far_end_stops_listening() {
    let bench = Bench::new();
    bench.flume_relay.out(&json!({ "screen": "screen 1" }));
    let mut plane = held(&bench);
    let (far, _) = opened(&bench, &mut plane, "o1", 1);
    assert_eq!(next(&far), Sent::Screen("screen 1".to_string()));
    let FarEnd { input, sent } = far;
    drop(sent);
    bench.flume_relay.out(&json!({ "screen": "screen 2" }));
    ended(&bench);
    drop(input);
    assert!(bench.flume_relay.closed(1), "flume's input reached its end");
    assert!(bench.relay_lines().contains(&format!(
        "t1: the terminal on o1 at {} is closed: its far end stopped listening",
        place_of("a")
    )));
}

/// The terminal-relay spec's "The launch let go": a report refusing the
/// terminal's launch as no longer this runner's closes Flume's input and
/// the terminal.
#[test]
fn the_launch_let_go() {
    let bench = Bench::new();
    bench.flume_relay.out(&json!({ "screen": "screen" }));
    let mut plane = held(&bench);
    let (far, _) = opened(&bench, &mut plane, "o1", 1);
    assert_eq!(next(&far), Sent::Screen("screen".to_string()));
    plane.refuse(Id::new("o1"), "its lease is another runner's");
    plane.set_wants(Vec::new());
    bench.pass(&mut plane);
    assert_eq!(closed(&far), "its launch o1 is let go");
    ended(&bench);
    assert!(bench.flume_relay.closed(1), "flume's input reached its end");
}

/// A launch the control plane carries no more closes Flume's input and
/// its terminal too.
#[test]
fn a_launch_the_control_plane_lets_go() {
    let bench = Bench::new();
    bench.flume_relay.out(&json!({ "screen": "screen" }));
    let mut plane = held(&bench);
    let (far, _) = opened(&bench, &mut plane, "o1", 1);
    assert_eq!(next(&far), Sent::Screen("screen".to_string()));
    plane.let_go(Id::new("o1"));
    bench.pass(&mut plane);
    assert_eq!(closed(&far), "its launch o1 is let go");
    ended(&bench);
    assert!(bench.flume_relay.closed(1), "flume's input reached its end");
}

/// The terminal-relay spec's "The runner stopped": the runner's terminals
/// dropped while one is open close Flume's input, and the drop returns
/// once Flume has exited, the link closed with `the runner stopped`.
#[test]
fn the_runner_stopped() {
    let bench = Bench::new();
    let mut plane = held(&bench);
    let report = bench.report();
    let far = plane.open_terminal(Id::new("t1"), Id::new("o1"), 1);
    let pulled = plane
        .pull(&shared::hello(&report), &report)
        .expect("a pull");
    let said = Arc::new(Mutex::new(Vec::new()));
    let lines = Arc::clone(&said);
    let relays = Relays::new(flume::Cli::new(bench.flume_relay.program()), move |line| {
        lines.lock().expect("the lines").push(line)
    });
    for terminal in pulled.terminals {
        relays.open(terminal, &report);
    }
    wait_until("flume relays", || !bench.flume_relay.calls().is_empty());
    drop(relays);
    assert!(bench.flume_relay.closed(1), "flume's input reached its end");
    assert_eq!(next(&far), Sent::Closed("the runner stopped".to_string()));
    assert!(said.lock().expect("the lines").contains(&format!(
        "t1: the terminal on o1 at {} is closed: the runner stopped",
        place_of("a")
    )));
}
