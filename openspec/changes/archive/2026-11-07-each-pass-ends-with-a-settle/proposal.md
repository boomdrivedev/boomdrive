# Proposal

## Why

A place waiting on Flume's merge queue moves only at a settle. Flume settles a place when its agent goes quiet, and every place when a caller runs a command of Flume's that acts or keeps watch over the host (Flume's 46, 119, 120). Until Flume v0.17.0, herdr's plugin kept that watch, settling every place on a bound. On 2026-11-06 the user ruled it out, reading Flume's design for every place waiting on the merge queue: "I do not like relying on the Herder plugin to make this work. I'd rather it be the Boomdrive process." Then, of a herdr server's nudge: "I thought we agreed that only the runner was doing the settling."

Flume v0.17.0 makes the watch a caller's. `flume settle --every <bound>` settles every place and records when it began and its bound. A caller keeps watch while its newest such settle began less than twice its bound ago. While one does, Flume runs unattended, and every waiting place moves within the bound, a submitted place with no agent in it among them. While none does, Flume runs attended, settling every place at each command that acts (Flume's 208, 119; Flume record 0051). Flume's statement names the runner as the caller that keeps it on a host that keeps one (section 9), and Flume's design names it as Boomdrive's follow-up B1. The planner's signal of 2026-11-06 puts it first: "The user wants agentplot's first bolt to be Boomdrive's work."

Today the runner runs no Flume command but those a control plane asks for and the reads of a control pass. Its core.13 and record 0003 left any settle loop to the host's service manager. So since Flume v0.17.0, a host that keeps the runner runs attended: its submitted places move only when someone acts.

This change has every pass of the runner end with a settle of every place, so while the runner runs, every Flume place on its host is settled within the runner's own interval, and a host whose runner has stopped runs attended. It builds only the watch. The other half of B1, a bolt as a launch of a base place, is the sibling change `a-bolt-is-a-launch`.

## What Changes

- **Record 0029, core.13 and core.1**, in this change's commit beside its design. Each turn of the runner's passes ends with one `flume settle --every <bound>`, run once and awaited: the one tool housekeeping the runner runs of its own accord. It decides nothing, goes to no control plane and is kept nowhere. Record 0003 is amended to read with it.
- **Each turn of passes ends with a settle.** After the passes due, keeping, control or both, and before it waits, `boomdrive run` runs `flume --json settle --every <bound>` once. A keeping pass a control pass follows at once shares its settle. `run --once` settles once, after its passes. A turn whose configuration did not read settles all the same. A runner told to stop begins none.
- **The bound is the runner's interval**, in whole seconds: `fetch_interval`, or the shorter of it and `poll_interval` while the runner makes control passes.
- **What the runner says.** One line when a settle is refused or gives no answer, with Flume's words or why, and one when Flume settles again. Nothing while it settles. A refused settle stops nothing, and is no failure of `run --once`.
- **The runner's programs are the runner's**, not only a control plane's, so a runner with no control plane settles too. A test hands its stubs to either.
- **The set's Flume is v0.17.1**, the first release of the set that takes `settle --every`. The chore `the-sets-flume-and-sluice-are-their-latest-releases` moves it, with sluice to v0.74.3, at `9946cc2`; this change takes that commit where its branch does not yet hold it.
- **No test runs a real Flume.** Every run of the runner in the tests, by its binary or in-process, runs a stub `flume` that answers the settle, or the set's Flume on a scratch host.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `running`: each turn of passes ends with a settle of every place, on the runner's interval, said once when refused and once when it settles again; `run --once` settles after its passes, and a refused settle is no failure.
- `configuration`: with no control plane, `run --once` runs the settle and no other `flume` command.
- `family-packages`: the set's Flume keeps the runner's watch on a scratch host.

## Impact

- `docs/adr/` gains record 0029 and amends record 0003. `docs/architecture/core.md` amends core.13's rule and details and core.1's details.
- `crates/boomdrive`:
  - `src/flume.rs`: `Cli::settle_every`;
  - `src/settle.rs`, new: the bound, its argument, and what the runner says of its settles;
  - `src/lib.rs`: the module;
  - `src/run.rs`: the runner's programs and settles; the settle at the end of each turn of `keep` and after `once`'s passes; `Runner::tools`;
  - tests: `tests/common/mod.rs`'s stub `flume` first on the binary's `PATH`, `tests/fakes/mod.rs`'s stub answering the settle, the fixture `tests/fixtures/flume-0.17.1/settled.json` and its `README.md`, `tests/flume.rs`, `tests/control.rs`, `tests/loop.rs`, `tests/family.rs`.
- `flake.nix`, `flake.lock`, `nix/packages/flume-git-crates.nix` and `README.md`'s override examples, from the chore's commit, where this branch lacks it.
- `README.md`: each turn of passes ends with a settle, and what it says.
- No crate, no tool, no tracked file type, no configuration key and no message of the protocol is added.
- Outside this repository: Flume's statement, section 9's bullet on the runner and its row, says each pass, keeping or control, runs the settle once; under record 0029 two passes run back to back share one. Told to the orchestrator, for the statement to say each turn of passes.

## Touches

```
docs/adr/0029-each-pass-of-the-runner-ends-with-a-settle-of-every-place-which-keeps-flumes-watch-over-the-host.md
docs/adr/0003-the-runner-starts-no-tools-process-shares-the-hosts-one-clone-and-asks-nothing-of-boom.md
docs/architecture/core.md
flake.nix
flake.lock
nix/packages/flume-git-crates.nix
crates/boomdrive/src/flume.rs
crates/boomdrive/src/settle.rs
crates/boomdrive/src/lib.rs
crates/boomdrive/src/run.rs
crates/boomdrive/tests/common/mod.rs
crates/boomdrive/tests/fakes/mod.rs
crates/boomdrive/tests/fixtures/flume-0.17.1/settled.json
crates/boomdrive/tests/fixtures/flume-0.17.1/README.md
crates/boomdrive/tests/flume.rs
crates/boomdrive/tests/control.rs
crates/boomdrive/tests/loop.rs
crates/boomdrive/tests/family.rs
README.md
```
