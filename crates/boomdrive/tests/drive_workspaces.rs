//! A drive's workspace goes with its drive (the views spec's "A drive's
//! workspace goes with its drive"; core.43; record 0017 as revised;
//! design Decisions 7 and 8): each kept repository's clone records the
//! drive the configuration names it under, and once a drive has left the
//! configuration and no clone the runner holds records it, the pass
//! removes the drive's workspace at the repository root, and no other.

mod common;

use boomdrive::pass::Outcome;
use common::{drives, keeping, Scratch};

/// The drive the clone of `address` records, `boomdrive.drive` in its
/// bare repository's own configuration.
fn recorded_drive(scratch: &Scratch, address: &str) -> String {
    scratch.git(
        &scratch.clone_of(address),
        &["config", "--local", "--get", "boomdrive.drive"],
    )
}

/// The clone records the drive its repository is named under, and the
/// one it is named under next where that changes.
#[test]
fn the_clone_records_its_drive() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");

    let (outcome, _) = scratch.pass(&keeping(&["agentplot/flume"]));

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(recorded_drive(&scratch, "agentplot/flume"), "home");

    let (outcome, lines) = scratch.pass(&drives(&[("work", &["agentplot/flume"])]));

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, [] as [String; 0], "a record is said nowhere");
    assert_eq!(recorded_drive(&scratch, "agentplot/flume"), "work");
}
