# Design

## Context

See proposal.md for why. There are no area pages, so `docs/architecture/core.md` binds every path this change touches. The change relies on these rules:

- core.1: the runner decides nothing the control plane or Flume decides;
- core.9: a launch is taken only while its stream has room, as Flume's status gives it;
- core.10: each act and command is carried out once, under its lease;
- core.11: each answer is the tool's;
- core.14: the runner moves no branch of the clone;
- core.15: a launch is passed as given, and no tool is asked anything for it;
- core.16: a command runs one tool's own command line, or is refused naming its tool;
- core.24: work under a lapsed lease waits;
- core.36: `report.json` keeps every field the release before reads;
- core.42: a protocol version a release has spoken is never changed.

It draws core.52, and amends record 0002 with a third amendment of 2026-11-07, proposed. Both are written with this change.

### What stands

- **A launch** becomes one `flume launch` naming what it gives, its kind passed as given (`carry.rs`, `flume.rs`; the work-into-flume spec). Room is read from the pass's listing and counted down as the pass launches.
- **A retry naming the submission** runs `flume --json retry <place> --submission` (`flume.rs`; `tests/acts.rs`'s `a_retry_of_the_submission`).
- **A branch command.** `Work::command` takes a `CommandSort::Branch` to `Work::branch`, which:
  - runs `Sluice::branch`, that is `sluice branch create <branch> --from <from> --json` in the clone's container;
  - reads `clone::origin_commit` by `git ls-remote`;
  - records a `Commanded` holding sluice's text and the origin commit;
  - sets `ran`, so the statuses' listing is read again (record 0027).
- **Any other command** is refused, naming its tool: `this binary does not carry <tool>'s commands`.
- **The interface.** `boomdrive-control`'s `CommandSort` has `Branch { repository, branch, from }` and `Other { tool, words }`, and `Commanded` has `origin_commit`. `report.json` is `Report` as its serde writes it.
- **Version 1 of the hosted protocol** carries the interface's `Want` and `Commanded` as their serde writes them (the runner-protocol spec). Its `command_sort` names `branch`, and its `commanded` requires `origin_commit`. Every release since v0.1.0 has spoken it.
- **The hosted implementation's `own_wants`** withholds a branch command naming a repository its account is not served (core.26).
- **crew's implementation** gives no command. Its requests give a retry's `Submission`, so a bolt's send reaches the runner from crew already.

### What Flume now holds

A branch that work lands on, and that itself lands, is a base place's: a place whose kind lands and has no stages (Flume's 202).

- **No worktree and no room.** It has no worktree, runs no agent and takes no room.
- **Its branch is sluice's.** Its launch asks the merge queue to make its branch at its base's revision (205), and its tidy asks the merge queue to close it (206).
- **Its units.** A launch on its branch is cut and lands there.
- **Its send.** It is submitted only on its caller's retry naming the submission. That retry is refused while a place on its branch is live (203).
- **Its drop** is refused while such a place is live (204).
- **Its caller.** Flume's section 9 names the runner its caller: "each launch its control plane asks, passed as given, a bolt's base place and its units' places among them; a retry of a base place's submission its control plane asks (203)".

## Goals / Non-Goals

**Goals:**

- A control plane's bolt reaches Flume through the runner as a launch, and its send as an act, with nothing of the runner's own added.
- The runner keeps no branch command and no code that asks sluice for a branch.
- Nothing in the runner's code names a bolt.

**Non-Goals:**

- **The host's watch**, B1's other half. That is the sibling change `each-pass-ends-with-a-settle`.
- **The cloud's and crew's halves.** They are named under *Named follow-ups*.
- **A new protocol version.** Version 1 already carries a launch, a retry naming the submission and a refused command.
- **A floor for Flume.** The runner runs no Flume command it did not run before (core.38).
- **Which host takes a bolt and its units, in what order, and when it is sent.** These are the control plane's (core.1).

## Decisions

### 1. A bolt is a launch, taken as any launch

The runner takes a launch of a kind that lands with no stages as it takes every launch. It passes the kind, the base and every other field as given, marks the place `boomdrive:<id>`, and runs no sluice. Its units are launches whose base is its branch, as they are now. The code does not change for this. The tests show it. A scenario that fails means the runner treats such a launch apart from others, and that is fixed in `carry.rs`.

Alternatives:

- **A launch of its own sort in the interface for a bolt.** Not taken. Flume knows a base place by its kind's shape alone (Flume's 202). A sort of the runner's would name a bolt in its code, which the user ruled out ("The code shouldn't know about bolts and units").

### 2. A bolt counts against its stream's room as any launch

The runner judges a bolt's launch by its stream's room, or the host's where it names no stream, as Flume's status gives it. Within its pass it counts one against that room, though Flume counts none for a base place (Flume's 202). The cost has two parts:

- A bolt waits for a slot in a full stream, where Flume would have made it at once.
- In the pass that makes it, the bolt holds a slot a unit after it could have taken. The next pass's listing gives the slot back.

A bolt's units, in the same stream, wait for room anyway, so the delay to the bolt's work is at most one pass.

Alternatives:

- **The control plane's word that a launch takes no room.** Not taken. That is a fact of Flume's given by the control plane, which would need a field in the interface and a new protocol version (core.15, core.42).
- **Reading the kind's stages before the launch.** Not taken. That asks Flume something for the launch (core.15).
- **Launching whatever the room, and letting Flume queue.** Not taken. It departs core.9 for every launch, and a host would take work another host has room for.

### 3. A bolt's send is an act

A bolt is sent by a retry naming the submission, an act on the bolt's launch under that launch's lease, carried out as `flume --json retry <place> --submission`. It is already built.

It is not a command:

- A command has a lease of its own and acts on no place.
- An act carries its launch's lease, so a lease the runner let go stops it (core.10).
- An act on a place Flume no longer stands is answered by Flume's refusal of its status (core.51).

Flume's refusal while a place on the bolt's branch is live is the act's answer, as Flume gave it (core.1, core.11).

### 4. A branch command is refused, naming sluice

`Work::command` answers every command under a live lease `refused`, with `this binary does not carry <tool>'s commands`:

- for `CommandSort::Branch { .. }` the tool is `sluice`;
- for `CommandSort::Other { tool, .. }` it is that tool.

It reads neither the command's repository nor its clone, and runs nothing. A command the report holds stays as it stands, under the lease the pull gives now. A lapsed one is left untaken. `Work::commanded` records the answer with no text and no origin commit.

These go:

- `Work::branch`;
- `Sluice::branch`, `Cli::branch` and `Printed` in `sluice.rs`;
- `clone::origin_commit`;
- sluice 0.61.1's `branch-made.json`, `branch-there.json` and `branch-ungated.json`, with the fixtures README's section *A bolt's branch*;
- `tests/sluice.rs`'s tests of a branch.

Alternatives:

- **Leaving a branch command untaken.** Not taken. The control plane would wait for good on a branch no host will make (cloud record 0017's "waiting when none is online"), never learning why.
- **Carrying the branch command until the cloud follows.** Not taken. The brief says the runner keeps none. And a bolt's branch made outside any place is the asymmetry the user ruled out.

### 5. The shapes stay

`CommandSort::Branch` stays in the interface, documented as a sort a control plane may still send and this binary refuses, naming sluice (core.52). The reason is version 1: its wants are the interface's `Want` as their serde writes it, and its schema passes a `branch` command. Dropping the variant would make a pull holding one, which the check passes, fail to read whole, and version 1 may not change (core.42). The hosted implementation's withholding of a branch command for a repository its account is not served stays as it is (core.26).

`Commanded::origin_commit` stays too, documented as none for every command this release answers. A branch command an earlier release carried out keeps its own until its lease is let go. The release before reads the field in `report.json` (core.36), and version 1's `commanded` requires it.

A later release may drop it from `report.json` once no release it rolls back to reads it, as core.36's details allow. Version 1 keeps it, `null`.

Alternatives:

- **Dropping `Branch`, with the hosted implementation reading version 1's branch command as `Other { tool: "sluice" }`.** Not taken. It needs a version 1 type of its own for commands, and a conversion that `protocol/v1.rs` says it holds none of, for no change in what the runner answers.

### 6. A command no longer makes the statuses' listing be read again

`Work::branch` set `ran`, so a pass whose wants made a branch read Flume's listing again before carrying the statuses (record 0027). With no command running a tool, only a launch or an act of Flume's sets it. A pass whose wants answered only commands reads Flume's listing once, before its pull.

### 7. The rule and the record

- **core.52**, under *What the runner decides*. The runner asks no tool to make, move or remove a branch but through Flume's own commands on a place. A branch other places land on is a Flume place's, launched as any launch and sent by a retry of its submission. A command for a branch is refused, naming sluice. Its details name what it rules out and what it leaves.
- **Record 0002's third amendment of 2026-11-07** retires settled point 3, the user's own of 2026-11-01, on their acceptance of Flume's record 0051 and its follow-ups on 2026-11-06. It is proposed until the user sees it as a record.
- **core.16 is unchanged**: a command of a sort this binary does not carry is refused, naming its tool, and its details already allow that.

### 8. The README

The README's control pass says a bolt is a launch like any other, its send a retry naming its submission, and that every command is refused, naming its tool. Its sentence on `sluice branch create` goes.

## Who owns what

| piece | owner | cited by |
|---|---|---|
| a bolt's branch, made and closed | sluice, at Flume's asking (Flume's 202, 205, 206) | the runner, crew, the cloud |
| a bolt's place, what it waits on, and the guards on it | Flume (202 to 204) | the runner, crew, the cloud |
| a bolt's launch and its send | the control plane, carried by the runner as a launch and an act (core.15, core.52) | Flume |
| which host takes a bolt and its units, their order, and when the bolt is sent | the control plane (core.1) | the runner |
| the host's watch, which moves a submitted bolt between commands | the runner (the sibling change `each-pass-ends-with-a-settle`; Flume's 208) | Flume |

## Named follow-ups

- **agentplot/boomdrive-cloud: a bolt is a launch.** The cloud's records 0017 and 0051 make a bolt's branch by a branch Command, and end the bolt on a refused one. The cloud follows:
  - its bolt is a launch of a kind that lands with no stages, which it names, its base the branch the bolt lands on;
  - its units are launched on the bolt's branch;
  - its send is an act retrying the bolt launch's submission;
  - it issues no branch Command.

  Flume's guards on a base place hold among one host's places (202 to 204), so the cloud keeps a bolt's units on its base place's host, or keeps those guards itself. Until the cloud follows, a hosted bolt's branch command is refused with the runner's words, and its record 0051 ends the bolt.
- **agentplot/crew: C1, a bolt is a base place**, as Flume's design names it. crew's implementation here needs nothing: a bolt's launch is a request like any other, and its send a retry request naming the submission.

## Risks / Trade-offs

- **Hosted bolts are refused until the cloud follows.** The refusal names sluice, and the cloud's record 0051 ends the bolt and asks on the rail. The runner can do nothing more.
- **A bolt waits on room it will not take** (Decision 2). It is bounded by one pass for its units.
- **A Flume that predates base places** refuses a kind with no stages, in weir's words. That refusal is the launch's answer (core.1). The set's Flume is the flake's pin (core.44, core.45).
- **A control plane that splits a bolt's units across hosts** loses Flume's guards on the base place, since Flume reads only its own host's places. That is the control plane's (core.1).

## Migration Plan

Nothing held changes shape: `report.json` keeps every field. A branch command an earlier release left untaken, because its clone was not yet made, is refused at this release's first pass. One an earlier release carried out keeps its answer, and runs nothing. A release rolled back to reads this release's refusal from the report and answers from it, so it makes no branch this release refused.

## Open Questions

None.

## Task notes

- **1.1.** `tests/branches.rs` is renamed `tests/commands.rs` by `git mv`, and its tests become the commands spec's. Its bench keeps the scratch forge where a scenario needs a clone made, and `Bench::clones.not_yet` where one needs a clone not yet made. Its `Host::makes_the_branch_at` goes.
- **1.2.** "A branch command an earlier release carried out" writes the report file as an earlier release left it, with `Commanded { outcome: Done, text: Some(<sluice's answer>), origin_commit: Some(C), .. }`, before the first pass.
- **1.4.** The test sits beside `a_place_tidied_while_the_pass_answers_its_wants` in `tests/statuses.rs`, and counts the listings Flume was asked as that test does.
- **2.1.** Once `Sluice::branch` goes, the stub's helpers that only the branch tests used are dropped from `tests/fakes/mod.rs`, so clippy finds none unused.
- **2.2.** The test reads every `.rs` file under `crates/*/src` and fails naming each that holds the argument list of `sluice branch create` or `sluice branch delete`, that is `"branch", "create"` or `"branch", "delete"`.
- **3.1 and 3.2.** `tests/bolts.rs` uses the fake Flume and stub of `tests/fakes`. The kind `bolt` and the names are neutral (core.41). The send's refusal is the stub's `refused` answer, `{"refused":{"refusal":{"reason":…}}}`, exit 1, as Flume's contract has it. In the room scenario, the fake Flume's listing is set to give `home` a room of 1 at the second pass, as Flume gives it with the base place taking none.
