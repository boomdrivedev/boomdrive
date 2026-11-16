//! Once the next control pass is due, a pass's launches and its acts on
//! places take turns (record 0002, as amended on 2026-11-14 on taking
//! turns; core.50; the work-into-flume spec's "Once a pass is due, it
//! makes no launch after its first", "A launch the pass took and left is
//! held for its turn" and "Once a pass is due, its launches and its acts
//! take turns"): it runs no `flume launch` after the first it ran, holding
//! each launch it would make after it for the next pass, `held` with the
//! words `waits for the next control pass, which is due`, its room counted
//! at its turn, and no act's command on a place after the first it ran.

mod fakes;

use boomdrive::carry::TURN;
use boomdrive::flume::Act;
use boomdrive_control::fake::Fake;
use boomdrive_control::{Id, Offer, Outcome, Want};
use fakes::{offer, place_of, Bench, Call};
use serde_json::json;

/// The offers `names`, each `id` and `name` alike, in [`fakes::KEPT`].
fn offers(names: &[&str]) -> Vec<Want> {
    names
        .iter()
        .map(|name| Want::Offer(offer(name, name)))
        .collect()
}

/// The answer the report file holds for `launch`, with its words, where
/// it answers it.
fn answer(bench: &Bench, launch: &str) -> Option<(Outcome, Option<String>)> {
    let report = boomdrive::report::read(&bench.report_file()).unwrap()?;
    let answer = &report.launches.get(&Id::new(launch))?.answer;
    Some((answer.outcome, answer.words.clone()))
}

/// The answer `held` for its turn.
fn held() -> Option<(Outcome, Option<String>)> {
    Some((Outcome::Held, Some(TURN.to_string())))
}

/// Each launch's name and each act on a place Flume was asked for, in
/// order: `launch <name>`, or `<act> at <place>`.
fn made(bench: &Bench) -> Vec<String> {
    bench
        .flume
        .calls()
        .into_iter()
        .filter_map(|call| match call {
            Call::Launch(args) => Some(format!("launch {}", args.name)),
            Call::Act(place, Act::Retry { submission, .. }) => {
                Some(format!("retry submission={submission} at {place}"))
            }
            Call::Act(place, act) => Some(format!("{act:?} at {place}")),
            _ => None,
        })
        .collect()
}

/// The line a launch held for its turn is said in.
fn held_line(id: &str) -> String {
    format!("{id}: held: {TURN}")
}

/// The work-into-flume spec's "A pass due from its start makes one
/// launch": `a` is launched, `b` and `c` held for their turn, each said in
/// one line; at the next pass, `b` is launched, and `c` stays held, said
/// in no line.
#[test]
fn a_pass_due_from_its_start_makes_one_launch() {
    let bench = Bench::new();
    let mut plane = Fake::holding(offers(&["a", "b", "c"]));
    let (passed, lines) = bench.pass_due(&bench.flume, &mut plane, &|| true);
    assert!(passed.reported);
    assert_eq!(made(&bench), ["launch a"]);
    assert_eq!(
        lines,
        [
            format!("a: launched at {}", place_of("a")),
            held_line("b"),
            held_line("c"),
        ]
    );
    assert_eq!(
        answer(&bench, "a").map(|(outcome, _)| outcome),
        Some(Outcome::Launched)
    );
    assert_eq!(answer(&bench, "b"), held());
    assert_eq!(answer(&bench, "c"), held());

    bench.flume.take_calls();
    let (_, lines) = bench.pass_due(&bench.flume, &mut plane, &|| true);
    assert_eq!(made(&bench), ["launch b"]);
    assert_eq!(lines, [format!("b: launched at {}", place_of("b"))]);
    assert_eq!(
        answer(&bench, "b").map(|(outcome, _)| outcome),
        Some(Outcome::Launched)
    );
    assert_eq!(answer(&bench, "c"), held());
}

/// The work-into-flume spec's "A pass that falls due among its launches
/// makes none more": the next pass falls due once two launches are made,
/// so `c` and `d` are held for their turn and no `flume launch` is run for
/// either.
#[test]
fn a_pass_that_falls_due_among_its_launches_makes_none_more() {
    let bench = Bench::new();
    let mut plane = Fake::holding(offers(&["a", "b", "c", "d"]));
    let due = || bench.flume.launches().len() >= 2;
    let (_, lines) = bench.pass_due(&bench.flume, &mut plane, &due);
    assert_eq!(made(&bench), ["launch a", "launch b"]);
    assert_eq!(
        lines,
        [
            format!("a: launched at {}", place_of("a")),
            format!("b: launched at {}", place_of("b")),
            held_line("c"),
            held_line("d"),
        ]
    );
    assert_eq!(answer(&bench, "c"), held());
    assert_eq!(answer(&bench, "d"), held());
}

/// An offer `id` named `name` that runs in the stream `work`.
fn in_work(name: &str) -> Want {
    let mut offer: Offer = offer(name, name);
    offer.launch.stream = Some("work".to_string());
    Want::Offer(offer)
}

/// The work-into-flume spec's "A launch held for its turn counts against
/// its stream's room": the stream `work` has room for 2 and an empty
/// queue. `a` is launched, `b` held for its turn, its room counted, and
/// `c`, for which no room is left, left unanswered.
#[test]
fn a_launch_held_for_its_turn_counts_against_its_streams_room() {
    let bench = Bench::new();
    bench.flume.host(json!({
        "running": 0, "bound": null, "room": null, "queue": [],
        "streams": [
            { "name": "work", "priority": 0, "running": 0, "bound": 2, "room": 2, "queue": [] }
        ]
    }));
    let mut plane = Fake::holding(vec![in_work("a"), in_work("b"), in_work("c")]);
    let (_, lines) = bench.pass_due(&bench.flume, &mut plane, &|| true);
    assert_eq!(made(&bench), ["launch a"]);
    assert_eq!(
        answer(&bench, "a").map(|(outcome, _)| outcome),
        Some(Outcome::Launched)
    );
    assert_eq!(answer(&bench, "b"), held());
    assert_eq!(answer(&bench, "c"), None, "no room is left for c");
    assert_eq!(
        lines,
        [format!("a: launched at {}", place_of("a")), held_line("b")]
    );
}
