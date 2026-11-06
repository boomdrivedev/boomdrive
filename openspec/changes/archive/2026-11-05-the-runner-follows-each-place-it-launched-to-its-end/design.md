# Design

## Context

See proposal.md for why. This change has the runner follow each place it launched to its end, whether or not it still keeps the place's repository, and has crew's implementation keep giving the launch.

What stands:

- **This repository**, at `f6d9092`.
  - `Work::carry_statuses` (`crates/boomdrive/src/carry.rs`) reads, for each offer the pull gives that the report holds launched at a place, the place's entry in the pass's listing. Where it differs from the carried one, it asks `flume --json status --repository <container> <place>`. It skips a launch whose repository is not kept with its clone made (`Keeping::Ready`), since it has no container to pass.
  - `facts::entry` reads a tidied place's record, `<location>@<at>`, as the place's entry, and `place_of` gives a tidied record found by its mark the location it stood at.
  - `Work::offer` answers an offer already answered `launched` from the report and runs nothing. An offer it holds, or a new one, for a repository the configuration does not name is refused, naming it.
  - crew's implementation, `pull_partition` (`crates/boomdrive-crew/src/lib.rs`), resolves a kit's launch's `Repo` against the hello's repositories first. Where none is kept (`Kept::No`) or two share the name (`Kept::Shared`), it gives nothing. Only then does it ask whether the runner already answers the launch. A launch naming a `Host` is given whatever its `Repo`.
- **Flume**, at 0.6.4 on the host and in its statement:
  - `flume status [PLACE]`: a place is "a location, or `<kind>--<name>` in the repository". `--repository` is "the repository, where it is not the one the command runs in", which only a `<kind>--<name>` needs.
  - Asked by a place's location, from any folder, with or without `--repository`, and with a `--repository` that does not exist, it answers the place, or its tidied record `<location>@<at>` (clause 58; clause 62).
  - Once every end of a tidied place is delivered, Flume keeps nothing of it, and `status` of it is refused (62).
- **crew**, at `afterthought/crew`'s `plugin/lib/runner.py`: `places.rec` holds `flume status --json <place>`'s answer on one line. `flume_facts` takes the place's entry at its location or at `<location>@…`, its phase, and how its work is over with the commit it landed at. `stage_of` reads a landed place `merged` into its bolt, or `landed` on main.

There are no area pages, so `docs/architecture/core.md` binds every path this change touches.

## Goals / Non-Goals

**Goals:**

- Every place the runner launched is followed, as long as the pull gives its launch, through its end, over or tidied, whether or not the configuration still keeps its repository.
- crew's implementation gives every launch the runner answered.

**Non-Goals:**

- **Matching a renamed repository to its new address.** Which repository a kit is, is crew's, by renaming its kit. Following a place needs no repository.
- **Reading sluice's entry where the runner keeps no clone.** sluice is run in the clone. Flume's status carries the landing, which is what crew reads first.
- **Following a launch the pull no longer gives.** Which work is the host's is the control plane's (core.1).

## Decisions

### 1. A place is asked for by its location where its repository is not kept

`Flume::status_of(container_dir: Option<&Path>, place)`. `Cli` passes `--repository <container>` where there is a container, and the place's location alone where there is none. `carry_statuses` passes the container where `Clones::keeping` gives `Ready`, and none for `NotYet` or `NotNamed`. Everything else stands: the listing decides when the place is read again, a tidied record is the place's entry, and a refusal leaves the last status standing, not listed.

Alternatives:

- **Every place by its location alone.** Not taken: it changes the call for every place the runner keeps, at a release that does not yet check Flume's floor (core.38). Keeping `--repository` where the clone is kept changes nothing for them.
- **The place's own `repository`, as the carried status names it, passed as `--repository`.** Not taken: that path is Flume's, and may no longer exist, as on the host where the old clone was removed. Flume reads a location without it.

### 2. A landing in a repository not kept is told to no keeper

The landing notice is sent only where the pass had a container. The keeper keeps only what the configuration names (core.5), and would otherwise say each time that it does not.

### 3. crew's implementation gives each launch the runner answered

In `pull_partition`, a kit's launch whose `Repo` names no kept address, or a name two kept addresses share, is given still where the runner's report or its branch answers it, claims left out. It is given as `read::want` gives it with no address resolved: its repository its `Repo` as written. It is neither claimed nor named in the log. A launch no runner has answered keeps its rule: given only where the runner keeps its repository.

On the runner's side, an offer it already answered `launched` runs nothing, and only its place is followed. One it holds `held` whose repository it no longer keeps is refused, naming the repository (core.5).

Alternatives:

- **The runner follows each launch its report holds, whether the pull gives it or not.** Not taken: what goes over a partition's connection names only the work it gives (core.26), and a control plane that stops giving a launch lets it go by its answer to the report.

### 4. The tests

Every test runs on the fakes or the stubs of its crate, with neutral names and paths (core.41), and in a temporary folder.

- `crates/boomdrive/tests/statuses.rs`:
  - `a_place_whose_repository_the_runner_keeps_no_more`: a launch's repository leaves the configuration; its place lands, then is tidied with an end pending. Each change is asked for with no container, carried, and told to no keeper.
  - `a_place_found_tidied_by_its_mark`: over the stub `flume`, an offer in a repository not kept whose place Flume 0.4.1's listing holds tidied under its mark (`status.json`'s `fix--gate`) is answered launched at its location. `status-tidied.json` is carried, asked for as `--json status <location>`.
- `crates/boomdrive/tests/flume.rs`: `status_and_one_places_status` asks once with a container and once without, and reads both argv.
- `crates/boomdrive/tests/fakes/mod.rs`: `FakeClones::drop_named`, `FakeFlume::tidy_pending`, and `FakeFlume::entry` reading a tidied record. `Call::StatusOf` holds the container where there was one.
- `crates/boomdrive-crew/tests/claims.rs`:
  - `a_launch_answered_before_its_repository_left_the_hello`: the branch answers a kit's launch whose `Repo` the hello no longer names, and the host has no room. The launch and an act on it are given, its repository `Repo` as written, and nothing is written. An unanswered launch of the same `Repo` is given nothing.
  - `a_launch_the_report_answers_is_given_whatever_the_hello_names`: an address no longer kept, and a name two kept addresses share.

### Rules relied on

- **core.5**, as amended in this change's commit: following a place in Flume's status keeps no clone or view of its repository.
- **core.1**: which work is the host's is the control plane's, so the runner follows only what the pull gives.
- **core.11** and **core.27**: Flume's answer is carried as Flume printed it, whatever repository it was asked in.
- **core.26**: what goes over a partition's connection is the work it gives.
- **core.41**: the tests name no host, no partition and no person's path.

### Rules departed from

None.

## Risks / Trade-offs

- **A launch whose `Repo` is unreadable against the hello is given with its `Repo` as written.** The runner reads no repository from it but to refuse a held one, naming it.
- **A tidied place stays followed until Flume drops its record.** That costs one comparison a pass with the listing, as for any place.

## Migration Plan

Nothing on a host moves. At the first pass of this release, crew's implementation gives a launch the runner answered in a repository it no longer keeps, and the pass carries the place's status, by its location, to the runner's branch.

## After deploy

Nothing. No step of this change is outside the gate.

## Task notes

- **1.1** `Flume::status_of(Option<&Path>, &str)`; `carry_statuses` per Decisions 1 and 2. The fakes as Decision 4 has them.
- **1.2** `pull_partition` per Decision 3.
