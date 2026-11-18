## ADDED Requirements

### Requirement: An observer is handed a part's passes and gives nothing back

`boomdrive-control` SHALL name an `Observer`, which takes a hello and a part's share of the report and returns nothing, and `Observed`, an observer with the label of the part it observes. No pull, want, answer or wake SHALL come from an observer. The hosted implementation SHALL give one observer for each partition of `[observer]`, reached through `planes.rs` alone (core.2, core.19; record 0048).

#### Scenario: An observer takes what it is handed

- **WHEN** the fake observer is handed a hello and a share of the report twice
- **THEN** it holds both, in order, and nothing it holds reaches the part it observes

#### Scenario: The hosted observers

- **WHEN** `[observer]` names the partitions `home` and `work`
- **THEN** the hosted implementation gives two observers, `home` then `work`
