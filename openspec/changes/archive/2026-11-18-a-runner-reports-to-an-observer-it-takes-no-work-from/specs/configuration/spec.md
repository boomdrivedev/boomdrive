## ADDED Requirements

### Requirement: The managed layer may name an observer

The managed layer MAY hold an `[observer]` table, handed whole to the hosted implementation, which reads its keys. It SHALL be refused where the file names no `[control_plane]`, whose parts' work it would observe. A local layer that sets it SHALL not be taken. It SHALL be read when the runner starts, as `[control_plane]` is (core.17, core.19; record 0048).

#### Scenario: An observer beside crew

- **WHEN** the configuration holds `[control_plane]` with `type = "crew"`, and `[observer]` with `url = "http://localhost:8787"` and the partition `home`
- **THEN** it reads, holding the `[observer]` table as written

#### Scenario: An observer of no control plane

- **WHEN** the configuration holds `[observer]` and no `[control_plane]`
- **THEN** it is refused, naming `[observer]` and saying it observes the work of a `[control_plane]`'s parts

#### Scenario: An observer in the local layer

- **WHEN** the local layer sets `[observer]`
- **THEN** the local layer is not taken, the managed layer is read alone, and the runner names `observer` as a key the local layer does not set

#### Scenario: An observer off the host

- **WHEN** the configuration's `[observer]` says `url = "https://control.example"`
- **THEN** it is refused, naming `[observer]` and `url`
