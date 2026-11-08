# Design

## Context

See proposal.md for why. This change drains a dropped repository while a place the runner launched there has not ended, follows a place only through its repository's clone, and has crew's implementation give a launch only for a repository the runner keeps or drains, as record 0021 decides.

What stands, at `3f428cf`:

- **The keeper**, `pass::pass` (`crates/boomdrive/src/pass.rs`), runs on the fetch timer. It keeps each repository the configuration names, then lets go of each clone `clone::find` gives whose address the configuration does not name (`let_go`). A dropped clone is never fetched. It is removed, view first, once `lacks::holdings` finds nothing the forge lacks: no ref ahead of origin, no worktree of the clone but its view, and nothing in the view. A Flume place is a worktree of the clone, so it holds the clone while it stands. Once Flume tidies it, keeping its record `<location>@<at>` while an end is pending (Flume's 62), nothing holds the clone, and the next pass removes it.
- **The control pass**, `Carrier::pass` (`crates/boomdrive/src/carry.rs`), runs on the control timer, on the same thread. `Clones::keeping` gives `Ready(container)`, `NotYet` or `NotNamed`, from the configuration and the clones made. `Work::offer` answers an offer already answered `launched` from the report, then reads Flume's listing for the offer's mark, and only then refuses an offer whose repository is `NotNamed`. `carry_statuses` asks `flume --json status --repository <container> <place>` where `Ready`, and `flume --json status <place>` otherwise (the fix `the-runner-follows-each-place-it-launched-to-its-end`). It tells the keeper of a landing only where `Ready`. `carry_entries`, `branch` and `inbox` act only where `Ready`. `act_on` asks `flume --json status <place>` with no repository where the listing does not hold the place, as core.51 has it, since the act's command names none.
- **The report**, `boomdrive_control::Report`, kept at `<data>/control/report.json`, holds each launch's lease, answer and carried status and entry, and not the repository it was launched in. The hosted implementation sends each launch's fields to version 1 by name (`Plane::upward`), and crew's writes crew's own formats.
- **The hello** names `repositories`, each address the configuration names whose clone is made. crew's implementation (`pull_partition`, `crates/boomdrive-crew/src/lib.rs`) resolves a launch's `Repo` against them with `read::kept`. Where none has it, it still gives a launch the runner's report or branch answers, its `Repo` as written (record 0021's option 1, which the record rejects). A launch naming this host is given whatever its `Repo`, with its repository resolved where it can be, else `Repo` as written. The hosted implementation sends each partition the `repositories` its table names (`Plane::hello`), into version 1's closed `hello`.

There are no area pages, so `docs/architecture/core.md` binds every path this change touches.

## Goals / Non-Goals

**Goals:**

- A repository the configuration drops while a place the runner launched there has not ended keeps its clone and view, with nothing more launched there, until every such place has ended, and is then removed (record 0021; core.5).
- A place is followed only through its repository's clone, and crew's implementation gives a launch only for a repository the runner keeps or drains.
- No old name is matched to a new one, by the runner or by crew's implementation.

**Non-Goals:**

- **A rename handled in flight.** The user ruled it out. A rename is a drain, a removal, the rename on the forge, and an addition.
- **Draining under the hosted implementation's version 1.** Its `hello` names no drained repository, and a new version is a change of its own with the cloud (core.42). The runner still drains the clone, and follows a launch the cloud gives there.
- **Draining a launch the report does not hold.** A launch the control plane lets go, or whose lease it refuses, leaves the report, and its place is left as Flume holds it (core.10).
- **A place followed after it has ended.** A tidied place's record may stand for a while, with an end pending. Its tidied status, with its landing, is what crew reads, so it is the last the runner carries.

## Decisions

### 1. The report records each launch's repository

`boomdrive_control::Launched` gains `repository: Option<String>`: the repository the offer named, as the offer wrote it. It is set in `Work::answer` where the report holds none, so the first answer sets it and no later pull changes it. A launch a report of before holds without it gains it at the first pull that gives the launch, in the early return `Work::offer` takes for one answered `launched`. It is `#[serde(default, skip_serializing_if = "Option::is_none")]`, so a report file of before reads, and the release before ignores the field (core.36). The hosted implementation sends version 1 the fields it names and takes none back (core.27, core.42). crew's implementation writes crew's formats, and gives `None` when it reads a report back from its branch.

Alternatives:

- **The place's repository read from Flume's status.** Not taken: the carried status names the repository as a path, which need not match the clone's container byte for byte. A launch whose first status is not yet carried would also drain nothing.
- **A map of the runner's own beside the report, in its file.** Not taken: two records of one launch, and the runner's file no longer the interface's report.
- **A record in the clone's git configuration at each launch.** Not taken: a write to the clone at every launch, and records to tidy in every repository the runner keeps.

### 2. A dropped repository is drained while a place the runner launched there has not ended

`report::holding(report, address) -> Vec<(Id, String)>`, in `crates/boomdrive/src/report.rs`, gives each launch of the report, with its place, that holds `address`. A launch holds an address when its `repository` parses to that address (`config::Address`, compared as it compares), its answer is `launched` at a place, and that place has not ended. A place has ended once the status carried of it stands not listed, or its entry there, by `facts::entry`, reads its phase `tidied` or `gone`, by a new `facts::ended`. A place with no status carried yet has not ended. One function serves the keeper, the hello and the control pass, so all three read draining the same way.

The keeper's `pass` reads the report file once, before it lets any dropped clone go. `let_go` takes the launches holding the clone's address beside `lacks::holdings`, and names each `{address}: left: held by the place {place} of {id}, which has not ended`. A drained clone is neither fetched nor moved, as every dropped clone already is. Once nothing holds it, it is removed as before. Where there is no report file, nothing drains. Where the file cannot be read, no dropped clone is let go that pass, each named `{address}: not let go: <why>`, and the pass fails.

Alternatives:

- **A place ends once its status reads it over.** Not taken: its worktree may stand until Flume tidies it, and once it does, the keeper could remove the clone before the control pass carries the tidied record. A tidied status is never lost this way.
- **A place ends once Flume keeps nothing of it.** Not taken: a tidied record stands while an end is pending for a recipient, which may never take it, and the clone would stay for it.
- **The keeper reads Flume's listing.** Not taken: the keeper runs only git, and the report already holds what Flume last said of each place.

### 3. The hello names the repositories drained

`boomdrive_control::Hello` gains `draining: Vec<String>`: each address the runner keeps the clone of while the configuration no longer names it, because a place it launched there has not ended. No launch is taken in one. `#[serde(default)]`. `Clones` gains `Keeping::Dropped(PathBuf)`, a clone made whose address the configuration does not name, and `fn dropped(&self) -> Vec<String>`, each such address. `hello::draining` gives the drained addresses: those of `dropped()` that `report::holding` finds held in the report the pass read before its pull. The pass sets them on the hello `hello::hello` makes, whose signature stays as it was, so its own tests stand. `INTERFACE_VERSION` stays 1: nothing outside the binary reads the interface's shapes, and the hosted implementation's version 1 is unchanged. Its `Plane::hello` builds `repositories` from the hello's `repositories` alone, so no drained repository goes up.

Alternatives:

- **The drained repositories among the hello's `repositories`.** Not taken: crew's implementation would claim new launches there, and the cloud would read them as room.
- **crew's implementation reads draining from the report it is handed.** Not taken: it would judge a place's end itself, a second reading of what the runner owns.

### 4. Nothing more is launched in a drained repository, and only kept work is taken

`Work` reads `Keeping::Dropped(container)` as drained where `report::holding` finds the address held in the pass's report. The order of `Work::offer` becomes:

1. An offer the report answers `launched` keeps its answer, and gains its repository where the report holds none (Decision 1).
2. An offer under a lapsed lease, or with no listing, waits as now.
3. An offer for a repository the runner neither keeps nor drains, `NotNamed` or `Dropped` not drained, is refused, naming the repository, before any mark is read. A launch is taken only for a repository the runner keeps (record 0021).
4. An offer Flume's listing holds a place for under its mark is answered `launched` at it, as now, where the repository is kept or drained, so a launch made just before a stop is not made twice (core.10).
5. An offer in a drained repository is refused with the words a repository not kept gets: nothing more is launched there.
6. `NotYet` waits, and the rest is as now.

`inbox` reads `Dropped` as `NotNamed`. A command reads no repository: `Work::command` refuses every command, naming its tool, a branch command's being sluice (core.52). An act on a launched place is carried out as now, by its location: a drop is how a person ends a place sooner.

Alternatives:

- **Marks read before the keeping, as now.** Not taken: an offer in a repository with no clone would be answered `launched` at a place no pass can follow.
- **An offer in a drained repository held.** Not taken: a hold is for an offer that will be launched, and none will be there.

### 5. A place is followed only through its repository's clone

`carry_statuses` and `carry_entries` follow a launch where its repository is `Ready(container)` or `Dropped(container)`, and skip it otherwise, its last status standing as it was. `carry_statuses` always passes `Some(container)` to `Flume::status_of`. The landing notice goes only where `Ready`, since the keeper fetches no dropped repository. `Flume::status_of` keeps its `Option`, which `act_on` needs (core.51). The follow is on any clone that stands, named or dropped. A dropped clone that stands holds a place that has not ended, or work the forge lacks, and in either case reading Flume or sluice there changes nothing.

Alternatives:

- **A dropped clone followed only while drained.** Not taken: it would be a second test of the same report, and a clone held by work the forge lacks gives the same answers.

### 6. crew's implementation gives a launch only for a repository the runner keeps or drains

In `pull_partition`:

- **A launch naming no `Host` that no runner answered** resolves its `Repo` against `hello.repositories` alone, as now. A drained repository gives nothing.
- **One the runner's report or this host's branch answers**, a claim apart, resolves its `Repo` against `hello.repositories`, and where that gives `Kept::No`, against `hello.draining`. `Kept::Address` is given, its repository the address, neither claimed nor named. `Kept::Shared` gives nothing and says both addresses, as an unanswered one does. `Kept::No` from both gives nothing and says nothing, and the arm that gave such a launch with its `Repo` as written goes.
- **A launch naming this host** resolves its `Repo` the same way, repositories then drained, else its `Repo` as written, so a launch answered in a drained repository is given at its address.

Alternatives:

- **The repositories and the drained ones read as one list.** Not taken: a name alone that a kept and a drained repository share would then give nothing, though the launch is the kept one's.

### 7. The tests

Every test runs on the fakes, stubs and scratch host of its crate, with neutral names and paths (core.41), in a temporary folder.

- `crates/boomdrive-control/tests/json.rs`: `a_report` and `a_hello` round-trip the new fields; `a_report_from_before_reads` reads a report with no `repository` and a hello with no `draining`.
- `crates/boomdrive/src/facts.rs`: `a_place_that_has_ended` reads `tidied` and `gone` ended, and `live`, `over` and a delivered wait not.
- `crates/boomdrive/src/report.rs`: `the_places_that_hold_a_repository` gives a launched place not ended, of the address, and not one ended, not listed, of another address, held or refused, or with no repository.
- `crates/boomdrive/tests/fakes/mod.rs`: `FakeClones::drop_named` keeps a made clone as `Dropped(container)`. `FakeClones::gone` forgets the address altogether. `dropped()` gives each `Dropped`.
- `crates/boomdrive/tests/offers.rs`:
  - `a_repository_draining_in_the_hello`: the hello names the drained repository apart;
  - `a_repository_drained`: a new offer there is refused, naming it, and no `flume launch` is run;
  - `a_mark_in_a_repository_not_kept`: refused before its mark is read;
  - `a_launchs_repository_kept_with_its_answer`: the report names the offer's repository; one of before gains it at the next pull.
- `crates/boomdrive/tests/statuses.rs`:
  - `a_place_in_a_repository_drained`: a launch's place lands, then is tidied with an end pending. Each status is asked for with `--repository` the dropped clone's container and carried, and no keeper is told.
  - `a_place_in_a_repository_with_no_clone`: no `status <place>` is run, and the carried status stands.
  - The tests `a_place_whose_repository_the_runner_keeps_no_more` and `a_place_found_tidied_by_its_mark` go with the requirement they proved.
- `crates/boomdrive/tests/entries.rs`: `an_entry_carried_from_a_repository_drained` runs `sluice status <id> --json` in the dropped clone's container.
- `crates/boomdrive/tests/dropped.rs`, on the scratch host, with a report file written into `<data>/control/report.json`:
  - `dropped_while_a_place_is_live`: kept, named held by the place;
  - `the_place_tidied` and `the_place_no_longer_listed`: removed at the next pass;
  - `a_report_that_cannot_be_read`: kept, the report file named.
- `crates/boomdrive-crew/tests/claims.rs`:
  - `an_unanswered_launch_in_a_repository_draining`: given nothing;
  - `a_launch_answered_in_a_repository_draining`: given at its address, nothing written or said;
  - `a_launch_answered_before_its_repository_was_renamed`: given nothing;
  - `a_launch_naming_this_host_in_a_repository_draining`: its offer names the address.
  - The tests `a_launch_answered_before_its_repository_left_the_hello` and `a_launch_the_report_answers_is_given_whatever_the_hello_names` go with the requirement they proved.
- `crates/boomdrive-hosted/tests/plane.rs`: `a_repository_draining_goes_up_in_no_hello`.

### Rules relied on

- **core.5**, whose "Not ruled out" names a dropped repository's clone and view kept, with nothing launched in it, while a place the runner launched there is live, then removed (record 0021). The drained clone is never fetched, moved or made again.
- **core.1** and **core.9**: work is taken only as offered, and only for a repository kept.
- **core.2**: crew's implementation reads the hello's drained repositories through the interface, and no code outside an implementation names crew.
- **core.10**: a launch the control plane lets go drains nothing, and its place is left as Flume holds it; a launch found by its mark is not made twice.
- **core.11** and **core.27**: Flume's and sluice's answers are carried as given, through whichever clone stands.
- **core.14**: the runner removes only its own clone and view, and reads the clone without moving a ref.
- **core.26**: what goes up to a hosted partition names its own repositories alone, and no drained one.
- **core.35** and **core.36**: the report file gains a field, read as empty where it is missing.
- **core.42**: version 1 of the protocol is unchanged.
- **core.51**: an act's status check still names no repository.
- **core.41**: the tests name no host, account or person's path.

### Rules departed from

None.

## Risks / Trade-offs

- **A launch the control plane neither gives nor lets go keeps its clone.** Its status is never carried to an end. The keeper names the place and the launch that hold the clone at every pass, so a person can drop the launch or the place.
- **A repository dropped before this release, whose place's worktree is gone, is not drained.** Its launch has no repository recorded, and crew's implementation gives it nothing once the hello no longer names its repository. Flume's record of the place stands, and crew reads the place as last carried.
- **A report file that cannot be read keeps every dropped clone.** It is named, and the control pass refuses to carry anything until it reads, as now.

## Migration Plan

Nothing on a host moves. At the first pull of this release that gives a launch, the report records its repository. From then on, a repository the configuration drops while a place there has not ended is drained.

## After deploy

Nothing. No step of this change is outside the gate.

## Task notes

- **1.1** The interface's two fields; every construction of `Launched` and `Hello` in every crate names them, `None` and `vec![]` where nothing else is meant.
- **2.1** `facts::ended`. **2.2** `report::holding`.
- **3.1** `Keeping::Dropped`, `Clones::dropped`, `Configured`'s, and the fakes.
- **3.2** Decision 1. **3.3** Decision 3. **3.4** and **3.5** Decision 4. **3.6** to **3.8** Decision 5.
- **4.1** `pass::let_go` per Decision 2.
- **5.1** `pull_partition` per Decision 6.
- **6.1** `Plane::hello` needs no change; the test pins it.
