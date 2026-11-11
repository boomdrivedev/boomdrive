# Design

## Context

See proposal.md for why. There is no area page for `crates/`, so `docs/architecture/core.md` binds every path this change touches. The change relies on these rules:

- core.2: the runner reaches its control plane through one interface, and no code outside an implementation names crew. The hello's new field is the interface's. Only `boomdrive-crew` names `repositories.rec`, crew's descriptor or a partition's label;
- core.5: the configuration names its accounts, each account's drives and each drive's repositories. Its details allow "the repositories kept reported to the control plane". That allowance says such a report "registers none of them to a drive" (boomdrive-cloud record 0007). This change registers nothing: it reports which drive the configuration names each repository under, and crew registers no repository from it;
- core.11: what the runner carries back is the tool's answer or a reference to it, and a path is not ruled out. A clone's folder is such a path, and no repository content is read;
- core.12: a path's name ends in `_dir`, and no name takes a word another tool uses for something else;
- core.14: the runner's clone is the host's one clone. Its container is the folder Flume is given, `flume launch --repository`;
- core.17: the runner is configured by its configuration alone. No key is added;
- core.26 and core.49: what goes to one account names only that account's repositories, and drives run on their own. Each drive's branch names that drive's repositories alone;
- core.27 and core.28: a hosted control plane is sent only the fields its protocol's copy names. No path leaves the host but in the protocol, git's calls and OTLP;
- core.36: no field of `report.json` changes;
- core.42: the protocol's versions do not change.

It departs from no rule, and changes none.

**The records it carries out.**

- Record 0028: "A drive is a body of work of one account: its repositories, one Flume stream and one branch of the account's crew state." "A repository belongs to one drive." "A drive's name is its stream's and its crew branch's." Its first amendment of 2026-11-07 makes a drive's name unique on the host across its accounts.
- Record 0032: "The runner's crew table is built from the same accounts: crew's host and each drive of this host under an account served by crew, with its account's state repository." So a crew partition's label is a drive's name.
- Record 0021: a dropped repository's clone is kept, drained, while a place the runner launched there is live. No launch is taken in it.

**What crew reads.** At crew's main `323c77fb`, `plugin/lib/runner.py` lists `repositories.rec` among a runner branch's `FILES`, under this descriptor, which `0c0acc15` added:

```
%rec: Repository
%doc: A repository the runner keeps on its host: its forge address as its configuration names it, and the folder of its clone, where Flume cuts its places.
%key: Repository
%mandatory: Repository Folder
%allowed: Repository Folder
```

`host_read` reads each record's `Repository` and `Folder`. A record with no address, or one with no folder, or an address twice, makes that runner unreadable. A branch with no file reads as a runner that reports no repositories. `kept_folder` finds a kit's folder among the taking runner's records, by the kit's address or its name alone. `launch_repositories` lists each runner's folder of a kit, as a place a launch of it could be made, and checks there that the branch a new unit would cut is free. crew's `drive-state` spec, "A kit's work is taken by a runner whose configuration names its repository", reads the file this way. crew's open change `a-started-bolts-kit-folder-is-read-as-main-reads-it` adds the fallback to a place's folder for a runner that reports no repositories. It also keeps refusing a runner that reports its repositories but none by the kit's name. The kit-rename change's R2 asked the runner for the file: "On each partition's branch it writes `repositories.rec` with crew's descriptor. Each record names one repository its configuration keeps, by forge address, and the folder of its clone, where Flume cuts its places."

**What the runner has today.**

- `Hello` (`crates/boomdrive-control/src/lib.rs`) holds `repositories`, the addresses the runner keeps with their clones made, every drive's together, and `draining`. It names no drive and no folder.
- `Configured` (`crates/boomdrive/src/carry.rs`) holds the configuration's addresses and each clone's container. `Clones::made` gives the hello's `repositories`. `Carrier::pass` builds the hello by `hello::hello` and sets `draining` after.
- `Crew` (`crates/boomdrive-crew/src/lib.rs`) is one partition's part (`parts`). Each pull keeps the hello's `runner_version` in `version` for the report's `runner.rec`. The report writes the branch from `write::texts`. A pull that claims or withdraws writes it from `claim_texts`, keeping the tip's other files as they stand.
- `write::Texts` holds `runner.rec`, `answers.rec`, `places.rec` and an optional `previews.rec`. `write::write` writes only the files whose text differs from the tip's, and `what` names what changed. `read::FILES` lists the files a tip is read for.
- The hosted implementation builds version 2's hello field by field from the interface's (`plane.rs`, `hello`), so a field the interface gains goes up only where that code names it.

## Goals / Non-Goals

**Goals:**

- Each drive's branch names exactly the repositories of that drive the runner keeps on its host, each by its address as the configuration writes it and with its clone's container. It names none of another drive's, and none the runner drains.
- The file is written at every write of the branch, so no branch the runner writes lacks it.
- The file opens with crew's descriptor byte for byte, and passes `recfix --check`.
- Nothing the runner holds or sends elsewhere changes shape: not `report.json`, the configuration or the protocol.

**Non-Goals:**

- Which runner takes a kit's launch (Decision 5).
- crew's reading of the file, which is crew's.
- The folders of crew's own teams file on a host, which the set gives crew (record 0036). Those are crew's main level's checkouts. This file is the runner's report of its own clones.
- A hosted control plane's knowledge of drives, which version 2 does not carry.

## Decisions

### 1. The hello names each drive's repositories with their folders

`Hello` gains:

```rust
/// Each drive of the configuration, by its name, with each repository
/// it names whose clone is made, in the configuration's order. A drive
/// none of whose clones is made is named with none. Read as none where
/// a hello lacks it.
#[serde(default)]
pub drives: BTreeMap<String, Vec<KeptRepository>>,

/// A repository a drive keeps, its clone made.
pub struct KeptRepository {
    /// Its address, `owner/name`, as the configuration writes it.
    pub address: String,
    /// Its clone's container, the folder `flume launch --repository`
    /// names.
    pub container_dir: String,
}
```

`KeptRepository` derives as `Hello`'s other parts do, and `boomdrive-control` exports it. `repositories` and `draining` stay as they are. The hosted implementation and crew's matching of a launch read `repositories` and `draining`.

The names follow core.12. `drives` is the runner's word for record 0028's drive, which crew's own word "drive" means too (crew record 0054). `container_dir` ends in `_dir` as a path's name does. "Container" is the clones spec's word for the folder, and `flume.rs` calls the same path `container_dir`.

Not taken:

- **The report carries the repositories.** The report is `report.json` (core.35, core.36) and goes to a hosted control plane as version 2's copy. It holds the runner's answers to its control plane's work. Which repositories a drive keeps is no answer to work, and the hello is where the runner tells its control plane what the host keeps.
- **crew's table names each partition's repositories, as the hosted table does.** A host would write each drive's repositories twice: in its accounts, and in crew's table. Record 0032 names such a fact once. crew's implementation would also have to make each folder from the runner's layout, `<data>/repositories/<owner>/<name>`, which is the runner's own (core.2).
- **The runner hands each part a hello of its drive alone.** The runner does not know which drive a part serves. A hosted part is an account and a crew part is a drive (record 0028), and the implementation alone knows which its labels are.
- **`repositories` carries each address's drive and folder.** Both implementations match launches and filter partitions on `repositories` as addresses. A new field leaves that code as it is, as `draining` did (record 0021).

### 2. A drive names the repositories the configuration names under it whose clones are made

`Clones` gains `fn drives(&self) -> BTreeMap<String, Vec<KeptRepository>>`. `Configured` keeps each drive of the configuration, by its name, with its addresses in the file's order. It gives each drive with each of its addresses whose clone is found, and that clone's container as text. A drive whose clones are none made is given with none. `Carrier::pass` sets `hello.drives` from it, as it sets `hello.draining`.

- **A clone not yet made is not named.** It has no folder, and crew would read one that is not there. `repositories` already names only the clones made.
- **A repository the runner drains is in no drive.** The configuration no longer names it (record 0021). Its launch's place is still followed through its answer and its carried status, which need no record of the file.
- **A drive with none is named.** An implementation can then tell a drive that keeps nothing yet from a label no drive has (Decision 4).

Not taken: **naming a drained repository under the drive its clone records** (`boomdrive.drive`, record 0017). The brief names each repository "by the forge address its configuration names", and the configuration no longer names it. A launch is no longer made there, so crew finds no new place in it.

### 3. crew's part writes `repositories.rec` at every write of its branch

`write.rs` gains `REPOSITORIES`, crew's descriptor at `323c77fb` byte for byte, documented as crew's. It also gains `repositories_text(kept: &[KeptRepository]) -> String`: one record per repository, in the order given, each holding `Repository` with the address and `Folder` with the container. `Texts` gains `repositories: String`, always written, and `by_path` gives it as `repositories.rec`. `what` names `repositories <address>…` for each address whose record is added, changed or removed. `read::FILES` gains `repositories.rec`, so a tip's text of it is read and compared.

`Crew` keeps the drives of its latest hello, as it keeps `version`. At a report, each partition's file is written from the drive named by its label, as the latest pull's hello gave it. At a pull that claims or withdraws, `claim_texts` writes it from the drive named in the hello the pull was handed. So the file is written at every write of the branch, from the hello of the pass that writes it. A pass always pulls before it reports (`Carrier::pass`), so a report's drives are its own pass's.

`host_read`, which reads another runner's branch, reads nothing of that runner's `repositories.rec`. crew's implementation never needs to know where another runner keeps a kit.

Not taken:

- **Written only at a report.** A claim's write can make the branch. That branch would then lack the file until the report, and crew reads a branch with no file as a runner of before.
- **Written only where the drive keeps a repository, as `previews.rec` is written only where a preview is or was.** A file absent reads in crew as a runner that reports no repositories, and crew falls back to a place's folder. This runner does report, and a drive that keeps nothing keeps nothing.

### 4. A partition is a drive by its label

crew's implementation takes the drive whose name is the partition's label, as records 0028 and 0032 make it. Drive names are unique on a host (record 0028's first amendment of 2026-11-07), so no other account's drive can answer to the label.

Where the hello names no drive by the label, the part writes `repositories.rec` as the descriptor alone. That is exactly the repositories of that drive the runner keeps: none. At its pull it says one line, `crew: <label>: this host's configuration names no drive <label>, so <label>/runner/<host> names no repository it keeps; its drives are <names>`, the drives in the hello's order, or `none`. It is said once while it stands, as every line of a pull is. Its launches, acts and claims go on as for any partition.

Not taken:

- **No file for such a partition.** It would read in crew as a runner of before. That hides the fault rather than naming it.
- **Refusing to open the partition.** crew's table opens before any hello, and a hand-written configuration may name a partition before its drive (core.17). Refusing would stop that partition's work, which the line names without stopping.
- **Every drive's repositories on every branch.** The brief: "and no other drive's or account's".

### 5. Which runner takes a kit's launch is unchanged

A launch naming no host is still claimed where `repositories`, every drive's, names its `Repo` (crew-control-plane "A launch naming no host is given only where this runner keeps its repository"). The delta adds a scenario that proves this, "A launch in a repository another drive keeps", under the file's requirement, since the file still names the drive's repositories alone.

Not taken: **a part claims only a launch whose `Repo` its drive keeps.** The claim would then agree with the file. It would also settle a name two drives share, such as two `blueprints`, which is given nothing today. But crew's `drive-state` spec lets a drive's main level act on a kit another drive keeps. Its "The main-level host's checkout of a kit is its teams file entry's, or else in its drive's folder" reads `agentplot/crew` in agentplot's copy for madswan's main level. crew's planning also counts every repository the teams file names. A part that claims only its drive's repositories would leave such a launch untaken by every runner, and crew would say only that no runner has taken it. Which repositories a drive's kits may be is crew's and the design's to settle (Risk 1, a named follow-up). The brief asks for the file alone.

### 6. The hosted implementation sends no drive and no folder

The hosted implementation's `hello` builds version 2's hello field by field. Version 2's schema is closed (core.42), and a folder is a path that may not go up (core.27, core.28). So the code is unchanged. A test proves that the hello a partition sends names neither a drive nor a folder.

## Who owns what

| piece | owner | cited by |
|---|---|---|
| which repositories a drive keeps | the runner's configuration (record 0028; core.5) | the runner, which tells its control plane |
| a clone's container, where Flume cuts its places | the runner (the clones spec; core.14) | Flume, crew |
| the hello's `drives` | the interface, `boomdrive-control` (core.2) | crew's implementation |
| a partition's label, which is a drive's name | records 0028 and 0032 | crew's implementation |
| `repositories.rec`'s descriptor, and what crew reads in it | crew, `plugin/lib/runner.py` at `323c77fb` | crew's implementation, which writes it |
| writing `repositories.rec` on a drive's branch | crew's implementation in this repository (core.2) | crew |
| which runner takes a kit's launch | crew's implementation, unchanged (Decision 5) | crew |

## Named follow-ups

- **The design and agentplot/crew: a drive's kit in a repository another drive keeps.** Once this change lands, a part that takes such a launch reports its drive's repositories, and none by the kit's name. crew then refuses that kit's folder, naming the runner's repositories and `crew kit rename` (crew's open change `a-started-bolts-kit-folder-is-read-as-main-reads-it`). Today, with no file, crew reads that folder from the runner's newest place. There are three ways on. crew's planning could keep a drive's kits to the drive's repositories. A part could claim only its drive's (Decision 5's path not taken). Or the file could name what the part has taken beside its drive's. That is for the design to settle with crew.
- **agentplot/crew:** nothing else. crew reads the file at its main `323c77fb`, and its fallback to a place's folder stays for a runner of before.

Each is told to the orchestrator.

## Risks / Trade-offs

1. **A kit another drive keeps reads as not kept** (the first follow-up). Where a drive's work runs in another drive's repository, crew stops landing that work and names why. Accepted: record 0028 makes a repository one drive's, and the refusal names the runner, what it keeps and the way on. The design settles which drive owns such work.
2. **A label no drive has reads in crew as a runner keeping nothing.** A host whose crew table names a partition that is none of its drives gets an empty file. crew then refuses that partition's kits. Accepted: the module builds the table from the drives (record 0032), and the line names the label and the drives there are.
3. **One commit on every branch at the first write after the upgrade.** It names each address of the drive. It is a commit of crew's state repository and costs nothing else.
4. **A rollback leaves the file as it was.** A runner of before writes only its own files and leaves `repositories.rec` at its last text. crew then reads folders that stand, since a clone's container does not move, until a repository is added or dropped. Accepted: the runner moves forward, and the next write of this release sets it again.

## Migration Plan

Nothing held changes shape. `report.json` keeps every field (core.36). No protocol file changes (core.42). No configuration key is added (core.17).

- **A branch written before** holds no `repositories.rec`. The first write after the upgrade, its first report or claim, adds it, and its commit names each address.
- **A host's data moved by hand** (record 0035) changes each folder. The next write names each address whose `Folder` changed.

## Open Questions

None.

## Task notes

- **1.1.** `KeptRepository` derives `Clone, Debug, PartialEq, Eq, Serialize, Deserialize`, as `HostPart` does. `tests/json.rs`'s `a_hello` builds its hello with `drives` naming `home` with `example/crew` at `/data/repositories/example/crew` and `work` with none, which proves "A hello naming its drives". `a_report_from_before_reads` asserts that its hello of before has no drives, beside `draining`, which proves "A hello from before its drives". `shared.rs`'s `hello` names no drive.
- **1.2.** `crates/boomdrive/src/hello.rs`'s `hello` names no drive: the pass sets them (task 2.2). `crates/boomdrive-crew/tests/common/mod.rs`'s `hello(repositories, room)` names the drive `home` with each repository at `/data/repositories/<address>`. A new `hello_of(drives, room)` takes `&[(&str, &[&str])]`, each drive by its name with its addresses. Its `repositories` is every drive's addresses in order, and each drive's repositories are at `/data/repositories/<address>`. Every other crew test that builds a `Hello` gains `drives`: `home` with its repositories, where it keeps any. The hosted tests' hellos name no drive, but in task 3.1.
- **2.1.** `Configured::new` keeps `drives: Vec<(String, Vec<Address>)>` from `config.drives()`, and `named` from them as now. `drives()` gives `BTreeMap<String, Vec<KeptRepository>>`, each address as `Address`'s `Display` writes it and each container by `to_string_lossy`. The unit test `each_drive_names_its_repositories_whose_clones_are_made` sits in `carry.rs`'s `tests`, beside `a_repository_is_kept_as_the_configuration_and_the_clones_have_it`. Its configuration is one account with the drives `home`, naming `agentplot/flume` then `example/crew`; `work`, naming `example/app`; and `idle`, naming `example/later`. Its clones are `agentplot/flume`, `example/app` and `example/kit`, the last named by no drive. It asserts `home` gives `agentplot/flume` alone at its container, `work` `example/app`, `idle` none, and no drive `example/kit`.
- **2.2.** `FakeClones` keeps each address's drive, `home` by default. `ready_in(address, container, drive)` makes one in another drive, and `drives()` gives each drive with its `Ready` addresses in the order set, dropped and not-yet ones left out. `tests/offers.rs`'s `each_drives_repositories_with_their_folders` sets `agentplot/flume` ready in `home`, `example/crew` not yet in `home`, `example/app` ready in `work`, and `example/kit` dropped. It makes one pass over `Fake::new()` and asserts `plane.hellos()[0].drives` holds `home` with `agentplot/flume` at its container alone and `work` with `example/app` at its container. It also asserts no drive names `example/kit` or `example/crew`. The drive `idle` of the spec's scenario is task 2.1's, since `FakeClones` names no drive with none.
- **3.1.** `tests/plane.rs`'s `no_drive_or_folder_goes_up` builds the test's `hello` with `drives` naming `family` with `example/app` at `/data/repositories/example/app`. It pulls `home` once and reads the first message the fake control plane took from `home`. That message names `example/app` among its repositories. Its JSON text holds neither `family` nor `/data/repositories`.
- **4.1.**
  - `tests/write.rs` holds crew's descriptor as a literal, `REPOSITORIES`, its comment citing crew's `plugin/lib/runner.py` at `323c77fb`, as the other descriptors cite `9913f249`. Its `files` helper reads `repositories.rec` too.
  - `pulled()` pulls with `hello_of(&[("home", &[CREW, KIT]), ("work", &[APP])], "4")`, where `KIT` is `example/kit` and `APP` is `example/app`.
  - `a_drives_repositories_and_their_folders` reports once. It asserts the file opens with `REPOSITORIES` and holds `Repository: example/crew` then `Repository: example/kit`, each followed by its `Folder: /data/repositories/<address>`. It holds no `example/app`, and `recfix` takes it.
  - `nothing_changed` and `only_the_runners_branch` still pass.
- **4.2.** `a_repository_the_drive_comes_to_keep` pulls with `home` naming `CREW` alone and reports, then pulls with `home` naming `CREW` and `KIT` and reports the same report. The second report's commit message, the branch's tip subject, names `repositories example/kit`, and the file holds both. `a_repository_the_drive_no_longer_names` does the reverse, its second hello naming `KIT` in `draining`.
- **4.3.** `tests/claims.rs`'s `a_claim_names_the_drives_repositories` uses the setup of `claimed_alone`, the spec's "Claimed alone", with no branch of `home` for `host-a` before the pull. It asserts the claim's write makes `home/runner/host-a` holding request 20's claim and a `repositories.rec` naming `example/crew` with its folder. `a_launch_in_a_repository_another_drive_keeps` pulls with `hello_of(&[("home", &["example/crew"]), ("work", &["example/app"])], "4")` a launch 20 naming no `Host` and saying `Repo: example/app`. It asserts the pull gives `home/20` with the repository `example/app`, and that the branch's `repositories.rec` names `example/crew` alone.
- **4.4.** `a_label_no_drive_has` pulls with `hello_of(&[("family", &[CREW]), ("work", &[APP])], "4")` and reports. It asserts the file is `REPOSITORIES` alone, and that one line the part said names `home`, `family` and `work`.
- **5.1.** `tests/control.rs`'s `the_runners_branch_names_each_repository_of_its_drive` configures the runner as `carrying("2m", "1m")` does, but with `common::drives(&[("home", &[KEPT]), ("work", &["example/app"])])` where `carrying` writes `keeping(&[KEPT])`, and makes both origins and `state_repository`. It runs `once` with the stub `flume` answering `launch` as `one_control_pass` does. It then asserts `home/runner/host-a:repositories.rec` holds `Repository: agentplot/flume` with `Folder: <scratch.data()>/repositories/agentplot/flume`, and names no `example/app`.
