# Proposal

## Why

crew drops a launch by writing a drop request naming it, `Act: drop` with `Of` its number. It writes that drop in the commit that takes the launch's work out of its plan. crew's own reading is that a launch a drop names no longer stands: crew's `plugin/lib/runner.py`, at `130ce42b`, its `standing` and `launch_of`. crew's implementation here does not read drops at all.

- **A dropped launch is still offered.** A launch the runner holds `held` is still given at every pull, and so is one only this host's claim answers. The runner launches it once its stream has room or its After lands. A kit's launch dropped before any runner answers it is claimed and offered like any other.
- **The drop reaches no runner.** crew's implementation gives a request whose work has left the plan only where the runner already answers it. A drop is written as its work leaves the plan, so it is never given, and a place launched before the drop arrived goes on working.

On 2026-11-07 a partition's planner dropped launch 398 by request 425, at 20:34:58Z. A host's runner launched 398 at 20:50:03Z and never answered 425, and 398's place was still working hours later. The runner was a release from before core.50. It took that launch from a pull made at 19:40, and its pass ran 70 minutes. So the pull that offered 398 came before the drop. The drop was then given at no later pull.

The brief: "crew's implementation of the control plane gives no offer for a launch that a drop in requests.rec names, and lets go one the runner holds unlaunched as the interface's let-go has it, so no runner claims or launches a dropped launch; the runner's own rules are unchanged".

## What Changes

- **A launch a drop names is offered only where the runner launched it.** Wherever the runner's report or its branch does not answer it `launched`, crew's implementation gives no offer for it and claims it not, whatever host it names. A claim of it standing on this host's branch leaves at the branch's next write. A launch the runner did launch is still given, so its place is followed to its end (record 0021).
- **A dropped launch the runner holds unlaunched is let go.** At each report, a launch a drop names whose answer is not `launched` leaves the runner's files and is named in the report's let-go, whatever the plan holds.
- **A drop is given to the runner that holds its launch, whatever the plan holds.** The runner then answers it with its own rules: `flume drop` on a launched place, or `done` from its report for a launch it holds unlaunched.
- **The plan filter is written down.** A request whose work has left crew's plan is given only while the runner holds it. That is built and unwritten until now, and the spec gains it with the drop's exception.
- **Record 0002** gains an amendment of 2026-11-07 that carries these decisions. No rule changes: core.1's details already allow what an implementation gives because it read the control plane's own rule from its own records. The runner's rules and code are unchanged.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `crew-control-plane`: a launch a drop names is offered only where the runner launched it, and let go where the runner holds it unlaunched. A drop is given to the runner that holds its launch whatever the plan holds. A request whose work has left the plan is given only while the runner holds it.

## Impact

- `crates/boomdrive-crew`:
  - `src/read.rs`: `Main` knows the launches a drop names;
  - `src/lib.rs`: `pull_partition` gives no offer and makes no claim for a dropped launch the runner has not launched, and gives a drop whose launch the runner holds; `report` keeps no claim of a dropped launch;
  - `src/write.rs`: `let_go` lets go a dropped launch the runner holds unlaunched;
  - `tests/drops.rs`: the spec's scenarios.
- `README.md`: crew's implementation offers no dropped launch.
- `docs/adr/0002-…`: an amendment of 2026-11-07.
- Nothing else changes: no rule, no crate, no tool, no tracked file type, no message of the protocol, no code of the runner's or of the hosted implementation.

## Touches

```
crates/boomdrive-crew/src/read.rs
crates/boomdrive-crew/src/lib.rs
crates/boomdrive-crew/src/write.rs
crates/boomdrive-crew/tests/drops.rs
README.md
docs/adr/0002-the-runner-takes-leased-work-while-its-flume-stream-has-room-and-carries-each-tools-record-back-as-given.md
```
