---
number: 1
title: The runner is a separate private repository, and its binary is flywheel
status: accepted
date: 2026-10-31
decision-makers:
- Chuck Swanberg
---

# The runner is a separate private repository, and its binary is flywheel

Status: accepted by the user on 2026-10-31. Its "It is private for now" is superseded by record 0012, accepted on 2026-11-02: the runner is open source.

Deciders: the user, on 2026-10-31.
Sources: flume's `docs/adr/0001` as the model for the form; flume and sluice, whose scaffolding this repository copies.

## Context and problem statement

Each host needs a thin runner: it keeps the clones and views of the repositories its configuration names, and turns the control plane's work into `flume launch` calls, carrying their results back. It reaches the control plane through one interface, crew's state repository now and a hosted control plane's outgoing WebSocket later. Where does the runner's code live, is it open, and what is its binary called?

## Considered options

1. The runner as a part of flume, a command of flume's own binary.
2. The runner as a part of crew.
3. The runner as a repository of its own, agentplot/flywheel-runner, private, its binary named `flywheel`.

## Decision outcome

Option 3, the user's decision on 2026-10-31. flume carries one launch through to its landing and decides nothing about what work there is; crew is one control plane among two to come. A runner inside either would tie it to that one's release and let it reach that one's internals, where the runner must reach flume only through `flume launch` and the control plane only through its one interface. A repository of its own keeps both seams honest. It is private for now; opening it later stays possible, and nothing here assumes it stays closed. Its binary is `flywheel`, the name a host's operator knows the whole by.

The decisions after this one are recorded here, one MADR record each, and the rules in `docs/architecture/core.md` name them as sources.

### Consequences

- The repository carries no license file while it is private.
- Nothing commercial, no pricing, tier or customer, appears in it, so opening it needs no scrub (core.6).
- The runner reaches flume by its command line and the control plane by one interface (core.1, core.2).
