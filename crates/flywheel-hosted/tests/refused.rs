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
