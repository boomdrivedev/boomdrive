//! An offer waits, held, for each launch it comes after, and an act on a
//! launch not yet launched is answered from the report with no command
//! run (design Decisions 7 and 9; the work-into-flume spec's "An offer
//! waits for each launch it comes after", "A priority or a drop of a
//! launch not yet launched is answered from its report" and "A retry, a
//! land, a confirm or a done of a launch not launched, and an act on
//! another's launch, are refused"; Task notes 4.3; core.53, core.56).

mod fakes;

use boomdrive::facts;
use boomdrive::flume::{LaunchArgs, Reply};
use boomdrive_control::fake::Fake;
use boomdrive_control::{ActSort, Id, Offer, Outcome, Want};
use fakes::{act, fixture, offer, place_of, printed, Bench, Call};
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

/// The work-into-flume spec's "A dependent of a landing held for the
/// runner": `a`'s place lands and is tidied between two passes, Flume
/// holding its record for the runner, and the next pass launches `b`, its
/// listing holding the record landed, carries the record as `a`'s status,
/// and confirms its landing once the control plane has taken it (design
/// Decision 3; record 0030 as amended on 2026-11-09).
#[test]
fn a_dependent_of_a_landing_held_for_the_runner() {
    let bench = Bench::new();
    let mut planned = offer("a", "a");
    planned.launch.recipients = vec!["scratch/planner".into()];
    let mut plane = Fake::holding(vec![Want::Offer(planned), after("b", "b", &["a"])]);
    bench.pass(&mut plane);
    assert_eq!(answer(&bench, "b").0, Outcome::Held);
    let place = place_of("a");
    let record = format!("{place}@1793559694");
    bench.flume.tidy_held(&place, 1793559694, &["boomdrive"]);
    bench.flume.take_calls();
    bench.pass(&mut plane);
    assert_eq!(answer(&bench, "b").0, Outcome::Launched);
    let names: Vec<_> = bench.flume.launches().into_iter().map(|a| a.name).collect();
    assert_eq!(names, ["b"]);
    let status = bench.report().launches[&Id::new("a")]
        .status
        .clone()
        .expect("a's status carried");
    let carried: Value = serde_json::from_str(&status.text).unwrap();
    let entry = &carried["status"]["places"][0];
    assert_eq!(entry["place"], record);
    assert_eq!(
        entry["phase"]["tidied"]["over"],
        json!({ "landed": { "commit": "c0ffee" } })
    );
    assert_eq!(
        bench.flume.confirms(),
        [vec!["a/t/submission/1/landed".to_string()]]
    );
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
    assert_eq!(bench.flume.launches().len(), 1, "only a's launch");
    assert_eq!(bench.flume.acts(), []);
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
    assert_eq!(bench.flume.launches().len(), 1, "only a's launch");
    assert_eq!(bench.flume.acts(), []);
}

/// The work-into-flume spec's "A confirm of a held launch": the confirm
/// is refused with the words a retry of it is given, and runs no `flume
/// confirm` (core.56).
#[test]
fn a_confirm_of_a_held_launch() {
    let bench = Bench::new();
    let confirm = ActSort::Confirm {
        recipient: "scratch/planner".to_string(),
        deliveries: vec!["change--a/0a1b2c3d/stage/code/1/delivered".to_string()],
    };
    let mut plane = Fake::holding(vec![
        Want::Offer(offer("a", "a")),
        after("b", "b", &["a"]),
        act("b/retry", "b", retry()),
        act("b/confirm", "b", confirm),
    ]);
    bench.pass(&mut plane);
    let retried = acted(&bench, "b/retry");
    let words = format!(
        "b is held, not launched: waits on a (launched at {}, not landed)",
        place_of("a")
    );
    assert_eq!(retried, (Outcome::Refused, Some(words)));
    assert_eq!(acted(&bench, "b/confirm"), retried);
    assert_eq!(bench.flume.acts(), []);
}

/// The work-into-flume spec's "A land of a held launch": the land is
/// refused with the words a retry of it is given, runs no `flume retry`,
/// and `b` is launched as its offer gives it, its submission still held,
/// once `a` reads landed (core.15, core.53).
#[test]
fn a_land_of_a_held_launch() {
    let bench = Bench::new();
    let Want::Offer(mut b) = after("b", "b", &["a"]) else {
        unreachable!()
    };
    b.launch.hold_submission = true;
    let mut plane = Fake::holding(vec![
        Want::Offer(offer("a", "a")),
        Want::Offer(b),
        act("b/retry", "b", retry()),
        act("b/land", "b", ActSort::Land),
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
    assert_eq!(acted(&bench, "b/land"), (Outcome::Refused, Some(words)));
    assert_eq!(bench.flume.launches().len(), 1, "only a's launch");
    assert_eq!(bench.flume.acts(), []);

    bench.flume.land(&place_of("a"));
    bench.pass(&mut plane);
    assert_eq!(answer(&bench, "b").0, Outcome::Launched);
    let launches = bench.flume.launches();
    assert_eq!(launches.len(), 2, "{launches:?}");
    assert_eq!(
        launches[1],
        LaunchArgs {
            container_dir: launches[0].container_dir.clone(),
            kind: "change".to_string(),
            name: "b".to_string(),
            hold_submission: true,
            mark: "boomdrive:b".to_string(),
            ..LaunchArgs::default()
        }
    );
    assert_eq!(bench.flume.acts(), [], "still no act run");
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
    let mut plane = Fake::holding(vec![act("home/100", "home/99", ActSort::Drop)]);
    let (_, lines) = bench.pass(&mut plane);
    assert_eq!(
        acted(&bench, "home/100"),
        (
            Outcome::Refused,
            Some("home/99 is no launch of this runner's".into())
        )
    );
    assert_eq!(bench.flume.calls(), [Call::Status]);
    assert_eq!(
        lines,
        ["home/100: drop of home/99: refused: home/99 is no launch of this runner's"]
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
    assert!(boomdrive::report::read(&bench.report_file())
        .unwrap()
        .is_none());
}
