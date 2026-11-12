---
number: 33
title: The runner's pass runs millpond's tick as it runs Flume's settle, so the host keeps one clock
status: accepted
date: 2026-11-09
decision-makers:
- Chuck Swanberg
- the research talk the-superfan-watches-the-family-at-work
---

# The runner's pass runs millpond's tick as it runs Flume's settle, so the host keeps one clock

Status: accepted by the user on 2026-11-11, on their card "Make the superfan plan official: a read-only Claude session per tool every 4 hours, reporting what it finds?": "Approved". Proposed on 2026-11-09 by the research talk `the-superfan-watches-the-family-at-work` (agentplot/crew, branch `research/the-superfan-watches-the-family-at-work`). No unit is queued from it before millpond's tick exists.

What the clock serves is the user's: one superfan for each tool, every four hours, its findings in millpond's capture pattern, and a watch beside the passes. Who runs the clock, and the runner's own watch, are the talk's, from the rules, with the paths not taken.

Deciders: the user.
- On their card req_e43693069d374067b440, 2026-11-09 (afterthought/swancloud record 0062 as amended): "One per tool (runner, Flume, sluice, crew), every 4 hours", and "These should be designed to be compatible with Milpond's capture pattern".
- On their card of 2026-11-09, through agentplot-design: on the watch, "Yes, beside the 4-hourly passes"; on the order, "Build Millpond's intake first, with crew".
- Their note there: "How does the plane program know there's a problem? Isn't part of the point that the superfan is evaluating what it sees and looking for problems with intent? / The blueprints repo does have a captures or signals folder. We can write captures there manually or run `write captures` into Crew. We could work on Mill Pond but I think we could write captures directly to Crew in the meantime / I assume that once a Claude session runs it could use the MCP server then to do additional analysis".
- For the host's one clock, 2026-11-06, quoted in record 0029: "I'd rather it be the Boomdrive process".

The talk decided the rest.

Sources:
- agentplot/crew `openspec/explorations/the-superfan/proposal.md` §4 and §5, and crew record 0063, proposed with this one;
- millpond's statement: 17, 28, 29, 31, 33, §9 and C.4;
- the family's principle 8 (agentplot/blueprints `design/principles.md`, proposed);
- Flume 132 and 208;
- records 0003, 0011 and 0029;
- rules core.1, core.11 and core.13.

## Context and problem statement

Once millpond's tick runs, each superfan's pass is a millpond adapter on the schedule `0 */4 * * *`, and each tool's watch is an adapter on a schedule of its own. `millpond tick` runs each adapter that is due on its host (millpond 28). Two tools disagree on what runs the tick:
- millpond's C.4 has "one timer on a host" run the tick, "on swancloud, a launchd agent per inbox".
- The family's principle 8 has the runner's pass as the host's one clock: "it catches what no process is there for, time, a crash, an outcome decided elsewhere". The user chose the runner over a terminal's plugin for Flume's settle (record 0029).

Two clocks on one host is a disagreement the family's rule asks to reconcile into one owner. Today core.13 lets the runner run one housekeeping of its own accord, Flume's settle.

The watch beside the passes knows a problem only where a tool already names one. An adapter ships in the plugin of the tool whose domain it wraps (millpond §9), so the runner's watch is Boomdrive's.

## Considered options

1. **The runner's pass runs `millpond tick` for each inbox the host keeps, once a pass and awaited, beside the settle; and Boomdrive ships the runner's own watch adapter (chosen).**
2. **A host timer runs the tick**, as millpond's C.4 has it today.
3. **Flume schedules the superfans' runs.** Flume keeps no schedule, and wakes a role only on a pending end or a caller's wake (Flume 132).
4. **crew's follower wakes them on a cadence.** That is a loop of crew's that launches roles, which crew's `roles.7` rules out.
5. **One watch of every tool in Boomdrive.** One program would know four tools' causes, where each tool knows its own, and millpond §9 has a tool ship the adapters of its own domain.

## Decision outcome

Option 1.

- **The tick is the runner's second housekeeping.** Each pass runs `millpond tick` once for each inbox the host's configuration names, awaited on the pass's thread, as it runs the settle.
- **millpond decides what is due.** Due is millpond's cadence (weir 59): a pass that sleeps through several of an adapter's times runs it once (millpond 29).
- **The tick decides nothing of the pass's other work, and goes nowhere.**
  - A tick that fails, or names no inbox, stops no pass.
  - Its answer is carried to no control plane (core.11) and kept nowhere but millpond's own status.
  - What a tick runs is millpond's (core.1): a superfan's adapter, a tool's watch, and the reader it starts for an unread capture (millpond 33).
- **One line at each change**, as for the settle: one when a tick is refused or gives no answer, and one when the ticks are answered again.
- **core.13 is amended** to name the tick beside the settle, as the second and last housekeeping the runner runs of its own accord.
- **Boomdrive ships the runner's watch**, an adapter in its plugin with no model. It offers each standing cause the runner names of itself as one capture, with raw material and no own excerpt, so it is unread (millpond 17):
  - a failing check of `boomdrive check` (record 0011, once built);
  - a pass failing by kind.

  It is keyed on the runner's own name for the cause and when it began. It judges nothing: the runner superfan reads it (crew record 0063).
- **millpond's C.4 is amended** for the statement, for agentplot-design to take to agentplot/blueprints once accepted:

  > On a host that keeps the runner, the runner's pass runs `millpond tick` for each inbox there, once a pass and awaited, as it runs Flume's settle. A host with no runner runs it on a timer of its own, one per inbox.

Not taken:
- Option 2: a second clock on the host (principle 8).
- Option 3: a scheduler Flume does not have, and its statement does not give it.
- Option 4: ruled out by `roles.7`.
- Option 5: for the reason above.

## Consequences

- Good: one clock on a host keeps Flume's watch and millpond's schedules. A host whose runner stops shows both lapsing together, never one silently.
- Good: the superfans' four-hourly passes and the tools' watches need no timer of the fleet's once the tick runs.
- Bad: a slow tick delays the pass's next turn by as long, as a slow settle does. An adapter that runs a superfan's session waits for that session's end.
  - millpond should let a tick start its due adapters and return without holding the pass. That is millpond's to say.
  - Until millpond says so, a pass's tick waits for each adapter. A superfan's adapter is due at most once each four hours.
- Bad: the runner gains a dependency on millpond. On a host whose configuration names no inbox, the tick is never run.
- Bad: a dead runner stops the tick, so the superfans and the watches stop with it. The runner superfan cannot report its own runner's death. A lapsed adapter is shown from millpond's status (millpond 26), and the hosted control plane sees a missing runner by the protocol (record 0013).
- Before millpond's tick runs, a `/loop` in one Claude session wakes the superfans every four hours. That is the user's ("could we use /loop in a claude session?"; crew record 0063, decision 10). It is no code, and the tick replaces it.
- The rule on acceptance: core.13 names the tick beside the settle. The running and configuration specs, and the runner's watch adapter, follow in the change that builds them, once millpond's tick exists.
