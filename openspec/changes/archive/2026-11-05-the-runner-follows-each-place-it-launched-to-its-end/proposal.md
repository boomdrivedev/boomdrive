# Proposal

## Why

On 2026-11-05 a host's runner launched a unit's place, from a kit request whose `Repo` named the control plane's repository by its old name, `flywheel-cloud`. It carried the place's status once, at construct, waiting on readiness. The place then built, its submission landed on its bolt, and Flume tidied it, keeping its record as `<location>@<at>` for an end pending for the planner. The runner's branch still holds the status of construct. crew reads the unit as waiting at construct, its planner cannot see that it landed, and the bolt cannot go on. Another partition's chore on the same host, in a repository the runner still keeps, was followed to its tidied landing.

No pass ran while the place built: the runner had no service on the host yet, and passes by hand launched the place and carried a retry of it. Once its service started, it followed the place no more, for two reasons:

- **crew's implementation gave no want for the launch.** By then the configuration named the repository by its new name, agentplot/boomdrive-cloud (record 0016). A kit's launch is given only where its `Repo` names a repository the hello names. That test ran before the one for a launch the runner already answered, though the crew-control-plane spec says such a launch is given at every later pull.
- **The pass read a place's status only through a kept clone**, to pass it as `--repository`. A launch whose repository the runner no longer keeps was skipped.

The orchestrator's brief, 2026-11-05: "Fix it here, so that every place the runner launched is followed until Flume reports it over or tidied. Write that state, with its landing, to `places.rec`."

## What Changes

- **Record 0021 and core.5**, in this change's commit beside its design. A place the runner launched is followed in Flume's status to its end, by its location alone, after the configuration drops its repository. No clone or view of that repository is made, fetched or kept for it.
- **The pass follows each place to its end, whatever the runner keeps.** Where the configuration keeps the launch's repository with its clone made, Flume is asked `flume --json status --repository <container> <place>`, as now. Where it does not, Flume is asked `flume --json status <place>`, by the place's location alone.
- **No keeper is told of a landing in a repository the runner does not keep.**
- **crew's implementation gives each launch the runner answered**, at every pull, whatever the hello names. Its repository is the kept address its `Repo` names where there is one, else its `Repo` as written. It is neither claimed again nor named in the log.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `work-into-flume`: a place is followed to its end whether or not the configuration still keeps its repository, asked for by its location alone where it does not; a landing in a repository not kept is told to no keeper.
- `crew-control-plane`: a launch the runner's report or branch answers is given whatever the hello names, and a launch naming no host is given nothing only where the runner has not answered it.

## Impact

- `docs/architecture/core.md` amends core.5's details. `docs/adr/` gains record 0021. Record 0020 is the sibling change `boomdrive-publishes-a-flake-of-the-family-and-the-runners-service`'s, on its branch.
- `crates/boomdrive`:
  - `src/flume.rs`: `Flume::status_of` takes the clone's container where there is one;
  - `src/carry.rs`: `carry_statuses` follows a place whose repository is not kept, and tells no keeper of its landing;
  - tests: `tests/statuses.rs`, `tests/flume.rs`, and the fakes in `tests/fakes/mod.rs` and `tests/once.rs`.
- `crates/boomdrive-crew`: `src/lib.rs` gives a kit's launch the runner answered whatever the hello names; `tests/claims.rs`.
- `README.md`: a place is followed to its end.
- Nothing else changes: no crate, no tool, no tracked file type, no message of the protocol.
- Outside this repository: a request made after the rename that names the repository by its old name is given to no runner until crew renames its kit. That is crew's, told to the orchestrator.

## Touches

```
docs/architecture/core.md
docs/adr/0021-the-runner-follows-each-place-it-launched-to-its-end-whether-or-not-it-still-keeps-the-places-repository.md
crates/boomdrive/src/flume.rs
crates/boomdrive/src/carry.rs
crates/boomdrive/tests/statuses.rs
crates/boomdrive/tests/flume.rs
crates/boomdrive/tests/once.rs
crates/boomdrive/tests/fakes/mod.rs
crates/boomdrive-crew/src/lib.rs
crates/boomdrive-crew/tests/claims.rs
README.md
```
