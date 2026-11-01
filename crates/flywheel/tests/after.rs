//! An offer waits, held, for each launch it comes after, and an act on a
//! launch not yet launched is answered from the report with no command
//! run (design Decisions 7 and 9; the work-into-flume spec's "An offer
//! waits for each launch it comes after", "A priority or a drop of a
//! launch not yet launched is answered from its report" and "A retry or a
//! done of a launch not launched, and an act on another's launch, are
//! refused"; Task notes 4.3).

mod fakes;

use fakes::{act, fixture, offer, place_of, printed, Bench, Call};
use flywheel::facts;
use flywheel::flume::Reply;
use flywheel_control::fake::Fake;
use flywheel_control::{ActSort, Id, Offer, Outcome, Want};
use serde_json::{json, Value};

/// An offer `id` named `name`, after each of `after`.
fn after(id: &str, name: &str, after: &[&str]) -> Want {
    let mut offer: Offer = offer(id, name);
    offer.launch.after = after.iter().map(|id| Id::new(*id)).collect();
    Want::Offer(offer)
}

/// The answer the report file holds for `launch`: its outcome and words.
fn answer(bench: &Bench, launch: &str) -> (Outcome, Option<String>) {
    let answer = &bench.report().launches[&Id::new(launch)].answer;
    (answer.outcome, answer.words.clone())
}

/// The answer the report file holds for act `id`.
fn acted(bench: &Bench, id: &str) -> (Outcome, Option<String>) {
    let acted = &bench.report().acts[&Id::new(id)];
    (acted.outcome, acted.words.clone())
}

/// `retry` with no stage, words or submission.
fn retry() -> ActSort {
    ActSort::Retry {
        stage: None,
        words: None,
        submission: false,
    }
}

/// The work-into-flume spec's "A dependent held, then launched".
#[test]
fn a_dependent_held_then_launched() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![Want::Offer(offer("a", "a")), after("b", "b", &["a"])]);
    let (_, lines) = bench.pass(&mut plane);
    let words = format!("waits on a (launched at {}, not landed)", place_of("a"));
    assert_eq!(answer(&bench, "b"), (Outcome::Held, Some(words.clone())));
    assert_eq!(
        lines,
        [
            format!("a: launched at {}", place_of("a")),
            format!("b: held: {words}")
        ]
    );
    let held_at = bench.report().launches[&Id::new("b")].answer.at;
    let (_, lines) = bench.pass(&mut plane);
    assert_eq!(lines, [] as [String; 0], "nothing changed");
    assert_eq!(bench.report().launches[&Id::new("b")].answer.at, held_at);
    bench.flume.land(&place_of("a"));
    bench.pass(&mut plane);
    assert_eq!(answer(&bench, "b").0, Outcome::Launched);
    let names: Vec<_> = bench.flume.launches().into_iter().map(|a| a.name).collect();
    assert_eq!(names, ["a", "b"]);
}

/// Task notes 4.3's first case, over Flume 0.4.1's own answers: `a`'s
/// place reads live as its launch answered, and landed as
/// `status-landed-kept.json` reads it.
#[test]
fn a_dependent_launched_once_flumes_own_status_reads_landed() {
    let bench = Bench::new();
    let launch: Value = serde_json::from_str(&fixture("launch.json")).unwrap();
    let live = facts::launched(&launch).unwrap().clone();
    let landed: Value = serde_json::from_str(&fixture("status-landed-kept.json")).unwrap();
    let listing = json!({ "status": { "places": [live], "host": landed["status"]["host"] } });
    bench.flume.listing(Ok(Reply::Given(printed(&listing))));
    let mut plane = Fake::holding(vec![
        Want::Offer(offer("scratch/1", "parser")),
        after("b", "b", &["scratch/1"]),
    ]);
    bench.pass(&mut plane);
    let place = facts::location(&live).unwrap().to_string();
    assert_eq!(answer(&bench, "scratch/1").0, Outcome::Launched);
    assert_eq!(
        answer(&bench, "b"),
        (
            Outcome::Held,
            Some(format!(
                "waits on scratch/1 (launched at {place}, not landed)"
            ))
        )
    );
    assert_eq!(bench.flume.launches(), []);
    bench
        .flume
        .listing(Ok(Reply::Given(fixture("status-landed-kept.json"))));
    bench.pass(&mut plane);
    assert_eq!(answer(&bench, "b").0, Outcome::Launched);
    assert_eq!(bench.flume.launches().len(), 1);
}

/// Held words name each launch waited on and where it stands: not yet
/// launched; launched at a place not landed; refused, with its words; and
/// dropped.
#[test]
fn held_words_for_each_standing() {
    let bench = Bench::new();
    let mut sluice = offer("z", "z");
    sluice.launch.repository = "agentplot/sluice".into();
    let mut plane = Fake::holding(vec![
        Want::Offer(offer("y", "y")),
        Want::Offer(sluice),
        after("w", "w", &["x"]),
        act("w/drop", "w", ActSort::Drop),
        after("b", "b", &["x", "y", "z", "w"]),
    ]);
    bench.pass(&mut plane);
    let words = format!(
        "waits on x (not yet launched), y (launched at {}, not landed), \
         z (refused: this host's configuration does not keep agentplot/sluice), \
         w (dropped, so its place will never land)",
        place_of("y")
    );
    assert_eq!(answer(&bench, "b"), (Outcome::Held, Some(words)));
}

/// The work-into-flume spec's "A dependent of a dropped launch".
#[test]
fn a_dependent_of_a_dropped_launch() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![
        after("a", "a", &["first"]),
        act("a/drop", "a", ActSort::Drop),
        after("b", "b", &["a"]),
    ]);
    bench.pass(&mut plane);
    bench.pass(&mut plane);
    assert_eq!(
        answer(&bench, "b"),
        (
            Outcome::Held,
            Some("waits on a (dropped, so its place will never land)".into())
        )
    );
    assert_eq!(bench.flume.launches(), []);
}

/// A held launch whose After has landed, with no room in its stream,
/// stays held, its words naming the stream.
#[test]
fn a_held_launch_with_no_room_names_its_stream() {
    let bench = Bench::new();
    let mut b: Offer = offer("b", "b");
    b.launch.after = vec![Id::new("a")];
    b.launch.stream = Some("work".into());
    let mut plane = Fake::holding(vec![Want::Offer(offer("a", "a")), Want::Offer(b)]);
    bench.pass(&mut plane);
    bench.flume.land(&place_of("a"));
    bench.flume.host(json!({
        "running": 1, "bound": null, "room": null, "queue": [],
        "streams": [{ "name": "work", "room": 0, "queue": [] }]
    }));
    let (_, lines) = bench.pass(&mut plane);
    assert_eq!(
        answer(&bench, "b"),
        (
            Outcome::Held,
            Some("waits for room in the stream work".into())
        )
    );
    assert_eq!(lines, ["b: held: waits for room in the stream work"]);
    assert_eq!(bench.flume.launches().len(), 1);
}

/// The work-into-flume spec's "A priority of a held launch".
#[test]
fn a_priority_of_a_held_launch() {
    let bench = Bench::new();
    let mut b: Offer = offer("b", "b");
    b.launch.after = vec![Id::new("a")];
    b.launch.priority = Some(2);
    let mut plane = Fake::holding(vec![
        Want::Offer(offer("a", "a")),
        Want::Offer(b),
        act("b/priority", "b", ActSort::Priority(7)),
    ]);
    let (_, lines) = bench.pass(&mut plane);
    assert_eq!(acted(&bench, "b/priority"), (Outcome::Done, None));
    assert!(lines.contains(&"b/priority: priority 7 of b: done".to_string()));
    assert_eq!(bench.flume.commands().len(), 1, "only a's launch");
    bench.flume.land(&place_of("a"));
    bench.pass(&mut plane);
    let launches = bench.flume.launches();
    assert_eq!(launches[1].name, "b");
    assert_eq!(launches[1].priority, Some(7));
}

/// The work-into-flume spec's "A drop of a held launch".
#[test]
fn a_drop_of_a_held_launch() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![
        Want::Offer(offer("a", "a")),
        after("b", "b", &["a"]),
        act("b/drop", "b", ActSort::Drop),
    ]);
    let (_, lines) = bench.pass(&mut plane);
    assert_eq!(acted(&bench, "b/drop"), (Outcome::Done, None));
    assert_eq!(
        answer(&bench, "b"),
        (
            Outcome::Done,
            Some("dropped by b/drop before it was launched".into())
        )
    );
    assert!(lines.ends_with(&[
        "b/drop: drop of b: done".to_string(),
        "b: done: dropped by b/drop before it was launched".to_string(),
    ]));
    bench.flume.land(&place_of("a"));
    bench.pass(&mut plane);
    let names: Vec<_> = bench.flume.launches().into_iter().map(|a| a.name).collect();
    assert_eq!(names, ["a"]);
}

/// The work-into-flume spec's "A retry of a held launch", and a done of
/// one.
#[test]
fn a_retry_or_a_done_of_a_held_launch() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![
        Want::Offer(offer("a", "a")),
        after("b", "b", &["a"]),
        act("b/retry", "b", retry()),
        act("b/done", "b", ActSort::Done),
    ]);
    bench.pass(&mut plane);
    let words = format!(
        "b is held, not launched: waits on a (launched at {}, not landed)",
        place_of("a")
    );
    assert_eq!(
        acted(&bench, "b/retry"),
        (Outcome::Refused, Some(words.clone()))
    );
    assert_eq!(acted(&bench, "b/done"), (Outcome::Refused, Some(words)));
    assert_eq!(bench.flume.commands().len(), 1, "only a's launch");
}

/// A retry of a launch dropped before it was launched is refused, naming
/// the drop.
#[test]
fn a_retry_of_a_dropped_launch() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![
        after("b", "b", &["a"]),
        act("b/drop", "b", ActSort::Drop),
        act("b/retry", "b", retry()),
    ]);
    bench.pass(&mut plane);
    assert_eq!(
        acted(&bench, "b/retry"),
        (
            Outcome::Refused,
            Some("b is done, not launched: dropped by b/drop before it was launched".into())
        )
    );
}

/// The work-into-flume spec's "An act on another's launch".
#[test]
fn an_act_on_anothers_launch() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![act("madswan/100", "madswan/99", ActSort::Drop)]);
    let (_, lines) = bench.pass(&mut plane);
    assert_eq!(
        acted(&bench, "madswan/100"),
        (
            Outcome::Refused,
            Some("madswan/99 is no launch of this runner's".into())
        )
    );
    assert_eq!(bench.flume.calls(), [Call::Status]);
    assert_eq!(
        lines,
        ["madswan/100: drop of madswan/99: refused: madswan/99 is no launch of this runner's"]
    );
}

/// An act on a launch the pull offers and the runner has not taken waits,
/// unanswered, as its launch does.
#[test]
fn an_act_on_a_launch_not_yet_taken_waits() {
    let bench = Bench::new();
    bench
        .flume
        .host(json!({ "running": 1, "bound": 1, "room": 0, "queue": [], "streams": [] }));
    let mut plane = Fake::holding(vec![
        Want::Offer(offer("a", "a")),
        act("a/priority", "a", ActSort::Priority(3)),
    ]);
    let (_, lines) = bench.pass(&mut plane);
    assert_eq!(lines, [] as [String; 0]);
    assert!(flywheel::report::read(&bench.report_file())
        .unwrap()
        .is_none());
}
