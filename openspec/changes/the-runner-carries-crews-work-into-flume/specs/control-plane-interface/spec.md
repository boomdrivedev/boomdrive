# Control-plane interface

## Purpose

The one interface through which the runner reaches its control plane.
A pull gives what the control plane wants of the host; a report sends
the runner's record of what it did. The configuration chooses the
implementation: crew's now, and a hosted control plane's, one outgoing
WebSocket, later. Every implementation passes the same cases (core.2,
core.3; record 0001).

## ADDED Requirements

### Requirement: The control plane is reached through one interface

The runner SHALL reach its control plane only by the interface's pull
and report. No code outside an implementation SHALL name crew's
implementation, except the runner's one list of the implementations it
carries, which SHALL name each by its configuration word and the
function that opens it. The interface's crate SHALL depend on no
implementation.

#### Scenario: The runner's own code names no implementation

- **WHEN** the sources of `crates/flywheel` and `crates/flywheel-control` are read
- **THEN** only `crates/flywheel/src/planes.rs` names `flywheel_crew`, and `crates/flywheel-control/Cargo.toml` depends on no implementation's crate

### Requirement: A pull gives what the control plane wants of the host

A pull SHALL give:

- the control plane's requests for this host, in its order, each with
  a key the control plane gives: a launch, or a retry, a drop or a
  priority naming a launch's key;
- the runs it wants kept, each with its launch and whether it is
  wanted now;
- the runner's record as the control plane last held it, where it
  holds one.

A pull the control plane cannot answer SHALL be refused with its
words.

#### Scenario: Requests in order

- **WHEN** the fake control plane holds a launch, then a priority naming it, then a drop naming it
- **THEN** a pull gives the three in that order, the priority and the drop naming the launch's key

#### Scenario: A pull refused

- **WHEN** the fake control plane is set to refuse a pull with the words `the state repository did not answer`
- **THEN** the pull is refused with those words

### Requirement: A launch carries everything Flume is to be given

A launch SHALL carry:

- its repository's forge address;
- its kind, name and base;
- its stream, priority and intent;
- its recipients and brief;
- the keys of the launches whose places must land first;
- whether its submission is held;
- any kinds file and agent settings.

#### Scenario: A launch pulled

- **WHEN** the fake control plane holds a launch of `chore` named `tidy` in `afterthought/crew` on `bolt/x`, with stream `madswan`, priority 3, intent `madswan/x` and recipient `madswan/planner`
- **THEN** a pull gives it with each of those fields and the key the fake gave it

### Requirement: The runner's record is reported whole

The runner SHALL report its whole record at the end of each pass:

- each request's answer, `held`, `launched` with its place, `done` or
  `refused`, with its words and when;
- each place it launched, with Flume's status of it and when that
  status was first read;
- each run's place, or why none stands.

The next pull SHALL give back the record as reported.

#### Scenario: The record comes back

- **WHEN** the runner reports a launch answered `launched` at a place, with that place's status, and pulls again
- **THEN** the pull's record holds that answer and that status as reported

### Requirement: A report lets go of the places the control plane carries no more

A report SHALL answer with the keys of the carried places the control
plane carries no more. The runner SHALL then drop those places from its
record, and keep their requests' answers.

#### Scenario: A place let go

- **WHEN** a report's answer names a carried place's key as let go
- **THEN** the runner's next record holds no carried place of that key, and still holds its request's answer

### Requirement: The configuration chooses the implementation

The configuration's `[control_plane]` table SHALL name the
implementation by one word in `kind`, and hand that implementation the
table its word names. A word naming an implementation the binary does
not carry SHALL be refused at start, naming the words it carries, and
the runner SHALL pull nothing. The hosted control plane SHALL NOT be
carried by this binary.

#### Scenario: The hosted control plane named

- **WHEN** the configuration's `[control_plane]` names `kind = "hosted"`
- **THEN** the runner refuses to open its control plane, naming `crew` as the one word it carries, and makes no pull

#### Scenario: crew named

- **WHEN** the configuration's `[control_plane]` names `kind = "crew"` with a `[control_plane.crew]` table
- **THEN** crew's implementation is opened from that table

### Requirement: Every implementation passes the shared cases

The interface's crate SHALL hold cases written against the interface
alone. Each implementation SHALL pass them, the fake control plane and
crew's implementation among them.

#### Scenario: crew's implementation passes them

- **WHEN** the shared cases run with a harness that makes crew's implementation over a bare state repository
- **THEN** every case passes
