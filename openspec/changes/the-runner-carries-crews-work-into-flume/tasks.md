# Tasks

Build the groups in order. Each is one session's work for an agent
that reads this change (proposal, design, specs) and its group, and
builds on the groups before it. Each task is true once the moon task it
names passes, run as it names it.

`the-runner-keeps-its-repositories` is built first: its configuration
file and reader, its data directory, its long-running process and
timer, and its clones (design, *What this change rests on*, 1). Where
one is missing when a task needs it, stop short with
`flume needs --found`, naming it.

Tests reach no network, run no real Flume, and touch no configuration
or data of the user's (design Decision 17):

- every state repository is a bare repository the test makes, reached
  by its file URL;
- every `flume` is the fake `Flume` or a stub program the test writes
  into its own temporary directory;
- every data directory is a temporary directory of the test's own.

## 1. The control-plane interface

- [ ] 1.1 `crates/flywheel-control` is a library crate in the cargo workspace's `members`, and a moon project `flywheel-control` in `.moon/workspace.yml` with its language declared as rust. Its check-fast tasks, format, lint, typecheck and test, are one task per check, each declaring its inputs with `/devenv.nix`, `/devenv.lock` and `/.prototools` among them (design Decision 1; core.7; Task notes 1.1). True when `devenv shell -- moon run flywheel-control:format flywheel-control:lint flywheel-control:typecheck flywheel-control:test` passes.
- [ ] 1.2 The `ControlPlane` trait, its pull and report, and the types of what a control plane wants and of the runner's record exist as design Decision 2 has them, the record round-tripping as JSON, and a carried place's time moving only when its text changes (Task notes 1.2). True when `devenv shell -- moon run flywheel-control:test` passes with a round-trip test of each type and of `Record::carry`.
- [ ] 1.3 The fake control plane holds the wants a test sets, refuses a pull when a test says so, gives back the last report, and lets go what a test names. The shared cases run against it: wants in order, the record coming back, a second report changing nothing, a run's `wanted` as held (control-plane-interface spec "A pull gives what the control plane wants of the host", "A launch carries everything Flume is to be given", "The runner's record is reported whole", "A report lets go of the places the control plane carries no more" and "Every implementation passes the shared cases"; Task notes 1.3). True when `devenv shell -- moon run flywheel-control:test` passes with a test per scenario of those requirements, the shared cases run against `Fake` among them.

## 2. Flume's command line

- [ ] 2.1 The `Flume` trait and its command-line implementation build each act's argv as design Decision 5 has it, with `--json` and no `FLUME_RECIPIENT`. They read exit `0`, `1` and `2` as Decision 6 does. The flume 0.3.6 fixtures are kept under `crates/flywheel/tests/fixtures/flume-0.3.6/` with their README, and `treefmt.nix` excludes them by name with its reason (core.7; Task notes 2.1). True when `devenv shell -- moon run flywheel:test root:format` passes, with a stub-program test of each act's argv, of each exit, and of a program that is not there.
- [ ] 2.2 One place's facts are read from its status, each guarded: its location, its mark, its phase (live or making; landed with its commit; dropped; done), and a wait of `delivered`. Each place's raw text is sliced out of a listing byte for byte (design Decision 10; Task notes 2.2). True when `devenv shell -- moon run flywheel:test` passes with a test over every fixture of 2.1.

## 3. Requests carried into Flume

- [ ] 3.1 A pass takes the record, pulls, reads Flume once, and turns each launch into `flume launch` in its clone with `--stream`, `--priority` and `--intent`. It refuses a launch lacking one, and answers each launch as Flume gave it (design Decisions 4 to 6; work-into-flume spec "A launch is one `flume launch` naming its stream, priority and intent" and "An answer is Flume's, as Flume gave it"; Task notes 3.1). True when `devenv shell -- moon run flywheel:test` passes with a test per scenario of those two requirements, over `Fake`, a fake `Flume` and a fake `Clones`.
- [ ] 3.2 A launch whose After work has not landed is answered `held` with words naming what it waits on, and launched once Flume reads each place landed, with the newest priority. A launch dropped before it was launched is never made, and an act on no launch here is refused (design Decision 8; work-into-flume spec "A launch waits for its After work to land" and "A launch dropped before it was launched is never made"; Task notes 3.2). True when `devenv shell -- moon run flywheel:test` passes with a test per scenario of those two requirements.
- [ ] 3.3 Each request is acted on once. The record is saved in the data directory after each act and read at start, and a launch with no answer whose mark Flume lists is answered `launched` without a second launch (design Decision 7; work-into-flume spec "Each request is acted on once"; Task notes 3.3). True when `devenv shell -- moon run flywheel:test` passes with a test of a restart after a retry and one of an answer lost with the data directory.
- [ ] 3.4 A retry, a drop and a priority of a launched launch run `flume retry`, `flume drop` and `flume priority` on its place, with no `--force`, each answered as Flume gave it (design Decisions 5 and 6; work-into-flume spec "A retry, a drop and a priority are Flume's own commands on the launch's place"; Task notes 3.4). True when `devenv shell -- moon run flywheel:test` passes with a test per act and one of a refusal.
- [ ] 3.5 Each launched place's status is carried as its text in Flume's listing, its time moving only on a change, and its last status standing once Flume answers nothing of it. Only the runner's own places are acted on. A repository the configuration does not name is refused, a clone not yet made is left unanswered, and the keeper is told once of each landing (design Decisions 9 and 10; work-into-flume spec "Flume's status of every place it launched is carried as Flume gave it", "The runner acts only on its own places" and "Work only in the repositories its configuration names"; Task notes 3.5). True when `devenv shell -- moon run flywheel:test` passes with a test per scenario of those three requirements.

## 4. Role runs

- [ ] 4.1 Each run a pull gives is launched only while wanted with no place of it standing, with no brief and no recipient, declared done once its place waits `delivered`, recorded by its key, and recorded with its words when it cannot be launched (design Decision 11; role-runs spec; Task notes 4.1). True when `devenv shell -- moon run flywheel:test` passes with a test per scenario of the role-runs spec.

## 5. crew's implementation

- [ ] 5.1 `crates/flywheel-crew` is a library crate in the cargo workspace's `members`, and a moon project `flywheel-crew` in `.moon/workspace.yml` with its language declared as rust. Its check-fast tasks, format, lint, typecheck and test, are one task per check, each declaring its inputs with `/devenv.nix`, `/devenv.lock`, `/.prototools` and `/crates/flywheel-control/**/*` among them. It depends on weir-branch and weir-capture at weir's `0b6eb5abb806a51fec128b01f97c4dcff6170219`, and `devenv.nix` pins recutils among its packages (design Decisions 1 and 17; core.7, core.8; Task notes 5.1). True when `devenv shell -- moon run flywheel-crew:format flywheel-crew:lint flywheel-crew:typecheck flywheel-crew:test root:toolchain` passes.
- [ ] 5.2 crew's table is read from the `[control_plane.crew]` configuration: `host`, `partitions` and `kinds`, each refusal naming its key (design Decision 3; crew-control-plane spec "crew's implementation is configured per host and partition"; Task notes 5.2). True when `devenv shell -- moon run flywheel-crew:test` passes with a test per scenario of that requirement and of Decision 3's example.
- [ ] 5.3 Each pull reads `requests.rec` at one tip of each partition's `<label>/main`, and gives this host's requests in order with every field crew's request names. A request it cannot read is given so the runner refuses it, and a partition it cannot read gives nothing (design Decision 12; crew-control-plane spec "The requests for this host are read from `<label>/main`'s tip" and "A request or a partition that cannot be read stops nothing else"; Task notes 5.3). True when `devenv shell -- moon run flywheel-crew:test` passes with a test per scenario of those two requirements over a bare state repository.
- [ ] 5.4 Each report writes `runner.rec`, `answers.rec` and `places.rec` on `<label>/runner/<host>` by weir-branch's write, with Decision 14's descriptors. Nothing is written when unchanged, no other ref is pushed, and the record is read back from the branch (design Decisions 13 and 14; crew-control-plane spec "The runner's branch is written by weir-branch's write", "No branch but the runner's own is pushed", "An answer is one record per request read", "A place is carried with Flume's status as one line" and "The record is read back from the runner's own branch"; Task notes 5.4). True when `devenv shell -- moon run flywheel-crew:test` passes with a test per scenario of those five requirements over a bare state repository.
- [ ] 5.5 Each role record for this host is given as a run, wanted while anything is pending for the role as crew's `asks.py` at `a5afd9ec` reads it, every host's runner branch read for waits (design Decision 15; crew-control-plane spec "Each role record of this host is a run" and "A role's run is wanted while anything is pending for it, read as crew reads it"; Task notes 5.5). True when `devenv shell -- moon run flywheel-crew:test` passes with a test per scenario of those two requirements and of each case Task notes 5.5 names.
- [ ] 5.6 A place Flume no longer lists keeps its record until its work has left crew's plan, and is then let go and named in the report's answer (design Decision 13; crew-control-plane spec "A tidied place's record is kept until its work has left crew's plan"; Task notes 5.6). True when `devenv shell -- moon run flywheel-crew:test` passes with a test per scenario of that requirement and of a fix of main kept.
- [ ] 5.7 crew's implementation passes `flywheel-control`'s shared cases (control-plane-interface spec "Every implementation passes the shared cases"; Task notes 5.7). True when `devenv shell -- moon run flywheel-crew:test` passes with the shared cases run against crew's implementation.

## 6. The runner runs its control plane

- [ ] 6.1 The runner opens the control plane its `[control_plane]` table names from its one list of implementations, refuses a word it does not carry naming `crew`, and runs a pass every `every` on the runner's timer, the keeper giving its clones (design Decisions 1, 3 and 4; control-plane-interface spec "The configuration chooses the implementation"; Task notes 6.1). True when `devenv shell -- moon run flywheel:test` passes with a test per scenario of that requirement and of `every` in each of Flume's time forms.
- [ ] 6.2 Only `planes.rs` and crew's crate name `flywheel_crew`, and the interface's crate depends on no implementation. No crate's sources name a listening socket. Every crate is a moon project with its four check-fast tasks, whose inputs name the sources of every crate it builds (design Decision 1; core.2, core.3, core.7; control-plane-interface spec "The control plane is reached through one interface"; work-into-flume spec "No connection comes in to the host"; Task notes 6.2). True when `devenv shell -- moon run flywheel:test` passes with `crates/flywheel/tests/rules.rs`.
- [ ] 6.3 One pass end to end, crew's implementation over a bare state repository and a stub `flume`: a give of `a` and `b`, `b` after `a`, answered `a` launched and `b` held, then `b` launched on `bolt/x` once `a`'s place reads landed (design Decisions 4, 8 and 13; Task notes 6.3). True when `devenv shell -- moon run flywheel:test` passes with `crates/flywheel/tests/pass.rs`.
- [ ] 6.4 README says what the runner carries and shows the `[control_plane]` table, and CLAUDE.md's opening says what the crate now does. README's example opens crew's implementation (Task notes 6.4). True when `devenv shell -- moon run flywheel:test` passes with `crates/flywheel/tests/readme.rs`.

## 7. The checks

- [ ] 7.1 The checks of the moon projects this change touches pass (Task notes 7.1). True when `devenv shell -- moon run flywheel:format flywheel:lint flywheel:typecheck flywheel:test flywheel-control:format flywheel-control:lint flywheel-control:typecheck flywheel-control:test flywheel-crew:format flywheel-crew:lint flywheel-crew:typecheck flywheel-crew:test root:format root:toolchain root:changes` passes.
