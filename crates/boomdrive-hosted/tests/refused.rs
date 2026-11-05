//! What version 1 refuses (design Decision 14): each case a valid example
//! altered, checked against its direction's envelope, refused, and named
//! in its assertion.

mod common;

use boomdrive_hosted::protocol::v1::{FromRunner, Message, ToRunner};
use common::{example, examples, from_runner, passes, refused, to_runner};
use serde_json::{json, Value};

/// The example `name` of `direction`, altered by `alter`.
fn altered(direction: &str, name: &str, alter: impl FnOnce(&mut Value)) -> Value {
    let mut value = example(direction, name);
    alter(&mut value);
    value
}

/// The runner-protocol spec's "A message with no id": every message
/// carries an id of its own, of 1 to 128 letters, digits and `._:/-`.
#[test]
fn a_message_with_no_id() {
    let validator = from_runner();
    let name = "hello.json";
    passes(&validator, &example("from-runner", name), name);
    let unnamed = altered("from-runner", name, |hello| {
        hello.as_object_mut().unwrap().remove("id");
    });
    refused(&validator, &unnamed, "a hello with no id");
    for (what, id) in [
        ("an empty id", "".to_string()),
        ("an id holding a space", "runner 1".to_string()),
        ("an id of 129 characters", "r".repeat(129)),
    ] {
        let wrong = altered("from-runner", name, |hello| hello["id"] = json!(id));
        refused(&validator, &wrong, &format!("a hello with {what}"));
    }
}

/// The runner-protocol spec's "A message of another version": every
/// message of version 1 names it.
#[test]
fn a_message_of_another_version() {
    let validator = from_runner();
    for version in [json!(2), json!(0), json!("1")] {
        let other = altered("from-runner", "hello.json", |hello| {
            hello["protocol_version"] = version.clone();
        });
        refused(
            &validator,
            &other,
            &format!("a hello naming protocol_version {version}"),
        );
    }
}

/// The runner-protocol spec's "A body of two kinds, or of none": a body
/// holds exactly one kind version 1 has.
#[test]
fn a_body_of_two_kinds_or_of_none() {
    let validator = from_runner();
    let report = example("from-runner", "report.json")["body"]["report"].clone();
    let two = altered("from-runner", "hello.json", |hello| {
        hello["body"]["report"] = report;
    });
    refused(&validator, &two, "a body holding both hello and report");
    let empty = altered("from-runner", "hello.json", |hello| {
        hello["body"] = json!({});
    });
    refused(&validator, &empty, "an empty body");
    let bodiless = altered("from-runner", "hello.json", |hello| {
        hello.as_object_mut().unwrap().remove("body");
    });
    refused(&validator, &bodiless, "a message with no body");
    let unknown = altered("from-runner", "hello.json", |hello| {
        let body = hello["body"].as_object_mut().unwrap();
        let hello = body.remove("hello").unwrap();
        body.insert("goodbye".to_string(), hello);
    });
    refused(
        &validator,
        &unknown,
        "a body of goodbye, a kind version 1 lacks",
    );
}

/// The runner-protocol spec's "A body of the other direction": each
/// direction's envelope takes its own kinds alone, so each example of a
/// kind one direction alone sends is refused as the other's.
#[test]
fn a_body_of_the_other_direction() {
    refused(
        &from_runner(),
        &example("to-runner", "pulled.json"),
        "a pulled as a runner's message",
    );
    refused(
        &to_runner(),
        &example("from-runner", "hello.json"),
        "a hello as a control plane's message",
    );
    let directions = [
        ("to-runner", from_runner(), "a runner's"),
        ("from-runner", to_runner(), "a control plane's"),
    ];
    for (direction, other, whose) in directions {
        for (name, value) in examples(direction) {
            if value["body"].get("message_refused").is_some() {
                continue;
            }
            refused(
                &other,
                &value,
                &format!("{direction}/{name} as {whose} message"),
            );
        }
    }
}

/// The runner-protocol spec's "Enrollment's kinds in the other direction":
/// an `enroll` is a runner's alone, and `enrolled` and `enroll_refused` a
/// control plane's alone, so each is refused as the other's.
#[test]
fn enrollments_kinds_in_the_other_direction() {
    let enroll = example("from-runner", "enroll.json");
    passes(&from_runner(), &enroll, "enroll.json");
    refused(
        &to_runner(),
        &enroll,
        "an enroll as a control plane's message",
    );
    for name in ["enrolled.json", "enroll_refused-a-lapsed-token.json"] {
        let answer = example("to-runner", name);
        passes(&to_runner(), &answer, name);
        refused(
            &from_runner(),
            &answer,
            &format!("{name} as a runner's message"),
        );
    }
}

/// The runner-protocol spec's "An enroll unsigned, or holding a token": an
/// `enroll` holds the key's signature, the key is 32 bytes and the
/// signature 64, and no `enroll` holds a token, which is the request's
/// bearer (core.4, core.31). The runner's types read no token either.
#[test]
fn an_enroll_unsigned_or_holding_a_token() {
    let validator = from_runner();
    let name = "enroll.json";
    passes(&validator, &example("from-runner", name), name);
    let enrolling = |alter: &dyn Fn(&mut Value)| {
        altered("from-runner", name, |message| {
            alter(&mut message["body"]["enroll"]);
        })
    };
    let unsigned = enrolling(&|enroll| {
        enroll.as_object_mut().unwrap().remove("signature");
    });
    refused(&validator, &unsigned, "an enroll with no signature");
    let key = example("from-runner", name)["body"]["enroll"]["key"]
        .as_str()
        .unwrap()
        .to_string();
    for (what, wrong) in [
        ("31 bytes", key[..42].to_string()),
        ("33 bytes", format!("{key}AA")[..44].to_string()),
        ("64 bytes", "A".repeat(86)),
        ("not base64url", format!("{}+", &key[..42])),
    ] {
        let short = enrolling(&|enroll| enroll["key"] = json!(wrong));
        refused(
            &validator,
            &short,
            &format!("an enroll whose key is {what}"),
        );
    }
    let short = enrolling(&|enroll| enroll["signature"] = json!(key));
    refused(
        &validator,
        &short,
        "an enroll whose signature is 32 bytes, not 64",
    );
    let nameless = enrolling(&|enroll| enroll["machine"] = json!(""));
    refused(&validator, &nameless, "an enroll whose machine is empty");
    let holding = enrolling(&|enroll| enroll["token"] = json!("example-enrollment-token"));
    refused(&validator, &holding, "an enroll holding a token");
    assert!(
        serde_json::from_value::<Message<FromRunner>>(holding).is_err(),
        "the runner's types read an enroll holding a token"
    );
}

/// The runner-protocol spec's "A field the version does not name": every
/// object version 1 names is closed, at every depth. The runner's types
/// read no such field of version 1's own objects either.
#[test]
fn a_field_the_version_does_not_name() {
    let from = from_runner();
    let to = to_runner();
    let enveloped = altered("from-runner", "hello.json", |hello| {
        hello["sent_at"] = json!("2026-11-01T10:00:00Z");
    });
    refused(&from, &enveloped, "a field the envelope does not name");
    assert!(
        serde_json::from_value::<Message<FromRunner>>(enveloped).is_err(),
        "the runner's types read a field the envelope does not name"
    );
    let launched = altered("to-runner", "pulled.json", |pulled| {
        pulled["body"]["pulled"]["wants"][0]["offer"]["launch"]["kinds_file"] = json!("kinds.toml");
    });
    refused(&to, &launched, "a field a launch does not name");
    let leased = altered("to-runner", "pulled.json", |pulled| {
        pulled["body"]["pulled"]["wants"][0]["offer"]["lease"]["renewed"] =
            json!("2026-11-01T10:00:00Z");
    });
    refused(&to, &leased, "a field a lease does not name");
    let name = "report-a-live-agent-with-its-pane.json";
    passes(&from, &example("from-runner", name), name);
    let copied = altered("from-runner", name, |report| {
        copy(report)["added_after"] = json!(true);
    });
    refused(
        &from,
        &copied,
        "a field a place's status copy does not name",
    );
    let shared = altered("to-runner", "terminal_open.json", |open| {
        open["body"]["terminal_open"]["share"]["nonce"] = json!("AAAA");
    });
    refused(&to, &shared, "a field a key share does not name");
    assert!(
        serde_json::from_value::<Message<ToRunner>>(shared).is_err(),
        "the runner's types read a field a key share does not name"
    );
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

/// The runner-protocol spec's "A frame with no offset": every frame
/// carries its place in its direction's stream.
#[test]
fn a_frame_with_no_offset() {
    let name = "terminal_output.json";
    let unplaced = altered("from-runner", name, |output| {
        let frame = output["body"]["terminal_output"].as_object_mut().unwrap();
        frame.remove("offset");
    });
    refused(
        &from_runner(),
        &unplaced,
        "a terminal_output with no offset",
    );
}

/// The runner-protocol spec's "A frame in the clear": a frame carries its
/// bytes only sealed, in base64url (core.30).
#[test]
fn a_frame_in_the_clear() {
    let frames = [
        (
            from_runner(),
            "from-runner",
            "terminal_output.json",
            "terminal_output",
        ),
        (
            to_runner(),
            "to-runner",
            "terminal_input.json",
            "terminal_input",
        ),
    ];
    for (validator, direction, name, kind) in frames {
        passes(&validator, &example(direction, name), name);
        let clear = altered(direction, name, |message| {
            let frame = message["body"][kind].as_object_mut().unwrap();
            frame.remove("sealed");
            frame.insert("text".to_string(), json!("cargo test\r\n"));
        });
        refused(
            &validator,
            &clear,
            &format!("a {kind} carrying its bytes as text"),
        );
        let beside = altered(direction, name, |message| {
            message["body"][kind]["bytes"] = json!("Y2FyZ28gdGVzdA");
        });
        refused(
            &validator,
            &beside,
            &format!("a {kind} carrying bytes beside its sealed"),
        );
        for unsealed in ["cargo test", "Y2FyZ28gdGVzdA==", "Y2FyZ28+dGVzdA", "Y"] {
            let wrong = altered(direction, name, |message| {
                message["body"][kind]["sealed"] = json!(unsealed);
            });
            refused(
                &validator,
                &wrong,
                &format!("a {kind} whose sealed is {unsealed:?}, not base64url"),
            );
        }
    }
}

/// The runner-protocol spec's "An unsigned share": a share holds its
/// signature and its signing key's certificate (core.30).
#[test]
fn an_unsigned_share() {
    let validator = to_runner();
    let name = "terminal_open.json";
    passes(&validator, &example("to-runner", name), name);
    for field in ["signature", "certificate"] {
        let unsigned = altered("to-runner", name, |open| {
            let share = open["body"]["terminal_open"]["share"]
                .as_object_mut()
                .unwrap();
            share.remove(field);
        });
        refused(
            &validator,
            &unsigned,
            &format!("a terminal_open whose share has no {field}"),
        );
    }
}
