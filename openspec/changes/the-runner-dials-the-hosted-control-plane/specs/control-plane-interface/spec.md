# Control-plane interface

## MODIFIED Requirements

### Requirement: The configuration chooses the implementation

The runner SHALL open the implementation the configuration's `[control_plane]` `type` names, from the list in `planes.rs`, handing it the table of that name, the `poll_interval`, and a wake, which it calls to wake the runner when its control plane pushes work. This binary SHALL carry `crew` and `hosted`. Any other word SHALL be refused at start, naming the words this binary carries.

#### Scenario: The hosted control plane named

- **WHEN** the configuration says `[control_plane]` with `type = "hosted"` and a `[control_plane.hosted]` table naming a partition
- **THEN** the runner opens the hosted implementation with that table, and names no word as one this binary does not carry

#### Scenario: A word not carried

- **WHEN** the configuration says `[control_plane]` with `type = "elsewhere"`
- **THEN** the runner names `elsewhere` as a control plane this binary does not carry and names `crew` and `hosted`, runs no control pass, and keeps its repositories

### Requirement: Every implementation passes the shared cases

`boomdrive-control` SHALL hold the cases every implementation passes, each over a harness that makes a control plane holding given wants: the wants given in order; a reported answer and status coming back in the next pull's report; a second report of one report changing nothing; and a settled want still given, which the runner answers from its report. The fake control plane, crew's implementation and the hosted implementation SHALL each run them.

#### Scenario: The fake control plane passes

- **WHEN** `boomdrive-control`'s tests run the shared cases over the fake control plane
- **THEN** every case passes

#### Scenario: crew's implementation passes

- **WHEN** `boomdrive-crew`'s tests run the shared cases over a bare state repository holding crew's records
- **THEN** every case passes

#### Scenario: The hosted implementation passes

- **WHEN** `boomdrive-hosted`'s tests run the shared cases over a fake hosted control plane on the loopback address, holding the wants of one partition
- **THEN** every case passes

### Requirement: The protocol is held in the hosted implementation's crate alone

The runner protocol's files and types SHALL be held in `crates/boomdrive-hosted` alone, which SHALL depend on no crate of the tree's but `boomdrive-control`. Only the binary's crate SHALL depend on it, and of the binary's sources only `planes.rs` SHALL name it, as it names crew's (core.2, core.42).

#### Scenario: Only the hosted implementation's crate holds the protocol

- **WHEN** the sources of `crates/boomdrive` but `planes.rs`, and every file of `crates/boomdrive-control` and `crates/boomdrive-crew`, are read, and every file under `crates/` outside `crates/boomdrive-hosted` is listed
- **THEN** none names `boomdrive_hosted` or `boomdrive-hosted`, none's name ends `.schema.json`, `planes.rs` opens `boomdrive_hosted`, only `boomdrive` depends on `boomdrive-hosted`, and `boomdrive-hosted` depends on `boomdrive-control` alone of the tree's crates
