//! The preview a control plane asks for, brought up through a stub
//! `sluice` in a repository the runner keeps, carried as sluice listed it
//! with no output of its command, and put away on its done only where the
//! runner asked sluice for it (the previews spec; design Decisions 2 to 8;
//! core.55; Task notes 3.1 and group 5).

mod fakes;

use boomdrive_control::fake::Fake;
use boomdrive_control::{At, Id, Lease, Outcome, Report, Want};
use fakes::{preview, Bench};

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
