# Design

## Context

See proposal.md for why. There are no area pages, so `docs/architecture/core.md` binds every path this change touches. The change relies on these rules:

- core.1: the runner decides nothing the control plane decides, and carries each tool's answer as the tool gave it;
- core.2: the control plane is reached through one interface, and only an implementation names crew or the hosted protocol;
- core.3: the runner opens no listening socket;
- core.5: the runner keeps a clone only of a repository its configuration names;
- core.10: each piece of work is carried out once, under its lease, across restarts, and a lease the runner lost leaves its work as the tool holds it;
- core.11: no task log is carried, written or kept;
- core.13: the runner starts, keeps and stops no tool's long-running process;
- core.14: the runner's clone is the host's one clone, which sluice may work in;
- core.16: a tool's command the control plane asked for runs once, awaited, its answer carried as given;
- core.24: work under a lapsed lease waits;
- core.25: a pass whose report cannot be saved carries out nothing more;
- core.26: what goes over one account's connection names only its own work;
- core.36: `report.json` keeps every field the release before reads;
- core.42: a protocol version a release has spoken is never changed;
- core.49: no drive's slow answer delays another drive's work;
- core.50: a pull's acts on places run after its launches, and once the next pass is due only the first runs.

It draws core.55, adds to core.13's details the ask core.55 interrupts, and writes record 0031, proposed. All three are written with this change. It departs from no rule.

The control plane is reached through `boomdrive-control`'s `ControlPlane` interface alone (core.2). A preview is a want of that interface. crew's implementation, `crates/boomdrive-crew`, gives it from crew's requests. The hosted implementation, `crates/boomdrive-hosted`, gives none under version 1 (Decision 11).

### What stands

- **The interface's wants.** `boomdrive-control`'s `Want` is `Offer`, `Act`, `Command` and `Inbox` (`src/want.rs`).
  - An offer and a command carry a `Lease`. An act carries its launch's lease number. An inbox carries none.
  - `Report` holds `launches`, `acts`, `commands` and `inboxes`, by id (`src/report.rs`).
  - `Report::leases` names each launch's and command's lease, and the hello carries it.
  - `report.json` is `Report` as its serde writes it, and reads with no field denied, so a field added is ignored by the release before.
- **The control pass** (`crates/boomdrive/src/carry.rs`) answers the pull's wants in two rounds, each in the control plane's order (core.50):
  - the first round takes every offer, inbox and command, and every act that runs no command;
  - the second carries out each act on its launch's place, and once the next pass is due runs none but its first, naming how many it left;
  - after the rounds it carries Flume's statuses and sluice's entries, then sends the report;
  - each want's answer is saved before the next, and a pass whose save fails carries out nothing more (core.25);
  - `ran` marks a pass whose wants ran a command of Flume's, so Flume's listing is read again before the statuses (record 0027).
- **sluice's command line** (`src/sluice.rs`) runs `sluice` in a clone's container, awaited, and reads its answer by its exit:
  - exit 0 with JSON is `Reply::Given`;
  - any other exit is `Reply::Refused`, with the `message` of sluice's `{"error": …}` document, else its standard error;
  - a program that cannot run, or a signal that ends it, is `Unrun`, of which nothing is kept, and the next pass asks again.

  It asks sluice only for a submission's entry, `sluice status <id> --json`. Of that it carries the fields the cloud's copy of an entry names, each as sluice printed it, and no log (`carried`).
- **crew's implementation**:
  - reads a request's `Act` in `read::want`: `launch`, `retry`, `drop` and `priority`;
  - resolves a request naming this host against the hello's repositories, then those drained, then its `Repo` as written;
  - gives a request it cannot read, naming this host, as an act the runner refuses, with words naming the field;
  - writes the runner's branch, `runner.rec`, `answers.rec` and `places.rec`, under crew's descriptors byte for byte (`src/write.rs`), and reads it back into a report where the runner's file is lost (`src/lib.rs`'s `read_back`).
- **Version 1 of the hosted protocol** carries the interface's `Want` as its serde writes it (`protocol/v1/wants.schema.json`). Its `want` names `offer`, `act`, `command` and `inbox`, each object closed. The hosted implementation writes version 1's `report` part by part, `launches`, `acts`, `commands` and `inboxes` (`src/plane.rs`'s `upward`), and checks every message against its version before reading it (core.42).

### What sluice holds of a preview

sluice's statement, clauses 98 to 103 and 150, and its `preview` and `preview-release` specs, as the set's sluice, v0.74.3, carries them:

- **The ask.** `sluice preview <branch> <revision> --json`, in the repository's clone, makes a detached checkout of a revision the branch has landed. It runs the `[preview] command` of that revision's policy under the host's executor, and returns once the command has started. It prints one document: the preview's `name`, `branch`, `revision`, `state`, `made`, `checkout`, `addresses`, `names` and `ports`, and `exit`.
- **A refusal.** A revision the branch has not landed, a branch sluice does not manage, or a policy with no `[preview]` table is refused, making nothing. With `--json`, sluice prints `{"error":{"kind","message","next"}}` and exits 2.
- **One preview of a revision on a host.** Asking again for a revision whose command has started gives the same preview back and starts nothing (100).
- **The wait.** While the host is at its bound, the ask waits for room, saying so on standard error, and the preview is listed `waiting`. A SIGINT ends the wait and nothing else: sluice says the preview waits and to ask again to start it, prints no document and exits 130. An ask made again takes the same preview up.
  - The host keeps a waiting preview's place in line only while its ask waits, and for half a minute after (150).
  - A SIGINT that comes while the ask is not waiting is read at no point, and the ask ends as it would have.
- **Its states.** `waiting`, `starting`, `up` once the command exited 0, and `failed`, read from the host's files, whichever process asks.
  - A failed preview's document holds its command's `output`.
  - sluice releases a failed preview itself, keeping it listed with its output until its caller releases it.
- **The listing.** `sluice previews --json` lists every preview of the repository on the host as one array of such documents. First, as every read of a preview's outcome does, it releases each preview nobody can use (101, 123).
- **The release.** `sluice release <name> --json` runs the release command and frees the preview. It prints `{"preview": <name>, "released": true}`, or `"released": false` where there is nothing to release, exiting 0 either way. A release command that fails exits 1, saying so on standard error, and holds the preview.
- **sluice ends no usable preview of its own accord.** One that is up stays up until its caller releases it.

## Goals / Non-Goals

**Goals:**

- A control plane asks for a preview of a branch at a revision, and to put it away, naming no word of sluice's.
- The runner brings it up through sluice's own preview command, once, and carries sluice's answer and listing of it as sluice gave them, with no command's output.
- No launch, and no other drive's work, waits on sluice's bound behind a preview.
- The runner puts away only what it asked sluice for.
- crew's implementation gives a preview from crew's `Act: preview`, and writes its answer where crew reads it.

**Non-Goals:**

- **crew's half.** crew posts `preview` and reads `previews.rec` from its own unit (*Named follow-ups*).
- **A protocol version that names `preview`.** Version 1 may not change (core.42). A later version is the runner's and the cloud's together.
- **Whether a kit declares a preview.** sluice reads that from the revision's policy at its ask. crew's record 0056 leaves it open: until the report can say it, crew offers a preview on every row, and sluice's refusal says where a kit has none.
- **Which host brings a preview up, and when.** These are the control plane's (core.1).
- **A floor for sluice.** sluice's preview, previews and release have stood since sluice 0.29, well below the set's 0.74.3. The runner reads no floor for sluice today, and this change adds none (core.38 stands as it does for every sluice command the runner runs).

## Decisions

### 1. A preview is a want of its own, under a lease of its own

`Want` gains `Preview(Preview)`:

```rust
pub struct Preview {
    pub id: Id,
    pub lease: Lease,
    /// The repository's address, `owner/name`.
    pub repository: String,
    pub branch: String,
    /// The revision to bring up, a commit, as the control plane gave it.
    pub revision: String,
    /// Whether the preview of `revision` is to be put away.
    pub done: bool,
}
```

Its serde form is `{"preview": {"id", "lease", "repository", "branch", "revision", "done"}}`. A preview is the control plane's ask of one host, so it carries a lease as a command does. `Want::id` gives its id.

crew's record has the bring-up and the put-away be one message, the second marked `done`, each naming the repository, the branch and the revision. So each is a want of its own id, answered once. A done names the preview it puts away by the repository and revision its bring-up named (Decision 6).

Alternatives:

- **A command sort.** Not taken. A command is answered once and holds no carried answer. Its `Commanded` is written whole into version 1's closed `commanded`, so it cannot gain the listing a preview carries (Decision 5).
- **An act.** Not taken. An act asks something of a launch's Flume place. A preview is of a branch at a revision, on no place of Flume's.
- **A done naming its bring-up by id.** Not taken. crew's record names the done by the repository, branch and revision, as the same message marked `done`.

### 2. The runner brings a preview up by sluice's own preview command, once

A preview not marked done, under a live lease, in a repository whose clone is made, is asked of sluice in the pass's second round (Decision 7). The ask is `sluice preview <branch> <revision> --json`, run in the clone's container, the branch and revision as the want gave them (core.15's shape: nothing added).

- **sluice brings it up.** sluice exits 0 with a document. The preview is answered `done`, its text what Decision 4 carries of the document. The line is `<id>: preview of <branch> at <revision>: brought up, <state>`, the state as sluice printed it.
- **sluice refuses.** The preview is answered `refused`, with sluice's words. The line is `<id>: preview of <branch> at <revision>: refused by sluice: <words>`.
- **sluice holds it waiting.** The preview is held (Decision 3).
- **sluice cannot be run.** Nothing is kept, the line is `<id>: preview of <branch> at <revision>: not carried out: <words>`, and the next pass asks again.

A preview the report holds `done` or `refused` is answered from the report, its lease the one the pull gives now, and sluice is not asked again (core.10). An ask made again after a restart that lost its answer is safe: sluice gives the same preview back and starts nothing (100).

The preview's stack is the repository's preview command's, which sluice runs under its executor and keeps until it is released (98, 101). The runner starts no process of its own and keeps none (core.13). The stack's addresses are the repository's processes', and the runner opens no socket (core.3).

### 3. The runner waits on sluice's bound no longer than its patience

The runner awaits `sluice preview` for at most its patience, 20 seconds. sluice has then not answered only where it waits at the host's bound, or is still readying its command. The runner then sends the process SIGINT and awaits its exit.

- **sluice was waiting at its bound.** It exits 130 with no document. The preview is held: answered `held`, with no text, and saved. The first time it is held, the line is `<id>: preview of <branch> at <revision>: waits at sluice's bound, asked again at the next pass`. A held preview is asked again at each pass, under the same rules, until sluice answers or its done puts it away (Decision 6).
- **sluice was not waiting.** It reads the SIGINT at no point, and finishes. Its answer is the preview's, as in Decision 2.

The interrupt is the runner's, a constant in `sluice.rs`, `PREVIEW_PATIENCE`. `sluice::Cli` takes another for a test.

Why: an ask that waits holds the runner's one thread. Every drive's launches would wait on sluice's bound behind it (core.49), and so would the next pull's (core.50). sluice's SIGINT is how a caller ends a wait and nothing else. The preview stays made and waiting, and an ask made again takes it up. The patience matches the 20 seconds crew gives sluice today to name a preview's addresses.

Alternatives:

- **Awaiting the ask whole.** Not taken: core.49 and core.50, for as long as the host's checks hold the bound.
- **`--wait`.** Not taken. It awaits the preview command's whole run as well, which is the repository's stack coming up.
- **A process kept across passes,** as crew hands its ask to one today. Not taken. core.13 rules out a tool's process the runner keeps, and a restart would orphan it.
- **SIGKILL.** Not taken. sluice survives an asker that dies while waiting, but a kill outside the wait could cut sluice short while it makes the checkout or readies its command.
- **A patience tied to `poll_interval`.** Not taken. It would grow a pass with the interval, for a wait sluice bounds by room, not by time.

### 4. What is carried of a preview leaves out its command's output

Of a preview's document, the runner carries a JSON object of `name`, `branch`, `revision`, `state`, `made` and `addresses`, in that order, each where sluice printed it, each value's text as sluice printed it (`sluice::preview_carried`, beside `carried`). It leaves out:

- `output`: what a failed preview's command printed, a task log (core.11);
- `release_failed`: what a failed release command printed, a log too;
- `checkout`, `names`, `ports` and `exit`: the runner's own path, the parts the addresses are made from, and the exit, none of which a control plane reads.

A document that is no JSON object, or names no `name`, is no preview. Where sluice gave one as its answer, the preview is answered `refused` with the words `sluice's answer is no preview`.

Of a release's document, the runner carries `preview` and `released`, each as sluice printed it.

Alternatives:

- **The document whole.** Not taken. A failed preview's `output` is its command's log (core.11's details: "a field of a tool's answer left out because it holds a log").
- **The addresses and state alone.** Not taken. `name` is what a done releases, and `revision` and `branch` say which preview it is.

### 5. sluice's listing of each preview is carried until it lists it no more

After the rounds, beside Flume's statuses and sluice's entries, the runner carries sluice's listing of each preview it holds up:

- **Which previews.** Each preview the pull gives, not marked done, that the report holds `held` or `done`, and whose status is not carried as unlisted.
- **One read per repository.** `sluice previews --json` is read once a pass in the clone's container of each such preview's repository, named or dropped.
- **Its row.** The row whose `name` is the one sluice's answer gave. For a preview held, with no answer yet, it is the row whose `revision` begins with the revision the want named.
- **What is carried.** The row's fields, as Decision 4 has them, as the preview's `status`, a `Carried` as a launch's status is. Its time moves only when its text does. Once the listing holds no such row, the status stands, not listed, and is read no more.
- **A listing that cannot be read.** Each status stands as it was. One line, `boomdrive: sluice's previews of <repository> could not be read: <words>`, is said, and the next pass reads it again.

So the control plane sees a preview move from `starting` to `up` or `failed`, and sees it gone once a person releases it.

sluice's listing releases each preview nobody can use before it lists them (101, 123). That is sluice's, as it is at sluice's next round on the host. A preview command runs only on the control plane's ask (core.13).

A pass's preview commands run sluice, not Flume, and set no `ran`: Flume's listing is not read again for them (record 0027).

Alternatives:

- **Asking `sluice preview` again each pass for its state.** Not taken. Where a person released the preview, the ask would bring it up anew.
- **The answer alone.** Not taken. It says `starting`, and the record's card shows a preview's addresses while it is up.

### 6. A done puts away only what the runner asked sluice for

A preview marked done is carried out in the second round, in the control plane's order (Decision 7). It puts away what the runner's report holds of the same repository and revision, brought up or held. An earlier want of the same pull counts, so a bring-up and its done given together are both carried out.

- **What it puts away.** Each preview of the report not marked done, whose repository and revision are the done's, answered `done` or `held`.
- **None there.** The done is answered `refused`, with the words `no preview of <revision> in <repository> was brought up here at the control plane's asking`, and sluice is not run.
- **The name.** The name sluice gave, in the answer or else in the listing, of the first such preview. Where neither names one, the revision the done names, which sluice takes as well.
- **The release.** `sluice release <name> --json`, once, in the clone's container, named or dropped.
  - sluice's answer is the done's, `done` with its text, or `refused` with sluice's words.
  - The line is `<id>: done of the preview of <branch> at <revision>: put away`, or `…: refused by sluice: <words>`.
- **A preview held, so put away.** Each such preview held is answered `refused`, with the words `put away before sluice brought it up`, and is asked of sluice no more.
- **A preview brought up, so put away.** It keeps its answer. Its status reads unlisted at the next pass's listing.
- **No clone stands.** Where the done's repository has no clone, its checkout went with the clone, and sluice frees such a preview itself (123). The done is answered `refused`, with the words `no clone of <repository> stands on this host`, and sluice is not run.

A person's own preview, of a revision no preview of the control plane's named, is never put away. sluice keeps one preview of a revision on a host (100). So where a person brought up the very revision the control plane also asked for, the control plane's done puts it away, as crew's own release does today (*Risks*).

Alternatives:

- **Releasing whatever sluice lists of the revision.** Not taken. It would put away a preview the control plane never asked for, which crew's record 0056 rules out for crew and the runner alike.
- **A done answered `done` where nothing was brought up.** Not taken. The control plane would read that a preview was put away where the runner ran nothing.
- **Releasing by the revision alone.** Not taken. sluice resolves a revision at the release, and the name is the preview's own.

### 7. A preview's command runs after every launch, and once the next pass is due none after the first

A preview's command, its ask or its release, runs in the pass's second round, among the acts on places, in the control plane's order. So no launch of its pull waits behind it.

- **What the first round answers.** A preview the report holds `done` or `refused`, one under a lapsed lease (left untaken), and a refusal that runs nothing. That last is a bring-up in a repository not kept, or a done whose repository has no clone. A done with nothing to put away is answered in the second round, where it is matched against the report as the round's earlier wants left it, and runs nothing, so it counts against no due.
- **Due.** Once the next pass is due, the second round runs no preview's command but its first, and leaves each after it to that pass. The line is `boomdrive: the next control pass is due, so 1 preview waits for it`, or `so <n> previews wait for it`.
- **Its own count.** The count is the previews', apart from the acts'. So core.50's rule for acts stands as it is, and a pass that is due runs at most one act's command and one preview's.
- **`boomdrive run --once`** carries out every preview, no pass following it.

Alternatives:

- **One count for acts and previews.** Not taken. It would change core.50's "runs none but its first" act whenever a preview ran first.
- **Previews in the first round.** Not taken. A launch would wait behind sluice's ask (core.50's reason).

### 8. Repositories, leases and restarts

- **A repository not kept.** A bring-up in a repository the runner neither keeps nor drains is answered `refused`, with the words `this host's configuration does not keep <repository>`, as an inbox's are. So is one in a repository it drains: nothing new is made in one. One whose clone is not yet made waits (core.5).
- **A lapsed lease.** A preview under a lease lapsed by the pass's clock is left untaken, and one held is not asked again, until a pull gives it under a live lease (core.24).
- **Let go or refused.** A preview the control plane lets go, or whose report it refuses as no longer the runner's, leaves the report. Its preview is left as sluice holds it, and the line names it: `<id>: let go, its lease no longer this runner's: <words>: its preview is left on this host as sluice holds it` (core.10).
- **Once, across restarts.** The report is saved after each preview's answer, as after every want's. A pass whose save fails carries out nothing more (core.25). An ask or a release made again after a lost answer is safe in sluice: the ask gives the same preview back, and the release finds nothing to release.

### 9. The report gains a part, `previews`

`Report` gains `previews: BTreeMap<Id, Previewed>`, serde `default` and skipped where empty, so a report holding none writes as before:

```rust
pub struct Previewed {
    pub lease: Lease,
    /// The repository and the revision, as the want named them.
    pub repository: String,
    pub revision: String,
    /// Whether the want asked that the preview be put away.
    pub done: bool,
    /// `held` while sluice waits at the host's bound to bring it up;
    /// else `done` or `refused`.
    pub outcome: Outcome,
    /// sluice's answer, the fields carried, as sluice printed each.
    pub text: Option<String>,
    pub words: Option<String>,
    /// sluice's listing of the preview, for one brought up or held.
    pub status: Option<Carried>,
    pub at: At,
}
```

- **Its time.** `at` moves only when the outcome, text or words do.
- **Its lease.** `Report::leases` names each preview's lease with the launches' and commands', so the hello carries it.
- **Forgetting.** `forget` drops a preview with every other part.
- **`Outcome`'s documentation.** An act or a command is only `done` or `refused`, and a preview `held`, `done` or `refused`.
- **The interface's version.** `INTERFACE_VERSION` stays 1. A preview is a want a control plane may give or never give. An implementation that gives none sees nothing change but a report part that stays empty.

`report.json` gains the part, which the release before ignores, and drops and renames nothing (core.36).

### 10. crew's implementation reads `Act: preview` and writes `previews.rec`

- **The request.** `read::want` gives `Some("preview")` as a preview: `Repo`, resolved as a launch naming this host resolves it, `Branch` and `Revision`, each required, and `Done`.
  - `Done` reads `yes`, `true` or `1` as done and `no`, `false` or `0` as not, and is not done where absent, as `Submission` is read. Any other value is a field it cannot read.
  - A preview missing `Branch` or `Revision` is a request it cannot read, refused by its host's runner with words naming the field, as now.
- **The host.** A preview is the ask of one host, the one its `Host` names, which crew reads as the host where the work runs. A preview naming no `Host` is given nothing, with the line `<label>/<n>: a preview names no Host, so no runner brings it up: it is given nothing`. No runner's answer to it would be the one, and two runners keeping its repository would each bring it up.
- **The runner's.** A preview the runner's report holds, or this host's `previews.rec` answers, is the runner's, as a launch or an act is through the report or `answers.rec`. It is given while its work has left crew's plan, as any request the runner answered is.
- **`previews.rec`.** A fourth file of the runner's branch, one `Preview` per preview request the report answers:

  ```
  %rec: Preview
  %key: Request
  %type: Request int
  %type: Outcome enum held done refused
  %type: Listed enum yes no
  %type: At date
  %mandatory: Request Outcome At
  %allowed: Request Outcome Text Status Listed Words At
  ```

  - `Text` is sluice's answer as carried, `Words` the refusal's words, and `Status` sluice's listing as carried, each on one line.
  - `Listed` says whether sluice's last listing held it, written with `Status`.
  - `At` is the status's time where a status is carried, else the answer's, as a role's `Run` takes its time.
  - The file is written where the partition's report holds a preview. Where the tip holds one and the report none, it is written as the descriptor alone. Otherwise the branch holds none, so a branch with no preview is written as before.
  - It passes `recfix --check` as the other three do. This descriptor is this change's proposal of crew's. crew's unit holds it in `runner.py` byte for byte (*Named follow-ups*).
- **Read back.** `previews.rec` is read with the runner's other files, and checked as they are: no record before its `%rec`, and no request twice. Where the runner's report file is lost, each `Preview` is read back as the preview its request names, under lease 1.
  - Its repository is resolved as at a pull, which takes the hello.
  - Its outcome, text and words are as written. Its status is `Status`, at `At`, listed as `Listed` says.
- **Let go.** A preview leaves the runner's files, and is named in the report's let-go, once its work has left crew's plan and it holds no preview up:
  - a done, or a preview refused;
  - a preview brought up or held whose status sluice's listing no longer holds.

  Work that never leaves the plan, as a `fix/main/…` launch's, keeps its last record.

Alternatives:

- **The answer in `answers.rec`, or a `Place` of `places.rec`.** Not taken. crew's descriptors allow no field for sluice's text, and crew reads a `Place` as a Flume place.
- **A preview naming no host claimed as a kit's launch is.** Not taken. crew names the host where the work runs, and a claim would bring a preview up on whichever runner claimed first.

### 11. Version 1 carries no preview

Version 1 is unchanged: no file under `protocol/v1/` changes, and its `want` names no preview (core.42).

- **A preview never reaches the hosted implementation.** A want that is a preview fails version 1's `want`, and a pulled message holding one is refused whole. So no preview reaches the implementation's types, which read a message only once it passes its check.
- **What the implementation does with one anyway.** Its type must take one, so:
  - `own_wants` withholds one naming a repository its account is not served, as an inbox (core.26);
  - `prefixed_want` gives its id the partition's prefix;
  - `part` takes the partition's previews, and `taken_back` gives none.
- **Nothing goes up.** `upward` writes version 1's four parts alone, so a report's `previews` never goes up.

`tests/interface.rs` shows it. A new test, `a_preview_is_no_want_of_version_1`, checks a preview want, as `boomdrive-control`'s types write it, against version 1's `want`, and finds it refused.

Alternatives:

- **Version 2 now, naming `preview`.** Not taken. A version is a new directory of every file, its examples and its tests, and the cloud pins the protocol by revision. It is the runner's and the cloud's to make together, with `land`, as crew's record 0056 leaves it.

### 12. The rule and the record

- **core.55**, under *Work under a lease*: how the runner brings up, carries and puts away a preview. Its details name what it rules out and what it leaves: sluice's own release of a preview nobody can use, the preview's stack, the interrupt, version 1.
- **core.13's details** leave a preview's ask of sluice the runner interrupts at its patience, which ends sluice's wait and nothing else (core.55).
- **Record 0031**, proposed, records the direction, crew's record 0056 on the user's words of 2026-11-08, and this stage's choices, Decisions 1 to 11. It is proposed until the user sees it as a record.
- **Numbers.** Rule core.53 is claimed on two branches not yet on `main`: `land-is-the-message-for-a-works-landing`'s, on its bolt, and `the-runner-carries-each-places-end`'s. When those two reconcile, one of them moves to core.54. So this change takes core.55, and record 0031, as record 0030 is the second change's.
- **Beside the land change.** `land-is-the-message-for-a-works-landing` modifies the runner-protocol requirement *The messages carry the interface's shapes*, to leave out a land. This change modifies it to leave out a preview and the report's previews. Whichever change reaches `main` second takes the other's words in: version 1 carries the interface's shapes, a land and a preview apart.

### 13. The README

The README's control pass gains a bullet: the runner brings up each preview the control plane asks for in a repository it keeps, as `sluice preview <branch> <revision>`, once every launch is taken. It waits at sluice's bound no longer than 20 seconds, asking again at the next pass, and carries sluice's listing of it until sluice lists it no more. It puts away, on a done, only a preview it asked sluice for, as `sluice release <name>`. It carries no output of the preview's command. The sentence on carrying back gains sluice's previews.

### 14. The fixtures

`tests/fixtures/sluice-0.74.3/` holds the set's sluice's own answers, captured on a scratch host as `sluice-0.61.1/`'s were, byte for byte, with a README saying how:

| file | the command | what stood |
|---|---|---|
| `preview-starting.json` | `sluice preview main <R1> --json` | exit 0: R1's preview made, `starting`, its addresses filled in |
| `preview-no-table.json` | `sluice preview main <R0> --json` | exit 2: R0's policy holds no `[preview]` table, as sluice's `error` answer |
| `previews.json` | `sluice previews --json` | exit 0: R1's preview `up`, and R2's `failed`, its `output` holding what its command printed |
| `release.json` | `sluice release preview/<R1> --json` | exit 0: R1's preview released |
| `release-nothing.json` | `sluice release preview/<R1> --json` again | exit 0: nothing to release |

The wait at the bound is not captured. A scratch host would have to hold its bound full. The stub `sluice` stands in for it, as Task notes 4.3 say.

## Who owns what

| piece | owner | cited by |
|---|---|---|
| which host brings up which preview, and when it is put away | the control plane (core.1) | the runner |
| the message `preview`, marked `done` to put it away | the interface, `boomdrive-control` (core.2, core.55) | crew's implementation, the runner |
| `Act: preview` in crew's requests, and `previews.rec`'s descriptor | crew, which writes the one and holds the other (crew's record 0056) | crew's implementation |
| how a preview is brought up, carried and put away | the runner (core.55) | crew, the cloud |
| a preview's checkout, stack, states, wait and release | sluice (98 to 103, 150) | the runner |
| a preview in the hosted protocol | a later version, the runner's and the cloud's together (core.42) | the hosted implementation |

## Named follow-ups

- **agentplot/crew: previews through the runner.** crew's record 0056 has this as one crew unit after this one. Its consequence on previews is on crew's branch `design/previews-through-the-runner`, not yet on its `main`. crew's unit:
  - posts `Act: preview` with `Host`, `Repo`, `Branch` and `Revision`, and the same marked `Done: yes`;
  - adds `preview` to its `requests.rec` descriptor's `Act` enum, and `Branch`, `Revision` and `Done` to what it allows;
  - holds `previews.rec`'s descriptor in `runner.py` byte for byte, and reads a preview's addresses and state from it.

  crew posts `preview` once its runners run the release that carries this change, which `runner.rec`'s `Version` names.
- **agentplot/boomdrive-cloud and this repository: a version that names `preview`,** with `land`.
- **agentplot/blueprints: sluice's statement.** Its table of drivers has Boomdrive's row name the machinery's effects. Boomdrive now asks for previews and puts them away. That is the statement's owner's to amend.
- **agentplot/sluice: an ask that never waits.** An ask of a preview that returns `waiting` at once and keeps the preview's place in line for its asker would let the runner ask without a patience. Until then a held preview may lose its place between passes (*Risks*). That is sluice's to weigh.

Each is for the orchestrator to carry.

## Risks / Trade-offs

- **A held preview may lose its place in line between passes.** sluice keeps a waiting preview's place only half a minute past its last ask. At the default `poll_interval`, a minute, the runner asks for 20 seconds of each minute. A preview on a host whose checks hold the bound may wait longer than one whose asker stays. It decides nothing (103), and the sluice follow-up would cure it.
- **A pass with a preview waiting is longer by up to the patience,** and a release or a first ask that readies a toolchain by as long as sluice takes. Once the next pass is due, only the first preview's command runs (Decision 7).
- **A person's preview of the very revision the control plane asked for is put away by its done** (Decision 6), since sluice keeps one preview of a revision on a host. crew's own release does the same today.
- **A preview whose lease the runner lost stays up** until a person or a later ask puts it away, as a lost launch's place stays (core.10). sluice never ends a usable preview of its own accord.
- **crew's `previews.rec` descriptor is the runner's until crew holds it.** A crew that reads the runner's branch before its unit lands ignores the file.

## Migration Plan

- **The runner's files.** `report.json` gains `previews` only where a preview is answered. The release before reads it and ignores the part (core.36), and a runner rolled back carries out no preview.
- **crew's files.** `previews.rec` appears on a runner's branch only once a preview is answered there.
- **crew posting `preview` before its runners run this release.** A runner of a release before this one refuses a preview naming its host, its words naming `Act` and `preview`. crew posts `preview` once its runners run this release (*Named follow-ups*).
- **Version 1.** No protocol file changes (core.42).

## Open Questions

None.

## Task notes

- **4.1, the capture.** sluice 0.74.3 is the flake's `sluice` package, `nix build .#sluice`, run by its path. The scratch host is as `sluice-0.61.1/README.md` describes it:
  - `HOME`, XDG directories and a git configuration of its own under `/private/tmp/scratch/host/`, and no network;
  - a bare `example/kit.git` on disk as its forge, whose `main` holds `.config/sluice.toml`, with `[checkout] ports = ["web"]`;
  - `[preview]`, with `command` a script that exits 0 at R1 and 1 at R2, `release` a script that exits 0, and `addresses = { web = "http://{name.web}.localhost:{port.web}" }`;
  - R0 a commit before the `[preview]` table.

  `previews.json` is read once R1's command has exited and R2's has failed. Every name is neutral (core.41). Paths under `/private/tmp/scratch/host/` stand in a fixture as they did in `sluice-0.61.1/`.
- **4.1, the test.** `tests/sluice.rs`'s `every_fixture_is_read` reads each directory's README and files. Its `READ` gains the new directory's five, or a second list of them.
- **4.3, the stub.** The stub `sluice` gains a wait: where `<what>.wait` exists, it traps INT, writes `stopped: preview/<what> waits; ask for it again to start it` on standard error and exits 130. It waits by `sleep 60 & wait`, so the trap runs at once. The bench builds `sluice::Cli` with the patience a test sets, `Bench::sluice_patience`, by default `PREVIEW_PATIENCE`. A test of the wait sets one second.
- **4.3, the interrupt.** The interrupt is `libc::kill(pid, libc::SIGINT)` on the child's id. The child's standard output and error are read on threads of their own while it runs, so neither pipe fills. Its exit is awaited by `try_wait` against the patience's deadline, then by `wait`.
- **3.1 and group 5.** `tests/previews.rs` uses the bench of `tests/fakes`, its `KEPT` repository, and the preview ids `home/30` and on. A preview's want is built by a helper beside `offer`, `preview(id, branch, revision, done)`, in `KEPT` under lease 1 with no expiry.
- **6.1.** `tests/common/mod.rs` gains `preview(n, host, work, repo, branch, revision, done)`, writing a preview request as `launch` writes a launch: `Act: preview`, `Done: yes` where `done`, and no `Done` otherwise. Its copy of crew's `requests.rec` descriptor gains `preview` in its `Act` enum, `%type: Done bool`, and `Branch`, `Revision` and `Done` in `%allowed`. Its `FIELDS` gains them after `Base`, in that order, as crew's unit writes them.
- **6.5.** `read_back` takes the hello, from `pull`, to resolve a preview's `Repo` as `pull_partition` resolves a request naming this host's.
