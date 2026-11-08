//! The preview a control plane asks for, brought up through a stub
//! `sluice` in a repository the runner keeps, carried as sluice listed it
//! with no output of its command, and put away on its done only where the
//! runner asked sluice for it (the previews spec; design Decisions 2 to 8;
//! core.55; Task notes 3.1 and group 5).

mod fakes;

use boomdrive_control::fake::Fake;
use boomdrive_control::{At, Id, Lease, Outcome, Report, Want};
use fakes::{preview, preview_fixture, Bench, KEPT};
use std::path::PathBuf;

/// `want`, a preview, in `repository`.
fn in_repository(mut want: Want, repository: &str) -> Want {
    if let Want::Preview(preview) = &mut want {
        preview.repository = repository.to_string();
    }
    want
}

/// `want`, a preview, under lease 1 expiring at `expires`.
fn expiring(mut want: Want, expires: &str) -> Want {
    if let Want::Preview(preview) = &mut want {
        preview.lease = Lease {
            number: 1,
            expires: Some(At::parse(expires).unwrap()),
        };
    }
    want
}

fn id(text: &str) -> Id {
    Id::new(text)
}

/// The report the bench's file holds, or none where no pass wrote one.
fn report(bench: &Bench) -> Report {
    boomdrive::report::read(&bench.report_file())
        .expect("the report file reads")
        .unwrap_or_default()
}

/// A bench whose stub `sluice` lists no preview.
fn bench() -> Bench {
    let bench = Bench::new();
    bench.sluice.answers_text("previews", "[]\n", 0);
    bench
}

/// Each call the stub `sluice` was given whose command is `what`, with
/// the directory it ran in.
fn calls_of(bench: &Bench, what: &str) -> Vec<(Vec<String>, PathBuf)> {
    bench
        .sluice
        .calls()
        .into_iter()
        .enumerate()
        .filter(|(_, call)| call.first().map(String::as_str) == Some(what))
        .map(|(n, call)| (call, bench.sluice.dir_of(n + 1).expect("its directory")))
        .collect()
}

/// The commands the stub `sluice` was given, in order.
fn commands(bench: &Bench) -> Vec<String> {
    bench
        .sluice
        .calls()
        .into_iter()
        .filter_map(|call| call.first().cloned())
        .collect()
}

/// The container of [`KEPT`]'s clone, as the stub reads its directory.
fn kept(bench: &Bench) -> PathBuf {
    bench.container(KEPT).canonicalize().unwrap()
}

/// What is carried of `preview-starting.json`.
const STARTING: &str = concat!(
    r#"{"name":"preview/c2b72605d2ad","branch":"main","#,
    r#""revision":"c2b72605d2ad8471a98dfae00551852c527d4dcf","state":"starting","#,
    r#""made":"2026-11-08T19:54:52Z","#,
    r#""addresses":{"web":"http://web-c2b7260--main--repo.localhost:20000"}}"#
);

/// The previews spec's "A preview brought up".
#[test]
fn a_preview_brought_up() {
    let bench = bench();
    bench
        .sluice
        .answers_preview("preview", "preview-starting.json", 0);
    let mut plane = Fake::holding(vec![preview("home/30", "main", "R1", false)]);
    let (_, lines) = bench.pass(&mut plane);
    let asked = calls_of(&bench, "preview");
    let call = ["preview", "main", "R1", "--json"]
        .map(String::from)
        .to_vec();
    assert_eq!(asked, [(call, kept(&bench))]);
    let previewed = &bench.report().previews[&id("home/30")];
    assert_eq!(previewed.outcome, Outcome::Done);
    assert_eq!(previewed.text.as_deref(), Some(STARTING));
    assert_eq!(previewed.words, None);
    assert!(
        lines.contains(&"home/30: preview of main at R1: brought up, starting".to_string()),
        "{lines:?}"
    );
    assert_eq!(plane.held().unwrap().previews[&id("home/30")], *previewed);
}

/// The previews spec's "A preview sluice refuses".
#[test]
fn a_preview_sluice_refuses() {
    let bench = bench();
    bench
        .sluice
        .answers_preview("preview", "preview-no-table.json", 2);
    let mut plane = Fake::holding(vec![preview("home/30", "main", "R1", false)]);
    let (_, lines) = bench.pass(&mut plane);
    let message = "no [preview] table in .config/sluice.toml at 99248c2";
    let fixture: serde_json::Value =
        serde_json::from_str(&preview_fixture("preview-no-table.json")).unwrap();
    assert_eq!(fixture["error"]["message"], message);
    let previewed = &bench.report().previews[&id("home/30")];
    assert_eq!(previewed.outcome, Outcome::Refused);
    assert_eq!(previewed.words.as_deref(), Some(message));
    assert_eq!(previewed.text, None);
    assert!(
        lines.contains(&format!(
            "home/30: preview of main at R1: refused by sluice: {message}"
        )),
        "{lines:?}"
    );
    assert!(!commands(&bench).contains(&"previews".to_string()));
}

/// The previews spec's "A preview given twice": the second pass, after
/// the runner restarts, runs no `sluice preview`, and reports the answer
/// with its time as it stood.
#[test]
fn a_preview_given_twice() {
    let bench = bench();
    bench
        .sluice
        .answers_preview("preview", "preview-starting.json", 0);
    let mut plane = Fake::holding(vec![preview("home/30", "main", "R1", false)]);
    bench.pass(&mut plane);
    let first = bench.report().previews[&id("home/30")].clone();
    std::thread::sleep(std::time::Duration::from_millis(1100));
    let mut restarted = Fake::holding(vec![preview("home/30", "main", "R1", false)]);
    bench.pass(&mut restarted);
    assert_eq!(calls_of(&bench, "preview").len(), 1);
    let sent = &restarted.reports().last().unwrap().previews[&id("home/30")];
    assert_eq!(sent.outcome, Outcome::Done);
    assert_eq!(sent.text, first.text);
    assert_eq!(sent.at, first.at);
}

/// The previews spec's "A preview in a repository not kept".
#[test]
fn a_preview_in_a_repository_not_kept() {
    let bench = Bench::new();
    let want = in_repository(preview("home/30", "main", "R1", false), "example/elsewhere");
    let mut plane = Fake::holding(vec![want]);
    let (_, lines) = bench.pass(&mut plane);
    let previewed = &bench.report().previews[&id("home/30")];
    assert_eq!(previewed.outcome, Outcome::Refused);
    assert_eq!(
        previewed.words.as_deref(),
        Some("this host's configuration does not keep example/elsewhere")
    );
    assert!(
        lines.contains(
            &"home/30: preview of main at R1: refused: \
              this host's configuration does not keep example/elsewhere"
                .to_string()
        ),
        "{lines:?}"
    );
    assert!(bench.sluice.calls().is_empty());
}

/// The previews spec's "A preview in a repository whose clone is not yet
/// made".
#[test]
fn a_preview_in_a_repository_whose_clone_is_not_yet_made() {
    let bench = Bench::new();
    bench.clones.not_yet("example/crew");
    let want = in_repository(preview("home/30", "main", "R1", false), "example/crew");
    let mut plane = Fake::holding(vec![want]);
    bench.pass(&mut plane);
    assert!(!report(&bench).previews.contains_key(&id("home/30")));
    assert!(plane.reports().iter().all(|sent| sent.previews.is_empty()));
    assert!(bench.sluice.calls().is_empty());
}

/// The previews spec's "A preview whose lease lapsed".
#[test]
fn a_preview_whose_lease_lapsed() {
    let bench = Bench::new();
    let want = expiring(
        preview("home/30", "main", "R1", false),
        "2026-11-01T10:00:00Z",
    );
    let mut plane = Fake::holding(vec![want]);
    bench.pass(&mut plane);
    assert!(!report(&bench).previews.contains_key(&id("home/30")));
    assert!(plane.reports().iter().all(|sent| sent.previews.is_empty()));
    assert!(bench.sluice.calls().is_empty());
}

/// The previews spec's "A preview waiting at sluice's bound": the ask is
/// interrupted at the runner's patience, the preview held and said once,
/// and asked again at the next pass (design Decision 3).
#[test]
fn a_preview_waiting_at_sluices_bound() {
    let bench = bench();
    bench.sluice_patience.set(std::time::Duration::from_secs(1));
    bench.sluice.waits("preview");
    let mut plane = Fake::holding(vec![preview("home/30", "main", "R1", false)]);
    let began = std::time::Instant::now();
    let (_, lines) = bench.pass(&mut plane);
    assert!(began.elapsed() < std::time::Duration::from_secs(5));
    let previewed = &bench.report().previews[&id("home/30")];
    assert_eq!(previewed.outcome, Outcome::Held);
    assert_eq!(previewed.text, None);
    let waits =
        "home/30: preview of main at R1: waits at sluice's bound, asked again at the next pass";
    assert!(lines.contains(&waits.to_string()), "{lines:?}");

    bench.sluice.waits_no_more("preview");
    bench
        .sluice
        .answers_preview("preview", "preview-starting.json", 0);
    let (_, lines) = bench.pass(&mut plane);
    let call = ["preview", "main", "R1", "--json"]
        .map(String::from)
        .to_vec();
    let asked: Vec<Vec<String>> = calls_of(&bench, "preview")
        .into_iter()
        .map(|(call, _)| call)
        .collect();
    assert_eq!(asked, [call.clone(), call]);
    let previewed = &bench.report().previews[&id("home/30")];
    assert_eq!(previewed.outcome, Outcome::Done);
    assert_eq!(previewed.text.as_deref(), Some(STARTING));
    assert!(
        !lines
            .iter()
            .any(|line| line.contains("waits at sluice's bound")),
        "{lines:?}"
    );
}
