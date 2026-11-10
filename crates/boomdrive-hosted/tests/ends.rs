//! Version 2 names every end Flume 0.21.0 gives a place (the
//! runner-protocol spec's "Version 2 names every end Flume 0.21.0 gives a
//! place"; design Decision 1 of
//! the-hosted-protocol-carries-every-end-of-a-place; record 0037): each
//! of a status copy's ends, a stage's, a submission's, a stall's and the
//! end of the place's work by a drop or a done, with its key, its
//! delivery's key and expiry, and the readers it is owed to and those that
//! confirmed it, checked against version 2's copy of a place's status.

mod common;

use common::{definition, example, passes, refused, validator};
use jsonschema::Validator;
use serde_json::{json, Value};

/// The validator of a place's status copy.
fn place_status() -> Validator {
    validator(&definition("place-status.schema.json", "place_status"))
}

/// The status copy of a tidied place, version 2's example of one, its
/// ends `ends`.
fn copy_with(ends: Value) -> Value {
    let report = example("from-runner", "report-a-tidied-place.json");
    let mut copy = report["body"]["report"]["launches"]["offer-6"]["status"]["copy"].clone();
    copy["ends"] = ends;
    copy
}

/// The end of the place's work by `over`, `dropped` or `done`, as Flume
/// 0.21.0 prints it, owed to `scratch/planner` and `boomdrive`.
fn over(over: &str) -> Value {
    json!({
        "of": "over",
        "over": over,
        "read_at": 1793559700,
        "deliveries": [],
        "key": format!("over/{over}"),
        "pending_for": ["scratch/planner", "boomdrive"],
        "delivery": format!("fix--gate/0a1b2c3d/over/{over}"),
        "expires_at": 1794164500,
        "owed": [
            { "recipient": "scratch/planner", "expired": false, "known": true },
            { "recipient": "boomdrive", "expired": false, "known": true }
        ],
        "confirmed": []
    })
}

/// A stage's end, as Flume 0.21.0 prints it, handed over to
/// `scratch/planner` by `by` and confirmed by it.
fn stage_end(by: &str) -> Value {
    json!({
        "of": "stage",
        "stage": "code",
        "start": 1,
        "state": "delivered",
        "end": {
            "head": "46588f14e46a90df6f92a257faf92da5e900ad92",
            "words": null,
            "needs": null,
            "commits": [],
            "made": []
        },
        "read_at": 1793559675,
        "deliveries": [{ "recipient": "scratch/planner", "at": 1793559680, "by": by }],
        "key": "stage/code/1/delivered",
        "pending_for": ["boomdrive"],
        "delivery": "fix--gate/0a1b2c3d/stage/code/1/delivered",
        "expires_at": 1794164475,
        "owed": [{ "recipient": "boomdrive", "expired": false, "known": true }],
        "confirmed": [{ "recipient": "scratch/planner", "at": 1793559680 }]
    })
}

/// The runner-protocol spec's "A drop's and a done's end".
#[test]
fn a_drops_and_a_dones_end() {
    let validator = place_status();
    for word in ["dropped", "done"] {
        passes(
            &validator,
            &copy_with(json!([over(word)])),
            &format!("a status copy holding the end of its work, {word}"),
        );
    }
    let mut no_word = over("dropped");
    no_word["over"] = json!("abandoned");
    refused(
        &validator,
        &copy_with(json!([no_word])),
        "an end of the work over by a word Flume has not",
    );
}

/// The runner-protocol spec's "A hand-over at a session's start".
#[test]
fn a_hand_over_at_a_sessions_start() {
    let validator = place_status();
    for by in ["notice", "wait", "inbox", "session_start"] {
        passes(
            &validator,
            &copy_with(json!([stage_end(by)])),
            &format!("a status copy whose end was handed over by {by}"),
        );
    }
    refused(
        &validator,
        &copy_with(json!([stage_end("pigeon")])),
        "an end handed over by a way Flume has not",
    );
}

/// The runner-protocol spec's "An end with no key".
#[test]
fn an_end_with_no_key() {
    let validator = place_status();
    for field in ["key", "owed", "confirmed"] {
        let mut end = stage_end("notice");
        end.as_object_mut().unwrap().remove(field);
        refused(
            &validator,
            &copy_with(json!([end])),
            &format!("a status copy whose stage's end has no {field}"),
        );
    }
    let mut end = over("done");
    end.as_object_mut().unwrap().remove("key");
    refused(
        &validator,
        &copy_with(json!([end])),
        "a status copy whose end of its work has no key",
    );
    let mut end = stage_end("notice");
    for field in ["delivery", "expires_at"] {
        end.as_object_mut().unwrap().remove(field);
    }
    passes(
        &validator,
        &copy_with(json!([end])),
        "a stage's end with no delivery, as Flume prints one with no recipient",
    );
}

/// An end `of` Flume adds, `moved`, by its delivery alone: its key, when
/// Flume read it, its delivery and its readers, and nothing else.
fn moved() -> Value {
    json!({
        "of": "moved",
        "key": "moved/1793559690",
        "read_at": 1793559690,
        "pending_for": ["boomdrive"],
        "delivery": "fix--gate/0a1b2c3d/moved/1793559690",
        "expires_at": 1794164490,
        "owed": [{ "recipient": "boomdrive", "expired": false, "known": true }],
        "confirmed": [{ "recipient": "scratch/planner", "at": 1793559695 }]
    })
}

/// The runner-protocol spec's "An end of a form unnamed" (the
/// requirement "An end of a form version 2 does not name goes up as its
/// delivery alone").
#[test]
fn an_end_of_a_form_unnamed() {
    let validator = place_status();
    passes(
        &validator,
        &copy_with(json!([moved()])),
        "a status copy whose end is of a form unnamed, by its delivery alone",
    );
    let mut bare = moved();
    for field in ["delivery", "expires_at"] {
        bare.as_object_mut().unwrap().remove(field);
    }
    passes(
        &validator,
        &copy_with(json!([bare])),
        "an end by its delivery alone with no delivery, as Flume prints one with no recipient",
    );
    let mut stage = moved();
    stage["of"] = json!("stage");
    stage["key"] = json!("stage/code/1/dreaming");
    passes(
        &validator,
        &copy_with(json!([stage, over("done")])),
        "a stage's end by its delivery alone, beside an end that passes whole",
    );
    for field in ["of", "key", "read_at", "owed", "confirmed", "pending_for"] {
        let mut end = moved();
        end.as_object_mut().unwrap().remove(field);
        refused(
            &validator,
            &copy_with(json!([end])),
            &format!("an end by its delivery alone with no {field}"),
        );
    }
}

/// The runner-protocol spec's "Content beside a delivery alone".
#[test]
fn content_beside_a_delivery_alone() {
    let validator = place_status();
    for (field, value) in [
        (
            "deliveries",
            json!([{ "recipient": "scratch/planner", "at": 1793559695, "by": "notice" }]),
        ),
        ("stage", json!("code")),
        ("words", json!("the agent's words")),
    ] {
        let mut end = moved();
        end[field] = value;
        refused(
            &validator,
            &copy_with(json!([end])),
            &format!("an end by its delivery alone holding its {field}"),
        );
    }
}
