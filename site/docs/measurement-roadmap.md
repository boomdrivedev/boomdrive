# Measurement roadmap

Candidate measurements, by the stage whose capability makes them measurable.
Start with the three marked ★ and add as each stage arrives; each stage's
arrival is a field note, and each measurement's first chart is another.

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
- **Health:** stuck agents (blocked or idle past a threshold); restarts per
  day.

## Stage 1: instrumented harnesses

Claude Code's OpenTelemetry export, and hooks that log tool calls.

- **Cost:** tokens and dollars per landed unit, and per stage; usage limits
  hit per week.
- **Health:** error loops (the same failing command repeated); context
  compactions per unit; tool-call failure rate.
- **Flow:** the same kind of unit compared across harnesses and models.

## Stage 2: pinout and testledger released

- **Proof:** re-proof rate, exact; check minutes avoided; keyed coverage
  (share of checks with stated inputs); false reuse (audit reruns that
  disagree); flaky keys; agent passes accepted at the merge gate.
- **Flow:** time to merge saved.
- **Quality:** accuracy of "affected" (what pinout said a change affects,
  against what actually changed).
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

## The first three, defined

| Measurement | Definition | Source | What it argues |
|---|---|---|---|
| Unit cycle time, by stage | Time from adding a unit to merging it into its bolt, split into construct, awaiting approval, code, verify and merge | Crew's plan and stage changes; unit branch commit times | Waiting for approval likely dominates: the case for the decision inbox |
| Sent back after "done" | Share of units where code reported done and verify sent the work back | Crew's stage changes | How often agents overclaim: why gates read evidence |
| Re-proof rate | Share of check runs whose exact inputs had already passed elsewhere in the fleet | Now: moon's run logs across worktrees (approximate). Later: testledger keys (exact) | The waste testledger removes; before and after is a strong note |

```yaml
# content/measurements/unit-cycle-time.yaml
id: unit-cycle-time
name: Unit cycle time, by stage
definitionVersion: 1
definition: >
  For each unit merged into a bolt, the time from `crew unit add` to merge,
  split into construct, awaiting approval, code, verify and merge.
unit: hours
principle: intents-and-bolts
collector: collectors/unit-cycle-time
points: []
```

Only our own repositories are measured: pinout, testledger, flywheel-next and
crew.
