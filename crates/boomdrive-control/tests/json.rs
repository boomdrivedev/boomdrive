//! Every type that passes over the interface round-trips as JSON (design
//! Decision 2), so the runner's report file and a hosted control plane's
//! messages carry each value unchanged. A terminal, and so a pull's
//! answer, holds a link, which is a channel and no value.

use boomdrive_control::*;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::BTreeMap;
use std::fmt::Debug;

/// `value` read back from its JSON, and that JSON written again, equal
/// what was written.
fn round_trip<T: Serialize + DeserializeOwned + PartialEq + Debug>(value: &T) {
    let text = serde_json::to_string(value).unwrap();
    let back: T = serde_json::from_str(&text).unwrap();
    assert_eq!(&back, value, "{text}");
    assert_eq!(serde_json::to_string(&back).unwrap(), text);
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
        repository: "example/crew".to_string(),
        kind: "crew/crew-planner".to_string(),
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
        after: vec![Id::new("home/1"), Id::new("home/2")],
        agent_settings: vec![AgentSetting {
            field: "model".to_string(),
            value: "opus".to_string(),
        }],
    }
}

fn carried(text: &str) -> Carried {
    Carried {
        text: text.to_string(),
        at: at("2026-11-01T10:01:00Z"),
        listed: false,
    }
}

#[test]
fn an_id() {
    round_trip(&Id::new("home/12"));
    assert_eq!(
        serde_json::to_string(&Id::new("home/12")).unwrap(),
        r#""home/12""#
    );
}

#[test]
fn a_time() {
    round_trip(&at("2026-11-01T10:00:00Z"));
    assert_eq!(
        serde_json::to_string(&at("2026-11-01T10:00:00Z")).unwrap(),
        r#""2026-11-01T10:00:00Z""#
    );
    assert!(serde_json::from_str::<At>(r#""soon""#).is_err());
}

#[test]
fn a_lease() {
    round_trip(&lease());
    round_trip(&Lease {
        number: 1,
        expires: None,
    });
}

#[test]
fn a_hello() {
    round_trip(&Hello {
        interface_version: INTERFACE_VERSION,
        runner_version: "boomdrive 0.0.0".to_string(),
        tool_versions: BTreeMap::from([
            (
                "flume".to_string(),
                ToolVersion::Printed("flume 0.4.1".to_string()),
            ),
            (
                "herdr".to_string(),
                ToolVersion::Missing("not found".to_string()),
            ),
        ]),
        repositories: vec!["example/crew".to_string()],
        draining: vec!["example/kit".to_string()],
        capacity: Capacity::Printed(HostPart {
            running: "2".to_string(),
            bound: "4".to_string(),
            room: "null".to_string(),
            streams: r#"[{"name":"default","room":2,"queue":[]}]"#.to_string(),
        }),
        leases: BTreeMap::from([(Id::new("home/1"), 1)]),
    });
    round_trip(&Capacity::Missing("flume is not on PATH".to_string()));
}

#[test]
fn a_launch() {
    round_trip(&launch());
    round_trip(&Launch {
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
    });
}

#[test]
fn an_offer() {
    round_trip(&Want::Offer(Offer {
        id: Id::new("home/1"),
        lease: lease(),
        launch: launch(),
    }));
}

#[test]
fn an_act_of_each_sort() {
    let sorts = [
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
        ActSort::Land,
        confirm(),
        ActSort::Drop,
        ActSort::Priority(5),
        ActSort::Done,
    ];
    for what in sorts {
        round_trip(&Want::Act(Act {
            id: Id::new("home/2"),
            launch: Id::new("home/1"),
            lease: 3,
            what,
        }));
    }
    // A land is the word `land`, holding nothing more (core.53).
    let land = Act {
        id: Id::new("home/2"),
        launch: Id::new("home/1"),
        lease: 3,
        what: ActSort::Land,
    };
    assert_eq!(
        serde_json::to_value(&land).unwrap(),
        serde_json::json!({"id": "home/2", "launch": "home/1", "lease": 3, "what": "land"}),
    );
    // A confirm holds its recipient and its keys, in the order it was
    // given them (the control-plane-interface spec's "A confirm's form";
    // core.56).
    let confirm = Act {
        what: confirm(),
        ..land
    };
    assert_eq!(
        serde_json::to_value(&confirm).unwrap(),
        serde_json::json!({
            "id": "home/2",
            "launch": "home/1",
            "lease": 3,
            "what": {"confirm": {
                "recipient": "scratch/planner",
                "deliveries": [
                    "change--a/0a1b2c3d/stage/code/1/delivered",
                    "change--a/0a1b2c3d/submission/1/landed",
                ],
            }},
        }),
    );
}

/// A confirm of two ends of one place, as `scratch/planner`.
fn confirm() -> ActSort {
    ActSort::Confirm {
        recipient: "scratch/planner".to_string(),
        deliveries: vec![
            "change--a/0a1b2c3d/stage/code/1/delivered".to_string(),
            "change--a/0a1b2c3d/submission/1/landed".to_string(),
        ],
    }
}

#[test]
fn a_command_of_each_sort() {
    let sorts = [
        CommandSort::Branch {
            repository: "agentplot/flume".to_string(),
            branch: "bolt/x".to_string(),
            from: "main".to_string(),
        },
        CommandSort::Other {
            tool: "boom".to_string(),
            words: "accept".to_string(),
        },
    ];
    for what in sorts {
        round_trip(&Want::Command(Command {
            id: Id::new("c1"),
            lease: lease(),
            what,
        }));
    }
}

#[test]
fn an_inbox() {
    round_trip(&Want::Inbox(Inbox {
        id: Id::new("i1"),
        repository: "agentplot/flume".to_string(),
        branch: "inbox".to_string(),
    }));
}

#[test]
fn a_preview_of_each_kind() {
    let up = Want::Preview(Preview {
        id: Id::new("home/30"),
        lease: lease(),
        repository: "agentplot/flume".to_string(),
        branch: "main".to_string(),
        revision: "0123456789abcdef0123456789abcdef01234567".to_string(),
        done: false,
    });
    round_trip(&up);
    let written = serde_json::to_value(&up).unwrap();
    assert_eq!(written["preview"]["done"], serde_json::json!(false));
    let mut fields: Vec<&String> = written["preview"].as_object().unwrap().keys().collect();
    fields.sort();
    assert_eq!(
        fields,
        ["branch", "done", "id", "lease", "repository", "revision"]
    );
    let Want::Preview(preview) = up else {
        unreachable!()
    };
    let done = Want::Preview(Preview {
        id: Id::new("home/31"),
        done: true,
        ..preview
    });
    round_trip(&done);
    assert_eq!(done.id(), &Id::new("home/31"));
}

#[test]
fn a_report() {
    let launched = Launched {
        lease: lease(),
        repository: Some("example/crew".to_string()),
        answer: Answer {
            outcome: Outcome::Launched,
            place_dir: Some("/places/plan-the-week".to_string()),
            words: None,
            at: at("2026-11-01T10:00:00Z"),
        },
        status: Some(carried(r#"{"status":{"places":[{"phase":"live"}]}}"#)),
        entry: Some(carried(r#"{"id":7,"state":"landed"}"#)),
    };
    let held = Launched {
        answer: Answer {
            outcome: Outcome::Held,
            place_dir: None,
            words: Some("waits on home/1, launched at a place not landed".to_string()),
            at: at("2026-11-01T10:00:00Z"),
        },
        repository: None,
        status: None,
        entry: None,
        ..launched.clone()
    };
    let report = Report {
        launches: BTreeMap::from([
            (Id::new("home/1"), launched.clone()),
            (Id::new("home/2"), held),
        ]),
        acts: BTreeMap::from([(
            Id::new("home/3"),
            Acted {
                launch: Id::new("home/1"),
                lease: 3,
                outcome: Outcome::Refused,
                words: Some("no such stage".to_string()),
                at: at("2026-11-01T10:02:00Z"),
            },
        )]),
        commands: BTreeMap::from([(
            Id::new("c1"),
            Commanded {
                lease: lease(),
                outcome: Outcome::Done,
                text: Some(r#"{"branch":"bolt/x","created":true,"from":"main"}"#.to_string()),
                words: None,
                origin_commit: Some("0123456789abcdef0123456789abcdef01234567".to_string()),
                at: at("2026-11-01T10:03:00Z"),
            },
        )]),
        inboxes: BTreeMap::from([(
            Id::new("i1"),
            InboxCarried {
                tip: Some("89abcdef0123456789abcdef0123456789abcdef".to_string()),
                files: BTreeMap::from([
                    (
                        "inbox/items/1/item.rec".to_string(),
                        InboxFile::Text("Item: 1\n".to_string()),
                    ),
                    (
                        "inbox/items/1/brief.md".to_string(),
                        InboxFile::Digest("sha256:00ff".to_string()),
                    ),
                ]),
                words: None,
                at: at("2026-11-01T10:04:00Z"),
            },
        )]),
        previews: BTreeMap::new(),
    };
    round_trip(&launched);
    round_trip(&report);
    round_trip(&Report::default());
    assert_eq!(
        serde_json::to_value(Outcome::Held).unwrap(),
        serde_json::json!("held")
    );
}

#[test]
fn a_report_from_before_reads() {
    let report = r#"{"launches":{"a":{"lease":{"number":1,"expires":null},"answer":{"outcome":"launched","place_dir":"/places/a","words":null,"at":"2026-11-01T10:00:00Z"},"status":null,"entry":null}},"acts":{},"commands":{},"inboxes":{}}"#;
    let read: Report = serde_json::from_str(report).expect("a report of before reads");
    assert_eq!(read.launches[&Id::new("a")].repository, None);
    assert!(read.previews.is_empty());
    assert_eq!(serde_json::to_string(&read).unwrap(), report);
    round_trip(&read);
    let hello = r#"{"interface_version":1,"runner_version":"boomdrive 0.0.0","tool_versions":{},"repositories":["example/app"],"capacity":{"missing":"flume is not on PATH"},"leases":{}}"#;
    let read: Hello = serde_json::from_str(hello).expect("a hello of before reads");
    assert!(read.draining.is_empty());
    round_trip(&read);
}

fn previewed(lease: Lease, outcome: Outcome) -> Previewed {
    Previewed {
        lease,
        repository: "agentplot/flume".to_string(),
        revision: "0123456789abcdef0123456789abcdef01234567".to_string(),
        done: false,
        outcome,
        text: None,
        words: None,
        status: Some(Carried {
            text: r#"{"name":"preview/0123456","state":"waiting"}"#.to_string(),
            at: at("2026-11-01T10:05:00Z"),
            listed: true,
        }),
        at: at("2026-11-01T10:05:00Z"),
    }
}

#[test]
fn a_report_holding_a_preview() {
    let report = Report {
        previews: BTreeMap::from([(Id::new("home/30"), previewed(lease(), Outcome::Held))]),
        ..Report::default()
    };
    round_trip(&report);
    let written = serde_json::to_value(&report).unwrap();
    assert_eq!(written["previews"]["home/30"]["outcome"], "held");
    assert_eq!(
        written["previews"]["home/30"]["status"]["listed"],
        serde_json::json!(true)
    );
    assert!(serde_json::to_value(Report::default())
        .unwrap()
        .get("previews")
        .is_none());
}

#[test]
fn a_previews_lease_among_the_reports_leases() {
    let lease_of = |number| Lease {
        number,
        expires: None,
    };
    let report = Report {
        launches: BTreeMap::from([(
            Id::new("a"),
            Launched {
                lease: lease_of(3),
                repository: None,
                answer: Answer {
                    outcome: Outcome::Launched,
                    place_dir: Some("/places/a".to_string()),
                    words: None,
                    at: at("2026-11-01T10:00:00Z"),
                },
                status: None,
                entry: None,
            },
        )]),
        previews: BTreeMap::from([(Id::new("p"), previewed(lease_of(5), Outcome::Done))]),
        ..Report::default()
    };
    assert_eq!(
        report.leases(),
        BTreeMap::from([(Id::new("a"), 3), (Id::new("p"), 5)])
    );
}

#[test]
fn a_reports_answer() {
    round_trip(&Reported {
        let_go: vec![Id::new("home/1")],
        refused: BTreeMap::from([(Id::new("home/2"), "lease 3 is replaced by 4".to_string())]),
    });
    round_trip(&Reported::default());
}
