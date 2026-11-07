---
number: 10
title: A lost lease's place is left until a command drops it, and the runner never retries back to back
status: accepted
date: 2026-11-01
decision-makers:
- Chuck Swanberg
---

# A lost lease's place is left until a command drops it, and the runner never retries back to back

Status: accepted by the user on 2026-11-07, on their card. Proposed on 2026-11-01, from the user's words in the constitution interview. Amended on 2026-11-07, below: an act Flume refuses because its place no longer stands costs its answer alone.

Deciders: the user, 2026-11-01.
Sources: the constitution interview of 2026-11-01 (`design/research/the-constitution-interview.md`), its topic on failure and offline behaviour; the clones, configuration, running and work-into-flume specs; the verify report of `the-runner-carries-crews-work-into-flume`, its observations; agentplot/boomdrive-cloud's rules core.13 and core.39, cited by id; rules core.9 and core.10.

## Context and problem statement

A failed git call, an unread configuration, an unread pull and an unsent report are each named, and the runner goes on. A lease the control plane refuses is let go, and nothing of it is removed (core.10).

In the cloud, a lease not renewed ends at its expiry and its work returns to the board (cloud core.39). A host that is offline past an expiry, through sleep, an upgrade or a network loss, comes back to a refused report. Its place is still running in Flume, and Flume submits through sluice on its own, while another host may have launched the same work. As built, a lapsed lease also makes control passes run back to back while the control plane cannot be reached.

## Considered options

For the live place: left, with the cloud sending a drop; the runner holding its submission; or the runner dropping it.

Then four rules drawn from the material, each kept or not: no launch past its lease; a backoff, never back to back; no act the runner cannot record; presence is the cloud's.

## Decision outcome

- **Left until a command drops it**, the user's choice. A place the runner let go stays as Flume holds it, and named, until the control plane sends a command under a lease of its own that drops it by its address. The runner carries that command as Flume's `drop`. core.10 stands: the lost lease's work is not acted on, and the command is new work. core.10's details are amended in this record's commit to allow it.
- **No launch past its lease**, kept. An offer whose lease has expired by the runner's clock is not launched, acted on or claimed until a pull gives it again under a live lease (core.24).
- **A backoff, never back to back**, kept. A control plane that cannot be reached is tried again on a bounded backoff with jitter, from seconds up to `poll_interval`, and a lease already lapsed sets no pass due at once (core.22).
- **No act the runner cannot record**, kept. A pass whose report cannot be saved carries out no further act or command that pass (core.25). This holds as built.
- **Presence is the cloud's**, kept. The runner raises no alarm of its own about being offline. It says one line when its control plane becomes unreachable and one when it returns, and the Host object tells the drive (core.23).

### Consequences

- The cloud is asked for the command that drops a stray place, sent when a lapsed lease's work goes to another host. Where two places of one unit conflict, sluice's gate refuses whichever lands second.
- The back-to-back passes the verify report names are fixed by a change of their own.
- A host offline for longer than its leases loses them, and its places keep running until the control plane drops them.

## Amendment, 2026-11-07: an act Flume refuses because its place no longer stands costs its answer alone

Status: the code stage's own choice, on the brief of the chore `a-launch-goes-ahead-of-other-acts`, quoted in record 0002's amendment of the same day.

- **Its answer is the whole of it.** An act on a place Flume no longer stands is carried out as any act on a place: Flume's command, run once. Its refusal, "no place <location> stands on this host", is its answer, kept in the report and carried back as Flume gave it (core.10, core.11). The runner runs it no second time and asks Flume nothing more for it, and the pass goes on to its next act.
- **It holds back no launch.** Since record 0002's amendment of 2026-11-07, it runs after every launch the pull offers (core.49).
- **What the answer takes is Flume's.** Flume took about half a minute to refuse one on the host where this was seen. That is told to the orchestrator for Flume. The runner does not answer the act from Flume's listing in Flume's place (record 0002's amendment, option 2).
