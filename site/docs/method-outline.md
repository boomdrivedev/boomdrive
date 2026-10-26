# The Method: outline

Each principle is sketched the way its page reads: its line (the one-line
form Home and the statement list it by), the claim, what it rules out, how we
do it today, where it is going, what measures it, and the idea of its
diagram, agreed with the operator before it is drawn (`site-design.md`,
Every principle gets a diagram). The
merge queue clauses cited are the [sluice statement](https://github.com/agentplot/blueprints/blob/main/design/sluice/requirements.md); the site shows no clause number, and this outline keeps them. Version 0.1
has ten principles, read on one page.

## 0. The statement

**Line:** Writing code got cheap; deciding and proving it is the work now.

**Claim:** Coding agents made writing code cheap; deciding and proving it is
now the bottleneck, and a fleet needs conventions the way web applications
needed Rails.

The page opens on the case the Method answers: one person can keep only so
many agents in mind, so running a fleet is a people and process problem, and
the Method is the process that brings that person only the decisions. It carries
the mission, every day the same: a check, a gate and an answer about the
repository are identical whether the work happens in an agent's worktree, on
a laptop, in CI or in a release. It gives the principles one line each, each
principle's line, and says what the Method is not: a framework, a vendor, or a finished answer.

## 1. One shell

**Line:** Every command runs in the one shell the repository declares.

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
- **Diagram:** drawn, `public/diagrams/one-shell.svg`.

## 2. One agent, one worktree

**Line:** Each agent works alone in its own worktree, and submits rather than merges.

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
- **Diagram:** three worktrees side by side, each with its own agent, ports,
  emulator and dev server, working at their own pace. Each submits its head
  to the line; none merges. One agent reaches into another's server and is
  refused.

## 3. Capture at the source

**Line:** Work begins as captures, taken unattended where things happen.

**Claim:** Work begins as captures taken where things happen, a meeting, a
chat, a log, a pull request, by adapters that run unattended, so development
is fed by what people and systems say rather than by tickets someone
remembered to write.

- A capture is one source event with its provenance and a pointer to the raw
  material, which stays out of version control; capturing an event twice
  yields one capture (Flywheel Next statement, clause 111).
- Capture is one gesture, no more than a sentence (112). Adapters write
  captures unattended; reading a capture into signals is a session's
  judgment and never runs unattended (115, 215).
- Curation moves every signal onto an intent, a claim, a chore, or off with a
  reason; an unmoved signal is never discarded (116, 118).
- **Rules out:** work that starts from nobody's record of why; a transcript
  committed to the repository; asking the operator to classify what they
  wrote; an adapter that judges; a signal silently dropped.
- **Today:** findings recorded by hand with Crew's signal command; the first
  corpus of captures written by hand from meeting transcripts.
- **Target:** Flywheel Next's adapters (chat, meeting transcript, log or
  monitor webhook, pull-request conversation, issue tracker, folder drop),
  writing captures from whichever host declares the source (217g).
- **Measure:** captures per day by source; unmoved signals by age; time from
  capture to intent; landed units traced back to a capture.
- **Diagram:** four sources, a meeting, a chat, a log and a pull request, each
  with an adapter beside it. Events pulse at the sources and flow as captures
  into one inbox; the same event arriving twice becomes one capture. A reader
  session turns a capture into signals. A hand-written ticket form is struck
  through.

## 4. Intents and bolts

**Line:** Work is an intent, landed in bolts of small units.

**Claim:** Work is an intent (what someone wants), broken into bolts (each
one delivery) made of units (one reviewable change each). A unit merges into
its bolt as it finishes, and the bolt lands on the shared line as one step,
so the shared line moves only at a landing.

- **Rules out:** long-lived feature branches; a 3,000-line agent pull request;
  work with no stated intent.
- **Today:** Crew's bolts and units, as OpenSpec changes.
- **Target:** Flywheel Next tracks intents from capture to landed.
- **Measure:** unit size; unit cycle time; units per intent.
- **Diagram:** drawn, `public/diagrams/intents-and-bolts.svg`.

## 5. Two loops, one gate

**Line:** Design and construction are separate loops, and a person decides where they meet.

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
- **Diagram:** two rings turning: the design loop (captures, signals, intents,
  proposals) and the construction loop (units, merges, bolts, landings),
  touching where the operator's word is given, at the proposal. A second mark
  lights at a channel's promotion. Between the test and the landing a stop
  sign is crossed out: the landing goes on green.

## 6. Order at landing, never at start

**Line:** Work starts at once; the merge queue orders it as it lands.

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
- **Diagram:** drawn, `public/diagrams/order-at-landing.svg`, the site's copy
  of sluice's merge-queue diagram.

## 7. Never prove twice

**Line:** A check that passed on these inputs is not run again.

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
- **Diagram:** a cache in the middle. Four places run the same check: a
  worktree, a second worktree, CI and the merge queue. The first run takes its
  time and fills the cache; the next three replay in a flash. One input
  changes and only that check runs again. Beside the cache, the queue's own
  record keeps a fail the cache never remembers.

## 8. Secrets by declaration

**Line:** An agent gets a secret only through the shell, for the task at hand.

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
- **Diagram:** a repository's declaration lists the secrets it needs by name.
  An agent's command runs inside the shell; for that command alone a key
  appears, resolved from the password manager, and vanishes when the command
  ends. Nothing reaches a file, the wider environment or the transcript. An
  agent pasting a token into a file is refused.

## 9. Measure the fleet

**Line:** Count what lands, what it cost and what came back.

**Claim:** A fleet that is not measured cannot be steered: count what lands,
what it cost in attention and money, and what came back, and publish the
definitions with the numbers.

- **Rules out:** judging by vibes or token counts; a number with no
  definition.
- **Today:** collectors over Crew's records, git and moon's run reports, on
  our own repositories.
- **Target:** a quality board.
- **Measure:** fleet utilization first, then `measurement-roadmap.md`.
- **Diagram:** collectors read three sources nightly, the plan's record of
  merges and landings, git and the task runner's run reports, into points. A
  board grows three series day by day: what landed, what it cost in attention
  (decisions answered) and in money (tokens by role). Where the stack changed,
  the axis is marked with the fleet context, and each definition sits beside
  its numbers.

## The tools each principle names

Each principle's place on the Method shows "In the tools", the tools that do
what it says, from the `tools` of its record; this table holds the same names
and addresses, with the clauses behind each, which the site does not show. A
link lands on the section of the tool's product page that does it, or of its
design book where the page has none. Crew and Flywheel Next have neither in
public, so they are named without a link. Each address was checked live on
2026-10-25.

| Principle | Tools | Link | What it lands on | Clauses behind it |
|---|---|---|---|---|
| One shell | sluice | <https://agentplot.github.io/sluice/reference/commands/#sluice-exec> | `sluice exec`: a check runs in the place's own shell, the one the repository declares | — |
| One agent, one worktree | sluice<br>flume | <https://agentplot.github.io/sluice/decisions/0016/><br><https://agentplot.github.io/flume/> | sluice gives a place its ports on the host and releases what its checks hand off<br>one agent launched in one worktree | sluice 1, 2, 30, 31, I5; record 0016<br>— |
| Capture at the source | millpond<br>headwater<br>eddy | <https://agentplot.github.io/millpond/><br><https://agentplot.github.io/headwater/><br><https://agentplot.github.io/eddy/> | the inbox records each event once<br>an adapter that writes captures once<br>an adapter that writes captures once | Flywheel Next 111, 112, 115, 116, 118, 215, 217g |
| Intents and bolts | Crew<br>Flywheel Next | no link<br>no link | bolts and units<br>a bolt lands on the shared line as one | — |
| Two loops, one gate | tailrace<br>sluice | <https://agentplot.github.io/tailrace/how-it-works/#channels-moved-only-by-proof><br><https://agentplot.github.io/sluice/#why> | the word is a promotion<br>landing is automatic on green | tailrace 97, C.16<br>sluice 4 |
| Order at landing, never at start | sluice | <https://agentplot.github.io/sluice/#why> | submit, combine, blame one, land | sluice 3, 6, 7, 8, 10 to 12, 17, 18, 20 |
| Never prove twice | tailrace<br>sluice | <https://agentplot.github.io/tailrace/#one-store-three-jobs><br><https://agentplot.github.io/sluice/reference/state/#the-log> | the build cache in the one store<br>every run logged, a failure exactly like a pass | tailrace 52 to 55, 100, C.11<br>sluice 15, 16, 19, 104, 108; core.7 |
| Secrets by declaration | sluice | <https://agentplot.github.io/sluice/reference/configuration/#executor-what-the-tests-may-touch> | a run gets the host's shared caches and nothing else outside its place | sluice 35 |
| Measure the fleet | Crew<br>sounding | no link<br><https://agentplot.github.io/sounding/> | the records the collectors read<br>the records the collectors read | — |

## Changelog

Version 0.1 is a draft; a revision within it is an entry here, in the shape
the site's change entries take (principle, kind, summary, what prompted it).

- **2026-10-12 · intents-and-bolts · revised.** Units merge into their bolt
  as they finish and the bolt lands on the shared line as one; the claim had
  said units land continuously on the trunk. Prompted by the operator reading
  the principle's diagram: "bolts land on main, not units themselves ... the
  whole point of the bolt is a delivery mechanism", which is how Crew works
  and what the Flywheel Next statement's glossary says of a work item (merged
  into the bolt when complete) and of landing (a bolt's line onto the shared
  line).

- **2026-10-22 · the Method · revised.** Each principle without a diagram
  (one agent one worktree, capture at the source, two loops one gate, never
  prove twice, secrets by declaration, measure the fleet) has its diagram's
  idea written here, agreed with the operator on 2026-10-22 before it is
  drawn, as `site-design.md` asks.

- **2026-10-26 · the Method · revised.** Each principle names the tools that
  do what it says, in a strip on its place that links each tool's own page;
  the principles' text drops the statements' clause numbers, which this
  outline keeps. Prompted by the operator on 2026-10-26: "cross-reference
  these sections to the tools pages as they're being worked on".

## Candidates for later versions

- **Attention is the scarce resource.** Design the fleet around the human's
  time: decisions batched and numbered, interruptions only where judgment is
  needed. It is the idea behind the rail.
- **Write down the destination.** Design records, the context map and settled
  decisions that agents read before building, with rules agents can check: a
  constitution of falsifiable rules with ids a reviewer can cite against a
  diff.
