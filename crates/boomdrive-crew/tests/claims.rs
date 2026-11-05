//! A launch naming no host, claimed on the runner's own branch before it
//! is given, and given only once no other runner's branch answers it
//! (design Decision 18; Task notes 8.5). Two implementations, `deneb` and
//! `vega`, run over one bare state repository.

mod common;

use boomdrive_control::{Hello, Want};
use boomdrive_crew::read::{host_read, RunnerFiles};
use boomdrive_crew::write;
use common::{act, file, hello, ids, launch, plan, pull, want, State, REQUESTS};
use std::collections::BTreeMap;
use std::sync::{Arc, Barrier};
use weir_capture::rec::Record;

const CREW: &str = "afterthought/crew";

fn kept() -> Hello {
    hello(&[CREW, "agentplot/flume"], "4")
}

/// `madswan/main` holding `requests`, its plan holding unit `a`.
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
        &format!("madswan/runner/{host}"),
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
    let branch = format!("madswan/runner/{host}");
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
    answering(&state, "vega", &[]);
    let (mut deneb, said) = state.crew("deneb");
    let pulled = pull(&mut deneb, &kept());
    assert_eq!(ids(&pulled), ["madswan/20"]);
    let claimed = &answers(&state, "deneb")[&20];
    assert_eq!(claimed.get("Outcome"), Some("held"));
    assert_eq!(
        claimed.get("Words"),
        Some("claimed by deneb's runner; launching once no other runner has claimed it")
    );
    let message = common::git(
        &state.bare(),
        &["log", "-1", "--format=%s", "madswan/runner/deneb"],
        None,
    );
    assert_eq!(message.trim(), "claims 20");
    assert!(
        said.holds(&["madswan/runner/deneb: claims 20"]),
        "{:?}",
        said.lines()
    );
}

#[test]
fn a_host_whose_room_is_unbounded_claims() {
    let state = State::new();
    main_holding(&state, &[kit(CREW)]);
    let (mut deneb, _) = state.crew("deneb");
    let pulled = pull(&mut deneb, &hello(&[CREW], "null"));
    assert_eq!(ids(&pulled), ["madswan/20"]);
}

#[test]
fn taken_elsewhere() {
    let state = State::new();
    main_holding(&state, &[kit(CREW)]);
    answering(&state, "vega", &[launched(20)]);
    let (mut deneb, _) = state.crew("deneb");
    let pulled = pull(&mut deneb, &kept());
    assert!(pulled.wants.is_empty(), "{:?}", ids(&pulled));
    assert_eq!(state.tip("madswan/runner/deneb"), None);
}

#[test]
fn another_claim_seen() {
    let state = State::new();
    main_holding(&state, &[kit(CREW)]);
    let (mut deneb, _) = state.crew("deneb");
    let bare = state.bare();
    deneb.between_claim_and_check(move || {
        let answers: BTreeMap<u64, Record> = [claim(20, "vega")].into_iter().collect();
        common::commit_to(
            &bare,
            "madswan/runner/vega",
            &[
                (
                    "runner.rec",
                    &write::runner_text("vega", "boomdrive 0.0.0", "2026-11-01T09:00:00Z"),
                ),
                ("answers.rec", &write::answers_text(&answers)),
            ],
        );
    });
    let pulled = pull(&mut deneb, &kept());
    assert!(pulled.wants.is_empty(), "{:?}", ids(&pulled));
    assert!(!answers(&state, "deneb").contains_key(&20));
    let log = common::git(
        &state.bare(),
        &["log", "--format=%s", "madswan/runner/deneb"],
        None,
    );
    assert_eq!(log, "withdraws 20\nclaims 20\n");
}

#[test]
fn both_claiming_before_either_checks() {
    let state = State::new();
    main_holding(&state, &[kit(CREW)]);
    let (mut deneb, _) = state.crew("deneb");
    let (mut vega, _) = state.crew("vega");
    // At their first pulls, each reads the other's branch before either
    // claims, and checks only once both have claimed.
    let read = Arc::new(Barrier::new(2));
    let claimed = Arc::new(Barrier::new(2));
    for crew in [&mut deneb, &mut vega] {
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
    let (vega_hello, deneb_hello) = (hello.clone(), hello.clone());
    let vega_run = std::thread::spawn(move || {
        let pulled = pull(&mut vega, &vega_hello);
        (vega, pulled)
    });
    let deneb_pulled = pull(&mut deneb, &deneb_hello);
    let (mut vega, vega_pulled) = vega_run.join().unwrap();
    assert!(deneb_pulled.wants.is_empty(), "{:?}", ids(&deneb_pulled));
    assert!(vega_pulled.wants.is_empty(), "{:?}", ids(&vega_pulled));
    assert!(!answers(&state, "deneb").contains_key(&20));
    assert!(!answers(&state, "vega").contains_key(&20));

    // deneb, first in name order, claims again at its next pull; vega not
    // before the pull after its next.
    assert!(pull(&mut vega, &hello).wants.is_empty());
    assert!(!answers(&state, "vega").contains_key(&20));
    assert_eq!(ids(&pull(&mut deneb, &hello)), ["madswan/20"]);
    assert!(pull(&mut vega, &hello).wants.is_empty());
    assert!(!answers(&state, "vega").contains_key(&20));
}

#[test]
fn no_room_on_the_host() {
    let state = State::new();
    main_holding(&state, &[kit(CREW)]);
    let (mut deneb, _) = state.crew("deneb");
    for room in ["0", "-1"] {
        let pulled = pull(&mut deneb, &hello(&[CREW], room));
        assert!(pulled.wants.is_empty(), "{room}: {:?}", ids(&pulled));
    }
    assert_eq!(state.tip("madswan/runner/deneb"), None);
}

#[test]
fn a_kit_named_by_its_name_alone() {
    let state = State::new();
    main_holding(&state, &[kit("crew")]);
    let (mut deneb, _) = state.crew("deneb");
    let pulled = pull(&mut deneb, &kept());
    let Want::Offer(offer) = want(&pulled, "madswan/20") else {
        panic!("not an offer")
    };
    assert_eq!(offer.launch.repository, CREW);
}

#[test]
fn a_name_two_kept_addresses_share() {
    let state = State::new();
    main_holding(&state, &[kit("crew")]);
    let (mut deneb, said) = state.crew("deneb");
    let pulled = pull(&mut deneb, &hello(&[CREW, "agentplot/crew"], "4"));
    assert!(pulled.wants.is_empty(), "{:?}", ids(&pulled));
    assert_eq!(state.tip("madswan/runner/deneb"), None);
    assert!(
        said.holds(&["madswan/20", "afterthought/crew and agentplot/crew"]),
        "{:?}",
        said.lines()
    );
}

#[test]
fn a_repository_not_kept() {
    let state = State::new();
    main_holding(&state, &[kit("agentplot/sluice")]);
    let (mut deneb, said) = state.crew("deneb");
    let pulled = pull(&mut deneb, &kept());
    assert!(pulled.wants.is_empty(), "{:?}", ids(&pulled));
    assert_eq!(state.tip("madswan/runner/deneb"), None);
    assert!(said.lines().is_empty(), "{:?}", said.lines());
}

#[test]
fn a_claim_waiting_for_its_streams_room() {
    use boomdrive_control::{ControlPlane, Report};
    let state = State::new();
    main_holding(&state, &[kit(CREW)]);
    let (mut deneb, _) = state.crew("deneb");
    assert_eq!(ids(&pull(&mut deneb, &kept())), ["madswan/20"]);
    // The runner leaves the offer unanswered, its stream having no room.
    deneb.report(&Report::default()).expect("a report");
    assert!(write::is_claim(&answers(&state, "deneb")[&20], "deneb"));
    let commits = state.commits("madswan/runner/deneb");
    assert_eq!(ids(&pull(&mut deneb, &kept())), ["madswan/20"]);
    assert_eq!(state.commits("madswan/runner/deneb"), commits);
}

#[test]
fn a_claim_replaced_by_the_runners_answer() {
    use boomdrive_control::{Answer, At, ControlPlane, Id, Launched, Outcome, Report};
    let state = State::new();
    main_holding(&state, &[kit(CREW)]);
    let (mut deneb, _) = state.crew("deneb");
    pull(&mut deneb, &kept());
    let mut report = Report::default();
    report.launches.insert(
        Id::new("madswan/20"),
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
    deneb.report(&report).expect("a report");
    assert_eq!(
        answers(&state, "deneb")[&20].get("Outcome"),
        Some("launched")
    );
    answering(&state, "vega", &[claim(20, "vega")]);
    // A launch the runner's report answers is given, whoever else claims.
    assert_eq!(ids(&deneb.pull(&kept(), &report).unwrap()), ["madswan/20"]);
}

#[test]
fn a_claim_found_again_after_a_stop() {
    let state = State::new();
    main_holding(&state, &[kit(CREW)]);
    answering(&state, "deneb", &[claim(20, "deneb")]);
    let (mut deneb, _) = state.crew("deneb");
    assert_eq!(ids(&pull(&mut deneb, &kept())), ["madswan/20"]);

    answering(&state, "vega", &[claim(20, "vega")]);
    let (mut fresh, _) = state.crew("deneb");
    let pulled = pull(&mut fresh, &kept());
    assert!(pulled.wants.is_empty(), "{:?}", ids(&pulled));
    assert!(!answers(&state, "deneb").contains_key(&20));
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
    answering(&state, "vega", &[launched(20)]);
    let (mut deneb, _) = state.crew("deneb");
    let pulled = pull(&mut deneb, &kept());
    // vega holds launch 20, so its retry 22 is vega's; deneb claims 21 and
    // takes its priority 23.
    assert_eq!(ids(&pulled), ["madswan/21", "madswan/23"]);
}

#[test]
fn another_runners_branch_that_cant_be_read_holds_back_a_claim() {
    let state = State::new();
    main_holding(&state, &[kit(CREW)]);
    state.commit(
        "madswan/runner/vega",
        &[(
            "runner.rec",
            &write::runner_text("other", "x", "2026-11-01T09:00:00Z"),
        )],
    );
    let (mut deneb, said) = state.crew("deneb");
    let pulled = pull(&mut deneb, &kept());
    assert!(pulled.wants.is_empty(), "{:?}", ids(&pulled));
    assert_eq!(state.tip("madswan/runner/deneb"), None);
    assert!(
        said.holds(&["vega's runner can't be read", "runner.rec names host other"]),
        "{:?}",
        said.lines()
    );
}
