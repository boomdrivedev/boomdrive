//! crew's records read at one tip of each partition's drive branch, and
//! each request for this host given as a want under the lease number 1
//! (design Decision 17; Task notes 8.3).

mod common;

use boomdrive_control::{Act, ActSort, ControlPlane, Id, Launch, Lease, Offer, Report, Want};
use common::{act, file, hello, ids, launch, plan, pull, want, State, REQUESTS};

const CREW: &str = "example/crew";

fn kept() -> boomdrive_control::Hello {
    hello(&[CREW, "agentplot/flume"], "4")
}

#[test]
fn only_this_hosts_requests() {
    let state = State::new();
    state.main(&[
        (
            "requests.rec",
            &file(
                REQUESTS,
                &[
                    launch(1, Some("host-b"), "unit/a", CREW, "a", &[]),
                    launch(2, Some("host-a"), "unit/b", CREW, "b", &[]),
                    launch(3, Some("host-b"), "unit/c", CREW, "c", &[]),
                    launch(4, Some("host-a"), "unit/d", CREW, "d", &[]),
                ],
            ),
        ),
        ("plan.rec", &plan(&["a", "b", "c", "d"], &[])),
    ]);
    let (mut crew, _) = state.crew("host-a");
    let pulled = pull(&mut crew, &kept());
    assert_eq!(ids(&pulled), ["home/2", "home/4"]);
    for w in &pulled.wants {
        let Want::Offer(offer) = w else {
            panic!("not an offer: {w:?}")
        };
        assert_eq!(
            offer.lease,
            Lease {
                number: 1,
                expires: None
            }
        );
    }
}

#[test]
fn requests_in_request_order_whatever_the_files() {
    let state = State::new();
    state.main(&[
        (
            "requests.rec",
            &file(
                REQUESTS,
                &[
                    launch(2, Some("host-a"), "unit/b", CREW, "b", &[]),
                    launch(1, Some("host-a"), "unit/a", CREW, "a", &[]),
                ],
            ),
        ),
        ("plan.rec", &plan(&["a", "b"], &[])),
    ]);
    let (mut crew, _) = state.crew("host-a");
    assert_eq!(ids(&pull(&mut crew, &kept())), ["home/1", "home/2"]);
}

#[test]
fn a_launchs_every_field() {
    let state = State::new();
    state.main(&[
        (
            "requests.rec",
            &file(
                REQUESTS,
                &[
                    launch(10, Some("host-a"), "unit/x", CREW, "x", &[]),
                    launch(11, Some("host-a"), "unit/y", CREW, "y", &[]),
                    launch(
                        12,
                        Some("host-a"),
                        "unit/z",
                        CREW,
                        "z",
                        &[
                            ("Stream", "home-main"),
                            ("Priority", "3"),
                            ("Intent", "home-bolt-b"),
                            ("Recipient", "home/planner"),
                            ("Hold", "submission"),
                            ("After", "10"),
                            ("After", "11"),
                            ("Brief", "Build z.\nAs its design says.\nAnd nothing more."),
                        ],
                    ),
                ],
            ),
        ),
        ("plan.rec", &plan(&["x", "y", "z"], &[])),
    ]);
    let (mut crew, _) = state.crew("host-a");
    let pulled = pull(&mut crew, &kept());
    assert_eq!(
        want(&pulled, "home/12"),
        &Want::Offer(Offer {
            id: Id::new("home/12"),
            lease: Lease {
                number: 1,
                expires: None
            },
            launch: Launch {
                repository: CREW.to_string(),
                kind: "change".to_string(),
                name: "z".to_string(),
                base: Some("bolt/b".to_string()),
                brief: Some("Build z.\nAs its design says.\nAnd nothing more.".to_string()),
                stream: Some("home-main".to_string()),
                priority: Some(3),
                intent: Some("home-bolt-b".to_string()),
                recipients: vec!["home/planner".to_string()],
                holds: vec![],
                hold_submission: true,
                after: vec![Id::new("home/10"), Id::new("home/11")],
                agent_settings: vec![],
            },
        })
    );
}

#[test]
fn a_retry_of_the_submission() {
    let state = State::new();
    state.main(&[
        (
            "requests.rec",
            &file(
                REQUESTS,
                &[
                    launch(12, Some("host-a"), "unit/a", CREW, "a", &[]),
                    act(
                        13,
                        "retry",
                        Some("host-a"),
                        "unit/a",
                        12,
                        &[("Stage", "code"), ("Words", "Four.")],
                    ),
                    act(
                        14,
                        "retry",
                        Some("host-a"),
                        "unit/a",
                        12,
                        &[("Submission", "yes")],
                    ),
                    act(15, "drop", Some("host-a"), "unit/a", 12, &[]),
                    act(
                        16,
                        "priority",
                        Some("host-a"),
                        "unit/a",
                        12,
                        &[("Priority", "7")],
                    ),
                ],
            ),
        ),
        ("plan.rec", &plan(&["a"], &[])),
    ]);
    let (mut crew, _) = state.crew("host-a");
    let pulled = pull(&mut crew, &kept());
    let on_12 = |n: u64, what: ActSort| {
        Want::Act(Act {
            id: Id::new(format!("home/{n}")),
            launch: Id::new("home/12"),
            lease: 1,
            what,
        })
    };
    assert_eq!(
        pulled.wants[1..],
        [
            on_12(
                13,
                ActSort::Retry {
                    stage: Some("code".to_string()),
                    words: Some("Four.".to_string()),
                    submission: false,
                }
            ),
            on_12(
                14,
                ActSort::Retry {
                    stage: None,
                    words: None,
                    submission: true,
                }
            ),
            on_12(15, ActSort::Drop),
            on_12(16, ActSort::Priority(7)),
        ]
    );
}

#[test]
fn an_act_crew_does_not_write() {
    let state = State::new();
    let pause = common::rec(&[
        ("Request", "15"),
        ("Act", "pause"),
        ("Host", "host-a"),
        ("Work", "unit/a"),
        ("Of", "12"),
        ("Date", "2026-11-01T09:00:00Z"),
        ("By", "crew@host-a"),
    ]);
    let kit_pause = common::rec(&[
        ("Request", "16"),
        ("Act", "pause"),
        ("Work", "unit/a"),
        ("Of", "12"),
        ("Date", "2026-11-01T09:00:00Z"),
        ("By", "crew@host-a"),
    ]);
    state.main(&[
        (
            "requests.rec",
            &file(
                REQUESTS,
                &[
                    launch(12, Some("host-a"), "unit/a", CREW, "a", &[]),
                    pause,
                    kit_pause,
                ],
            ),
        ),
        ("plan.rec", &plan(&["a"], &[])),
    ]);
    let (mut crew, said) = state.crew("host-a");
    let pulled = pull(&mut crew, &kept());
    assert_eq!(ids(&pulled), ["home/12", "home/15"]);
    // The runner refuses an act on a launch that is no launch of its
    // own, its words naming that launch: here, what can't be read.
    let Want::Act(paused) = want(&pulled, "home/15") else {
        panic!("not an act")
    };
    assert_eq!(
        paused.launch.as_str(),
        "request 15 (its Act pause, which crew's implementation does not read)"
    );
    assert!(
        said.holds(&["home/16", "names no Host", "Act"]),
        "{:?}",
        said.lines()
    );
}

#[test]
fn a_hold_or_a_number_crew_does_not_write() {
    let state = State::new();
    state.main(&[
        (
            "requests.rec",
            &file(
                REQUESTS,
                &[
                    launch(1, Some("host-a"), "unit/a", CREW, "a", &[("Hold", "code")]),
                    launch(
                        2,
                        Some("host-a"),
                        "unit/b",
                        CREW,
                        "b",
                        &[("Priority", "high")],
                    ),
                    launch(3, Some("host-a"), "unit/c", CREW, "c", &[("After", "x")]),
                ],
            ),
        ),
        ("plan.rec", &plan(&["a", "b", "c"], &[])),
    ]);
    let (mut crew, _) = state.crew("host-a");
    let pulled = pull(&mut crew, &kept());
    let launch_of = |id: &str| match want(&pulled, id) {
        Want::Act(act) => act.launch.to_string(),
        other => panic!("not refused: {other:?}"),
    };
    assert_eq!(
        launch_of("home/1"),
        "request 1 (its Hold code, which crew's implementation does not read)"
    );
    assert_eq!(
        launch_of("home/2"),
        "request 2 (its Priority high, which is no whole number)"
    );
    assert_eq!(
        launch_of("home/3"),
        "request 3 (its After x, which is no whole number)"
    );
}

#[test]
fn a_record_with_no_request_number_is_left_out_and_named() {
    let state = State::new();
    let unnumbered = common::rec(&[
        ("Act", "launch"),
        ("Host", "host-a"),
        ("Work", "unit/a"),
        ("Date", "2026-11-01T09:00:00Z"),
        ("By", "crew@host-a"),
    ]);
    state.main(&[
        (
            "requests.rec",
            &file(
                REQUESTS,
                &[
                    unnumbered,
                    launch(2, Some("host-a"), "unit/a", CREW, "a", &[]),
                ],
            ),
        ),
        ("plan.rec", &plan(&["a"], &[])),
    ]);
    let (mut crew, said) = state.crew("host-a");
    assert_eq!(ids(&pull(&mut crew, &kept())), ["home/2"]);
    assert!(
        said.holds(&["home/main", "no whole Request number"]),
        "{:?}",
        said.lines()
    );
}

#[test]
fn a_file_missing_at_the_tip() {
    let state = State::new();
    state.main(&[
        (
            "requests.rec",
            &file(
                REQUESTS,
                &[launch(1, Some("host-a"), "fix/main/crew/x", CREW, "x", &[])],
            ),
        ),
        (
            "asks.rec",
            &file(
                common::ASKS,
                &[common::rec(&[
                    ("Id", "1"),
                    ("Role", "planner"),
                    ("From", "crew"),
                    ("What", "tell"),
                    ("At", "2026-11-01T09:00:00Z"),
                ])],
            ),
        ),
    ]);
    let (mut crew, _) = state.crew("host-a");
    // No roles.rec: no role's run; no plan.rec: a fix of main is no work
    // the plan holds, so it is given.
    assert_eq!(ids(&pull(&mut crew, &kept())), ["home/1"]);
}

#[test]
fn one_partition_missing() {
    let state = State::new();
    state.make("example/work-state");
    state.main(&[
        (
            "requests.rec",
            &file(
                REQUESTS,
                &[launch(1, Some("host-a"), "unit/a", CREW, "a", &[])],
            ),
        ),
        ("plan.rec", &plan(&["a"], &[])),
    ]);
    let (mut crew, said) = state.crew_serving(
        "host-a",
        &[
            ("work", "example/work-state"),
            ("home", "example/home-state"),
        ],
    );
    let pulled = pull(&mut crew, &kept());
    assert_eq!(ids(&pulled), ["home/1"]);
    assert!(
        said.holds(&["work/main", "example/work-state", "could not be read"]),
        "{:?}",
        said.lines()
    );
}

#[test]
fn no_partition_read() {
    let state = State::new();
    let (mut crew, _) = state.crew_serving(
        "host-a",
        &[("home", "example/home-state"), ("work", "example/nowhere")],
    );
    let unread = crew
        .pull(&kept(), &Report::default())
        .expect_err("an unread pull");
    assert!(unread.0.contains("home/main"), "{unread}");
    assert!(unread.0.contains("work/main"), "{unread}");
}

/// git's words on a state repository it cannot reach run over several
/// lines; crew's implementation says them, and gives them to the runner
/// to say, on one line, each of git's lines kept.
#[test]
fn a_state_repository_that_cannot_be_reached_is_named_on_one_line() {
    let state = State::new();
    let (mut crew, said) = state.crew_serving("host-a", &[("work", "example/nowhere")]);
    let unread = crew
        .pull(&kept(), &Report::default())
        .expect_err("an unread pull");
    assert!(!unread.0.contains('\n'), "{unread}");
    assert!(
        unread
            .0
            .contains("does not appear to be a git repository; "),
        "{unread}"
    );
    for line in said.lines() {
        assert!(!line.contains('\n'), "{line}");
    }
}

#[test]
fn a_partition_read_again_once_it_can_be() {
    let state = State::new();
    let (mut crew, said) = state.crew("host-a");
    assert!(crew.pull(&kept(), &Report::default()).is_err());
    state.main(&[
        (
            "requests.rec",
            &file(
                REQUESTS,
                &[launch(1, Some("host-a"), "unit/a", CREW, "a", &[])],
            ),
        ),
        ("plan.rec", &plan(&["a"], &[])),
    ]);
    assert_eq!(ids(&pull(&mut crew, &kept())), ["home/1"]);
    assert_eq!(said.lines().len(), 1, "{:?}", said.lines());
}

#[test]
fn a_repo_by_its_name_alone_on_a_request_naming_this_host() {
    let state = State::new();
    state.main(&[
        (
            "requests.rec",
            &file(
                REQUESTS,
                &[
                    launch(1, Some("host-a"), "unit/a", "crew", "a", &[]),
                    launch(2, Some("host-a"), "unit/b", "agentplot/sluice", "b", &[]),
                ],
            ),
        ),
        ("plan.rec", &plan(&["a", "b"], &[])),
    ]);
    let (mut crew, _) = state.crew("host-a");
    let pulled = pull(&mut crew, &kept());
    let repository = |id: &str| match want(&pulled, id) {
        Want::Offer(offer) => offer.launch.repository.clone(),
        other => panic!("not an offer: {other:?}"),
    };
    assert_eq!(repository("home/1"), CREW);
    // A request naming this host for a repository it does not keep is the
    // runner's to refuse.
    assert_eq!(repository("home/2"), "agentplot/sluice");
}

#[test]
fn a_kits_launch_naming_no_stream() {
    let state = State::new();
    state.main(&[
        (
            "requests.rec",
            &file(REQUESTS, &[launch(20, None, "unit/a", CREW, "a", &[])]),
        ),
        ("plan.rec", &plan(&["a"], &[])),
    ]);
    let (mut crew, _) = state.crew("host-a");
    let pulled = pull(&mut crew, &kept());
    let Want::Offer(offer) = want(&pulled, "home/20") else {
        panic!("not an offer")
    };
    assert_eq!(offer.launch.stream, None);
    assert_eq!(offer.launch.repository, CREW);
}
