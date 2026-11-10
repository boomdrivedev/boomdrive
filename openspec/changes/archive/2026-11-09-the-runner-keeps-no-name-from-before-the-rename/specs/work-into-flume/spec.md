# work-into-flume

## MODIFIED Requirements

### Requirement: The runner acts only on its own places

The runner SHALL carry and act on only the places its report holds, or whose mark Flume's listing gives as `boomdrive:<id>` for an offer it holds, and SHALL find no place by any other mark (record 0035; core.10). Every other place on the host SHALL be read in the listing and left alone.

#### Scenario: The user's own place

- **WHEN** Flume's listing holds a place with no mark beside the runner's own
- **THEN** no `flume` command names it, and no report carries it

#### Scenario: A mark the runner does not give

- **WHEN** an offer `scratch/4` is pulled with no answer, and Flume's listing holds the mark `other:scratch/4` on place `P` and no place marked `boomdrive:scratch/4`
- **THEN** the offer is not answered `launched` at `P`, and no line names the mark `other:scratch/4`

## REMOVED Requirements

### Requirement: A launch marked before the rename is found by its old mark

**Reason**: Record 0035, accepted on 2026-11-09 on the user's words ("When we cut over everything should be Boom Drive"), drops every reading of the runner's name from before the rename. A launch's place is found only by `boomdrive:<id>` (core.10, core.36).

**Migration**: Each host moves its data by hand with no place live, so no place marked before the rename is left to find, before it runs the release, as record 0035's chore says and the release's notes name.
