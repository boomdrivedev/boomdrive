//! A retry, a drop, a priority and a done are Flume's own commands on the
//! launch's place, never forced, each answered as Flume answered it
//! (design Decision 9; the work-into-flume spec's "A retry, a drop, a
//! priority and a done are Flume's own commands on the launch's place";
//! Task notes 4.5). They run over a stub `flume` whose listing is Flume
//! 0.4.1's own, so each launch is found by the mark the scratch host's
//! places carry.

mod fakes;

use boomdrive::facts;
use boomdrive::flume::{Cli, Reply, Unrun};
use boomdrive_control::fake::Fake;
use boomdrive_control::{ActSort, Id, Outcome, Want};
use fakes::{act, offer, place_of, Bench, Call, Stub};
use serde_json::Value;

/// The location of the place `status.json` marks `flywheel:<id>`, as a
/// runner marked it before the rename, when the listing was read (record
/// 0016).
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
    let (_, outcome, words, lines) = acting(&stub, "scratch/4", "home/20", retry);
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
    assert!(lines.contains(&format!("home/20: retry of scratch/4 at {p}: done")));
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
    acting(&stub, "scratch/4", "home/21", retry);
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
    let (_, outcome, _, _) = acting(&stub, "scratch/7", "home/22", ActSort::Drop);
    let p = place("scratch/7");
    assert_eq!(calls_of(&stub, "drop"), [argv(&["--json", "drop", &p])]);
    assert_eq!(outcome, Outcome::Done);
}

/// A priority, on the queued place Flume 0.4.1 moved.
#[test]
fn a_priority() {
    let stub = Stub::new();
    stub.answers("priority", "priority.json", 0);
    let (_, outcome, _, _) = acting(&stub, "scratch/7", "home/23", ActSort::Priority(7));
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
    let (_, outcome, _, _) = acting(&stub, "scratch/planner/1", "home/24", ActSort::Done);
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
    let (_, outcome, words, lines) = acting(&stub, "scratch/2", "home/25", retry);
    let reason =
        "the agent flume:2867c6a5947d:fix--lexer:code:1 is not quiet; ask again forced to end it";
    assert_eq!(
        (outcome, words.as_deref()),
        (Outcome::Refused, Some(reason))
    );
    let p = place("scratch/2");
    assert!(lines.contains(&format!(
        "home/25: retry of scratch/2 at {p}: refused by Flume: {reason}"
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

/// The calls that launch or act, each as `launch <name>` or `<act> at
/// <place>`, forgotten once given.
fn made(bench: &Bench) -> Vec<String> {
    bench
        .flume
        .take_calls()
        .into_iter()
        .filter_map(|call| match call {
            Call::Launch(args) => Some(format!("launch {}", args.name)),
            Call::Act(place, act) => Some(format!("{act:?} at {place}")),
            _ => None,
        })
        .collect()
}

/// Each of `calls` that read the status of the place at `of`, as
/// `status <place>`, with the container where one was named.
fn statuses_read(calls: &[Call], of: &str) -> Vec<String> {
    calls
        .iter()
        .filter_map(|call| match call {
            Call::StatusOf(None, place) if place == of => Some(format!("status {place}")),
            Call::StatusOf(Some(container), place) if place == of => {
                Some(format!("status {place} in {}", container.display()))
            }
            _ => None,
        })
        .collect()
}

/// The work-into-flume spec's "A launch ahead of acts on places that are
/// over": acts on a place Flume no longer stands, ahead of an offer in
/// the control plane's order, are answered after the launch, each with
/// Flume's refusal of the place's status, read once and never again, and
/// no act's command is run.
#[test]
fn a_launch_ahead_of_acts_on_places_that_are_over() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![Want::Offer(offer("a", "a"))]);
    bench.pass(&mut plane);
    let p = place_of("a");
    bench.flume.tidy(&p);
    let reason = format!("no place {p} stands on this host");
    let retry = ActSort::Retry {
        stage: None,
        words: None,
        submission: false,
    };
    plane.set_wants(vec![
        act("a/priority", "a", ActSort::Priority(3)),
        act("a/retry", "a", retry),
        Want::Offer(offer("a", "a")),
        Want::Offer(offer("b", "b")),
    ]);
    bench.flume.take_calls();
    let (passed, lines) = bench.pass(&mut plane);
    assert!(passed.reported);
    let calls = bench.flume.calls();
    let b_launched = calls
        .iter()
        .position(|call| matches!(call, Call::Launch(args) if args.name == "b"))
        .expect("b is launched");
    let read_for_acts = calls
        .iter()
        .position(|call| *call == Call::StatusOf(None, p.clone()))
        .expect("the place's status is read for the acts");
    assert!(b_launched < read_for_acts, "{calls:?}");
    let container = bench.container(fakes::KEPT);
    assert_eq!(
        statuses_read(&calls, &p),
        [
            format!("status {p}"),
            format!("status {p}"),
            format!("status {p} in {}", container.display()),
        ],
        "once for each act, then once for the status carried"
    );
    assert_eq!(made(&bench), ["launch b".to_string()], "no act is run");
    assert_eq!(
        lines,
        [
            format!("b: launched at {}", place_of("b")),
            format!("a/priority: priority 3 of a at {p}: refused by Flume: {reason}"),
            format!("a/retry: retry of a at {p}: refused by Flume: {reason}"),
        ]
    );
    let report = bench.report();
    for id in ["a/priority", "a/retry"] {
        let acted = &report.acts[&Id::new(id)];
        assert_eq!(
            (acted.outcome, acted.words.as_deref()),
            (Outcome::Refused, Some(reason.as_str()))
        );
    }
    let (_, lines) = bench.pass(&mut plane);
    let calls = bench.flume.take_calls();
    assert_eq!(
        statuses_read(&calls, &p),
        [format!("status {p} in {}", container.display())],
        "nothing read again for the acts"
    );
    assert_eq!(made(&bench), [] as [String; 0], "nothing run again");
    assert_eq!(lines, [] as [String; 0]);
}

/// An act answered with Flume's refusal of its place's status runs no
/// command, so the pass reads Flume's listing once, and carries the
/// statuses against it (the work-into-flume spec's "The statuses are
/// carried against Flume's listing as it stands once the wants are
/// answered"; record 0027; core.51).
#[test]
fn an_act_refused_by_its_places_status_reads_the_listing_once() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![Want::Offer(offer("a", "a"))]);
    bench.pass(&mut plane);
    let p = place_of("a");
    bench.flume.tidy(&p);
    plane.set_wants(vec![
        Want::Offer(offer("a", "a")),
        act("a/drop", "a", ActSort::Drop),
    ]);
    bench.flume.take_calls();
    bench.pass(&mut plane);
    let calls = bench.flume.calls();
    let listings = calls.iter().filter(|call| **call == Call::Status).count();
    assert_eq!(listings, 1, "{calls:?}");
    assert_eq!(made(&bench), [] as [String; 0], "no command is run");
    assert_eq!(
        bench.report().acts[&Id::new("a/drop")].outcome,
        Outcome::Refused
    );
}

/// The work-into-flume spec's "An act on a place the listing does not
/// hold, which Flume stands": where Flume gives the place's status, the
/// act is Flume's own command on it, and Flume's answer is the act's.
#[test]
fn an_act_on_a_place_the_listing_does_not_hold_which_flume_stands() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![Want::Offer(offer("a", "a"))]);
    bench.pass(&mut plane);
    let p = place_of("a");
    bench.flume.listing(Ok(Reply::Given(fakes::printed(
        &serde_json::json!({ "status": { "places": [], "host": fakes::unbounded() } }),
    ))));
    plane.set_wants(vec![
        Want::Offer(offer("a", "a")),
        act("a/priority", "a", ActSort::Priority(3)),
    ]);
    bench.flume.take_calls();
    let (_, lines) = bench.pass(&mut plane);
    assert_eq!(made(&bench), [format!("Priority(3) at {p}")]);
    assert!(lines.contains(&format!("a/priority: priority 3 of a at {p}: done")));
    assert_eq!(
        bench.report().acts[&Id::new("a/priority")].outcome,
        Outcome::Done
    );
}

/// The work-into-flume spec's "A place whose status Flume could not be
/// read for": the act is kept unanswered, named, and carried out at the
/// next pass.
#[test]
fn an_act_whose_places_status_flume_could_not_be_read_for() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![Want::Offer(offer("a", "a"))]);
    bench.pass(&mut plane);
    let p = place_of("a");
    bench.flume.tidy(&p);
    bench
        .flume
        .status_answers(&p, Err(Unrun("flume could not run: gone".into())));
    plane.set_wants(vec![
        Want::Offer(offer("a", "a")),
        act("a/drop", "a", ActSort::Drop),
    ]);
    bench.flume.take_calls();
    let (_, lines) = bench.pass(&mut plane);
    assert!(!bench.report().acts.contains_key(&Id::new("a/drop")));
    assert!(lines.contains(&format!(
        "a/drop: drop of a at {p}: not carried out: flume could not run: gone"
    )));
    assert_eq!(made(&bench), [] as [String; 0]);
    let reason = format!("no place {p} stands on this host");
    bench
        .flume
        .status_answers(&p, Ok(Reply::Refused(reason.clone())));
    bench.pass(&mut plane);
    let acted = &bench.report().acts[&Id::new("a/drop")];
    assert_eq!(
        (acted.outcome, acted.words.as_deref()),
        (Outcome::Refused, Some(reason.as_str()))
    );
}

/// The work-into-flume spec's "Acts on places wait for the next pass once
/// it is due": once the next control pass is due, a pass runs no act's
/// command but its first; the rest are answered at the passes after, each
/// of which takes its launches first.
#[test]
fn acts_on_places_wait_for_the_next_pass_once_it_is_due() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![
        Want::Offer(offer("a", "a")),
        Want::Offer(offer("b", "b")),
    ]);
    bench.pass(&mut plane);
    let (pa, pb) = (place_of("a"), place_of("b"));
    plane.set_wants(vec![
        act("a/priority", "a", ActSort::Priority(3)),
        act("b/priority", "b", ActSort::Priority(4)),
        Want::Offer(offer("a", "a")),
        Want::Offer(offer("b", "b")),
        Want::Offer(offer("c", "c")),
    ]);
    bench.flume.take_calls();
    let due = || !bench.flume.acts().is_empty();
    let (passed, lines) = bench.pass_due(&bench.flume, &mut plane, &due);
    assert!(passed.reported);
    assert_eq!(
        made(&bench),
        ["launch c".to_string(), format!("Priority(3) at {pa}")]
    );
    assert_eq!(
        lines,
        [
            format!("c: launched at {}", place_of("c")),
            format!("a/priority: priority 3 of a at {pa}: done"),
            "boomdrive: the next control pass is due, so 1 act on a place waits for it".to_string(),
        ]
    );
    assert!(!bench.report().acts.contains_key(&Id::new("b/priority")));
    plane.set_wants(vec![
        act("a/priority", "a", ActSort::Priority(3)),
        act("b/priority", "b", ActSort::Priority(4)),
        Want::Offer(offer("a", "a")),
        Want::Offer(offer("b", "b")),
        Want::Offer(offer("c", "c")),
        Want::Offer(offer("d", "d")),
    ]);
    let (_, lines) = bench.pass_due(&bench.flume, &mut plane, &due);
    assert_eq!(
        made(&bench),
        ["launch d".to_string(), format!("Priority(4) at {pb}")],
        "the next pass's launch goes first, then the act left"
    );
    assert!(lines.contains(&format!("b/priority: priority 4 of b at {pb}: done")));
}

/// An act ahead of its own launch in the control plane's order is
/// carried out on the place that launch makes in the same pass.
#[test]
fn an_act_on_a_launch_the_same_pass_takes() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![
        act("a/priority", "a", ActSort::Priority(3)),
        Want::Offer(offer("a", "a")),
    ]);
    let (_, lines) = bench.pass(&mut plane);
    let p = place_of("a");
    assert_eq!(
        made(&bench),
        ["launch a".to_string(), format!("Priority(3) at {p}")]
    );
    assert_eq!(
        lines,
        [
            format!("a: launched at {p}"),
            format!("a/priority: priority 3 of a at {p}: done"),
        ]
    );
}

/// An act whose launch the runner holds unlaunched is still answered in
/// the control plane's order, ahead of the launches after it: a drop of
/// a held launch whose After lands in the same pass keeps it unmade.
#[test]
fn a_drop_ahead_of_its_held_launch_keeps_it_unmade() {
    let bench = Bench::new();
    let mut b = offer("b", "b");
    b.launch.after = vec![Id::new("a")];
    let mut plane = Fake::holding(vec![Want::Offer(offer("a", "a")), Want::Offer(b.clone())]);
    bench.pass(&mut plane);
    bench.flume.land(&place_of("a"));
    plane.set_wants(vec![
        Want::Offer(offer("a", "a")),
        act("b/drop", "b", ActSort::Drop),
        Want::Offer(b),
    ]);
    bench.flume.take_calls();
    bench.pass(&mut plane);
    assert_eq!(made(&bench), [] as [String; 0]);
    let answer = &bench.report().launches[&Id::new("b")].answer;
    assert_eq!(
        (answer.outcome, answer.words.as_deref()),
        (
            Outcome::Done,
            Some("dropped by b/drop before it was launched")
        )
    );
}
