//! A launch naming no host, claimed on the runner's own branch before it
//! is given, and given only once no other runner's branch answers it
//! (design Decision 18; Task notes 8.5). Two implementations, `host-a` and
//! `host-b`, run over one bare state repository.

mod common;

use boomdrive_control::{Hello, Want};
use boomdrive_crew::read::{host_read, RunnerFiles};
use boomdrive_crew::write;
use common::{act, file, hello, ids, launch, plan, pull, want, State, REQUESTS};
use std::collections::BTreeMap;
use std::sync::{Arc, Barrier};
use weir_capture::rec::Record;

const CREW: &str = "example/crew";

fn kept() -> Hello {
    hello(&[CREW, "agentplot/flume"], "4")
}

/// `home/main` holding `requests`, its plan holding unit `a`.
fn main_holding(state: &State, requests: &[String]) {
    state.main(&[
        ("requests.rec", &file(REQUESTS, requests)),
        ("plan.rec", &plan(&["a"], &[])),
    ]);
}

/// Kit launch 20 of unit `a` in `repo`, naming no host or stream.
fn kit(repo: &str) -> String {
    launch(20, None, "unit/a", repo, "a", &[])
}

/// `host`'s runner's branch as its runner writes it, answering `answers`.
fn answering(state: &State, host: &str, answers: &[(u64, Record)]) {
    let answers: BTreeMap<u64, Record> = answers.iter().cloned().collect();
    state.commit(
        &format!("home/runner/{host}"),
        &[
            (
                "runner.rec",
                &write::runner_text(host, "boomdrive 0.0.0", "2026-11-01T09:00:00Z"),
            ),
            ("answers.rec", &write::answers_text(&answers)),
            (
                "places.rec",
                &write::places_text(&BTreeMap::new(), &BTreeMap::new()),
            ),
        ],
    );
}

/// `host`'s claim of request `n`.
fn claim(n: u64, host: &str) -> (u64, Record) {
    (n, write::claim(n, host, "2026-11-01T09:30:00Z"))
}

/// Request `n` answered launched by a host's runner.
fn launched(n: u64) -> (u64, Record) {
    let mut record = Record::new();
    record.push("Request", n.to_string());
    record.push("Outcome", "launched");
    record.push("Place", "/places/change--a");
    record.push("At", "2026-11-01T09:40:00Z");
    (n, record)
}

/// `host`'s branch as crew reads it: its answers, by request.
fn answers(state: &State, host: &str) -> BTreeMap<u64, Record> {
    let branch = format!("home/runner/{host}");
    let mut files = RunnerFiles::new();
    for path in ["runner.rec", "answers.rec", "places.rec"] {
        files.insert(path, state.file(&branch, path));
    }
    host_read(host, &files).expect("crew reads it").answers
}

#[test]
fn claimed_alone() {
    let state = State::new();
    main_holding(&state, &[kit(CREW)]);
    answering(&state, "host-b", &[]);
    let (mut host_a, said) = state.crew("host-a");
    let pulled = pull(&mut host_a, &kept());
    assert_eq!(ids(&pulled), ["home/20"]);
    let claimed = &answers(&state, "host-a")[&20];
    assert_eq!(claimed.get("Outcome"), Some("held"));
    assert_eq!(
        claimed.get("Words"),
        Some("claimed by host-a's runner; launching once no other runner has claimed it")
    );
    let message = common::git(
        &state.bare(),
        &["log", "-1", "--format=%s", "home/runner/host-a"],
        None,
    );
    assert_eq!(message.trim(), "claims 20");
    assert!(
        said.holds(&["home/runner/host-a: claims 20"]),
        "{:?}",
        said.lines()
    );
}

#[test]
fn a_host_whose_room_is_unbounded_claims() {
    let state = State::new();
    main_holding(&state, &[kit(CREW)]);
    let (mut host_a, _) = state.crew("host-a");
    let pulled = pull(&mut host_a, &hello(&[CREW], "null"));
    assert_eq!(ids(&pulled), ["home/20"]);
}

#[test]
fn taken_elsewhere() {
    let state = State::new();
    main_holding(&state, &[kit(CREW)]);
    answering(&state, "host-b", &[launched(20)]);
    let (mut host_a, _) = state.crew("host-a");
    let pulled = pull(&mut host_a, &kept());
    assert!(pulled.wants.is_empty(), "{:?}", ids(&pulled));
    assert_eq!(state.tip("home/runner/host-a"), None);
}

#[test]
fn another_claim_seen() {
    let state = State::new();
    main_holding(&state, &[kit(CREW)]);
    let (mut host_a, _) = state.crew("host-a");
    let bare = state.bare();
    host_a.between_claim_and_check(move || {
        let answers: BTreeMap<u64, Record> = [claim(20, "host-b")].into_iter().collect();
        common::commit_to(
            &bare,
            "home/runner/host-b",
            &[
                (
                    "runner.rec",
                    &write::runner_text("host-b", "boomdrive 0.0.0", "2026-11-01T09:00:00Z"),
                ),
                ("answers.rec", &write::answers_text(&answers)),
            ],
        );
    });
    let pulled = pull(&mut host_a, &kept());
    assert!(pulled.wants.is_empty(), "{:?}", ids(&pulled));
    assert!(!answers(&state, "host-a").contains_key(&20));
    let log = common::git(
        &state.bare(),
        &["log", "--format=%s", "home/runner/host-a"],
        None,
    );
    assert_eq!(log, "withdraws 20\nclaims 20\n");
}

#[test]
fn both_claiming_before_either_checks() {
    let state = State::new();
    main_holding(&state, &[kit(CREW)]);
    let (mut host_a, _) = state.crew("host-a");
    let (mut host_b, _) = state.crew("host-b");
    // At their first pulls, each reads the other's branch before either
    // claims, and checks only once both have claimed.
    let read = Arc::new(Barrier::new(2));
    let claimed = Arc::new(Barrier::new(2));
    for crew in [&mut host_a, &mut host_b] {
        let (read, claimed) = (read.clone(), claimed.clone());
        let (mut first_read, mut first_claim) = (true, true);
        crew.before_claim(move || {
            if std::mem::take(&mut first_read) {
                read.wait();
            }
        });
        crew.between_claim_and_check(move || {
            if std::mem::take(&mut first_claim) {
                claimed.wait();
            }
        });
    }
    let hello = kept();
    let (host_b_hello, host_a_hello) = (hello.clone(), hello.clone());
    let host_b_run = std::thread::spawn(move || {
        let pulled = pull(&mut host_b, &host_b_hello);
        (host_b, pulled)
    });
    let host_a_pulled = pull(&mut host_a, &host_a_hello);
    let (mut host_b, host_b_pulled) = host_b_run.join().unwrap();
    assert!(host_a_pulled.wants.is_empty(), "{:?}", ids(&host_a_pulled));
    assert!(host_b_pulled.wants.is_empty(), "{:?}", ids(&host_b_pulled));
    assert!(!answers(&state, "host-a").contains_key(&20));
    assert!(!answers(&state, "host-b").contains_key(&20));

    // host-a, first in name order, claims again at its next pull; host-b not
    // before the pull after its next.
    assert!(pull(&mut host_b, &hello).wants.is_empty());
    assert!(!answers(&state, "host-b").contains_key(&20));
    assert_eq!(ids(&pull(&mut host_a, &hello)), ["home/20"]);
    assert!(pull(&mut host_b, &hello).wants.is_empty());
    assert!(!answers(&state, "host-b").contains_key(&20));
}

#[test]
fn no_room_on_the_host() {
    let state = State::new();
    main_holding(&state, &[kit(CREW)]);
    let (mut host_a, _) = state.crew("host-a");
    for room in ["0", "-1"] {
        let pulled = pull(&mut host_a, &hello(&[CREW], room));
        assert!(pulled.wants.is_empty(), "{room}: {:?}", ids(&pulled));
    }
    assert_eq!(state.tip("home/runner/host-a"), None);
}

#[test]
fn a_kit_named_by_its_name_alone() {
    let state = State::new();
    main_holding(&state, &[kit("crew")]);
    let (mut host_a, _) = state.crew("host-a");
    let pulled = pull(&mut host_a, &kept());
    let Want::Offer(offer) = want(&pulled, "home/20") else {
        panic!("not an offer")
    };
    assert_eq!(offer.launch.repository, CREW);
}

#[test]
fn a_name_two_kept_addresses_share() {
    let state = State::new();
    main_holding(&state, &[kit("crew")]);
    let (mut host_a, said) = state.crew("host-a");
    let pulled = pull(&mut host_a, &hello(&[CREW, "agentplot/crew"], "4"));
    assert!(pulled.wants.is_empty(), "{:?}", ids(&pulled));
    assert_eq!(state.tip("home/runner/host-a"), None);
    assert!(
        said.holds(&["home/20", "example/crew and agentplot/crew"]),
        "{:?}",
        said.lines()
    );
}

#[test]
fn a_repository_not_kept() {
    let state = State::new();
    main_holding(&state, &[kit("agentplot/sluice")]);
    let (mut host_a, said) = state.crew("host-a");
    let pulled = pull(&mut host_a, &kept());
    assert!(pulled.wants.is_empty(), "{:?}", ids(&pulled));
    assert_eq!(state.tip("home/runner/host-a"), None);
    assert!(said.lines().is_empty(), "{:?}", said.lines());
}

/// The crew-control-plane spec's "A launch answered before its repository
/// left the hello": a kit's launch this host's branch answers, whose
/// `Repo` the hello no longer names, is given still, its `Repo` as
/// written, and an act on it with it; one no runner answers is given
/// nothing (record 0021).
#[test]
fn a_launch_answered_before_its_repository_left_the_hello() {
    let state = State::new();
    main_holding(
        &state,
        &[
            kit("old-kit"),
            launch(21, None, "unit/a", "old-kit", "b", &[]),
            act(22, "retry", None, "unit/a", 20, &[]),
        ],
    );
    answering(&state, "host-a", &[launched(20)]);
    let (mut host_a, said) = state.crew("host-a");
    // The hello names the repository under its new name alone, and the
    // host has no room.
    let pulled = pull(&mut host_a, &hello(&["example/new-kit"], "0"));
    assert_eq!(ids(&pulled), ["home/20", "home/22"]);
    let Want::Offer(offer) = want(&pulled, "home/20") else {
        panic!("not an offer")
    };
    assert_eq!(offer.launch.repository, "old-kit");
    assert_eq!(state.commits("home/runner/host-a"), 1, "nothing written");
    assert!(said.lines().is_empty(), "{:?}", said.lines());
}

/// A launch the runner's report answers is given whatever the hello
/// names, its `Repo` an address no longer kept, or a name two kept
/// addresses share (record 0021).
#[test]
fn a_launch_the_report_answers_is_given_whatever_the_hello_names() {
    use boomdrive_control::{Answer, At, ControlPlane, Id, Launched, Outcome, Report};
    let mut report = Report::default();
    report.launches.insert(
        Id::new("home/20"),
        Launched {
            lease: boomdrive_crew::read::LEASE,
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
    for (repo, kept) in [
        ("example/old-kit", vec!["example/new-kit"]),
        ("crew", vec![CREW, "agentplot/crew"]),
    ] {
        let state = State::new();
        main_holding(&state, &[kit(repo)]);
        let (mut host_a, _) = state.crew("host-a");
        let pulled = host_a.pull(&hello(&kept, "4"), &report).unwrap();
        let Want::Offer(offer) = want(&pulled, "home/20") else {
            panic!("not an offer")
        };
        assert_eq!(offer.launch.repository, repo);
        assert_eq!(state.tip("home/runner/host-a"), None, "nothing written");
    }
}

#[test]
fn a_claim_waiting_for_its_streams_room() {
    use boomdrive_control::{ControlPlane, Report};
    let state = State::new();
    main_holding(&state, &[kit(CREW)]);
    let (mut host_a, _) = state.crew("host-a");
    assert_eq!(ids(&pull(&mut host_a, &kept())), ["home/20"]);
    // The runner leaves the offer unanswered, its stream having no room.
    host_a.report(&Report::default()).expect("a report");
    assert!(write::is_claim(&answers(&state, "host-a")[&20], "host-a"));
    let commits = state.commits("home/runner/host-a");
    assert_eq!(ids(&pull(&mut host_a, &kept())), ["home/20"]);
    assert_eq!(state.commits("home/runner/host-a"), commits);
}

#[test]
fn a_claim_replaced_by_the_runners_answer() {
    use boomdrive_control::{Answer, At, ControlPlane, Id, Launched, Outcome, Report};
    let state = State::new();
    main_holding(&state, &[kit(CREW)]);
    let (mut host_a, _) = state.crew("host-a");
    pull(&mut host_a, &kept());
    let mut report = Report::default();
    report.launches.insert(
        Id::new("home/20"),
        Launched {
            lease: boomdrive_crew::read::LEASE,
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
    host_a.report(&report).expect("a report");
    assert_eq!(
        answers(&state, "host-a")[&20].get("Outcome"),
        Some("launched")
    );
    answering(&state, "host-b", &[claim(20, "host-b")]);
    // A launch the runner's report answers is given, whoever else claims.
    assert_eq!(ids(&host_a.pull(&kept(), &report).unwrap()), ["home/20"]);
}

#[test]
fn a_claim_found_again_after_a_stop() {
    let state = State::new();
    main_holding(&state, &[kit(CREW)]);
    answering(&state, "host-a", &[claim(20, "host-a")]);
    let (mut host_a, _) = state.crew("host-a");
    assert_eq!(ids(&pull(&mut host_a, &kept())), ["home/20"]);

    answering(&state, "host-b", &[claim(20, "host-b")]);
    let (mut fresh, _) = state.crew("host-a");
    let pulled = pull(&mut fresh, &kept());
    assert!(pulled.wants.is_empty(), "{:?}", ids(&pulled));
    assert!(!answers(&state, "host-a").contains_key(&20));
}

#[test]
fn an_act_naming_no_host_goes_to_the_runner_holding_its_launch() {
    let state = State::new();
    let launch_21 = launch(21, None, "unit/a", CREW, "b", &[]);
    main_holding(
        &state,
        &[
            kit(CREW),
            launch_21,
            act(22, "retry", None, "unit/a", 20, &[]),
            act(23, "priority", None, "unit/a", 21, &[("Priority", "5")]),
        ],
    );
    answering(&state, "host-b", &[launched(20)]);
    let (mut host_a, _) = state.crew("host-a");
    let pulled = pull(&mut host_a, &kept());
    // host-b holds launch 20, so its retry 22 is host-b's; host-a claims 21 and
    // takes its priority 23.
    assert_eq!(ids(&pulled), ["home/21", "home/23"]);
}

#[test]
fn another_runners_branch_that_cant_be_read_holds_back_a_claim() {
    let state = State::new();
    main_holding(&state, &[kit(CREW)]);
    state.commit(
        "home/runner/host-b",
        &[(
            "runner.rec",
            &write::runner_text("other", "x", "2026-11-01T09:00:00Z"),
        )],
    );
    let (mut host_a, said) = state.crew("host-a");
    let pulled = pull(&mut host_a, &kept());
    assert!(pulled.wants.is_empty(), "{:?}", ids(&pulled));
    assert_eq!(state.tip("home/runner/host-a"), None);
    assert!(
        said.holds(&[
            "host-b's runner can't be read",
            "runner.rec names host other"
        ]),
        "{:?}",
        said.lines()
    );
}
