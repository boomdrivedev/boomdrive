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

use boomdrive::flume::{Cli, Reply, Unrun};
use boomdrive_control::fake::Fake;
use boomdrive_control::{
    ActSort, At, Carried, Command, CommandSort, Id, Lease, Offer, Outcome, Want,
};
use fakes::{act, fixture, offer, place_of, printed, Bench, Call, Stub, KEPT};
use serde_json::{json, Value};
use std::fs;
use std::slice;

/// The status the report file carries for `launch`.
fn carried(bench: &Bench, launch: &str) -> Carried {
    bench.report().launches[&Id::new(launch)]
        .status
        .clone()
        .expect("a status carried")
}

/// An offer naming the recipient `scratch/planner`, so its launch names
/// the runner one more, and Flume holds its place's end for the runner.
fn planned(id: &str, name: &str) -> Offer {
    let mut planned = offer(id, name);
    planned.launch.recipients = vec!["scratch/planner".into()];
    planned
}

/// The entry of the one place the status carried for `launch` holds.
fn carried_entry(bench: &Bench, launch: &str) -> Value {
    let answer: Value = serde_json::from_str(&carried(bench, launch).text).unwrap();
    answer["status"]["places"][0].clone()
}

/// The keys of each confirmation the fake Flume was asked, forgotten once
/// given.
fn confirmed(bench: &Bench) -> Vec<Vec<String>> {
    bench
        .flume
        .take_calls()
        .into_iter()
        .filter_map(|call| match call {
            Call::Confirm(_, keys) => Some(keys),
            _ => None,
        })
        .collect()
}

/// The delivery key of the landing of the place launched as `a`.
const LANDING: &str = "a/t/submission/1/landed";

/// The delivery key of the first code stage's end of the place launched
/// as `a`.
const CODE: &str = "a/t/stage/code/1/delivered";

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

/// The work-into-flume spec's "A landing held for the runner": a carried
/// place that lands and is tidied between two passes, its record held for
/// the runner alone, is carried as `<location>@<at>`, tidied and landed,
/// reported, and, once the control plane's answer names its status taken,
/// its landing confirmed once as the runner, after its status is read and
/// with no inbox; and at the pass after, Flume refusing the place's
/// status, that status stands, not listed, and nothing is confirmed
/// (design Decisions 3 to 5; Task notes 5.2).
#[test]
fn a_landing_held_for_the_runner() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![Want::Offer(planned("a", "a"))]);
    bench.pass(&mut plane);
    assert!(carried(&bench, "a").listed);
    let place = place_of("a");
    bench.flume.tidy_held(&place, 1793559694, &["boomdrive"]);
    bench.flume.take_calls();
    let (_, lines) = bench.pass(&mut plane);
    let calls = bench.flume.take_calls();
    let record = format!("{place}@1793559694");
    let status = carried(&bench, "a");
    let entry = carried_entry(&bench, "a");
    assert_eq!(entry["place"], record);
    assert_eq!(
        entry["phase"]["tidied"]["over"],
        json!({ "landed": { "commit": "c0ffee" } })
    );
    assert_eq!(
        entry["phase"]["tidied"]["pending_for"],
        json!(["boomdrive"])
    );
    assert!(status.listed);
    let read = calls
        .iter()
        .position(|call| matches!(call, Call::StatusOf(..)))
        .expect("the place's status read");
    let confirms: Vec<_> = calls
        .iter()
        .enumerate()
        .filter(|(_, call)| matches!(call, Call::Confirm(..)))
        .collect();
    assert_eq!(confirms.len(), 1, "{calls:?}");
    assert_eq!(
        confirms[0].1,
        &Call::Confirm("boomdrive".into(), vec![LANDING.into()])
    );
    assert!(read < confirms[0].0, "confirmed after its status is read");
    assert!(
        !calls.iter().any(|call| matches!(call, Call::Collect(_))),
        "no inbox: {calls:?}"
    );
    assert_eq!(lines, [] as [String; 0]);
    let sent = plane.reports().last().expect("the report sent");
    assert_eq!(sent.launches[&Id::new("a")].status.as_ref(), Some(&status));

    let (_, lines) = bench.pass(&mut plane);
    let after = carried(&bench, "a");
    assert_eq!((&after.text, after.at), (&status.text, status.at));
    assert!(!after.listed, "Flume refuses the place's status");
    assert_eq!(confirmed(&bench), [] as [Vec<String>; 0]);
    assert_eq!(lines, [] as [String; 0]);
}

/// The work-into-flume spec's "A drop held for the runner": an act drops
/// a launch whose place Flume then tidies, holding its record for the
/// runner, and the pass carries the record, tidied and dropped, as the
/// launch's status and, once its answer names that status taken, confirms
/// the drop's end once (design Decisions 3 to 5).
#[test]
fn a_drop_held_for_the_runner() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![Want::Offer(planned("a", "a"))]);
    bench.pass(&mut plane);
    plane.set_wants(vec![
        Want::Offer(planned("a", "a")),
        act("a/drop", "a", ActSort::Drop),
    ]);
    bench.flume.take_calls();
    let (_, lines) = bench.pass(&mut plane);
    let record = format!("{}@101", place_of("a"));
    let entry = carried_entry(&bench, "a");
    assert_eq!(entry["place"], record);
    assert_eq!(entry["phase"]["tidied"]["over"], json!("dropped"));
    assert_eq!(
        entry["phase"]["tidied"]["pending_for"],
        json!(["boomdrive"])
    );
    assert!(carried(&bench, "a").listed);
    assert_eq!(confirmed(&bench), [vec!["a/t/over/dropped".to_string()]]);
    assert_eq!(
        lines,
        [format!("a/drop: drop of a at {}: done", place_of("a"))]
    );

    bench.pass(&mut plane);
    assert_eq!(confirmed(&bench), [] as [Vec<String>; 0], "confirmed once");
    assert!(!carried(&bench, "a").listed);
}

/// The work-into-flume spec's "A live place's end": a stage's end of a
/// live place, pending for the runner and `scratch/planner`, is confirmed
/// as the runner once the answer to the report names the status taken,
/// and the next pass, Flume giving the end pending for `scratch/planner`
/// alone, confirms nothing (design Decisions 3 and 4).
#[test]
fn a_live_places_end() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![Want::Offer(planned("a", "a"))]);
    bench.pass(&mut plane);
    let place = place_of("a");
    bench.flume.end(
        &place,
        "stage/code/1/delivered",
        &["boomdrive", "scratch/planner"],
    );
    bench.flume.take_calls();
    let (_, lines) = bench.pass(&mut plane);
    assert_eq!(lines, [] as [String; 0]);
    let entry = carried_entry(&bench, "a");
    assert_eq!(entry["phase"], json!("live"));
    assert_eq!(
        entry["ends"][0]["pending_for"],
        json!(["boomdrive", "scratch/planner"])
    );
    assert_eq!(confirmed(&bench), [vec![CODE.to_string()]]);

    bench.pass(&mut plane);
    assert_eq!(confirmed(&bench), [] as [Vec<String>; 0]);
    let entry = carried_entry(&bench, "a");
    assert_eq!(entry["ends"][0]["delivery"], CODE);
    assert_eq!(entry["ends"][0]["pending_for"], json!(["scratch/planner"]));
    assert!(carried(&bench, "a").listed);
}

/// A bench that launched `a`, naming `scratch/planner`, at its place, and
/// whose Flume then landed and tidied that place, holding its record with
/// its landing pending for the runner alone; and the record.
fn held_for_the_runner() -> (Bench, Fake, String) {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![Want::Offer(planned("a", "a"))]);
    bench.pass(&mut plane);
    let place = place_of("a");
    bench.flume.tidy_held(&place, 1793559694, &["boomdrive"]);
    bench.flume.take_calls();
    (bench, plane, format!("{place}@1793559694"))
}

/// The work-into-flume spec's "A report not taken": a pass whose report
/// the control plane refuses confirms nothing, and the record stays
/// pending for the runner; the next pass, its report taken, confirms the
/// landing (design Decision 3).
#[test]
fn a_report_not_taken() {
    let (bench, mut plane, record) = held_for_the_runner();
    plane.refuse_reports("the control plane is away");
    let (passed, _) = bench.pass(&mut plane);
    assert!(!passed.reported);
    assert_eq!(confirmed(&bench), [] as [Vec<String>; 0]);
    let held = bench.flume.entry(&place_of("a")).expect("Flume holds it");
    assert_eq!(held["place"], record);
    assert_eq!(held["phase"]["tidied"]["pending_for"], json!(["boomdrive"]));
    plane.take_reports();
    let (passed, _) = bench.pass(&mut plane);
    assert!(passed.reported);
    assert_eq!(confirmed(&bench), [vec![LANDING.to_string()]]);
    assert!(bench.flume.entry(&place_of("a")).is_none());
}

/// The work-into-flume spec's "A status not taken": a report the control
/// plane takes, leaving the launch's status, confirms nothing of it; a
/// later pass whose answer names it taken confirms the landing (design
/// Decision 3).
#[test]
fn a_status_not_taken() {
    let (bench, mut plane, record) = held_for_the_runner();
    plane.leave_status(Id::new("a"));
    for _ in 0..2 {
        let (passed, _) = bench.pass(&mut plane);
        assert!(passed.reported);
        assert_eq!(confirmed(&bench), [] as [Vec<String>; 0]);
    }
    let sent = plane.reports().last().expect("the report sent");
    let status = sent.launches[&Id::new("a")].status.as_ref().unwrap();
    assert!(status.text.contains(&record));
    plane.take_status(Id::new("a"));
    bench.pass(&mut plane);
    assert_eq!(confirmed(&bench), [vec![LANDING.to_string()]]);
}

/// The work-into-flume spec's "The report file lost": the control plane
/// took a report carrying `a` live; its place then lands and is tidied,
/// held for the runner; a pass whose report is refused confirms nothing;
/// and with the report file removed, the next pass takes `a` from its
/// pull, carries the record from Flume, still pending for the runner, and
/// confirms its landing once its answer names the status taken (design
/// Decision 3; Task notes 5.3).
#[test]
fn the_report_file_lost() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![Want::Offer(planned("a", "a"))]);
    bench.pass(&mut plane);
    let (passed, _) = bench.pass(&mut plane);
    assert!(passed.reported);
    let live = plane.held().expect("the control plane holds the report");
    assert_eq!(carried_entry(&bench, "a")["phase"], json!("live"));
    assert!(live.launches[&Id::new("a")].status.is_some());
    let place = place_of("a");
    let record = format!("{place}@1793559694");
    bench.flume.tidy_held(&place, 1793559694, &["boomdrive"]);
    plane.refuse_reports("the control plane is away");
    bench.flume.take_calls();
    bench.pass(&mut plane);
    assert_eq!(confirmed(&bench), [] as [Vec<String>; 0]);
    fs::remove_file(bench.report_file()).expect("the report file removed");
    plane.take_reports();
    let (passed, _) = bench.pass(&mut plane);
    assert!(passed.reported);
    let entry = carried_entry(&bench, "a");
    assert_eq!(entry["place"], record);
    assert_eq!(
        entry["phase"]["tidied"]["pending_for"],
        json!(["boomdrive"])
    );
    assert_eq!(confirmed(&bench), [vec![LANDING.to_string()]]);
}

/// The work-into-flume spec's "A record pending for a recipient too": a
/// record Flume holds for the runner and for `scratch/planner` has its
/// landing confirmed once, and at the next pass is carried as Flume gives
/// it, pending for `scratch/planner` alone, listed, and nothing more is
/// confirmed (design Decision 4).
#[test]
fn a_record_pending_for_a_recipient_too() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![Want::Offer(planned("a", "a"))]);
    bench.pass(&mut plane);
    let place = place_of("a");
    let record = format!("{place}@1793559694");
    bench
        .flume
        .tidy_held(&place, 1793559694, &["boomdrive", "scratch/planner"]);
    bench.flume.take_calls();
    bench.pass(&mut plane);
    assert_eq!(confirmed(&bench), [vec![LANDING.to_string()]]);
    bench.pass(&mut plane);
    assert_eq!(
        confirmed(&bench),
        [] as [Vec<String>; 0],
        "no second confirmation"
    );
    let entry = carried_entry(&bench, "a");
    assert_eq!(entry["place"], record);
    assert_eq!(
        entry["phase"]["tidied"]["pending_for"],
        json!(["scratch/planner"])
    );
    assert!(carried(&bench, "a").listed);
}

/// The work-into-flume spec's "A record pending for no end of the
/// runner's": a place launched naming no recipient, as one launched before
/// the runner named itself one, tidied while its landing is pending for
/// `scratch/planner` alone, is carried as its record and nothing of it is
/// confirmed (design Decision 4).
#[test]
fn a_record_pending_for_no_end_of_the_runners() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![Want::Offer(offer("a", "a"))]);
    bench.pass(&mut plane);
    let place = place_of("a");
    bench.flume.tidy_pending(&place, 1793559694);
    bench.flume.take_calls();
    bench.pass(&mut plane);
    bench.pass(&mut plane);
    assert_eq!(confirmed(&bench), [] as [Vec<String>; 0]);
    let entry = carried_entry(&bench, "a");
    assert_eq!(entry["place"], format!("{place}@1793559694"));
    assert!(carried(&bench, "a").listed);
}

/// The work-into-flume spec's "A confirmation Flume could not run": the
/// pass says one line, prefixed with the launch's id, naming the record
/// and why, and the next pass, Flume running, confirms the landing (design
/// Decision 3).
#[test]
fn a_confirmation_flume_could_not_run() {
    let (bench, mut plane, record) = held_for_the_runner();
    bench
        .flume
        .confirm_answers(LANDING, Err(Unrun("flume was ended: signal: 9".into())));
    let (passed, lines) = bench.pass(&mut plane);
    assert!(passed.reported);
    let unrun =
        format!("a: the ends of {record} could not be confirmed: flume was ended: signal: 9");
    assert_eq!(lines, slice::from_ref(&unrun));
    assert_eq!(confirmed(&bench), [vec![LANDING.to_string()]]);
    bench.flume.confirm_as_flume(LANDING);
    let (_, lines) = bench.pass(&mut plane);
    assert_eq!(lines, [] as [String; 0]);
    assert_eq!(confirmed(&bench), [vec![LANDING.to_string()]]);
    assert!(
        bench.flume.entry(&place_of("a")).is_none(),
        "Flume holds the record no more"
    );
}

/// The work-into-flume spec's "A confirmation Flume refuses": the pass
/// says one line, prefixed with the launch's id, naming the record and
/// Flume's words, and the next pass confirms the landing again (design
/// Decision 3).
#[test]
fn a_confirmation_flume_refuses() {
    let (bench, mut plane, record) = held_for_the_runner();
    bench.flume.confirm_answers(
        LANDING,
        Ok(Reply::Refused("the host's state is held by another".into())),
    );
    let refused = format!(
        "a: Flume refused to confirm the ends of {record} as boomdrive: \
         the host's state is held by another"
    );
    let (_, lines) = bench.pass(&mut plane);
    assert_eq!(lines, slice::from_ref(&refused));
    assert_eq!(confirmed(&bench), [vec![LANDING.to_string()]]);
    let (_, lines) = bench.pass(&mut plane);
    assert_eq!(lines, slice::from_ref(&refused));
    assert_eq!(
        confirmed(&bench),
        [vec![LANDING.to_string()]],
        "confirmed again"
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

/// The repository a test drains.
const KIT: &str = "example/kit";

/// A bench that launched `a` at its place in `repository`, its clone made,
/// and carried its status, the control plane holding it.
fn launched_in(repository: &str) -> (Bench, Fake) {
    let bench = Bench::new();
    bench.clones.ready(repository, &bench.container(repository));
    let mut a = offer("a", "a");
    a.launch.repository = repository.to_string();
    let mut plane = Fake::holding(vec![Want::Offer(a)]);
    bench.pass(&mut plane);
    assert!(carried(&bench, "a").listed);
    (bench, plane)
}

/// The `status <place>` calls made of the fake Flume since last asked.
fn asked(bench: &Bench) -> Vec<Call> {
    bench
        .flume
        .take_calls()
        .into_iter()
        .filter(|call| matches!(call, Call::StatusOf(..)))
        .collect()
}

/// The work-into-flume spec's "A place in a repository drained" and "A
/// landing in a repository not kept": a launch's place is followed to its
/// end through its repository's clone, which the runner drains once the
/// configuration drops it, its landing and its tidied record carried, and
/// no keeper told (record 0021).
#[test]
fn a_place_in_a_repository_drained() {
    let (bench, mut plane) = launched_in(KIT);
    let container = bench.container(KIT);
    bench.clones.drop_named(KIT);
    bench.flume.take_calls();
    let place = place_of("a");
    bench.flume.land(&place);
    let (_, lines) = bench.pass(&mut plane);
    assert_eq!(lines, [] as [String; 0]);
    assert_eq!(plane.hellos()[1].draining, [KIT]);
    assert_eq!(
        asked(&bench),
        [Call::StatusOf(Some(container.clone()), place.clone())]
    );
    let landed: Value = serde_json::from_str(&carried(&bench, "a").text).unwrap();
    assert_eq!(
        landed["status"]["places"][0]["phase"]["over"]["over"],
        json!({ "landed": { "commit": "c0ffee" } })
    );
    assert_eq!(bench.landings(), [] as [String; 0], "no keeper told");

    bench.flume.tidy_pending(&place, 1793559694);
    bench.pass(&mut plane);
    assert_eq!(
        asked(&bench),
        [Call::StatusOf(Some(container), place.clone())]
    );
    let tidied = carried(&bench, "a");
    assert!(tidied.listed);
    let tidied: Value = serde_json::from_str(&tidied.text).unwrap();
    let entry = &tidied["status"]["places"][0];
    assert_eq!(entry["place"], format!("{place}@1793559694"));
    assert_eq!(
        entry["phase"]["tidied"]["over"],
        json!({ "landed": { "commit": "c0ffee" } })
    );
    assert_eq!(
        bench.report().launches[&Id::new("a")].answer.place_dir,
        Some(place)
    );
    assert_eq!(bench.landings(), [] as [String; 0], "no keeper told");

    bench.pass(&mut plane);
    assert_eq!(
        plane.hellos()[3].draining,
        [] as [String; 0],
        "a place tidied has ended"
    );
}

/// The work-into-flume spec's "A place in a repository with no clone":
/// once the runner has no clone of its repository, a launch's place is
/// asked of Flume no more, and its last status stands.
#[test]
fn a_place_in_a_repository_with_no_clone() {
    let (bench, mut plane) = launched_in("example/old-kit");
    let before = carried(&bench, "a");
    bench.clones.gone("example/old-kit");
    bench.flume.take_calls();
    bench.flume.land(&place_of("a"));
    let (passed, lines) = bench.pass(&mut plane);
    assert!(passed.reported);
    assert_eq!(lines, [] as [String; 0]);
    assert_eq!(asked(&bench), [] as [Call; 0]);
    assert_eq!(carried(&bench, "a"), before);
    assert_eq!(bench.landings(), [] as [String; 0]);
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
        .stand(json!({ "place": "/places/theirs", "phase": "live", "mark": "boomdrive:work/3" }));
    let mut plane = Fake::holding(vec![Want::Offer(offer("a", "a"))]);
    bench.pass(&mut plane);
    bench.flume.land("/places/mine");
    bench.pass(&mut plane);
    for call in bench.flume.calls() {
        let named = match &call {
            Call::Act(place, _) | Call::StatusOf(_, place) | Call::Collect(place) => {
                Some(place.as_str())
            }
            Call::Launch(_) | Call::Status | Call::Confirm(..) => None,
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
    let mut plane = Fake::holding(vec![Want::Offer(offer("home/12", "a"))]);
    let (_, lines) = bench.pass(&mut plane);
    assert_eq!(lines.len(), 1, "{lines:?}");
    assert!(lines[0].starts_with("home/12: "));
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
