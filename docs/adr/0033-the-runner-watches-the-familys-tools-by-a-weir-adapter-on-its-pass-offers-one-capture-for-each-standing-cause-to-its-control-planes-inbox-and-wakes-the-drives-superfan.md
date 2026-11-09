---
number: 33
title: The runner watches the family's tools by a weir adapter on its pass, offers one capture for each standing cause to its control plane's inbox, and wakes the drive's superfan
status: proposed
date: 2026-11-08
decision-makers:
- Chuck Swanberg
- the research talk the-superfan-watches-the-family-at-work
---

# The runner watches the family's tools by a weir adapter on its pass, offers one capture for each standing cause to its control plane's inbox, and wakes the drive's superfan

Status: proposed on 2026-11-08 by the research talk `the-superfan-watches-the-family-at-work` (agentplot/crew, branch `research/the-superfan-watches-the-family-at-work`). It is proposed until the user has seen it as a record and says yes. That an agent watches Boomdrive at work is the user's own word. The shape is the talk's, from the rules, each with the path not taken. No unit is queued from it while it is proposed.

Deciders: the user, 2026-11-08, in agentplot-design's pane: "I'm particularly interested in agent-first usage. I'd like to implement the superfan pattern from Anthropic, where I can have an agent monitoring Boomdrive in action. I think having it connected to a metrics platform would be part of that". The talk decided the rest.

Sources:
- agentplot/crew `openspec/explorations/the-superfan/reading.md` and `proposal.md`.
- crew record 0063, proposed with this one, for the role this record wakes.
- Ben Congdon, "Agentic Monitoring" (2026-10-19): "a harness that allows the agent to write a spec of how to monitor, along with a deterministic system for running those specs, produces significantly cleaner signals with dramatically lower token usage".
- `anthropics/oncall-kit`: "Claude is never the detector".
- weir's statement, 3, 12, 59 and C.1.
- millpond's statement, 9, 15, 17, 20, 26, 27 and 33, and C.4.
- The family's principles 3, 7, 8 and 12 (agentplot/blueprints `design/principles.md`, proposed).
- Records 0011, 0013, 0019, 0020, 0028, 0029 and 0032.
- Rules core.1, core.11, core.13, core.28 and core.35.
- sluice's core.28 and core.47; crew's `signals.6` and `roles.7`.

## Context and problem statement

An agent that polls the family's tools to find what is wrong pays for every look. On deneb on 2026-11-08 a full look read close to a megabyte of JSON, some 250,000 tokens, and found nothing new on most looks. The published shapes that work at less than Anthropic's budget detect deterministically, and wake a model only to read what was detected.

The runner already reads every tool's answers: it carries Flume's and sluice's records back as the tools gave them (core.1, core.11). Its pass is the host's one clock, "it catches what no process is there for, time, a crash, an outcome decided elsewhere" (principle 8). The user chose it over any other process for the host's watch on 2026-11-06: "I'd rather it be the Boomdrive process" (record 0029).

Every drive gets the runner with the family's flake (record 0020). Today it runs no check of its own, and core.13 lets it run one housekeeping of its own accord, Flume's settle.

## Considered options

1. **A weir adapter of the family's tools, run by the runner's pass on each check's cadence, its offers to the control plane's inbox, waking the drive's superfan on a new capture and once a day (chosen).**
2. **The adapter on a host timer of its own**, as millpond's C.4 has "one timer on a host" run each inbox's tick.
3. **A watch in crew**, in crew's follower.
4. **Each tool watching itself**, each offering its own captures.
5. **The platform's own check rules alone**, the platform calling out on a breach.

## Decision outcome

Option 1.

- **The watch is a weir adapter, `boomdrive watch`.**
  - It prints weir's lines, `capture`, `cursor` and `done` (weir C.1), and nothing else.
  - Each check names:
    - its source, a family tool's JSON command or a file the tool keeps;
    - what it reads in the answer and its condition;
    - how long the condition must stand, by the tool's own clock;
    - its key, built from the tool's own fields;
    - whether its capture is its own excerpt;
    - its schedule, five-field cron (weir C.5).
  - The first checks are a submission stuck in sluice, a flaky check, a standing refusal (sluice core.28 and core.47), a place of a kind that should not wait stalled past its bound, Flume's host watch lapsed, and main red after a landing. They are drafted from the superfan's first runs (crew record 0063).
- **It keeps nothing, and the tool names the cause.**
  - A key is the tool's identity of the cause and when it began, for example `sluice/refused/<repository>/<branch>/<host>/<command>/<first>` or `flume/stalled/<place>/<since>`.
  - So one standing cause gives one key on every run and every host (weir 3), and the watch keeps no list of what it offered (weir 12).
  - A cause that clears and returns begins again with a new time, a new episode.
- **The runner's pass runs it.**
  - Each pass runs the checks that are due (weir 59), once and awaited, beside its settle.
  - A watch that fails, or is not configured, stops no pass and decides nothing of a pass's other work.
  - core.13 is amended to name it, as the second and last housekeeping the runner runs of its own accord.
- **Its offers go to the control plane's inbox, through the seam the runner has.**
  - Under crew, `boomdrive-crew` hands each capture the inbox does not hold to `crew signal land` on the drive's branch, as a program's capture (crew `signals.6`).
  - Under the hosted control plane, `boomdrive-hosted` carries the inbox's records as it carries an inbox, never their raw material (core.11).
  - When millpond keeps a drive's inbox, the lines are millpond's at its intake (millpond 27). crew's signals move to it by `weir import signals`, a step done by hand, and no code carries the transition (principle 12).
- **It keeps no raw material** (core.11, core.35).
  - The watch writes each capture's raw material, the tool's answer as the tool gave it, in the raw place its inbox's host gives the run. The host banks it (millpond 15, 20).
  - Under crew, that is a change of crew's: `crew signal land` banks what the folder holds in crew's own raw bank. Today the program banks it.
- **It wakes the drive's superfan.**
  - A pass whose watch handed the inbox a capture it did not hold runs `flume wake <label>/superfan`, and so does the first pass of each day.
  - The address is declared by the control plane's roles file (crew record 0057; crew record 0063), and the wake is Flume's (crew `roles.7`).
  - core.1 is amended to name this wake beside `flume launch` as work the runner starts.
- **One drive's watch reads its own repositories.**
  - It reads the repositories the host's configuration names for the drive (record 0028).
  - Checks of the host itself, Flume's watch and the host's room, go to the drive whose machine the host is (record 0032).
- **Its silence is shown.**
  - Every run leaves its status: when it ran, whether it was complete, and what it took, held and refused (millpond 26).
  - The runner carries that status to the control plane as it carries a place's.
  - Under crew, the board shows a watch whose last run is older than twice its cadence as lapsed. Under the hosted control plane, a runner's health goes up by the protocol (record 0013).
- **The platform is a source of its checks, not their base.**
  - A check may read a platform's failed checks through the platform's own query, keyed by the rule and when it began failing, so a rule the keeper applies there becomes one capture per episode.
  - A host takes no inbound call (crew `roles.9`), so the runner reads; the platform never calls a host.

Not taken:

- **Option 2**, the host timer. It gives the host two clocks, the runner's pass and the timer, which principle 8 rules out. The user ruled it out for Flume's settle in record 0029. millpond's C.4 is told to the orchestrator for its amendment: millpond's tick run by the runner's pass, as the settle is.
- **Option 3**, the watch in crew. crew builds no mechanism a tool is designed to hold (crew `roles.5`), runs no loop that wakes roles (crew `roles.7`), and leaves when a drive's own control plane replaces it (crew `roles.9`).
- **Option 4**, each tool watching itself. What goes wrong in the family is mostly between the tools: a place waits on a submission, which waits on a check, which waits on a slot. Tools fit by reading each other's records (principle 7). Each tool already keeps its own causes, as sluice's core.47 does, and the watch reads them there.
- **Option 5**, the platform's check rules alone. They see only what is exported, which on deneb today is nothing, and nothing of Flume, crew or the runner (crew's reading, §4). A breach would need an inbound call to a host. And a person with no platform would get no watch (principle 3).

## Consequences

- Good: the detection costs no tokens, and the superfan is woken only for a cause it has not seen.
- Good: every drive gets the watch with the runner, and an operator with no platform gets the same checks.
- Good: one standing cause is one capture, however many passes see it, with no state of the watch's own.
- Bad: a dead runner stops the watch too. Its silence is shown as a lapse, never read as health; under crew alone, a person sees it on the board.
- Bad: a slow check delays the pass's other work by as long. Each check reads one answer or one file, and the pass awaits it as it awaits the settle.
- Bad: until millpond keeps the drive's inbox, a capture that needs judgment lands as its own signal under crew, so a false alarm reaches curation as one signal.
- The rules on acceptance:
  - core.13 names the watch beside the settle as the runner's housekeeping.
  - core.1 names `flume wake` of the drive's superfan beside `flume launch`.
  - A new rule says the watch keeps nothing, its key is the tool's own identity of the cause, and its raw material is the inbox host's to bank.
  - The running and configuration specs follow in the change that builds it.
- It depends on `boomdrive status --json` (record 0011, not built) only for the runner's own health checks. The first checks read Flume's host watch instead.
