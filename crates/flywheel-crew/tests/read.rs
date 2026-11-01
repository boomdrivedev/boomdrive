//! crew's records read at one tip of each partition's flywheel branch, and
//! each request for this host given as a want under the lease number 1
//! (design Decision 17; Task notes 8.3).

mod common;

use common::{act, file, hello, ids, launch, plan, pull, want, State, REQUESTS};
use flywheel_control::{Act, ActSort, ControlPlane, Id, Launch, Lease, Offer, Report, Want};

const CREW: &str = "afterthought/crew";

fn kept() -> flywheel_control::Hello {
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
                    launch(1, Some("vega"), "unit/a", CREW, "a", &[]),
                    launch(2, Some("deneb"), "unit/b", CREW, "b", &[]),
                    launch(3, Some("vega"), "unit/c", CREW, "c", &[]),
                    launch(4, Some("deneb"), "unit/d", CREW, "d", &[]),
                ],
            ),
        ),
        ("plan.rec", &plan(&["a", "b", "c", "d"], &[])),
    ]);
    let (mut crew, _) = state.crew("deneb");
    let pulled = pull(&mut crew, &kept());
    assert_eq!(ids(&pulled), ["madswan/2", "madswan/4"]);
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
                    launch(2, Some("deneb"), "unit/b", CREW, "b", &[]),
                    launch(1, Some("deneb"), "unit/a", CREW, "a", &[]),
                ],
            ),
        ),
        ("plan.rec", &plan(&["a", "b"], &[])),
    ]);
    let (mut crew, _) = state.crew("deneb");
    assert_eq!(ids(&pull(&mut crew, &kept())), ["madswan/1", "madswan/2"]);
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
                    launch(10, Some("deneb"), "unit/x", CREW, "x", &[]),
                    launch(11, Some("deneb"), "unit/y", CREW, "y", &[]),
                    launch(
                        12,
                        Some("deneb"),
                        "unit/z",
                        CREW,
                        "z",
                        &[
                            ("Stream", "madswan-main"),
                            ("Priority", "3"),
                            ("Intent", "madswan-bolt-b"),
                            ("Recipient", "madswan/planner"),
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
    let (mut crew, _) = state.crew("deneb");
    let pulled = pull(&mut crew, &kept());
    assert_eq!(
        want(&pulled, "madswan/12"),
        &Want::Offer(Offer {
            id: Id::new("madswan/12"),
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
                stream: Some("madswan-main".to_string()),
                priority: Some(3),
                intent: Some("madswan-bolt-b".to_string()),
                recipients: vec!["madswan/planner".to_string()],
                holds: vec![],
                hold_submission: true,
                after: vec![Id::new("madswan/10"), Id::new("madswan/11")],
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
                    launch(12, Some("deneb"), "unit/a", CREW, "a", &[]),
                    act(
                        13,
                        "retry",
                        Some("deneb"),
                        "unit/a",
                        12,
                        &[("Stage", "code"), ("Words", "Four.")],
                    ),
                    act(
                        14,
                        "retry",
                        Some("deneb"),
                        "unit/a",
                        12,
                        &[("Submission", "yes")],
                    ),
                    act(15, "drop", Some("deneb"), "unit/a", 12, &[]),
                    act(
                        16,
                        "priority",
                        Some("deneb"),
                        "unit/a",
                        12,
                        &[("Priority", "7")],
                    ),
                ],
            ),
        ),
        ("plan.rec", &plan(&["a"], &[])),
    ]);
    let (mut crew, _) = state.crew("deneb");
    let pulled = pull(&mut crew, &kept());
    let on_12 = |n: u64, what: ActSort| {
        Want::Act(Act {
            id: Id::new(format!("madswan/{n}")),
            launch: Id::new("madswan/12"),
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
        ("Host", "deneb"),
        ("Work", "unit/a"),
        ("Of", "12"),
        ("Date", "2026-11-01T09:00:00Z"),
        ("By", "crew@deneb"),
    ]);
    let kit_pause = common::rec(&[
        ("Request", "16"),
        ("Act", "pause"),
        ("Work", "unit/a"),
        ("Of", "12"),
        ("Date", "2026-11-01T09:00:00Z"),
        ("By", "crew@deneb"),
    ]);
    state.main(&[
        (
            "requests.rec",
            &file(
                REQUESTS,
                &[
                    launch(12, Some("deneb"), "unit/a", CREW, "a", &[]),
                    pause,
                    kit_pause,
                ],
            ),
        ),
        ("plan.rec", &plan(&["a"], &[])),
    ]);
    let (mut crew, said) = state.crew("deneb");
    let pulled = pull(&mut crew, &kept());
    assert_eq!(ids(&pulled), ["madswan/12", "madswan/15"]);
    // The runner refuses an act on a launch that is no launch of its
    // own, its words naming that launch: here, what can't be read.
    let Want::Act(paused) = want(&pulled, "madswan/15") else {
        panic!("not an act")
    };
    assert_eq!(
        paused.launch.as_str(),
        "request 15 (its Act pause, which crew's implementation does not read)"
    );
    assert!(
        said.holds(&["madswan/16", "names no Host", "Act"]),
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
                    launch(1, Some("deneb"), "unit/a", CREW, "a", &[("Hold", "code")]),
                    launch(
                        2,
                        Some("deneb"),
                        "unit/b",
                        CREW,
                        "b",
                        &[("Priority", "high")],
                    ),
                    launch(3, Some("deneb"), "unit/c", CREW, "c", &[("After", "x")]),
                ],
            ),
        ),
        ("plan.rec", &plan(&["a", "b", "c"], &[])),
    ]);
    let (mut crew, _) = state.crew("deneb");
    let pulled = pull(&mut crew, &kept());
    let launch_of = |id: &str| match want(&pulled, id) {
        Want::Act(act) => act.launch.to_string(),
        other => panic!("not refused: {other:?}"),
    };
    assert_eq!(
        launch_of("madswan/1"),
        "request 1 (its Hold code, which crew's implementation does not read)"
    );
    assert_eq!(
        launch_of("madswan/2"),
        "request 2 (its Priority high, which is no whole number)"
    );
    assert_eq!(
        launch_of("madswan/3"),
        "request 3 (its After x, which is no whole number)"
    );
}

#[test]
fn a_record_with_no_request_number_is_left_out_and_named() {
    let state = State::new();
    let unnumbered = common::rec(&[
        ("Act", "launch"),
        ("Host", "deneb"),
        ("Work", "unit/a"),
        ("Date", "2026-11-01T09:00:00Z"),
        ("By", "crew@deneb"),
    ]);
    state.main(&[
        (
            "requests.rec",
            &file(
                REQUESTS,
                &[
                    unnumbered,
                    launch(2, Some("deneb"), "unit/a", CREW, "a", &[]),
                ],
            ),
        ),
        ("plan.rec", &plan(&["a"], &[])),
    ]);
    let (mut crew, said) = state.crew("deneb");
    assert_eq!(ids(&pull(&mut crew, &kept())), ["madswan/2"]);
    assert!(
        said.holds(&["madswan/main", "no whole Request number"]),
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
                &[launch(1, Some("deneb"), "fix/main/crew/x", CREW, "x", &[])],
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
    let (mut crew, _) = state.crew("deneb");
    // No roles.rec: no role's run; no plan.rec: a fix of main is no work
    // the plan holds, so it is given.
    assert_eq!(ids(&pull(&mut crew, &kept())), ["madswan/1"]);
}

#[test]
fn one_partition_missing() {
    let state = State::new();
    state.make("afterthought/swancloud-state");
    state.main(&[
        (
            "requests.rec",
            &file(
                REQUESTS,
                &[launch(1, Some("deneb"), "unit/a", CREW, "a", &[])],
            ),
        ),
        ("plan.rec", &plan(&["a"], &[])),
    ]);
    let (mut crew, said) = state.crew_serving(
        "deneb",
        &[
            ("swancloud", "afterthought/swancloud-state"),
            ("madswan", "afterthought/madswan-state"),
        ],
    );
    let pulled = pull(&mut crew, &kept());
    assert_eq!(ids(&pulled), ["madswan/1"]);
    assert!(
        said.holds(&[
            "swancloud/main",
            "afterthought/swancloud-state",
            "could not be read"
        ]),
        "{:?}",
        said.lines()
    );
}

#[test]
fn no_partition_read() {
    let state = State::new();
    let (mut crew, _) = state.crew_serving(
        "deneb",
        &[
            ("madswan", "afterthought/madswan-state"),
            ("swancloud", "afterthought/nowhere"),
        ],
    );
    let unread = crew
        .pull(&kept(), &Report::default())
        .expect_err("an unread pull");
    assert!(unread.0.contains("madswan/main"), "{unread}");
    assert!(unread.0.contains("swancloud/main"), "{unread}");
}

/// git's words on a state repository it cannot reach run over several
/// lines; crew's implementation says them, and gives them to the runner
/// to say, on one line, each of git's lines kept.
#[test]
fn a_state_repository_that_cannot_be_reached_is_named_on_one_line() {
    let state = State::new();
    let (mut crew, said) = state.crew_serving("deneb", &[("swancloud", "afterthought/nowhere")]);
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
    let (mut crew, said) = state.crew("deneb");
    assert!(crew.pull(&kept(), &Report::default()).is_err());
    state.main(&[
        (
            "requests.rec",
            &file(
                REQUESTS,
                &[launch(1, Some("deneb"), "unit/a", CREW, "a", &[])],
            ),
        ),
        ("plan.rec", &plan(&["a"], &[])),
    ]);
    assert_eq!(ids(&pull(&mut crew, &kept())), ["madswan/1"]);
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
                    launch(1, Some("deneb"), "unit/a", "crew", "a", &[]),
                    launch(2, Some("deneb"), "unit/b", "agentplot/sluice", "b", &[]),
                ],
            ),
        ),
        ("plan.rec", &plan(&["a", "b"], &[])),
    ]);
    let (mut crew, _) = state.crew("deneb");
    let pulled = pull(&mut crew, &kept());
    let repository = |id: &str| match want(&pulled, id) {
        Want::Offer(offer) => offer.launch.repository.clone(),
        other => panic!("not an offer: {other:?}"),
    };
    assert_eq!(repository("madswan/1"), CREW);
    // A request naming this host for a repository it does not keep is the
    // runner's to refuse.
    assert_eq!(repository("madswan/2"), "agentplot/sluice");
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
    let (mut crew, _) = state.crew("deneb");
    let pulled = pull(&mut crew, &kept());
    let Want::Offer(offer) = want(&pulled, "madswan/20") else {
        panic!("not an offer")
    };
    assert_eq!(offer.launch.stream, None);
    assert_eq!(offer.launch.repository, CREW);
}
