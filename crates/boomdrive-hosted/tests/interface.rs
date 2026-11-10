//! Version 2 carries the interface's shapes as `boomdrive-control`'s serde
//! types write them (the runner-protocol spec's "The messages carry the
//! interface's shapes"; design Decision 6 of
//! the-runner-dials-the-hosted-control-plane): each value below, built as
//! the interface's own tests build theirs, with digests and commits of
//! full length, passes its definition, every act sort version 2 names
//! among them. A land and a confirm apart: version 1 names neither, and a
//! release has spoken it, so a want whose act is a land or a confirm fails
//! its check (the runner-protocol spec's "Version 1 carries no land" and
//! "Version 1 carries no confirm"; core.42, core.53, core.56).

mod common;

use boomdrive_control::*;
use common::{definition, passes, refused, validator, V1};
use serde::Serialize;
use std::collections::BTreeMap;

/// `value`, as the interface's types write it, passes the definition
/// `name` of `file`.
fn passes_as<T: Serialize>(file: &str, name: &str, value: &T) {
    let json = serde_json::to_value(value).unwrap();
    passes(
        &validator(&definition(file, name)),
        &json,
        &json.to_string(),
    );
}

fn at(text: &str) -> At {
    At::parse(text).unwrap()
}

fn lease() -> Lease {
    Lease {
        number: 3,
        expires: Some(at("2026-11-01T11:00:00Z")),
    }
}

fn launch() -> Launch {
    Launch {
        repository: "example/app".to_string(),
        kind: "example/kinds/plan@1".to_string(),
        name: "plan-the-week".to_string(),
        base: Some("bolt/x".to_string()),
        brief: Some("Plan the week.\n\nEvery unit.\n".to_string()),
        stream: Some("roles".to_string()),
        priority: Some(-2),
        intent: Some("plan".to_string()),
        recipients: vec!["role/planner".to_string(), "operator".to_string()],
        holds: vec![Hold {
            stage: "construct".to_string(),
            words: "wait for the brief".to_string(),
        }],
        hold_submission: true,
        after: vec![Id::new("offer-1"), Id::new("offer-2")],
        agent_settings: vec![AgentSetting {
            field: "model".to_string(),
            value: "opus".to_string(),
        }],
    }
}

fn no_more_than_needed() -> Launch {
    Launch {
        base: None,
        brief: None,
        stream: None,
        priority: None,
        intent: None,
        recipients: vec![],
        holds: vec![],
        hold_submission: false,
        after: vec![],
        agent_settings: vec![],
        ..launch()
    }
}

/// The runner-protocol spec's "A land is no act of version 1".
#[test]
fn a_land_is_no_act_of_version_1() {
    let land = Want::Act(Act {
        id: Id::new("act-1"),
        launch: Id::new("offer-1"),
        lease: 3,
        what: ActSort::Land,
    });
    let json = serde_json::to_value(&land).unwrap();
    refused(
        &V1.validator(&V1.definition("wants.schema.json", "want")),
        &json,
        "a land",
    );
}

/// The runner-protocol spec's "A confirm is no act of version 1".
#[test]
fn a_confirm_is_no_act_of_version_1() {
    let confirm = Want::Act(Act {
        id: Id::new("act-1"),
        launch: Id::new("offer-1"),
        lease: 3,
        what: ActSort::Confirm {
            recipient: "scratch/planner".to_string(),
            deliveries: vec!["change--a/0a1b2c3d/stage/code/1/delivered".to_string()],
        },
    });
    let json = serde_json::to_value(&confirm).unwrap();
    refused(
        &V1.validator(&V1.definition("wants.schema.json", "want")),
        &json,
        "a confirm",
    );
}

/// One value of each want sort but a preview, which no version names
/// (`a_preview_is_no_want_of_version_1`), and of each act sort version 2
/// names, a land (`a_land_is_no_act_of_version_1`) and a confirm
/// (`a_confirm_is_no_act_of_version_1`) apart, and each command sort, and
/// each other value of the interface, passes its definition in version 2.
#[test]
fn the_interfaces_values_pass() {
    passes_as("shared.schema.json", "id", &Id::new("example/12"));
    passes_as("shared.schema.json", "time", &at("2026-11-01T10:00:00Z"));
    passes_as("shared.schema.json", "lease", &lease());
    passes_as(
        "shared.schema.json",
        "lease",
        &Lease {
            number: 1,
            expires: None,
        },
    );
    passes_as("wants.schema.json", "launch", &launch());
    passes_as("wants.schema.json", "launch", &no_more_than_needed());

    let mut wants = vec![
        Want::Offer(Offer {
            id: Id::new("offer-1"),
            lease: lease(),
            launch: launch(),
        }),
        Want::Offer(Offer {
            id: Id::new("offer-2"),
            lease: lease(),
            launch: no_more_than_needed(),
        }),
        Want::Inbox(Inbox {
            id: Id::new("inbox-1"),
            repository: "example/tools".to_string(),
            branch: "inbox".to_string(),
        }),
    ];
    let acts = [
        ActSort::Retry {
            stage: Some("code".to_string()),
            words: Some("again".to_string()),
            submission: true,
        },
        ActSort::Retry {
            stage: None,
            words: None,
            submission: false,
        },
        ActSort::Drop,
        ActSort::Priority(5),
        ActSort::Done,
    ];
    wants.extend(acts.into_iter().map(|what| {
        Want::Act(Act {
            id: Id::new("act-1"),
            launch: Id::new("offer-1"),
            lease: 3,
            what,
        })
    }));
    let commands = [
        CommandSort::Branch {
            repository: "example/app".to_string(),
            branch: "bolt/x".to_string(),
            from: "main".to_string(),
        },
        CommandSort::Other {
            tool: "boom".to_string(),
            words: "accept".to_string(),
        },
    ];
    wants.extend(commands.into_iter().map(|what| {
        Want::Command(Command {
            id: Id::new("command-1"),
            lease: lease(),
            what,
        })
    }));
    for want in &wants {
        passes_as("wants.schema.json", "want", want);
    }

    passes_as(
        "hello.schema.json",
        "tool_version",
        &ToolVersion::Printed("flume 0.4.1".to_string()),
    );
    passes_as(
        "hello.schema.json",
        "tool_version",
        &ToolVersion::Missing("not found".to_string()),
    );
    passes_as(
        "hello.schema.json",
        "capacity",
        &Capacity::Missing("flume is not on PATH".to_string()),
    );

    passes_as(
        "report.schema.json",
        "answer",
        &Answer {
            outcome: Outcome::Launched,
            place_dir: Some("/home/runner/places/plan-the-week".to_string()),
            words: None,
            at: at("2026-11-01T10:00:00Z"),
        },
    );
    for outcome in [Outcome::Done, Outcome::Refused] {
        passes_as(
            "report.schema.json",
            "acted",
            &Acted {
                launch: Id::new("offer-1"),
                lease: 3,
                outcome,
                words: Some("no such stage".to_string()),
                at: at("2026-11-01T10:02:00Z"),
            },
        );
        passes_as(
            "report.schema.json",
            "commanded",
            &Commanded {
                lease: lease(),
                outcome,
                text: Some(r#"{"branch":"bolt/x","created":true,"from":"main"}"#.to_string()),
                words: None,
                origin_commit: Some("0123456789abcdef0123456789abcdef01234567".to_string()),
                at: at("2026-11-01T10:03:00Z"),
            },
        );
    }
    let digest = format!("sha256:{}", "00ff".repeat(16));
    passes_as(
        "report.schema.json",
        "inbox_carried",
        &InboxCarried {
            tip: Some("89abcdef0123456789abcdef0123456789abcdef".to_string()),
            files: BTreeMap::from([
                (
                    "inbox/items/1/item.rec".to_string(),
                    InboxFile::Text("Item: 1\n".to_string()),
                ),
                (
                    "inbox/items/1/brief.md".to_string(),
                    InboxFile::Digest(digest),
                ),
            ]),
            words: None,
            at: at("2026-11-01T10:04:00Z"),
        },
    );
    passes_as(
        "report.schema.json",
        "inbox_carried",
        &InboxCarried {
            tip: None,
            files: BTreeMap::new(),
            words: Some("no branch inbox on origin".to_string()),
            at: at("2026-11-01T10:04:00Z"),
        },
    );
    let report = Report {
        acts: BTreeMap::from([(
            Id::new("act-1"),
            Acted {
                launch: Id::new("offer-1"),
                lease: 3,
                outcome: Outcome::Done,
                words: None,
                at: at("2026-11-01T10:02:00Z"),
            },
        )]),
        ..Report::default()
    };
    passes_as("report.schema.json", "report", &report);
    passes_as("report.schema.json", "report", &Report::default());
}

/// A preview, as the interface's types write it, fails version 1's
/// `want`: version 1 names no preview, and a release has spoken it
/// (the runner-protocol spec's "A preview is no want of version 1";
/// core.42).
#[test]
fn a_preview_is_no_want_of_version_1() {
    for done in [false, true] {
        let preview = Want::Preview(Preview {
            id: Id::new("preview-1"),
            lease: lease(),
            repository: "example/app".to_string(),
            branch: "main".to_string(),
            revision: "0123456789abcdef0123456789abcdef01234567".to_string(),
            done,
        });
        let json = serde_json::to_value(&preview).unwrap();
        refused(
            &V1.validator(&V1.definition("wants.schema.json", "want")),
            &json,
            "a preview",
        );
    }
}
