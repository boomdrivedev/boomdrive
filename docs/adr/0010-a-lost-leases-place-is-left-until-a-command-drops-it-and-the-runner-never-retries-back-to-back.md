---
number: 10
title: A lost lease's place is left until a command drops it, and the runner never retries back to back
status: accepted
date: 2026-11-01
decision-makers:
- Chuck Swanberg
---

# A lost lease's place is left until a command drops it, and the runner never retries back to back

Status: accepted by the user on 2026-11-07, on their card. Proposed on 2026-11-01, from the user's words in the constitution interview. Amended on 2026-11-07, below: an act on a place Flume does not stand costs the reading of Flume's refusal.

Deciders: the user, 2026-11-01.
Sources: the constitution interview of 2026-11-01 (agentplot/boomdrive's `design/research/the-constitution-interview.md`), its topic on failure and offline behaviour; the clones, configuration, running and work-into-flume specs; the verify report of `the-runner-carries-crews-work-into-flume`, its observations; agentplot/boomdrive-cloud's rules core.13 and core.39, cited by id; rules core.9 and core.10.

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

## Amendment, 2026-11-07: an act on a place Flume does not stand costs the reading of Flume's refusal

Status: the code stage's own choice, on the brief of the chore `a-launch-waits-on-no-other-act`, quoted in record 0002's second amendment of the same day. It changes how a refusal is read, not what it says, so it is cheap to change and is not asked.

### Context

Flume's command acting on a place settles every place on the host first, where no caller keeps watch (Flume's clause 119), and only then finds that the place does not stand. On a host on 2026-11-07, `flume --json priority <place> 12` refused a place that no longer stood in 27 to 33 seconds. `flume --json status <place>`, which settles nothing (Flume's clause 119), refused the same place in about ten milliseconds, in the same words: "no place <location> stands on this host". Of some 180 acts on places one pass carried out there, 162 were such refusals.

### Considered options

1. Run Flume's act command on every place, its refusal the act's answer, once.
2. Answer an act on a place the pass's listing does not hold from that listing, running no Flume.
3. Read Flume's status of a place the pass's listing does not hold before running the act's command on it: where Flume refuses it, carry that refusal as the act's answer; where Flume gives it, run the command.
4. Leave it to Flume, to refuse a place that does not stand before it settles.

### Decision outcome

Option 3.

- **Flume is asked, of the place.** Where the pass's listing holds the place, standing or as the record Flume keeps of it tidied, the act is Flume's command, as before. Where it does not, or no listing was read, the runner first reads `flume --json status <place>`, naming no repository, as the act's command names none.
- **Flume's refusal is the answer.** Where Flume refuses the place's status, the act is answered `refused` with Flume's words, as Flume gave them, kept in the report and carried back (core.10, core.11), and the act's command is not run. The runner reads none of the refusal's words to decide it, and runs nothing more for the act, at that pass or after.
- **Flume's status given, its command runs.** On a place whose status Flume gives, a tidied place's record among them, the act's command is run, and Flume's answer is the act's.
- **Flume not run, nothing kept.** Where Flume could not be run for the status, the act is left unanswered and named, and carried out at the next pass, as an act whose command could not be run is.
- **It holds back no launch** (record 0002's amendments of 2026-11-07; core.50).

Not taken:

- **Option 1.** An act on a place that no longer stands costs the pass Flume's settle of every place, for the answer Flume's status gives in milliseconds.
- **Option 2.** An act's answer is Flume's (core.1, core.11): the runner would judge from its listing what Flume says of the place.
- **Option 4.** It is Flume's to do, and is told to the orchestrator for Flume, with a caller keeping watch on the host (Flume's clause 208), under which no act settles first. Until Flume does it, each refusal would still cost a settle.

core.51 holds it, and core.1's details allow it. The work-into-flume spec gains it through the chore `a-launch-waits-on-no-other-act`.
