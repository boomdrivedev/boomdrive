---
number: 31
title: The runner brings up the preview its control plane asks for through sluice, and puts away only what it asked for
status: proposed
date: 2026-11-08
decision-makers:
- Chuck Swanberg
---

# The runner brings up the preview its control plane asks for through sluice, and puts away only what it asked for

Status: proposed on 2026-11-08 by the construct stage of the change `the-runner-brings-up-a-preview-its-control-plane-asks-for`. The direction is the user's, in their words of 2026-11-08 quoted below, which crew's record 0056 holds. That record's consequence on previews, agentplot-design's choice on its point 1, moves a card's preview onto the runner. How the runner meets it is the stage's choice, made on this repository's rules and sluice's statement: the want's shape, the patience at sluice's bound, what is carried, what a done puts away, and crew's file. The record is accepted when the user says yes to it.

Deciders: the user, 2026-11-08, for the direction; the construct stage, for how it is met.
Sources: crew's record 0056, as rewritten and accepted on 2026-11-08, and its consequence on previews, on crew's branch `design/previews-through-the-runner`; sluice's statement, clauses 98 to 103 and 150, and sluice's `preview` and `preview-release` specs, at sluice v0.74.3; records 0002, 0003, 0010, 0026 and 0027; rules core.1, core.3, core.5, core.10, core.11, core.13, core.16, core.24, core.42, core.49 and core.50; the control-plane-interface, crew-control-plane and runner-protocol specs.

## Context and problem statement

crew brings up a card's preview itself. It runs `sluice preview <branch> <revision>` on the work's host, reads the preview back with `sluice previews`, and puts it away with `sluice release`. It hands the ask to a process of its own, since an ask waits while the host is at sluice's bound.

On 2026-11-08 the user ruled that crew is the control plane and talks to no tool but the runner. Declining a proposal that crew land each bolt by name: "It's not crew's job to land anymore. That's the whole point of the runner in Flume and sluice." On crew's record 0056's card: "But crew is like the control plane. It doesn't talk to Flume. Remember, the Boom Drive runner is pulling from the control plane. Everything has to happen from the Boom Drive runner. There can be messages back and forth with the control plane but crew doesn't know about Flume directly except for whatever the control plane contract allows." They accepted the rewritten record that day.

crew's record counts a card's preview among the places where crew still calls sluice against its point 1, and moves previews onto the runner. In short:

- the control plane asks for a preview by one message, `preview`, which names a repository, a branch and a revision. It asks for the preview's end by that message again, with a `done` mark;
- the runner on the work's host, which keeps the repository, runs sluice's preview command for it once, and hands back what sluice answered;
- crew learns a preview's addresses and state from the runner's report and nowhere else;
- neither crew nor the runner puts away a preview it did not ask for, so a preview a person brought up stays.

It has one Boomdrive unit come first, then a crew unit, and leaves `preview` in the hosted protocol to a later version, with `land`.

This repository's runner carries out a control plane's work in one control pass, on one thread, for every drive it serves (records 0002 and 0026). The pass's acts on places wait until every launch of its pull is taken, and once the next pass is due only the first runs (core.50). No drive's slow answer may delay another drive's work (core.49). The runner starts, keeps and stops no tool's long-running process (core.13). A sluice ask that waits at the host's bound holds whatever runs it.

## Considered options

For the want:

1. **A want of its own, `preview`, under a lease of its own**, naming a repository, a branch, a revision and whether it is done.
2. **A command sort.**
3. **An act on a launch.**

For the wait at sluice's bound:

1. **A patience of 20 seconds, then SIGINT**, the preview held and asked again at the next pass.
2. **The ask awaited whole.**
3. **`--wait`.**
4. **A process kept across passes**, as crew keeps one.
5. **SIGKILL at the patience.**

For what a done puts away:

1. **What the runner's report holds of the done's repository and revision that it asked sluice for**, and nothing where it holds none.
2. **Whatever sluice lists of the revision.**
3. **The preview a done names by its bring-up's id.**

## Decision outcome

The first of each.

- **A preview is a want of its own.** `Want::Preview` holds an id, a lease, a repository, a branch, a revision, and whether it is done. A bring-up and its done are wants of their own ids, each answered once. The report gains `previews`, each a `Previewed`: its lease, the repository and revision it named, whether it was done, its outcome `held`, `done` or `refused`, sluice's text, the words, sluice's listing of it as its status, and its time.
- **The bring-up is sluice's own command, once.** `sluice preview <branch> <revision> --json`, in the clone's container of a repository the runner keeps, its branch and revision as the want gives them. sluice's answer is the preview's, `done` or `refused` in sluice's words. A preview in a repository the runner does not keep, or drains, is refused, and one whose clone is not yet made waits (core.5).
- **The runner waits on sluice's bound no longer than 20 seconds.** It then sends sluice SIGINT, which sluice takes as ending its wait and nothing else. The preview stays made and waiting, is answered `held`, and is asked again at each pass until sluice answers it or its done puts it away.
- **What is carried leaves out the command's output.** Of a preview's document, its answer and each row of its listing, the runner carries `name`, `branch`, `revision`, `state`, `made` and `addresses`, each as sluice printed it. Of a release's, it carries `preview` and `released`. A failed preview's `output` is its command's log, and is never carried (core.11).
- **sluice's listing is carried until it lists the preview no more.** After the rounds, `sluice previews --json` is read once a pass per repository of a preview held or brought up. Its row is carried as the preview's status, so the control plane sees it come up, fail, or go.
- **A done puts away only what the runner asked sluice for.** It finds what the report holds of its repository and revision, brought up or held, and puts it away by one `sluice release <name>`, the name sluice gave. A held preview so put away is refused and asked no more. A done with nothing there, or no clone, is refused and runs nothing.
- **A preview's command runs after every launch.** It runs in the pass's second round, and once the next pass is due none runs after the first. The count is the previews' own, so core.50's rule for acts stands as it is.
- **Leases as any lease.** A lapsed lease waits (core.24). A lease let go or refused leaves the preview as sluice holds it, named (core.10).
- **crew's implementation reads `Act: preview`.** Its fields are `Host`, required, `Repo`, `Branch`, `Revision` and `Done`. A preview naming no `Host` is given nothing. Its answer goes to a fourth file of the runner's branch, `previews.rec`, under a descriptor this record proposes for crew to hold byte for byte, and is read back from it.
- **Version 1 carries no preview** and is not changed (core.42). A later version that names `preview`, with `land`, is the runner's and the cloud's together.

The command sort was not taken: a command holds no carried answer, and its `Commanded` is written whole into version 1's closed `commanded`. The act was not taken: a preview is on no place of Flume's. Awaiting the ask whole holds every drive's work on sluice's bound (core.49, core.50). `--wait` awaits the preview command's whole run too. A process kept across passes is what core.13 rules out, and a restart would orphan it. SIGKILL could cut sluice short outside its wait. Releasing whatever sluice lists of the revision would put away a preview the control plane never asked for. A done naming its bring-up's id is not the message crew's record names.

### Consequences

- Good: crew calls sluice for no preview, and reads a preview's addresses and state from the runner's report.
- Good: no launch, and no other drive's work, waits on sluice's bound behind a preview for longer than the patience.
- Good: a failed preview's output never leaves the host.
- Bad: a preview held loses its place in sluice's line between passes. sluice keeps a waiting preview's place only half a minute past its last ask, and at the default `poll_interval` the runner asks for 20 seconds of each minute. An ask of sluice's that never waits would cure it. That is sluice's to weigh, and is told to the orchestrator.
- Bad: sluice keeps one preview of a revision on a host. So where a person brought up the very revision the control plane asked for, the control plane's done puts it away, as crew's own release does today.
- Bad: a preview whose lease the runner lost stays up until someone puts it away, as a lost launch's place stays.
- `previews.rec`'s descriptor is the runner's proposal until crew's unit holds it. sluice's statement's row for Boomdrive names the machinery's effects, and Boomdrive now asks for previews too. Both are told to the orchestrator.
- Rule core.55 says it, under *Work under a lease*.
