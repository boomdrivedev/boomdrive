## ADDED Requirements

### Requirement: repositories.rec names exactly the repositories the partition's drive keeps

`repositories.rec` SHALL open with crew's descriptor of a `Repository`, byte for byte as crew's `runner.py` holds it. It SHALL hold one `Repository` per repository the hello names under the drive whose name is the partition's label, in the hello's order: its address as `Repository`, and the clone's container named with it as `Folder`. It SHALL name no repository of another drive, and none the runner drains (record 0028; record 0032; core.5, core.26).

#### Scenario: A drive's repositories and their folders

- **WHEN** the hello names the drive `home` with `example/crew` at `/data/repositories/example/crew` and `example/kit` at `/data/repositories/example/kit`, and the drive `work` with `example/app` at `/data/repositories/example/app`, and the runner reports for `home` on host `host-a`
- **THEN** `home/runner/host-a`'s `repositories.rec` opens with crew's descriptor of a `Repository`, byte for byte, holds `example/crew` then `example/kit`, each with its folder as its `Folder`, names no `example/app`, and passes `recfix --check`

#### Scenario: A launch in a repository another drive keeps

- **WHEN** the hello names the drive `home` with `example/crew` and the drive `work` with `example/app`, launch 20 of `home` names no `Host` and says `Repo: example/app`, no runner's branch answers it, and the host has room
- **THEN** `home`'s pull claims 20 and gives the offer `home/20`, its repository `example/app`, and `home/runner/host-a`'s `repositories.rec` names `example/crew` alone

### Requirement: repositories.rec is written at every write of the runner's branch

crew's implementation SHALL write `repositories.rec` at every write of `<label>/runner/<host>`: at a report, from the drives of the partition's latest pull, and at a pull that claims or withdraws, from the drives of the hello it was handed. A write's commit SHALL name each address whose record it adds, changes or removes.

#### Scenario: A repository the drive comes to keep

- **WHEN** `home/runner/host-a`'s `repositories.rec` names `example/crew` alone, and the next pull's hello names the drive `home` with `example/crew` and `example/kit`
- **THEN** the next report's commit names `repositories example/kit`, and the file holds both

#### Scenario: A repository the drive no longer names

- **WHEN** `home/runner/host-a`'s `repositories.rec` names `example/crew` and `example/kit`, and the next pull's hello names the drive `home` with `example/crew` alone and `example/kit` as drained
- **THEN** the next report's commit names `repositories example/kit`, and the file holds `example/crew` alone

#### Scenario: A claim names the drive's repositories

- **WHEN** `host-a` has no branch of `home`, and its pull claims launch 20, the hello naming the drive `home` with `example/crew`
- **THEN** the claim's write makes `home/runner/host-a` holding request 20's claim and a `repositories.rec` naming `example/crew` with its folder

### Requirement: A partition whose label names no drive names no repository

Where the hello names no drive by the partition's label, crew's implementation SHALL write `repositories.rec` as crew's descriptor alone, and SHALL say one line naming the label and each drive the hello names, once while it stands. It SHALL give and claim launches as it does for any partition.

#### Scenario: A label no drive has

- **WHEN** crew's table serves the partition `home` on host `host-a`, and the hello names the drives `family` and `work` alone
- **THEN** `home/runner/host-a`'s `repositories.rec` is crew's descriptor alone, and one line says this host's configuration names no drive `home`, naming `family` and `work`
