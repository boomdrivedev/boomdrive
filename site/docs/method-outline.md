# The Method: outline

Each principle is sketched the way its page would read: the claim, what it
rules out, how we do it today, where it is going, and what measures it. The
list is not settled; the proposed reshape is at the end.

## 0. The statement

**Claim:** Coding agents made writing code cheap; deciding and proving it is
now the bottleneck, and a fleet needs conventions the way web applications
needed Rails.

The page states the problem at 5 agents and at 50, gives the principles one
line each, and says what the Method is not: a framework, a vendor, or a
finished answer.

## 1. Every day the same

**Claim:** A check, a gate and an answer about the repository are identical
whether the work happens in an agent's worktree, on a laptop, in CI or in a
release.

- **Rules out:** CI-only steps; "works on my machine"; globally installed
  tools; a different test command for agents.
- **Today:** devenv declares the tools, moon runs every check, and the same
  hooks run locally and in CI.
- **Target:** the agent's check and the server's check share one key and one
  record.
- **Measure:** checks that exist only in CI.

## 2. One agent, one worktree

**Claim:** Each agent works in its own worktree with its own ports, emulator
and dev servers, so nothing it does can disturb another.

- **Rules out:** a shared checkout; a shared emulator; fixed ports; a server
  that outlives its worktree.
- **Today:** Worktrunk, hashed ports, an emulator per worktree, Portless names,
  servers tethered to the worktree.
- **Target:** worktree environments declared once and stood up from the
  declaration.
- **Measure:** collisions per week; time to a working fresh worktree.

## 3. Intents and bolts

**Claim:** Work is an intent (what someone wants), broken into bolts (a body
of work landed as one) made of units (one reviewable change each), and small
units land continuously on the trunk.

- **Rules out:** long-lived feature branches; a 3,000-line agent pull request;
  work with no stated intent.
- **Today:** Crew's bolts and units, as OpenSpec changes.
- **Target:** Flywheel Next tracks intents from capture to landed.
- **Measure:** unit size; unit cycle time; units per intent.

## 4. Two loops, one gate

**Claim:** Design (settling what to build) and construction (building it) are
separate loops, coupled at one gate where a human decides, and agents never
approve agents.

- **Rules out:** designing and building in one conversation; an agent
  approving its own work; approvals scattered through a session.
- **Today:** the operator approves each unit in Crew.
- **Target:** Flywheel's rail, with numbered decisions answerable from
  anywhere.
- **Measure:** approval wait; decisions per landed unit; leverage.

## 5. Never prove twice

**Claim:** A check that passed on exactly these inputs has passed, and gates
run only what is missing.

- **Rules out:** re-running everything after a merge "to be safe"; a result
  keyed to a commit rather than to inputs; a flaky check hidden by a retry.
- **Today:** moon's cache within each worktree; flakes retried by hand.
- **Target:** testledger shared across worktrees, CI and release.
- **Measure:** re-proof rate; check minutes avoided.

## 6. Know what a change touches

**Claim:** Every agent and gate can ask the repository what its projects and
tasks are, what a change affects and what a check reads, whichever monorepo
tool it uses.

- **Rules out:** an agent guessing which tests to run; hard-coded paths in CI;
  a check that reads undeclared files.
- **Today:** moon's project graph, plus a hand-kept list of which tests read
  which files.
- **Target:** pinout, as a CLI and MCP server.
- **Measure:** keyed coverage; accuracy of "affected".

## 7. Trust belongs to the reader

**Claim:** A pass is evidence and whoever reads it decides whether it counts:
an agent's pass can speed a merge, while only a confined run can make a
release.

- **Rules out:** one trust level for every gate; an agent's own claim that it
  is done; a policy that widens trust downstream.
- **Today:** the merge gate re-runs everything; nothing is trusted across
  machines.
- **Target:** gates with layered policy reading signed records.
- **Measure:** share of agent passes accepted; false-reuse audits.

## 8. Measure the fleet

**Claim:** A fleet that is not measured cannot be steered: count what lands,
what it cost in attention and money, and what came back, and publish the
definitions with the numbers.

- **Rules out:** judging by vibes or token counts; a number with no
  definition.
- **Today:** the collectors in `site-design.md`.
- **Target:** a quality board.
- **Measure:** `measurement-roadmap.md`.

## Where the list is off

- **Too much verification.** Principles 5, 6 and 7 are all the proof story.
  Fold 6 and 7 into 5: never proving twice needs knowing what a change touches
  and deciding whose pass counts.
- **Missing: attention is the scarce resource.** Design the fleet around the
  human's time: decisions batched and numbered, interruptions only where
  judgment is needed. It is the idea behind the rail and the decision inbox.
- **Missing: write down the destination.** Design records, the context map and
  settled decisions that agents read before building; how the design loop's
  output reaches construction.
- **Missing: rules agents can check.** A constitution of falsifiable rules
  with ids, so a reviewer, human or agent, can point at a diff and cite the
  rule it breaks.
- **"Every day the same" may be the mission, not a principle.** It can live on
  the statement page, with principle 1 becoming its concrete form: one
  declaration of tools and checks.

### Proposed v0.1

0. The statement (the mission: every day the same)
1. One declaration of tools and checks
2. One agent, one worktree
3. Intents and bolts
4. Two loops, one gate
5. Attention is the scarce resource
6. Write down the destination (with rules agents can check)
7. Never prove twice (with what a change touches, and whose pass counts)
8. Measure the fleet
