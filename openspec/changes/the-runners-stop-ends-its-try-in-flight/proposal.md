# Proposal

## Why

The brief: "A stop of the runner ends a call to a remote it has in flight as that call's limit would, asked to end and killed two seconds later, so the runner stops within a few seconds under any service manager and leaves no git running after it."

Record 0042 bounds each try of the runner's git call to a remote, a clone's fetch and crew's listing of runner branches, at thirty seconds. Each try runs in a process group of its own. The runner's stop is read between repositories and between passes, so a try in flight runs to its end or its limit before the runner stops: up to thirty seconds and two of grace. Record 0042 chose this. It did not take "ending the group in flight when the runner is asked to stop", since "the stop is already bounded by one try".

That bound is longer than a service manager waits. launchd kills a job twenty seconds after it asks the job to stop. The darwin module keeps launchd's default and ends the runner alone (`AbandonProcessGroup`), as the NixOS module does (`KillMode=process`), so a `flume launch` in flight runs to its end. A runner killed while a try is in flight leaves that try's git running in its own group, which nothing then ends: its limit was the runner's to enforce. `boomdrive run --once` takes no signal of its own, so a Ctrl-C ends it at once and leaves its try's git running the same way. Since record 0042, git in a group of its own no longer takes the terminal's Ctrl-C with the runner.

agentplot-design ruled on 2026-11-10: "a read stopped is harmless, so the stop ends it; no service manager's stop timeout is raised for it."

## What Changes

- **A halt ends a try as its limit does.** `boomdrive-bounded` gains `Halt`, a value a stop sets once and every holder of it reads. `output` runs its call to a limit and a halt. At whichever comes first it asks the group to end, gives it two seconds, then kills what is left, as it does at the limit today. A call begun once its halt is set runs no process. `Ended` says which stopped the call. A halted call's words are `<call> with <remote> was stopped as the runner stopped`.
- **The runner's stop sets its halt.** `Runner::start` makes one halt for the runner. It gives the halt to the runner's git, by `Git::halted_by`, and to the control plane's implementation, through `planes.rs`'s opener. `Stopper::stop` sets the runner's stop, then its halt. A clone's fetch in flight when the runner stops ends within its grace. Its repository is named `not fetched`, or `not cloned` with nothing left, with the halted words, and the runner stops as it does today.
- **crew's listing takes the halt.** crew's implementation gives the halt to each part and each part's store. A listing in flight when the runner stops ends within its grace, and its partition is unread with the halted words.
- **`boomdrive run --once` stops as `boomdrive run` does.** It takes SIGTERM and SIGINT. Its try in flight is ended. It keeps no further repository and begins no part's pass and no settle. It lets each part end the pass in hand, says `boomdrive: stopped` and exits 1.
- **No service manager's stop timeout changes.** The modules' `AbandonProcessGroup` and `KillMode=process` stay as they are.
- **Record 0042 is amended, and core.58 with it.** Both are written with this change.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `clones`: a fetch in flight when the runner stops is ended as its limit would end it, and one begun after runs nothing. Either is named with the halted words.
- `crew-control-plane`: a listing in flight when the runner stops is ended the same way, and its partition is unread with the halted words.
- `running`: `boomdrive run` ends each try in flight when it is stopped. `boomdrive run --once` takes SIGTERM and SIGINT and stops the same way, exiting 1.

## Impact

- `crates/boomdrive-bounded/src/lib.rs`: `Halt`, `Stopped`, `output` run to a limit and a halt, `Ended.status` none where no process ran, and `halted(call, remote)`. Its own unit tests.
- `crates/boomdrive`:
  - `src/git.rs`: `Git` carries a halt, and `call_remote` runs to it;
  - `src/run.rs`: the runner's halt, made at start and set by the stopper; `run --once` stops on SIGTERM and SIGINT;
  - `src/planes.rs`: the opener takes the runner's halt and gives it to crew's implementation;
  - `tests/common/mod.rs`: `Scratch::silent` gives a `Silent` that says when its connection is taken and when it is closed;
  - `tests/running.rs`: `boomdrive run` and `boomdrive run --once` stopped with a clone's fetch in flight;
  - `tests/planes.rs`, `tests/readme.rs` and `tests/module_files.rs`: their calls of `planes::open` give a halt.
- `crates/boomdrive-crew`:
  - `src/lib.rs`: `open`, `parts` and `Crew` take the halt, and `Crew::store` gives it to the store;
  - `src/read.rs`: `Store` carries a halt, and `hosts` runs to it;
  - `tests/common/mod.rs`: its parts are given a halt;
  - `tests/read.rs`: a listing in flight when the runner stops, and a pull after it.
- `README.md`: the runner's stop, `run --once`'s and crew's listing.
- `docs/adr/0042-…` and `docs/architecture/core.md`: record 0042's amendment of 2026-11-11 and core.58 as amended, written with this change.
- Nothing else changes. No crate, tool, tracked file type, configuration key, field of `report.json`, file of crew's, protocol message or module option changes. `boomdrive-control` is untouched. The hosted implementation makes no git call and takes no halt.

## Touches

```
crates/boomdrive-bounded/src/lib.rs
crates/boomdrive/src/git.rs
crates/boomdrive/src/run.rs
crates/boomdrive/src/planes.rs
crates/boomdrive/tests/common/mod.rs
crates/boomdrive/tests/running.rs
crates/boomdrive/tests/planes.rs
crates/boomdrive/tests/readme.rs
crates/boomdrive/tests/module_files.rs
crates/boomdrive-crew/src/lib.rs
crates/boomdrive-crew/src/read.rs
crates/boomdrive-crew/tests/common/mod.rs
crates/boomdrive-crew/tests/read.rs
README.md
docs/adr/0042-no-try-of-the-runners-call-to-a-remote-outlasts-its-limit.md
docs/architecture/core.md
openspec/specs/clones/spec.md
openspec/specs/crew-control-plane/spec.md
openspec/specs/running/spec.md
```
