//! The protocol carries the interface's shapes as `boomdrive-control`'s
//! serde types write them (the runner-protocol spec's "The messages carry
//! the interface's shapes"; design Decision 6 of
//! the-runner-dials-the-hosted-control-plane): each value below, built as
//! the interface's own tests build theirs, with digests and commits of
//! full length, passes its definition in version 4, the version the runner
//! speaks. Versions 3 and 4 carry every sort the interface names, a hold,
//! a preview, a land and a confirm among them (the runner-protocol spec's "Every sort the interface names", "Version
//! 3 carries a control plane's hold", "Version 3 carries a control
//! plane's confirm", "Version 3 carries a land" and "Version 3 carries a
//! preview and its done"; records 0052 and 0053). Version 2 carries every sort but
//! the hold, since `v0.9.0` spoke it before the interface named one, and
//! version 1 names no land, no confirm and no preview; a release has
//! spoken each, so a want whose act is a land or a confirm, or a preview,
//! fails its check of version 1 (the runner-protocol spec's "Version 1
//! carries no land" and "Version 1 carries no confirm"; core.42, core.53,
//! core.55, core.56), and a want whose act is a hold fails the check of
//! each (the runner-protocol spec's "A hold is no act of version 1 or 2";
//! core.42, core.59). Version 5 carries a report's unleased places, and
//! no observing hello or observed report (record 0054).

mod common;

use boomdrive_control::*;
use common::{definition, passes, refused, validator, Version, V1, V2, V3, V4, V5};
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

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

/// The runner-protocol spec's "A hold is no act of version 1 or 2": each
/// was spoken by a release before the interface named a hold, and never
/// changes (core.42, core.59).
#[test]
fn a_hold_is_no_act_of_version_1_or_2() {
    let hold = Want::Act(Act {
        id: Id::new("act-1"),
        launch: Id::new("offer-1"),
        lease: 3,
        what: ActSort::Hold {
            words: "the user sends it".to_string(),
            bound_interval: 28800,
        },
    });
    let json = serde_json::to_value(&hold).unwrap();
    refused(
        &V1.validator(&V1.definition("wants.schema.json", "want")),
        &json,
        "a hold, in version 1",
    );
    refused(
        &V2.validator(&V2.definition("wants.schema.json", "want")),
        &json,
        "a hold, in version 2",
    );
}

/// A want whose act on `offer-1` is a hold under `words` for
/// `bound_interval` seconds, as the interface writes it.
fn hold_of(words: &str, bound_interval: u64) -> Value {
    act_of(ActSort::Hold {
        words: words.to_string(),
        bound_interval,
    })
}

/// The runner-protocol spec's "A hold is an act of version 3" and "A
/// hold's bound past what JavaScript reads exactly": a hold of words and
/// a bound, one of no words and one of bound 0 pass version 3's `want`,
/// which bounds neither further, so a hold the runner refuses reaches it
/// as an act (core.59); a hold of bound 2^53 fails it, as every integer of
/// the protocol past what JavaScript reads exactly does; and
/// `pulled-with-a-hold.json` passes version 3's `to-runner.schema.json`
/// (design Decisions 1 and 2; record 0052).
#[test]
fn a_hold_is_an_act_of_version_3() {
    let want = V3.validator(&V3.definition("wants.schema.json", "want"));
    passes(
        &want,
        &hold_of("the user sends it on the landings card, row 3", 28800),
        "a hold",
    );
    passes(&want, &hold_of("", 28800), "a hold of no words");
    passes(
        &want,
        &hold_of("the user sends it on the landings card, row 3", 0),
        "a hold of bound 0",
    );
    refused(
        &want,
        &hold_of("the user sends it", 1 << 53),
        "a hold of bound 2^53",
    );
    let example = V3.example("to-runner", "pulled-with-a-hold.json");
    passes(
        &V3.messages_to_runner(),
        &example,
        "pulled-with-a-hold.json",
    );
    assert_eq!(
        example["body"]["pulled"]["wants"][1]["act"]["what"],
        hold_of("the user sends it on the landings card, row 3", 28800)["act"]["what"],
        "pulled-with-a-hold.json holds a hold"
    );
}

/// `value` with `member` set to `to`, at the object `pointer` names.
fn with(value: &Value, pointer: &str, member: &str, to: Value) -> Value {
    let mut changed = value.clone();
    changed
        .pointer_mut(pointer)
        .and_then(Value::as_object_mut)
        .unwrap_or_else(|| panic!("no object at {pointer}"))
        .insert(member.to_string(), to);
    changed
}

/// The runner-protocol spec's "A place started elsewhere", "An unleased
/// place that names a lease" and "No observing connection in version 5":
/// version 5's report of a place started elsewhere passes, with a lease
/// added to its place or with no `unleased` it fails, and version 4's
/// observing hello and observed report, each naming version 5, fail
/// version 5's envelope (record 0054; core.10, core.42).
#[test]
fn an_unleased_place_is_of_version_5_and_no_observing_form() {
    let from_runner = V5.messages_from_runner();
    let name = "report-a-place-started-elsewhere.json";
    let elsewhere = V5.example("from-runner", name);
    passes(&from_runner, &elsewhere, name);
    let at = "/body/report/unleased/~1home~1runner~1places~1note--by-hand";
    refused(
        &from_runner,
        &with(
            &elsewhere,
            at,
            "lease",
            json!({ "number": 1, "expires": null }),
        ),
        "an unleased place with a lease",
    );
    let mut none = elsewhere.clone();
    none["body"]["report"]
        .as_object_mut()
        .unwrap()
        .remove("unleased");
    refused(&from_runner, &none, "a report with no unleased");
    for name in ["hello-observing.json", "report-observed.json"] {
        let mut observing = V4.example("from-runner", name);
        observing["protocol_version"] = json!(5);
        refused(&from_runner, &observing, &format!("{name}, in version 5"));
    }
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

/// One value of each want sort, a preview not done and one done among
/// them, and of each act sort, a land, a confirm and a hold among them,
/// and each command sort, and each other value of the interface, passes
/// its definition in version 4 (the runner-protocol spec's "The
/// interface's values pass").
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
        preview_of("0123456789abcdef0123456789abcdef01234567", false),
        preview_of("0123456789abcdef0123456789abcdef01234567", true),
    ];
    let acts = [
        ActSort::Land,
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
        confirm_of(&["change--a/0a1b2c3d/stage/code/1/delivered"]),
        ActSort::Hold {
            words: "the user sends it".to_string(),
            bound_interval: 28800,
        },
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
    // Version 5 writes every part of a report, its `previews` among them,
    // which the interface's file leaves out where it holds none (design
    // Decision 3 of the-hosted-protocol-carries-land-and-preview), and its
    // `unleased`, which the interface's file never holds (record 0054).
    let report_definition = validator(&definition("report.schema.json", "report"));
    for report in [report, Report::default()] {
        let mut json = serde_json::to_value(&report).unwrap();
        assert!(json.get("previews").is_none(), "{json}");
        assert!(json.get("unleased").is_none(), "{json}");
        json["previews"] = json!({});
        json["unleased"] = json!({});
        passes(&report_definition, &json, &json.to_string());
    }
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

/// A confirm as `scratch/planner` of `keys`, as the interface writes it.
fn confirm_of(keys: &[&str]) -> ActSort {
    ActSort::Confirm {
        recipient: "scratch/planner".to_string(),
        deliveries: keys.iter().map(|key| key.to_string()).collect(),
    }
}

/// A want whose act on `offer-1` is `what`, as the interface writes it.
fn act_of(what: ActSort) -> Value {
    serde_json::to_value(Want::Act(Act {
        id: Id::new("act-1"),
        launch: Id::new("offer-1"),
        lease: 3,
        what,
    }))
    .unwrap()
}

/// A confirm of two keys, and one of no key, pass `version`'s `want`,
/// which bounds neither, so a confirm the runner refuses reaches it as an
/// act (core.56); a `pulled` holding the first, and
/// `pulled-with-a-confirm.json`, pass its `to-runner.schema.json`; and a
/// confirm holding a field the version does not name fails its `want`.
fn a_confirm_is_an_act_of(version: Version) {
    let n = version.number;
    let want = version.validator(&version.definition("wants.schema.json", "want"));
    let to_runner = version.messages_to_runner();
    let two = act_of(confirm_of(&[
        "change--a/0a1b2c3d/stage/code/1/delivered",
        "change--a/0a1b2c3d/submission/1/landed",
    ]));
    passes(&want, &two, "a confirm of two keys");
    passes(&want, &act_of(confirm_of(&[])), "a confirm of no key");
    let mut pulled = version.example("to-runner", "pulled.json");
    pulled["body"]["pulled"]["wants"] = json!([two]);
    passes(&to_runner, &pulled, "a pulled holding a confirm");
    passes(
        &to_runner,
        &version.example("to-runner", "pulled-with-a-confirm.json"),
        "pulled-with-a-confirm.json",
    );
    let mut unnamed = act_of(confirm_of(&["k"]));
    unnamed["act"]["what"]["confirm"]["stage"] = json!("code");
    refused(
        &want,
        &unnamed,
        &format!("a confirm holding a field version {n} does not name"),
    );
    assert!(version
        .address("wants.schema.json")
        .contains(&format!("/v{n}/")));
}

/// The runner-protocol spec's "A confirm is an act of version 2".
#[test]
fn a_confirm_is_an_act_of_version_2() {
    a_confirm_is_an_act_of(V2);
}

/// The runner-protocol spec's "A confirm is an act of version 3".
#[test]
fn a_confirm_is_an_act_of_version_3() {
    a_confirm_is_an_act_of(V3);
}

/// A land, as the interface writes it, passes `version`'s `want`, its
/// `what` the word `"land"`, holding nothing but its act's launch and
/// lease (core.53); and a `pulled` holding it, and
/// `pulled-with-a-land.json`, pass its `to-runner.schema.json` (design
/// Decision 1 of the-hosted-protocol-carries-land-and-preview).
fn a_land_is_an_act_of(version: Version) {
    let to_runner = version.messages_to_runner();
    let land = act_of(ActSort::Land);
    assert_eq!(land["act"]["what"], json!("land"));
    passes(
        &version.validator(&version.definition("wants.schema.json", "want")),
        &land,
        "a land",
    );
    let mut pulled = version.example("to-runner", "pulled.json");
    pulled["body"]["pulled"]["wants"] = json!([land]);
    passes(&to_runner, &pulled, "a pulled holding a land");
    let example = version.example("to-runner", "pulled-with-a-land.json");
    passes(&to_runner, &example, "pulled-with-a-land.json");
    assert_eq!(
        example["body"]["pulled"]["wants"][1]["act"]["what"],
        json!("land"),
        "pulled-with-a-land.json holds a land"
    );
}

/// The runner-protocol spec's "A land is an act of version 2".
#[test]
fn a_land_is_an_act_of_version_2() {
    a_land_is_an_act_of(V2);
}

/// The runner-protocol spec's "A land is an act of version 3".
#[test]
fn a_land_is_an_act_of_version_3() {
    a_land_is_an_act_of(V3);
}

/// A preview of `main` in `example/app` at `revision`, done or not, as the
/// interface writes it.
fn preview_of(revision: &str, done: bool) -> Want {
    Want::Preview(Preview {
        id: Id::new("preview-1"),
        lease: lease(),
        repository: "example/app".to_string(),
        branch: "main".to_string(),
        revision: revision.to_string(),
        done,
    })
}

/// A preview not done, one done, and one whose revision is `R1`, which
/// the version does not bound, so sluice answers it, as the interface
/// writes them, pass `version`'s `want`; and
/// `pulled-with-a-preview-and-its-done.json` passes its
/// `to-runner.schema.json` (design Decision 2 of
/// the-hosted-protocol-carries-land-and-preview; core.55).
fn a_preview_is_a_want_of(version: Version) {
    let want = version.validator(&version.definition("wants.schema.json", "want"));
    let commit = "0123456789abcdef0123456789abcdef01234567";
    for (preview, what) in [
        (preview_of(commit, false), "a preview"),
        (preview_of(commit, true), "a preview done"),
        (preview_of("R1", false), "a preview of the revision R1"),
    ] {
        passes(&want, &serde_json::to_value(&preview).unwrap(), what);
    }
    let example = version.example("to-runner", "pulled-with-a-preview-and-its-done.json");
    passes(
        &version.messages_to_runner(),
        &example,
        "pulled-with-a-preview-and-its-done.json",
    );
    let done: Vec<&Value> = example["body"]["pulled"]["wants"]
        .as_array()
        .unwrap()
        .iter()
        .map(|want| &want["preview"]["done"])
        .collect();
    assert_eq!(
        done,
        [&json!(false), &json!(true)],
        "a preview and its done"
    );
}

/// The runner-protocol spec's "A preview is a want of version 2".
#[test]
fn a_preview_is_a_want_of_version_2() {
    a_preview_is_a_want_of(V2);
}

/// The runner-protocol spec's "A preview is a want of version 3".
#[test]
fn a_preview_is_a_want_of_version_3() {
    a_preview_is_a_want_of(V3);
}

/// The name of `want`'s sort, by a match with no `_` arm, so a sort the
/// interface adds fails to build this test until it is named here.
fn want_sort(want: &Want) -> &'static str {
    match want {
        Want::Offer(_) => "offer",
        Want::Act(_) => "act",
        Want::Command(_) => "command",
        Want::Inbox(_) => "inbox",
        Want::Preview(_) => "preview",
    }
}

/// The name of `what`'s sort, by a match with no `_` arm.
fn act_sort(what: &ActSort) -> &'static str {
    match what {
        ActSort::Retry { .. } => "retry",
        ActSort::Land => "land",
        ActSort::Confirm { .. } => "confirm",
        ActSort::Hold { .. } => "hold",
        ActSort::Drop => "drop",
        ActSort::Priority(_) => "priority",
        ActSort::Done => "done",
    }
}

/// The name of `what`'s sort, by a match with no `_` arm.
fn command_sort(what: &CommandSort) -> &'static str {
    match what {
        CommandSort::Branch { .. } => "branch",
        CommandSort::Other { .. } => "other",
    }
}

/// Every sort of `Want`, `ActSort` and `CommandSort`, each named as the
/// matches above name it.
const WANT_SORTS: [&str; 5] = ["offer", "act", "command", "inbox", "preview"];
const ACT_SORTS: [&str; 7] = [
    "retry", "land", "confirm", "hold", "drop", "priority", "done",
];
const COMMAND_SORTS: [&str; 2] = ["branch", "other"];

/// A value of every sort the interface names: an offer, an act of each
/// act sort, a command of each command sort, an inbox and a preview, as
/// the interface writes them.
fn every_sort() -> Vec<Want> {
    let acts = [
        ActSort::Retry {
            stage: None,
            words: None,
            submission: true,
        },
        ActSort::Land,
        confirm_of(&["change--a/0a1b2c3d/stage/code/1/delivered"]),
        ActSort::Hold {
            words: "the user sends it".to_string(),
            bound_interval: 28800,
        },
        ActSort::Drop,
        ActSort::Priority(5),
        ActSort::Done,
    ];
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
    let mut wants = vec![Want::Offer(Offer {
        id: Id::new("offer-1"),
        lease: lease(),
        launch: launch(),
    })];
    wants.extend(acts.into_iter().map(|what| {
        Want::Act(Act {
            id: Id::new("act-1"),
            launch: Id::new("offer-1"),
            lease: 3,
            what,
        })
    }));
    wants.extend(commands.into_iter().map(|what| {
        Want::Command(Command {
            id: Id::new("command-1"),
            lease: lease(),
            what,
        })
    }));
    wants.push(Want::Inbox(Inbox {
        id: Id::new("inbox-1"),
        repository: "example/tools".to_string(),
        branch: "inbox".to_string(),
    }));
    wants.push(preview_of(
        "0123456789abcdef0123456789abcdef01234567",
        false,
    ));
    wants
}

/// The runner-protocol spec's "Every sort the interface names" (design
/// Decision 8 of the-hosted-protocol-carries-land-and-preview, and design
/// Decision 7 of the-hosted-protocol-carries-a-hold): a value of every
/// sort of `Want`, `ActSort` and `CommandSort`, each named by a match with
/// no `_` arm, passes version 4's `want` and version 3's, a hold among
/// them, and passes version 2's but the hold, which version 2 refuses,
/// since `v0.9.0` spoke version 2 before the interface named one; and the
/// sorts named are every sort the interface has. A sort the interface adds
/// fails to build the match until it is named, then fails this test until
/// a value of it is given, and fails version 4 until a version names it
/// (core.42, core.59).
#[test]
fn every_sort_the_interface_names() {
    let v4 = V4.validator(&V4.definition("wants.schema.json", "want"));
    let v3 = V3.validator(&V3.definition("wants.schema.json", "want"));
    let v2 = V2.validator(&V2.definition("wants.schema.json", "want"));
    let wants = every_sort();
    let mut named = (BTreeSet::new(), BTreeSet::new(), BTreeSet::new());
    for value in &wants {
        named.0.insert(want_sort(value));
        let mut in_version_2 = true;
        match value {
            Want::Act(act) => {
                named.1.insert(act_sort(&act.what));
                in_version_2 = !matches!(act.what, ActSort::Hold { .. });
            }
            Want::Command(command) => {
                named.2.insert(command_sort(&command.what));
            }
            Want::Offer(_) | Want::Inbox(_) | Want::Preview(_) => {}
        }
        let json = serde_json::to_value(value).unwrap();
        passes(&v4, &json, &format!("in version 4, {json}"));
        passes(&v3, &json, &format!("in version 3, {json}"));
        if in_version_2 {
            passes(&v2, &json, &format!("in version 2, {json}"));
        } else {
            refused(&v2, &json, &format!("in version 2, {json}"));
        }
    }
    assert_eq!(named.0, BTreeSet::from(WANT_SORTS), "every want sort");
    assert_eq!(named.1, BTreeSet::from(ACT_SORTS), "every act sort");
    assert_eq!(named.2, BTreeSet::from(COMMAND_SORTS), "every command sort");
}
