# Control-plane interface

## Purpose

The one interface through which the runner reaches its control plane, by a pull and a report. It covers what passes over it: the host's hello, launches offered under leases, acts on a launch, commands, inboxes to carry, terminals, and the runner's report. It also covers how the configuration chooses the implementation, and the cases every implementation passes. crew's implementation is built now. A hosted control plane's is named and left unbuilt, and the interface carries the cloud's runner contract so that it can take crew's place. Nothing that passes over it names a kinds file or a kind's source: a launch names its kind, and where the host's kinds come from is its Flume configuration (core.2, core.5, core.12; record 0002; flywheel-cloud records 0005, 0009, 0017 and 0022).

## ADDED Requirements

### Requirement: The control plane is reached through one interface

The runner SHALL reach its control plane only through `flywheel-control`'s `ControlPlane` interface: a pull, which takes the runner's hello and report, and a report, which takes the runner's report. No code outside an implementation SHALL name crew, its records or a hosted control plane's protocol. The one list of implementations, `crates/flywheel/src/planes.rs`, SHALL name each by its word and its opener alone (core.2).

#### Scenario: Only crew's crate names crew

- **WHEN** the sources of `crates/flywheel` and `crates/flywheel-control` are read
- **THEN** no file names `flywheel_crew` but `crates/flywheel/src/planes.rs`, and `flywheel-control` depends on no implementation

### Requirement: A pull gives what the control plane wants, in its order

A pull SHALL give the control plane's wants of the host in the control plane's order, each an offer of a launch under a lease, an act on a launch, a command or an inbox to carry. With them it SHALL give the terminals to open, and the runner's report as the control plane last held it, or none. A pull that cannot be read SHALL give `Unread` with words saying why, and the runner SHALL then act on nothing that pass.

#### Scenario: Wants in order

- **WHEN** the fake control plane holds an offer of launch `a`, then a priority of `a`, then a command `c`
- **THEN** a pull gives the offer, the act and the command in that order

#### Scenario: A pull that cannot be read

- **WHEN** the fake control plane refuses a pull with the words `no network`
- **THEN** the control pass names `no network`, runs no `flume` command but the hello's `flume --json status` and `flume --version`, and reports nothing

### Requirement: Work carries the control plane's ids and leases

Every want SHALL carry an id the control plane gave, which the runner SHALL never read into. An offer and a command SHALL carry a lease: its number, and its expiry or no expiry where the control plane never moves the work. An act SHALL name its launch and that launch's lease number.

#### Scenario: An offer's lease kept with its answer

- **WHEN** the fake control plane offers launch `a` under lease number 3, and the runner launches it
- **THEN** the runner's report holds `a` under lease 3, and its next report names 3 with `a`

### Requirement: A launch names what Flume is to be given

An offer's launch SHALL hold its repository, kind and name. It MAY hold its base, brief, stream, priority, intent, recipients, held stages with their words, a held submission, the launches it comes after and its agent settings. Its kind SHALL be the kind as the control plane gave it, a name or an identity `<source>/<name>@<version>`, and the launch SHALL hold no kinds file and no kind's source apart from it (core.5).

#### Scenario: An offer with every field

- **WHEN** the fake control plane holds an offer giving every field of a launch, its kind `crew/crew-planner`
- **THEN** a pull gives each field as the fake holds it, its kind `crew/crew-planner`, and the runner's report holding the offer round-trips through JSON unchanged

### Requirement: The runner's report is reported whole

A report SHALL take the runner's whole report: each launch with its lease number, answer and carried statuses, and each act, command and inbox, by id. A second report of the same report SHALL change nothing the control plane holds.

#### Scenario: A report sent twice

- **WHEN** the runner reports the same report twice to the fake control plane
- **THEN** the fake control plane's last report is the same after the second report as after the first

### Requirement: A report's answer names what is let go and what is refused

A report's answer SHALL name the ids the control plane carries no more, which the runner SHALL drop from its report, and the ids whose reports it refused because their leases are no longer the runner's, with its words. A report that cannot be sent SHALL give `Unsent`, and the runner SHALL keep its report to send at the next pass.

#### Scenario: A report not sent

- **WHEN** the fake control plane refuses a report
- **THEN** the runner's report file holds every answer of the pass, and the next pass reports them

#### Scenario: A launch let go

- **WHEN** the fake control plane answers a report naming launch `a` as carried no more
- **THEN** `a` leaves the runner's report, and later reports do not carry it

### Requirement: The configuration chooses the implementation

The runner SHALL open the implementation the configuration's `[control_plane]` `type` names, from the list in `planes.rs`, handing it the table of that name. This binary SHALL carry `crew`. Any other word, `hosted` among them, SHALL be refused at start, naming the words this binary carries. The hosted control plane's implementation SHALL be named and not built.

#### Scenario: The hosted control plane named

- **WHEN** the configuration says `[control_plane]` with `type = "hosted"`
- **THEN** the runner names `hosted` as a control plane this binary does not carry and names `crew`, runs no control pass, and keeps its repositories

### Requirement: Every implementation passes the shared cases

`flywheel-control` SHALL hold the cases every implementation passes, each over a harness that makes a control plane holding given wants: the wants given in order; a reported answer and status coming back in the next pull's report; a second report of one report changing nothing; and a settled want still given, which the runner answers from its report. The fake control plane and crew's implementation SHALL each run them.

#### Scenario: The fake control plane passes

- **WHEN** `flywheel-control`'s tests run the shared cases over the fake control plane
- **THEN** every case passes

#### Scenario: crew's implementation passes

- **WHEN** `flywheel-crew`'s tests run the shared cases over a bare state repository holding crew's records
- **THEN** every case passes
