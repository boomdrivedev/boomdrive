# Proposal

## Why

sluice numbers the queue of each branch on its own. So one number can be on more than one branch: a unit's submission to a bolt's branch, and the bolt's submission to `main`. The runner asks `sluice status <id> --json` and names no branch. Since sluice 0.110, sluice refuses a number that is on more than one branch, and names the `--branch` to give.

On deneb, on 2026-11-26, the host ran boomdrive 0.29.0 and sluice 0.110.2. sluice refused every ask for four old submissions: boomdrive-cloud 18, boomdrive 27, flume 32 and flume 33. The runner read each refusal, kept nothing, and asked again at the next pass. It asked about every 4 seconds, with no end.

The runner reads an entry until its state is final. It stops only when it carries a final entry. A refused ask carries no entry, so the runner never stops. With `--branch`, sluice answers each of the four.

sluice began to refuse a number on two branches on 2026-11-12, in 0.92.0 (sluice record 0114). The gate did not see it. The runner's tests of its sluice asks answer through a stub that takes any arguments. The one test of the set's sluice asks for a number that no branch holds.

## What Changes

- **The runner names the branch.** Flume's status names the submission's number, `id`, and the branch whose queue holds it, `target`. The runner asks `sluice status <id> --branch <target> --json`. So sluice never has to choose. A status that names no `target` runs no sluice.
- **An entry is its number on its branch.** A final entry stops the asks only for its number on its branch. The same number on another branch is another entry.
- **The gate runs each of the runner's asks against the set's sluice.** A test makes a scratch repository whose `main` and `next` each hold a submission 1, from two counts. Through the runner's own reader, it asks the set's sluice for the entry of 1 on each branch, a preview, the previews and a release. It fails where sluice cannot read the runner's command line, exit 64, or refuses the entry. The ask with no branch fails it.
- **A refusal is kept, and the runner asks no more.** Where sluice refuses an entry, the runner keeps the refusal in its report: the number, the branch and sluice's words. It says the refusal once, and does not ask sluice for that entry again. It asks again only for another entry that Flume's status names. Where sluice could not be run, the runner keeps nothing and asks at the next pass, as before.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `tool-records`: the ask names the branch, an entry is its number on its branch, and a refused entry is asked for no more.
- `family-packages`: the set's sluice takes each of the runner's asks.

## Impact

- `crates/boomdrive`: `sluice.rs` (`Sluice::entry` takes the branch), `facts.rs` (`submission` gives the number and the branch), `carry.rs` (`carry_entries`), and the tests `entries`, `sluice`, `facts` and `family`, with the stub `sluice` that answers by branch. `tests/common`'s `Resident` runs `flume host` or `sluice host` as the test's child. The tests take `ed25519-dalek` and `base64` to make sluice's signing key at run time.
- `crates/boomdrive-control`: `report.rs`, a launch's `entry_refused`. A report file of before reads with none (core.36).
- Hosts take the release through their own configuration. No host changes a file by hand.
