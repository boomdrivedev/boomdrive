# Proposal

## Why

The brief: "Each drive's runner branch on a host names, in crew's repositories.rec, exactly the repositories of that drive the runner keeps there at each report, by the forge address its configuration names with the folder it gives Flume, and no other drive's or account's, so crew can tell where a launch of any of the drive's kits would be made."

crew reads a fifth file on each runner's branch, `repositories.rec`: each repository the runner keeps on its host, by its forge address, with the folder of its clone, where Flume cuts its places. crew's `plugin/lib/runner.py` holds its descriptor, and its `drive-state` spec reads it in "A kit's work is taken by a runner whose configuration names its repository". crew finds a started bolt's kit folder there, and the repositories a launch of a kit would be made in, so it can check that the branch a new unit would cut is free. No runner writes the file. `boomdrive-crew`'s `write.rs` writes `runner.rec`, `answers.rec`, `places.rec` and, where a preview is or was, `previews.rec`, and nothing else. The signal `2026-11-09-agentplot-operator-966bfd4a/01`, "crew needs a repositories file no runner writes", names the gap. So crew falls back to the folder Flume's status of a place names, and where the runner carries no place of the kit, it can tell no folder at all (crew's open change `a-started-bolts-kit-folder-is-read-as-main-reads-it`).

The runner knows each repository its configuration keeps and where its clone is. It does not tell crew's implementation either. The hello names the addresses whose clones are made, every drive's together, and no folder and no drive. A crew partition is a drive of this host: a drive's name is its crew branch's (record 0028), and the runner's crew table is built from the host's drives (record 0032). So crew's implementation can name a drive's repositories on that drive's branch once the hello says which repositories are each drive's, and where each clone is.

## What Changes

- **The hello names each drive's repositories with their folders.** `Hello` gains `drives`: each drive of the configuration, by its name, with each repository it names whose clone is made, in the configuration's order. Each is named by its address as the configuration writes it, with its clone's container, the folder `flume launch --repository` names. A drive none of whose clones is made yet is named with none. A hello's JSON with no `drives` reads as naming none. `repositories` and `draining` are unchanged.
- **The runner fills it.** `Clones` gains `drives`. The control pass sets the hello's `drives` from it, as it sets `draining`. A repository the runner drains is in no drive, since the configuration no longer names it.
- **crew's implementation writes `repositories.rec`.** At every write of a partition's branch, `<label>/runner/<host>`, it writes `repositories.rec` under crew's descriptor, byte for byte as crew's `runner.py` holds it. The file holds one `Repository` for each repository the hello names under the drive named by the partition's label, in the hello's order. Each record has its `Repository`, the address, and its `Folder`, the clone's container. It names no other drive's repository and none the runner drains. A report takes the drives from its partition's latest pull. A pull that claims or withdraws takes them from its own hello. A partition whose label names no drive of the hello writes the descriptor alone, and says one line naming the label and the drives there are. A write's commit names each address whose record changed.
- **Which runner takes a kit's launch is unchanged.** A launch naming no host is still claimed where the hello's `repositories`, every drive's, names its `Repo` (design Decision 5).
- **The hosted implementation sends no drive and no folder.** Version 2's hello names neither, and the protocol does not change (core.42).
- README's crew section names the new file.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `work-into-flume`: every pull's hello names each drive's repositories with their clones' containers.
- `control-plane-interface`: a hello carries its drives through JSON, and one with none reads as naming none.
- `crew-control-plane`: the runner's branch holds `repositories.rec`, naming exactly the repositories of the partition's drive the runner keeps, with their folders, at every write; a partition whose label names no drive names none.
- `hosted-control-plane`: no hello names a drive or a clone's folder.

## Impact

- `crates/boomdrive-control`:
  - `src/lib.rs`: `Hello::drives` and the type of a drive's kept repository;
  - `src/shared.rs`: the hellos the shared cases build;
  - `tests/json.rs`: a hello with drives, and one of before.
- `crates/boomdrive`:
  - `src/carry.rs`: `Clones::drives`, `Configured` keeping each drive's repositories, and the pass setting the hello's drives;
  - `src/hello.rs`: the hello it builds names no drive until the pass sets them;
  - `tests/fakes/mod.rs`: `FakeClones` records each repository's drive;
  - `tests/offers.rs`: each drive's repositories in the hello;
  - `tests/control.rs`: `boomdrive run --once` writing a drive's `repositories.rec` through crew's implementation;
  - `tests/readme.rs`: README names the file.
- `crates/boomdrive-crew`:
  - `src/write.rs`: the descriptor, the file's text, `Texts`, and what a write names;
  - `src/read.rs`: the file among the runner's files a tip is read for;
  - `src/lib.rs`: the drives of the latest pull, and every write of the branch writing the file;
  - `tests/common/mod.rs`: the hello's drives;
  - `tests/write.rs`, `tests/claims.rs`: the file at a report and at a claim, and a partition naming no drive;
  - `tests/drops.rs`, `tests/letgo.rs`, `tests/read.rs`, `tests/roles.rs`: each hello they build.
- `crates/boomdrive-hosted`: `tests/plane.rs` and `tests/connection.rs`, each hello they build. Its source is unchanged: it builds version 2's hello field by field.
- `README.md`: crew's section.
- Nothing else changes. No configuration key, field of `report.json`, protocol message or rule changes. No record is written: the change carries out records 0028 and 0032 and the rules as they stand.
- Outside this repository: crew reads the file already, at its main `323c77fb`, whose descriptor `0c0acc15` added. A drive whose kit another drive keeps on the host reads in crew as a kit the runner does not keep (design Risk 1, a named follow-up).

## Touches

```
crates/boomdrive-control/src/lib.rs
crates/boomdrive-control/src/shared.rs
crates/boomdrive-control/tests/json.rs
crates/boomdrive/src/carry.rs
crates/boomdrive/src/hello.rs
crates/boomdrive/tests/fakes/mod.rs
crates/boomdrive/tests/offers.rs
crates/boomdrive/tests/control.rs
crates/boomdrive/tests/readme.rs
crates/boomdrive-crew/src/write.rs
crates/boomdrive-crew/src/read.rs
crates/boomdrive-crew/src/lib.rs
crates/boomdrive-crew/tests/common/mod.rs
crates/boomdrive-crew/tests/write.rs
crates/boomdrive-crew/tests/claims.rs
crates/boomdrive-crew/tests/drops.rs
crates/boomdrive-crew/tests/letgo.rs
crates/boomdrive-crew/tests/read.rs
crates/boomdrive-crew/tests/roles.rs
crates/boomdrive-hosted/tests/plane.rs
crates/boomdrive-hosted/tests/connection.rs
README.md
openspec/specs/work-into-flume/spec.md
openspec/specs/control-plane-interface/spec.md
openspec/specs/crew-control-plane/spec.md
openspec/specs/hosted-control-plane/spec.md
```
