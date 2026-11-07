---
number: 5
title: The runner's files roll back one release, and its data directory is made again from origin, Flume and the control plane
status: accepted
date: 2026-11-01
decision-makers:
- Chuck Swanberg
---

# The runner's files roll back one release, and its data directory is made again from origin, Flume and the control plane

Status: accepted by the user on 2026-11-07, on their card. Proposed on 2026-11-01, from the user's words in the constitution interview.

Deciders: the user, 2026-11-01.
Sources: the constitution interview of 2026-11-01 (`design/research/the-constitution-interview.md`), its topic on the state directory; the clones, running and work-into-flume specs; the design of `the-runner-carries-crews-work-into-flume`, Decisions 11 and 17, and its verify report's observations; agentplot/boomdrive-cloud's rule core.67, cited by id, as the model; rules core.10 and core.11.

## Context and problem statement

The runner keeps a data directory: its lock, its clones, its report at `control/report.json`, a launch's brief while Flume reads it, and each implementation's caches. It also records in git the address of each clone it made and the commit each view was last put at. Each act and command is carried out once across a restart (core.10). A launch whose answer was lost is found by its mark, and a report lost with the data directory is taken from the control plane.

Nothing said what holds of these files across an upgrade or a rollback. Nor did anything say what the directory may and may not hold.

## Considered options

1. Roll back one release: a release never drops or renames a field the release before it reads.
2. Forward only: each file names its version, a release rewrites the one before it, and going back means clearing the control files.
3. Disposable: a file the runner cannot read is set aside and rebuilt.

## Decision outcome

Option 1, the user's choice.

- **One release back.** A release never drops, renames or changes the meaning of a field that the release before it reads in a file the runner keeps, so a host goes back one release with its data directory as it stands. A file the runner cannot read makes it refuse to start, with exit 2, and that file is never discarded or overwritten (core.36).
- **What the data directory holds**, kept by the user from the material: the lock, the clones, the report, a launch's brief while Flume reads it, and each implementation's caches. Everything in it but the report is made again from origin, Flume and the control plane, and the report is taken from the control plane where it is lost (core.35).

### Consequences

- A field is added in one release and dropped no sooner than the release after the one that stopped reading it.
- A brief left by a launch that was killed is removed at the next start, since no launch holds it (the verify report's "a brief left by a killed launch").
- The host's key is not in the data directory: it is in the host's own credential store (record 0006).
