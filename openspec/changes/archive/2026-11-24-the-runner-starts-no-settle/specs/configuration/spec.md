## MODIFIED Requirements

### Requirement: The configuration names the inboxes the host keeps

The configuration file's top-level `inboxes` SHALL be a list of the names millpond's host configuration gives the inboxes this host keeps, each a string that is not empty and named once, in the order the runner ticks them; none where the file names none. Any other value SHALL be refused, naming the file and the line. The local layer SHALL NOT set it (record 0033).

#### Scenario: The inboxes a host keeps

- **WHEN** the file says `inboxes = ["home", "work"]`
- **THEN** the runner ticks `home`, then `work`, at the end of each pass; and a file naming no `inboxes` ticks none

#### Scenario: An inbox named twice or not named

- **WHEN** the file's `inboxes` names `home` twice, holds an empty name, or is a string
- **THEN** the file is refused, naming its line and `inboxes`

### Requirement: The control-plane table holds the table of each plane the accounts name

The configuration MAY hold a `[control_plane]` table: `poll_interval` and the table of each implementation an account names, each handed whole to its implementation. It SHALL be refused where it holds `type`, naming `accounts.<account>.control_plane`; where it holds a table no account names, or none; and where it holds any other key. Where no account names a control plane, the runner SHALL make no control pass (record 0054; core.19).

#### Scenario: crew named

- **WHEN** the account `own` says `control_plane = "crew"`, and the configuration holds `[control_plane]` with a `[control_plane.crew]` table
- **THEN** the runner opens crew's implementation with that table

#### Scenario: A type of before

- **WHEN** the configuration holds `[control_plane]` with `type = "crew"` and a `[control_plane.crew]` table
- **THEN** it is refused, naming `type` and saying each account names its control plane by `accounts.<account>.control_plane`

#### Scenario: A table no account names

- **WHEN** the account `own` says `control_plane = "crew"`, and `[control_plane]` holds `[control_plane.crew]` and `[control_plane.hosted]`
- **THEN** it is refused, naming `[control_plane.hosted]` as a table no account names

#### Scenario: No control plane

- **WHEN** the configuration holds no `[control_plane]` and names no `fetch_interval`
- **THEN** `boomdrive run --once` keeps the repositories, makes no control pass, and runs no `flume` command

