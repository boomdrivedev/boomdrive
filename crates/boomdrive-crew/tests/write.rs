//! The runner's branch, written in crew's formats by weir's write, with
//! crew's descriptors byte for byte, and no other ref pushed (design
//! Decision 17; Task notes 8.4).

mod common;

use boomdrive_control::{
    Acted, Answer, At, Carried, ControlPlane, Id, Launched, Lease, Outcome, Previewed, Report,
    Reported,
};
use boomdrive_crew::read::{host_read, RunnerFiles};
use common::{act, file, hello, hello_of, launch, plan, pull, State, REQUESTS};
use std::collections::BTreeSet;
use std::io::Write as _;
use std::process::{Command, Stdio};

const CREW: &str = "example/crew";
const KIT: &str = "example/kit";
const APP: &str = "example/app";
const BRANCH: &str = "home/runner/host-a";

// crew's descriptors, copied from `plugin/lib/runner.py` at `9913f249`,
// which `the-team-machinery-leaves` (`2b97ee57`) leaves as they are.
const RUNNER: &str = "%rec: Runner\n%key: Host\n%type: Since date\n\
                      %mandatory: Host Version Since\n%allowed: Host Version Since\n";
const ANSWERS: &str = "%rec: Answer\n%key: Request\n%type: Request int\n\
                       %type: Outcome enum held launched done refused\n%type: At date\n\
                       %mandatory: Request Outcome At\n%allowed: Request Outcome Place Words At\n";
const PLACES: &str = "%rec: Place\n%key: Request\n%type: Request int\n%type: At date\n\
                      %mandatory: Request Place At Status\n%allowed: Request Place At Status\n";
const RUNS: &str = "%rec: Run\n%key: Address\n%type: At date\n%mandatory: Address At\n\
                    %allowed: Address Place At Status Words\n";
// crew's descriptor of `repositories.rec`, copied from
// `plugin/lib/runner.py` at `323c77fb`, whose `0c0acc15` added it
// (the-runner-names-each-repository-it-keeps, design Decision 3).
const REPOSITORIES: &str = "%rec: Repository\n\
                            %doc: A repository the runner keeps on its host: its forge address \
                            as its configuration names it, and the folder of its clone, where \
                            Flume cuts its places.\n\
                            %key: Repository\n%mandatory: Repository Folder\n\
                            %allowed: Repository Folder\n";
// The descriptor of `previews.rec` this repository proposes for crew to
// hold (design Decision 10).
const PREVIEWS: &str = "%rec: Preview\n%key: Request\n%type: Request int\n\
                        %type: Outcome enum held done refused\n%type: Listed enum yes no\n\
                        %type: At date\n%mandatory: Request Outcome At\n\
                        %allowed: Request Outcome Text Status Listed Words At\n";

/// What is carried of a preview sluice brought up, and of its listing.
const TEXT: &str = r#"{"name":"preview/c2b72605d2ad","branch":"main","revision":"c2b72605d2ad8471a98dfae00551852c527d4dcf","state":"starting","made":"2026-11-08T19:54:52Z","addresses":{"web":"http://web-c2b7260--main--repo.localhost:20000"}}"#;
const STATUS: &str = r#"{"name":"preview/c2b72605d2ad","branch":"main","revision":"c2b72605d2ad8471a98dfae00551852c527d4dcf","state":"up","made":"2026-11-08T19:54:52Z","addresses":{"web":"http://web-c2b7260--main--repo.localhost:20000"}}"#;

/// Preview 30 brought up, its text and its status carried, listed.
fn brought_up() -> Previewed {
    Previewed {
        lease: Lease {
            number: 1,
            expires: None,
        },
        repository: CREW.to_string(),
        revision: "c2b72605d2ad8471a98dfae00551852c527d4dcf".to_string(),
        done: false,
        outcome: Outcome::Done,
        text: Some(TEXT.to_string()),
        words: None,
        status: Some(Carried {
            text: STATUS.to_string(),
            at: at("2026-11-01T10:07:00Z"),
            listed: true,
        }),
        at: at("2026-11-01T10:06:00Z"),
    }
}

/// `report` holding preview 30 brought up.
fn with_preview(mut report: Report) -> Report {
    report.previews.insert(Id::new("home/30"), brought_up());
    report
}

fn at(text: &str) -> At {
    At::parse(text).unwrap()
}

fn answer(outcome: Outcome, place: Option<&str>, words: Option<&str>, when: &str) -> Answer {
    Answer {
        outcome,
        place_dir: place.map(str::to_string),
        words: words.map(str::to_string),
        at: at(when),
    }
}

fn launched(answer: Answer, status: Option<(&str, &str)>) -> Launched {
    Launched {
        lease: Lease {
            number: 1,
            expires: None,
        },
        repository: None,
        answer,
        status: status.map(|(text, when)| Carried {
            text: text.to_string(),
            at: at(when),
            listed: true,
        }),
        entry: None,
    }
}

/// A state repository whose `home/main` holds launch 12 and its
/// retry 13 for host-a, and the planner's role on host-a, and host-a's
/// implementation, pulled once with the drive `home` keeping `CREW` and
/// `KIT`, and the drive `work` keeping `APP`.
fn pulled() -> (State, boomdrive_crew::Crew) {
    let (state, mut crew) = unpulled();
    pull(
        &mut crew,
        &hello_of(&[("home", &[CREW, KIT]), ("work", &[APP])], "4"),
    );
    (state, crew)
}

/// The state repository of [`pulled`], and host-a's implementation, not
/// yet pulled.
fn unpulled() -> (State, boomdrive_crew::Crew) {
    let state = State::new();
    state.main(&[
        (
            "requests.rec",
            &file(
                REQUESTS,
                &[
                    launch(12, Some("host-a"), "unit/a", CREW, "a", &[]),
                    act(13, "retry", Some("host-a"), "unit/a", 12, &[]),
                ],
            ),
        ),
        ("plan.rec", &plan(&["a"], &[])),
        (
            "roles.rec",
            &file(common::ROLES, &[common::role("planner", "host-a")]),
        ),
    ]);
    let (crew, _) = state.crew("host-a");
    (state, crew)
}

/// A report answering launch 12 launched at `/places/change--a`, with
/// its place's status `status`.
fn report_launched(status: &str) -> Report {
    let mut report = Report::default();
    report.launches.insert(
        Id::new("home/12"),
        launched(
            answer(
                Outcome::Launched,
                Some("/places/change--a"),
                None,
                "2026-11-01T10:00:00Z",
            ),
            Some((status, "2026-11-01T10:01:00Z")),
        ),
    );
    report
}

fn files(state: &State) -> RunnerFiles {
    let mut files = RunnerFiles::new();
    for path in [
        "runner.rec",
        "answers.rec",
        "places.rec",
        "repositories.rec",
        "previews.rec",
    ] {
        files.insert(path, state.file(BRANCH, path));
    }
    files
}

/// recfix's words on `text`, none where it takes it.
fn recfix(text: &str) -> Option<String> {
    let mut child = Command::new("recfix")
        .arg("--check")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("recfix runs: recutils is on PATH, as devenv.nix pins it");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(text.as_bytes())
        .unwrap();
    let out = child.wait_with_output().unwrap();
    (!out.status.success()).then(|| String::from_utf8_lossy(&out.stderr).into_owned())
}

#[test]
fn the_first_report() {
    let (state, mut crew) = pulled();
    let main = state.tip("home/main");
    assert_eq!(state.tip(BRANCH), None);
    let reported = crew
        .report(&report_launched(r#"{"status":{}}"#))
        .expect("a report");
    assert_eq!(
        reported,
        Reported {
            statuses_taken: BTreeSet::from([Id::new("home/12")]),
            ..Reported::default()
        },
        "the status places.rec holds is taken, and nothing is let go or refused"
    );
    let runner = state.file(BRANCH, "runner.rec").expect("runner.rec");
    let body = runner.strip_prefix(RUNNER).expect("crew's descriptor");
    let since = body
        .strip_prefix("\nHost: host-a\nVersion: boomdrive 0.0.0\nSince: ")
        .and_then(|rest| rest.strip_suffix('\n'))
        .unwrap_or_else(|| panic!("{runner}"));
    assert!(At::parse(since).is_ok(), "{since}");
    assert_eq!(state.tip("home/main"), main);
    let message = common::git(&state.bare(), &["log", "-1", "--format=%B", BRANCH], None);
    assert_eq!(
        message,
        "runner, answers 12, places 12, repositories example/crew example/kit\n\n\
         Asked-by: boomdrive@host-a\n\n"
    );
}

/// The crew-control-plane spec's "A drive's repositories and their
/// folders": `repositories.rec` opens with crew's descriptor, holds the
/// drive `home`'s repositories in the hello's order, each with its
/// folder, names none of the drive `work`'s, and passes `recfix --check`
/// (design Decision 3).
#[test]
fn a_drives_repositories_and_their_folders() {
    let (state, mut crew) = pulled();
    crew.report(&Report::default()).expect("a report");
    let repositories = state
        .file(BRANCH, "repositories.rec")
        .expect("repositories.rec");
    assert_eq!(
        repositories,
        format!(
            "{REPOSITORIES}\nRepository: example/crew\nFolder: /data/repositories/example/crew\n\n\
             Repository: example/kit\nFolder: /data/repositories/example/kit\n"
        )
    );
    assert!(!repositories.contains(APP), "{repositories}");
    assert_eq!(recfix(&repositories), None, "{repositories}");
}

#[test]
fn since_is_the_first_writes_time_kept_after() {
    let (state, mut crew) = pulled();
    crew.report(&Report::default()).expect("a report");
    let first = state.file(BRANCH, "runner.rec").unwrap();
    std::thread::sleep(std::time::Duration::from_millis(1100));
    crew.report(&report_launched("{}")).expect("a report");
    assert_eq!(state.file(BRANCH, "runner.rec").unwrap(), first);
    assert_eq!(state.commits(BRANCH), 2);
}

#[test]
fn a_held_answer_becoming_launched() {
    let (state, mut crew) = pulled();
    let mut report = Report::default();
    report.launches.insert(
        Id::new("home/12"),
        launched(
            answer(
                Outcome::Held,
                None,
                Some("waits for room in its stream"),
                "2026-11-01T10:00:00Z",
            ),
            None,
        ),
    );
    crew.report(&report).expect("a report");
    assert_eq!(
        state.file(BRANCH, "answers.rec").unwrap(),
        format!(
            "{ANSWERS}\nRequest: 12\nOutcome: held\nWords: waits for room in its stream\n\
             At: 2026-11-01T10:00:00Z\n"
        )
    );
    crew.report(&report_launched("{}")).expect("a report");
    assert_eq!(
        state.file(BRANCH, "answers.rec").unwrap(),
        format!(
            "{ANSWERS}\nRequest: 12\nOutcome: launched\nPlace: /places/change--a\n\
             At: 2026-11-01T10:00:00Z\n"
        )
    );
}

#[test]
fn a_places_status_as_flume_gave_it() {
    let (state, mut crew) = pulled();
    let status = common::status_code();
    let mut report = report_launched(&status);
    report.launches.insert(
        Id::new("home/14"),
        launched(
            answer(Outcome::Held, None, None, "2026-11-01T10:00:00Z"),
            None,
        ),
    );
    let reported = crew.report(&report).expect("a report");
    assert_eq!(
        reported.statuses_taken,
        BTreeSet::from([Id::new("home/12")]),
        "the status places.rec holds is taken, and the held request has none"
    );
    let places = state.file(BRANCH, "places.rec").unwrap();
    assert_eq!(
        places,
        format!(
            "{PLACES}\nRequest: 12\nPlace: /places/change--a\nAt: 2026-11-01T10:01:00Z\n\
             Status: {status}\n\n{RUNS}"
        )
    );
    assert!(state
        .file(BRANCH, "answers.rec")
        .unwrap()
        .starts_with(ANSWERS));
    assert!(state
        .file(BRANCH, "runner.rec")
        .unwrap()
        .starts_with(RUNNER));
    let read = host_read("host-a", &files(&state)).expect("crew reads it");
    assert_eq!(read.places[&12].get("Status"), Some(status.as_str()));
}

#[test]
fn an_acts_done() {
    let (state, mut crew) = pulled();
    let mut report = report_launched("{}");
    report.acts.insert(
        Id::new("home/13"),
        Acted {
            launch: Id::new("home/12"),
            lease: 1,
            outcome: Outcome::Done,
            words: None,
            at: at("2026-11-01T10:05:00Z"),
        },
    );
    crew.report(&report).expect("a report");
    let read = host_read("host-a", &files(&state)).expect("crew reads it");
    let done = &read.answers[&13];
    assert_eq!(done.get("Outcome"), Some("done"));
    assert_eq!(done.get("At"), Some("2026-11-01T10:05:00Z"));
    assert_eq!(done.get("Place"), None);
}

#[test]
fn a_run_with_words() {
    let (state, mut crew) = pulled();
    let mut report = Report::default();
    report.launches.insert(
        Id::new("home/planner/1"),
        launched(
            answer(
                Outcome::Refused,
                None,
                Some("no kind crew/crew-planner on this host"),
                "2026-11-01T10:00:00Z",
            ),
            None,
        ),
    );
    crew.report(&report).expect("a report");
    assert_eq!(
        state.file(BRANCH, "places.rec").unwrap(),
        format!(
            "{PLACES}\n{RUNS}\nAddress: home/planner\nAt: 2026-11-01T10:00:00Z\n\
             Words: no kind crew/crew-planner on this host\n"
        )
    );
}

#[test]
fn nothing_changed() {
    let (state, mut crew) = pulled();
    let report = with_preview(report_launched(&common::status_code()));
    crew.report(&report).expect("a report");
    let tip = state.tip(BRANCH);
    crew.report(&report).expect("a report");
    assert_eq!(state.tip(BRANCH), tip);
    assert_eq!(state.commits(BRANCH), 1);
}

#[test]
fn only_the_runners_branch() {
    let (state, mut crew) = pulled();
    let before = state.refs();
    crew.report(&with_preview(report_launched(&common::status_code())))
        .expect("a report");
    let after = state.refs();
    let added: Vec<&str> = after
        .lines()
        .filter(|line| !before.lines().any(|old| old == *line))
        .collect();
    assert_eq!(added.len(), 1, "{after}");
    assert!(
        added[0].ends_with(" refs/heads/home/runner/host-a"),
        "{after}"
    );
    for line in before.lines() {
        assert!(after.lines().any(|now| now == line), "{line} moved");
    }
    for path in [
        "runner.rec",
        "answers.rec",
        "places.rec",
        "repositories.rec",
        "previews.rec",
    ] {
        let text = state.file(BRANCH, path).unwrap();
        assert_eq!(recfix(&text), None, "{path}: {text}");
    }
}

#[test]
fn each_file_reads_as_crew_reads_it() {
    let (state, mut crew) = pulled();
    let mut report = report_launched(&common::status_code());
    report.acts.insert(
        Id::new("home/13"),
        Acted {
            launch: Id::new("home/12"),
            lease: 1,
            outcome: Outcome::Refused,
            words: Some("its agent is not quiet".to_string()),
            at: at("2026-11-01T10:05:00Z"),
        },
    );
    report.launches.insert(
        Id::new("home/planner/2"),
        launched(
            answer(
                Outcome::Launched,
                Some("/places/crew-planner--home"),
                None,
                "2026-11-01T10:00:00Z",
            ),
            Some((&common::status_delivered(), "2026-11-01T10:02:00Z")),
        ),
    );
    crew.report(&report).expect("a report");
    let read = host_read("host-a", &files(&state)).expect("crew reads it");
    assert!(read.since.is_some());
    assert_eq!(read.answers.keys().copied().collect::<Vec<_>>(), [12, 13]);
    assert_eq!(read.places.keys().copied().collect::<Vec<_>>(), [12]);
    assert_eq!(
        read.runs.keys().cloned().collect::<Vec<_>>(),
        ["home/planner"]
    );
    let statuses = read
        .places
        .values()
        .chain(read.runs.values())
        .filter_map(|record| record.get("Status"));
    for status in statuses {
        serde_json::from_str::<serde_json::Value>(status).expect("Flume's JSON");
    }
    assert!(host_read("host-b", &files(&state)).is_err());
}

#[test]
fn the_report_comes_back_in_the_next_pull() {
    let (state, mut crew) = pulled();
    let report = report_launched(&common::status_code());
    crew.report(&report).expect("a report");
    let (mut fresh, _) = state.crew("host-a");
    let pulled = pull(&mut fresh, &hello(&[CREW], "4"));
    let back = pulled.report.expect("the report on the branch");
    assert_eq!(back, report);
}

/// The crew-control-plane spec's "A preview brought up": `previews.rec`
/// opens with crew's descriptor of a `Preview`, byte for byte, and holds
/// one `Preview` as the report answers it, passing `recfix --check`.
#[test]
fn a_preview_brought_up_is_written_in_previews_rec() {
    let (state, mut crew) = pulled();
    crew.report(&with_preview(Report::default()))
        .expect("a report");
    let previews = state.file(BRANCH, "previews.rec").expect("previews.rec");
    assert_eq!(
        previews,
        format!(
            "{PREVIEWS}\nRequest: 30\nOutcome: done\nText: {TEXT}\nStatus: {STATUS}\n\
             Listed: yes\nAt: 2026-11-01T10:07:00Z\n"
        )
    );
    assert_eq!(recfix(&previews), None, "{previews}");
    let message = common::git(&state.bare(), &["log", "-1", "--format=%s", BRANCH], None);
    assert!(message.contains("previews 30"), "{message}");
}

/// The crew-control-plane spec's "A branch with no preview".
#[test]
fn a_branch_with_no_preview() {
    let (state, mut crew) = pulled();
    crew.report(&report_launched("{}")).expect("a report");
    assert!(state.file(BRANCH, "answers.rec").is_some());
    assert_eq!(state.file(BRANCH, "previews.rec"), None);
}

/// The crew-control-plane spec's "A preview read back": a pull handed no
/// report gives back the preview `previews.rec` holds, as its request
/// names it, its repository resolved as at a pull.
#[test]
fn a_preview_read_back() {
    let state = State::new();
    state.main(&[
        (
            "requests.rec",
            &file(
                REQUESTS,
                &[common::preview(
                    30,
                    Some("host-a"),
                    "unit/a",
                    "crew",
                    "main",
                    "c2b72605d2ad8471a98dfae00551852c527d4dcf",
                    false,
                )],
            ),
        ),
        ("plan.rec", &plan(&["a"], &[])),
    ]);
    let (mut crew, _) = state.crew("host-a");
    pull(&mut crew, &hello(&[CREW], "4"));
    let report = with_preview(Report::default());
    crew.report(&report).expect("a report");
    let (mut fresh, _) = state.crew("host-a");
    let pulled = pull(&mut fresh, &hello(&[CREW], "4"));
    let back = pulled.report.expect("the report on the branch");
    let previewed = &back.previews[&Id::new("home/30")];
    let sent = &report.previews[&Id::new("home/30")];
    assert_eq!(previewed.outcome, Outcome::Done);
    assert_eq!(previewed.text, sent.text);
    assert_eq!(previewed.words, None);
    assert_eq!(previewed.status, sent.status);
    assert_eq!(previewed.repository, CREW);
    assert_eq!(previewed.revision, sent.revision);
    assert!(!previewed.done);
    assert_eq!(previewed.lease.number, 1);
    let read = host_read("host-a", &files(&state)).expect("crew reads it");
    assert_eq!(read.previews.keys().copied().collect::<Vec<_>>(), [30]);
}
