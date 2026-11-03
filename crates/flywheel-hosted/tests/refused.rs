//! What version 1 refuses (design Decision 14): each case a valid example
//! altered, checked against its direction's envelope, refused, and named
//! in its assertion.

mod common;

use common::{example, from_runner, passes, refused, to_runner};
use serde_json::{json, Value};

/// The example `name` of `direction`, altered by `alter`.
fn altered(direction: &str, name: &str, alter: impl FnOnce(&mut Value)) -> Value {
    let mut value = example(direction, name);
    alter(&mut value);
    value
}

/// The runner-protocol spec's "An act's outcome": an act or a command is
/// done or refused, never launched.
#[test]
fn an_acts_outcome() {
    let validator = from_runner();
    passes(
        &validator,
        &example("from-runner", "report.json"),
        "report.json",
    );
    let acted = altered("from-runner", "report.json", |report| {
        report["body"]["report"]["acts"]["act-1"]["outcome"] = json!("launched");
    });
    refused(&validator, &acted, "an acted whose outcome is launched");
    let commanded = altered("from-runner", "report.json", |report| {
        report["body"]["report"]["commands"]["command-1"]["outcome"] = json!("launched");
    });
    refused(
        &validator,
        &commanded,
        "a commanded whose outcome is launched",
    );
}

/// The runner-protocol spec's "A reported with no report named": a
/// reported names the report message it answers.
#[test]
fn a_reported_with_no_report_named() {
    let validator = to_runner();
    passes(
        &validator,
        &example("to-runner", "reported.json"),
        "reported.json",
    );
    let unnamed = altered("to-runner", "reported.json", |reported| {
        let body = reported["body"]["reported"].as_object_mut().unwrap();
        body.remove("report");
    });
    refused(&validator, &unnamed, "a reported with no report");
    let empty = altered("to-runner", "reported.json", |reported| {
        reported["body"]["reported"]["report"] = json!("");
    });
    refused(&validator, &empty, "a reported naming an empty report id");
}

/// The copy of `report`'s one launch's status.
fn copy(report: &mut Value) -> &mut Value {
    let launches = report["body"]["report"]["launches"]
        .as_object_mut()
        .unwrap();
    let launched = launches.values_mut().next().unwrap();
    &mut launched["status"]["copy"]
}

/// A case of a status copy refused: what it holds, the example it alters,
/// and how it alters that example's copy.
type Holding = (&'static str, &'static str, fn(&mut Value));

/// The runner-protocol spec's "Text the cloud does not keep": a place's
/// status copy holds none of it (core.27).
#[test]
fn text_the_cloud_does_not_keep() {
    let validator = from_runner();
    let live = "report-a-live-agent-with-its-pane.json";
    let cases: [Holding; 8] = [
        ("an agent's last_words", live, |copy| {
            copy["agent"]["last_words"] = json!("Done.");
        }),
        ("an agent's screen", live, |copy| {
            copy["agent"]["screen"] = json!("$ cargo test\n");
        }),
        ("an agent's printed command", live, |copy| {
            copy["agent"]["printed"] =
                json!("cd /home/runner/places/change--parse-words && claude");
        }),
        ("a task's text", live, |copy| {
            copy["tasks"][0]["text"] = json!("1.1 Read a word as its parts.");
        }),
        ("readiness", live, |copy| {
            copy["readiness"] = json!({"steps": []});
        }),
        ("a group's title", live, |copy| {
            copy["stages"][1]["group"]["title"] = json!("The parser");
        }),
        (
            "a teardown's words",
            "report-a-landing-kept-by-a-failed-teardown.json",
            |copy| {
                copy["phase"]["over"]["kept"]["teardown"]["words"] = json!("sluice: not found");
            },
        ),
        (
            "an end's report",
            "report-a-place-waiting-on-an-answer.json",
            |copy| {
                copy["stages"][0]["end"]["report"] = json!("/home/runner/state/report.md");
            },
        ),
    ];
    for (what, name, alter) in cases {
        passes(&validator, &example("from-runner", name), name);
        let holding = altered("from-runner", name, |report| alter(copy(report)));
        refused(
            &validator,
            &holding,
            &format!("a status copy holding {what}"),
        );
    }
}

/// The runner-protocol spec's "A stream's queue": a hello's capacity
/// holds no stream's queue, which names places of every partition on the
/// host (core.26).
#[test]
fn a_streams_queue() {
    let validator = from_runner();
    passes(
        &validator,
        &example("from-runner", "hello.json"),
        "hello.json",
    );
    let queued = altered("from-runner", "hello.json", |hello| {
        let streams = &mut hello["body"]["hello"]["capacity"]["printed"]["streams"];
        streams[0]["queue"] = json!([]);
    });
    refused(&validator, &queued, "a capacity's stream holding its queue");
}

/// The runner-protocol spec's "A row's log": sluice's entry holds no
/// failed row's log (core.11).
#[test]
fn a_rows_log() {
    let validator = from_runner();
    let name = "report-a-submission-failed.json";
    passes(&validator, &example("from-runner", name), name);
    let logged = altered("from-runner", name, |report| {
        let entry = &mut report["body"]["report"]["launches"]["offer-8"]["entry"]["copy"];
        entry["failed"][0]["log"] = json!("/var/lib/sluice/logs/main/c41/app:test.log");
    });
    refused(&validator, &logged, "an entry's failed row holding its log");
}
