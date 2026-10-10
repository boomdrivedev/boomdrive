# The Method: outline

Each principle is sketched the way its page reads: the claim, what it rules
out, how we do it today, where it is going, and what measures it. The
merge queue clauses cited are the [sluice statement](https://github.com/agentplot/blueprints/blob/main/design/sluice/requirements.md). Version 0.1
has nine pages.

## 0. The statement

**Claim:** Coding agents made writing code cheap; deciding and proving it is
now the bottleneck, and a fleet needs conventions the way web applications
needed Rails.

The page opens on the journey the Method is for: scaling a fleet from one
agent to a thousand, 1, 5, 20, 50, 100, 500, 1,000, where each step is a
change in how you think and work before it is a change in tools. It carries
the mission, every day the same: a check, a gate and an answer about the
repository are identical whether the work happens in an agent's worktree, on
a laptop, in CI or in a release. It gives the principles one line each, and
says what the Method is not: a framework, a vendor, or a finished answer.

## 1. One shell

**Claim:** Every command, an agent's, a person's or CI's, runs inside one
shell the repository declares, so the tools and their versions are the same
everywhere.

- **The shell** is whatever the repository declares: devenv here; a container
  image, mise or another tool runner elsewhere. The Method names the idea, not
  the product.
- **Rules out:** a tool installed globally; a CI job that installs its own
  tools; an agent running `npx tool@latest`; a script that assumes a tool is on
  the machine.
- **Today:** devenv declares every tool; hooks, CI and agents enter the same
  shell.
- **Target:** the shell is part of a repository's manifest, so any platform
  that runs the repository knows how to enter it.
- **Measure:** commands run outside the shell; tools declared in more than one
  place.

## 2. One agent, one worktree

**Claim:** Each agent works in its own worktree with its own ports, emulator
and dev servers, and submits its work to a merge queue rather than merging
it, so nothing it does can disturb another.

- **Rules out:** a shared checkout; a shared emulator; fixed ports; a server
  that outlives its worktree.
- **Today:** Worktrunk, hashed ports, an emulator per worktree, Portless names,
  servers tethered to the worktree.
- **Target:** worktree environments declared once and stood up from the
  declaration. A session submits its head to a line and does nothing more;
  the merge queue performs every merge and landing (sluice statement,
  clauses 1 and 2).
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
separate loops, coupled where a human decides, and agents never approve
agents; the operator's word comes before the work is tested, never between
test and landing.

- **Rules out:** designing and building in one conversation; an agent
  approving its own work; approvals scattered through a session.
- **Today:** the operator approves each unit in Crew.
- **Target:** Flywheel's rail, with numbered decisions answerable from
  anywhere. The word sits at the proposal and at a channel's promotion;
  closing a bolt submits it to the shared line, landing is automatic on
  green, and a fix after a failed landing needs no new word (sluice
  statement, clause 4).
- **Measure:** approval wait; decisions per landed unit; leverage.

## 5. Order at landing, never at start

**Claim:** Dependencies are constraints the merge queue honours when work
lands, not gates that keep a session from starting; conflicts are found
cheaply when work is combined, not planned around.

- A unit starts at once, off its dependency's branch when it has one, and
  lands after it (sluice statement, clauses 3, 6 and 7).
- Each candidate is the line plus every entry ahead of it, tested as it would
  land; a conflict is found when the candidate is formed, before any check
  runs (clauses 10 to 12); a failure has one culprit, named once everything
  ahead has a verdict (clauses 17 and 18).
- **Rules out:** holding a unit until its dependency merges; a planner
  sequencing work to avoid conflicts; testing a change against a line it
  will not land on.
- **Today:** Crew starts a unit only after the units it depends on have
  merged into the bolt.
- **Target:** sluice, one queue design with two instances: the bolt line's
  trusts the sessions' own runs and the local cache, the shared line's trusts
  only the confined executor.
- **Measure:** time a unit waits to start; conflicts and culprits per landing;
  landings per hour per line.

## 6. Never prove twice

**Claim:** The task runner's cache, keyed by what each task reads and shared
across every worktree, agent and CI run, means a check that passed on these
inputs is not run again.

- **Knowing what a change touches** is the task runner's project graph and its
  declared inputs: moon here. A task that reads an undeclared file makes the
  cache lie, so inputs are declared and checked.
- **Whose results count:** every machine reads the remote cache; only the
  confined executor writes it, because it alone holds the write credential.
  An agent's local cache speeds its own worktree and the bolt line's queue;
  it never decides the shared line.
- **Rules out:** re-running every suite after a merge "to be safe"; a task with
  undeclared inputs; an agent writing to the shared remote cache; a flaky
  check hidden by a retry.
- **Today:** moon's cache, with archives in the git common directory so
  worktrees of one repository share them.
- **Target:** a remote cache every machine reads and only the confined
  executor writes; a pass proven in a worktree replays in the queue's
  candidate, and the queue records every run beside the cache, a fail as
  faithfully as a pass (sluice statement, clauses 15, 16 and 19).
- **Measure:** re-proof rate; cache hit rate by source (own worktree, another
  worktree, remote).

## 7. Secrets by declaration

**Claim:** A repository declares the secrets it needs, and an agent gets them
only through the shell, resolved from a password manager for the task at
hand, never pasted, stored or printed.

- **Why it matters:** most development needs few secrets, but operations work,
  connecting to running services, needs real credentials, and an agent is the
  last place they should leak.
- **Rules out:** a credential in a prompt, a file in the repository, a shell's
  history or an agent's transcript; one long-lived token shared by every
  agent; a secret an agent can read but no person can revoke.
- **Today:** secretspec declares what each repository needs.
- **Target:** secrets resolved from a password manager (1Password or similar)
  at run time, scoped to the task, with each access attributable and
  revocable.
- **Measure:** secrets resolved per day by agent and task; secrets found in
  transcripts or commits (the target is none).

## 8. Measure the fleet

**Claim:** A fleet that is not measured cannot be steered: count what lands,
what it cost in attention and money, and what came back, and publish the
definitions with the numbers.

- **Rules out:** judging by vibes or token counts; a number with no
  definition.
- **Today:** collectors over Crew's records, git and moon's run reports, on
  our own repositories.
- **Target:** a quality board.
- **Measure:** fleet utilization first, then `measurement-roadmap.md`.

## Candidates for later versions

- **Attention is the scarce resource.** Design the fleet around the human's
  time: decisions batched and numbered, interruptions only where judgment is
  needed. It is the idea behind the rail.
- **Write down the destination.** Design records, the context map and settled
  decisions that agents read before building, with rules agents can check: a
  constitution of falsifiable rules with ids a reviewer can cite against a
  diff.
