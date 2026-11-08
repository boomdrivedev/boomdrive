//! A role's run, offered while crew's records want it and none stands,
//! and declared done once its turn has delivered, as crew's `asks.py`
//! reads them (design Decision 19; Task notes 8.6). The cases are crew's
//! `role-runs` scenarios and `tests/py/test_asks.py` at `9913f249`.

mod common;

use boomdrive_control::{
    Act, ActSort, AgentSetting, Answer, At, Carried, ControlPlane, Hello, Id, Launch, Launched,
    Lease, Offer, Outcome, Report, Want,
};
use boomdrive_crew::write;
use common::{
    file, hello, ids, launch, pull, rec, role, want, State, ASKS, REQUESTS, ROLES, TURNS,
};
use std::collections::BTreeMap;

const PLACE: &str = "/places/crew-planner--home";

fn kept() -> Hello {
    hello(&["example/blueprints", "example/crew"], "4")
}

fn ask(n: u64, role: &str) -> String {
    rec(&[
        ("Id", &n.to_string()),
        ("Role", role),
        ("From", "crew"),
        ("What", "tell"),
        ("At", "2026-11-01T09:00:00Z"),
    ])
}

/// A turn of the planner's run, taking `took`, ended or superseded where
/// said.
fn turn(n: u64, took: &[&str], ended: bool, superseded: Option<u64>) -> String {
    let number = n.to_string();
    let mut fields: Vec<(&str, String)> = vec![
        ("Turn", number),
        ("Role", "planner".to_string()),
        ("Run", "home-planner".to_string()),
        ("Place", "crew-planner--home".to_string()),
        ("Host", "host-a".to_string()),
    ];
    fields.extend(took.iter().map(|t| ("Took", t.to_string())));
    fields.push(("Began", "2026-11-01T09:10:00Z".to_string()));
    if ended {
        fields.push(("Ended", "2026-11-01T09:20:00Z".to_string()));
        fields.push(("Said", "done".to_string()));
    }
    if let Some(by) = superseded {
        fields.push(("Superseded", by.to_string()));
    }
    let fields: Vec<(&str, &str)> = fields.iter().map(|(f, v)| (*f, v.as_str())).collect();
    rec(&fields)
}

/// `asks.rec` as crew writes it: its header and `Ask` descriptor, the
/// asks, its `Turn` descriptor and the turns.
fn asks_rec(asks: &[String], turns: &[String]) -> String {
    format!("{}\n{}", file(ASKS, asks), file(TURNS, turns))
}

/// `home/main` holding the planner's role on host-a, `asks`, `turns`
/// and `requests`.
fn main_with(state: &State, asks: &[String], turns: &[String], requests: &[String]) {
    state.main(&[
        ("roles.rec", &file(ROLES, &[role("planner", "host-a")])),
        ("asks.rec", &asks_rec(asks, turns)),
        ("requests.rec", &file(REQUESTS, requests)),
    ]);
}

/// Flume's status of one place, as the runner carries it.
fn status(place: serde_json::Value) -> String {
    serde_json::json!({"status": {"places": [place], "host": {}}}).to_string()
}

/// A place at `PLACE` waiting on its caller with `why`.
fn waiting(why: &str, stage: Option<&str>, stalled: bool) -> String {
    status(
        serde_json::json!({"place": PLACE, "phase": "live", "stalled": stalled,
        "waiting_on": {"act": "retry", "stage": stage, "why": why, "since": 1793440000}}),
    )
}

/// The planner's offer `k`, as crew's role record gives it.
fn planner(k: u64) -> Want {
    Want::Offer(Offer {
        id: Id::new(format!("home/planner/{k}")),
        lease: Lease {
            number: 1,
            expires: None,
        },
        launch: Launch {
            repository: "example/blueprints".to_string(),
            kind: "crew/crew-planner".to_string(),
            name: "home".to_string(),
            base: None,
            brief: None,
            stream: Some("home-main".to_string()),
            priority: Some(0),
            intent: Some("home-planner".to_string()),
            recipients: vec![],
            holds: vec![],
            hold_submission: false,
            after: vec![],
            agent_settings: vec![AgentSetting {
                field: "model".to_string(),
                value: "claude-fable-5-1".to_string(),
            }],
        },
    })
}

/// A report holding the planner's run `k`, answered `outcome` at `place`
/// with `status` carried.
fn run(k: u64, outcome: Outcome, place: Option<&str>, status: Option<&str>) -> Report {
    let mut report = Report::default();
    report.launches.insert(
        Id::new(format!("home/planner/{k}")),
        Launched {
            lease: Lease {
                number: 1,
                expires: None,
            },
            answer: Answer {
                outcome,
                place_dir: place.map(str::to_string),
                words: (outcome == Outcome::Refused).then(|| "no stream".to_string()),
                at: At::parse("2026-11-01T10:00:00Z").unwrap(),
            },
            status: status.map(|text| Carried {
                text: text.to_string(),
                at: At::parse("2026-11-01T10:01:00Z").unwrap(),
                listed: true,
            }),
            entry: None,
        },
    );
    report
}

fn wants(state: &State, report: &Report) -> Vec<Want> {
    let (mut crew, _) = state.crew("host-a");
    crew.pull(&kept(), report).expect("a pull").wants
}

#[test]
fn a_burst_of_asks() {
    let state = State::new();
    main_with(
        &state,
        &[ask(1, "planner"), ask(2, "planner"), ask(3, "planner")],
        &[],
        &[],
    );
    assert_eq!(wants(&state, &Report::default()), [planner(1)]);
}

#[test]
fn an_ask_after_a_turn_began() {
    let state = State::new();
    let asks = [ask(1, "planner"), ask(2, "planner"), ask(3, "planner")];
    main_with(
        &state,
        &asks,
        &[turn(1, &["ask/1", "ask/2"], false, None)],
        &[],
    );
    assert_eq!(wants(&state, &Report::default()), [planner(1)]);
    main_with(
        &state,
        &asks,
        &[turn(1, &["ask/1", "ask/2"], true, None)],
        &[],
    );
    assert_eq!(wants(&state, &Report::default()), [planner(1)]);
}

#[test]
fn nothing_pending() {
    let state = State::new();
    main_with(
        &state,
        &[ask(1, "planner"), ask(2, "planner"), ask(3, "design")],
        &[
            turn(1, &["ask/1"], true, None),
            turn(2, &["ask/2"], true, None),
        ],
        &[],
    );
    assert_eq!(wants(&state, &Report::default()), []);
}

#[test]
fn a_superseded_turn_answers_nothing() {
    let state = State::new();
    main_with(
        &state,
        &[ask(1, "planner")],
        &[
            turn(1, &["ask/1"], false, Some(2)),
            turn(2, &["ask/1"], false, None),
        ],
        &[],
    );
    assert_eq!(wants(&state, &Report::default()), [planner(1)]);
}

#[test]
fn a_role_of_another_host() {
    let state = State::new();
    state.main(&[
        ("roles.rec", &file(ROLES, &[role("planner", "host-b")])),
        ("asks.rec", &asks_rec(&[ask(1, "planner")], &[])),
    ]);
    assert_eq!(wants(&state, &Report::default()), []);
}

/// host-b's branch, its places.rec holding `places`, as host-b's runner
/// writes it; `runner` its runner.rec.
fn host_b_places(state: &State, runner: &str, places: &[(u64, &str)]) {
    let places: BTreeMap<u64, weir_capture::rec::Record> = places
        .iter()
        .map(|(n, status)| {
            let mut record = weir_capture::rec::Record::new();
            record.push("Request", n.to_string());
            record.push("Place", "/places/change--a");
            record.push("At", "2026-11-01T10:00:00Z");
            record.push("Status", *status);
            (*n, record)
        })
        .collect();
    state.commit(
        "home/runner/host-b",
        &[
            ("runner.rec", runner),
            ("answers.rec", &write::answers_text(&BTreeMap::new())),
            ("places.rec", &write::places_text(&places, &BTreeMap::new())),
        ],
    );
}

/// Launch 7 for host-b, its recipient the planner.
fn asking_launch() -> String {
    launch(
        7,
        Some("host-b"),
        "fix/main/crew/x",
        "example/crew",
        "x",
        &[("Recipient", "home/planner"), ("Hold", "submission")],
    )
}

#[test]
fn another_hosts_place_asks_the_planner() {
    let state = State::new();
    main_with(&state, &[], &[], &[asking_launch()]);
    let runner = write::runner_text("host-b", "boomdrive 0.0.0", "2026-11-01T09:00:00Z");
    host_b_places(
        &state,
        &runner,
        &[(7, &waiting("answer", Some("code"), false))],
    );
    assert_eq!(wants(&state, &Report::default()), [planner(1)]);
}

#[test]
fn the_place_taken_by_an_ended_turn_is_no_longer_pending() {
    let state = State::new();
    main_with(
        &state,
        &[],
        &[turn(1, &["place/7@1793440000"], true, None)],
        &[asking_launch()],
    );
    let runner = write::runner_text("host-b", "boomdrive 0.0.0", "2026-11-01T09:00:00Z");
    host_b_places(
        &state,
        &runner,
        &[(7, &waiting("answer", Some("code"), false))],
    );
    assert_eq!(wants(&state, &Report::default()), []);
}

#[test]
fn a_place_lost_past_its_bound_held_or_stalled_waits_on_the_role() {
    let state = State::new();
    main_with(&state, &[], &[], &[asking_launch()]);
    let runner = write::runner_text("host-b", "boomdrive 0.0.0", "2026-11-01T09:00:00Z");
    for (status, wanted) in [
        (waiting("lost", Some("code"), false), true),
        (waiting("past_bound", Some("code"), false), true),
        (waiting("on_hold", None, false), true),
        (waiting("on_hold", Some("code"), false), false),
        (waiting("never_begun", Some("code"), true), true),
        (waiting("never_begun", Some("code"), false), false),
        (waiting("delivered", None, false), false),
    ] {
        host_b_places(&state, &runner, &[(7, &status)]);
        let expected = if wanted { vec![planner(1)] } else { vec![] };
        assert_eq!(wants(&state, &Report::default()), expected, "{status}");
    }
}

#[test]
fn a_host_whose_branch_does_not_parse() {
    let state = State::new();
    main_with(&state, &[], &[], &[asking_launch()]);
    let broken = write::runner_text("host-a", "boomdrive 0.0.0", "2026-11-01T09:00:00Z");
    host_b_places(
        &state,
        &broken,
        &[(7, &waiting("answer", Some("code"), false))],
    );
    let (mut crew, said) = state.crew("host-a");
    assert_eq!(pull(&mut crew, &kept()).wants, []);
    assert!(
        said.holds(&[
            "host-b's runner can't be read",
            "runner.rec names host host-a"
        ]),
        "{:?}",
        said.lines()
    );
}

#[test]
fn a_roles_work_landed() {
    let state = State::new();
    let landed = launch(
        9,
        Some("host-a"),
        "role/planner/record-0053",
        "example/crew",
        "record-0053",
        &[("Recipient", "home/planner"), ("Stream", "home-main")],
    );
    main_with(&state, &[], &[], &[landed]);
    let mut report = Report::default();
    report.launches.insert(
        Id::new("home/9"),
        Launched {
            lease: Lease {
                number: 1,
                expires: None,
            },
            answer: Answer {
                outcome: Outcome::Launched,
                place_dir: Some("/places/chore--record-0053".to_string()),
                words: None,
                at: At::parse("2026-11-01T10:00:00Z").unwrap(),
            },
            status: Some(Carried {
                text: status(serde_json::json!({"place": "/places/chore--record-0053",
                    "phase": {"tidied": {"over": {"landed": {"commit": "abcdef1234567"}}, "at": 1793440000}}})),
                at: At::parse("2026-11-01T10:01:00Z").unwrap(),
                listed: true,
            }),
            entry: None,
        },
    );
    let given = wants(&state, &report);
    assert_eq!(given[1..], [planner(1)]);
    // Taken by an ended turn, it waits no more.
    main_with(
        &state,
        &[],
        &[turn(1, &["landed/9"], true, None)],
        &[launch(
            9,
            Some("host-a"),
            "role/planner/record-0053",
            "example/crew",
            "record-0053",
            &[("Recipient", "home/planner"), ("Stream", "home-main")],
        )],
    );
    assert_eq!(wants(&state, &report).len(), 1);
}

#[test]
fn a_run_standing() {
    let state = State::new();
    main_with(&state, &[ask(1, "planner")], &[], &[]);
    let live = status(serde_json::json!({"place": PLACE, "phase": "live", "waiting_on": null}));
    let report = run(1, Outcome::Launched, Some(PLACE), Some(&live));
    // The run the report holds is offered again, so its status is carried;
    // no second run is.
    assert_eq!(wants(&state, &report), [planner(1)]);
    // Launched, its status not yet carried, it stands.
    let report = run(1, Outcome::Launched, Some(PLACE), None);
    assert_eq!(wants(&state, &report), [planner(1)]);
}

#[test]
fn a_run_on_the_branch_before_the_first_report() {
    let state = State::new();
    main_with(&state, &[ask(1, "planner")], &[], &[]);
    let live = status(serde_json::json!({"place": PLACE, "phase": "live"}));
    let mut run = weir_capture::rec::Record::new();
    run.push("Address", "home/planner");
    run.push("Place", PLACE);
    run.push("At", "2026-11-01T10:00:00Z");
    run.push("Status", live);
    let runs: BTreeMap<String, _> = [("home/planner".to_string(), run)].into();
    state.commit(
        "home/runner/host-a",
        &[
            (
                "runner.rec",
                &write::runner_text("host-a", "boomdrive 0.0.0", "2026-11-01T09:00:00Z"),
            ),
            ("places.rec", &write::places_text(&BTreeMap::new(), &runs)),
        ],
    );
    assert_eq!(wants(&state, &Report::default()), []);
}

#[test]
fn a_turn_delivered() {
    let state = State::new();
    main_with(&state, &[ask(1, "planner")], &[], &[]);
    let delivered = common::status_delivered();
    let value: serde_json::Value = serde_json::from_str(&delivered).unwrap();
    let place = value["status"]["places"][0]["place"].as_str().unwrap();
    let mut report = run(1, Outcome::Refused, None, None);
    report
        .launches
        .extend(run(2, Outcome::Launched, Some(place), Some(&delivered)).launches);
    let given = wants(&state, &report);
    assert_eq!(
        given[2],
        Want::Act(Act {
            id: Id::new("home/planner/2/done"),
            launch: Id::new("home/planner/2"),
            lease: 1,
            what: ActSort::Done,
        })
    );
    // The run delivered stands until its done is carried out.
    assert_eq!(given.len(), 3, "{given:?}");
}

#[test]
fn a_run_done_or_tidied_stands_no_more() {
    let state = State::new();
    main_with(&state, &[ask(1, "planner")], &[], &[]);
    let delivered = common::status_delivered();
    let value: serde_json::Value = serde_json::from_str(&delivered).unwrap();
    let place = value["status"]["places"][0]["place"].as_str().unwrap();
    let mut report = run(2, Outcome::Launched, Some(place), Some(&delivered));
    report.acts.insert(
        Id::new("home/planner/2/done"),
        boomdrive_control::Acted {
            launch: Id::new("home/planner/2"),
            lease: 1,
            outcome: Outcome::Done,
            words: None,
            at: At::parse("2026-11-01T10:05:00Z").unwrap(),
        },
    );
    assert_eq!(wants(&state, &report), [planner(2), planner(3)]);
    let live = status(serde_json::json!({"place": PLACE, "phase": "live"}));
    let mut unlisted = run(4, Outcome::Launched, Some(PLACE), Some(&live));
    for launched in unlisted.launches.values_mut() {
        launched.status.as_mut().unwrap().listed = false;
    }
    assert_eq!(wants(&state, &unlisted), [planner(4), planner(5)]);
}

#[test]
fn a_run_waiting_for_room_keeps_its_id() {
    let state = State::new();
    main_with(&state, &[ask(1, "planner")], &[], &[]);
    let mut report = run(1, Outcome::Refused, None, None);
    report.launches.extend(run(2, Outcome::Launched, Some(PLACE), Some(&status(
        serde_json::json!({"place": format!("{PLACE}@1793474400"), "phase": {"tidied": {"over": "done"}}}),
    ))).launches);
    let (mut crew, _) = state.crew("host-a");
    let offered = |crew: &mut boomdrive_crew::Crew, report: &Report| {
        let pulled = crew.pull(&kept(), report).expect("a pull");
        ids(&pulled)
    };
    // Left untaken for want of room: the report is as it was, and the
    // same run is offered again.
    assert_eq!(
        offered(&mut crew, &report),
        ["home/planner/1", "home/planner/2", "home/planner/3"]
    );
    assert_eq!(
        offered(&mut crew, &report),
        ["home/planner/1", "home/planner/2", "home/planner/3"]
    );
    // Recorded refused, the next is offered.
    report
        .launches
        .extend(run(3, Outcome::Refused, None, None).launches);
    let pulled = crew.pull(&kept(), &report).expect("a pull");
    assert_eq!(want(&pulled, "home/planner/4"), &planner(4));
}

#[test]
fn a_dones_refusal_is_the_runs_words() {
    let state = State::new();
    main_with(&state, &[ask(1, "planner")], &[], &[]);
    let delivered = common::status_delivered();
    let value: serde_json::Value = serde_json::from_str(&delivered).unwrap();
    let place = value["status"]["places"][0]["place"].as_str().unwrap();
    let mut report = run(2, Outcome::Launched, Some(place), Some(&delivered));
    report.acts.insert(
        Id::new("home/planner/2/done"),
        boomdrive_control::Acted {
            launch: Id::new("home/planner/2"),
            lease: 1,
            outcome: Outcome::Refused,
            words: Some("its agent is not quiet".to_string()),
            at: At::parse("2026-11-01T10:05:00Z").unwrap(),
        },
    );
    let (mut crew, _) = state.crew("host-a");
    crew.pull(&kept(), &report).expect("a pull");
    crew.report(&report).expect("a report");
    let places = state.file("home/runner/host-a", "places.rec").unwrap();
    let run = places.split("%rec: Run").nth(1).unwrap();
    assert!(
        run.contains(&format!("\nAddress: home/planner\nPlace: {place}\n")),
        "{run}"
    );
    assert!(run.ends_with("\nWords: its agent is not quiet\n"), "{run}");
    // The done's own answer is the run's, and no request's.
    let answers = state.file("home/runner/host-a", "answers.rec").unwrap();
    assert!(!answers.contains("its agent is not quiet"), "{answers}");
}
