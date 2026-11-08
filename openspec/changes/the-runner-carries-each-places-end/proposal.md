# Proposal

## Why

The brief, 2026-11-08: "The runner carries each place's end, landed, failed, refused or dropped, to its control plane's report before Flume tidies the place, so a control plane reads every piece's end from the runner's report and never from another tool."

The runner reads Flume only between settles. One settle of a place reads its submission landed, makes its work over, tidies it and delivers its ends (Flume's 46). Flume then keeps the place's document only while an end of it is pending for a recipient, and removes it otherwise (62). So a place that lands and is tidied inside one settle shows the runner nothing in between. Its next `flume status <place>` is refused, and the status last carried stands, not listed.

On 2026-11-08 a host's runner carried a fix's place live, its submission under test, and one settle then landed and tidied it. The runner's report also holds sluice's entry, landed, but crew reads the status. So crew read the fix as submitted, and its bolt waited until the planner dropped the fix by hand. The same day a bolt's unit landed and was tidied with no status of its place ever carried, and crew refused to close the bolt. crew keeps a stopgap until this is mended, three readings of GitHub's landing commits (crew record 0062), against its record 0056, under which crew reads only the runner's report. The runner's own After reads a landing only from Flume's status, so an offer after such a place stays held.

agentplot-design chose, on 2026-11-08, that Flume holds each place's end for the runner until the runner has carried it and collected it: "Make the runner the caller its launches' ends are pending for, collected by flume inbox rather than shown in a pane, and amend that requirement in this change." Record 0030 holds the decision.

This change builds the runner's half. It waits on a Flume unit that holds a place's end for its caller, which agentplot-planner places ahead of it in the bolt.

## What Changes

- **Record 0030 and core.53**, in this change's commit beside its design, with core.1's, core.13's and core.15's details amended.
- **Every launch names the runner its caller.** Each `flume launch` ends `--caller boomdrive`. It is not a recipient, and the launch's recipients are as the control plane gave them.
- **The held record is the place's status.** Where Flume holds a place's record for the runner after its tidy, `<location>@<at>`, the runner carries it as it carries any place's status, under the launch's lease number. A control plane reads every place's end from the runner's report.
- **The runner collects a held record once its report holds it.** In the control pass that carries it, once the report file holds it, the runner runs `flume --json inbox --caller boomdrive <location>@<at>` for each launch whose carried status is listed and names `boomdrive` among those its record is kept for. Nothing of the answer is carried or kept. A refused collection, or one Flume could not run, is said in one line and made again at the next pass. No record is collected for a launch whose lease the control plane refused.
- **A drained repository's clone waits for its place's held record to be collected.**
- **The least Flume.** Below the first Flume release that holds a place's end for its caller, or where `flume --version` names no version, no offer is launched, and one line says so until Flume's version changes. Acts and statuses go on.
- **The set's Flume** moves to that release, with its lock, its git crates' hashes and README's override example.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `work-into-flume`: every launch names the runner its caller; a place's end is held for the runner and collected once its report holds it; what waits on a place reads its landing from the record held; no launch is made with a Flume older than the least.
- `clones`: a place whose carried record is kept for the runner has not ended.
- `family-packages`: the set's Flume takes the runner's caller.

## Impact

- `docs/adr/` gains record 0030. `docs/architecture/core.md` gains core.53 and amends the details of core.1, core.13 and core.15.
- `crates/boomdrive`:
  - `src/flume.rs`: `CALLER`, `--caller` on every launch, `Flume::collect`, `LEAST` and the reading of Flume's version;
  - `src/carry.rs`: the collection after the report is saved, and the least Flume before a launch;
  - `src/facts.rs`: whom a tidied record is kept for;
  - `src/report.rs`: a place kept for the runner has not ended;
  - `src/run.rs`: the runner keeps what it last said of Flume's version, as it keeps its reach;
  - tests: `tests/fakes/mod.rs` (`FakeFlume` holds and collects a record, `FakeTools` prints a version), `tests/flume.rs`, `tests/offers.rs` and `tests/bolts.rs` (each launch's arguments), `tests/pass.rs` (its `Carrier`), `tests/statuses.rs`, `tests/after.rs`, `tests/leases.rs`, `tests/once.rs` (its `Watching` Flume), `tests/dropped.rs` and `tests/family.rs`.
- `flake.nix`, `flake.lock`, `nix/packages/flume-git-crates.nix`: the set's Flume.
- `README.md`: the runner's caller, the held record and its collection, and the least Flume.
- No crate, no tool, no tracked file type, no configuration key, no field of the report and no message of the protocol is added. crew's and the hosted implementation are unchanged: each already carries the status as the runner gives it, a held record among them.
- Outside this repository: the Flume unit this change waits on, and crew's unit `crew-reads-each-pieces-end-from-the-runners-report`, which follows it.

## Touches

```
docs/adr/0030-every-launch-names-the-runner-its-caller-so-flume-holds-each-places-end-until-the-runner-has-carried-and-collected-it.md
docs/architecture/core.md
flake.nix
flake.lock
nix/packages/flume-git-crates.nix
README.md
crates/boomdrive/src/flume.rs
crates/boomdrive/src/carry.rs
crates/boomdrive/src/facts.rs
crates/boomdrive/src/report.rs
crates/boomdrive/src/run.rs
crates/boomdrive/tests/fakes/mod.rs
crates/boomdrive/tests/flume.rs
crates/boomdrive/tests/offers.rs
crates/boomdrive/tests/bolts.rs
crates/boomdrive/tests/pass.rs
crates/boomdrive/tests/statuses.rs
crates/boomdrive/tests/after.rs
crates/boomdrive/tests/leases.rs
crates/boomdrive/tests/once.rs
crates/boomdrive/tests/dropped.rs
crates/boomdrive/tests/family.rs
openspec/changes/the-runner-carries-each-places-end/
```
