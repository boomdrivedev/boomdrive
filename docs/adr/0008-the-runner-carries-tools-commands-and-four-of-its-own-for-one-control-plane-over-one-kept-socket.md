---
number: 8
title: The runner carries tools' commands and four of its own, for one control plane, over one kept socket
status: accepted
date: 2026-11-01
decision-makers:
- Chuck Swanberg
---

# The runner carries tools' commands and four of its own, for one control plane, over one kept socket

Status: accepted by the user on 2026-11-01, in the constitution interview.

Deciders: the user, 2026-11-01.
Sources: the constitution interview of 2026-11-01, its topics on control planes and on the command line; the control-plane-interface, branch-commands and crew-control-plane specs; agentplot/flywheel-cloud's records 0017, 0022 and 0026 and its rules core.36 to core.39, core.50 and core.52, cited by id; agentplot/flywheel-cloud `design/research/awesome-lists-disposition.md`, row F49; rules core.1, core.2 and core.3.

## Context and problem statement

The runner reaches its control plane through one interface (core.2). crew's implementation is built, and the hosted one is named and not yet built. Commands so far are a bolt's branch, made by sluice; any other sort is refused. The cloud's model also lists cancel, drain, upgrade and setup as controls, plus cloning a repository, and defines none of them. One `[control_plane]` table with one `type` was ruled before the interview. Still open: what becomes of crew's implementation, which commands the runner carries, how a person on the host reaches a running runner that opens no socket, and how the hosted socket is kept.

## Considered options

For crew's implementation: removed once the operator's own work runs on the cloud, or kept for good.

For commands:

1. Tools' commands, plus drain and resume.
2. That, and also upgrade and setup.
3. That, and also a clone.

For the socket (F49): protocol pings and a resend of what was not acknowledged after every reconnect; an application heartbeat; or leaving both to the change that builds it.

## Decision outcome

- **crew until the cloud**, the user's choice. crew's implementation stays, held to the shared cases, until the operator's own partitions run on the hosted control plane, and one change then removes it.
- **One control plane per runner**, kept by the user from the material. A runner serves the one control plane its `type` names, and every partition it serves is that control plane's (core.19).
- **Tools' commands and four of the runner's own**, option 2, the user's choice. Every command runs one tool's own command line once, under its lease, and its answer is carried as the tool gave it. Otherwise it is one of the runner's own: drain, which takes no new offer and carries on with what the runner holds; resume; upgrade, which installs a release of the runner; and setup, which configures the host (record 0009). No command adds a repository for the host to keep (core.16).
- **A local command by file**, kept by the user from the material. `drain`, `resume` and `upgrade` given on the host reach a running runner only through a file in its data directory, which it reads at its next pass (core.20; core.3).
- **Pings and a resend**, F49 kept. The runner keeps its socket alive with the WebSocket protocol's own pings, never an application message, so an idle socket wakes no object. After every reconnect it sends its whole hello and resends, each under its own id, what the control plane has not acknowledged (core.21).

### Consequences

- A change to crew's records or to its reading of what a role is wanted for (`asks.py`) is this repository's chore until crew's implementation goes.
- A drain survives a restart, since it is a file in the data directory.
- A stray place's drop is a command of the control plane's under a lease of its own (record 0010).
- The hosted implementation is built with tokio-tungstenite, its reconnect on the backoff of record 0010.
