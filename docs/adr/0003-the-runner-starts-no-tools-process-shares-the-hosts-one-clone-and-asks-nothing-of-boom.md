---
number: 3
title: The runner starts no tool's process, shares the host's one clone, and asks nothing of Boom for a launch
status: proposed
date: 2026-11-01
decision-makers:
- Chuck Swanberg
---

# The runner starts no tool's process, shares the host's one clone, and asks nothing of Boom for a launch

Status: proposed on 2026-11-01, from the user's words in the constitution interview; accepted on the user's yes once they have read it as a record.

Revised 2026-11-06, still proposed, for sluice record 0093 (accepted by the user that day): sluice's queues move in sluice's own host process, so a host's service manager keeps no sluice process; the context's `sluice run` describes the tools as they stood on 2026-11-01.

Revised 2026-11-06, still proposed, on the user's word that day: "I don't know what tool or how Boom should be coupled to Flume and/or Boom Drive". The record now decides only that the runner asks nothing of Boom; which tool asks Boom for a launch's claims, and how Boom is coupled to Flume and Boomdrive, is left to Boom's own design.

Deciders: the user, 2026-11-01.
Sources: the constitution interview of 2026-11-01 (agentplot/boomdrive-archive's `design/research/the-constitution-interview.md`), its topic on what the runner owns; flume's statement, clauses 1, 70 and 119 to 120, and flume record 0020; sluice's statement, clauses 104 to 106, and sluice record 0026; agentplot/flywheel-next, the spike `research/board-shows-flume-sluice-and-timber` of 2026-11-01, its assumption 7; rules core.1 and core.5.

## Context and problem statement

core.1 holds that the runner decides nothing the control plane or Flume decides, and core.5 that it keeps a clone only of a repository its configuration names. Three things around them were unsettled.

- **Tools' long-running processes.** A repository's landings need sluice's driver, `sluice run`. A host with no multiplexer watch moves its places only when a Flume command runs, `flume settle` among them. Flume needs the multiplexer's server. Nothing said whether the runner starts or keeps any of them.
- **The clone.** Flume makes its places in the runner's clone container. crew makes a landing's place there and submits from it, so sluice's records land in the runner's bare clone. Nothing said whose the clone is.
- **Boom's claims.** The board spike drew the runner running `boom query` before a launch and putting the standing claims in the launch's brief. Boom's own design, not yet written, will say who asks.

## Considered options

For the processes:

1. None: each is the host's service manager's, as the runner itself is.
2. The runner keeps one `sluice run` for each repository it keeps.
3. The runner starts or stops a tool's process only as a command the control plane sends.

For the clone:

1. The host's one clone, shared: Flume, sluice and what a launch runs may work in it.
2. The runner's and Flume's only.

For Boom's claims:

1. The runner asks Boom nothing; which tool asks, Flume at the launch or the control plane in the brief, is Boom's own design to decide.
2. The runner asks, as the spike drew.

## Decision outcome

Option 1 each time, the user's choice.

- **No tool's process.** The runner starts, keeps and stops no tool's long-running process, and runs no tool's housekeeping of its own accord. The multiplexer's server and any settle loop are the host's service manager's, and sluice's host process is sluice's own, started by its commands and gone when idle, so no host keeps a sluice service (core.13; sluice record 0093).
- **The host's one clone.** The runner's clone is the host's one clone of the repository. Flume, sluice and what a launch runs may work in its container. The runner alone fetches it and moves its view. It moves no branch of the clone and reads each at origin's tracking ref (record 0019), and it never removes, moves or rewrites what another tool made there (core.14).
- **The runner asks Boom nothing.** The runner passes a launch as its control plane gave it and reads nothing of Boom for it (core.15). Which tool asks Boom for a launch's standing claims is not decided here.

### Consequences

- crew's main-level checkouts come from the runner's clone rather than from swancloud's spaces, and swancloud's own containers retire. Both are asks of theirs.
- sluice's statement, which still lists Boomdrive among its drivers, is read as Flume being the driver of a submission.
- How Boom is coupled to Flume and Boomdrive, and which of them asks Boom for a launch's claims, is decided with Boom's own design. Until then a launch carries what its control plane wrote.
- The spike's assumption 7 is read with this record: the runner composes no brief.
