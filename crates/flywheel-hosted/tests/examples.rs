//! Version 1's examples, each a whole message under
//! `examples/from-runner/` or `examples/to-runner/`, pass their
//! direction's envelope, and the runner's types read each and write it
//! back as it was (design Decisions 2, 13 and 14).

mod common;

use common::{example, examples, from_runner, passes, to_runner, version_dir};
use flywheel_hosted::protocol::v1::{FromRunner, Message, MessageId, ToRunner};
use jsonschema::Validator;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeSet;

/// Every kind a runner sends, and every kind a control plane sends
/// (design Decision 5).
const FROM_RUNNER: [&str; 7] = [
    "enrol",
    "hello",
    "report",
    "terminal_opened",
    "terminal_output",
    "terminal_closed",
    "message_refused",
];
const TO_RUNNER: [&str; 9] = [
    "enrolled",
    "enrol_refused",
    "pulled",
    "reported",
    "terminal_open",
    "terminal_resume",
    "terminal_input",
    "terminal_close",
    "message_refused",
];

/// Each example of `direction` read from its text into `Message<Body>`
/// and written back, the written value equal to the example's as JSON
/// and passing `validator`.
fn read_and_written_back<Body: Serialize + DeserializeOwned>(
    direction: &str,
    validator: &Validator,
) {
    for (name, value) in examples(direction) {
        let what = format!("{direction}/{name}");
        let file = version_dir().join("examples").join(direction).join(&name);
        let text = std::fs::read_to_string(&file).unwrap();
        let message: Message<Body> = serde_json::from_str(&text)
            .unwrap_or_else(|e| panic!("{what} does not read into its direction's Message: {e}"));
        let written = serde_json::to_string(&message).unwrap();
        let written: Value = serde_json::from_str(&written).unwrap();
        assert_eq!(written, value, "{what} written back differs");
        passes(validator, &written, &format!("{what} written back"));
    }
}

/// The kind each example of `direction` holds.
fn kinds(direction: &str) -> BTreeSet<String> {
    examples(direction)
        .into_iter()
        .flat_map(|(name, value)| {
            let body = value["body"].as_object().cloned().unwrap_or_default();
            assert_eq!(body.len(), 1, "{direction}/{name} holds one kind");
            body.into_iter().map(|(kind, _)| kind)
        })
        .collect()
}

#[test]
fn every_example_passes_its_directions_schema() {
    let directions = [("from-runner", from_runner()), ("to-runner", to_runner())];
    for (direction, validator) in directions {
        let examples = examples(direction);
        assert!(!examples.is_empty(), "no example {direction}");
        for (name, value) in examples {
            passes(&validator, &value, &format!("{direction}/{name}"));
        }
    }
}

/// The runner-protocol spec's "Every example read and written back".
#[test]
fn every_example_read_and_written_back() {
    read_and_written_back::<FromRunner>("from-runner", &from_runner());
    read_and_written_back::<ToRunner>("to-runner", &to_runner());
}

/// The runner-protocol spec's "Every kind has an example": each kind of a
/// direction is held by one example of it or more, and no example holds
/// a kind of the other direction's or of none.
#[test]
fn every_kind_has_an_example() {
    for (direction, listed) in [
        ("from-runner", &FROM_RUNNER[..]),
        ("to-runner", &TO_RUNNER[..]),
    ] {
        let listed: BTreeSet<String> = listed.iter().map(|kind| kind.to_string()).collect();
        assert_eq!(
            kinds(direction),
            listed,
            "the kinds of {direction}'s examples"
        );
    }
}

/// The runner-protocol spec's "A message refused": either end's
/// `message_refused` names a message by its id, with words.
#[test]
fn a_message_refused() {
    let from = example("from-runner", "message_refused.json");
    passes(&from_runner(), &from, "a runner's message_refused");
    let to = example("to-runner", "message_refused.json");
    passes(&to_runner(), &to, "a control plane's message_refused");
    for refusal in [&from, &to] {
        let body = &refusal["body"]["message_refused"];
        assert!(body["message"].is_string() && body["words"].is_string());
    }
}

/// The runner-protocol spec's "Enrolment's examples": the examples of
/// `enrol`, `enrolled` and `enrol_refused` each pass their direction's
/// envelope, read into their direction's type as their own kind, and
/// write back equal (design Decision 12).
#[test]
fn enrolments_examples() {
    let value = example("from-runner", "enrol.json");
    passes(&from_runner(), &value, "enrol.json");
    let enrol: Message<FromRunner> = serde_json::from_value(value.clone()).unwrap();
    let FromRunner::Enrol(body) = &enrol.body else {
        panic!("enrol.json reads as {:?}", enrol.body);
    };
    assert_eq!(body.machine, "host-a");
    assert_eq!(serde_json::to_value(&enrol).unwrap(), value, "enrol.json");
    let value = example("to-runner", "enrolled.json");
    passes(&to_runner(), &value, "enrolled.json");
    let enrolled: Message<ToRunner> = serde_json::from_value(value.clone()).unwrap();
    let ToRunner::Enrolled(body) = &enrolled.body else {
        panic!("enrolled.json reads as {:?}", enrolled.body);
    };
    assert_eq!(body.host, "host-1");
    assert_eq!(
        serde_json::to_value(&enrolled).unwrap(),
        value,
        "enrolled.json"
    );
    let name = "enrol_refused-a-lapsed-token.json";
    let value = example("to-runner", name);
    passes(&to_runner(), &value, name);
    let refused: Message<ToRunner> = serde_json::from_value(value.clone()).unwrap();
    let ToRunner::EnrolRefused(body) = &refused.body else {
        panic!("{name} reads as {:?}", refused.body);
    };
    assert_eq!(body.enrol, enrol.id, "{name} answers enrol.json by its id");
    assert_eq!(body.enrol, MessageId::new("runner:1"));
    assert_eq!(serde_json::to_value(&refused).unwrap(), value, "{name}");
}

/// The runner-protocol spec's "A terminal's messages": an example of each
/// terminal kind, and of a frame with and without a redraw, passes its
/// direction's envelope (design Decision 10).
#[test]
fn a_terminals_messages() {
    let from = from_runner();
    for name in [
        "terminal_opened.json",
        "terminal_output.json",
        "terminal_output-bytes-as-they-come.json",
        "terminal_output-a-redraw-after-a-resume.json",
        "terminal_closed.json",
    ] {
        passes(&from, &example("from-runner", name), name);
    }
    let to = to_runner();
    for name in [
        "terminal_open.json",
        "terminal_input.json",
        "terminal_resume.json",
        "terminal_close.json",
    ] {
        passes(&to, &example("to-runner", name), name);
    }
    let redraws: Vec<bool> = [
        "terminal_output-bytes-as-they-come.json",
        "terminal_output-a-redraw-after-a-resume.json",
    ]
    .iter()
    .map(|name| example("from-runner", name)["body"]["terminal_output"]["redraw"] == true)
    .collect();
    assert_eq!(redraws, [false, true], "a frame without and with a redraw");
}
