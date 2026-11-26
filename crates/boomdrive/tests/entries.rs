//! sluice's entry of each submission Flume names, carried from sluice's
//! own answer with no log, and read again until it is final (design
//! Decision 13; core.11; the tool-records spec's "Each submission Flume
//! names is carried from sluice's own answer", "No log of sluice's is
//! carried" and "An entry is read until it is final"; Task notes 5.1).

mod fakes;

use boomdrive_control::fake::Fake;
use boomdrive_control::{At, Carried, Id, Want};
use fakes::{offer, place_of, sluice_fixture, Bench, KEPT};
use serde_json::{json, Value};
use std::fs;

/// A bench whose launch `scratch/1`, a fix named `parser`, has been
/// launched and its status carried, naming no submission, with the
/// control plane holding it.
fn launched() -> (Bench, Fake) {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![Want::Offer(offer("scratch/1", "parser"))]);
    bench.pass(&mut plane);
    assert!(bench.report().launches[&Id::new("scratch/1")]
        .status
        .is_some());
    (bench, plane)
}

/// sluice 0.110.2's refusal of a number on two branches, asked with no
/// branch.
const AMBIGUOUS: &str = r#"{"error":{"kind":"refused","message":"submission 18 is on more than one branch: bolt/parser, main; name one with --branch, as sluice status 18 --branch bolt/parser","next":["sluice status 18 --branch bolt/parser","sluice status 18 --branch main"]}}"#;

/// The place of `parser` submitted, Flume's status naming sluice's entry
/// `id` on `main`, as Flume 0.4.1 names one (`status-submitted.json`).
fn submitted(bench: &Bench, id: &str) {
    submitted_to(bench, id, "main");
}

/// The place of `parser` submitted, Flume's status naming sluice's entry
/// `id` in the queue of `target`.
fn submitted_to(bench: &Bench, id: &str, target: &str) {
    let submission = json!({
        "read": { "found": { "submission": {
            "id": id, "branch": "fix/parser", "target": target, "state": "under_test"
        } } },
        "submits": 1, "send_backs": 0, "verdict": null, "waiting": null, "held": false
    });
    bench
        .flume
        .set(&place_of("parser"), "submission", submission);
}

/// The entry the report file carries for `scratch/1`.
fn entry(bench: &Bench) -> Option<Carried> {
    bench.report().launches[&Id::new("scratch/1")].entry.clone()
}

/// sluice's fixture `name`, its submission's `id` written `id` and its
/// `state` written `state`, each otherwise as sluice printed it.
fn as_if(name: &str, id: u64, state: &str) -> String {
    let printed = sluice_fixture(name);
    let value: Value = serde_json::from_str(&printed).unwrap();
    printed
        .replacen(
            &format!("\"id\": {}", value["id"]),
            &format!("\"id\": {id}"),
            1,
        )
        .replacen(
            &format!("\"state\": {}", value["state"]),
            &format!("\"state\": \"{state}\""),
            1,
        )
}

/// sluice's fixture `status-landed.json`, its submission's `id` written
/// `id`, its `branch` written `branch` and its `state` written `state`.
fn on(branch: &str, id: u64, state: &str) -> String {
    as_if("status-landed.json", id, state).replacen(
        "\"branch\": \"main\"",
        &format!("\"branch\": \"{branch}\""),
        1,
    )
}

/// The tool-records spec's "An entry carried": sluice is asked in the
/// clone's container, and what it printed is carried.
#[test]
fn an_entry_carried() {
    let (bench, mut plane) = launched();
    submitted(&bench, "412");
    let printed = as_if("status-landed.json", 412, "landed");
    bench.sluice.answers_text("status", &printed, 0);
    let before = At::now();
    let (passed, lines) = bench.pass(&mut plane);
    assert!(passed.reported);
    assert_eq!(lines, [] as [String; 0]);
    assert_eq!(
        bench.sluice.calls(),
        [["status", "412", "--branch", "main", "--json"]]
    );
    let container = bench.container(KEPT).canonicalize().unwrap();
    assert_eq!(bench.sluice.dir_of(1), Some(container));
    let entry = entry(&bench).expect("an entry carried");
    assert!(entry.listed && entry.at >= before);
    assert_eq!(
        entry.text,
        r#"{"id":412,"branch":"main","head":"3cb12e20c76c17ef51cb8be1c32bec8671a7ab73","state":"landed","landed_as":"fdaaeaf1b1094b39db4ccea5f01e88a3bedb95d8"}"#
    );
    let carried: Value = serde_json::from_str(&entry.text).unwrap();
    let answer: Value = serde_json::from_str(&printed).unwrap();
    for field in ["id", "branch", "head", "state", "landed_as"] {
        assert_eq!(carried[field], answer[field], "{field}");
    }
}

/// The tool-records spec's "No submission".
#[test]
fn no_submission() {
    let (bench, mut plane) = launched();
    bench.sluice.answers("status", "status-landed.json", 0);
    bench.pass(&mut plane);
    assert_eq!(bench.sluice.calls(), [] as [Vec<String>; 0]);
    assert_eq!(entry(&bench), None);
}

/// No `sluice` runs for a launch not yet launched, whose place Flume has
/// not made.
#[test]
fn a_launch_held_runs_no_sluice() {
    let bench = Bench::new();
    let mut after = offer("scratch/2", "after");
    after.launch.after = vec![Id::new("scratch/1")];
    let mut plane = Fake::holding(vec![Want::Offer(after)]);
    bench.pass(&mut plane);
    bench.pass(&mut plane);
    assert_eq!(bench.sluice.calls(), [] as [Vec<String>; 0]);
}

/// The tool-records spec's "A failed row without its log", over a row
/// whose `flaky` reading names a log too.
#[test]
fn a_failed_row_without_its_log() {
    for (name, id, task, hash) in [
        (
            "status-failed.json",
            "2",
            "sh check.sh",
            "09e46ec47a28327f23dfc07c480bec754024a2e7",
        ),
        (
            "status-withdrawn.json",
            "4",
            "sh check.sh",
            "871867da54ed6ec660b7123312f2dc68760f7ec2",
        ),
    ] {
        let (bench, mut plane) = launched();
        submitted(&bench, id);
        bench.sluice.answers("status", name, 0);
        bench.pass(&mut plane);
        let entry = entry(&bench).expect("an entry carried");
        assert!(!entry.text.contains("\"log\""), "{}", entry.text);
        let carried: Value = serde_json::from_str(&entry.text).unwrap();
        let row = &carried["failed"][0];
        assert_eq!(
            (row["task"].as_str(), row["hash"].as_str()),
            (Some(task), Some(hash))
        );
        assert!(row.get("log").is_none(), "{row}");
        assert!(entry.text.contains(&format!("\"hash\":\"{hash}\"")));
        let file = fs::read_to_string(bench.report_file()).unwrap();
        assert!(!file.contains("/.logs/"), "a log's location is kept");
        for call in bench.sluice.calls() {
            assert_eq!(call[0], "status", "sluice was asked for more: {call:?}");
        }
    }
}

/// The tool-records spec's "A landed entry".
#[test]
fn a_landed_entry() {
    let (bench, mut plane) = launched();
    submitted(&bench, "1");
    bench.sluice.answers("status", "status-landed.json", 0);
    bench.pass(&mut plane);
    assert_eq!(bench.sluice.calls().len(), 1);
    bench.pass(&mut plane);
    bench.pass(&mut plane);
    assert_eq!(
        bench.sluice.calls().len(),
        1,
        "a landed entry is read again"
    );
}

/// An entry is read at each pass until its state is final, carried anew
/// only when what is carried of it changes.
#[test]
fn an_entry_is_read_until_it_is_final() {
    let (bench, mut plane) = launched();
    submitted(&bench, "2");
    bench
        .sluice
        .answers_text("status", &as_if("status-failed.json", 2, "testing"), 0);
    bench.pass(&mut plane);
    let testing = entry(&bench).expect("an entry carried");
    assert!(
        testing.text.contains(r#""state":"testing""#),
        "{}",
        testing.text
    );
    let held = fs::read(bench.report_file()).unwrap();
    let (_, lines) = bench.pass(&mut plane);
    assert_eq!(
        bench.sluice.calls().len(),
        2,
        "an entry testing is read again"
    );
    assert_eq!(
        fs::read(bench.report_file()).unwrap(),
        held,
        "nothing changed"
    );
    assert_eq!(lines, [] as [String; 0]);

    bench.sluice.answers("status", "status-failed.json", 0);
    let before = At::now();
    bench.pass(&mut plane);
    let failed = entry(&bench).expect("an entry carried");
    assert!(
        failed.text.contains(r#""state":"failed""#),
        "{}",
        failed.text
    );
    assert!(failed.at >= before);
    bench.pass(&mut plane);
    assert_eq!(
        bench.sluice.calls().len(),
        3,
        "a failed entry is read again"
    );
}

/// A resubmission names a new entry, which is read though the entry
/// carried before is final.
#[test]
fn a_new_submission_is_read() {
    let (bench, mut plane) = launched();
    submitted(&bench, "4");
    bench.sluice.answers("status", "status-withdrawn.json", 0);
    bench.pass(&mut plane);
    submitted(&bench, "5");
    bench
        .sluice
        .answers_text("status", &as_if("status-withdrawn.json", 5, "queued"), 0);
    bench.pass(&mut plane);
    assert_eq!(
        bench.sluice.calls()[1],
        ["status", "5", "--branch", "main", "--json"]
    );
    let carried: Value = serde_json::from_str(&entry(&bench).unwrap().text).unwrap();
    assert_eq!(
        (carried["id"].clone(), carried["state"].clone()),
        (json!(5), json!("queued"))
    );
}

/// Where sluice refuses, the entry last carried stands, not listed, and
/// the refusal is kept and said once: sluice is asked for that entry no
/// more.
#[test]
fn an_entry_sluice_refuses_keeps_what_was_carried() {
    let (bench, mut plane) = launched();
    submitted(&bench, "2");
    let testing = as_if("status-failed.json", 2, "testing");
    bench.sluice.answers_text("status", &testing, 0);
    bench.pass(&mut plane);
    let carried = entry(&bench).unwrap();
    bench.sluice.answers("status", "status-missing.json", 1);
    let (_, lines) = bench.pass(&mut plane);
    assert_eq!(
        lines,
        ["scratch/1: sluice's entry 2 on main is refused, and is asked for no more: no submission 99"]
    );
    let refused = entry(&bench).unwrap();
    assert_eq!((&refused.text, refused.at), (&carried.text, carried.at));
    assert!(!refused.listed);
    let (_, lines) = bench.pass(&mut plane);
    assert_eq!(lines, [] as [String; 0], "the refusal is said again");
    assert_eq!(
        bench.sluice.calls().len(),
        2,
        "a refused entry is asked again"
    );
    let kept = bench.report().launches[&Id::new("scratch/1")]
        .entry_refused
        .clone()
        .expect("the refusal kept");
    assert_eq!(
        (kept.id.as_str(), kept.branch.as_str(), kept.words.as_str()),
        ("2", "main", "no submission 99")
    );
}

/// The tool-records spec's "A refused entry is asked for no more": a
/// refusal sluice gives at every ask runs sluice once, however many
/// passes follow, and another entry the status names is asked for.
#[test]
fn a_refused_entry_is_asked_for_no_more() {
    let (bench, mut plane) = launched();
    submitted(&bench, "18");
    bench.sluice.answers("status", "status-missing.json", 1);
    for _ in 0..5 {
        bench.pass(&mut plane);
    }
    assert_eq!(
        bench.sluice.calls(),
        [["status", "18", "--branch", "main", "--json"]]
    );
    assert_eq!(entry(&bench), None);

    submitted(&bench, "19");
    bench
        .sluice
        .answers_text("status", &as_if("status-landed.json", 19, "landed"), 0);
    bench.pass(&mut plane);
    bench.pass(&mut plane);
    assert_eq!(
        bench.sluice.calls()[1..],
        [["status", "19", "--branch", "main", "--json"]]
    );
    let launched = &bench.report().launches[&Id::new("scratch/1")];
    assert_eq!(launched.entry_refused, None, "the refusal is left");
    let carried: Value = serde_json::from_str(&launched.entry.as_ref().unwrap().text).unwrap();
    assert_eq!(
        (carried["id"].clone(), carried["state"].clone()),
        (json!(19), json!("landed"))
    );
}

/// The tool-records spec's "A number on two branches": sluice is asked
/// for the entry by its number and the branch whose queue holds it, as
/// Flume names them, and answers with that branch's entry alone.
#[test]
fn a_number_on_two_branches() {
    let (bench, mut plane) = launched();
    submitted_to(&bench, "18", "bolt/parser");
    bench.sluice.answers_text("status", AMBIGUOUS, 1);
    bench
        .sluice
        .answers_on("status", "bolt/parser", &on("bolt/parser", 18, "passed"), 0);
    bench
        .sluice
        .answers_on("status", "main", &on("main", 18, "landed"), 0);
    let (_, lines) = bench.pass(&mut plane);
    assert_eq!(lines, [] as [String; 0]);
    assert_eq!(
        bench.sluice.calls(),
        [["status", "18", "--branch", "bolt/parser", "--json"]]
    );
    let carried: Value = serde_json::from_str(&entry(&bench).unwrap().text).unwrap();
    assert_eq!(
        (carried["branch"].clone(), carried["state"].clone()),
        (json!("bolt/parser"), json!("passed"))
    );
}

/// A final entry on one branch leaves sluice asked for the same number
/// on another branch, which is another entry.
#[test]
fn the_same_number_on_another_branch_is_read() {
    let (bench, mut plane) = launched();
    submitted_to(&bench, "18", "bolt/parser");
    bench
        .sluice
        .answers_on("status", "bolt/parser", &on("bolt/parser", 18, "landed"), 0);
    bench.pass(&mut plane);
    submitted_to(&bench, "18", "main");
    bench
        .sluice
        .answers_on("status", "main", &on("main", 18, "queued"), 0);
    bench.pass(&mut plane);
    assert_eq!(
        bench.sluice.calls()[1],
        ["status", "18", "--branch", "main", "--json"]
    );
    let carried: Value = serde_json::from_str(&entry(&bench).unwrap().text).unwrap();
    assert_eq!(
        (carried["branch"].clone(), carried["state"].clone()),
        (json!("main"), json!("queued"))
    );
}

/// A `sluice` that cannot be run is named, keeps nothing, and is asked
/// again at the next pass.
#[test]
fn a_sluice_not_there() {
    let (bench, mut plane) = launched();
    submitted(&bench, "1");
    fs::remove_file(bench.sluice.program()).unwrap();
    let (passed, lines) = bench.pass(&mut plane);
    assert!(passed.reported);
    assert_eq!(lines.len(), 1, "{lines:?}");
    assert!(
        lines[0].starts_with(
            "scratch/1: sluice's entry 1 on main could not be read: sluice could not run: "
        ),
        "{lines:?}"
    );
    assert_eq!(entry(&bench), None);
    bench.sluice.answers("status", "status-landed.json", 0);
    fs::copy(fakes::SluiceStub::new().program(), bench.sluice.program()).unwrap();
    let (passed, _) = bench.pass(&mut plane);
    assert!(passed.reported);
    assert!(entry(&bench).is_some(), "sluice is asked again");
}

/// The tool-records spec's "An entry carried from a repository drained":
/// once the configuration drops the launch's repository, sluice is asked
/// in its clone's container, which the runner drains (record 0021).
#[test]
fn an_entry_carried_from_a_repository_drained() {
    let bench = Bench::new();
    let kit = bench.container("example/kit");
    fs::create_dir_all(&kit).unwrap();
    bench.clones.ready("example/kit", &kit);
    let mut parser = offer("scratch/1", "parser");
    parser.launch.repository = "example/kit".to_string();
    let mut plane = Fake::holding(vec![Want::Offer(parser)]);
    bench.pass(&mut plane);
    bench.clones.drop_named("example/kit");
    submitted(&bench, "412");
    let printed = as_if("status-landed.json", 412, "landed");
    bench.sluice.answers_text("status", &printed, 0);
    let (passed, lines) = bench.pass(&mut plane);
    assert!(passed.reported);
    assert_eq!(lines, [] as [String; 0]);
    assert_eq!(plane.hellos()[1].draining, ["example/kit"]);
    assert_eq!(
        bench.sluice.calls(),
        [["status", "412", "--branch", "main", "--json"]]
    );
    assert_eq!(bench.sluice.dir_of(1), Some(kit.canonicalize().unwrap()));
    let entry = entry(&bench).expect("an entry carried");
    let carried: Value = serde_json::from_str(&entry.text).unwrap();
    assert_eq!(carried["id"], json!(412));
    assert_eq!(carried["state"], json!("landed"));
}
