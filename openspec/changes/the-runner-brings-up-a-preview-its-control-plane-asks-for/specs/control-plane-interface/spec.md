## ADDED Requirements

### Requirement: A preview names a repository, a branch and a revision, under a lease of its own

A preview SHALL hold its id, a lease, its repository's address, a branch, a revision and whether it is done. Not done, it asks the host to bring up the preview of the branch at the revision; done, it asks the host to put away the preview of that repository and revision it brought up. It SHALL name no word of sluice's, and SHALL be written as `{"preview": {"id", "lease", "repository", "branch", "revision", "done"}}` (core.55).

#### Scenario: A preview of each kind

- **WHEN** a preview not done and a preview done are written by `boomdrive-control`'s types and read back
- **THEN** each reads back equal, the first written as `{"preview": {…}}` holding `"done": false`

### Requirement: The report holds what the runner answered of each preview

The report's `previews` SHALL hold, by id, what the runner answered of each preview: its lease, the repository and revision it named, whether it was done, its outcome, `held`, `done` or `refused`, the tool's text and the words where it has them, the tool's listing of it where carried, and its time (core.55).

#### Scenario: A report holding a preview

- **WHEN** a report holding a preview answered `held`, with a carried status, is written by `boomdrive-control`'s types and read back
- **THEN** it reads back equal, the preview written under `"previews"`

## MODIFIED Requirements

### Requirement: A pull gives what the control plane wants, in its order

A pull SHALL give the control plane's wants of the host in the control plane's order, each an offer of a launch under a lease, an act on a launch, a command, an inbox to carry or a preview. With them it SHALL give the terminals to open, and the runner's report as the control plane last held it, or none. A pull that cannot be read SHALL give `Unread` with words saying why, and the runner SHALL then act on nothing that pass.

#### Scenario: Wants in order

- **WHEN** the fake control plane holds an offer of launch `a`, then a priority of `a`, then a command `c`
- **THEN** a pull gives the offer, the act and the command in that order

#### Scenario: A pull that cannot be read

- **WHEN** the fake control plane refuses a pull with the words `no network`
- **THEN** the control pass names `no network`, runs no `flume` command but the hello's `flume --json status` and `flume --version`, and reports nothing

### Requirement: Work carries the control plane's ids and leases

Every want SHALL carry an id the control plane gave, which the runner SHALL never read into. An offer, a command and a preview SHALL carry a lease: its number, and its expiry or no expiry where the control plane never moves the work. An act SHALL name its launch and that launch's lease number. The report's leases SHALL name each launch's, command's and preview's id with its lease number.

#### Scenario: An offer's lease kept with its answer

- **WHEN** the fake control plane offers launch `a` under lease number 3, and the runner launches it
- **THEN** the runner's report holds `a` under lease 3, and its next report names 3 with `a`

#### Scenario: A preview's lease among the report's leases

- **WHEN** a report holds launch `a` under lease 3 and preview `p` under lease 5
- **THEN** its leases are `a` with 3 and `p` with 5

### Requirement: The runner's report is reported whole

A report SHALL take the runner's whole report: each launch with its lease number, the repository it was answered in where the runner recorded one, its answer and carried statuses, and each act, command, inbox and preview, by id. A report file or a message that names no repository for a launch SHALL read as recording none, and one that names no previews SHALL read as holding none. A report holding no preview SHALL be written with no `previews`. A second report of the same report SHALL change nothing the control plane holds.

#### Scenario: A report sent twice

- **WHEN** the runner reports the same report twice to the fake control plane
- **THEN** the fake control plane's last report is the same after the second report as after the first

#### Scenario: A report from before

- **WHEN** a report's JSON names no repository for launch `a` and no previews, and a hello's JSON names no drained repository
- **THEN** each reads, `a` recording no repository, the report holding no preview and the hello naming none drained, and each round-trips through JSON unchanged, the report written with no `previews`
