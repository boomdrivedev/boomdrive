## ADDED Requirements

### Requirement: An implementation serves its control plane as parts

An implementation SHALL serve its control plane as one or more parts, in its table's order, each a `Part` holding a label, given once among its parts, and a `ControlPlane` of its own. crew's implementation SHALL give a part for each partition of its table, a drive, and the hosted implementation a part for each partition of its table, an account's enrollment (record 0026; record 0028; core.49).

#### Scenario: crew's partitions as parts

- **WHEN** crew's table names the partitions `home` and `work`
- **THEN** crew's implementation opens two parts, `home` then `work`

#### Scenario: The hosted partitions as parts

- **WHEN** the hosted table names the partitions `home` and `work`
- **THEN** the hosted implementation opens two parts, `home` then `work`

### Requirement: A part says which ids are its own

`ControlPlane` SHALL say, by `owns`, whether an id is one it gives. Every want a part gives SHALL carry an id that part owns and no other part of its implementation owns. The runner SHALL keep each part's share of its report by `owns`, and SHALL read no id itself.

#### Scenario: crew's ids by partition

- **WHEN** crew's parts `home` and `work` each give a launch numbered 1
- **THEN** `home` owns `home/1` and not `work/1`, and `work` owns `work/1` and not `home/1`

#### Scenario: The hosted ids by partition

- **WHEN** the hosted parts `home` and `work` each give a launch `l-1`
- **THEN** `home` owns `home/l-1` and not `work/l-1`, and `work` owns `work/l-1` and not `home/l-1`

## MODIFIED Requirements

### Requirement: The control plane is reached through one interface

The runner SHALL reach its control plane only through `boomdrive-control`'s `ControlPlane` interface, served as parts: a pull, which takes the runner's hello and the part's share of its report, a report, which takes that share, and `owns`. No code outside an implementation SHALL name crew, its records or a hosted control plane's protocol. The one list of implementations, `crates/boomdrive/src/planes.rs`, SHALL name each by its word and its opener alone (core.2).

#### Scenario: Only crew's crate names crew

- **WHEN** the sources of `crates/boomdrive` and `crates/boomdrive-control` are read
- **THEN** no file names `boomdrive_crew` but `crates/boomdrive/src/planes.rs`, and `boomdrive-control` depends on no implementation

### Requirement: The configuration chooses the implementation

The runner SHALL open the implementation the configuration's `[control_plane]` `type` names, from the list in `planes.rs`, handing it the table of that name, the `poll_interval`, and, for each part it opens, a say and a wake of that part's own, which it calls to wake that part when its control plane pushes the part's work. This binary SHALL carry `crew` and `hosted`. Any other word SHALL be refused at start, naming the words this binary carries.

#### Scenario: The hosted control plane named

- **WHEN** the configuration says `[control_plane]` with `type = "hosted"` and a `[control_plane.hosted]` table naming a partition
- **THEN** the runner opens the hosted implementation with that table, and names no word as one this binary does not carry

#### Scenario: A word not carried

- **WHEN** the configuration says `[control_plane]` with `type = "elsewhere"`
- **THEN** the runner names `elsewhere` as a control plane this binary does not carry and names `crew` and `hosted`, runs no control pass, and keeps its repositories

### Requirement: Every implementation passes the shared cases

`boomdrive-control` SHALL hold the cases every implementation passes, each over a harness that makes a control plane holding given wants: the wants given in order; a reported answer and status coming back in the next pull's report; a second report changing nothing; a settled want still given, which the runner answers from its report; and each want's id owned by its giver. The fake control plane, crew's implementation and the hosted implementation SHALL each run them.

#### Scenario: The fake control plane passes

- **WHEN** `boomdrive-control`'s tests run the shared cases over the fake control plane
- **THEN** every case passes

#### Scenario: crew's implementation passes

- **WHEN** `boomdrive-crew`'s tests run the shared cases over a bare state repository holding crew's records
- **THEN** every case passes

#### Scenario: The hosted implementation passes

- **WHEN** `boomdrive-hosted`'s tests run the shared cases over a fake hosted control plane on the loopback address, holding the wants of one partition
- **THEN** every case passes
