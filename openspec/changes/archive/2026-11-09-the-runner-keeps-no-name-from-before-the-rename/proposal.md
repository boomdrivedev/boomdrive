# Proposal

## Why

The brief, 2026-11-09: "The runner and the set name nothing flywheel: the data directory is boomdrive's alone, a clone's records and a launch's place are read only under boomdrive's names, and no path, ref, key, mark, constant, test or README section reads or says the old name, a host's own data moved by hand before it runs the release."

Record 0016 renamed the runner and kept a host working through the rename with code. The runner takes `$XDG_DATA_HOME/flywheel` as its data directory where no `boomdrive` one is there. It carries a clone's `flywheel.address` and `refs/flywheel/view` over to `boomdrive` the first time a pass finds them. And it finds a launch's place by `flywheel:<id>` where `boomdrive:<id>` finds none. So a host that never moves its folder keeps the old name for good, and the code that reads it never goes.

The user, 2026-11-09, reading a host's draft configuration that named `~/.local/share/flywheel` for every repository: "I don't know why this says flywheel. We've spent all this time moving to Boom Drive. When we cut over everything should be Boom Drive"; and on 2026-11-08: "I want the system to be built the way it should be and then have agents fucking fix the data transition as necessary". Record 0035, accepted that day, drops the readings and has each host move its data by hand, with the runner stopped and no place live, before it runs the release. Its rules are already written: core.36 as amended, and core.10's details.

## What Changes

- **The data directory is Boomdrive's alone.** It is `$XDG_DATA_HOME/boomdrive`, else `~/.local/share/boomdrive`, whatever else is beside it. The runner reads no other folder as it.
- **A clone is the runner's only by its own records.** The runner reads `boomdrive.address`, `boomdrive.view` and `refs/boomdrive/view` alone. The carry-over of a clone's records goes. A bare repository that records its address under another section is a folder the runner did not make: left as it is and named, and no clone made over it.
- **A launch's place is found only by `boomdrive:<id>`.** The second mark a launch was found by goes, and with it the constant `BEFORE_RENAME`.
- **Flume 0.4.1's fixtures carry the runner's mark.** Each place's mark becomes `boomdrive:scratch/<n>`. Every other word in them stays Flume's as Flume printed it (record 0035).
- **No test says the old name.** The tests of the readings go. Three tests prove the readings are the runner's own alone, each with a neutral other name: the data directory, a clone's records and a launch's mark.
- **README's section "A host from before the rename" goes.**
- **The release's notes name the move.** Each commit that drops a reading is marked breaking, and its subject names the move each host makes by hand before it runs the release, so git-cliff's notes carry it under "Breaking changes" (core.36, core.48).

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `clones`: the data directory is the runner's own name alone; a clone is the runner's only by its own records; a data directory and a clone's records from before the rename are no longer read; a guarded default branch under a new repository root is any clone's.
- `work-into-flume`: the runner finds a launch's place only by its own mark; a launch marked before the rename is no longer found by that mark.

## Impact

- `crates/boomdrive`:
  - `src/lib.rs`: `BEFORE_RENAME` goes.
  - `src/config.rs`: `data_dir` reads no folder on disk; its unit test of the folder from before the rename becomes one of the runner's own folder alone.
  - `src/clone.rs`: `carry_over`, its call in `find` and `SECTION` go.
  - `src/carry.rs`: `marks` goes; an offer's place is found by `mark` alone.
  - `tests/fixtures/flume-0.4.1/`: each mark, in the JSON and in the README, is the runner's.
  - `tests/acts.rs`, `tests/facts.rs`, `tests/once.rs`, `tests/statuses.rs`: the marks they read are the runner's; `once.rs` gains `a_mark_the_runner_does_not_give`.
  - `tests/views.rs`: `a_clone_made_before_the_rename` goes, and `a_new_root_while_sluice_keeps_the_default_branch` keeps a clone of the runner's own.
  - `tests/clones.rs`: gains `a_clone_recording_its_address_under_another_name`.
- `README.md`: the section "A host from before the rename" goes.
- `openspec/specs/clones` and `openspec/specs/work-into-flume`, through this change's deltas.
- No record or rule is written: record 0035, core.36 and core.10's details already hold the decision. No crate, tool, tracked file type, task, configuration key, report field or protocol message is added. `docs/adr/`, `docs/architecture/` and `openspec/changes/archive/` keep the old name where they record what was.
- Each host moves its own data by hand at its cutover, as record 0035 says. That chore is ops', on each host, and no task of this change.
