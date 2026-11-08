# Tasks

Build the tasks in order. A task is true once the check it names passes, run as it names it.

- Every test runs over a bare state repository in a temporary folder, through `crates/boomdrive-crew/tests/common`, with neutral names: partition `madswan`, hosts `deneb` and `vega` (core.41). No test reads the user's configuration, touches a real place or state repository, or reaches the network.
- Each new test goes in `crates/boomdrive-crew/tests/drops.rs`, named as design Decision 6's table names it.
- No task adds a crate, a tool or a tracked file type (core.7, core.8). Nothing outside the files proposal.md's Touches lists is changed.

## 1. A launch a drop names is offered only where the runner launched it

- [ ] 1.1 `Main` holds the launches a drop names, read from `requests.rec` at the tip, whatever each drop's `Host`, `Work` or place in the file. `pull_partition` gives no offer for a dropped launch the runner has not launched, by `Pulling::launched` (design Decisions 1 and 2; Task notes 1.1 and 1.2). True when `devenv shell -- cargo test -p boomdrive-crew --test drops a_drop_naming_another_host` passes, proving the crew-control-plane spec's "A drop naming another host".
- [ ] 1.2 A held launch a drop names is given no offer, and its drop naming this host is given. The test's plan holds the launch's unit, so the plan alone gives the drop. True when `devenv shell -- cargo test -p boomdrive-crew --test drops a_held_launch_dropped` passes, proving "A held launch dropped".
- [ ] 1.3 A dropped launch the runner launched is still given, with its drop. True when `devenv shell -- cargo test -p boomdrive-crew --test drops a_dropped_launch_the_runner_launched` passes, proving "A dropped launch the runner launched".
- [ ] 1.4 A kit's launch a drop names is never claimed, and no branch is written for it. True when `devenv shell -- cargo test -p boomdrive-crew --test drops a_kits_launch_dropped_before_it_is_claimed` passes, proving "A kit's launch dropped before it is claimed".
- [ ] 1.5 A claim of a dropped launch is not given. No act naming no host goes with it, as `own_claims` leaves dropped launches out. The report's write keeps no claim of a dropped launch (design Decision 2; Task notes 1.3). True when `devenv shell -- cargo test -p boomdrive-crew --test drops a_claimed_launch_dropped` passes, proving "A claimed launch dropped".
- [ ] 1.6 The claims of launches no drop names stand as they were. True when `devenv shell -- cargo test -p boomdrive-crew --test claims` passes, proving the MODIFIED requirements' scenarios that design Decision 6 names in `tests/claims.rs`.

## 2. A drop reaches the runner that holds its launch

- [ ] 2.1 A request whose work has left the plan, which the runner does not hold, is given nothing, as built (design Decision 5). True when `devenv shell -- cargo test -p boomdrive-crew --test drops work_gone_that_the_runner_does_not_hold` passes, proving "Work gone that the runner does not hold".
- [ ] 2.2 `Pulling::current` passes a drop whose launch the runner's report or this host's branch answers, a claim apart, whatever the plan holds (design Decision 4; Task notes 1.4). True when `devenv shell -- cargo test -p boomdrive-crew --test drops a_drop_written_as_its_work_leaves_the_plan` passes, proving "A drop written as its work leaves the plan".
- [ ] 2.3 A drop naming no host goes to the runner that holds its launch, and to no other. True when `devenv shell -- cargo test -p boomdrive-crew --test drops a_kits_drop_goes_to_the_runner_holding_its_launch` passes, proving "A kit's drop goes to the runner holding its launch".

## 3. A dropped launch the runner holds unlaunched is let go

- [ ] 3.1 `write::let_go` lets go a launch a drop names whose answer is not `launched`, whatever the plan holds, and counts it among the launches gone for its acts (design Decision 3; Task notes 1.5). True when `devenv shell -- cargo test -p boomdrive-crew --test drops a_held_launch_dropped_is_let_go` passes, proving "A held launch dropped is let go".
- [ ] 3.2 A drop the runner answered from its report goes with its launch once its work has left the plan. True when `devenv shell -- cargo test -p boomdrive-crew --test drops a_drop_answered_from_the_report_its_work_gone` passes, proving "A drop answered from the report, its work gone".
- [ ] 3.3 A dropped launch the runner launched, and its drop, are let go only once Flume no longer lists its place. True when `devenv shell -- cargo test -p boomdrive-crew --test drops a_dropped_launch_the_runner_launched_stays_until_its_place_is_over` passes, proving "A dropped launch the runner launched stays until its place is over".

## 4. What it says, and its spec

- [ ] 4.1 README's section on crew's table says that crew's implementation offers and claims no launch a drop names unless the runner launched it, lets go one the runner holds unlaunched, and gives a drop to the runner that holds its launch. True when `sluice exec main -- moon run root:format` passes.
- [ ] 4.2 The spec delta is archived into `openspec/specs/crew-control-plane/spec.md`. True when, after `devenv shell -- openspec archive a-dropped-launch-is-never-offered --yes`, `sluice exec main -- moon run root:specs` passes.
