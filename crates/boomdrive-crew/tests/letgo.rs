//! A launch let go once Flume no longer lists its place and its work has
//! left crew's plan, and a role's run once a later one is recorded or its
//! role has gone (design Decision 17; Task notes 8.7).

mod common;

use boomdrive_control::{
    Acted, Answer, At, Carried, ControlPlane, Hello, Id, Launched, Lease, Outcome, Report,
};
use boomdrive_crew::Crew;
use common::{act, file, hello, launch, plan, rec, role, State, REQUESTS, ROLES};

const BRANCH: &str = "home/runner/host-a";
const PLACE: &str = "/places/change--a";

fn kept() -> Hello {
    hello(&["example/crew"], "4")
}

/// `home/main` holding launch 12 of `work` and its priority 13 for
/// host-a, the planner's role where `planner`, and `plan` as its plan.
fn main_with(state: &State, work: &str, plan_rec: &str, planner: bool) {
    let roles: Vec<String> = match planner {
        true => vec![role("planner", "host-a")],
        false => vec![],
    };
    state.main(&[
        (
            "requests.rec",
            &file(
                REQUESTS,
                &[
                    launch(12, Some("host-a"), work, "example/crew", "a", &[]),
                    act(
                        13,
                        "priority",
                        Some("host-a"),
                        work,
                        12,
                        &[("Priority", "4")],
                    ),
                ],
            ),
        ),
        ("plan.rec", plan_rec),
        ("roles.rec", &file(ROLES, &roles)),
    ]);
}

fn at(text: &str) -> At {
    At::parse(text).unwrap()
}

/// A launch answered `outcome` at `place`, its status carried and
/// listed or not.
fn launched(outcome: Outcome, place: Option<&str>, listed: Option<bool>) -> Launched {
    Launched {
        lease: Lease {
            number: 1,
            expires: None,
        },
        repository: None,
        answer: Answer {
            outcome,
            place_dir: place.map(str::to_string),
            words: None,
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

/// The runner's report of launch 12 at its place, listed or not, and its
/// priority 13 done.
fn report(listed: bool) -> Report {
    let mut report = Report::default();
    report.launches.insert(
        Id::new("home/12"),
        launched(Outcome::Launched, Some(PLACE), Some(listed)),
    );
    report.acts.insert(
        Id::new("home/13"),
        Acted {
            launch: Id::new("home/12"),
            lease: 1,
            outcome: Outcome::Done,
            words: None,
            at: at("2026-11-01T10:02:00Z"),
        },
    );
    report
}

/// A pull of `crew`, handed `report`, then a report of it: the ids let
/// go.
fn let_go(crew: &mut Crew, report: &Report) -> Vec<String> {
    crew.pull(&kept(), report).expect("a pull");
    let reported = crew.report(report).expect("a report");
    reported.let_go.iter().map(Id::to_string).collect()
}

#[test]
fn a_unit_still_in_the_plan() {
    let state = State::new();
    main_with(&state, "unit/u", &plan(&["u"], &["b"]), false);
    let (mut crew, _) = state.crew("host-a");
    assert!(let_go(&mut crew, &report(false)).is_empty());
    let places = state.file(BRANCH, "places.rec").unwrap();
    assert!(
        places.contains("\nRequest: 12\nPlace: /places/change--a\n"),
        "{places}"
    );
}

#[test]
fn a_unit_landed_and_gone_from_the_plan() {
    let state = State::new();
    main_with(&state, "unit/u", &plan(&["u"], &["b"]), false);
    let (mut crew, _) = state.crew("host-a");
    let_go(&mut crew, &report(true));
    main_with(&state, "unit/u", &plan(&[], &["b"]), false);
    // Still listed by Flume, it stays.
    assert!(let_go(&mut crew, &report(true)).is_empty());
    // Tidied, it goes, its acts with it.
    assert_eq!(let_go(&mut crew, &report(false)), ["home/12", "home/13"]);
    let answers = state.file(BRANCH, "answers.rec").unwrap();
    let places = state.file(BRANCH, "places.rec").unwrap();
    assert!(!answers.contains("Request:"), "{answers}");
    assert!(!places.contains("Request:"), "{places}");
    // Let go, the runner's report holds it no more, and it is given no
    // more.
    let pulled = crew.pull(&kept(), &Report::default()).expect("a pull");
    assert!(pulled.wants.is_empty(), "{:?}", pulled.wants);
}

/// The crew-control-plane spec's "One partition missing at a report":
/// `work/main` unread, `home`'s branch is written all the same,
/// its let-go given, and `work`'s branch named as not written.
#[test]
fn one_partition_missing_at_a_report() {
    let state = State::new();
    state.make("example/work-state");
    main_with(&state, "unit/u", &plan(&[], &["b"]), false);
    let (mut crew, said) = state.crew_serving(
        "host-a",
        &[
            ("work", "example/work-state"),
            ("home", "example/home-state"),
        ],
    );
    assert_eq!(let_go(&mut crew, &report(false)), ["home/12", "home/13"]);
    let answers = state.file(BRANCH, "answers.rec").unwrap();
    assert!(!answers.contains("Request:"), "{answers}");
    assert!(
        said.holds(&[
            "crew: work/main has not been read",
            "work/runner/host-a is not written"
        ]),
        "{:?}",
        said.lines()
    );
}

/// With no partition's `<label>/main` read, nothing is written and the
/// report is unsent, naming each.
#[test]
fn a_report_before_any_partition_is_read() {
    let state = State::new();
    let (mut crew, _) = state.crew("host-a");
    let unsent = crew.report(&report(false)).unwrap_err();
    assert!(
        unsent.0.contains("home/main has not been read"),
        "{}",
        unsent.0
    );
    assert_eq!(state.tip(BRANCH), None);
}

#[test]
fn a_unit_gone_from_the_plan_is_still_given_while_the_runner_holds_it() {
    let state = State::new();
    main_with(&state, "unit/u", &plan(&[], &[]), false);
    let (mut crew, _) = state.crew("host-a");
    let given = crew.pull(&kept(), &report(true)).expect("a pull");
    let ids: Vec<String> = given.wants.iter().map(|w| w.id().to_string()).collect();
    assert_eq!(ids, ["home/12", "home/13"]);
}

#[test]
fn a_fix_on_a_bolt_gone_with_its_bolt() {
    let state = State::new();
    main_with(&state, "fix/b/x", &plan(&[], &["b"]), false);
    let (mut crew, _) = state.crew("host-a");
    assert!(let_go(&mut crew, &report(false)).is_empty());
    main_with(&state, "fix/b/x", &plan(&[], &[]), false);
    assert_eq!(let_go(&mut crew, &report(false)), ["home/12", "home/13"]);
}

#[test]
fn a_fix_of_main() {
    let state = State::new();
    main_with(&state, "fix/main/crew/x", &plan(&[], &[]), false);
    let (mut crew, _) = state.crew("host-a");
    assert!(let_go(&mut crew, &report(false)).is_empty());
    let places = state.file(BRANCH, "places.rec").unwrap();
    assert!(places.contains("\nRequest: 12\n"), "{places}");
}

#[test]
fn a_refused_launch_of_work_gone_goes() {
    let state = State::new();
    main_with(&state, "unit/u", &plan(&[], &[]), false);
    let (mut crew, _) = state.crew("host-a");
    let mut refused = Report::default();
    refused
        .launches
        .insert(Id::new("home/12"), launched(Outcome::Refused, None, None));
    assert_eq!(let_go(&mut crew, &refused), ["home/12"]);
}

/// A report of the planner's runs, each `k` with its place listed live
/// or answered refused.
fn runs(runs: &[(u64, bool)]) -> Report {
    let mut report = Report::default();
    for (k, live) in runs {
        let run = match live {
            true => launched(
                Outcome::Launched,
                Some("/places/crew-planner--home"),
                Some(true),
            ),
            false => launched(Outcome::Refused, None, None),
        };
        report
            .launches
            .insert(Id::new(format!("home/planner/{k}")), run);
    }
    report
}

#[test]
fn a_roles_earlier_run_once_a_later_one_is_recorded() {
    let state = State::new();
    main_with(&state, "unit/u", &plan(&["u"], &[]), true);
    let (mut crew, _) = state.crew("host-a");
    assert!(let_go(&mut crew, &runs(&[(1, false)])).is_empty());
    assert_eq!(
        let_go(&mut crew, &runs(&[(1, false), (2, true)])),
        ["home/planner/1"]
    );
    let places = state.file(BRANCH, "places.rec").unwrap();
    assert!(
        places.contains("\nAddress: home/planner\nPlace: /places/crew-planner--home\n"),
        "{places}"
    );
}

#[test]
fn a_roles_last_run_once_its_role_has_gone_and_its_place_is_over() {
    let state = State::new();
    main_with(&state, "unit/u", &plan(&["u"], &[]), false);
    let (mut crew, _) = state.crew("host-a");
    // Its place still live, it stays.
    assert!(let_go(&mut crew, &runs(&[(3, true)])).is_empty());
    assert_eq!(let_go(&mut crew, &runs(&[(3, false)])), ["home/planner/3"]);
    let places = state.file(BRANCH, "places.rec").unwrap();
    assert!(!places.contains("Address:"), "{places}");
}

#[test]
fn a_record_of_a_kind_crew_writes_is_left_as_it_stands() {
    // A work name no plan record holds keeps its record.
    let state = State::new();
    state.main(&[
        (
            "requests.rec",
            &file(
                REQUESTS,
                &[rec(&[
                    ("Request", "12"),
                    ("Act", "launch"),
                    ("Host", "host-a"),
                    ("Work", "role/planner/record-0053"),
                    ("Repo", "example/crew"),
                    ("Kind", "chore"),
                    ("Name", "record-0053"),
                    ("Base", "main"),
                    ("Date", "2026-11-01T09:00:00Z"),
                    ("By", "crew@host-a"),
                ])],
            ),
        ),
        ("plan.rec", &plan(&[], &[])),
    ]);
    let (mut crew, _) = state.crew("host-a");
    let mut report = Report::default();
    report.launches.insert(
        Id::new("home/12"),
        launched(Outcome::Launched, Some(PLACE), Some(false)),
    );
    assert!(let_go(&mut crew, &report).is_empty());
}
