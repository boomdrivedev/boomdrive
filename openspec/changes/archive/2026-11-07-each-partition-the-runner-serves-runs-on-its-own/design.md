# Design

## Context

A runner serves one control plane (core.19) through one interface, `ControlPlane`, a pull and a report (core.2). Each implementation serves the partitions of its table. crew's table names a partition for each drive this host serves, a branch `<label>/main` of its account's state repository. The hosted table names a partition for each account enrolled, one connection each (record 0028; the change `a-hosts-configuration-names-its-accounts-and-their-drives`, Decision 9). Each implementation's pull and report go through its partitions in turn.

`boomdrive run` makes every pass on one loop, one at a time: a keeping pass on its timer, then a control pass that reads Flume's listing, pulls every partition, answers every want, carries every place's status and sluice entry, and reports to every partition (`crates/boomdrive/src/run.rs`, `carry.rs`). So every partition's work waits in one line. On deneb on 2026-11-07 one drive's backlog of acts held another drive's launches for a quarter of an hour, while that drive's stream had room for four agents and ran none (record 0026).

core.50 already takes a pull's launches ahead of its acts on places, and once the next pass is due, runs no act but the first. That is the first step within one line. Record 0026, accepted by the user, takes the rest: each drive runs on its own, and what drives share on a host is a cap or a synchronization its owner designs. core.49 says it.

The rules this design relies on:

- **core.49**: no drive's backlog, slow answer, refusal or failure delays another drive's launches, acts or reports; a shared cap is its owner's.
- **core.2**: the control plane is reached through one interface; no code outside an implementation names crew or the hosted protocol.
- **core.19** and **core.26**: one control plane, several accounts, and each account's connection carrying its own work alone.
- **core.9**: a launch is taken only while its stream has room, as Flume's status gives it.
- **core.10**, **core.25**, **core.35** and **core.36**: each act and command once across restarts, through the one report file, which only gains fields.
- **core.14**: the runner alone fetches a clone and moves its view; the tools work in its container.
- **core.22**, **core.23** and **core.50**: the control timer, its reach said once each way, and launches ahead of acts within a pass.
- **core.12** and **core.41**: names that say what they hold, and neutral names in tests.

This design departs from core.49 in one place, under the hosted implementation, and says why in Decision 8.

## Goals / Non-Goals

**Goals**

- Each part of the control plane, a crew partition or a hosted partition, is pulled, answered and reported on a thread and a timer of its own.
- No part's control pass waits on a keeping pass, and a landing waits on no more than the repository in hand.
- What parts share is a tool's cap or the runner's own short synchronization, each named here: Flume's room, a clone's lock and the report file's write.

**Non-Goals**

- **Drives within one hosted account.** Protocol version 1 names no drive. Splitting an account's drives is the hosted control plane's design (Decision 8).
- **The configuration's accounts and drives.** The runner's own grouping of its repositories is the change `a-hosts-configuration-names-its-accounts-and-their-drives`, on another bolt. This design reads only the implementations' partitions, which that change leaves as they are.
- **A keeping pass per drive.** The keeping pass stays the host's, one pass over every repository.
- **A status command.** The runner has none. Each part's lines name it in the log.
- **Messages.** No protocol message and no field of the protocol changes (core.42).

## Decisions

### 1. An implementation serves its control plane as parts

`boomdrive-control` gains a part:

```rust
/// A part of the control plane the runner serves on its own (record
/// 0026; core.49).
pub struct Part {
    /// The implementation's name for it, given once among its parts.
    pub label: String,
    pub plane: Box<dyn ControlPlane + Send>,
}
```

`ControlPlane` gains `fn owns(&self, id: &Id) -> bool`: whether `id` is one this control plane gives. The runner reads no id (the control-plane-interface spec). It keeps each part's share of its report by asking the part.

An implementation's opener gives `Vec<Part>`, at least one, each label once, in its table's order. It is handed, for each part it makes, a wake of that part's own: `wake_for: &dyn Fn(&str) -> Wake`, called with the part's label. It is handed `say_for: &dyn Fn() -> Say` the same way, so each part says its lines through a say of its own, sent from its own thread. `planes.rs` stays the one list, naming each implementation by its word and its opener alone (core.2).

- **crew's implementation** gives one part for each partition of its table, its label the partition's. A partition is a drive (record 0028). Decision 7 has the rest.
- **The hosted implementation** gives one part for each partition of its table, its label the partition's. A partition is an account's enrollment and its one connection (records 0018 and 0028). Decision 8 has the rest.
- **The fake control plane** is one part. A test makes as many as it needs. `Fake::owns` holds every id unless the test names a prefix, `Fake::owning("<label>/")`.

The shared cases gain one: every want a part gives carries an id the part owns. Each implementation runs it.

Not taken:

- **The runner splitting one pull's wants by id or by repository.** It reads no id. Without the configuration's drives it knows no repository's drive, and a hosted account's repositories are the hosted table's, which only its implementation reads (core.2).
- **A runner per part.** Record 0026's option 3: a process per drive, and two runners fetching one clone.
- **Each implementation pulling its partitions side by side inside one pull.** Every partition's wants would still be answered in one pass, so one drive's acts would still hold another's launches.

### 2. Each part's control passes run on a thread of their own

Where the configuration names a control plane and it opens, the runner starts a thread for each part once its first keeping pass has ended. The thread makes the part's control passes until the runner stops:

- **Its own timer.** A pass is due `poll_interval` after the start of the part's pass before, sooner by its own leases (core.22, the running spec's control timer), and at once on its own wake.
- **Its own pass.** Each pass reads the configuration, the clones and Flume's listing, builds the part's hello, pulls, answers the part's wants in the control plane's order, carries the part's statuses and sluice entries, opens its terminals, and reports. That is `Carrier::pass` as it stands, over one part.
- **Its own due.** The pass's `due` is the part's own: `poll_interval` since the part's pass began, or the part woken since it began. So core.50 holds within each part, and a part's backlog of acts waits only for that part's next pass.
- **Its own reach.** Each part keeps its own `Reach`. Its lines name its label: `boomdrive: <label>: the control plane could not be read: <words>`, and `boomdrive: <label>: the control plane is read again` (core.23). Every other line the runner says of a part's pass names the label after `boomdrive:`. A line about a want keeps its id first, and the id is already the part's own.
- **Its own terminals.** Each part keeps its own `Relays`, which close the terminals of a launch the part lets go.

The runner's say is shared by the loop and every part's thread, `Send` and `Sync`, and each line is said whole.

A stop begins no further pass on any thread. Each part ends the pass in hand, its report saved, and `keep` waits for every part's thread before it says it stopped. A part's pass does not read the stop between its wants, as a control pass does not today.

`run --once` makes its keeping pass, then every part's one control pass side by side, and waits for each. Each part's pass carries out every act, since no pass follows it (core.50). It exits 0 where every git call of the keeping pass succeeded and at least one part both pulled and reported. That is the meaning it has today, when an implementation's pull is unread only where no partition is read, and its report unsent only where no partition took it.

Not taken:

- **An async runtime in the binary.** A pass runs tools' command lines and git, each a process the pass waits for. A thread per part is plain, and a host serves a handful of drives.
- **A fixed pool of threads for every part.** A part would wait for a free thread behind another's backlog. That is a cap across drives no record decided (record 0026).
- **A part's first pass at once, alongside the first keeping pass.** A launch at start would wait a whole `poll_interval` for its clone. The first keeping pass gates only the first control pass, once.

### 3. The loop keeps the host's own work, and serves a landing between repositories

The loop's thread keeps what is the host's and no part's: the keeping passes on `fetch_interval`, the landings it is told of, and its stop. It no longer runs a control pass, so a slow fetch holds no part's pull, act or report.

A landing a part's pass reads is told to the loop, as today (the work-into-flume spec's "A landing is told to the keeper once"). The loop keeps it between passes, and also between two repositories of a keeping pass under way. `pass::pass` takes a hook it runs after each repository it keeps, which the loop uses to keep the landings told since. So a landing waits for no more than the repository in hand, and not for the whole keeping pass. That is record 0026's "a repository's one clone is fetched by the runner under a synchronization of its own, never one drive waiting for another's whole pass".

Not taken:

- **A part keeping its own landing.** A part would then fetch clones and move views beside the keeper, a second writer of every clone.
- **A thread for landings.** That would also be a second writer, and needs Decision 4's lock on every step of a keep. Serving landings between repositories bounds the wait as well, with one writer.

### 4. Each clone has a lock of its own

The runner holds one lock for each address it keeps, `Locks`, compared without case as `Address` compares. It is a reader-writer lock: the keeper takes it alone, and launches share it.

- **The keeper** takes a repository's lock while it keeps or lets go of it: its clone made or fetched, its HEAD settled, its view kept, or its clone and view removed. A landing's keep takes it the same way.
- **A part's pass** takes it, shared, for a launch into the repository's clone. It holds it from finding the clone made to the end of `flume launch`, and lets it go before the answer is saved. Once it holds the lock, it finds the clone again. A clone the keeper let go meanwhile is one not yet made, and the offer is left unanswered, as one whose clone is not made.

Nothing else takes the lock:

- **A tool's command** on a place runs in the clone's container as the tool's own (core.14).
- **The runner's reads of a clone** read at origin's tracking ref (core.14). Those are an inbox read and a branch read for a command. A read that fails because the keeper removed the clone meanwhile is named, and carried at the part's next pass.

The lock is held for one repository's keep or one launch, never across a pass, a pull or a report. Two launches into one clone, of one part or of two, never wait on each other: Flume keeps its own launches apart. So a launch waits only for the keeper's keep of its own repository, and the keeper only for the launches into it under way.

Not taken: the keeper as the clones' one thread alone, with no lock. A part's launch could then make a worktree in a clone the keeper is removing as dropped, between the keeper's check of its worktrees and its removal (record 0021; core.14).

### 5. One report file, each part's share of it

The runner keeps one report, `<data>/control/report.json`, in its form of today: the whole report, its launches, acts, commands and inboxes by id. core.35 names one report, and the release before reads this file (core.36; record 0005).

- **The file gains one field**, `parts`: the labels of the parts whose share it holds. A file with no `parts`, which the release before wrote, holds every part's share.
- **A part's share** is the file's entries its control plane owns (Decision 1). A part reads its share at the start of its pass, where the file lists it. A part the file does not list, the file missing among it, takes its share from its pull's report, as a pass takes the whole report where the file is missing today (the work-into-flume spec's "Each launch, act and command is carried out once").
- **A part saves its share** after every want that changes it, as a pass saves today. The runner takes the file's own mutex, reads the file, replaces the entries the part owns with the part's, keeps every other entry as it stands, adds the part's label to `parts`, and writes the whole through a temporary file and a rename. It lets go of the mutex once the rename is done.
- **An entry no part owns**, such as one of a partition taken out of the table, is kept as it stands, as today it is kept and never let go.

The mutex is held for one read and one write of the runner's own file, never across a tool's call, a pull or a report. So no part's backlog, slow answer, refusal or failure can hold another's save. A write that fails, a full disk among it, fails each part's save in its own pass (core.25).

Not taken:

- **One file per part.** A host rolled back one release would read a whole file left as it stood at the upgrade, and carry out again every act and command answered since (core.10, core.36).
- **The file as a table of parts.** The release before would read the top-level maps as empty.

### 6. Each part counts its own room

Each part's pass reads room from its own listing, Flume's status read before its pull, and counts its own launches against it, as a pass does today (the work-into-flume spec's "Room is read from Flume's status and counted down within a pass").

Two parts that read one room at once may each take it. Flume's own bound then holds: the later place waits for room in its stream, in Flume's order (Flume's 70). Each drive has a stream of its own (record 0028), so two parts meet only at the host's room.

core.9's details gain this: two parts that read one room at once each taking it, Flume holding the later place waiting for room.

Not taken: a room counted across parts by the runner. Record 0026 rules out a line of the runner's own in front of a cap a tool owns: "The host's agents are Flume's bound and its streams' room".

### 7. crew's parts: each partition with a cache of its own

`boomdrive_crew::open` gives one `Crew` for each partition of its table, each holding its one partition, the table's `host` and a say of its own. `Crew` can still serve several partitions in turn, as crew's own tests make it. The runner never opens it so.

- **A cache for each partition.** A partition's state repository is reached through a bare cache of its own, `<data>/control/crew/<owner>/<name>/<label>.git`. Two drives of one account share a state repository, but no git directory and no weir lock, which keeps one git directory's writers in turn (weir's `lock.rs`). So one drive's slow fetch or push holds no other drive's pull or report.
- **The shared cache of before**, `<data>/control/crew/<owner>/<name>.git`, is removed at open where it is there, with one line naming it. A cache is made again from the control plane (core.35).
- **A part unread is its own.** A part whose `<label>/main` cannot be read gives `Unread`, naming it. A part whose runner branch cannot be written gives `Unsent`, naming it. Every other part pulls and reports at its own pace.
- **What a part owns.** `Crew::owns` holds an id `ids::named` reads as one of its partitions'.

### 8. The hosted implementation's parts: each account's connection

`boomdrive_hosted::open` gives one `Hosted` for each partition of its table, each holding its one connection. The parts share one runtime, held by each part's `Arc`, and the host's credential store. Dropping a part closes its socket. Dropping the last stops the runtime.

- **A pull** hands the part's hello to its own connection, and waits up to 10 seconds for that connection alone. It is unread where its connection is not read. Its words no longer begin with the label, which the runner's line names.
- **A report** goes to the part's own connection, and waits up to 10 seconds for its `reported`.
- **A wake.** A `pulled` that comes on a connection calls the wake of that connection's part alone.
- **What a part owns.** `Hosted::owns` holds an id under its label, `<label>/<id>`.

Each account's connection carries the work of every drive of that account, in the hosted control plane's order (core.26; record 0028). Protocol version 1 names no drive on a want, so the runner cannot tell one account's drives apart. Within an account, launches go ahead of acts on places, and acts wait for the next pass once it is due (core.50).

This departs from core.49, under the hosted implementation, for two drives of one account: one drive's backlog of acts on places can hold that account's next pull. The runner reads no id and knows no hosted repository's drive. Telling them apart needs a protocol version that names a want's drive (core.42), agreed with boomdrive-cloud, whose control plane holds the account's order. crew's implementation, which deneb runs, gives each drive a part of its own, and does not depart. The proposal tells the orchestrator.

### 9. The record and the rules

The commit holding this change also holds:

- **Record 0026** gains an amendment of 2026-11-07, holding Decisions 1 to 8 as this design's own choices within the record as accepted. They are cheap to change, and the record already decided each question a person had to.
- **core.49**'s details name what the parts share: a clone's lock for one keep or one launch, the report file's write, and Flume's room. They also name the hosted account's drives, sharing its connection.
- **core.9**'s details allow two parts to take one room read at once.
- **core.23**'s details allow each part's reach said on its own.

## Risks / Trade-offs

- **Every pass of every part runs the hello's reads.** These are `flume --json status` and each tool's `--version`. A host with four drives runs them four times per `poll_interval`. Each is a read, and Flume's status settles nothing.
- **Two parts may each take the host's last room**, and Flume holds the later place waiting for room. The runner then holds a launch it could not start at once, which core.9 would have it leave for another host. On a host whose drives have streams of their own, that happens only at the host's bound.
- **A stop waits for the slowest part's pass in hand**, as it waits today for the one control pass.
- **Sibling changes on other bolts edit `run.rs`, `pass.rs` and the running spec.**
  - `each-pass-ends-with-a-settle`, on `bolt/the-runner-keeps-the-hosts-watch`, ends each turn of the loop with Flume's settle. The settle is the host's watch, no part's. Where both land, it stays with the loop's turn, on its own bound, and no part's pass runs it or waits on it.
  - `a-hosts-configuration-names-its-accounts-and-their-drives`, on `bolt/the-boomdrive-flake-configures-the-whole-host`, changes the keeping pass and the configuration's types. Its Decision 9 keeps the implementations' partitions, which this design reads.
  - Whichever lands second takes the other's shape.
- **The hosted implementation's drives of one account share a part** (Decision 8).

## Migration Plan

1. On the first start of this release, the report file has no `parts`, so it holds every part's share. Each part's first save adds its label.
2. crew's implementation fetches each partition's state into its own cache at its first pull, and removes the shared cache of the release before at open.
3. Rollback: the release before reads `report.json` whole and ignores `parts`, and fetches its shared cache again.
