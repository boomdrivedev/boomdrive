//! A priority given only while it is its launch's newest, or while the
//! runner holds it (record 0002 as amended on 2026-11-16; the change
//! the-pull-gives-only-a-launchs-newest-priority). Each test runs over one
//! bare state repository, `host-a` its host.

mod common;

use boomdrive_control::{
    ActSort, Acted, Answer, At, ControlPlane, Hello, Id, Launched, Outcome, Report, Want,
};
use boomdrive_crew::read::LEASE;
use common::{act, file, hello, ids, launch, plan, State, REQUESTS};

const CREW: &str = "example/crew";

fn kept() -> Hello {
    hello(&[CREW], "4")
}

fn at(text: &str) -> At {
    At::parse(text).unwrap()
}

/// `home/main` holding `requests`, its plan holding unit `u`.
fn main_holding(state: &State, requests: &[String]) {
    state.main(&[
        ("requests.rec", &file(REQUESTS, requests)),
        ("plan.rec", &plan(&["u"], &[])),
    ]);
}

/// A priority request `n` of launch `of`, setting `priority`, for `host`
/// where one is named.
fn priority(n: u64, host: Option<&str>, of: u64, priority: &str) -> String {
    act(n, "priority", host, "unit/u", of, &[("Priority", priority)])
}

/// The runner's report answering each of `launches` launched.
fn launched(launches: &[u64]) -> Report {
    let mut report = Report::default();
    for n in launches {
        report.launches.insert(
            Id::new(format!("home/{n}")),
            Launched {
                lease: LEASE,
                repository: None,
                answer: Answer {
                    outcome: Outcome::Launched,
                    place_dir: Some(format!("/places/change--{n}")),
                    words: None,
                    at: at("2026-11-01T10:00:00Z"),
                },
                status: None,
                entry: None,
                entry_refused: None,
            },
        );
    }
    report
}

/// `report` answering act `n` on launch `of` done.
fn acted(mut report: Report, n: u64, of: u64) -> Report {
    report.acts.insert(
        Id::new(format!("home/{n}")),
        Acted {
            launch: Id::new(format!("home/{of}")),
            lease: LEASE.number,
            outcome: Outcome::Done,
            words: None,
            at: at("2026-11-01T10:02:00Z"),
        },
    );
    report
}

/// The priority the act `id` sets.
fn set(pulled: &boomdrive_control::Pulled, id: &str) -> i64 {
    match common::want(pulled, id) {
        Want::Act(act) => match act.what {
            ActSort::Priority(p) => p,
            ref other => panic!("{id} is {other:?}"),
        },
        other => panic!("{id} is {other:?}"),
    }
}

/// The crew-control-plane spec's "Moot priorities the runner does not
/// hold": of launch 12's three priorities, only the newest is given, and
/// launch 13's one priority beside it.
#[test]
fn moot_priorities_the_runner_does_not_hold() {
    let state = State::new();
    main_holding(
        &state,
        &[
            launch(12, Some("host-a"), "unit/u", CREW, "u", &[]),
            launch(13, Some("host-a"), "unit/u", CREW, "v", &[]),
            priority(14, Some("host-a"), 12, "3"),
            priority(15, Some("host-a"), 12, "5"),
            priority(16, Some("host-a"), 13, "2"),
            priority(17, Some("host-a"), 12, "7"),
        ],
    );
    let (mut host_a, _) = state.crew("host-a");
    let pulled = host_a.pull(&kept(), &launched(&[12, 13])).expect("a pull");
    assert_eq!(ids(&pulled), ["home/12", "home/13", "home/16", "home/17"]);
    assert_eq!(set(&pulled, "home/16"), 2);
    assert_eq!(set(&pulled, "home/17"), 7);
}

/// "A priority the runner holds": the runner's report answers 14, so it
/// is given still, to be answered from the report, and 15, which no one
/// answers and 16 makes moot, is not.
#[test]
fn a_priority_the_runner_holds() {
    let state = State::new();
    main_holding(
        &state,
        &[
            launch(12, Some("host-a"), "unit/u", CREW, "u", &[]),
            priority(14, Some("host-a"), 12, "3"),
            priority(15, Some("host-a"), 12, "5"),
            priority(16, Some("host-a"), 12, "7"),
        ],
    );
    let (mut host_a, _) = state.crew("host-a");
    let report = acted(launched(&[12]), 14, 12);
    let pulled = host_a.pull(&kept(), &report).expect("a pull");
    assert_eq!(ids(&pulled), ["home/12", "home/14", "home/16"]);
}

/// "A later priority that cannot be read": 15's `Priority` is no whole
/// number, so 14 is still its launch's newest, and 15 is given as one
/// the runner refuses.
#[test]
fn a_later_priority_that_cannot_be_read() {
    let state = State::new();
    main_holding(
        &state,
        &[
            launch(12, Some("host-a"), "unit/u", CREW, "u", &[]),
            priority(14, Some("host-a"), 12, "3"),
            priority(15, Some("host-a"), 12, "high"),
        ],
    );
    let (mut host_a, _) = state.crew("host-a");
    let pulled = host_a.pull(&kept(), &launched(&[12])).expect("a pull");
    assert_eq!(ids(&pulled), ["home/12", "home/14", "home/15"]);
    assert_eq!(set(&pulled, "home/14"), 3);
    match common::want(&pulled, "home/15") {
        Want::Act(act) => assert!(
            act.launch.as_str().starts_with("request 15 (") && act.launch.as_str().contains("high"),
            "{act:?}"
        ),
        other => panic!("home/15 is {other:?}"),
    }
}

/// "Moot priorities naming no host": the runner holds kit launch 20, so
/// its priorities naming no host are its, and only the newest is given.
#[test]
fn moot_priorities_naming_no_host() {
    let state = State::new();
    main_holding(
        &state,
        &[
            launch(20, None, "unit/u", CREW, "u", &[]),
            priority(21, None, 20, "3"),
            priority(22, None, 20, "5"),
        ],
    );
    let (mut host_a, _) = state.crew("host-a");
    let pulled = host_a.pull(&kept(), &launched(&[20])).expect("a pull");
    assert_eq!(ids(&pulled), ["home/20", "home/22"]);
    assert_eq!(set(&pulled, "home/22"), 5);
}

/// The brief's case: a launch whose priority crew appended at every
/// start or reorder, hundreds of times, is given its newest alone.
#[test]
fn a_launch_reordered_many_times_is_given_its_newest_priority_alone() {
    let state = State::new();
    let mut requests = vec![launch(12, Some("host-a"), "unit/u", CREW, "u", &[])];
    requests.extend((13..=400).map(|n| priority(n, Some("host-a"), 12, &n.to_string())));
    main_holding(&state, &requests);
    let (mut host_a, _) = state.crew("host-a");
    let pulled = host_a.pull(&kept(), &launched(&[12])).expect("a pull");
    assert_eq!(ids(&pulled), ["home/12", "home/400"]);
    assert_eq!(set(&pulled, "home/400"), 400);
}
