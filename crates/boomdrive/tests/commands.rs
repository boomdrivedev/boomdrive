//! The commands the control plane sends, each under a lease of its own:
//! this binary carries no tool's command, so every one is refused, naming
//! its tool, a branch command's being sluice, and runs nothing, whatever
//! its repository and whether or not its clone is made (core.16, core.52;
//! design Decision 4; the commands spec; Task notes 1.1). A command the
//! report holds is answered from it, one an earlier release carried out
//! among them (core.10, core.36; design Decision 5; Task notes 1.2). It
//! runs over a stub `sluice`, which no command runs, and a scratch forge
//! where a scenario needs a clone made.

mod common;
mod fakes;

use boomdrive::clone;
use boomdrive::config::Address;
use boomdrive::report;
use boomdrive_control::fake::Fake;
use boomdrive_control::{At, Command, CommandSort, Commanded, Id, Lease, Outcome, Report, Want};
use common::Scratch;
use fakes::Bench;
use std::fs;

/// The repository the bolt is in.
const CREW: &str = "example/crew";

/// The command's id.
const ID: &str = "branch/1";

/// The words a branch command is refused with.
const REFUSED: &str = "this binary does not carry sluice's commands";

/// The command `branch/1`, under lease 1, asking for `what`.
fn command(what: CommandSort) -> Want {
    Want::Command(Command {
        id: Id::new(ID),
        lease: Lease {
            number: 1,
            expires: None,
        },
        what,
    })
}

/// The command `branch/1` asking for `bolt/x` from `main` in
/// `repository`.
fn bolt_in(repository: &str) -> Want {
    command(CommandSort::Branch {
        repository: repository.to_string(),
        branch: "bolt/x".to_string(),
        from: "main".to_string(),
    })
}

/// A bench keeping `example/crew`, whose forge on `scratch` holds
/// `main`, its clone made by the runner's own `clone::make`.
fn keeping_crew(scratch: &Scratch) -> Bench {
    scratch.origin(CREW);
    let git = scratch.runner_git();
    let container =
        clone::make(&git, &scratch.data(), &Address::parse(CREW).unwrap()).expect("the clone made");
    let mut bench = Bench::new();
    bench.git = git;
    bench.clones.ready(CREW, &container);
    bench
}

/// What the report file holds of the command.
fn commanded(bench: &Bench) -> Commanded {
    bench.report().commands[&Id::new(ID)].clone()
}

/// Asserts that `bench`'s pass ran no tool for a command: no `sluice` and
/// no `flume` but its listing.
fn ran_nothing(bench: &Bench) {
    assert!(
        bench.sluice.calls().is_empty(),
        "{:?}",
        bench.sluice.calls()
    );
    assert!(
        bench.flume.commands().is_empty(),
        "{:?}",
        bench.flume.commands()
    );
}

/// The commands spec's "A branch command".
#[test]
fn a_branch_command() {
    let scratch = Scratch::new();
    let bench = keeping_crew(&scratch);
    let mut plane = Fake::holding(vec![bolt_in(CREW)]);
    let (passed, lines) = bench.pass(&mut plane);
    assert!(passed.reported, "{lines:?}");
    let commanded = commanded(&bench);
    assert_eq!(commanded.outcome, Outcome::Refused);
    assert_eq!(commanded.words.as_deref(), Some(REFUSED));
    assert_eq!(commanded.text, None);
    assert_eq!(commanded.origin_commit, None);
    assert_eq!(lines, [format!("{ID}: refused: {REFUSED}")]);
    ran_nothing(&bench);
    let sent = plane.reports().last().expect("a report sent");
    assert_eq!(sent.commands[&Id::new(ID)], commanded);
}

/// The commands spec's "A clone not yet made": refused, naming sluice,
/// rather than left untaken; and so is one in a repository the
/// configuration does not keep.
#[test]
fn a_clone_not_yet_made() {
    let bench = Bench::new();
    bench.clones.not_yet(CREW);
    let mut plane = Fake::holding(vec![bolt_in(CREW)]);
    let (passed, lines) = bench.pass(&mut plane);
    assert!(passed.reported, "{lines:?}");
    let commanded = commanded(&bench);
    assert_eq!(commanded.outcome, Outcome::Refused);
    assert_eq!(commanded.words.as_deref(), Some(REFUSED));
    assert_eq!(commanded.text, None);
    assert_eq!(commanded.origin_commit, None);
    assert_eq!(lines, [format!("{ID}: refused: {REFUSED}")]);
    ran_nothing(&bench);

    let unkept = Bench::new();
    let mut plane = Fake::holding(vec![bolt_in("agentplot/sluice")]);
    unkept.pass(&mut plane);
    let refused = unkept.report().commands[&Id::new(ID)].clone();
    assert_eq!(refused.words.as_deref(), Some(REFUSED));
    ran_nothing(&unkept);
}

/// The commands spec's "A boom command".
#[test]
fn a_boom_command() {
    let bench = Bench::new();
    let boom = command(CommandSort::Other {
        tool: "boom".to_string(),
        words: "go off".to_string(),
    });
    let mut plane = Fake::holding(vec![boom]);
    let (_, lines) = bench.pass(&mut plane);
    let commanded = commanded(&bench);
    assert_eq!(commanded.outcome, Outcome::Refused);
    assert_eq!(
        commanded.words.as_deref(),
        Some("this binary does not carry boom's commands")
    );
    assert_eq!(commanded.text, None);
    assert_eq!(commanded.origin_commit, None);
    assert_eq!(
        lines,
        [format!(
            "{ID}: refused: this binary does not carry boom's commands"
        )]
    );
    ran_nothing(&bench);
}

/// The commands spec's "A command given twice": the runner restarts
/// between the passes, and its control plane, a new one, gives it no
/// report; then its report file is lost, and the control plane's report
/// answers it.
#[test]
fn a_command_given_twice() {
    let scratch = Scratch::new();
    let bench = keeping_crew(&scratch);
    let mut plane = Fake::holding(vec![bolt_in(CREW)]);
    let (_, lines) = bench.pass(&mut plane);
    assert_eq!(lines, [format!("{ID}: refused: {REFUSED}")]);
    let first = commanded(&bench);
    assert_eq!(first.outcome, Outcome::Refused);

    let mut restarted = Fake::holding(vec![bolt_in(CREW)]);
    let (passed, lines) = bench.pass(&mut restarted);
    assert!(passed.reported, "{lines:?}");
    assert!(lines.is_empty(), "{lines:?}");
    assert_eq!(commanded(&bench), first);
    let sent = restarted.reports().last().expect("a report sent");
    assert_eq!(sent.commands[&Id::new(ID)], first);
    let hello = &restarted.hellos()[0];
    assert_eq!(hello.leases.get(&Id::new(ID)), Some(&1));

    fs::remove_file(bench.report_file()).expect("the report file removed");
    let (_, lines) = bench.pass(&mut plane);
    assert!(lines.is_empty(), "{lines:?}");
    assert_eq!(commanded(&bench), first);
    ran_nothing(&bench);
}

/// The commands spec's "A branch command an earlier release carried out":
/// the report file holds `branch/1` as an earlier release answered it,
/// with sluice's text and the origin commit `C`.
#[test]
fn a_branch_command_an_earlier_release_carried_out() {
    let scratch = Scratch::new();
    let bench = keeping_crew(&scratch);
    let carried = Commanded {
        lease: Lease {
            number: 1,
            expires: None,
        },
        outcome: Outcome::Done,
        text: Some(
            "{\n  \"branch\": \"bolt/x\",\n  \"created\": \"C\",\n  \"from\": \"main\"\n}\n"
                .to_string(),
        ),
        words: None,
        origin_commit: Some("C".to_string()),
        at: At::parse("2026-11-06T10:00:00Z").unwrap(),
    };
    let mut earlier = Report::default();
    earlier.commands.insert(Id::new(ID), carried.clone());
    report::save(&bench.report_file(), &earlier).expect("the report file written");

    let mut plane = Fake::holding(vec![bolt_in(CREW)]);
    let (passed, lines) = bench.pass(&mut plane);
    assert!(passed.reported, "{lines:?}");
    assert!(lines.is_empty(), "{lines:?}");
    assert_eq!(commanded(&bench), carried);
    let sent = plane.reports().last().expect("a report sent");
    assert_eq!(sent.commands[&Id::new(ID)], carried);
    ran_nothing(&bench);
}
