# Measurement roadmap

Candidate measurements, by the stage whose capability makes them measurable.
Start with fleet utilization, work in flight and landed, which the Fleet's
front page reads together, then the three marked ★, and add as each stage
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

- **Flow:** ★ landed (units merged into their bolt and bolts landed on
  main, by day, by flywheel); ★ work in flight (bolts open, units by stage,
  repositories with work, by flywheel); ★ unit cycle time by stage
  (construct, awaiting approval, code, verify, merge); peak and typical
  concurrent agents; agent-hours per landed unit.
- **Attention:** approval wait; approvals per landed unit; operator
  interventions per unit (`crew fix`, manual prompts).
- **Proof:** ★ sent back after "done" (code said done, verify disagreed);
  merge-gate refusals per merge; main red rate and time back to green.
- **Health:** ★ fleet utilization (agents running and how many of them are
  busy, split by role, with each machine's CPU, memory and process count,
  over time); stuck agents (blocked or idle past a threshold); restarts per
  day.
- **Cost:** ★ tokens by role (tokens by kind and list-price dollars, by
  role and by model, per day, from Crew's session registry joined to the
  transcripts' per-message counts; cache reads weighted at their price).

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

## The first measurements, defined

| Measurement | Definition | Source | What it argues |
|---|---|---|---|
| Fleet utilization | Every 5 minutes on each fleet host, the agents alive and how many are busy (working, as against waiting on the operator or idle), with the host's CPU and memory in use and its process count, one series per machine. Each day's value is the median of the day's samples, the day being the host's calendar day with every sample counted; the agent counts also publish the day's peak. The busy share is busy divided by running at a point | `herdr agent list`; the host's process table | Idle agents are waste, like idle cores: the case for keeping the fleet fed, and the peak that marks the journey on Home |
| Work in flight | Every 5 minutes, from each flywheel's plan: bolts open, units in flight by stage (construct, awaiting approval, code, verify, merge), and repositories with work in flight; each day's value the median of the day's samples, per flywheel and in total | Crew's plan (`crew bolts`) | Where open work sits: a pile at awaiting approval is the attention bottleneck |
| Landed | Each day, from each flywheel's run record: units merged into their bolt (work finished) and bolts landed on main (work delivered), counted, per flywheel and in total; the headline shows the week's total | Crew's run record (`crew events`) | What the fleet delivers, read against how busy it was; small and lumpy at first, which the week's total absorbs |
| Tokens by role | Each day, every session Crew started, joined by its session id to its transcript: tokens by kind (input, output, cache written, cache read) and list-price dollars, by role (conductor, team ops, unit agent, design, planner, dispatcher, operator) and by model, per flywheel and in total. Dollars come from a dated rate table the site keeps (base input, 5-minute and 1-hour cache writes, cache reads, output, per model), at the rates in force on the point's date; a price change dates the table, never the definition. A transcript that does not split cache writes by lifetime is priced at the 5-minute rate, and the page says so | `crew usage --json` over Crew's session registry and the transcripts' per-message usage | What the fleet costs and where: whether the spend sits with the agents that build or the ones that manage, and which model earns its price |
| Unit cycle time, by stage | Time from adding a unit to merging it into its bolt, split into construct, awaiting approval, code, verify and merge | Crew's plan and stage changes; unit branch commit times | Waiting for approval likely dominates: the case for the decision inbox |
| Sent back after "done" | Share of units where code reported done and verify sent the work back | Crew's stage changes | How often agents overclaim: why gates read evidence |
| Re-proof rate | Share of check runs whose exact inputs had already passed elsewhere in the fleet | Now: moon's run reports across worktrees (task hashes run again rather than replayed). Later: the remote cache's hit records | The waste a shared cache removes; before and after is a strong note |

A measurement is one definition with one or more series, versioned
together because they come from the same sample: a stage's hours, or the
agents and the host's load read at the same instant. A point is one value at
one date. A series says which statistic of the day's samples its value is,
the median unless it says otherwise, and a series that is the day's peak of
another names it with `peakOf`, so the page draws the pair as one band and
one tile. Only a quantity whose definition needs a peak carries one: the
agent counts do, because the journey on Home reads the peak of agents
running; the host's load does not.

```yaml
# content/measurements/fleet-utilization.yaml
id: fleet-utilization
name: Fleet utilization
definitionVersion: 1
definition: >
  Every 5 minutes on each fleet host: the agents alive, and how many of them
  are busy (working, as against waiting on the operator or idle). Beside
  them, the host's CPU and memory in use and its process count. Each day's
  value is the median of the day's samples, every sample counted; the agent
  counts also carry the day's peak. The busy share is busy divided by
  running at a point.
principle: measure-the-fleet
collector: collectors/fleet-utilization
series:
  - { name: agents running, unit: agents, statistic: median, points: [] }
  - { name: agents running, peak, unit: agents, statistic: max, peakOf: agents running, points: [] }
  - { name: agents busy, unit: agents, statistic: median, points: [] }
  - { name: agents busy, peak, unit: agents, statistic: max, peakOf: agents busy, points: [] }
  - { name: cpu, unit: percent, statistic: median, host: mac-studio, points: [] }
  - { name: memory, unit: GB, statistic: median, host: mac-studio, points: [] }
  - { name: processes, unit: processes, statistic: median, host: mac-studio, points: [] }
```

```yaml
# content/measurements/work-in-flight.yaml
id: work-in-flight
name: Work in flight
definitionVersion: 1
definition: >
  Every 5 minutes, from each flywheel's plan: the bolts open, the units in
  flight by stage, and the repositories with work in flight. Each day's
  value is the median of the day's samples.
principle: intents-and-bolts
collector: collectors/work-in-flight
series:
  - { name: bolts open, unit: bolts, statistic: median, points: [] }
  - { name: units in construct, unit: units, statistic: median, stage: construct, points: [] }
  - { name: units awaiting approval, unit: units, statistic: median, stage: awaiting approval, points: [] }
  - { name: units in code, unit: units, statistic: median, stage: code, points: [] }
  - { name: units in verify, unit: units, statistic: median, stage: verify, points: [] }
  - { name: units in merge, unit: units, statistic: median, stage: merge, points: [] }
  - { name: repositories with work, unit: repositories, statistic: median, points: [] }
```

```yaml
# content/measurements/landed.yaml
id: landed
name: Landed
definitionVersion: 1
definition: >
  Each day, from each flywheel's run record: the units merged into their
  bolt, and the bolts landed on main. Counted, not sampled.
principle: intents-and-bolts
collector: collectors/landed
series:
  - { name: units merged, unit: units, statistic: count, points: [] }
  - { name: bolts landed, unit: bolts, statistic: count, points: [] }
```

A series split by flywheel repeats with `flywheel: <name>` beside the
total; the page stacks the split and headlines the total. Only our own
flywheels appear.

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
