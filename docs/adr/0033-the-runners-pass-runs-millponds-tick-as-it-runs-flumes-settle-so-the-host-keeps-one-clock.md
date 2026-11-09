---
number: 33
title: The runner's pass runs millpond's tick as it runs Flume's settle, so the host keeps one clock
status: proposed
date: 2026-11-09
decision-makers:
- Chuck Swanberg
- the research talk the-superfan-watches-the-family-at-work
---

# The runner's pass runs millpond's tick as it runs Flume's settle, so the host keeps one clock

Status: proposed on 2026-11-09 by the research talk `the-superfan-watches-the-family-at-work` (agentplot/crew, branch `research/the-superfan-watches-the-family-at-work`), until the user has seen it as a record and says yes. No unit is queued from it while it is proposed, and none before millpond's tick exists.

The schedule it serves is the user's: one superfan for each tool, every four hours, its findings in millpond's capture pattern (afterthought/swancloud record 0062 as amended through 910edb62). Who runs the clock is the talk's, from the rules, with the paths not taken.

Deciders: the user, on their card req_e43693069d374067b440, 2026-11-09: "One per tool (runner, Flume, sluice, crew), every 4 hours", and "These should be designed to be compatible with Milpond's capture pattern". And, for the host's one clock, 2026-11-06, quoted in record 0029: "I'd rather it be the Boomdrive process". The talk decided the rest.

Sources:
- agentplot/crew `openspec/explorations/the-superfan/proposal.md` §4, and crew record 0063, proposed with this one;
- millpond's statement: 28, 29 and C.4;
- the family's principle 8 (agentplot/blueprints `design/principles.md`, proposed);
- Flume 208;
- records 0003 and 0029;
- rules core.1, core.13 and core.11.

## Context and problem statement

A superfan's pass is a millpond adapter on a schedule, `0 */4 * * *`, and `millpond tick` runs each adapter that is due on its host (millpond 28). millpond's C.4 has "one timer on a host" run the tick, "on swancloud, a launchd agent per inbox". The family's principle 8 has the runner's pass as the host's one clock: "it catches what no process is there for, time, a crash, an outcome decided elsewhere". The user chose the runner over a terminal's plugin for Flume's settle (record 0029). Two clocks on one host is the disagreement the family's rule asks to reconcile into one owner.

core.13 lets the runner run one housekeeping of its own accord, Flume's settle.

## Considered options

1. **The runner's pass runs `millpond tick` for each inbox the host keeps, once a pass and awaited, beside the settle (chosen).**
2. **A host timer runs the tick**, as millpond's C.4 has it today.
3. **Flume schedules the superfans' runs.** Flume keeps no schedule, and wakes a role only on a pending end or a caller's wake (Flume 132).
4. **crew's follower wakes them on a cadence.** That would be a loop of crew's that launches roles, which crew's `roles.7` rules out.

## Decision outcome

Option 1.

- **The tick is the runner's second housekeeping.** Each pass runs `millpond tick` once for each inbox the host's configuration names, awaited on the pass's thread, as it runs the settle.
- **millpond decides what is due.** Due is millpond's cadence (weir 59): a pass that sleeps through several of an adapter's times runs it once (millpond 29).
- **The tick decides nothing of the pass's other work, and goes nowhere.**
  - A tick that fails, or names no inbox, stops no pass.
  - Its answer is carried to no control plane (core.11) and kept nowhere but millpond's own status.
  - What a tick runs, a superfan's adapter among them, is millpond's (core.1).
- **One line at each change**, as for the settle: one when a tick is refused or gives no answer, and one when the ticks are answered again.
- **core.13 is amended** to name the tick beside the settle, as the second and last housekeeping the runner runs of its own accord.
- **millpond's C.4 is amended** for the statement, for agentplot-design to take to agentplot/blueprints once accepted:

  > On a host that keeps the runner, the runner's pass runs `millpond tick` for each inbox there, once a pass and awaited, as it runs Flume's settle. A host with no runner runs it on a timer of its own, one per inbox.

Not taken:
- Option 2, a second clock on the host (principle 8).
- Option 3, a scheduler Flume does not have and its statement does not give it.
- Option 4, which `roles.7` rules out.

## Consequences

- Good: one clock on a host keeps Flume's watch and millpond's schedules. A host whose runner stops shows both lapsing together, never one silently.
- Good: the superfans' four-hourly passes need no timer of the fleet's.
- Bad: a slow tick delays the pass's next turn by as long, as a slow settle does. An adapter that runs a superfan's session waits for that session's end.
  - The tick should run its adapters without holding the pass, which is millpond's to say: a tick that starts its due adapters and returns.
  - Until millpond says so, a pass's tick waits for each adapter. The superfans' adapters are due at most once each four hours.
- Bad: the runner gains a dependency on millpond. On a host whose configuration names no inbox, the tick is never run.
- The rule on acceptance: core.13 names the tick beside the settle. The running and configuration specs follow in the change that builds it, once millpond's tick exists.
- Not decided here: whether a deterministic watch, an adapter of its own on the same tick, feeds each superfan or replaces its timer. That is the user's, asked through agentplot-design (crew `proposal.md` §5).
