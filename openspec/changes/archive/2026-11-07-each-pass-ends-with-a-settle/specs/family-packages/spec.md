# family-packages

## MODIFIED Requirements

### Requirement: The set is the one the runner's tests pass with

The gate SHALL run the set's own Flume and sluice, the flake's packages of its lock, given to the runner's tests by their paths. Those tests SHALL check each program's version against its tag, and read the answers a scratch host gives with the runner's own readers. No test of the set SHALL run a Flume or a sluice found on the host's `PATH`.

#### Scenario: Each tool's version

- **WHEN** the runner's tests run in the gate
- **THEN** the set's Flume says `flume <Flume's tag without the v>` and its sluice says `sluice <sluice's tag without the v>`

#### Scenario: Flume's status of a scratch host

- **WHEN** the set's Flume is asked `--json status` on a scratch host that holds no place
- **THEN** the runner reads the answer as Flume's status, with no place and the host part as Flume printed it

#### Scenario: The set's Flume keeps the runner's watch

- **WHEN** the set's Flume is asked `--json settle --every 60s` on a scratch host that holds no place, as the runner asks it
- **THEN** the runner reads the answer as given, and the set's Flume's `--json status` then gives the host's watch with a bound of 60 seconds

#### Scenario: A submission sluice has no record of

- **WHEN** the set's sluice is asked, in a scratch repository, for the entry of a submission it has no record of
- **THEN** the runner reads its answer as a missing entry

#### Scenario: Not the host's tools

- **WHEN** the runner's tests of the set run in a shell whose `PATH` holds another `flume` and `sluice`
- **THEN** they run the set's, by their paths
