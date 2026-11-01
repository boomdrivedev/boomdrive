# Control-plane interface

## Purpose

The one interface through which the runner reaches its control plane, by a pull and a report. It covers what passes over it: the host's hello, launches offered under leases, acts on a launch, commands, inboxes to carry, terminals, and the runner's record. It also covers how the configuration chooses the implementation, and the cases every implementation passes. crew's implementation is built now. A hosted control plane's is named and left unbuilt, and the interface carries the cloud's runner contract so that it can take crew's place (core.2; record 0002; flywheel-cloud records 0005, 0009, 0017 and 0022).

## ADDED Requirements

### Requirement: The control plane is reached through one interface

The runner SHALL reach its control plane only through `flywheel-control`'s `ControlPlane` interface: a pull, which takes the runner's hello and record, and a report, which takes the runner's record. No code outside an implementation SHALL name crew, its records or a hosted control plane's protocol. The one list of implementations, `crates/flywheel/src/planes.rs`, SHALL name each by its word and its opener alone (core.2).

#### Scenario: Only crew's crate names crew

- **WHEN** the sources of `crates/flywheel` and `crates/flywheel-control` are read
- **THEN** no file names `flywheel_crew` but `crates/flywheel/src/planes.rs`, and `flywheel-control` depends on no implementation

### Requirement: A pull gives what the control plane wants, in its order

A pull SHALL give the control plane's wants of the host in the control plane's order, each an offer of a launch under a lease, an act on a launch, a command or a watch of an inbox. With them it SHALL give the terminals to open, and the runner's record as the control plane last held it, or none. A pull that cannot be read SHALL give `Unread` with words saying why, and the runner SHALL then act on nothing that pass.

#### Scenario: Wants in order

- **WHEN** the fake control plane holds an offer of launch `a`, then a priority of `a`, then a command `c`
- **THEN** a pull gives the offer, the act and the command in that order

#### Scenario: A pull that cannot be read

- **WHEN** the fake control plane refuses a pull with the words `no network`
- **THEN** the control pass names `no network`, runs no `flume` command, and reports nothing

### Requirement: Work carries the control plane's ids and leases

Every want SHALL carry an id the control plane gave, which the runner SHALL never read into. An offer and a command SHALL carry a lease: its number, and its expiry or no expiry where the control plane never moves the work. An act SHALL name its launch and that launch's lease number.

#### Scenario: An offer's lease kept with its answer

- **WHEN** the fake control plane offers launch `a` under lease number 3, and the runner launches it
- **THEN** the runner's record holds `a` under lease 3, and its next report names 3 with `a`

### Requirement: A launch's asks are what Flume is to be given

An offer's asks SHALL hold its repository, kind and name. They MAY hold its kinds declaration, base, brief, stream, priority, intent, recipients, held stages with their words, a held submission, the launches it comes after and its agent settings.

#### Scenario: An offer with every ask

- **WHEN** the fake control plane holds an offer giving every ask
- **THEN** a pull gives each ask as the fake holds it, and the runner's record holding the offer round-trips through JSON unchanged

### Requirement: The runner's record is reported whole

A report SHALL take the runner's whole record: each launch with its lease number, answer and carried statuses, and each act, command and inbox, by id. A second report of the same record SHALL change nothing the control plane holds.

#### Scenario: A report sent twice

- **WHEN** the runner reports the same record twice to the fake control plane
- **THEN** the fake control plane's last record is the same after the second report as after the first

### Requirement: A report's answer names what is let go and what is refused

A report's answer SHALL name the ids the control plane carries no more, which the runner SHALL drop from its record, and the ids whose reports it refused because their leases are no longer the runner's, with its words. A report that cannot be sent SHALL give `Unsent`, and the runner SHALL keep its record to report at the next pass.

#### Scenario: A report not sent

- **WHEN** the fake control plane refuses a report
- **THEN** the runner's record file holds every answer of the pass, and the next pass reports them

#### Scenario: A launch let go

- **WHEN** the fake control plane answers a report naming launch `a` as carried no more
- **THEN** `a` leaves the runner's record, and later reports do not carry it

### Requirement: The configuration chooses the implementation

The runner SHALL open the implementation the configuration's `[control_plane]` `kind` names, from the list in `planes.rs`, handing it the table of that name. This binary SHALL carry `crew`. Any other word, `hosted` among them, SHALL be refused at start, naming the words this binary carries. The hosted control plane's implementation SHALL be named and not built.

#### Scenario: The hosted control plane named

- **WHEN** the configuration says `[control_plane]` with `kind = "hosted"`
- **THEN** the runner names `hosted` as a control plane this binary does not carry and names `crew`, runs no control pass, and keeps its repositories

### Requirement: Every implementation passes the shared cases

`flywheel-control` SHALL hold the cases every implementation passes, each over a harness that makes a control plane holding given wants: the wants given in order; a reported answer and status coming back in the next pull's record; a second report of one record changing nothing; and a settled want still given, which the runner answers from its record. The fake control plane and crew's implementation SHALL each run them.

#### Scenario: The fake control plane passes

- **WHEN** `flywheel-control`'s tests run the shared cases over the fake control plane
- **THEN** every case passes

#### Scenario: crew's implementation passes

- **WHEN** `flywheel-crew`'s tests run the shared cases over a bare state repository holding crew's records
- **THEN** every case passes
