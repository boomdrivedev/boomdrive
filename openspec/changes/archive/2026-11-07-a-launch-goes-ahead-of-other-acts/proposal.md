# Proposal

## Why

A control pass answered each want in the control plane's order. crew's implementation gives its wants in `Request` order, so acts asked before a launch come ahead of it.

On 2026-11-07 a host's pull gave some twenty priority acts on launches whose places Flume no longer stood, ahead of new offers. Flume refused each, "no place <location> stands on this host", and each refusal took Flume about half a minute on that host. The launches waited some ten minutes behind them.

The orchestrator's brief, 2026-11-07: "Boomdrive's runner takes a pull's launches ahead of its other acts, and an act Flume refuses because its place no longer stands costs no more than its answer, as records 0002 and 0010 say, so a launch never waits behind acts on places that are over."

## What Changes

- **Two rounds over the pull**, each in the control plane's order. The first takes each offer, inbox and command, and answers each act that runs no command. The second carries out each act that runs Flume's command on a place, after every launch of the pull.
- **No answer changes.** Room and holds are read from the pass's one listing, and an act on a place moves neither within the pass. An act ahead of its own launch is carried out on the place that launch makes in the same pass, where it waited for the next.
- **A refused act's answer is the whole of it.** An act Flume refuses because its place no longer stands is answered with Flume's refusal once, and holds back no launch.
- **Rule core.49** is added, and core.1's details allow it. Records 0002 and 0010 are amended with it.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `work-into-flume`: a pull's launches go ahead of its acts on places, and no answer turns on the round a want is taken in.

## Impact

- `docs/architecture/core.md` adds core.49 and amends core.1's details. Records 0002 and 0010 gain amendments of 2026-11-07.
- `crates/boomdrive`:
  - `src/carry.rs`: the pass's two rounds;
  - `tests/acts.rs`: the spec's three new scenarios.
- `README.md`: when a control pass carries out an act on a place.
- Nothing else changes: no crate, no tool, no tracked file type, no message of the protocol, no control plane's implementation.
- Outside this repository: Flume took about half a minute to refuse an act on a place that no longer stands. That is Flume's, and is told to the orchestrator.

## Touches

```
docs/architecture/core.md
docs/adr/0002-the-runner-takes-leased-work-while-its-flume-stream-has-room-and-carries-each-tools-record-back-as-given.md
docs/adr/0010-a-lost-leases-place-is-left-until-a-command-drops-it-and-the-runner-never-retries-back-to-back.md
crates/boomdrive/src/carry.rs
crates/boomdrive/tests/acts.rs
README.md
```
