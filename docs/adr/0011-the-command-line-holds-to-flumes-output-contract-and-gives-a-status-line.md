---
number: 11
title: The command line holds to Flume's output contract, its lines carry their time, and it gives a status line
status: accepted
date: 2026-11-01
decision-makers:
- Chuck Swanberg
---

# The command line holds to Flume's output contract, its lines carry their time, and it gives a status line

Status: accepted by the user on 2026-11-01, in the constitution interview.

Deciders: the user, 2026-11-01.
Sources: the constitution interview of 2026-11-01, its topics on the command line and on what stays private; the running and work-into-flume specs; Flume's README and statement on its JSON form, refusals and exits, and sluice's on its own; agentplot/flywheel-next, the spike `research/board-shows-flume-sluice-and-timber` of 2026-11-01, its page 8 and notes on the unit line; rule core.3.

## Context and problem statement

The runner has `--version` and `run [--config] [--once]`. It exits 0, 1 or 2, and writes one line in words on standard error for each thing it does. The interview adds `enroll` and `forge-token` (record 0006 and record 0007) and a setup wizard (record 0009). Flume and sluice each answer in JSON, with fixed refusal shapes and exit codes that differ from each other. Under launchd, a log file adds no time to a line.

## Considered options

For local commands: status, drain and resume, check, and upgrade, chosen in any combination.

For the output contract: Flume's, sluice's, or words only.

For the running runner's lines: words with a time, JSON lines, or as now.

## Decision outcome

- **All four local commands**, the user's choice. `status [--json]` is read-only and needs no lock: the repositories kept, the leases held, what was launched and where, whether the runner is drained, and its last pass. `check` reads the configuration, the tools and their versions, the reach of origin and of the control plane, and the enrolment. It changes nothing, and it exits 0 or names what is wrong. `drain`, `resume` and `upgrade` are the control plane's commands, given on the host (record 0008).
- **A status line.** The user added: "I've been wondering if there should be a Claude Code status line concept as well". The board spike proposes one shape for the board inside each agent's terminal: each tool writes its own token in the family's key, and a mod, herdr's rows or a person's own `statusLine` lays them out, for display only. The runner writes its token as `flywheel line`, beside `flume line`. It reads its data directory alone, with no network call, and gives the host's reach, the leases held, a drain, and the unit a place's launch is. The runner itself draws nothing in a session.
- **Flume's output contract**, the user's choice. Every command that answers takes `--json`, and within a version its JSON only ever gains fields. A refusal is `{"refused":{"refusal":{"reason":…}}}` on standard output. It exits 0 answered, 1 refused, and 2 for a usage error or a host that must be fixed (core.39).
- **A line carries its time**, the user's choice. Each line the running runner writes starts with its RFC 3339 time in UTC, to the second, then its subject and its words (core.40).

### Consequences

- `run`'s lines stay words on standard error. A machine reads `status --json` or the control plane, never the log.
- `flywheel line` is built with the family's mod or line, in whichever repository holds it, and the runner's token takes the family key's marks.
