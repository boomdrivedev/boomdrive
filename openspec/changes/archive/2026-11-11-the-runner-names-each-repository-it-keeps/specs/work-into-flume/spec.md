## ADDED Requirements

### Requirement: Every pull's hello names each drive's repositories with their folders

The hello SHALL name each drive of the configuration, by its name, with each repository the drive names whose clone is made, in the configuration's order: its address as the configuration writes it, and its clone's container, the folder `flume launch --repository` names. A drive none of whose clones is made SHALL be named with none, and no drive SHALL name a repository the configuration no longer names.

#### Scenario: Each drive's repositories with their folders

- **WHEN** the configuration's drive `home` names `agentplot/flume` and `example/crew`, its drive `work` names `example/app`, and its drive `idle` names `example/later`; the clones of `agentplot/flume` and `example/app` are made, `example/crew`'s and `example/later`'s are not; and `example/kit`'s clone stands, which the configuration no longer names
- **THEN** the hello names `home` with `agentplot/flume` at its container alone, `work` with `example/app` at its container, and `idle` with none, and no drive names `example/kit`
