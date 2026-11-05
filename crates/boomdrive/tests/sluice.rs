//! sluice's command line against a stub `sluice` written into each test's
//! own temporary directory (design Decision 21): the argv it is given,
//! where it runs, the reading of each answer by sluice's exit, and what
//! the runner carries of a submission and of a branch it makes, over
//! sluice 0.61.1's own answers (design Decisions 13 and 15; core.11; Task
//! notes 5.1 and 6.1).

mod fakes;

use boomdrive::sluice::{self, Cli, Printed, Reply, Sluice};
use fakes::{sluice_fixture, sluice_fixtures, SluiceStub};
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::fs;

/// Every fixture of sluice's some test here reads.
const READ: [&str; 7] = [
    "branch-made.json",
    "branch-there.json",
    "branch-ungated.json",
    "status-failed.json",
    "status-landed.json",
    "status-missing.json",
    "status-withdrawn.json",
];

#[test]
fn every_fixture_is_read() {
    let held: BTreeSet<String> = fs::read_dir(sluice_fixtures())
        .expect("the fixtures")
        .map(|file| {
            file.expect("a fixture")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .filter(|name| name != "README.md")
        .collect();
    let read: BTreeSet<String> = READ.iter().map(|name| name.to_string()).collect();
    assert_eq!(
        held, read,
        "a fixture no test reads, or one read and not held"
    );
    let readme = sluice_fixture("README.md");
    for name in READ {
        assert!(
            readme.contains(&format!("`{name}`")),
            "README says nothing of {name}"
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
fn a_branch_is_made_in_the_clones_container() {
    let stub = SluiceStub::new();
    stub.answers("branch", "branch-made.json", 0);
    let container = container();
    let printed = Cli::new(stub.program()).branch(container.path(), "bolt/x", "main");
    assert_eq!(
        printed,
        Ok(Printed {
            text: sluice_fixture("branch-made.json"),
            refused: None,
        })
    );
    assert_eq!(
        stub.calls(),
        [["branch", "create", "bolt/x", "--from", "main", "--json"]]
    );
    assert_eq!(
        stub.dir_of(1),
        Some(container.path().canonicalize().unwrap())
    );
}

/// A refusal keeps sluice's whole answer as printed, and its words are
/// the `message` of its `error` answer.
#[test]
fn a_branch_refused_keeps_sluices_answer() {
    let stub = SluiceStub::new();
    stub.answers("branch", "branch-there.json", 2);
    let printed = Cli::new(stub.program()).branch(container().path(), "bolt/x", "main");
    assert_eq!(
        printed,
        Ok(Printed {
            text: sluice_fixture("branch-there.json"),
            refused: Some(
                "bolt/x is already there, at ee68c82edcfe7ff041906f0e2842921de57f6d35".to_string()
            ),
        })
    );
    stub.answers("branch", "branch-ungated.json", 2);
    let printed = Cli::new(stub.program()).branch(container().path(), "bolt/x", "main");
    assert_eq!(
        printed.unwrap().refused.as_deref(),
        Some("no [branches] table in .config/sluice.toml on main names bolt/x")
    );
}

#[test]
fn a_branch_refused_with_no_error_answer_is_sluices_standard_error() {
    let stub = SluiceStub::new();
    stub.answers_text("branch", "", 2);
    stub.says("branch", "error: unrecognized subcommand 'create'\n");
    let printed = Cli::new(stub.program()).branch(container().path(), "bolt/x", "main");
    assert_eq!(
        printed,
        Ok(Printed {
            text: String::new(),
            refused: Some("error: unrecognized subcommand 'create'\n".to_string()),
        })
    );
}

/// sluice's exit decides: 0 is done, whatever it printed.
#[test]
fn a_branch_is_done_by_sluices_exit() {
    let stub = SluiceStub::new();
    stub.answers_text("branch", "created bolt/x\n", 0);
    let printed = Cli::new(stub.program()).branch(container().path(), "bolt/x", "main");
    assert_eq!(
        printed,
        Ok(Printed {
            text: "created bolt/x\n".to_string(),
            refused: None,
        })
    );
}

#[test]
fn a_branch_with_no_program_gives_no_answer() {
    let stub = SluiceStub::absent();
    let failure = Cli::new(stub.program())
        .branch(container().path(), "bolt/x", "main")
        .expect_err("an answer from no program");
    assert!(failure.0.starts_with("sluice could not run: "), "{failure}");
}

#[test]
fn a_landed_submission_carried() {
    let carried = sluice::carried(&sluice_fixture("status-landed.json")).unwrap();
    assert_eq!(
        carried,
        r#"{"id":1,"branch":"main","head":"91aff4b74c91b37c374c46b43b35e87b128b0c65","state":"landed","landed_as":"3e61c31926e3e265212226d30fe3c5934665cc4d"}"#
    );
}

#[test]
fn a_failed_row_carried_without_its_log() {
    let carried = sluice::carried(&sluice_fixture("status-failed.json")).unwrap();
    assert_eq!(
        carried,
        concat!(
            r#"{"id":45,"branch":"main","head":"0d43da70a2ff2bdc4e31fa260252ace4a9ff44b6","state":"failed","failed":["#,
            r#"{"task":"flume:tools","hash":"e0e41e5811489647069b018088b150c920c426e67932fe2206b24c0ad6342f8d","flaky":"#,
            r#"{"here":"pass","at":"2026-10-29T17:53:11Z","host":"deneb","other":"#,
            r#"{"outcome":"fail","at":"2026-10-29T17:19:43Z","branch":"main","combination":"0d43da70a2ff2bdc4e31fa260252ace4a9ff44b6","host":"deneb","worktree":"/Users/chuck/Code/github_agentplot/flume/fix-live-trust-and-secret-env"}}}]}"#
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
            "id": 291,
            "branch": "main",
            "head": "4480899ef7e43922145534dc0b9e09b5d30ed105",
            "state": "withdrawn",
            "failed": [{
                "task": "cli:test",
                "hash": "ac52c612a1fb27a96a0e1b72b709852ef4267f1a3e53b5bbc43ded025ce0f2d1",
                "flaky": {
                    "here": "pass",
                    "at": "2026-10-29T15:29:18Z",
                    "host": "deneb",
                    "other": {
                        "outcome": "fail",
                        "at": "2026-10-29T14:54:24Z",
                        "branch": "main",
                        "combination": "01cd2fc56e2c947392f198d3dd43082fd5967d6b",
                        "host": "deneb"
                    }
                }
            }]
        })
    );
}
