# Proposal

## Why

The brief: "A stop of the runner ends every call to a remote it has in flight, so the runner stops within a few seconds under any service manager and leaves no git running after it." Its sources are record 0042 as amended on 2026-11-12 (`33c87f6d`, core.58), under which the runner's stop is carried to a call through `weir-bounded`'s halt, and boomdrivedev/weir record 0008 as amended on 2026-11-12 (`41cd694`, weir's core.53). It is built once weir's change `a-bounded-call-ends-at-its-callers-halt` has landed on weir's main, with boomdrive pinning weir at the revision that carries the halt. That change landed on weir's main at `5e3f128`.

Today every call of the runner's to a remote runs through `weir-bounded` to its limit, thirty seconds, in a process group of its own: a clone's fetch, crew's listing of the runner branches, and weir-branch's fetch, listing and push of a state repository. The runner reads its stop only between repositories and between passes, so a call in flight runs to its end or its limit before the runner stops, up to thirty seconds and two of grace. launchd kills a job twenty seconds after it asks the job to stop. The darwin module ends the runner alone (`AbandonProcessGroup`), as the NixOS module does (`KillMode=process`), so a runner killed with a call in flight leaves that call's git running in its own group, and nothing then ends it. `boomdrive run --once` takes no signal of its own, so a Ctrl-C ends it at once and leaves its call's git running the same way.

Record 0042's amendment of 2026-11-12 decides the cure: the runner makes one halt, `weir-bounded`'s, which its stop sets after its stop flag; its git and crew's implementation take it, crew's for its listing and for every read and write through weir-branch; every try ends at the first of its limit and the halt, by `weir-bounded`'s one ending; a try begun once the halt is set runs no process; and `boomdrive run --once` stops on SIGTERM and SIGINT as `boomdrive run` does, exiting 1. No service manager's stop timeout changes, on agentplot-design's ruling of 2026-11-10: "a read stopped is harmless, so the stop ends it; no service manager's stop timeout is raised for it."

## What Changes

- **weir at `5e3f128`.** The runner takes weir-signals, weir-capture, weir-branch and weir-bounded at weir's main `5e3f128`, the first revision on it whose `weir-bounded` holds `Halt` and whose weir-branch takes its caller's halt by `Branch::with_halt`. `nix/packages/git-crates.nix` names the four at that revision with fetchgit's hash there.
- **The runner's git takes the runner's halt.** `Git` carries a halt, given by `Git::halted_by`, and its call to a remote runs to its limit and that halt. A fetch halted in flight, or begun once the halt is set, fails with `weir-bounded`'s halted words, `git fetch with origin was stopped as its caller asked`, after git's last line where git wrote one. Its repository is named not fetched, or not cloned with nothing left, as any failed fetch.
- **The runner makes one halt and its stop sets it.** `Runner::start` makes the halt and gives it to its git. `Stopper::stop` sets the stop flag, then the halt, then wakes the loop and every part's bell.
- **crew's implementation takes the runner's halt.** `planes.rs`'s opener hands the halt to crew's `open`, which gives it to each part's `Crew`, which gives it to each partition's `Store`. The store's listing runs to its limit and the halt, and every branch the store gives weir-branch carries the halt, so its read of `<label>/main` and of each runner's branch, and its write of its own, each end at the halt too. A halted listing or read leaves its partition unread, and a halted write its report unsent, each with the halted words. The hosted implementation makes no git call and is given no halt.
- **`boomdrive run --once` stops as `boomdrive run` does.** It takes SIGTERM and SIGINT. Its call in flight is halted, it keeps no further repository, begins no part's pass and no settle, lets each part end the pass in hand, says `boomdrive: stopped` and exits 1.
- **No service manager's stop timeout changes.** The modules' `AbandonProcessGroup` and `KillMode=process` stay as they are.
- **core.58 names the revision.** Its rule names weir at `5e3f128` or later, the first that holds the halt, and its details rule out a pin whose `weir-bounded` has no halt.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `clones`: a fetch in flight when the runner stops ends as its limit would end it, and one begun after runs no git; either is named with the halted words.
- `crew-control-plane`: crew's listing, and weir-branch's read and write of a state repository, end at the runner's stop the same way; a halted listing or read leaves its partition unread, and a halted write its report unsent.
- `running`: `boomdrive run` ends each call to a remote in flight when it is stopped; `boomdrive run --once` takes SIGTERM and SIGINT and stops the same way, exiting 1.

## Impact

- `Cargo.toml`, `Cargo.lock` and `nix/packages/git-crates.nix`: weir's four crates at `5e3f128`.
- `crates/boomdrive`:
  - `src/git.rs`: `Git` carries a halt, and a call to a remote runs to its limit and its halt;
  - `src/run.rs`: the runner's halt, made at start, given to its git and its control plane, and set by its stopper; `run --once` stops on SIGTERM and SIGINT;
  - `src/planes.rs`: the opener takes the runner's halt and gives it to crew's implementation;
  - `tests/common/mod.rs`: `Scratch::silent` gives a `Silent` that says when its connection is taken and when it is closed;
  - `tests/running.rs`: `boomdrive run` and `boomdrive run --once` stopped with a clone's fetch in flight;
  - `tests/control.rs`: `boomdrive run` stopped with a part's read of its state repository in flight;
  - `tests/planes.rs`, `tests/readme.rs` and `tests/module_files.rs`: their calls of `planes::open` give a halt.
- `crates/boomdrive-crew`:
  - `src/lib.rs`: `open`, `parts` and `Crew` take the halt, and `Crew::store` gives it to the store;
  - `src/read.rs`: `Store` carries a halt, its listing runs to it, and each branch it gives carries it;
  - `tests/common/mod.rs`: its parts are given a halt;
  - `tests/read.rs`: a listing, a read and a write halted in flight, a store's halt, and a pull after the runner's stop.
- `README.md`: the runner's stop, `run --once`'s, and crew's calls to its state repository at the stop.
- `docs/architecture/core.md`: core.58 names weir at `5e3f128`.
- Nothing else changes. No crate, tool, tracked file type, configuration key, field of `report.json`, file of crew's, protocol message or module option is added or changed. `boomdrive-control` and `boomdrive-hosted` are untouched. Record 0042 is not amended: its amendment of 2026-11-12 decides what this change builds.

## Touches

```
Cargo.toml
Cargo.lock
nix/packages/git-crates.nix
crates/boomdrive/src/git.rs
crates/boomdrive/src/run.rs
crates/boomdrive/src/planes.rs
crates/boomdrive/tests/common/mod.rs
crates/boomdrive/tests/running.rs
crates/boomdrive/tests/control.rs
crates/boomdrive/tests/planes.rs
crates/boomdrive/tests/readme.rs
crates/boomdrive/tests/module_files.rs
crates/boomdrive-crew/src/lib.rs
crates/boomdrive-crew/src/read.rs
crates/boomdrive-crew/tests/common/mod.rs
crates/boomdrive-crew/tests/read.rs
README.md
docs/architecture/core.md
openspec/specs/clones/spec.md
openspec/specs/crew-control-plane/spec.md
openspec/specs/running/spec.md
```
