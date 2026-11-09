//! A bolt is a launch, and it is landed on a land: a launch of a kind
//! that lands with no stages, as a bolt's base place is, is taken as any
//! launch, counted against its stream's room within its pass and launched
//! as one `flume launch`, and its units after it on its branch; its send
//! is its control plane's land, carried out as any land, by Flume's own
//! retry naming the submission, whose answer is Flume's. The runner reads
//! no kind and asks no tool for a branch (core.9, core.15, core.52,
//! core.53; the work-into-flume spec's "A bolt is a launch" and "A bolt
//! is landed on a land"). It runs over the fake Flume and the stub
//! `flume` and `sluice`, with the neutral kind `bolt`.

mod fakes;

use boomdrive::flume::Cli;
use boomdrive_control::fake::Fake;
use boomdrive_control::{ActSort, Id, Offer, Outcome, Want};
use fakes::{act, offer, printed, Bench, Stub};
use serde_json::{json, Value};
use std::fs;

/// The repository the bolt is in.
const CREW: &str = "example/crew";

/// The bolt's launch.
const BOLT: &str = "home/bolt/3";

/// Its unit's launch.
const UNIT: &str = "home/14";

/// The place the bolt stands at, `P`.
const P: &str = "/places/bolt--x";

/// A launch `id` of kind `kind` named `name` in [`CREW`] on `base`, in
/// the stream `home`.
fn launch(id: &str, kind: &str, name: &str, base: &str) -> Offer {
    let mut offer = offer(id, name);
    offer.launch.repository = CREW.to_string();
    offer.launch.kind = kind.to_string();
    offer.launch.base = Some(base.to_string());
    offer.launch.stream = Some("home".to_string());
    offer
}

/// The bolt `home/bolt/3`, of kind `bolt` named `x` on `main`.
fn bolt() -> Want {
    Want::Offer(launch(BOLT, "bolt", "x", "main"))
}

/// Its unit `home/14`, of kind `change` named `a` on `bolt/x`.
fn unit() -> Want {
    Want::Offer(launch(UNIT, "change", "a", "bolt/x"))
}

/// The bolt's send: a land of its launch (core.53).
fn send() -> Want {
    act(&format!("{BOLT}/send"), BOLT, ActSort::Land)
}

/// A host part with no bound of its own and the stream `home`, whose
/// room is `room` and whose queue is empty.
fn host(room: i64) -> Value {
    json!({
        "running": 0, "bound": null, "room": null, "queue": [],
        "streams": [
            { "name": "home", "priority": 0, "running": 0, "bound": room, "room": room, "queue": [] }
        ],
        "pools": [], "addresses": [], "bases": [], "terminal_kept": false
    })
}

/// A bench keeping [`CREW`] with its clone's container made.
fn keeping_crew() -> Bench {
    let bench = Bench::new();
    let container = bench.container(CREW);
    fs::create_dir_all(&container).expect("the clone's container");
    bench.clones.ready(CREW, &container);
    bench
}

/// A stub `flume` whose listing holds `places` on [`host`] with a room
/// of `room`.
fn stub(places: Value, room: i64) -> Stub {
    let stub = Stub::new();
    let listing = json!({ "status": { "places": places, "host": host(room) } });
    stub.answers_text("status", &printed(&listing), 0);
    stub
}

/// `words` as a call's arguments.
fn argv(words: &[&str]) -> Vec<String> {
    words.iter().map(|word| word.to_string()).collect()
}

/// The answer the report file holds for launch `id`, where it holds one.
fn outcome(bench: &Bench, id: &str) -> Option<Outcome> {
    let report = bench.report();
    let launched = report.launches.get(&Id::new(id))?;
    Some(launched.answer.outcome)
}

/// The work-into-flume spec's "A bolt and its unit".
#[test]
fn a_bolt_and_its_unit() {
    let bench = keeping_crew();
    let stub = stub(json!([]), 2);
    stub.answers("launch", "launch.json", 0);
    let mut plane = Fake::holding(vec![bolt(), unit()]);
    let (passed, lines) = bench.pass_over(&Cli::new(stub.program()), &mut plane);
    assert!(passed.reported, "{lines:?}");
    let container = bench.container(CREW).display().to_string();
    let launches: Vec<Vec<String>> = stub
        .calls()
        .into_iter()
        .filter(|call| call[1] == "launch")
        .collect();
    assert_eq!(
        launches,
        [
            argv(&[
                "--json",
                "launch",
                "--repository",
                &container,
                "--kind",
                "bolt",
                "--name",
                "x",
                "--base",
                "main",
                "--stream",
                "home",
                "--mark",
                "boomdrive:home/bolt/3",
            ]),
            argv(&[
                "--json",
                "launch",
                "--repository",
                &container,
                "--kind",
                "change",
                "--name",
                "a",
                "--base",
                "bolt/x",
                "--stream",
                "home",
                "--mark",
                "boomdrive:home/14",
            ]),
        ]
    );
    assert_eq!(outcome(&bench, BOLT), Some(Outcome::Launched));
    assert_eq!(outcome(&bench, UNIT), Some(Outcome::Launched));
    assert!(
        bench.sluice.calls().is_empty(),
        "{:?}",
        bench.sluice.calls()
    );
}

/// The work-into-flume spec's "A bolt counts against its stream's room
/// within its pass": the next pass's listing gives `home` a room of 1
/// again, as Flume gives it with the base place taking none.
#[test]
fn a_bolt_counts_against_its_streams_room_within_its_pass() {
    let bench = keeping_crew();
    bench.flume.host(host(1));
    let mut plane = Fake::holding(vec![bolt(), unit()]);
    let (passed, lines) = bench.pass(&mut plane);
    assert!(passed.reported, "{lines:?}");
    let launched: Vec<String> = bench
        .flume
        .launches()
        .into_iter()
        .map(|args| args.name)
        .collect();
    assert_eq!(launched, ["x"]);
    assert_eq!(outcome(&bench, BOLT), Some(Outcome::Launched));
    assert_eq!(outcome(&bench, UNIT), None, "the unit is left unanswered");

    bench.flume.host(host(1));
    bench.pass(&mut plane);
    let launches = bench.flume.launches();
    assert_eq!(launches.len(), 2, "{launches:?}");
    assert_eq!(launches[1].name, "a");
    assert_eq!(launches[1].base.as_deref(), Some("bolt/x"));
    assert_eq!(launches[1].stream.as_deref(), Some("home"));
    assert_eq!(outcome(&bench, UNIT), Some(Outcome::Launched));
    assert!(
        bench.sluice.calls().is_empty(),
        "{:?}",
        bench.sluice.calls()
    );
}

/// The bolt's place `P`, live, as Flume lists it, marked for its launch.
fn bolt_at_p() -> Value {
    json!([{
        "place": P,
        "kind": "bolt",
        "name": "x",
        "branch": "bolt/x",
        "base": "main",
        "mark": "boomdrive:home/bolt/3",
        "lands": true,
        "phase": "live",
        "stages": [],
        "stream": { "name": "home", "from": "launch" }
    }])
}

/// A pass over `stub` holding the bolt at `P` and its send, and what the
/// report file holds of the send, with the lines the pass said.
fn sending(stub: &Stub) -> (Bench, Outcome, Option<String>, Vec<String>) {
    let bench = keeping_crew();
    let mut plane = Fake::holding(vec![bolt(), send()]);
    let (_, lines) = bench.pass_over(&Cli::new(stub.program()), &mut plane);
    let acted = bench.report().acts[&Id::new(format!("{BOLT}/send"))].clone();
    (bench, acted.outcome, acted.words, lines)
}

/// The work-into-flume spec's "A bolt's send".
#[test]
fn a_bolts_send() {
    let stub = stub(bolt_at_p(), 2);
    stub.answers("retry", "retry.json", 0);
    let (bench, outcome, words, lines) = sending(&stub);
    let retries: Vec<Vec<String>> = stub
        .calls()
        .into_iter()
        .filter(|call| call[1] == "retry")
        .collect();
    assert_eq!(retries, [argv(&["--json", "retry", P, "--submission"])]);
    assert_eq!((outcome, words), (Outcome::Done, None));
    assert!(
        lines.contains(&format!("{BOLT}/send: land of {BOLT} at {P}: done")),
        "{lines:?}"
    );
    assert!(stub.call_of("launch").is_none(), "found by its mark");
    assert!(
        bench.sluice.calls().is_empty(),
        "{:?}",
        bench.sluice.calls()
    );
}

/// The work-into-flume spec's "A send Flume refuses".
#[test]
fn a_send_flume_refuses() {
    let stub = stub(bolt_at_p(), 2);
    let reason = "the places on bolt/x are live: Q";
    let refused = json!({ "refused": { "refusal": { "reason": reason } } });
    stub.answers_text("retry", &printed(&refused), 1);
    let (bench, outcome, words, lines) = sending(&stub);
    assert_eq!(
        (outcome, words.as_deref()),
        (Outcome::Refused, Some(reason))
    );
    assert!(
        lines.contains(&format!(
            "{BOLT}/send: land of {BOLT} at {P}: refused by Flume: {reason}"
        )),
        "{lines:?}"
    );
    assert!(
        bench.sluice.calls().is_empty(),
        "{:?}",
        bench.sluice.calls()
    );
}
