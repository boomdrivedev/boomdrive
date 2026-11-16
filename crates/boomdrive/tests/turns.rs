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
use boomdrive_control::{ActSort, Id, Offer, Outcome, Want};
use fakes::{act, offer, place_of, Bench, Call};
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

/// The work-into-flume spec's "A drop of a launch held for its turn": the
/// pass is due from its start, and its pull offers `a` and `b`, then gives
/// a drop of `b`. `a` is launched, the drop is done, and `b` is answered
/// done with words naming the drop, no `flume launch` run for it.
#[test]
fn a_drop_of_a_launch_held_for_its_turn() {
    let bench = Bench::new();
    let mut wants = offers(&["a", "b"]);
    wants.push(act("b/drop", "b", ActSort::Drop));
    let mut plane = Fake::holding(wants);
    let (_, lines) = bench.pass_due(&bench.flume, &mut plane, &|| true);
    assert_eq!(made(&bench), ["launch a"]);
    let report = bench.report();
    assert_eq!(report.acts[&Id::new("b/drop")].outcome, Outcome::Done);
    assert_eq!(
        answer(&bench, "b"),
        Some((
            Outcome::Done,
            Some("dropped by b/drop before it was launched".to_string())
        ))
    );
    assert_eq!(
        lines,
        [
            format!("a: launched at {}", place_of("a")),
            held_line("b"),
            "b/drop: drop of b: done".to_string(),
            "b: done: dropped by b/drop before it was launched".to_string(),
        ]
    );
}

/// A bench whose runner holds a launch of each of `names` at a place Flume
/// stands, made by a pass whose next was never due, with a control plane
/// that offered them, and Flume's calls forgotten.
fn holding(names: &[&str]) -> (Bench, Fake) {
    let bench = Bench::new();
    let mut plane = Fake::holding(offers(names));
    bench.pass(&mut plane);
    for name in names {
        assert_eq!(
            answer(&bench, name).map(|(outcome, _)| outcome),
            Some(Outcome::Launched)
        );
    }
    bench.flume.take_calls();
    (bench, plane)
}

/// The work-into-flume spec's "An act waits behind one launch once the
/// pass is due": the runner holds `x`, and the pass, due from its start,
/// is given a land of `x`, then offers `a` and `b`. `a` is launched, then
/// the land is carried out on `x`'s place, and `b` is held for its turn.
/// At the next pass, due from its start, given a priority of `x`, then
/// offers `a`, `b` and `c`, `b` is launched, then the priority is carried
/// out, and `c` stays held.
#[test]
fn an_act_waits_behind_one_launch_once_the_pass_is_due() {
    let (bench, mut plane) = holding(&["x"]);
    let px = place_of("x");
    let mut wants = vec![act("x/land", "x", ActSort::Land)];
    wants.extend(offers(&["a", "b"]));
    plane.set_wants(wants);
    bench.pass_due(&bench.flume, &mut plane, &|| true);
    assert_eq!(
        made(&bench),
        [
            "launch a".to_string(),
            format!("retry submission=true at {px}")
        ]
    );
    assert_eq!(
        bench.report().acts[&Id::new("x/land")].outcome,
        Outcome::Done
    );
    assert_eq!(answer(&bench, "b"), held());

    bench.flume.take_calls();
    let mut wants = vec![act("x/priority", "x", ActSort::Priority(5))];
    wants.extend(offers(&["a", "b", "c"]));
    plane.set_wants(wants);
    bench.pass_due(&bench.flume, &mut plane, &|| true);
    assert_eq!(
        made(&bench),
        ["launch b".to_string(), format!("Priority(5) at {px}")]
    );
    assert_eq!(
        answer(&bench, "b").map(|(outcome, _)| outcome),
        Some(Outcome::Launched)
    );
    assert_eq!(answer(&bench, "c"), held());
}

/// The work-into-flume spec's "A pass that falls due among its launches
/// runs its first act next": the runner holds `x` and `y`, the pull gives
/// a priority of each, then offers `a`, `b`, `c` and `d`, and the next pass
/// falls due once two launches are made. `a` and `b` are launched, `c` and
/// `d` held, then `x`'s priority is carried out; `y`'s is left unanswered,
/// and the pass says that 1 act on a place waits for the next.
#[test]
fn a_pass_that_falls_due_among_its_launches_runs_its_first_act_next() {
    let (bench, mut plane) = holding(&["x", "y"]);
    let px = place_of("x");
    let mut wants = vec![
        act("x/priority", "x", ActSort::Priority(3)),
        act("y/priority", "y", ActSort::Priority(4)),
    ];
    wants.extend(offers(&["a", "b", "c", "d"]));
    plane.set_wants(wants);
    let due = || bench.flume.launches().len() >= 2;
    let (_, lines) = bench.pass_due(&bench.flume, &mut plane, &due);
    assert_eq!(
        made(&bench),
        [
            "launch a".to_string(),
            "launch b".to_string(),
            format!("Priority(3) at {px}"),
        ]
    );
    assert_eq!(answer(&bench, "c"), held());
    assert_eq!(answer(&bench, "d"), held());
    assert!(!bench.report().acts.contains_key(&Id::new("y/priority")));
    assert_eq!(
        lines,
        [
            format!("a: launched at {}", place_of("a")),
            format!("b: launched at {}", place_of("b")),
            held_line("c"),
            held_line("d"),
            format!("x/priority: priority 3 of x at {px}: done"),
            "boomdrive: home: the next control pass is due, so 1 act on a place waits for it"
                .to_string(),
        ]
    );
}
