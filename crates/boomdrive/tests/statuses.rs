//! Flume's status of each place the runner launched, carried as Flume
//! gave it and read again only when the place's entry in the pass's
//! listing changes; a landing told to the keeper once; no place but the
//! runner's own acted on or carried; and each act said in one line, a
//! quiet pass saying nothing (design Decisions 5 and 12; core.11; the
//! work-into-flume spec's "Flume's status of each place is carried as
//! Flume gave it", "A place the listing no longer holds keeps its last
//! status", "A landing is told to the keeper once", "The runner acts only
//! on its own places" and "The runner says each act, one line each"; Task
//! notes 4.7).

mod fakes;

use boomdrive::flume::{Cli, Unrun};
use boomdrive_control::fake::Fake;
use boomdrive_control::{At, Carried, Command, CommandSort, Id, Lease, Outcome, Want};
use fakes::{fixture, offer, place_of, printed, Bench, Call, Stub, KEPT};
use serde_json::{json, Value};
use std::fs;

/// The status the report file carries for `launch`.
fn carried(bench: &Bench, launch: &str) -> Carried {
    bench.report().launches[&Id::new(launch)]
        .status
        .clone()
        .expect("a status carried")
}

/// The number of `status <place>` calls the stub was run for.
fn statuses_of(stub: &Stub) -> usize {
    stub.calls()
        .iter()
        .filter(|call| call[1] == "status" && call.len() > 2)
        .count()
}

/// `status.json` with the entry of the place marked `flywheel:scratch/4`
/// stalled, as a later listing would show it.
fn stalled(answer: &str) -> String {
    let mut value: Value = serde_json::from_str(&fixture(answer)).unwrap();
    for entry in value["status"]["places"].as_array_mut().unwrap() {
        if entry["mark"] == "flywheel:scratch/4" {
            entry["stalled"] = json!(true);
        }
    }
    printed(&value)
}

/// The work-into-flume spec's "The status byte for byte", and a carried
/// place read again only when its entry changes, over Flume 0.4.1's own
/// answers: `status-answer.json` is `fix--tabs`'s status, whose entry
/// `status.json` holds as it stands.
#[test]
fn the_status_byte_for_byte() {
    let bench = Bench::new();
    let stub = Stub::new();
    stub.answers("status-of", "status-answer.json", 0);
    let flume = Cli::new(stub.program());
    let mut plane = Fake::holding(vec![Want::Offer(offer("scratch/4", "tabs"))]);
    let before = At::now();
    bench.pass_over(&flume, &mut plane);
    let status = carried(&bench, "scratch/4");
    let printed = fixture("status-answer.json");
    assert_eq!(status.text, printed.strip_suffix('\n').unwrap());
    assert_eq!(status.text.lines().count(), 1, "on one line");
    assert!(status.at >= before && status.listed);
    assert_eq!(statuses_of(&stub), 1);
    let call = stub
        .calls()
        .into_iter()
        .find(|call| call.len() > 2 && call[1] == "status");
    let container = bench.container(KEPT);
    let tabs = bench.report().launches[&Id::new("scratch/4")]
        .answer
        .place_dir
        .clone()
        .unwrap();
    assert_eq!(
        call.unwrap(),
        [
            "--json",
            "status",
            "--repository",
            container.to_str().unwrap(),
            &tabs
        ]
    );

    let (_, lines) = bench.pass_over(&flume, &mut plane);
    assert_eq!(statuses_of(&stub), 1, "the entry has not changed");
    assert_eq!(carried(&bench, "scratch/4"), status);
    assert_eq!(lines, [] as [String; 0]);

    stub.answers_text("status", &stalled("status.json"), 0);
    let changed = stalled("status-answer.json");
    stub.answers_text("status-of", &changed, 0);
    let before = At::now();
    bench.pass_over(&flume, &mut plane);
    assert_eq!(statuses_of(&stub), 2);
    let status = carried(&bench, "scratch/4");
    assert_eq!(status.text, changed.strip_suffix('\n').unwrap());
    assert!(status.at >= before);
}

/// The work-into-flume spec's "Only the host changed": `status.json` and
/// `status-bounded.json` differ only in Flume's host part.
#[test]
fn only_the_host_changed() {
    let bench = Bench::new();
    let stub = Stub::new();
    stub.answers("status-of", "status-answer.json", 0);
    let flume = Cli::new(stub.program());
    let mut plane = Fake::holding(vec![Want::Offer(offer("scratch/4", "tabs"))]);
    bench.pass_over(&flume, &mut plane);
    let held = fs::read(bench.report_file()).unwrap();
    stub.answers("status", "status-bounded.json", 0);
    let (_, lines) = bench.pass_over(&flume, &mut plane);
    assert_eq!(statuses_of(&stub), 1, "no status <place> is run");
    assert_eq!(fs::read(bench.report_file()).unwrap(), held);
    assert_eq!(lines, [] as [String; 0]);
}

/// The work-into-flume spec's "A place tidied": a carried place the
/// listing no longer holds is asked for once a pass, and where Flume
/// refuses, its last status stands, not listed.
#[test]
fn a_place_tidied() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![Want::Offer(offer("a", "a"))]);
    bench.pass(&mut plane);
    let launched = carried(&bench, "a");
    assert!(
        launched.listed,
        "the listing is read again after the launch"
    );
    bench.pass(&mut plane);
    let listed = carried(&bench, "a");
    assert_eq!((&listed.text, listed.at), (&launched.text, launched.at));
    assert!(listed.listed);
    bench.flume.tidy(&place_of("a"));
    bench.flume.take_calls();
    let (_, lines) = bench.pass(&mut plane);
    let tidied = carried(&bench, "a");
    assert_eq!((&tidied.text, tidied.at), (&launched.text, launched.at));
    assert!(!tidied.listed);
    assert_eq!(lines, [] as [String; 0]);
    let asked = |calls: Vec<Call>| {
        calls
            .into_iter()
            .filter(|call| matches!(call, Call::StatusOf(..)))
            .count()
    };
    assert_eq!(asked(bench.flume.take_calls()), 1);
    bench.pass(&mut plane);
    assert_eq!(asked(bench.flume.take_calls()), 1, "once a pass");
    assert_eq!(carried(&bench, "a"), tidied);
}

/// The work-into-flume spec's "A place tidied while the pass answers its
/// wants": a carried place that lands and is tidied while the pass
/// launches another is carried that pass as Flume's record of it,
/// `<location>@<at>`, tidied and landed, though the listing the pass read
/// before the launch holds it live (record 0027).
#[test]
fn a_place_tidied_while_the_pass_answers_its_wants() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![Want::Offer(offer("a", "a"))]);
    bench.pass(&mut plane);
    let place = place_of("a");
    let tidied = place.clone();
    bench
        .flume
        .meanwhile(move |flume| flume.tidy_pending(&tidied, 1793559694));
    plane.set_wants(vec![
        Want::Offer(offer("a", "a")),
        Want::Offer(offer("b", "b")),
    ]);
    bench.flume.take_calls();
    bench.pass(&mut plane);
    let listings = |calls: Vec<Call>| calls.into_iter().filter(|c| *c == Call::Status).count();
    assert_eq!(
        listings(bench.flume.take_calls()),
        2,
        "read again once b is launched"
    );
    let status = carried(&bench, "a");
    let answer: Value = serde_json::from_str(&status.text).unwrap();
    let entry = &answer["status"]["places"][0];
    assert_eq!(entry["place"], format!("{place}@1793559694"));
    assert_eq!(
        entry["phase"]["tidied"]["over"],
        json!({ "landed": { "commit": "c0ffee" } })
    );
    assert!(status.listed);
    let launched = carried(&bench, "b");
    assert!(launched.listed, "b's place is in the listing read after it");

    bench.pass(&mut plane);
    assert_eq!(
        listings(bench.flume.take_calls()),
        1,
        "a quiet pass reads it once"
    );
    assert_eq!(carried(&bench, "a"), status);
}

/// The work-into-flume spec's "A pass that answers a command reads the
/// listing once": a command runs no tool, so it makes Flume's listing be
/// read no second time (record 0027; design Decision 6; Task notes 1.4).
#[test]
fn a_pass_that_answers_a_command_reads_the_listing_once() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![Want::Offer(offer("a", "a"))]);
    bench.pass(&mut plane);
    assert!(
        bench.flume.entry(&place_of("a")).is_some(),
        "a's place stands"
    );
    let branch = Want::Command(Command {
        id: Id::new("branch/1"),
        lease: Lease {
            number: 1,
            expires: None,
        },
        what: CommandSort::Branch {
            repository: KEPT.to_string(),
            branch: "bolt/x".to_string(),
            from: "main".to_string(),
        },
    });
    plane.set_wants(vec![Want::Offer(offer("a", "a")), branch]);
    bench.flume.take_calls();
    bench.pass(&mut plane);
    let calls = bench.flume.take_calls();
    let listings = calls.iter().filter(|c| **c == Call::Status).count();
    assert_eq!(listings, 1, "{calls:?}");
    assert_eq!(calls.first(), Some(&Call::Status), "read before the pull");
    let commanded = &bench.report().commands[&Id::new("branch/1")];
    assert_eq!(
        (commanded.outcome, commanded.words.as_deref()),
        (
            Outcome::Refused,
            Some("this binary does not carry sluice's commands")
        )
    );
}

/// A place's status Flume could not be run for is named, and what was
/// carried stands.
#[test]
fn a_status_flume_could_not_run() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![Want::Offer(offer("a", "a"))]);
    bench.pass(&mut plane);
    let before = carried(&bench, "a");
    bench.flume.set(&place_of("a"), "stalled", json!(true));
    bench.flume.status_answers(
        &place_of("a"),
        Err(Unrun("flume was ended: signal: 9".into())),
    );
    let (passed, lines) = bench.pass(&mut plane);
    assert!(passed.reported);
    assert_eq!(
        lines,
        [format!(
            "a: Flume's status of {} could not be read: flume was ended: signal: 9",
            place_of("a")
        )]
    );
    assert_eq!(carried(&bench, "a").text, before.text);
}

/// The work-into-flume spec's "A landing told once".
#[test]
fn a_landing_told_once() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![Want::Offer(offer("a", "a"))]);
    bench.pass(&mut plane);
    assert_eq!(bench.landings(), [] as [String; 0]);
    bench.flume.land(&place_of("a"));
    bench.pass(&mut plane);
    assert_eq!(bench.landings(), [KEPT]);
    bench.pass(&mut plane);
    bench.flume.tidy(&place_of("a"));
    bench.pass(&mut plane);
    assert_eq!(bench.landings(), [] as [String; 0]);
}

/// The work-into-flume spec's "A place whose repository the runner keeps
/// no more": a launch's place is followed to its end after the
/// configuration drops its repository, asked for by its location alone,
/// its landing and its tidied record carried, and no keeper told (record
/// 0020).
#[test]
fn a_place_whose_repository_the_runner_keeps_no_more() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![Want::Offer(offer("a", "a"))]);
    bench.pass(&mut plane);
    bench.clones.drop_named(KEPT);
    bench.flume.take_calls();
    let place = place_of("a");
    bench.flume.land(&place);
    let (_, lines) = bench.pass(&mut plane);
    assert_eq!(lines, [] as [String; 0]);
    let asked: Vec<Call> = bench
        .flume
        .take_calls()
        .into_iter()
        .filter(|call| matches!(call, Call::StatusOf(..)))
        .collect();
    assert_eq!(asked, [Call::StatusOf(None, place.clone())]);
    let landed: Value = serde_json::from_str(&carried(&bench, "a").text).unwrap();
    assert_eq!(
        landed["status"]["places"][0]["phase"]["over"]["over"],
        json!({ "landed": { "commit": "c0ffee" } })
    );
    assert_eq!(bench.landings(), [] as [String; 0], "no keeper told");

    bench.flume.tidy_pending(&place, 1793559694);
    bench.pass(&mut plane);
    let tidied = carried(&bench, "a");
    let tidied: Value = serde_json::from_str(&tidied.text).unwrap();
    let entry = &tidied["status"]["places"][0];
    assert_eq!(entry["place"], format!("{place}@1793559694"));
    assert_eq!(
        entry["phase"]["tidied"]["over"],
        json!({ "landed": { "commit": "c0ffee" } })
    );
    assert!(carried(&bench, "a").listed);
    assert_eq!(
        bench.report().launches[&Id::new("a")].answer.place_dir,
        Some(place)
    );
}

/// The work-into-flume spec's "A place found tidied by its mark": an
/// offer in a repository the runner keeps no more, whose place Flume's
/// listing holds tidied with its landing under the runner's mark, is
/// answered launched at its location, and Flume's own answer for it is
/// carried, asked for by its location alone (record 0021).
#[test]
fn a_place_found_tidied_by_its_mark() {
    let bench = Bench::new();
    let stub = Stub::new();
    stub.answers("status-of", "status-tidied.json", 0);
    let flume = Cli::new(stub.program());
    let mut gate = offer("scratch/5", "gate");
    gate.launch.repository = "example/old-kit".to_string();
    let mut plane = Fake::holding(vec![Want::Offer(gate)]);
    let (_, lines) = bench.pass_over(&flume, &mut plane);
    let launched = &bench.report().launches[&Id::new("scratch/5")];
    let location = launched.answer.place_dir.clone().expect("a place");
    assert!(location.ends_with("/fix--gate"), "{location}");
    assert_eq!(
        lines,
        [format!(
            "scratch/5: launched at {location}, as its mark flywheel:scratch/5 shows"
        )]
    );
    let call = stub
        .calls()
        .into_iter()
        .find(|call| call.len() > 2 && call[1] == "status");
    assert_eq!(call.unwrap(), ["--json", "status", &location]);
    let status = carried(&bench, "scratch/5");
    let printed = fixture("status-tidied.json");
    assert_eq!(status.text, printed.strip_suffix('\n').unwrap());
    assert!(status.listed);
    assert_eq!(bench.landings(), [] as [String; 0], "no keeper told");
}

/// The work-into-flume spec's "The user's own place": a place with no
/// mark, and one marked for a launch the runner does not hold, are read
/// in the listing and left alone.
#[test]
fn the_users_own_place() {
    let bench = Bench::new();
    bench
        .flume
        .stand(json!({ "place": "/places/mine", "phase": "live", "mark": null }));
    bench
        .flume
        .stand(json!({ "place": "/places/theirs", "phase": "live", "mark": "boomdrive:vega/3" }));
    let mut plane = Fake::holding(vec![Want::Offer(offer("a", "a"))]);
    bench.pass(&mut plane);
    bench.flume.land("/places/mine");
    bench.pass(&mut plane);
    for call in bench.flume.calls() {
        let named = match &call {
            Call::Act(place, _) | Call::StatusOf(_, place) => Some(place.as_str()),
            Call::Launch(_) | Call::Status => None,
        };
        assert!(
            !matches!(named, Some("/places/mine" | "/places/theirs")),
            "{call:?}"
        );
    }
    let report = bench.report();
    assert_eq!(report.launches.keys().collect::<Vec<_>>(), [&Id::new("a")]);
    assert_eq!(bench.landings(), [] as [String; 0]);
}

/// The work-into-flume spec's "A launch said".
#[test]
fn a_launch_said() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![Want::Offer(offer("madswan/12", "a"))]);
    let (_, lines) = bench.pass(&mut plane);
    assert_eq!(lines.len(), 1, "{lines:?}");
    assert!(lines[0].starts_with("madswan/12: "));
    assert!(lines[0].contains(&place_of("a")));
}

/// The work-into-flume spec's "A quiet pass".
#[test]
fn a_quiet_pass() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![
        Want::Offer(offer("a", "a")),
        fakes::act("a/priority", "a", boomdrive_control::ActSort::Priority(3)),
    ]);
    let (_, lines) = bench.pass(&mut plane);
    assert_eq!(lines.len(), 2, "{lines:?}");
    for _ in 0..2 {
        let (passed, lines) = bench.pass(&mut plane);
        assert_eq!(lines, [] as [String; 0]);
        assert!(passed.reported);
    }
}
