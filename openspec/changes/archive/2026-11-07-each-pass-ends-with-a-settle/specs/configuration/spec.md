# configuration

## MODIFIED Requirements

### Requirement: The configuration may name the control plane

The configuration MAY hold a `[control_plane]` table. In it, `type`, the word of the implementation to open, SHALL be required, and the table named by `type` SHALL be handed whole to that implementation, which reads its own keys. Where the configuration holds no `[control_plane]`, the runner SHALL keep its repositories and make no control pass.

#### Scenario: crew named

- **WHEN** the configuration holds `[control_plane]` with `type = "crew"` and a `[control_plane.crew]` table
- **THEN** the runner opens crew's implementation with that table

#### Scenario: No type

- **WHEN** the configuration holds `[control_plane]` with `poll_interval = "30s"` and no `type`
- **THEN** it is refused, naming `type` as required

#### Scenario: No control plane

- **WHEN** the configuration holds no `[control_plane]` and names no `fetch_interval`
- **THEN** `boomdrive run --once` keeps the repositories, makes no control pass, and runs no `flume` command but the settle that ends its pass, `flume --json settle --every 120s`
