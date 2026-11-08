//! A launch a drop names, offered and claimed only where the runner
//! launched it, let go where the runner holds it unlaunched, and its drop
//! given to the runner that holds it whatever the plan holds (the change
//! a-dropped-launch-is-never-offered, design Decisions 1 to 6). Each test
//! runs over one bare state repository, `deneb` and `vega` its hosts.

mod common;

use boomdrive_control::{
    Acted, Answer, At, Carried, ControlPlane, Hello, Id, Launched, Outcome, Report, Want,
};
use boomdrive_crew::read::{host_read, RunnerFiles, LEASE};
use boomdrive_crew::write;
use common::{act, file, hello, ids, launch, plan, pull, State, REQUESTS};
use std::collections::BTreeMap;
use weir_capture::rec::Record;

const CREW: &str = "afterthought/crew";
const PLACE: &str = "/places/change--a";

fn kept() -> Hello {
    hello(&[CREW], "4")
}

/// `madswan/main` holding `requests`, its plan holding `units`.
fn main_holding(state: &State, requests: &[String], units: &[&str]) {
    state.main(&[
        ("requests.rec", &file(REQUESTS, requests)),
        ("plan.rec", &plan(units, &[])),
    ]);
}

/// `host`'s branch as crew reads it: its answers, by request.
fn answers(state: &State, host: &str) -> BTreeMap<u64, Record> {
    let branch = format!("madswan/runner/{host}");
    let mut files = RunnerFiles::new();
    for path in ["runner.rec", "answers.rec", "places.rec"] {
        files.insert(path, state.file(&branch, path));
    }
    host_read(host, &files).expect("crew reads it").answers
}

fn at(text: &str) -> At {
    At::parse(text).unwrap()
}

/// A launch answered `outcome` at `place` with `words`, its place's
/// status carried and listed or not.
fn answered(
    outcome: Outcome,
    place: Option<&str>,
    words: Option<&str>,
    listed: Option<bool>,
) -> Launched {
    Launched {
        lease: LEASE,
        answer: Answer {
            outcome,
            place_dir: place.map(str::to_string),
            words: words.map(str::to_string),
            at: at("2026-11-01T10:00:00Z"),
        },
        status: listed.map(|listed| Carried {
            text: format!(
                r#"{{"status":{{"places":[{{"place":"{}","phase":"live"}}],"host":{{}}}}}}"#,
                place.unwrap_or_default()
            ),
            at: at("2026-11-01T10:01:00Z"),
            listed,
        }),
        entry: None,
    }
}

/// The runner's report answering launch `n` as `launched` has it.
fn report_of(n: u64, launched: Launched) -> Report {
    let mut report = Report::default();
    report
        .launches
        .insert(Id::new(format!("madswan/{n}")), launched);
    report
}

/// Launch `n` held on its After, as the runner answers it.
fn held(n: u64) -> Report {
    report_of(
        n,
        answered(
            Outcome::Held,
            None,
            Some("held on its After madswan/11"),
            None,
        ),
    )
}

/// The crew-control-plane spec's "A drop naming another host": a drop
/// counts whatever host it names, so the launch it names is given no
/// offer, and the drop, naming another host, is given nothing.
#[test]
fn a_drop_naming_another_host() {
    let state = State::new();
    main_holding(
        &state,
        &[
            launch(12, Some("deneb"), "unit/u", CREW, "u", &[]),
            act(14, "drop", Some("vega"), "unit/u", 12, &[]),
        ],
        &["u"],
    );
    let (mut deneb, _) = state.crew("deneb");
    let pulled = deneb.pull(&kept(), &held(12)).expect("a pull");
    assert!(pulled.wants.is_empty(), "{:?}", ids(&pulled));
}

/// "A held launch dropped": the drop, naming this host, is given while
/// the plan holds the launch's unit, and the launch it names is not.
#[test]
fn a_held_launch_dropped() {
    let state = State::new();
    main_holding(
        &state,
        &[
            launch(12, Some("deneb"), "unit/u", CREW, "u", &[]),
            act(14, "drop", Some("deneb"), "unit/u", 12, &[]),
        ],
        &["u"],
    );
    let (mut deneb, _) = state.crew("deneb");
    let pulled = deneb.pull(&kept(), &held(12)).expect("a pull");
    assert_eq!(ids(&pulled), ["madswan/14"]);
}

/// "A dropped launch the runner launched": it is given still, so its
/// place is followed to its end, and its drop with it.
#[test]
fn a_dropped_launch_the_runner_launched() {
    let state = State::new();
    main_holding(
        &state,
        &[
            launch(12, Some("deneb"), "unit/u", CREW, "u", &[]),
            act(14, "drop", Some("deneb"), "unit/u", 12, &[]),
        ],
        &["u"],
    );
    let (mut deneb, _) = state.crew("deneb");
    let report = report_of(
        12,
        answered(Outcome::Launched, Some(PLACE), None, Some(true)),
    );
    let pulled = deneb.pull(&kept(), &report).expect("a pull");
    assert_eq!(ids(&pulled), ["madswan/12", "madswan/14"]);
}

/// "A kit's launch dropped before it is claimed": no runner answers it,
/// so it is claimed by no one, no branch is written, and neither it nor
/// its drop is given.
#[test]
fn a_kits_launch_dropped_before_it_is_claimed() {
    let state = State::new();
    main_holding(
        &state,
        &[
            launch(20, None, "unit/u", CREW, "u", &[]),
            act(21, "drop", None, "unit/u", 20, &[]),
        ],
        &["u"],
    );
    let (mut deneb, _) = state.crew("deneb");
    let pulled = pull(&mut deneb, &kept());
    assert!(pulled.wants.is_empty(), "{:?}", ids(&pulled));
    assert_eq!(state.tip("madswan/runner/deneb"), None);
}

/// "A claimed launch dropped": a claim standing on this host's branch is
/// not given once a drop names its launch, nor is the drop, and the
/// report's write leaves the claim out.
#[test]
fn a_claimed_launch_dropped() {
    let state = State::new();
    let kit = launch(20, None, "unit/u", CREW, "u", &[]);
    main_holding(&state, &[kit.clone()], &["u"]);
    let (mut deneb, _) = state.crew("deneb");
    assert_eq!(ids(&pull(&mut deneb, &kept())), ["madswan/20"]);
    // The runner leaves the offer unanswered, its stream having no room.
    deneb.report(&Report::default()).expect("a report");
    assert!(answers(&state, "deneb").contains_key(&20));

    main_holding(
        &state,
        &[kit, act(21, "drop", None, "unit/u", 20, &[])],
        &["u"],
    );
    let pulled = pull(&mut deneb, &kept());
    assert!(pulled.wants.is_empty(), "{:?}", ids(&pulled));
    deneb.report(&Report::default()).expect("a report");
    assert!(!answers(&state, "deneb").contains_key(&20));
}

/// "Work gone that the runner does not hold": a request whose work has
/// left the plan, which no runner answers, is given nothing.
#[test]
fn work_gone_that_the_runner_does_not_hold() {
    let state = State::new();
    main_holding(
        &state,
        &[launch(12, Some("deneb"), "unit/u", CREW, "u", &[])],
        &[],
    );
    let (mut deneb, _) = state.crew("deneb");
    let pulled = pull(&mut deneb, &kept());
    assert!(pulled.wants.is_empty(), "{:?}", ids(&pulled));
}

/// "A drop written as its work leaves the plan": the runner holds the
/// launch it names, so the drop is given though its work has left the
/// plan.
#[test]
fn a_drop_written_as_its_work_leaves_the_plan() {
    let state = State::new();
    main_holding(
        &state,
        &[
            launch(12, Some("deneb"), "unit/u", CREW, "u", &[]),
            act(14, "drop", Some("deneb"), "unit/u", 12, &[]),
        ],
        &[],
    );
    let (mut deneb, _) = state.crew("deneb");
    let report = report_of(
        12,
        answered(Outcome::Launched, Some(PLACE), None, Some(true)),
    );
    let pulled = deneb.pull(&kept(), &report).expect("a pull");
    assert_eq!(ids(&pulled), ["madswan/12", "madswan/14"]);
    assert!(
        matches!(common::want(&pulled, "madswan/14"), Want::Act(_)),
        "{:?}",
        pulled.wants
    );
}

/// "A kit's drop goes to the runner holding its launch": `deneb`'s branch
/// answers the kit's launch, so `deneb` is given it and its drop, and
/// `vega` neither.
#[test]
fn a_kits_drop_goes_to_the_runner_holding_its_launch() {
    let state = State::new();
    main_holding(
        &state,
        &[
            launch(20, None, "unit/u", CREW, "u", &[]),
            act(21, "drop", None, "unit/u", 20, &[]),
        ],
        &[],
    );
    let mut answer = Record::new();
    answer.push("Request", "20");
    answer.push("Outcome", "launched");
    answer.push("Place", PLACE);
    answer.push("At", "2026-11-01T09:40:00Z");
    state.commit(
        "madswan/runner/deneb",
        &[
            (
                "runner.rec",
                &write::runner_text("deneb", "boomdrive 0.0.0", "2026-11-01T09:00:00Z"),
            ),
            (
                "answers.rec",
                &write::answers_text(&[(20, answer)].into_iter().collect()),
            ),
            (
                "places.rec",
                &write::places_text(&BTreeMap::new(), &BTreeMap::new()),
            ),
        ],
    );
    let (mut deneb, _) = state.crew("deneb");
    assert_eq!(
        ids(&pull(&mut deneb, &kept())),
        ["madswan/20", "madswan/21"]
    );
    let (mut vega, _) = state.crew("vega");
    let pulled = pull(&mut vega, &kept());
    assert!(pulled.wants.is_empty(), "{:?}", ids(&pulled));
    assert_eq!(state.tip("madswan/runner/vega"), None);
}

/// A pull of `crew`, handed `report`, then a report of it: the ids let
/// go.
fn let_go(crew: &mut boomdrive_crew::Crew, report: &Report) -> Vec<String> {
    crew.pull(&kept(), report).expect("a pull");
    let reported = crew.report(report).expect("a report");
    reported.let_go.iter().map(Id::to_string).collect()
}

/// "A held launch dropped is let go": a launch a drop names that the
/// runner holds unlaunched leaves its files, though its unit is still in
/// the plan.
#[test]
fn a_held_launch_dropped_is_let_go() {
    let state = State::new();
    main_holding(
        &state,
        &[
            launch(12, Some("deneb"), "unit/u", CREW, "u", &[]),
            act(14, "drop", Some("deneb"), "unit/u", 12, &[]),
        ],
        &["u"],
    );
    let (mut deneb, _) = state.crew("deneb");
    assert_eq!(let_go(&mut deneb, &held(12)), ["madswan/12"]);
    assert!(!answers(&state, "deneb").contains_key(&12));
}

/// Drop 14 of launch 12, done, as the runner answers it.
fn with_drop_done(mut report: Report) -> Report {
    report.acts.insert(
        Id::new("madswan/14"),
        Acted {
            launch: Id::new("madswan/12"),
            lease: LEASE.number,
            outcome: Outcome::Done,
            words: None,
            at: at("2026-11-01T10:02:00Z"),
        },
    );
    report
}

/// "A drop answered from the report, its work gone": the launch the
/// runner answered done from its report, and its drop, go together.
#[test]
fn a_drop_answered_from_the_report_its_work_gone() {
    let state = State::new();
    main_holding(
        &state,
        &[
            launch(12, Some("deneb"), "unit/u", CREW, "u", &[]),
            act(14, "drop", Some("deneb"), "unit/u", 12, &[]),
        ],
        &[],
    );
    let (mut deneb, _) = state.crew("deneb");
    let report = with_drop_done(report_of(
        12,
        answered(
            Outcome::Done,
            None,
            Some("dropped by madswan/14 before it was launched"),
            None,
        ),
    ));
    assert_eq!(let_go(&mut deneb, &report), ["madswan/12", "madswan/14"]);
}

/// "A dropped launch the runner launched stays until its place is over":
/// while Flume lists its place, neither it nor its drop is let go; once
/// Flume lists it no more, both are.
#[test]
fn a_dropped_launch_the_runner_launched_stays_until_its_place_is_over() {
    let state = State::new();
    main_holding(
        &state,
        &[
            launch(12, Some("deneb"), "unit/u", CREW, "u", &[]),
            act(14, "drop", Some("deneb"), "unit/u", 12, &[]),
        ],
        &[],
    );
    let (mut deneb, _) = state.crew("deneb");
    let listed = |listed: bool| {
        with_drop_done(report_of(
            12,
            answered(Outcome::Launched, Some(PLACE), None, Some(listed)),
        ))
    };
    assert!(let_go(&mut deneb, &listed(true)).is_empty());
    assert_eq!(
        let_go(&mut deneb, &listed(false)),
        ["madswan/12", "madswan/14"]
    );
}
