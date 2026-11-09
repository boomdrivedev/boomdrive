# Proposal

## Why

The brief, 2026-11-09: "The runner confirms a place's end to Flume only once its control plane has taken it, so the end stays owed in Flume until then and losing the runner's report loses no end." Its source is weir's record 0006, accepted by the user on their card of 2026-11-08: a message one tool owes a reader "stands owed to a reader until that reader confirms it", and handing it over "confirms nothing by itself".

Flume `v0.21.0` (`77d5b93`) carries that form into Flume (Flume record 0061, accepted by agentplot-design on 2026-11-09). Each end of a place is a delivery, owed to each recipient until the recipient confirms it by its key, `flume confirm <key>… --recipient <name>`. `flume inbox` and `flume wait` hand an end over and confirm it only with `--confirm`. Flume record 0061 says what follows for its callers: "every caller that collects must now confirm. crew's role kinds, the runner and a person's own scripts confirm what they act on, or are handed it again at each inbox."

The runner collects a tidied place's record by `flume inbox <location>@<at> --recipient boomdrive` once its report file holds it, before the report is sent (record 0030, core.54). Under Flume `v0.21.0` that inbox confirms nothing, so each record is handed over again at every pass and never let go. Under Flume `v0.20.0` it let the record go before the control plane held the end, while the report file was the end's one copy. Of everything in the data directory, that file alone is made again from nothing (core.35).

## What Changes

- **Record 0030 is amended on 2026-11-09, and core.54's rule and details with it**, as are core.1's and core.13's details. These are written with this change. Flume's statement's runner row and its S121 are amended on blueprints' branch `design/the-runner-confirms-an-end-once-its-control-plane-has-it`.
- **The interface's answer to a report names the statuses the control plane took.** `Reported` gains `statuses_taken`: each launch whose carried status the control plane took with the report that answer answers. crew's implementation names each launch whose status `places.rec` holds once the runner's branch is written. The hosted implementation names each launch whose status went up as version 1's copy, and none whose copy went up null. No message of the protocol changes.
- **The runner confirms each end owed to it once its control plane has taken it.** After the report is answered and the part's share saved, for each launch the pull gives whose clone stands and whose carried status is listed and named taken, the runner runs `flume --json confirm --recipient=boomdrive -- <key>…`, the form a control plane's confirm runs (core.56). It names the delivery key of each end the status gives pending for `boomdrive`, in the status's order, one call for each launch. A live place's ends are confirmed as they are taken, and a tidied record's alike.
- **The runner runs no `flume inbox`.** `Flume::collect` gives way to `Flume::confirm`.
- **An end not taken stays owed.** That holds for a report unsent, unanswered or refused, a status not named taken, a lease refused, a launch let go, a pass whose save failed after the answer, and a Flume older than the least. A confirmation Flume refuses, or cannot be run for, is said in one line and made again at a later pass whose report is taken.
- **The least Flume is `0.21.0`**, and the set's Flume moves to `v0.21.0`, with its lock, its git crates' hashes, and README's override example one tag on.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `work-into-flume`: each end Flume owes the runner is confirmed by its key once the control plane has taken a report holding it, a live place's among them, and none is handed over by an inbox; the least Flume is `0.21.0`.
- `clones`: a place whose carried record is pending for the runner has not ended until the runner has confirmed its ends.
- `family-packages`: the set's Flume takes `flume confirm` as the runner asks it.
- `control-plane-interface`: a report's answer names the statuses taken.
- `crew-control-plane`: the answer names each status `places.rec` holds.
- `hosted-control-plane`: the answer names each status that went up as its copy, and none that went up null.

## Impact

- `docs/adr/0030-…md` gains its amendment of 2026-11-09. `docs/architecture/core.md` amends core.54 and the details of core.1, core.13 and core.54. Both are written with this change.
- `crates/boomdrive-control`: `src/lib.rs` (`Reported::statuses_taken`), `src/fake.rs` (`Fake` names each status taken, but those a test leaves), `tests/json.rs`, `tests/fake.rs`.
- `crates/boomdrive-crew`: `src/lib.rs` (`report` names each status `places.rec` holds), `src/write.rs` (the launches whose statuses the files hold), `tests/write.rs`.
- `crates/boomdrive-hosted`: `src/plane.rs` (each status that went up, kept with the report sent, named taken in its answer), `tests/plane.rs`.
- `crates/boomdrive`:
  - `src/flume.rs`: `Flume::confirm` and `Cli`'s, in place of `collect`; `LEAST` `0.21.0`;
  - `src/facts.rs`: the delivery keys of the ends an entry gives pending for a name;
  - `src/carry.rs`: `Work::confirm` after the report's answer, in place of `Work::collect` before it;
  - `src/report.rs`: `ended`'s documentation;
  - tests: `tests/fakes/mod.rs` (`FakeFlume`'s ends, their keys and their confirmation), `tests/flume.rs`, `tests/statuses.rs`, `tests/once.rs`, `tests/leases.rs`, `tests/after.rs`, `tests/dropped.rs`, `tests/offers.rs`, `tests/family.rs`.
- `flake.nix`, `flake.lock`, `nix/packages/flume-git-crates.nix`: the set's Flume.
- `README.md`: the runner confirms each end once its control plane has it, and the least Flume.
- No crate, no tool, no tracked file type, no configuration key, no field of the report file and no message of the protocol is added.
- Outside this repository: blueprints' branch `design/the-runner-confirms-an-end-once-its-control-plane-has-it`, which amends Flume's statement's runner row and S121, for the orchestrator to push.

## Touches

```
docs/adr/0030-the-runner-is-one-more-recipient-of-each-place-it-launches-so-flume-holds-its-end-until-the-runner-has-carried-and-collected-it.md
docs/architecture/core.md
flake.nix
flake.lock
nix/packages/flume-git-crates.nix
README.md
crates/boomdrive-control/src/lib.rs
crates/boomdrive-control/src/fake.rs
crates/boomdrive-control/tests/json.rs
crates/boomdrive-control/tests/fake.rs
crates/boomdrive-crew/src/lib.rs
crates/boomdrive-crew/src/write.rs
crates/boomdrive-crew/tests/write.rs
crates/boomdrive-hosted/src/plane.rs
crates/boomdrive-hosted/tests/plane.rs
crates/boomdrive/src/flume.rs
crates/boomdrive/src/facts.rs
crates/boomdrive/src/carry.rs
crates/boomdrive/src/report.rs
crates/boomdrive/tests/fakes/mod.rs
crates/boomdrive/tests/flume.rs
crates/boomdrive/tests/statuses.rs
crates/boomdrive/tests/once.rs
crates/boomdrive/tests/leases.rs
crates/boomdrive/tests/after.rs
crates/boomdrive/tests/dropped.rs
crates/boomdrive/tests/offers.rs
crates/boomdrive/tests/family.rs
openspec/changes/the-runner-confirms-an-end-once-its-control-plane-has-it/
```
