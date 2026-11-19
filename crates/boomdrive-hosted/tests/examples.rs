//! Version 4's examples, each a whole message under
//! `examples/from-runner/` or `examples/to-runner/`, pass their
//! direction's envelope, and the runner's types read each and write it
//! back as it was (design Decisions 2, 13 and 14 of
//! the-runner-dials-the-hosted-control-plane). Version 1's, version 2's
//! and version 3's examples still pass their own version's envelopes
//! (records 0037, 0052 and 0053).

mod common;

use boomdrive_hosted::protocol::v4::{
    FromRunner, HelloForm, Message, MessageId, ReportForm, ToRunner,
};
use common::{example, examples, from_runner, passes, to_runner, version_dir, V1, V2};
use jsonschema::Validator;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeSet;

/// Every kind a runner sends, and every kind a control plane sends
/// (design Decision 5).
const FROM_RUNNER: [&str; 8] = [
    "enroll",
    "hello",
    "report",
    "terminal_opened",
    "terminal_output",
    "terminal_closed",
    "message_refused",
    "forge_token",
];
const TO_RUNNER: [&str; 11] = [
    "enrolled",
    "enroll_refused",
    "pulled",
    "reported",
    "terminal_open",
    "terminal_resume",
    "terminal_input",
    "terminal_close",
    "message_refused",
    "forge_token_given",
    "forge_token_refused",
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

/// The runner-protocol spec's "Every example read and written back":
/// the observing connection's among them, its hello and its report each
/// read as its observing form, and the working ones as theirs (design
/// Decision 1 of a-runner-reports-to-an-observer-it-takes-no-work-from).
#[test]
fn every_example_read_and_written_back() {
    read_and_written_back::<FromRunner>("from-runner", &from_runner());
    read_and_written_back::<ToRunner>("to-runner", &to_runner());
    let read = |name: &str| -> FromRunner {
        let message: Message<FromRunner> =
            serde_json::from_value(example("from-runner", name)).unwrap();
        message.body
    };
    assert!(matches!(
        read("hello-observing.json"),
        FromRunner::Hello(HelloForm::Observing(_))
    ));
    assert!(matches!(
        read("hello.json"),
        FromRunner::Hello(HelloForm::Working(_))
    ));
    assert!(matches!(
        read("report-observed.json"),
        FromRunner::Report(ReportForm::Observed(_))
    ));
    assert!(matches!(
        read("report.json"),
        FromRunner::Report(ReportForm::Working(_))
    ));
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

/// The runner-protocol spec's "Enrollment's examples": the examples of
/// `enroll`, `enrolled` and `enroll_refused` each pass their direction's
/// envelope, read into their direction's type as their own kind, and
/// write back equal (design Decision 12).
#[test]
fn enrollments_examples() {
    let value = example("from-runner", "enroll.json");
    passes(&from_runner(), &value, "enroll.json");
    let enroll: Message<FromRunner> = serde_json::from_value(value.clone()).unwrap();
    let FromRunner::Enroll(body) = &enroll.body else {
        panic!("enroll.json reads as {:?}", enroll.body);
    };
    assert_eq!(body.machine, "host-a");
    assert_eq!(serde_json::to_value(&enroll).unwrap(), value, "enroll.json");
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
    let name = "enroll_refused-a-lapsed-token.json";
    let value = example("to-runner", name);
    passes(&to_runner(), &value, name);
    let refused: Message<ToRunner> = serde_json::from_value(value.clone()).unwrap();
    let ToRunner::EnrollRefused(body) = &refused.body else {
        panic!("{name} reads as {:?}", refused.body);
    };
    assert_eq!(
        body.enroll, enroll.id,
        "{name} answers enroll.json by its id"
    );
    assert_eq!(body.enroll, MessageId::new("runner:1"));
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

/// The runner-protocol spec's "Version 1's examples still pass": each
/// example of version 1 passes its direction's envelope of version 1,
/// which the runner no longer speaks and a control plane may still take
/// (core.42; record 0037).
#[test]
fn version_1s_examples_still_pass() {
    let directions = [
        ("from-runner", V1.messages_from_runner()),
        ("to-runner", V1.messages_to_runner()),
    ];
    for (direction, validator) in directions {
        let examples = V1.examples(direction);
        assert!(!examples.is_empty(), "no example of version 1 {direction}");
        for (name, value) in examples {
            assert_eq!(
                value["protocol_version"], 1,
                "version 1's {direction}/{name}"
            );
            passes(
                &validator,
                &value,
                &format!("version 1's {direction}/{name}"),
            );
        }
    }
}

/// The runner-protocol spec's "Version 2's examples still pass": each
/// example of version 2 passes its direction's envelope of version 2,
/// which the runner no longer speaks and a control plane may still take
/// (core.42; record 0052).
#[test]
fn version_2s_examples_still_pass() {
    let directions = [
        ("from-runner", V2.messages_from_runner()),
        ("to-runner", V2.messages_to_runner()),
    ];
    for (direction, validator) in directions {
        let examples = V2.examples(direction);
        assert!(!examples.is_empty(), "no example of version 2 {direction}");
        for (name, value) in examples {
            assert_eq!(
                value["protocol_version"], 2,
                "version 2's {direction}/{name}"
            );
            passes(
                &validator,
                &value,
                &format!("version 2's {direction}/{name}"),
            );
        }
    }
}

/// The runner-protocol spec's "Version 3's examples still pass": each
/// example of version 3 passes its direction's envelope of version 3,
/// which the runner no longer speaks and a control plane may still take
/// (core.42; record 0053).
#[test]
fn version_3s_examples_still_pass() {
    let v3 = common::V3;
    let directions = [
        ("from-runner", v3.messages_from_runner()),
        ("to-runner", v3.messages_to_runner()),
    ];
    for (direction, validator) in directions {
        let examples = v3.examples(direction);
        assert!(!examples.is_empty(), "no example of version 3 {direction}");
        for (name, value) in examples {
            assert_eq!(
                value["protocol_version"], 3,
                "version 3's {direction}/{name}"
            );
            passes(
                &validator,
                &value,
                &format!("version 3's {direction}/{name}"),
            );
        }
    }
}
