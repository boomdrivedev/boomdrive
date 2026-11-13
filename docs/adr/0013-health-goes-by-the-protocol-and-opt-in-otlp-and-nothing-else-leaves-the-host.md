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
Sources: the constitution interview of 2026-11-01 (agentplot/boomdrive's `design/research/the-constitution-interview.md`), its topic on telemetry; sluice's README on its opt-in OTLP export; Flume's, which sends none; agentplot/boomdrive-cloud's rules core.59 and core.60, cited by id; rule core.11.

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

## Amendment, 2026-11-12: a drive's Claude Code sessions export their own telemetry the same way

Status: accepted by the user on 2026-11-12, on their card. Every Claude Code session on a drive's accounts exports Claude Code's own telemetry to the collector its host names and nowhere else, with no session's words in it, and the user's hosts send its traces to Dash0 alone.

Deciders: the user, 2026-11-12, in agentplot-design's pane, on Dash0's AI SDLC Insights: "I think I would want traces only for dash0 not for DataDog. We would want to wire up this Claude Code integration as part of the nix setup for Claude or as part of the Boom Drive configuration. Either way it might be nice to externalize it to Boom Drive. I can't tell". That traces reach Dash0 alone is the user's. That Boomdrive's set carries the wiring, which export it turns on and what the export holds are agentplot-design's, from this record and core.28, each with the path not taken.

Sources: Claude Code's monitoring page (`code.claude.com/docs/en/monitoring-usage`: its metrics, its events as logs, its traces in beta, and a session's words redacted unless a setting adds them); Dash0's AI SDLC Insights pages (setup, key concepts and troubleshooting under `dash0.com/docs/dash0/darkplane/insights`: a session is read from its spans, Claude Code's own tracing is taken beside Dash0's agent plugin, and a session counts toward a pull request by its repository and branch); afterthought/swancloud records 0061 and 0062 and its `lib/agent-config.nix`, where only the Bedrock account's sessions export Claude Code's metrics and logs today, and no session exports traces; signal `2026-11-13-agentplot-design-11141437/01`.

A Claude Code session can export its metrics, its events as logs, and its traces over OTLP. On the user's hosts today only the Bedrock account's sessions export anything, and none exports traces (swancloud record 0061). The user wants their coding sessions in Dash0, whose AI SDLC Insights reads a session from its traces, and wants traces to reach Dash0 and no other backend.

- **Traces reach Dash0 alone (the user's).** The user's hosts send their sessions' traces to Dash0, and never to Datadog or any other backend. Which backend each signal reaches is the host collector's choice, so this is a setting of the user's hosts in afterthought/swancloud, written there as an amendment of its record 0061. Boomdrive names no backend.
- **Boomdrive's set carries the wiring.** Every Claude Code session on a drive's accounts exports Claude Code's own telemetry to the collector the host's configuration names, as the runner does by this record. That covers a Flume place's session, a standing role's and the person's own. With no collector named, a session exports nothing. The host names which of metrics, logs and traces its sessions export. Not taken: the wiring in swancloud's Nix alone, which every other person's drive would have to write again; and each account's settings written by hand, which a new account misses.
- **Only to the host's collector, with no backend's key in a session.** A session exports to the collector on its own host and nowhere else, and the collector holds each backend's key (core.28). Not taken: a session exporting straight to a backend, which puts that backend's key in every session and every tool the session runs.
- **Claude Code's own export, and no vendor's plugin.** The set turns on Claude Code's own OTLP export, traces included where the host names them, and installs no backend's plugin. A host that wants more than Claude Code's own telemetry gives, such as Dash0's agent plugin with its tool calls and skills, adds that plugin from its own marketplace, pointed at its collector. Not taken: Dash0's agent plugin in the set, which ties every drive to one vendor and exports under its own token.
- **No session's words leave in it.** The export holds Claude Code's counts, times, models, costs, tool names and outcomes, and the session, drive and account it came from. It holds no prompt, response, tool input, command, file path or tool output, and the settings that add them stay off (core.28). Not taken: prompts and tool details for Dash0's conversation replay, which would send a session's words off the host.
- **Each session names its drive and account.** Every agent commits under the same person's name, so a backend tells sessions apart only by the drive and account each names.

### Consequences of the amendment

- Good: a person turns on their coding sessions' telemetry by naming a collector, as for the runner, and chooses each signal's backend in one place, the collector.
- Bad: Claude Code's traces are a beta, so a release can change their shape, and Dash0's Insights may read less from them than from its own plugin. Whether Insights fills from Claude Code's own traces is read on one host before the setting reaches every host.
- Neutral: Dash0 counts a session toward a pull request by its branch. A Flume place works on its own branch and lands through its bolt's, so Dash0's pull-request views count few sessions as assisted. That is Dash0's reading, and this amendment doesn't change it.
