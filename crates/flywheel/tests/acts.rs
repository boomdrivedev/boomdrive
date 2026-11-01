//! A retry, a drop, a priority and a done are Flume's own commands on the
//! launch's place, never forced, each answered as Flume answered it
//! (design Decision 9; the work-into-flume spec's "A retry, a drop, a
//! priority and a done are Flume's own commands on the launch's place";
//! Task notes 4.5). They run over a stub `flume` whose listing is Flume
//! 0.4.1's own, so each launch is found by the mark the scratch host's
//! places carry.

mod fakes;

use fakes::{act, offer, Bench, Stub};
use flywheel::facts;
use flywheel::flume::{Cli, Reply, Unrun};
use flywheel_control::fake::Fake;
use flywheel_control::{ActSort, Id, Outcome, Want};
use serde_json::Value;

/// The location of the place `status.json` marks `flywheel:<id>`.
fn place(id: &str) -> String {
    let listing: Value = serde_json::from_str(&fakes::fixture("status.json")).unwrap();
    facts::marked(&listing, &format!("flywheel:{id}"))
        .and_then(facts::location)
        .expect("the place")
        .to_string()
}

/// Each call the stub was run for whose command is `what`.
fn calls_of(stub: &Stub, what: &str) -> Vec<Vec<String>> {
    stub.calls()
        .into_iter()
        .filter(|call| call[1] == what)
        .collect()
}

/// `words` as a call's arguments.
fn argv(words: &[&str]) -> Vec<String> {
    words.iter().map(|word| word.to_string()).collect()
}

/// A pass over `stub` acting `what` as `act_id` on the launch `launch`,
/// found by its mark, and what the report file holds of the act.
fn acting(
    stub: &Stub,
    launch: &str,
    act_id: &str,
    what: ActSort,
) -> (Bench, Outcome, Option<String>, Vec<String>) {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![
        Want::Offer(offer(launch, "it")),
        act(act_id, launch, what),
    ]);
    let (_, lines) = bench.pass_over(&Cli::new(stub.program()), &mut plane);
    let acted = bench.report().acts[&Id::new(act_id)].clone();
    (bench, acted.outcome, acted.words, lines)
}

/// The work-into-flume spec's "A retry with words".
#[test]
fn a_retry_with_words() {
    let stub = Stub::new();
    stub.answers("retry", "retry.json", 0);
    let retry = ActSort::Retry {
        stage: Some("code".into()),
        words: Some("use the second option".into()),
        submission: false,
    };
    let (_, outcome, words, lines) = acting(&stub, "scratch/4", "madswan/20", retry);
    let p = place("scratch/4");
    assert_eq!(
        calls_of(&stub, "retry"),
        [argv(&[
            "--json",
            "retry",
            &p,
            "--stage",
            "code",
            "--words",
            "use the second option"
        ])]
    );
    assert_eq!((outcome, words), (Outcome::Done, None));
    assert!(lines.contains(&format!("madswan/20: retry of scratch/4 at {p}: done")));
}

/// A retry asking for the submission, with no stage or words.
#[test]
fn a_retry_of_the_submission() {
    let stub = Stub::new();
    stub.answers("retry", "retry.json", 0);
    let retry = ActSort::Retry {
        stage: None,
        words: None,
        submission: true,
    };
    acting(&stub, "scratch/4", "madswan/21", retry);
    let p = place("scratch/4");
    assert_eq!(
        calls_of(&stub, "retry"),
        [argv(&["--json", "retry", &p, "--submission"])]
    );
}

/// A drop, on the queued place Flume 0.4.1 dropped.
#[test]
fn a_drop() {
    let stub = Stub::new();
    stub.answers("drop", "drop.json", 0);
    let (_, outcome, _, _) = acting(&stub, "scratch/7", "madswan/22", ActSort::Drop);
    let p = place("scratch/7");
    assert_eq!(calls_of(&stub, "drop"), [argv(&["--json", "drop", &p])]);
    assert_eq!(outcome, Outcome::Done);
}

/// A priority, on the queued place Flume 0.4.1 moved.
#[test]
fn a_priority() {
    let stub = Stub::new();
    stub.answers("priority", "priority.json", 0);
    let (_, outcome, _, _) = acting(&stub, "scratch/7", "madswan/23", ActSort::Priority(7));
    let p = place("scratch/7");
    assert_eq!(
        calls_of(&stub, "priority"),
        [argv(&["--json", "priority", &p, "7"])]
    );
    assert_eq!(outcome, Outcome::Done);
}

/// The work-into-flume spec's "A done", on the delivered role's run
/// Flume 0.4.1 tidied.
#[test]
fn a_done() {
    let stub = Stub::new();
    stub.answers("done", "done.json", 0);
    let (_, outcome, _, _) = acting(&stub, "scratch/planner/1", "madswan/24", ActSort::Done);
    let p = place("scratch/planner/1");
    assert_eq!(calls_of(&stub, "done"), [argv(&["--json", "done", &p])]);
    assert_eq!(outcome, Outcome::Done);
}

/// A refusal carried with Flume's words: Flume 0.4.1 refuses a retry of
/// a place whose agent is not quiet, and the runner never forces it.
#[test]
fn a_refusal_carried_with_flumes_words() {
    let stub = Stub::new();
    stub.answers("retry", "retry-refused.json", 1);
    let retry = ActSort::Retry {
        stage: None,
        words: None,
        submission: false,
    };
    let (_, outcome, words, lines) = acting(&stub, "scratch/2", "madswan/25", retry);
    let reason =
        "the agent flume:2867c6a5947d:fix--lexer:code:1 is not quiet; ask again forced to end it";
    assert_eq!(
        (outcome, words.as_deref()),
        (Outcome::Refused, Some(reason))
    );
    let p = place("scratch/2");
    assert!(lines.contains(&format!(
        "madswan/25: retry of scratch/2 at {p}: refused by Flume: {reason}"
    )));
    for call in stub.calls() {
        assert!(!call.iter().any(|arg| arg == "--force"), "{call:?}");
    }
}

/// An act Flume could not be run for is kept unanswered, named, and
/// carried out at the next pass.
#[test]
fn an_act_flume_could_not_run() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![
        Want::Offer(offer("a", "a")),
        act("a/drop", "a", ActSort::Drop),
    ]);
    let p = fakes::place_of("a");
    bench
        .flume
        .act_answers(&p, Err(Unrun("flume could not run: gone".into())));
    let (passed, lines) = bench.pass(&mut plane);
    assert!(passed.reported);
    assert!(!bench.report().acts.contains_key(&Id::new("a/drop")));
    assert!(lines.contains(&format!(
        "a/drop: drop of a at {p}: not carried out: flume could not run: gone"
    )));
    bench
        .flume
        .act_answers(&p, Ok(Reply::Given("{\"acted\":{}}\n".into())));
    bench.pass(&mut plane);
    assert_eq!(
        bench.report().acts[&Id::new("a/drop")].outcome,
        Outcome::Done
    );
}
