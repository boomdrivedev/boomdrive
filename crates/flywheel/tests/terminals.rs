//! A terminal the control plane opens on a launch's agent is relayed to
//! the herdr pane Flume's status names, through herdr's command line, on a
//! thread of its own; it closes with its link, its pane or its launch, and
//! nothing of it is kept (design Decision 16; core.11; the terminal-relay
//! spec, every requirement; Task notes 7.1). Each runs against the fake
//! control plane's link and a stub `herdr`.

mod fakes;

use fakes::{offer, place_of, Bench};
use flywheel::terminal::READ_INTERVAL;
use flywheel_control::fake::{Fake, FarEnd, Sent};
use flywheel_control::{Id, Input, Want};
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc::TryRecvError;
use std::thread;
use std::time::{Duration, Instant};

/// The pane `o1`'s agent shows in, and its session's socket.
const PANE: &str = "w1:p2";
const SOCKET: &str = "/run/herdr/herdr@1.sock";

/// The control plane offering `o1`, a launch named `a`, after a pass that
/// launches it and one that carries its status once its agent shows in
/// [`PANE`] at [`SOCKET`], with the lines both passes said.
fn shown(bench: &Bench) -> (Fake, Vec<String>) {
    let mut plane = Fake::holding(vec![Want::Offer(offer("o1", "a"))]);
    let (_, mut lines) = bench.pass(&mut plane);
    let agent = format!("herdr:{PANE}@{SOCKET}");
    let clues = json!({ "shown_at": { "agent": agent } });
    bench.flume.set(&place_of("a"), "clues", clues);
    lines.extend(bench.pass(&mut plane).1);
    (plane, lines)
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

/// The words the link was closed with, past every screen sent before.
fn closed(far: &FarEnd) -> String {
    loop {
        match next(far) {
            Sent::Screen(_) => continue,
            Sent::Closed(words) => return words,
        }
    }
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
/// closed with words naming the launch, and no `herdr` runs.
#[test]
fn a_launch_not_held() {
    let bench = Bench::new();
    let (mut plane, _) = shown(&bench);
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
    assert_eq!(bench.herdr.calls(), Vec::<Vec<String>>::new());
    assert!(bench.relay_lines().contains(
        &"t1: the terminal on o9 is not opened: this runner holds no launch o9 under lease 1"
            .to_string()
    ));
}

/// The terminal-relay spec's "No agent running": a launch held whose
/// carried status names no pane closes its terminal at once, and no
/// `herdr` runs.
#[test]
fn no_agent_running() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![Want::Offer(offer("o1", "a"))]);
    bench.pass(&mut plane);
    let (far, _) = opened(&bench, &mut plane, "o1", 1);
    assert_eq!(
        next(&far),
        Sent::Closed("no agent is running in its place".to_string())
    );
    assert!(bench.relays.relaying().is_empty());
    assert_eq!(bench.herdr.calls(), Vec::<Vec<String>>::new());
}

/// The terminal-relay spec's "A screen sent once while it stands": the
/// stub prints one screen at three reads, then another, and the link
/// receives two screens; each read names the pane, at its socket.
#[test]
fn a_screen_sent_once_while_it_stands() {
    let bench = Bench::new();
    let one = "\u{1b}[1mone\u{1b}[0m\r\n$ ";
    bench.herdr.screen(1, one);
    bench.herdr.screen(4, "two\r\n");
    let (mut plane, _) = shown(&bench);
    let (far, _) = opened(&bench, &mut plane, "o1", 1);
    assert_eq!(next(&far), Sent::Screen(one.to_string()));
    assert_eq!(next(&far), Sent::Screen("two\r\n".to_string()));
    assert!(bench.herdr.reads() >= 4);
    let reads = bench.herdr.reads();
    wait_until("the pane is read twice more", || {
        bench.herdr.reads() >= reads + 2
    });
    assert_eq!(far.sent.try_recv(), Err(TryRecvError::Empty));
    drop(far);
    ended(&bench);
    let read = argv(&[
        "pane", "read", PANE, "--source", "visible", "--format", "ansi",
    ]);
    let calls = bench.herdr.calls();
    assert!(calls.len() >= 6);
    assert!(calls.iter().all(|call| *call == read), "{calls:?}");
    let sockets = bench.herdr.sockets();
    assert!(sockets.iter().all(|socket| socket == SOCKET), "{sockets:?}");
}

/// The terminal-relay spec's "Input passed in": the text `yes` and then
/// the keys `enter` are passed to the pane in order, and a set of keys is
/// passed as one call naming each.
#[test]
fn input_passed_in() {
    let bench = Bench::new();
    bench.herdr.screen(1, "Proceed? ");
    let (mut plane, _) = shown(&bench);
    let (far, _) = opened(&bench, &mut plane, "o1", 1);
    assert_eq!(next(&far), Sent::Screen("Proceed? ".to_string()));
    let send = |input: Input| far.input.send(input).expect("the link takes input");
    send(Input::Text("yes".to_string()));
    send(Input::Keys(vec!["enter".to_string()]));
    send(Input::Keys(vec!["ctrl+c".to_string(), "esc".to_string()]));
    wait_until("the keys are passed", || {
        bench
            .herdr
            .calls()
            .iter()
            .filter(|call| call[1] == "send-keys")
            .count()
            == 2
    });
    let passed: Vec<Vec<String>> = bench
        .herdr
        .calls()
        .into_iter()
        .filter(|call| call[1] != "read")
        .collect();
    assert_eq!(
        passed,
        [
            argv(&["pane", "send-text", PANE, "yes"]),
            argv(&["pane", "send-keys", PANE, "enter"]),
            argv(&["pane", "send-keys", PANE, "ctrl+c", "esc"]),
        ]
    );
    let sockets = bench.herdr.sockets();
    assert!(sockets.iter().all(|socket| socket == SOCKET), "{sockets:?}");
}

/// The terminal-relay spec's "The pane gone": herdr refuses a read, so
/// the link is closed with herdr's words and the thread ends, reading no
/// more.
#[test]
fn the_pane_gone() {
    let bench = Bench::new();
    bench.herdr.refuse("pane w1:p2 not found");
    let (mut plane, _) = shown(&bench);
    let (far, _) = opened(&bench, &mut plane, "o1", 1);
    assert_eq!(
        next(&far),
        Sent::Closed("herdr refused a read of the pane: pane w1:p2 not found".to_string())
    );
    ended(&bench);
    assert_eq!(bench.herdr.reads(), 1);
}

/// Task notes 7.1, closed when the link closes: the far end closes it, and
/// the thread ends and reads no more.
#[test]
fn the_link_closed() {
    let bench = Bench::new();
    bench.herdr.screen(1, "screen");
    let (mut plane, _) = shown(&bench);
    let (far, _) = opened(&bench, &mut plane, "o1", 1);
    assert_eq!(next(&far), Sent::Screen("screen".to_string()));
    drop(far.input);
    ended(&bench);
    let reads = bench.herdr.reads();
    thread::sleep(READ_INTERVAL * 3);
    assert_eq!(bench.herdr.reads(), reads);
    assert!(bench
        .relay_lines()
        .contains(&"t1: the terminal on o1 at pane w1:p2 is closed: its link closed".to_string()));
}

/// The far end stops listening: the next screen to send finds no one, so
/// the terminal closes.
#[test]
fn the_far_end_stops_listening() {
    let bench = Bench::new();
    for read in 1..=40 {
        bench.herdr.screen(read, &format!("screen {read}"));
    }
    let (mut plane, _) = shown(&bench);
    let (far, _) = opened(&bench, &mut plane, "o1", 1);
    assert_eq!(next(&far), Sent::Screen("screen 1".to_string()));
    let FarEnd { input, sent } = far;
    drop(sent);
    ended(&bench);
    drop(input);
    assert!(bench.relay_lines().contains(
        &"t1: the terminal on o1 at pane w1:p2 is closed: its far end stopped listening"
            .to_string()
    ));
}

/// The terminal-relay spec's "The launch let go": a report refusing the
/// terminal's launch as no longer this runner's closes the terminal.
#[test]
fn the_launch_let_go() {
    let bench = Bench::new();
    bench.herdr.screen(1, "screen");
    let (mut plane, _) = shown(&bench);
    let (far, _) = opened(&bench, &mut plane, "o1", 1);
    assert_eq!(next(&far), Sent::Screen("screen".to_string()));
    plane.refuse(Id::new("o1"), "its lease is another runner's");
    plane.set_wants(Vec::new());
    bench.pass(&mut plane);
    assert_eq!(closed(&far), "its launch o1 is let go");
    ended(&bench);
}

/// A launch the control plane carries no more closes its terminal too,
/// and leaves a terminal on another launch relaying.
#[test]
fn a_launch_the_control_plane_lets_go() {
    let bench = Bench::new();
    bench.herdr.screen(1, "screen");
    let (mut plane, _) = shown(&bench);
    let (far, _) = opened(&bench, &mut plane, "o1", 1);
    assert_eq!(next(&far), Sent::Screen("screen".to_string()));
    plane.let_go(Id::new("o1"));
    bench.pass(&mut plane);
    assert_eq!(closed(&far), "its launch o1 is let go");
    ended(&bench);
}

/// The terminal-relay spec's "Nothing written": a screen and an input
/// relayed, then closed, leave nothing of either in the data directory, a
/// line the runner said or a report; the lines name the terminal, its
/// launch and the pane.
#[test]
fn nothing_written() {
    let screen = "secret-screen-text";
    let typed = "secret-typed-text";
    let bench = Bench::new();
    bench
        .herdr
        .screen(1, &format!("\u{1b}[32m{screen}\u{1b}[0m"));
    let (mut plane, mut lines) = shown(&bench);
    let (far, opening) = opened(&bench, &mut plane, "o1", 1);
    lines.extend(opening);
    assert!(matches!(next(&far), Sent::Screen(text) if text.contains(screen)));
    far.input
        .send(Input::Text(typed.to_string()))
        .expect("the link takes input");
    wait_until("the text is passed", || {
        bench
            .herdr
            .calls()
            .iter()
            .any(|call| call[1] == "send-text")
    });
    drop(far);
    ended(&bench);
    lines.extend(bench.pass(&mut plane).1);
    lines.extend(bench.relay_lines());
    let files = files(&bench.data());
    assert!(files.iter().any(|(path, _)| path.ends_with("report.json")));
    for secret in [screen, typed] {
        for (path, bytes) in &files {
            let text = String::from_utf8_lossy(bytes);
            assert!(!text.contains(secret), "{} holds {secret}", path.display());
        }
        for line in &lines {
            assert!(!line.contains(secret), "{line:?} holds {secret}");
        }
        for report in plane.reports() {
            let sent = serde_json::to_string(report).expect("the report as JSON");
            assert!(!sent.contains(secret), "a report carries {secret}");
        }
    }
    let relay_lines = bench.relay_lines();
    assert_eq!(
        relay_lines,
        [
            "t1: the terminal on o1 relays pane w1:p2",
            "t1: the terminal on o1 at pane w1:p2 is closed: its link closed",
        ]
    );
}

/// Every file under `dir`, with its bytes.
fn files(dir: &Path) -> Vec<(PathBuf, Vec<u8>)> {
    let mut found = Vec::new();
    for entry in fs::read_dir(dir).expect("the directory reads") {
        let path = entry.expect("its entry").path();
        if path.is_dir() {
            found.extend(files(&path));
        } else {
            let bytes = fs::read(&path).expect("the file reads");
            found.push((path, bytes));
        }
    }
    found
}
