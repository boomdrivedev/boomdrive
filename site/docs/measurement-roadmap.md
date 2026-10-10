# Measurement roadmap

Candidate measurements, by the stage whose capability makes them measurable.
Start with fleet utilization, then the three marked ★, and add as each stage
arrives; each stage's arrival is a field note, and each measurement's first
chart is another.

The same seven dimensions recur in every stage, so the Fleet page grows
without being reorganized:

| Dimension | Asks |
|---|---|
| Flow | How does work move? |
| Attention | What does it cost the human? |
| Proof | How is it verified, and how often twice? |
| Quality | Is what lands any good? |
| Cost | What does it cost in tokens and money? |
| Health | Is the fleet itself working? |
| Design | How fast does intent become decisions? |

**Measured at every stage: leverage**, landed units per hour of operator
attention. It says whether more agents is working.

## Stage 0: now

Crew, Herdr, git and moon already record this.

- **Flow:** ★ units and bolts landed per week; ★ unit cycle time by stage
  (construct, awaiting approval, code, verify, merge); work in flight; peak
  and typical concurrent agents; agent-hours per landed unit.
- **Attention:** approval wait; approvals per landed unit; operator
  interventions per unit (`crew fix`, manual prompts).
- **Proof:** ★ sent back after "done" (code said done, verify disagreed);
  merge-gate refusals per merge; main red rate and time back to green.
- **Health:** ★ fleet utilization (agents running and how many of them are
  busy, with the host's CPU, memory and process count, over time); stuck
  agents (blocked or idle past a threshold); restarts per day.

## Stage 1: instrumented harnesses

Claude Code's OpenTelemetry export, and hooks that log tool calls.

- **Cost:** tokens and dollars per landed unit, and per stage; usage limits
  hit per week.
- **Health:** error loops (the same failing command repeated); context
  compactions per unit; tool-call failure rate.
- **Flow:** the same kind of unit compared across harnesses and models.

## Stage 2: a shared remote cache, and secrets through the shell

moon's remote cache shared by every machine, written by CI; secrets resolved
from a password manager.

- **Proof:** re-proof rate, exact; check minutes avoided; cache hits by source
  (own worktree, another worktree, remote); declared-input coverage (share of
  tasks whose inputs are declared and checked); false hits (a sample of cached
  passes re-run, disagreeing); flaky tasks.
- **Flow:** time to merge saved.
- **Health:** commands run outside the declared shell.
- **Secrets:** secrets resolved per day, by agent and task; secrets found in
  transcripts or commits (the target is none).
- **Watch:** proof cost per landed unit.

## Stage 3: Flywheel Next's design loop and rail

- **Design:** intent lead time (captured to landed); open questions closed per
  week; decisions per intent; decisions reversed later; spec churn after
  approval; units per intent.
- **Attention:** decision latency; decisions answered from a phone; decisions
  per operator per day (the load ceiling).

## Stage 4: several hosts and several people

- **Flow:** agents per operator; host utilization; scheduling wait;
  cross-repository bolts.
- **Attention:** decision load per person; time for a decision to reach its
  owner.
- **Health:** landings while nobody was watching.

## Stage 5: outcomes over months

- **Quality:** escaped defects (bugs traced to agent-landed units within 30
  days); revert rate; change failure rate; churn of agent-written code within
  30 days; review findings per unit.
- **Method:** the before-and-after effect of a principle change, linked to the
  Method's changelog.
- **Community:** an opt-in benchmark across readers' fleets.

## The first four, defined

| Measurement | Definition | Source | What it argues |
|---|---|---|---|
| Fleet utilization | Every 5 minutes on each fleet host, the agents alive and how many are busy (working, as against waiting on the operator or idle), with the host's CPU and memory in use and its process count; rolled up to the day as peak and typical | `herdr agent list`; the host's process table | Idle agents are waste, like idle cores: the case for keeping the fleet fed, and the count that marks the journey on Home |
| Unit cycle time, by stage | Time from adding a unit to merging it into its bolt, split into construct, awaiting approval, code, verify and merge | Crew's plan and stage changes; unit branch commit times | Waiting for approval likely dominates: the case for the decision inbox |
| Sent back after "done" | Share of units where code reported done and verify sent the work back | Crew's stage changes | How often agents overclaim: why gates read evidence |
| Re-proof rate | Share of check runs whose exact inputs had already passed elsewhere in the fleet | Now: moon's run reports across worktrees (task hashes run again rather than replayed). Later: the remote cache's hit records | The waste a shared cache removes; before and after is a strong note |

A measurement is one definition with one or more series, versioned
together because they come from the same sample: a stage's hours, or the
agents and the host's load read at the same instant.

```yaml
# content/measurements/fleet-utilization.yaml
id: fleet-utilization
name: Fleet utilization
definitionVersion: 1
definition: >
  Every 5 minutes on each fleet host: the agents alive, how many of them are
  busy (working, as against waiting on the operator or idle), and the host's
  CPU and memory in use and its process count; rolled up to the day as peak
  and typical.
principle: measure-the-fleet
collector: collectors/fleet-utilization
series:
  - { name: agents running, unit: agents, points: [] }
  - { name: agents busy, unit: agents, points: [] }
  - { name: cpu, unit: percent, points: [] }
  - { name: memory, unit: GB, points: [] }
  - { name: processes, unit: processes, points: [] }
```

```yaml
# content/measurements/unit-cycle-time.yaml
id: unit-cycle-time
name: Unit cycle time, by stage
definitionVersion: 1
definition: >
  For each unit merged into a bolt, the time from `crew unit add` to merge,
  split into construct, awaiting approval, code, verify and merge.
principle: intents-and-bolts
collector: collectors/unit-cycle-time
series:
  - { name: construct, unit: hours, points: [] }
  - { name: awaiting approval, unit: hours, points: [] }
  - { name: code, unit: hours, points: [] }
  - { name: verify, unit: hours, points: [] }
  - { name: merge, unit: hours, points: [] }
```

Only our own repositories are measured: flywheel-next, crew and
flywheel-web.
