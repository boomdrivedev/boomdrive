---
number: 27
title: The runner carries each place's status against Flume's listing as it stands once the pass's commands have run
status: accepted
date: 2026-11-07
decision-makers:
- agentplot-design
---

# The runner carries each place's status against Flume's listing as it stands once the pass's commands have run

Status: accepted on 2026-11-08 by agentplot-design. The direction is the chore `the-runner-reads-a-tidied-places-record`'s brief, quoted below, and when the listing is read again is its code stage's choice, made on this repository's rules and Flume's statement. Neither is a choice of the user's alone, and both are cheap to change, so the design agent settles them.

Deciders: the chore's brief, for the direction; the code stage, for how it is met.
Sources: the chore's brief, 2026-11-07, and the design agent's signal behind it; a host's runner report and Flume's listing on 2026-11-07; Flume's statement, clauses 43 and 62, and its `keep_for_ends` and `status`; records 0002, 0021 and 0026; rules core.1, core.9 and core.11; the work-into-flume spec.

## Context and problem statement

The brief: "Boomdrive's runner reads a place Flume has tidied from Flume's record of it at `<location>@<at>`, as work-into-flume's spec says, so a unit, chore or fix that landed and was then tidied is carried as landed, never at the stage it last showed."

The runner already reads a tidied place's record as the place's entry: the one standing at its location, else `<location>@<at>`, the latest where Flume keeps several. What kept two chores at the stage they last showed was when it looked. A control pass reads Flume's listing once, before its pull, then answers the pull's wants, then carries each place's status, asking Flume again only for a place whose entry in that listing differs from the one carried. On 2026-11-07 one pass began at about 14:01Z and ran its launches and acts until 14:40Z, each act about 30 seconds (record 0026). Flume tidied two of its chores at 14:07Z and 14:09Z, landed, keeping each one's record for an end pending for its planner. At 14:40Z the pass compared both against the listing of 14:01Z, in which both stood live, just as carried, and carried them on as live.

Flume keeps a tidied place's record only while an end of it is pending for a recipient, and removes it once the recipient takes its ends (Flume's 43, 62). A landing the runner has not read by then is gone: Flume refuses `status <place>`, and the last status carried stands, not listed. Two fixes of the same day that landed at 12:56Z still read as the status carried at 12:51Z; by the time they were looked at, Flume held nothing of them, so which of these lost them cannot be shown.

## Considered options

1. **Read Flume's listing again before the statuses are carried, where answering the wants ran a tool's command**: a launch, an act of Flume's or a branch of sluice's.
2. **Read it again at every pass**, quiet ones among them.
3. **Ask each carried place's status at every pass**, with no listing to compare against.
4. **Carry the statuses before answering the wants.**
5. **Keep the pass's one listing**, and rely on passes being short once each partition runs on its own and launches go first (record 0026).

## Decision outcome

Option 1.

- **The statuses are carried against Flume's listing as it stands once the wants are answered.** Where answering them ran a tool's command, the listing is read again then. Where it ran none, the pass's listing is seconds old and stands.
- **The hello and the room are the pass's own listing's**, read before the pull, as now. The room is counted down as the pass launches (core.9).
- **Nothing else changes.** A place's entry is the one standing at its location, else its tidied record. Its status is asked again only where that entry differs from the one carried, and carried as Flume printed it (core.1, core.11).

Option 2 doubles Flume's whole-host status on every quiet pass, which reads each landing place's submission from sluice. Option 3 runs one `flume status` for every place at every pass. Option 4 carries a launch or an act of this pass only at the next one. Option 5 leaves a pass that runs a launch, whose readiness can take minutes, comparing against what Flume showed before it.

### Consequences

- Good: a place that lands and is tidied while a pass runs its commands is carried that pass from its record, tidied and landed.
- Good: a place launched in a pass is carried in that pass as listed.
- Bad: a pass that runs a command reads Flume's listing twice.
- Not cured: a landing whose record Flume removes between two of the runner's reads, because its recipient took the place's ends first, is lost to the runner. Only Flume holds it, so that is Flume's to keep for the place's launcher, or the control plane's to read from sluice's entry of the submission, which the runner carries too.
- The work-into-flume spec says it, in this record's change.
