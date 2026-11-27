//! The spoken version's examples, each a whole message under
//! `examples/from-runner/` or `examples/to-runner/`, pass their
//! direction's envelope, and the runner's types read each and write it
//! back as it was (design Decisions 2, 13 and 14 of
//! the-runner-dials-the-hosted-control-plane). Version 1's, version 2's
//! version 3's and version 4's examples still pass their own version's
//! envelopes (records 0037, 0052, 0053 and 0054). Version 6's examples
//! pass version 6's envelopes and read into the runner's types, its
//! `newest_release` among them, while the runner speaks version 5.

mod common;

use boomdrive_hosted::protocol::v7::{FromRunner, Message, MessageId, ToRunner};
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
/// each of version 5's, a hello and a report each read as the one form
/// version 5 has.
#[test]
fn every_example_read_and_written_back() {
    read_and_written_back::<FromRunner>("from-runner", &from_runner());
    read_and_written_back::<ToRunner>("to-runner", &to_runner());
    let read = |name: &str| -> FromRunner {
        let message: Message<FromRunner> =
            serde_json::from_value(example("from-runner", name)).unwrap();
        message.body
    };
    assert!(matches!(read("hello.json"), FromRunner::Hello(_)));
    assert!(matches!(read("report.json"), FromRunner::Report(_)));
    let FromRunner::Report(elsewhere) = read("report-a-place-started-elsewhere.json") else {
        panic!("a report");
    };
    assert_eq!(
        elsewhere.unleased["/home/runner/places/note--by-hand"].repository,
        "example/app"
    );
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
        let mut listed: BTreeSet<String> = listed.iter().map(|kind| kind.to_string()).collect();
        if direction == "to-runner" && common::current().number >= 6 {
            listed.insert("newest_release".to_string());
        }
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

/// The runner-protocol spec's "Version 4's examples still pass": each
/// example of version 4 passes its direction's envelope of version 4,
/// which the runner no longer speaks and a control plane may still take
/// (core.42; record 0054).
#[test]
fn version_4s_examples_still_pass() {
    let v4 = common::V4;
    let directions = [
        ("from-runner", v4.messages_from_runner()),
        ("to-runner", v4.messages_to_runner()),
    ];
    for (direction, validator) in directions {
        let examples = v4.examples(direction);
        assert!(!examples.is_empty(), "no example of version 4 {direction}");
        for (name, value) in examples {
            assert_eq!(
                value["protocol_version"], 4,
                "version 4's {direction}/{name}"
            );
            passes(
                &validator,
                &value,
                &format!("version 4's {direction}/{name}"),
            );
        }
    }
}

/// The runner-protocol spec's "A token's Debug names its end alone"
/// (design Decision 4 of a-cloud-host-gets-its-github-tokens-from-the-cloud;
/// core.4): version 5's `forge_token_given.json`, read into its type and
/// written with `{:?}`, names its ask, its repository and its end, and no
/// character of its token: the text is the same whatever the token is.
#[test]
fn a_tokens_debug_names_its_end_alone() {
    let given = example("to-runner", "forge_token_given.json");
    let token = given["body"]["forge_token_given"]["token"]
        .as_str()
        .expect("a token")
        .to_string();
    let written = |value: Value| -> String {
        let message: Message<ToRunner> = serde_json::from_value(value).expect("a message");
        let ToRunner::ForgeTokenGiven(given) = message.body else {
            panic!("forge_token_given.json holds no forge_token_given");
        };
        format!("{given:?}")
    };
    let text = written(given.clone());
    for named in ["runner:9", "example/app", "2026-11-18T13:00:00Z"] {
        assert!(text.contains(named), "{text} names no {named}");
    }
    assert!(!text.contains(&token), "{text} holds the token");
    let mut other = given;
    other["body"]["forge_token_given"]["token"] = Value::from("another-token-entirely");
    assert_eq!(
        written(other),
        text,
        "the Debug of a token given turns on its token"
    );
}

/// The runner-protocol spec's "Version 6's examples read": each example
/// of version 6 passes its direction's envelope of version 6, reads into
/// the runner's types and writes back equal, and `newest_release.json`
/// reads as a newest release naming its version; a `newest_release` that
/// names a field version 6 does not, names no version, or names one that
/// is no release's, fails version 6's envelope, as does any of them under
/// version 5 (design Decision 1 of
/// protocol-version-6-names-the-newest-release).
#[test]
fn version_6s_examples_read() {
    let (v5, v6) = (common::V5, common::V6);
    for (direction, validator) in [
        ("from-runner", v6.messages_from_runner()),
        ("to-runner", v6.messages_to_runner()),
    ] {
        for (name, value) in v6.examples(direction) {
            let what = format!("version 6's {direction}/{name}");
            passes(&validator, &value, &what);
            let written = match direction {
                "from-runner" => serde_json::to_value(
                    serde_json::from_value::<Message<FromRunner>>(value.clone())
                        .unwrap_or_else(|e| panic!("{what} does not read: {e}")),
                ),
                _ => serde_json::to_value(
                    serde_json::from_value::<Message<ToRunner>>(value.clone())
                        .unwrap_or_else(|e| panic!("{what} does not read: {e}")),
                ),
            }
            .unwrap();
            assert_eq!(written, value, "{what} written back differs");
        }
    }

    let newest = v6.example("to-runner", "newest_release.json");
    let message: Message<ToRunner> = serde_json::from_value(newest.clone()).unwrap();
    let ToRunner::NewestRelease(release) = message.body else {
        panic!("newest_release.json holds no newest_release");
    };
    assert_eq!(release.version, "0.32.0");

    let to_v6 = v6.messages_to_runner();
    let to_v5 = v5.messages_to_runner();
    let mut as_v5 = newest.clone();
    as_v5["protocol_version"] = Value::from(5);
    assert!(!to_v5.is_valid(&as_v5), "version 5 takes a newest_release");
    for body in [
        serde_json::json!({ "version": "0.32.0", "url": "https://example.com" }),
        serde_json::json!({}),
        serde_json::json!({ "version": "v0.32.0" }),
        serde_json::json!({ "version": "0.32" }),
        serde_json::json!({ "version": "0.32.0-rc.1" }),
        serde_json::json!({ "version": "00.32.0" }),
    ] {
        let mut failing = newest.clone();
        failing["body"]["newest_release"] = body.clone();
        assert!(!to_v6.is_valid(&failing), "version 6 takes {body}");
    }
}

/// The runner-protocol spec's "Version 7's examples read": each example
/// of version 7 passes its direction's envelope of version 7, reads into
/// the runner's types and writes back equal, and its hellos name each of
/// their repositories' statuses once; a hello of version 7 with no
/// `repository_statuses`, a status of a form version 7 does not name, a
/// status that names the clone's folder, and any hello of version 7 under
/// version 6 fail (design Decision 1 of
/// protocol-version-7-carries-the-binding).
#[test]
fn version_7s_examples_read() {
    let (v6, v7) = (common::V6, common::V7);
    for (direction, validator) in [
        ("from-runner", v7.messages_from_runner()),
        ("to-runner", v7.messages_to_runner()),
    ] {
        for (name, value) in v7.examples(direction) {
            let what = format!("version 7's {direction}/{name}");
            passes(&validator, &value, &what);
            let written = match direction {
                "from-runner" => serde_json::to_value(
                    serde_json::from_value::<Message<FromRunner>>(value.clone())
                        .unwrap_or_else(|e| panic!("{what} does not read: {e}")),
                ),
                _ => serde_json::to_value(
                    serde_json::from_value::<Message<ToRunner>>(value.clone())
                        .unwrap_or_else(|e| panic!("{what} does not read: {e}")),
                ),
            }
            .unwrap();
            assert_eq!(written, value, "{what} written back differs");
            if let Some(hello) = value.pointer("/body/hello") {
                let named: Vec<&String> = hello["repository_statuses"]
                    .as_object()
                    .unwrap_or_else(|| panic!("{what} names no repository_statuses"))
                    .keys()
                    .collect();
                let mut kept: Vec<&str> = hello["repositories"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .filter_map(Value::as_str)
                    .collect();
                kept.sort_unstable();
                assert_eq!(named, kept, "{what} names each repository's status once");
            }
        }
    }

    let from_v7 = v7.messages_from_runner();
    let from_v6 = v6.messages_from_runner();
    let hello = v7.example("from-runner", "hello.json");
    let mut as_v6 = hello.clone();
    as_v6["protocol_version"] = Value::from(6);
    assert!(
        !from_v6.is_valid(&as_v6),
        "version 6 takes repository_statuses"
    );
    let mut without = hello.clone();
    without["body"]["hello"]
        .as_object_mut()
        .unwrap()
        .remove("repository_statuses");
    assert!(
        !from_v7.is_valid(&without),
        "version 7 takes a hello with no statuses"
    );
    let app = "/body/hello/repository_statuses/example~1app";
    for (pointer, value) in [
        (app.to_string(), serde_json::json!({ "read": {} })),
        (
            app.to_string(),
            serde_json::json!({ "printed": {}, "missing": "both" }),
        ),
        (
            format!("{app}/printed/repository"),
            serde_json::json!("/data/repositories/example/app"),
        ),
        (
            format!("{app}/printed/binding/read/from"),
            serde_json::json!("remote"),
        ),
        (
            format!("{app}/printed/merge_queue/ready"),
            serde_json::json!("yes"),
        ),
        (format!("{app}/printed/kinds/0/added"), serde_json::json!(1)),
        (
            "/body/hello/repository_statuses/not an address".to_string(),
            serde_json::json!({ "missing": "words" }),
        ),
    ] {
        let mut failing = hello.clone();
        let (parent, member) = pointer.rsplit_once('/').unwrap();
        let member = member.replace("~1", "/");
        failing
            .pointer_mut(parent)
            .and_then(Value::as_object_mut)
            .unwrap_or_else(|| panic!("hello.json holds {parent}"))
            .insert(member.clone(), value.clone());
        assert!(
            !from_v7.is_valid(&failing),
            "version 7 takes {member} as {value} at {parent}"
        );
    }
}
