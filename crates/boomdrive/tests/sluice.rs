//! sluice's command line against a stub `sluice` written into each test's
//! own temporary directory (design Decision 21): the argv it is given,
//! where it runs, the reading of each answer by sluice's exit, and what
//! the runner carries of a submission, over sluice 0.61.1's own answers
//! (design Decision 13; core.11; Task notes 5.1).

mod fakes;

use boomdrive::sluice::{self, Asked, Cli, Reply, Sluice};
use fakes::{preview_fixture, preview_fixtures, sluice_fixture, sluice_fixtures, SluiceStub};
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::time::{Duration, Instant};

/// Every fixture of sluice 0.61.1's some test here reads.
const READ: [&str; 4] = [
    "status-failed.json",
    "status-landed.json",
    "status-missing.json",
    "status-withdrawn.json",
];

/// Every fixture of sluice 0.74.3's, a preview's, some test reads.
const READ_PREVIEWS: [&str; 5] = [
    "preview-no-table.json",
    "preview-starting.json",
    "previews.json",
    "release-nothing.json",
    "release.json",
];

#[test]
fn every_fixture_is_read() {
    each_fixture_is_read(&sluice_fixtures(), &READ);
    each_fixture_is_read(&preview_fixtures(), &READ_PREVIEWS);
}

/// `dir` holds `read` and its README, and the README names each of them.
fn each_fixture_is_read(dir: &Path, read: &[&str]) {
    let held: BTreeSet<String> = fs::read_dir(dir)
        .expect("the fixtures")
        .map(|file| {
            file.expect("a fixture")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .filter(|name| name != "README.md")
        .collect();
    let names: BTreeSet<String> = read.iter().map(|name| name.to_string()).collect();
    assert_eq!(
        held,
        names,
        "{}: a fixture no test reads, or one read and not held",
        dir.display()
    );
    let readme = fs::read_to_string(dir.join("README.md")).expect("the README");
    for name in read {
        assert!(
            readme.contains(&format!("`{name}`")),
            "{}: README says nothing of {name}",
            dir.display()
        );
    }
}

/// A clone's container for a stub to run in.
fn container() -> tempfile::TempDir {
    tempfile::tempdir().expect("a container")
}

#[test]
fn an_entry_is_asked_for_in_the_clones_container() {
    let stub = SluiceStub::new();
    stub.answers("status", "status-landed.json", 0);
    let container = container();
    let reply = Cli::new(stub.program()).entry(container.path(), "412");
    assert_eq!(
        reply,
        Ok(Reply::Given(sluice_fixture("status-landed.json")))
    );
    assert_eq!(stub.calls(), [["status", "412", "--json"]]);
    assert_eq!(
        stub.dir_of(1),
        Some(container.path().canonicalize().unwrap())
    );
}

#[test]
fn a_refusal_is_sluices_message() {
    let stub = SluiceStub::new();
    stub.answers("status", "status-missing.json", 1);
    let reply = Cli::new(stub.program()).entry(container().path(), "99");
    assert_eq!(reply, Ok(Reply::Refused("no submission 99".to_string())));
}

#[test]
fn a_refusal_with_no_error_answer_is_sluices_standard_error() {
    let stub = SluiceStub::new();
    stub.answers_text("status", "", 2);
    stub.says("status", "error: unexpected argument '--jsn'\n");
    let reply = Cli::new(stub.program()).entry(container().path(), "1");
    assert_eq!(
        reply,
        Ok(Reply::Refused(
            "error: unexpected argument '--jsn'\n".to_string()
        ))
    );
}

#[test]
fn an_answer_that_is_not_json_is_refused_with_sluices_standard_error() {
    let stub = SluiceStub::new();
    stub.answers_text("status", "submission 1: landed\n", 0);
    stub.says("status", "warning: no --json\n");
    let reply = Cli::new(stub.program()).entry(container().path(), "1");
    assert_eq!(
        reply,
        Ok(Reply::Refused("warning: no --json\n".to_string()))
    );
}

#[test]
fn a_program_that_is_not_there_gives_no_answer() {
    let stub = SluiceStub::absent();
    let failure = Cli::new(stub.program())
        .entry(container().path(), "1")
        .expect_err("an answer from no program");
    assert!(failure.0.starts_with("sluice could not run: "), "{failure}");
}

#[test]
fn a_landed_submission_carried() {
    let carried = sluice::carried(&sluice_fixture("status-landed.json")).unwrap();
    assert_eq!(
        carried,
        r#"{"id":1,"branch":"main","head":"3cb12e20c76c17ef51cb8be1c32bec8671a7ab73","state":"landed","landed_as":"fdaaeaf1b1094b39db4ccea5f01e88a3bedb95d8"}"#
    );
}

#[test]
fn a_failed_row_carried_without_its_log() {
    let carried = sluice::carried(&sluice_fixture("status-failed.json")).unwrap();
    assert_eq!(
        carried,
        concat!(
            r#"{"id":2,"branch":"main","head":"490ad47e09844724909e5bd15919616a9a76e74b","state":"failed","failed":["#,
            r#"{"task":"sh check.sh","hash":"09e46ec47a28327f23dfc07c480bec754024a2e7","flaky":"#,
            r#"{"here":"pass","at":"2026-11-08T00:34:38Z","host":"host-a","other":"#,
            r#"{"outcome":"fail","at":"2026-11-08T00:34:35Z","branch":"main","combination":"09e46ec47a28327f23dfc07c480bec754024a2e7","host":"host-a","worktree":"/private/tmp/scratch/host/worktrees/tabs-combination"}}}]}"#
        )
    );
}

/// The withdrawn submission's `flaky` reading names where the run it
/// contradicts kept its output, and that is left out too (core.11).
#[test]
fn a_flaky_readings_log_is_left_out() {
    let printed = sluice_fixture("status-withdrawn.json");
    assert_eq!(printed.matches("\"log\"").count(), 2);
    let carried = sluice::carried(&printed).unwrap();
    assert!(!carried.contains("\"log\""), "{carried}");
    assert!(!carried.contains("/.logs/"), "{carried}");
    let value: Value = serde_json::from_str(&carried).unwrap();
    assert_eq!(
        value,
        json!({
            "id": 4,
            "branch": "main",
            "head": "2b944a5326f3bcbbff3e0a717bc05f403c2fd3cb",
            "state": "withdrawn",
            "failed": [{
                "task": "sh check.sh",
                "hash": "871867da54ed6ec660b7123312f2dc68760f7ec2",
                "flaky": {
                    "here": "pass",
                    "at": "2026-11-08T00:34:45Z",
                    "host": "host-a",
                    "other": {
                        "outcome": "fail",
                        "at": "2026-11-08T00:34:42Z",
                        "branch": "main",
                        "combination": "871867da54ed6ec660b7123312f2dc68760f7ec2",
                        "host": "host-a"
                    }
                }
            }]
        })
    );
}

/// `preview-starting.json` carried as its `name`, `branch`, `revision`,
/// `state`, `made` and `addresses`, in that order, each as the fixture
/// prints it, and nothing more (design Decision 4).
#[test]
fn a_preview_carried() {
    let carried = sluice::preview_carried(&preview_fixture("preview-starting.json")).unwrap();
    assert_eq!(
        carried,
        concat!(
            r#"{"name":"preview/c2b72605d2ad","branch":"main","#,
            r#""revision":"c2b72605d2ad8471a98dfae00551852c527d4dcf","state":"starting","#,
            r#""made":"2026-11-08T19:54:52Z","#,
            r#""addresses":{"web":"http://web-c2b7260--main--repo.localhost:20000"}}"#
        )
    );
}

/// The previews spec's "A failed preview's output left out":
/// `previews.json`'s failed row is carried with its `state`, `name` and
/// `addresses` as the fixture prints them, and no `output` (core.11).
#[test]
fn a_failed_previews_output_is_left_out() {
    let printed = preview_fixture("previews.json");
    let output = "the stack did not come up";
    assert!(printed.contains(output));
    let listed = sluice::listed(&printed).unwrap();
    let failed = listed
        .iter()
        .find(|row| sluice::text_of(&row.carried, "state").as_deref() == Some("failed"))
        .expect("the failed row");
    assert!(!failed.carried.contains("\"output\""), "{}", failed.carried);
    assert!(!failed.carried.contains(output), "{}", failed.carried);
    assert_eq!(
        failed.carried,
        concat!(
            r#"{"name":"preview/124c789156d0","branch":"main","#,
            r#""revision":"124c789156d05fed6f36914a4d0a57ccbb92abe2","state":"failed","#,
            r#""made":"2026-11-08T19:54:52Z","#,
            r#""addresses":{"web":"http://web-124c789--main--repo.localhost:20001"}}"#
        )
    );
    assert_eq!(failed.name.as_deref(), Some("preview/124c789156d0"));
    assert_eq!(listed.len(), 2);
}

/// `release.json` carried as its `preview` and `released`.
#[test]
fn a_release_carried() {
    assert_eq!(
        sluice::release_carried(&preview_fixture("release.json")).as_deref(),
        Some(r#"{"preview":"preview/c2b72605d2ad","released":true}"#)
    );
    assert_eq!(
        sluice::release_carried(&preview_fixture("release-nothing.json")).as_deref(),
        Some(r#"{"preview":"preview/c2b72605d2ad","released":false}"#)
    );
    assert_eq!(sluice::release_carried("[]"), None);
    assert_eq!(sluice::preview_carried(r#"{"state":"up"}"#), None);
}

/// A preview is asked of sluice in the clone's container, the branch and
/// the revision as given, and its answer read by sluice's exit (design
/// Decision 2).
#[test]
fn a_preview_is_asked_in_the_clones_container() {
    let stub = SluiceStub::new();
    stub.answers_preview("preview", "preview-starting.json", 0);
    let container = container();
    let reply = Cli::new(stub.program()).preview(container.path(), "main", "R1");
    assert_eq!(
        reply,
        Ok(Asked::Replied(Reply::Given(preview_fixture(
            "preview-starting.json"
        ))))
    );
    assert_eq!(stub.calls(), [["preview", "main", "R1", "--json"]]);
    assert_eq!(
        stub.dir_of(1),
        Some(container.path().canonicalize().unwrap())
    );
    stub.answers_preview("preview", "preview-no-table.json", 2);
    let reply = Cli::new(stub.program()).preview(container.path(), "main", "R0");
    assert_eq!(
        reply,
        Ok(Asked::Replied(Reply::Refused(
            "no [preview] table in .config/sluice.toml at 99248c2".to_string()
        )))
    );
}

/// An ask sluice holds past the patience is sent SIGINT, and its exit
/// with no document read as sluice holding the preview waiting (design
/// Decision 3).
#[test]
fn an_ask_past_the_patience_is_interrupted_and_waits() {
    let stub = SluiceStub::new();
    stub.waits("preview");
    let began = Instant::now();
    let reply = Cli::new(stub.program())
        .patient(Duration::from_secs(1))
        .preview(container().path(), "main", "R1");
    let took = began.elapsed();
    assert_eq!(reply, Ok(Asked::Waiting));
    assert!(took >= Duration::from_secs(1), "{took:?}");
    assert!(took < Duration::from_secs(5), "{took:?}");
}

/// sluice's previews are listed in the clone's container (design
/// Decision 5).
#[test]
fn previews_are_listed_in_the_clones_container() {
    let stub = SluiceStub::new();
    stub.answers_preview("previews", "previews.json", 0);
    let container = container();
    let reply = Cli::new(stub.program()).previews(container.path());
    assert_eq!(reply, Ok(Reply::Given(preview_fixture("previews.json"))));
    assert_eq!(stub.calls(), [["previews", "--json"]]);
    assert_eq!(
        stub.dir_of(1),
        Some(container.path().canonicalize().unwrap())
    );
}

/// A preview is released by its name in the clone's container (design
/// Decision 6).
#[test]
fn a_release_is_asked_by_name() {
    let stub = SluiceStub::new();
    stub.answers_preview("release", "release.json", 0);
    let container = container();
    let reply = Cli::new(stub.program()).release(container.path(), "preview/c2b72605d2ad");
    assert_eq!(reply, Ok(Reply::Given(preview_fixture("release.json"))));
    assert_eq!(
        stub.calls(),
        [["release", "preview/c2b72605d2ad", "--json"]]
    );
    assert_eq!(
        stub.dir_of(1),
        Some(container.path().canonicalize().unwrap())
    );
}
