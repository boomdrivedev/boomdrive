# Design

## Context

See proposal.md for why. There are no area pages, so `docs/architecture/core.md` binds every path this change touches. Only crew's implementation, `crates/boomdrive-crew`, changes. The runner and the hosted implementation do not.

What stands, in `crates/boomdrive-crew`:

- **`read::Main`** holds `requests.rec`'s requests at the tip, in `Request` order, and the plan's `Unit` and `Bolt` names. `Main::plans(work)` says whether a request's work is still in the plan.
- **`Crew::pull_partition`** (`src/lib.rs`) walks the requests:
  - It skips each one `Pulling::current` refuses. A request is current where its work is in the plan, the runner's report answers it, or this host's branch answers it, claims apart. That filter is built and was never written in the spec.
  - It gives a request naming this host at once.
  - It puts each launch naming no host that the runner keeps in `kits`, which it claims, checks and gives as Decision 18 of `the-runner-carries-crews-work-into-flume` has it. A launch the report or the branch answers goes straight into `alone`.
  - It gives an act naming no host where the report, the branch or `own_claims` holds its launch.
- **`Crew::report`** writes the branch from the report, minus `write::let_go`. It keeps each claim of this host's whose launch the report does not answer and whose work is in the plan.
- **`write::let_go`** lets go a launch whose answer is not `launched`, or whose status is no longer listed, once its work has left the plan. It lets go an act once its launch is gone and its own work has left the plan.
- **Nothing reads a drop.** A drop is given only as an act, by the rules above. A launch it names is given and claimed as if no drop stood.

crew, at its `plugin/lib/runner.py` at `130ce42b`, reads a launch a drop names as no longer standing: `standing` gives the launches no drop request names, and `launch_of` the newest launch of a work no drop names. A drop request is `Act: drop` with `Of` the launch's number. crew's `reach` writes it, with the launch's own `Host`, in the commit that takes the work out of the plan. So a drop's work has always left the plan by the time a pull reads it, unless the same commit adds the work again under its name.

On the runner's side, which this change leaves as it stands:

- the runner launches only an offer the pull gives (core.9);
- it answers a drop of a launch it holds unlaunched from its report, `done`, and never makes that launch;
- it carries out a drop of a launched place as `flume drop <place>`;
- it drops from its report whatever the report's answer lets go;
- it carries a place's status only for a launch the pull gives (record 0021).

The 2026-11-07 case, from the brief's sources:

- request 425, the drop of launch 398, was written at 20:34:58Z, in the commit that took 398's unit out of the plan;
- the host's runner had claimed 398 at 19:40:48Z, at a pull before that commit;
- it launched 398 at 20:50:03Z, from that same pull, in a pass of some 90 wants carried out one after another, as runners did before core.50;
- request 425's work had left the plan and the runner had not answered it, so no later pull gave it, and 398's place went on working.

## Goals / Non-Goals

**Goals:**

- No pull whose tip holds a drop offers or claims the launch it names, unless the runner has already launched it.
- A dropped launch the runner holds unlaunched leaves its report.
- A drop reaches the runner that holds its launch, so a place launched before the drop arrived is dropped by Flume's command.

**Non-Goals:**

- **A launch taken from a pull made before its drop.** crew's implementation cannot know a drop its tip does not hold. Since core.50, a pass takes its pull's launches first, so that gap is the pass's first round.
- **The runner's rules.** It already answers a drop of a launch it holds unlaunched, and carries out one on a place.
- **Acts other than a drop of work gone.** Decision 4 says why.
- **The hosted control plane.** Its board drops a launch itself and lets it go by the interface's answer, which this change does not touch.

## Decisions

### 1. A launch is dropped as crew reads it

`Main` gains the set of launch numbers a drop names, `dropped`, read when `Store::main` reads `requests.rec`: each request with `Act: drop` and a whole `Of`. `Main::dropped(n)` asks it. A drop counts whatever its `Host`, its `Work` or its place in the file, as crew's `standing` counts it. crew owns the rule, and this implementation reads it from crew's own records (core.1, "Not ruled out"; core.2).

Alternatives:

- **A drop counted only where it names this host, or none.** Not taken: crew's `standing` counts every drop, and crew writes a drop with its launch's own `Host`, so the two never differ in crew's own writes.
- **A drop counted only after its launch in the file.** Not taken, for the same reason: crew's reading turns on no order.

### 2. A dropped launch the runner has not launched is given nothing and claimed by no one

`Pulling` gains `launched(n)`:

- where the runner's report holds an answer to `n`, the report answers it `launched`;
- where it holds none, this host's branch answers `n` with `Outcome: launched`.

A claim is never launched. In `pull_partition`'s walk, a launch request that `main.dropped` holds and that is not `launched` is skipped, before the host match, whatever host it names and whatever the plan holds. So it is never given, and a launch naming no host never reaches `kits`, so it is never claimed or withdrawn. Nothing is said: a drop is crew's ordinary work.

A claim of it already on this host's branch is not given. It leaves at the report's write: `Crew::report` keeps a claim only where its launch is not dropped, beside its two tests that stand. `own_claims`, which lets an act naming no host follow a claim given at the same pull, leaves out a dropped launch, so no act goes with a launch that is not given. That covers a retry or priority as well as the drop itself.

A dropped launch the runner did launch is given as before. Its place is then followed to its end, and its drop is carried out on that place (Decision 4).

Alternatives:

- **No offer for a dropped launch at all, launched or not, as the brief's first clause reads alone.** Not taken: the runner carries a place's status only for a launch the pull gives (record 0021). A launched one's place would never be seen over, so it would never be let go, and its last status would stand on the branch.
- **The claim withdrawn at the pull, by a write of its own.** Not taken: the report's write at the end of the same pass removes it. A pull that writes for a drop would push once more for nothing a runner reads, since a claim of a dropped launch holds back no other runner from work that stands.
- **The drop given ahead of its launch in the pull's wants**, so the runner's own rule answers it, as in work-into-flume's "A drop ahead of a held launch". Not taken: crew's order is its requests' order. A launch naming no host would still be claimed. A dropped launch no runner ever held would still be offered, to be answered `done` from a report that holds nothing of it.

### 3. A dropped launch the runner holds unlaunched is let go

In `write::let_go`'s first loop, a launch the report holds that `main.dropped` holds and whose answer is not `launched` goes into `gone` and `launches`, whatever the plan holds. So its acts follow it as the second loop has them: once the launch is gone and the act's own work has left the plan. A dropped launch answered `launched` keeps the rule every launch has: let go once its place is no longer listed and its work has left the plan.

The runner drops what is let go from its report and acts on it no more (control-plane-interface, "A report's answer names what is let go and what is refused"). Nothing of a place is removed, since an unlaunched launch has none (core.10).

Alternatives:

- **Left to the plan's test alone.** Not taken: a dropped launch's work may still be in the plan, as when crew drops a unit and adds it again under its name in one commit. A held launch there would be kept, and given while its plan held it.

### 4. A drop reaches the runner that holds its launch, whatever the plan holds

`Pulling::current` also passes a request whose `Act` is `drop` where the runner's report or this host's branch answers its `Of`, a claim apart, by `reported(of) || branch_answers(of, false)`. The host rules then give it:

- a drop naming this host goes at once;
- one naming another host goes nowhere;
- one naming none goes where the runner holds its launch, by the act rule, its `own_claims` without dropped launches (Decision 2).

The runner carries it out as it stands: `flume drop <place>` on a launched place, or `done` from its report for a launch it holds unlaunched.

This is the part of the change the signal needs most. The pull that offered launch 398 was made before its drop, so no reading at the pull could have held 398 back. What kept its place working is that request 425 was never given. The brief's goal, that no runner launches a dropped launch, holds only once a launch a runner did make from such a pull is dropped at the next pull.

Alternatives:

- **Every act on a launch the runner holds, whatever the plan holds.** Not taken: crew writes no act but a drop for work that has left the plan. Its `reach` drops such work. Its `answer` and `reprioritise` name only launches that stand. So widening the rule would change nothing crew writes today, and would give an old act left unanswered a way back.
- **The drop left to the plan's test**, as built. Not taken: crew writes a drop as its work leaves the plan, so the drop would never be given.

### 5. The plan's filter is written in the spec

The crew-control-plane spec gains "A request whose work has left crew's plan is given only while the runner holds it". It holds `Pulling::current` as built, and its drop exception as "A drop is given to the runner that holds its launch whatever the plan holds". The claims, "given whatever the hello names" and the umbrella "Each request for this host…" requirements name the drop's exception, so no requirement gives what another withholds.

### 6. The tests

Every test runs over a bare state repository in a temporary folder, through `tests/common`, with neutral names: partition `madswan`, hosts `deneb` and `vega`, units `u` and `a` (core.41). They go in a new file, `crates/boomdrive-crew/tests/drops.rs`, one test per scenario:

| Test | The spec's scenario |
|---|---|
| `a_drop_naming_another_host` | "A drop naming another host" |
| `a_held_launch_dropped` | "A held launch dropped" |
| `a_kits_launch_dropped_before_it_is_claimed` | "A kit's launch dropped before it is claimed" |
| `a_dropped_launch_the_runner_launched` | "A dropped launch the runner launched" |
| `a_claimed_launch_dropped` | "A claimed launch dropped" |
| `work_gone_that_the_runner_does_not_hold` | "Work gone that the runner does not hold" |
| `a_drop_written_as_its_work_leaves_the_plan` | "A drop written as its work leaves the plan" |
| `a_kits_drop_goes_to_the_runner_holding_its_launch` | "A kit's drop goes to the runner holding its launch" |
| `a_held_launch_dropped_is_let_go` | "A held launch dropped is let go" |
| `a_drop_answered_from_the_report_its_work_gone` | "A drop answered from the report, its work gone" |
| `a_dropped_launch_the_runner_launched_stays_until_its_place_is_over` | "A dropped launch the runner launched stays until its place is over" |

The MODIFIED requirements' other scenarios are proven by `tests/claims.rs` as they stand: `claimed_alone`, `another_claim_seen`, `taken_elsewhere`, `a_claim_waiting_for_its_streams_room`, `a_claim_found_again_after_a_stop`, `a_launch_answered_before_its_repository_left_the_hello` and `a_launch_the_report_answers_is_given_whatever_the_hello_names`. "Only this host's requests" is proven by `tests/read.rs`'s `only_this_hosts_requests`.

### Rules relied on

- **core.1**: crew's implementation gives what it reads from crew's own rule, in crew's own records. Its details' "Not ruled out" names such a reading. The runner decides nothing here.
- **core.2**: only crew's crate reads crew's requests, and the interface carries nothing new.
- **core.9**: the runner takes a launch only under an offer, so a launch given no offer is never made.
- **core.10**: a let-go removes nothing of a place, and an unlaunched launch has none. A place a runner launched is dropped only by the drop crew wrote, as Flume's own command.
- **core.11**: the runner's answers go to the branch as the runner gave them.
- **core.50**: since it, a pass takes its pull's launches first, which is what keeps a launch taken from a pull before its drop rare.
- **core.41**: the tests name no real host, partition, state repository or path.

### Rules departed from

None.

## Risks / Trade-offs

- **A launch taken from a pull before its drop is still made**, and its place works until the next pull gives the drop. Since core.50 that window is a pass's first round. Before core.50, as on 2026-11-07, it was the whole pass.
- **An act naming no host on a launch that is only claimed is given nothing once a drop names that launch.** The runner would refuse such an act anyway, since it holds no launch that is never offered.
- **Every drop crew wrote earlier whose launch the runner still holds launched, and which was never given, is given at the first pass of this release.** The runner then drops that place with `flume drop`. That is what crew asked. On a place already over, Flume's answer to the drop is the act's.

## Migration Plan

Nothing on a host moves. At the first pass of this release:

- crew's implementation gives each drop its runner holds the launch of, and offers no dropped launch the runner has not launched;
- the report's write lets go each dropped launch held unlaunched, and removes each claim of a dropped launch from the runner's branch.

## After deploy

Nothing. No step of this change is outside the gate.

## Task notes

- **1.1** `Main::dropped`, per Decision 1.
- **1.2** `Pulling::launched` and the skip in `pull_partition`'s walk, per Decision 2.
- **1.3** The claim filter in `Crew::report` and `own_claims`, per Decision 2.
- **1.4** `Pulling::current`'s drop clause, per Decision 4.
- **1.5** `write::let_go`, per Decision 3.
