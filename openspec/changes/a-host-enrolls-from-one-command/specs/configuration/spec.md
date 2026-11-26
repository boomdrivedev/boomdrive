## MODIFIED Requirements

### Requirement: The local layer sets the repositories and a control plane's partitions alone

The local layer MAY set each drive's `repositories`. In a control plane's table, it MAY set the keys its implementation lists: `partitions`, in crew's table and in the hosted one's. The runner SHALL read each as one key, its value whole. Every command of the runner SHALL read both layers (records 0009 and 0028, core.17 and flume record 0032).

#### Scenario: The local layer's repositories

- **WHEN** the managed layer keeps `agentplot/flume` in the account `own`'s drive `home`, and the local layer names `agentplot/sluice` in that drive
- **THEN** the runner keeps `agentplot/flume`, then `agentplot/sluice`, both in the drive `home`

#### Scenario: A partition enrolled from the local layer

- **WHEN** the managed layer's account `cloud` names `hosted`, and its hosted table names a loopback port with nothing on it. The local layer names the drive `work` of `cloud` and the hosted partition `work`, each with `example/app`. enroll runs for that `url` and `--drive work --repository example/app`, with a token.
- **THEN** enroll refuses with the control plane unreachable, exits 1, and writes no file
