# Configuration

## MODIFIED Requirements

### Requirement: The configuration holds nothing the runner does not read

The configuration SHALL be refused, naming the key, when it holds any key this spec does not name. In particular, no key SHALL hold a token, key or credential: git's own credential helper on the host answers for every fetch (core.4). No key SHALL name a stream: streams, and the stream a repository's launches run in, are each host's Flume configuration, and a refusal of `stream` or `streams` SHALL say so (core.5).

#### Scenario: A misspelt key

- **WHEN** the configuration says `view = "~/Code"`
- **THEN** it is refused, naming `view` as a key the runner does not read

#### Scenario: A token

- **WHEN** the configuration says `token = "..."`
- **THEN** it is refused, naming `token` as a key the runner does not read, and the value appears in no output

#### Scenario: A stream named

- **WHEN** the configuration says `stream = "work"`, or holds a `[streams]` table
- **THEN** it is refused, naming the key as one the runner does not read and saying streams are Flume's host configuration

## ADDED Requirements

### Requirement: The configuration may name the control plane

The configuration MAY hold a `[control_plane]` table. In it, `kind`, the word of the implementation to open, SHALL be required, and the table named by `kind` SHALL be handed whole to that implementation, which reads its own keys. Where the configuration holds no `[control_plane]`, the runner SHALL keep its repositories and make no control pass.

#### Scenario: crew named

- **WHEN** the configuration holds `[control_plane]` with `kind = "crew"` and a `[control_plane.crew]` table
- **THEN** the runner opens crew's implementation with that table

#### Scenario: No kind

- **WHEN** the configuration holds `[control_plane]` with `every = "30s"` and no `kind`
- **THEN** it is refused, naming `kind` as required

#### Scenario: No control plane

- **WHEN** the configuration holds no `[control_plane]`
- **THEN** `flywheel run --once` keeps the repositories and runs no `flume` command

### Requirement: How often the control plane is passed is a setting with a default

`[control_plane]`'s `every` SHALL be how often the runner makes a control pass, a whole number of seconds, minutes or hours written `<n>s`, `<n>m` or `<n>h`, more than zero. Where it is absent, the period SHALL be one minute. Any other value SHALL be refused, naming it.

#### Scenario: No period

- **WHEN** `[control_plane]` has no `every`
- **THEN** the runner makes a control pass every minute

#### Scenario: A period of nothing

- **WHEN** `[control_plane]` says `every = "0s"`
- **THEN** it is refused, naming the value and the form `<n>s`, `<n>m` or `<n>h`

### Requirement: The control-plane table is read when the runner starts

The `[control_plane]` table SHALL be read when the runner starts, and a change to it SHALL take effect when the runner starts again.

#### Scenario: A period changed while running

- **WHEN** `every` is changed from `1m` to `5m` in the file of a running runner
- **THEN** its control passes keep a one-minute period until the runner starts again
