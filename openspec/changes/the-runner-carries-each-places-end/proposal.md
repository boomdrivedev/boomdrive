# Proposal

## Why

The brief, 2026-11-08: "The runner carries each place's end, landed, failed, refused or dropped, to its control plane's report before Flume tidies the place, so a control plane reads every piece's end from the runner's report and never from another tool."

The runner reads Flume only between settles. One settle of a place reads its submission landed, makes its work over, tidies it and delivers its ends (Flume's 46). Flume then keeps the place's document only while an end of it is pending for a recipient (62), and removes it otherwise. So a place that lands and is tidied inside one settle shows the runner nothing in between. Its next `flume status <place>` is refused, and the status last carried stands, not listed.

On 2026-11-08 a host's runner carried a fix's place live, its submission under test, and one settle then landed and tidied it. The runner's report also holds sluice's entry, landed, but crew reads the status. So crew read the fix as submitted, and its bolt waited until the planner dropped the fix by hand. The same day a bolt's unit landed and was tidied with no status of its place ever carried, and crew refused to close the bolt. crew keeps a stopgap until this is mended, three readings of GitHub's landing commits (crew record 0062), against its record 0056, under which crew reads only the runner's report. The runner's own After reads a landing only from Flume's status, so an offer after such a place stays held.

The user answered agentplot-design's card on 2026-11-08: "Runner as one more recipient now; one shared inbox in weir next." agentplot-design carries it: "the runner passes --recipient <its own name> beside the role's, and collects with flume inbox <place> --recipient <its own name> only after its report is saved." Flume `v0.20.0` (`9ab0e2c`), carrying the bolt `a-tidied-places-end-stays-readable` (`72107a2`) and Flume record 0060, holds a place's ends, a drop's and a done's among them, for a recipient no pane holds until it collects them, and collects one place's by `flume inbox <place>`. Record 0030 holds the decision.

## What Changes

- **Record 0030 and core.54**, with core.1's, core.13's and core.15's details amended.
- **The runner is one more recipient.** Each `flume launch` of a launch that names recipients names `--recipient boomdrive` after them. A launch that names none is passed with none, so Flume's default is its recipient.
- **The held record is the place's status.** Where Flume holds a place's record for the runner after its tidy, `<location>@<at>`, the runner carries it as it carries any place's status, under the launch's lease number. A control plane reads the place's end from the runner's report.
- **The runner collects a held record once its report holds it.** In the control pass that carries it, once the report file holds it, the runner runs `flume --json inbox <location>@<at> --recipient boomdrive` for each launch whose carried status is listed and reads the place tidied, its `pending_for` naming `boomdrive`. Nothing of the answer is carried or kept. A refused collection, or one Flume could not run, is said in one line and made again at the next pass. No record is collected for a launch whose lease the control plane refused.
- **A drained repository's clone waits for its place's held record to be collected.**
- **The least Flume is `0.20.0`.** Below it, or where `flume --version` names no version, no offer is launched, and one line says so until Flume's version changes. Acts and statuses go on.
- **The set's Flume** moves to `v0.20.0`, with its lock, its git crates' hashes and README's override example.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `work-into-flume`: the runner is one more recipient of each launch that names recipients; a place's end is held for the runner and collected once its report holds it; what waits on a place reads its landing from the record held; no launch is made with a Flume older than the least.
- `clones`: a place whose carried record is pending for the runner has not ended.
- `family-packages`: the set's Flume takes the runner as a recipient and collects one place for it.

## Impact

- `docs/adr/` gains record 0030. `docs/architecture/core.md` gains core.54 and amends the details of core.1, core.13 and core.15.
- `crates/boomdrive`:
  - `src/flume.rs`: `RECIPIENT`, the runner's `--recipient` after a launch's own, `Flume::collect`, `LEAST` and the reading of Flume's version;
  - `src/carry.rs`: the collection after the report is saved, and the least Flume before a launch;
  - `src/facts.rs`: whom a tidied record is pending for;
  - `src/report.rs`: a place pending for the runner has not ended;
  - `src/run.rs`: the runner keeps what it last said of Flume's version, as it keeps its reach;
  - tests: `tests/fakes/mod.rs` (`FakeFlume` holds and collects a record, `FakeTools` prints a version), `tests/flume.rs`, `tests/offers.rs` and `tests/bolts.rs` (each launch's arguments), `tests/pass.rs` (its `Carrier`), `tests/statuses.rs`, `tests/after.rs`, `tests/leases.rs`, `tests/once.rs` (its `Watching` Flume), `tests/dropped.rs` and `tests/family.rs`.
- `flake.nix`, `flake.lock`, `nix/packages/flume-git-crates.nix`: the set's Flume.
- `README.md`: the runner as one more recipient, the held record and its collection, and the least Flume.
- No crate, no tool, no tracked file type, no configuration key, no field of the report and no message of the protocol is added. crew's and the hosted implementation are unchanged: each already carries the status as the runner gives it, a held record among them.
- Outside this repository: crew's unit `crew-reads-each-pieces-end-from-the-runners-report`, which follows this change.

## Touches

```
docs/adr/0030-the-runner-is-one-more-recipient-of-each-place-it-launches-so-flume-holds-its-end-until-the-runner-has-carried-and-collected-it.md
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
