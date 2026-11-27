## ADDED Requirements

### Requirement: Every pull's hello names each kept repository's status, as Flume gave it

Each status a pass reads SHALL name the folder of each clone of the hello's drives, once each and in order. The hello SHALL name each kept repository's status by its address. The status SHALL be the item of Flume's `status.repositories` that names the clone's folder, as Flume printed it. Where there is none, it SHALL be missing, with words saying why (core.1, core.11).

#### Scenario: Each kept repository's status, as Flume gave it

- **WHEN** `agentplot/flume` and `example/app` are kept, and Flume's status gives an item for `agentplot/flume`'s clone alone
- **THEN** each status read names both clones' folders in order, the hello names `agentplot/flume`'s status as Flume printed its item, and `example/app`'s missing with `Flume's status names no status of the repository's clone`

#### Scenario: A status Flume refused gives no repository's status

- **WHEN** Flume refuses its status with `no store`
- **THEN** the hello names each kept repository's status missing with `no store`
