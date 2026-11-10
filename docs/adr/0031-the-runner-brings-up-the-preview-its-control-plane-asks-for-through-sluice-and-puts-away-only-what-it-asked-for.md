---
number: 31
title: The runner brings up the preview its control plane asks for through sluice, and puts away only what it asked for
status: accepted
date: 2026-11-08
decision-makers:
- Chuck Swanberg
---

# The runner brings up the preview its control plane asks for through sluice, and puts away only what it asked for

Status: accepted by the user on 2026-11-09, on their card, with its amendments of 2026-11-08 and 2026-11-09. Proposed on 2026-11-08 by the construct stage of the change `the-runner-brings-up-a-preview-its-control-plane-asks-for`. The direction is the user's, in their words of 2026-11-08 quoted below, which crew's record 0056 holds. That record's consequence on previews, the design agent's choice on its point 1, moves a card's preview onto the runner. How the runner meets it is the stage's choice, made on this repository's rules and sluice's statement: the want's shape, the patience at sluice's bound, what is carried, what a done puts away, and crew's file. The record is accepted when the user says yes to it.

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

The code stage made three choices of its own, each cheap to change. A held preview is not asked of sluice in a pass whose pull gives its done, since the done puts away what sluice holds of it; asking it first and then putting it away was not taken. A preview's lease counts with the launches' and commands' toward the expiry that hastens the next pass, so it is renewed before it lapses; leaving it to lapse between polls was not taken. A held preview that sluice brings up after its listing read it gone drops that status, so the listing of the preview brought up is read anew; leaving it standing unlisted, read no more, was not taken.

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

## Amendment, 2026-11-08: a part's pass brings up its previews

Status: accepted by the user on 2026-11-09, on their card, with the record. Proposed on 2026-11-08 by the code stage of the fix `the-runner-brings-up-previews--takes-main-in-1`, which took `main` into the bolt `the-runner-brings-up-previews`. It is the stage's own choice, within this record and record 0026 as they stand, and cheap to change, so it is not asked. It is accepted with this record.

### Context

This record was written while one control pass carried every drive's work, the keeping pass on the same loop before it. Record 0026, as `main` now carries it, runs each part of the control plane on a thread of its own, beside the loop's keeping passes. The parts share two things, each under a lock: a clone's lock, held by the keeper for one repository's keep or let-go and by a part for one launch into that clone, and the report file's, held for one save of a part's share. A launch holds the clone's lock so that it never makes a worktree in a clone the keeper is removing as dropped.

A preview's ask, `sluice preview`, also makes a checkout in the clone, and may wait at sluice's bound for this record's patience of 20 seconds.

### Decision outcome

- **A preview is brought up on its part's thread.** Its rounds, its listing and its done are its part's pass's, and what this record says of the pass holds of each part's. A preview held at sluice's bound holds its own part's pass alone, for no longer than the patience.
- **A part's share of the report file holds its previews**, as it holds the part's launches, acts, commands and inboxes: a part's save replaces its own previews and keeps every other part's.
- **A preview's ask takes no clone's lock.** A clone the keeper removes while sluice brings up a preview there takes the preview's checkout with it, and sluice frees a preview nobody can use (sluice 123), as it does of a dropped clone removed between passes. This record already answers a done whose clone no longer stands.

Not taken:

- **The clone's lock held shared for a preview's ask, as a launch holds it.** The ask may wait the whole patience. The keeper's keep of that repository would wait for it, and every launch into the repository behind that keep, so a launch would wait on sluice's bound, which this record and core.50 rule out.

### Consequences

- Good: no part's launch waits on another part's preview, and none waits on a preview of its own part's beyond the order core.50 and core.55 give.
- Bad: where a repository leaves the configuration while a part brings up a preview in it, the keeper may remove the clone under the new checkout; the preview is brought up and then freed by sluice, and with no clone its listing is read no more, its status left as it was last carried.
- Record 0026's list of what parts share stands as it is: a preview takes neither lock but the report file's, at its part's save.

## Amendment, 2026-11-09: a preview leaves the runner's report once its done is answered

Status: accepted by the user on 2026-11-09, on their card, with the record. Proposed that day by the construct stage of the change `a-put-away-preview-leaves-the-runners-report`. Its direction is that change's brief: "The runner's report keeps a preview only while its control plane still asks for it: once its done is answered, or the work it previewed has landed or been dropped, the preview is let go and leaves the runner's branch". How crew's implementation meets it is the stage's choice, made within this record and crew's record 0056, and cheap to change, so it is not asked. It is accepted with this record.

### Context

crew's implementation let a preview go only once its work had left crew's plan and the preview was no longer up. Two things kept previews on the runner's branch after crew had stopped asking for them:

- **A bolt's work never left the plan.** crew asks for a landings row's preview as `bolt/<b>`. `Main::plans` read no `bolt/<b>` as leaving, so a bolt's previews and their dones never left. crew's design of `a-cards-preview-is-asked-of-the-runner` names this as a follow-up for this repository. On crew's `main`, `bolt/<b>` is also the work of a bolt's base place, whose records stayed too.
- **A preview put away stayed while its work was in the plan,** and so did its done. crew's ask of a host, repository and revision stands while its newest preview request of them is a bring-up. A done ends it.

crew's implementation gives every request whose work is in the plan. So a preview let go while its work is in the plan would be given again, and brought up again, unless the pull also stops giving it.

### Decision outcome

- **A done answered is let go with what it put away.** At each report, a done the runner's report holds leaves the runner's files, and is named in the let-go, whatever the plan holds. With it go the bring-ups the report holds of its `Host`, `Repo` and `Revision`, each written before it.
- **A preview request is given while crew still asks for it, or while the runner holds it.** A request the runner does not hold is given only while it is crew's newest preview request of its `Host`, `Repo` and `Revision`. A bring-up is then given only while its work is in the plan. A done is then given whatever the plan holds, but only while the runner holds a bring-up of those three. So a preview and its done, once let go, are given no more. A bring-up crew put away before the runner answered it is never brought up.
- **A bolt's work leaves the plan with its `Bolt` record,** as `fix/<b>/<n>` does. A bolt's refused preview leaves once the bolt lands or is dropped, and so does one sluice lists no more. Its base place leaves with its acts once Flume no longer lists the place, as a unit's does.
- **A preview still up stays until its done,** whatever the plan holds. crew writes the done when the row ends, and the done is given whatever the plan holds.

Not taken:

- **Letting a preview go once its work leaves, up or not.** crew writes the done as the row ends, often after the bolt has left the plan. With the bring-up gone, the done finds nothing to put away. sluice's preview then stays up with no ask left to put it away (core.55).
- **Keeping the done until its work leaves the plan.** A done is never given again once answered. Keeping it would leave a bolt's dones on its branch for the bolt's life.
- **The runner dropping a preview it put away of its own accord.** What is let go is the control plane's to name (core.1, core.10). The runner keeps each answer until then, so it answers each want once.
- **A done given whatever came after it.** A done puts away every preview of its repository and revision the report holds. So a done written before a later bring-up would put that bring-up away, though it is crew's ask.
- **A done given while it is the newest, whatever the runner holds.** Once both were let go, it would be given and refused at every pass.
- **`bolt/<b>` read as leaving for previews alone.** One work would then leave the plan for some requests and not others.

### Consequences

- Good: a put-away preview and its done leave the runner's branch at the report of the pass that put the preview away. A bolt's previews and its base place leave once the bolt lands or is dropped.
- Good: a bring-up and its done written before the runner answers either bring up nothing.
- Bad: a done crew writes with no bring-up the runner holds is never given, so never refused in the runner's words. crew writes a done only for an ask that stands, and reads no done's answer.
- Bad: requests are matched by `Repo` as written, as crew keys its asks. Two spellings of one repository are two asks. crew writes each done with its ask's own `Repo`, so a done always matches its ask.
- Rule core.55 stands as it is: it says what the runner brings up and puts away, and nothing of what an implementation lets go.
