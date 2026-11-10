# Proposal

## Why

The brief: "The runner's report keeps a preview only while its control plane still asks for it: once its done is answered, or the work it previewed has landed or been dropped, the preview is let go and leaves the runner's branch."

crew asks the runner for a card's preview, and puts it away with a done, as crew's record 0056 has it. Its consequence on previews says a preview "goes when they are done or the row ends". crew's implementation lets go of a preview only once its work has left crew's plan and the preview is no longer up. Two things keep a preview on the runner's branch after crew has stopped asking for it:

- **A bolt's work never leaves the plan.** crew asks for a landings row's preview as the work `bolt/<b>`. `Main::plans` reads no `bolt/<b>` as leaving, so a bolt's previews and their dones stay on the runner's branch for good. crew's design of `a-cards-preview-is-asked-of-the-runner` names this as a follow-up for this repository.
- **A preview put away stays while its work is in the plan.** Its done stays too, though crew no longer asks for either. crew reads its ask of a host, repository and revision as standing while its newest preview request of them is a bring-up. A done ends the ask.

Letting a preview go while its work is in the plan is not enough alone. crew's implementation gives every request whose work is in the plan, so a preview let go would be given again and brought up again. The pull must stop giving it too.

## What Changes

- **A done answered is let go with what it put away.** At each report, crew's implementation names in the let-go a done the runner's report holds, whatever the plan holds. With it go the bring-ups that the report holds of the same `Host`, `Repo` and `Revision`, each written before the done. Each leaves `previews.rec`.
- **A preview request is given while crew still asks for it, or while the runner holds it.** A request the runner does not hold is given only while it is crew's newest preview request of its `Host`, `Repo` and `Revision`. A bring-up is then given only while its work is in the plan. A done is given whatever the plan holds, but only while the runner holds a bring-up of those three. So a preview and its done, once let go, are given no more. A bring-up crew put away before the runner answered it is never brought up.
- **A bolt's work leaves the plan with its `Bolt` record.** `bolt/<b>` has left the plan once `plan.rec` holds no `Bolt: <b>`, as `fix/<b>/<name>` has. A bolt's refused preview leaves once the bolt lands or is dropped, and so does one that sluice lists no more. On crew's `main`, `bolt/<b>` is also the work of the bolt's base place. So its launch and its acts now leave once its place is over and crew has closed the bolt, as a unit's do.
- **A preview still up stays until its done.** This holds whatever the plan holds, as it does today. crew writes the done when the row ends, and the done is given whatever the plan holds, so it puts the preview away.
- **The runner's half is proved.** The runner already drops a preview its control plane lets go and runs nothing against it. A scenario of the control-plane interface now proves it.
- **Record 0031 gains an amendment**, proposed, recording these choices. It is written with this change. No rule changes: core.55 stands as it is.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `crew-control-plane`: which preview requests are given, and when a preview is let go. A done answered goes with the bring-ups it put away. `bolt/<b>` leaves the plan with its `Bolt` record, for a bolt's base place as for its previews.
- `control-plane-interface`: a report's answer letting go of a preview drops it from the runner's report, and nothing is run against it.

## Impact

- `docs/adr/0031-…`: an amendment, proposed, written with this change.
- `crates/boomdrive-crew`:
  - `src/read.rs`: `Main::plans` reads `bolt/<b>`, and gives a preview request's key;
  - `src/lib.rs`: `Pulling::current` gives a preview request while crew still asks for it, or while the runner holds it;
  - `src/write.rs`: `let_go` lets a done go with the bring-ups it put away;
  - `tests/read.rs` and `tests/letgo.rs`.
- `crates/boomdrive`:
  - `tests/previews.rs`: a preview let go;
  - `tests/readme.rs`: the README says when a preview leaves the runner's branch.
- `README.md`: crew's implementation section says when a preview leaves the runner's branch.
- Nothing else changes. No crate, tool, tracked file type, protocol version or file, field of `report.json`, file of crew's or descriptor changes. The runner's own code and the hosted implementation are untouched.
- Outside this repository: this change relies on how crew's unit `a-cards-preview-is-asked-of-the-runner` writes a done. It writes the done with its ask's own `Host`, `Work`, `Repo` and `Revision`, and only for an ask that stands (`plan.py` `preview_written`). crew's design names this change's `bolt/<b>` reading as its follow-up. Nothing is asked of crew.

## Touches

```
docs/adr/0031-the-runner-brings-up-the-preview-its-control-plane-asks-for-through-sluice-and-puts-away-only-what-it-asked-for.md
crates/boomdrive-crew/src/read.rs
crates/boomdrive-crew/src/lib.rs
crates/boomdrive-crew/src/write.rs
crates/boomdrive-crew/tests/read.rs
crates/boomdrive-crew/tests/letgo.rs
crates/boomdrive/tests/previews.rs
crates/boomdrive/tests/readme.rs
README.md
openspec/specs/crew-control-plane/spec.md
openspec/specs/control-plane-interface/spec.md
```
