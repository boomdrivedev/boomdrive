//! crew's records read at one tip of each partition's drive branch, and
//! each request for this host given as a want under the lease number 1
//! (design Decision 17; Task notes 8.3).

mod common;

use boomdrive_control::{
    Act, ActSort, Answer, At, ControlPlane, Id, Launch, Launched, Lease, Offer, Outcome, Preview,
    Report, Want,
};
use boomdrive_crew::read::LEASE;
use common::{
    act, commit_to, file, git, hello, ids, launch, plan, preview, pull, want, State, REQUESTS,
};
use std::path::Path;

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
    // The runner launched 12, so the drop naming it leaves it given.
    let mut report = Report::default();
    report.launches.insert(
        Id::new("home/12"),
        Launched {
            lease: LEASE,
            repository: None,
            answer: Answer {
                outcome: Outcome::Launched,
                place_dir: Some("/places/change--a".to_string()),
                words: None,
                at: At::parse("2026-11-01T10:00:00Z").unwrap(),
            },
            status: None,
            entry: None,
        },
    );
    let pulled = crew.pull(&kept(), &report).expect("a pull");
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

/// The crew-control-plane spec's "A land": an act landing the launch its
/// `Of` names, under lease 1 (core.53; crew record 0056).
#[test]
fn a_land_is_an_act_on_the_launch_it_names() {
    let state = State::new();
    state.main(&[
        (
            "requests.rec",
            &file(
                REQUESTS,
                &[
                    launch(12, Some("host-a"), "unit/a", CREW, "a", &[]),
                    act(17, "land", Some("host-a"), "unit/a", 12, &[]),
                ],
            ),
        ),
        ("plan.rec", &plan(&["a"], &[])),
    ]);
    let (mut crew, _) = state.crew("host-a");
    let pulled = pull(&mut crew, &kept());
    assert_eq!(
        pulled.wants[1..],
        [Want::Act(Act {
            id: Id::new("home/17"),
            launch: Id::new("home/12"),
            lease: 1,
            what: ActSort::Land,
        })]
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

/// The crew-control-plane spec's "One partition missing", over parts:
/// `work`'s state repository holds no `work/main`, and `home`'s part gives
/// `home`'s requests while `work`'s is unread, naming `work/main`, each
/// saying through its own say.
#[test]
fn one_partition_missing() {
    let state = State::new();
    let home = state.make("example/home-state");
    state.make("example/work-state");
    commit_to(
        &home,
        "home/main",
        &[
            (
                "requests.rec",
                &file(
                    REQUESTS,
                    &[launch(1, Some("host-a"), "unit/a", CREW, "a", &[])],
                ),
            ),
            ("plan.rec", &plan(&["a"], &[])),
        ],
    );
    let (mut parts, saids) = state.parts(
        "host-a",
        &[
            ("home", "example/home-state"),
            ("work", "example/work-state"),
        ],
    );
    let none = Report::default();
    let pulled = parts[0].plane.pull(&kept(), &none).expect("home's pull");
    assert_eq!(ids(&pulled), ["home/1"]);
    let unread = parts[1]
        .plane
        .pull(&kept(), &none)
        .expect_err("work's pull unread");
    assert!(unread.0.contains("work/main"), "{unread}");
    let work = saids.of("work");
    assert!(
        work.holds(&["work/main", "example/work-state", "could not be read"]),
        "{:?}",
        work.lines()
    );
    let home = saids.of("home").lines();
    assert!(home.iter().all(|line| !line.contains("work")), "{home:?}");
}

/// The crew-control-plane spec's "No partition read", over parts: every
/// part is unread, each naming its own.
#[test]
fn no_partition_read() {
    let state = State::new();
    let (mut parts, _) = state.parts(
        "host-a",
        &[("home", "example/state"), ("work", "example/nowhere")],
    );
    for part in &mut parts {
        let unread = part
            .plane
            .pull(&kept(), &Report::default())
            .expect_err("an unread pull");
        let own = format!("{}/main", part.label);
        assert!(unread.0.contains(&own), "{unread}");
        let other = if part.label == "home" {
            "work/"
        } else {
            "home/"
        };
        assert!(!unread.0.contains(other), "{unread}");
    }
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

/// `example/state` on the scratch forge, its `home/main` and `work/main`
/// each holding a launch request for `host-a`, numbered 1, of a unit the
/// partition's plan holds.
fn two_drives(state: &State) -> std::path::PathBuf {
    let bare = state.make("example/state");
    for label in ["home", "work"] {
        commit_to(
            &bare,
            &format!("{label}/main"),
            &[
                (
                    "requests.rec",
                    &file(
                        REQUESTS,
                        &[launch(1, Some("host-a"), "unit/a", CREW, label, &[])],
                    ),
                ),
                ("plan.rec", &plan(&["a"], &[])),
            ],
        );
    }
    bare
}

/// Every ref the bare cache at `cache` holds.
fn refs_of(cache: &Path) -> String {
    git(cache, &["for-each-ref", "--format=%(refname)"], None)
}

/// The crew-control-plane spec's "Two partitions of one state
/// repository": each partition reads through a bare cache of its own, and
/// neither writes in the other's.
#[test]
fn two_partitions_of_one_state_repository() {
    let state = State::new();
    two_drives(&state);
    let (mut crew, _) = state.crew_serving(
        "host-a",
        &[("home", "example/state"), ("work", "example/state")],
    );
    assert_eq!(ids(&pull(&mut crew, &kept())), ["home/1", "work/1"]);
    let caches = state.root.join("data-host-a/control/crew/example");
    let (home, work) = (caches.join("state/home.git"), caches.join("state/work.git"));
    assert!(home.is_dir() && work.is_dir());
    assert!(!caches.join("state.git").exists());
    let (home_refs, work_refs) = (refs_of(&home), refs_of(&work));
    assert!(home_refs.contains("home/main"), "{home_refs}");
    assert!(!home_refs.contains("work/"), "{home_refs}");
    assert!(work_refs.contains("work/main"), "{work_refs}");
    assert!(!work_refs.contains("home/"), "{work_refs}");
}

/// The crew-control-plane spec's "The shared cache of a release before":
/// opened with the cache a release before shared among `example/state`'s
/// partitions there, crew's implementation removes it, one line names it,
/// and each partition's pull fetches into its own cache.
#[test]
fn the_shared_cache_of_a_release_before() {
    let state = State::new();
    two_drives(&state);
    let caches = state.root.join("data-host-a/control/crew/example");
    let shared = caches.join("state.git");
    std::fs::create_dir_all(&shared).unwrap();
    git(&shared, &["init", "--quiet", "--bare"], None);
    let (mut crew, said) = state.crew_serving(
        "host-a",
        &[("home", "example/state"), ("work", "example/state")],
    );
    assert!(!shared.exists());
    let lines = said.lines();
    assert_eq!(lines.len(), 1, "{lines:?}");
    assert!(
        lines[0].starts_with(&format!("crew: removed {}, ", shared.display())),
        "{lines:?}"
    );
    assert_eq!(ids(&pull(&mut crew, &kept())), ["home/1", "work/1"]);
    assert!(caches.join("state/home.git").is_dir());
    assert!(caches.join("state/work.git").is_dir());
    assert!(!shared.exists());
}

/// The control-plane-interface spec's "crew's partitions as parts": a
/// table naming `home` and `work` opens two parts, `home` then `work`,
/// each pulling its own partition alone.
#[test]
fn crews_partitions_as_parts() {
    let state = State::new();
    two_drives(&state);
    let (mut parts, _) = state.parts(
        "host-a",
        &[("home", "example/state"), ("work", "example/state")],
    );
    let labels: Vec<&str> = parts.iter().map(|part| part.label.as_str()).collect();
    assert_eq!(labels, ["home", "work"]);
    let none = Report::default();
    for part in &mut parts {
        let pulled = part.plane.pull(&kept(), &none).expect("a pull");
        assert_eq!(ids(&pulled), [format!("{}/1", part.label)]);
    }
}

/// The control-plane-interface spec's "crew's ids by partition": each
/// part owns its own launch numbered 1, and not the other's.
#[test]
fn crews_ids_by_partition() {
    let state = State::new();
    two_drives(&state);
    let (parts, _) = state.parts(
        "host-a",
        &[("home", "example/state"), ("work", "example/state")],
    );
    let (home, work) = (&parts[0].plane, &parts[1].plane);
    let (home_1, work_1) = (Id::new("home/1"), Id::new("work/1"));
    assert!(home.owns(&home_1) && !home.owns(&work_1));
    assert!(work.owns(&work_1) && !work.owns(&home_1));
}

/// The crew-control-plane spec's "One partition missing at a report",
/// over parts: `work`'s state repository holds no `work/main`, and
/// `home`'s unit `u` is tidied and gone from its plan. `home`'s part
/// writes `home/runner/host-a`, its let-go naming `u`'s launch, and
/// `work`'s part's report is unsent, naming `work/runner/host-a`.
#[test]
fn one_partition_missing_at_a_report() {
    let state = State::new();
    let home = state.make("example/home-state");
    state.make("example/work-state");
    commit_to(
        &home,
        "home/main",
        &[
            (
                "requests.rec",
                &file(
                    REQUESTS,
                    &[
                        launch(12, Some("host-a"), "unit/u", CREW, "u", &[]),
                        act(
                            13,
                            "priority",
                            Some("host-a"),
                            "unit/u",
                            12,
                            &[("Priority", "4")],
                        ),
                    ],
                ),
            ),
            ("plan.rec", &plan(&[], &["b"])),
        ],
    );
    let (mut parts, _) = state.parts(
        "host-a",
        &[
            ("home", "example/home-state"),
            ("work", "example/work-state"),
        ],
    );
    let report = tidied("home");
    parts[0].plane.pull(&kept(), &report).expect("home's pull");
    let reported = parts[0].plane.report(&report).expect("home's report");
    let let_go: Vec<String> = reported.let_go.iter().map(Id::to_string).collect();
    assert_eq!(let_go, ["home/12", "home/13"]);
    assert!(common::tip_of(&home, "home/runner/host-a").is_some());
    let none = Report::default();
    parts[1]
        .plane
        .pull(&kept(), &none)
        .expect_err("work's pull unread");
    let unsent = parts[1]
        .plane
        .report(&none)
        .expect_err("work's report unsent");
    assert!(unsent.0.contains("work/runner/host-a"), "{unsent}");
}

/// The runner's report of partition `label`'s launch 12, at a place Flume
/// lists no more, and its priority 13 done.
fn tidied(label: &str) -> Report {
    let at = |text: &str| boomdrive_control::At::parse(text).unwrap();
    let launch = Id::new(format!("{label}/12"));
    let mut report = Report::default();
    report.launches.insert(
        launch.clone(),
        boomdrive_control::Launched {
            lease: Lease {
                number: 1,
                expires: None,
            },
            repository: None,
            answer: boomdrive_control::Answer {
                outcome: boomdrive_control::Outcome::Launched,
                place_dir: Some("/places/change--u".to_string()),
                words: None,
                at: at("2026-11-01T10:00:00Z"),
            },
            status: Some(boomdrive_control::Carried {
                text: r#"{"status":{"places":[{"place":"/places/change--u","phase":"live"}],"host":{}}}"#
                    .to_string(),
                at: at("2026-11-01T10:01:00Z"),
                listed: false,
            }),
            entry: None,
        },
    );
    report.acts.insert(
        Id::new(format!("{label}/13")),
        boomdrive_control::Acted {
            launch,
            lease: 1,
            outcome: boomdrive_control::Outcome::Done,
            words: None,
            at: at("2026-11-01T10:02:00Z"),
        },
    );
    report
}

/// `home/main` holding `requests` and a plan of the unit `u`.
fn with_requests(state: &State, requests: &[String]) {
    with_requests_and_plan(state, requests, &plan(&["u"], &[]));
}

/// `home/main` holding `requests` and `plan_rec` as its plan.
fn with_requests_and_plan(state: &State, requests: &[String], plan_rec: &str) {
    state.main(&[
        ("requests.rec", &file(REQUESTS, requests)),
        ("plan.rec", plan_rec),
    ]);
}

/// The runner's report holding preview `home/30` of `R1` brought up,
/// answered `done`, its status listed where `listed`.
fn brought_up(listed: bool) -> Report {
    let at = |text: &str| At::parse(text).unwrap();
    let mut report = Report::default();
    report.previews.insert(
        Id::new("home/30"),
        boomdrive_control::Previewed {
            lease: LEASE,
            repository: CREW.to_string(),
            revision: "R1".to_string(),
            done: false,
            outcome: Outcome::Done,
            text: Some(r#"{"name":"preview/r1","state":"starting"}"#.to_string()),
            words: None,
            status: listed.then(|| boomdrive_control::Carried {
                text: r#"{"name":"preview/r1","state":"up"}"#.to_string(),
                at: at("2026-11-01T10:06:00Z"),
                listed: true,
            }),
            at: at("2026-11-01T10:05:00Z"),
        },
    );
    report
}

/// The crew-control-plane spec's "A preview".
#[test]
fn a_preview() {
    let state = State::new();
    with_requests(
        &state,
        &[preview(
            30,
            Some("host-a"),
            "unit/u",
            "crew",
            "main",
            "R1",
            false,
        )],
    );
    let (mut crew, _) = state.crew("host-a");
    let pulled = pull(&mut crew, &kept());
    assert_eq!(
        want(&pulled, "home/30"),
        &Want::Preview(Preview {
            id: Id::new("home/30"),
            lease: LEASE,
            repository: CREW.to_string(),
            branch: "main".to_string(),
            revision: "R1".to_string(),
            done: false,
        })
    );
}

/// The crew-control-plane spec's "A preview marked done": given while
/// the runner holds the bring-up it puts away.
#[test]
fn a_preview_marked_done() {
    let state = State::new();
    with_requests(
        &state,
        &[
            preview(30, Some("host-a"), "unit/u", "crew", "main", "R1", false),
            preview(31, Some("host-a"), "unit/u", "crew", "main", "R1", true),
        ],
    );
    let (mut crew, _) = state.crew("host-a");
    let pulled = crew.pull(&kept(), &brought_up(false)).expect("a pull");
    assert_eq!(ids(&pulled), ["home/30", "home/31"]);
    let Want::Preview(done) = want(&pulled, "home/31") else {
        panic!("not a preview")
    };
    assert!(done.done);
    assert_eq!(
        (
            done.repository.as_str(),
            done.branch.as_str(),
            done.revision.as_str()
        ),
        (CREW, "main", "R1")
    );
}

/// The crew-control-plane spec's "A preview and its done the runner does
/// not hold": crew put the preview away before the runner answered it,
/// so neither is given.
#[test]
fn a_preview_and_its_done_the_runner_does_not_hold() {
    let state = State::new();
    with_requests(
        &state,
        &[
            preview(30, Some("host-a"), "unit/u", "crew", "main", "R1", false),
            preview(31, Some("host-a"), "unit/u", "crew", "main", "R1", true),
        ],
    );
    let (mut crew, _) = state.crew("host-a");
    let pulled = pull(&mut crew, &kept());
    assert!(pulled.wants.is_empty(), "{:?}", ids(&pulled));
}

/// The crew-control-plane spec's "A done of a landed bolt's preview": the
/// done is given though the bolt has left the plan, while the runner
/// holds the bring-up it puts away.
#[test]
fn a_done_of_a_landed_bolts_preview() {
    let state = State::new();
    with_requests_and_plan(
        &state,
        &[
            preview(30, Some("host-a"), "bolt/y", "crew", "main", "R1", false),
            preview(31, Some("host-a"), "bolt/y", "crew", "main", "R1", true),
        ],
        &plan(&[], &[]),
    );
    let (mut crew, _) = state.crew("host-a");
    let pulled = crew.pull(&kept(), &brought_up(true)).expect("a pull");
    assert_eq!(ids(&pulled), ["home/30", "home/31"]);
    let Want::Preview(done) = want(&pulled, "home/31") else {
        panic!("not a preview")
    };
    assert!(done.done);
}

/// The crew-control-plane spec's "A done written before a later
/// bring-up": the done is no longer crew's newest of its repository and
/// revision, so it is not given, and would not put the later bring-up
/// away.
#[test]
fn a_done_of_a_preview_written_before_a_later_bring_up() {
    let state = State::new();
    with_requests(
        &state,
        &[
            preview(30, Some("host-a"), "unit/u", "crew", "main", "R1", false),
            preview(31, Some("host-a"), "unit/u", "crew", "main", "R1", true),
            preview(32, Some("host-a"), "unit/u", "crew", "main", "R1", false),
        ],
    );
    let (mut crew, _) = state.crew("host-a");
    let pulled = crew.pull(&kept(), &brought_up(false)).expect("a pull");
    assert_eq!(ids(&pulled), ["home/30", "home/32"]);
}

/// The crew-control-plane spec's "A preview of a bolt gone that the
/// runner does not hold".
#[test]
fn a_preview_of_a_bolt_gone_that_the_runner_does_not_hold() {
    let state = State::new();
    with_requests_and_plan(
        &state,
        &[preview(
            30,
            Some("host-a"),
            "bolt/y",
            "crew",
            "main",
            "R1",
            false,
        )],
        &plan(&[], &[]),
    );
    let (mut crew, _) = state.crew("host-a");
    let pulled = pull(&mut crew, &kept());
    assert!(pulled.wants.is_empty(), "{:?}", ids(&pulled));
}

/// The crew-control-plane spec's "A preview with no revision": given as
/// one the runner refuses, its words naming `Revision`; and one whose
/// `Done` is no yes or no likewise.
#[test]
fn a_preview_with_no_revision() {
    let state = State::new();
    let no_revision = common::rec(&[
        ("Request", "32"),
        ("Act", "preview"),
        ("Host", "host-a"),
        ("Work", "unit/u"),
        ("Repo", "crew"),
        ("Branch", "main"),
        ("Date", "2026-11-01T09:00:00Z"),
        ("By", "crew@host-a"),
    ]);
    let done_maybe = preview(34, Some("host-a"), "unit/u", "crew", "main", "R1", false)
        .replace("Date:", "Done: maybe\nDate:");
    with_requests(&state, &[no_revision, done_maybe]);
    let (mut crew, _) = state.crew("host-a");
    let pulled = pull(&mut crew, &kept());
    let Want::Act(refused) = want(&pulled, "home/32") else {
        panic!("not one the runner refuses")
    };
    assert_eq!(refused.launch.as_str(), "request 32 (it names no Revision)");
    let Want::Act(refused) = want(&pulled, "home/34") else {
        panic!("not one the runner refuses")
    };
    assert_eq!(
        refused.launch.as_str(),
        "request 34 (its Done maybe, which is no yes or no)"
    );
}

/// The crew-control-plane spec's "A preview naming no host".
#[test]
fn a_preview_naming_no_host() {
    let state = State::new();
    with_requests(
        &state,
        &[preview(33, None, "unit/u", CREW, "main", "R1", false)],
    );
    let (mut crew, said) = state.crew("host-a");
    let pulled = pull(&mut crew, &kept());
    assert!(pulled.wants.is_empty(), "{:?}", ids(&pulled));
    crew.report(&Report::default()).expect("a report");
    let answers = state
        .file("home/runner/host-a", "answers.rec")
        .unwrap_or_default();
    assert!(!answers.contains("Request: 33"), "{answers}");
    assert!(
        said.lines().contains(
            &"crew: home/33: a preview names no Host, so no runner brings it up: \
              it is given nothing"
                .to_string()
        ),
        "{:?}",
        said.lines()
    );
}
