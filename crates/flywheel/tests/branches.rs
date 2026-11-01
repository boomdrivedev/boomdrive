//! A bolt's branch, made by sluice in the runner's clone when the control
//! plane sends that command: sluice's answer carried as given, and the
//! commit origin's branch stands at read from origin, never from sluice's
//! words. It runs over a stub `sluice` answering with sluice 0.61.1's own
//! answers, and a scratch forge holding the repository (design Decision
//! 15; core.5, core.9, core.10; the branch-commands spec; Task notes 6.1).

mod common;
mod fakes;

use common::{Origin, Scratch};
use fakes::{sluice_fixture, Bench, SluiceStub};
use flywheel::clone;
use flywheel::config::Address;
use flywheel_control::fake::Fake;
use flywheel_control::{Command, CommandSort, Commanded, Id, Lease, Outcome, Want};
use serde_json::json;
use std::fs;

/// The repository the bolt is in.
const CREW: &str = "afterthought/crew";

/// The commit sluice's captured answers name, the scratch forge's they
/// were read on.
const CAPTURED: &str = "ee68c82edcfe7ff041906f0e2842921de57f6d35";

/// The command's id.
const ID: &str = "branch/1";

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

/// A bench keeping `afterthought/crew`, whose forge on `scratch` holds
/// `main`, its clone made by the runner's own `clone::make`.
struct Host<'s> {
    bench: Bench,
    origin: Origin<'s>,
}

impl<'s> Host<'s> {
    fn new(scratch: &'s Scratch) -> Host<'s> {
        let origin = scratch.origin(CREW);
        let git = scratch.runner_git();
        let container = clone::make(&git, &scratch.data(), &Address::parse(CREW).unwrap())
            .expect("the clone made");
        let mut bench = Bench::new();
        bench.git = git;
        bench.clones.ready(CREW, &container);
        Host { bench, origin }
    }

    /// The stub `sluice` making `bolt/x` on the forge at `commit` once it
    /// has answered, as sluice makes it on origin.
    fn makes_the_branch_at(&self, commit: &str) {
        let script = format!(
            "GIT_CONFIG_NOSYSTEM=1 GIT_CONFIG_GLOBAL=/dev/null \
             git --git-dir '{}' update-ref refs/heads/bolt/x {commit}\n",
            self.origin.bare.display()
        );
        self.bench.sluice.then("branch", &script);
    }
}

/// What the report file holds of the command.
fn commanded(bench: &Bench) -> Commanded {
    bench.report().commands[&Id::new(ID)].clone()
}

/// The branch-commands spec's "No gate for the branch".
#[test]
fn no_gate_for_the_branch() {
    let scratch = Scratch::new();
    let host = Host::new(&scratch);
    host.bench
        .sluice
        .answers("branch", "branch-ungated.json", 2);
    let mut plane = Fake::holding(vec![bolt_in(CREW)]);
    let (passed, lines) = host.bench.pass(&mut plane);
    assert!(passed.reported, "{lines:?}");
    let commanded = commanded(&host.bench);
    assert_eq!(commanded.outcome, Outcome::Refused);
    assert_eq!(
        commanded.words.as_deref(),
        Some("no [branches] table in .config/sluice.toml on main names bolt/x")
    );
    assert_eq!(commanded.text, Some(sluice_fixture("branch-ungated.json")));
    assert_eq!(commanded.origin_commit, None);
    assert_eq!(
        lines,
        [format!(
            "{ID}: branch bolt/x from main in {CREW}: refused by sluice: \
             no [branches] table in .config/sluice.toml on main names bolt/x; \
             origin holds no bolt/x"
        )]
    );
}

/// The branch-commands spec's "A branch made". sluice's answer names the
/// commit of the forge it was captured on; the runner carries the one
/// origin holds.
#[test]
fn a_branch_made() {
    let scratch = Scratch::new();
    let host = Host::new(&scratch);
    let main = host.origin.tip("main");
    assert_ne!(main, CAPTURED);
    host.bench.sluice.answers("branch", "branch-made.json", 0);
    host.makes_the_branch_at(&main);
    let mut plane = Fake::holding(vec![bolt_in(CREW)]);
    let (passed, lines) = host.bench.pass(&mut plane);
    assert!(passed.reported, "{lines:?}");
    assert_eq!(
        host.bench.sluice.calls(),
        [["branch", "create", "bolt/x", "--from", "main", "--json"]]
    );
    assert_eq!(
        host.bench.sluice.dir_of(1),
        Some(scratch.container(CREW).canonicalize().unwrap())
    );
    let commanded = commanded(&host.bench);
    assert_eq!(commanded.outcome, Outcome::Done);
    assert_eq!(commanded.text, Some(sluice_fixture("branch-made.json")));
    assert_eq!(commanded.words, None);
    assert_eq!(commanded.origin_commit, Some(main.clone()));
    assert_eq!(
        lines,
        [format!(
            "{ID}: branch bolt/x from main in {CREW}: done; origin holds bolt/x at {main}"
        )]
    );
    let sent = plane.reports().last().expect("a report sent");
    assert_eq!(sent.commands[&Id::new(ID)], commanded);
}

/// The branch-commands spec's "A branch already there".
#[test]
fn a_branch_already_there() {
    let scratch = Scratch::new();
    let host = Host::new(&scratch);
    let there = host.origin.commit("bolt/x", "x", "a bolt\n");
    let printed = sluice_fixture("branch-there.json").replace(CAPTURED, &there);
    host.bench.sluice.answers_text("branch", &printed, 2);
    let mut plane = Fake::holding(vec![bolt_in(CREW)]);
    let (passed, lines) = host.bench.pass(&mut plane);
    assert!(passed.reported, "{lines:?}");
    let commanded = commanded(&host.bench);
    assert_eq!(commanded.outcome, Outcome::Refused);
    assert_eq!(commanded.text, Some(printed));
    assert_eq!(
        commanded.words,
        Some(format!("bolt/x is already there, at {there}"))
    );
    assert_eq!(commanded.origin_commit, Some(there));
}

/// The branch-commands spec's "A repository not kept".
#[test]
fn a_repository_not_kept() {
    let bench = Bench::new();
    let mut plane = Fake::holding(vec![bolt_in("agentplot/sluice")]);
    let (_, lines) = bench.pass(&mut plane);
    let commanded = commanded(&bench);
    assert_eq!(commanded.outcome, Outcome::Refused);
    assert_eq!(
        commanded.words.as_deref(),
        Some("this host's configuration does not keep agentplot/sluice")
    );
    assert_eq!(commanded.text, None);
    assert_eq!(commanded.origin_commit, None);
    assert_eq!(
        lines,
        [format!(
            "{ID}: refused: this host's configuration does not keep agentplot/sluice"
        )]
    );
    assert!(bench.sluice.calls().is_empty());
}

/// The branch-commands spec's "A clone not yet made".
#[test]
fn a_clone_not_yet_made() {
    let bench = Bench::new();
    bench.clones.not_yet(CREW);
    let mut plane = Fake::holding(vec![bolt_in(CREW)]);
    let (passed, lines) = bench.pass(&mut plane);
    assert!(passed.reported, "{lines:?}");
    assert!(lines.is_empty(), "{lines:?}");
    let sent = plane.reports().last().expect("a report sent");
    assert!(sent.commands.is_empty(), "{sent:?}");
    assert!(bench.sluice.calls().is_empty());
    bench.pass(&mut plane);
    let hello = plane.hellos().last().expect("a pull");
    assert!(hello.leases.is_empty(), "{:?}", hello.leases);
}

/// The branch-commands spec's "A command given twice": the runner
/// restarts between the passes, and its control plane, a new one, gives
/// it no report; then its report file is lost, and the control plane's
/// report answers it.
#[test]
fn a_command_given_twice() {
    let scratch = Scratch::new();
    let host = Host::new(&scratch);
    host.bench.sluice.answers("branch", "branch-made.json", 0);
    host.makes_the_branch_at(&host.origin.tip("main"));
    let mut plane = Fake::holding(vec![bolt_in(CREW)]);
    host.bench.pass(&mut plane);
    let first = commanded(&host.bench);
    let mut restarted = Fake::holding(vec![bolt_in(CREW)]);
    let (passed, lines) = host.bench.pass(&mut restarted);
    assert!(passed.reported, "{lines:?}");
    assert!(lines.is_empty(), "{lines:?}");
    assert_eq!(host.bench.sluice.calls().len(), 1);
    assert_eq!(commanded(&host.bench), first);
    let hello = &restarted.hellos()[0];
    assert_eq!(hello.leases.get(&Id::new(ID)), Some(&1));
    fs::remove_file(host.bench.report_file()).expect("the report file removed");
    let (_, lines) = host.bench.pass(&mut plane);
    assert!(lines.is_empty(), "{lines:?}");
    assert_eq!(host.bench.sluice.calls().len(), 1);
    assert_eq!(commanded(&host.bench), first);
}

/// The branch-commands spec's "A boom command".
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
    assert_eq!(
        lines,
        [format!(
            "{ID}: refused: this binary does not carry boom's commands"
        )]
    );
    assert!(bench.sluice.calls().is_empty());
    assert!(bench.flume.commands().is_empty());
}

/// The branch-commands spec's "A bolt's branch is made by sluice in the
/// clone": a command asks no room, so it is carried out on a host with
/// none (core.9 "Not ruled out").
#[test]
fn a_branch_is_made_whatever_the_room() {
    let scratch = Scratch::new();
    let host = Host::new(&scratch);
    host.bench.flume.host(json!({
        "running": 1, "bound": 1, "room": 0, "queue": [],
        "streams": [
            { "name": "default", "priority": 0, "running": 1, "bound": 1, "room": 0, "queue": [] }
        ],
        "pools": [], "addresses": [], "bases": [], "terminal_kept": false
    }));
    host.bench.sluice.answers("branch", "branch-made.json", 0);
    host.makes_the_branch_at(&host.origin.tip("main"));
    let mut plane = Fake::holding(vec![bolt_in(CREW)]);
    host.bench.pass(&mut plane);
    assert_eq!(host.bench.sluice.calls().len(), 1);
    assert_eq!(commanded(&host.bench).outcome, Outcome::Done);
}

/// Where sluice cannot be run, the command is left untaken and named,
/// and the next pass carries it out.
#[test]
fn a_command_sluice_could_not_run_waits_for_the_next_pass() {
    let scratch = Scratch::new();
    let mut host = Host::new(&scratch);
    let present = std::mem::replace(&mut host.bench.sluice, SluiceStub::absent());
    let mut plane = Fake::holding(vec![bolt_in(CREW)]);
    let (passed, lines) = host.bench.pass(&mut plane);
    assert!(passed.reported);
    assert_eq!(lines.len(), 1, "{lines:?}");
    assert!(
        lines[0].starts_with(&format!(
            "{ID}: branch bolt/x from main in {CREW}: not carried out: sluice could not run: "
        )),
        "{lines:?}"
    );
    let sent = plane.reports().last().expect("a report sent");
    assert!(sent.commands.is_empty(), "{sent:?}");
    host.bench.sluice = present;
    host.bench.sluice.answers("branch", "branch-made.json", 0);
    host.makes_the_branch_at(&host.origin.tip("main"));
    let (passed, lines) = host.bench.pass(&mut plane);
    assert!(passed.reported, "{lines:?}");
    assert_eq!(commanded(&host.bench).outcome, Outcome::Done);
}
