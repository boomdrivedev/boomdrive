## ADDED Requirements

### Requirement: Each line of a part's pass names the part

Each line the runner says of a part's control pass SHALL name the part's label after `boomdrive:`, as `boomdrive: <label>: <words>`. A line about one want SHALL begin with the want's id, as it does, the id being the part's own.

#### Scenario: A part's line names its label

- **WHEN** `home`'s control pass cannot read Flume's status, which answers `flume was ended: signal: 9`
- **THEN** one line says `boomdrive: home: Flume's status could not be read: flume was ended: signal: 9`

## REMOVED Requirements

### Requirement: The runner's lines of a part's pass name the part

**Reason**: Its one scenario, acts left for the next pass, no longer happens: no act waits for the next pass because it is due (record 0002 as amended on 2026-11-19; core.50). "Each line of a part's pass names the part" holds the rule.

**Migration**: None.
