---
number: 13
title: Health goes by the protocol and an opt-in OTLP export, and nothing else leaves the host
status: accepted
date: 2026-11-01
decision-makers:
- Chuck Swanberg
---

# Health goes by the protocol and an opt-in OTLP export, and nothing else leaves the host

Status: accepted by the user on 2026-11-07, on their card. Proposed on 2026-11-01, from the user's words in the constitution interview.

Deciders: the user, 2026-11-01.
Sources: the constitution interview of 2026-11-01 (`design/research/the-constitution-interview.md`), its topic on telemetry; sluice's README on its opt-in OTLP export; Flume's, which sends none; agentplot/boomdrive-cloud's rules core.59 and core.60, cited by id; rule core.11.

## Context and problem statement

The runner sends nothing but its control plane's messages and git's calls, and it writes its lines on standard error. With thousands of hosts, and fleets later, the question was what the runner may send about its own health, and to whom.

## Considered options

1. Nothing by default, with an opt-in OTLP export to a collector the host names.
2. Health to the control plane through the protocol, and the first option as well.
3. Nothing at all.

## Decision outcome

Option 2. The user chose health to the control plane, and left the OTLP export to the recommendation: "you tell me what's better". The export was recommended and taken, off unless the host names a collector, since that is how an enterprise watches a fleet in its own observability, as sluice already does.

- **Health through the protocol.** The runner sends its control plane its health in the protocol's own messages: pass durations, failures by kind, reconnects and its tools' versions. The console can then show a fleet's health.
- **Nothing else leaves.** Nothing leaves a host from the runner but its control plane's protocol, git's calls, and an OTLP export to a collector the host's configuration names. There is no telemetry SDK, crash reporter or update ping to anyone else. What goes as health or by OTLP holds only allowed fields: ids, names, statuses, times, counts and digests, never a tool's text (core.28).

### Consequences

- The OTLP exporter is optional, built in and off by default, like sluice's.
- Under crew no health goes up, since crew's records have no place for it.
