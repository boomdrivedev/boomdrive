# Proposal

## Why

crew brings up a card's preview itself today. `crew card preview` runs `sluice preview` on the work's host, reads it back with `sluice previews`, and puts it away with `sluice release`. That is crew calling sluice. On 2026-11-08 the user ruled that crew is the control plane and talks only to Boomdrive's runner: "crew doesn't know about Flume directly except for whatever the control plane contract allows", and "Everything has to happen from the Boom Drive runner." crew's record 0056, as rewritten that day, holds their words. Its consequence on previews, agentplot-design's choice on point 1, moves previews onto the runner:

- the control plane asks for a preview of a branch at a revision by one message, `preview`, and for its end by that message again, marked `done`;
- the runner on the work's host, which keeps the repository, runs sluice's preview command for it once, and hands back what sluice answered;
- crew learns a preview's addresses and state from the runner's report alone;
- neither crew nor the runner puts away a preview it did not ask for.

That record has one Boomdrive unit come first, then a crew unit, and leaves `preview` in the hosted protocol to a later version.

The brief: "A runner that keeps a repository brings up the preview its control plane asks for, of a branch at a revision, puts it away when asked, and reports its addresses and state, or its refusal, as the tool that keeps previews gave them."

## What Changes

- **A preview is a want of its own.** The interface gains `Want::Preview`. It names a repository, a branch and a revision, and whether it is done, under a lease of its own. The report gains a part, `previews`, holding what the runner answered of each, by id.
- **The runner brings a preview up through sluice.** It runs `sluice preview <branch> <revision> --json` once, in the clone's container of a repository it keeps, after every launch its pull offers. It carries sluice's answer, or its refusal in sluice's words.
- **The runner waits on sluice's bound no longer than its patience.** sluice's ask waits while the host is at its bound. After 20 seconds the runner interrupts it, which sluice takes as ending its wait and nothing else. The preview is held, and asked for again at the next pass. So no launch and no other drive's work waits on sluice's bound behind it (core.49, core.50).
- **The runner carries what sluice lists of each preview** at each pass, as `sluice previews --json` lists it, until sluice no longer lists it. So the control plane sees it come up or fail.
- **What is carried leaves out the command's output.** Of sluice's answer and its listing, the runner carries a preview's `name`, `branch`, `revision`, `state`, `made` and `addresses`, each as sluice printed it. It leaves out the output a failed preview's command printed, which is a log, and the rest (core.11).
- **A done puts away only what the runner asked sluice for.** On a preview marked done, the runner runs `sluice release <name>` once. It does so only where its report holds a preview of that repository and revision that it asked sluice for. Otherwise it refuses the done and runs nothing. A person's own preview is never put away.
- **crew's implementation reads `Act: preview`.** A preview request names its `Host`, `Repo`, `Branch` and `Revision`, and `Done: yes` to put the preview away. Its answer is written to a new file of the runner's branch, `previews.rec`, under a descriptor this change proposes for crew to hold, and read back from it. A preview request naming no `Host` is given nothing.
- **Version 1 of the hosted protocol carries no preview.** It may not change (core.42). A preview fails its check, and the report's `previews` never goes up in it. A later version that names `preview` is the runner's and the cloud's together.
- **Rule core.55** says how the runner brings up, carries and puts away a preview. **Record 0031**, proposed, records the decision. Both are written with this change.

## Capabilities

### New Capabilities

- `previews`: how the runner brings up the preview its control plane asks for through sluice, carries sluice's word of it, and puts away only what it asked sluice for.

### Modified Capabilities

- `control-plane-interface`: a pull may give a preview, under a lease of its own, and the report holds what the runner answered of each.
- `crew-control-plane`: a `preview` request is a preview of what it names, given only to the host it names. Its answer is written in `previews.rec` and read back from it, and let go once its work has left crew's plan and it holds no preview up.
- `runner-protocol`: version 1 carries the interface's shapes but a preview, which its check refuses, and its report carries no previews.

## Impact

- `docs/adr/0031-…`: the record, proposed. `docs/architecture/core.md`: core.55 and its details, and core.13's details leaving the ask core.55 interrupts. These are written with this change.
- `crates/boomdrive-control`:
  - `src/want.rs`: `Want::Preview` and `Preview`;
  - `src/report.rs`: `Report::previews` and `Previewed`, and `Report::leases` naming each preview's lease;
  - `src/lib.rs`: the new types re-exported;
  - `tests/json.rs`.
- `crates/boomdrive`:
  - `Cargo.toml`: `libc`, a workspace dependency the crate already uses in its tests, taken as a dependency too, for the interrupt;
  - `src/sluice.rs`: sluice's preview, previews and release, the patience and its interrupt, and what is carried of a preview;
  - `src/carry.rs`: a preview answered, brought up, held, carried and put away, and the second round's preview commands;
  - `tests/fakes/mod.rs`: the stub `sluice` waits where a test asks, and the bench sets the patience;
  - `tests/previews.rs`, new, for the previews spec;
  - `tests/sluice.rs`;
  - `tests/fixtures/sluice-0.74.3/`, new: the set's sluice's own answers to a preview, its listing and its release, with a README.
- `crates/boomdrive-crew`:
  - `src/read.rs`: a `preview` request read, and `previews.rec` read with the runner's files;
  - `src/lib.rs`: a preview naming no host given nothing, and a preview read back into the report;
  - `src/write.rs`: `previews.rec`, its descriptor, and a preview's let-go;
  - `tests/read.rs`, `tests/write.rs`, `tests/letgo.rs` and `tests/common/mod.rs`.
- `crates/boomdrive-hosted`:
  - `src/plane.rs`: a preview is given its partition's prefix, withheld where its repository is not served, and its report part is left out of version 1's;
  - `tests/interface.rs`: a preview is no want of version 1.
- `README.md`: the control pass brings up and puts away previews.
- Nothing else changes: no crate, no tool, no tracked file type, no protocol version or file, no field of `report.json` but the new part, and no command of Flume's.
- Outside this repository:
  - crew's unit that moves its previews onto the runner posts `Act: preview` and reads `previews.rec`, holding its descriptor byte for byte;
  - boomdrive-cloud and this repository make the version that names `preview`;
  - sluice's statement names Boomdrive as a caller of its previews;
  - sluice may give an ask that never waits at the bound.

  Each is named in design.md, for the orchestrator to carry.
- **Beside this change.** The change `land-is-the-message-for-a-works-landing`, landed on its bolt and not yet on `main`, modifies the runner-protocol requirement this change modifies. Whichever reaches `main` second takes the other's words in (design Decision 12).

## Touches

```
docs/adr/0031-the-runner-brings-up-the-preview-its-control-plane-asks-for-through-sluice-and-puts-away-only-what-it-asked-for.md
docs/architecture/core.md
crates/boomdrive-control/src/want.rs
crates/boomdrive-control/src/report.rs
crates/boomdrive-control/src/lib.rs
crates/boomdrive-control/tests/json.rs
crates/boomdrive/Cargo.toml
Cargo.lock
crates/boomdrive/src/sluice.rs
crates/boomdrive/src/carry.rs
crates/boomdrive/tests/fakes/mod.rs
crates/boomdrive/tests/previews.rs
crates/boomdrive/tests/sluice.rs
crates/boomdrive/tests/fixtures/sluice-0.74.3/README.md
crates/boomdrive/tests/fixtures/sluice-0.74.3/preview-starting.json
crates/boomdrive/tests/fixtures/sluice-0.74.3/preview-no-table.json
crates/boomdrive/tests/fixtures/sluice-0.74.3/previews.json
crates/boomdrive/tests/fixtures/sluice-0.74.3/release.json
crates/boomdrive/tests/fixtures/sluice-0.74.3/release-nothing.json
crates/boomdrive-crew/src/read.rs
crates/boomdrive-crew/src/lib.rs
crates/boomdrive-crew/src/write.rs
crates/boomdrive-crew/tests/read.rs
crates/boomdrive-crew/tests/write.rs
crates/boomdrive-crew/tests/letgo.rs
crates/boomdrive-crew/tests/common/mod.rs
crates/boomdrive-hosted/src/plane.rs
crates/boomdrive-hosted/tests/interface.rs
README.md
openspec/specs/previews/spec.md
openspec/specs/control-plane-interface/spec.md
openspec/specs/crew-control-plane/spec.md
openspec/specs/runner-protocol/spec.md
```
