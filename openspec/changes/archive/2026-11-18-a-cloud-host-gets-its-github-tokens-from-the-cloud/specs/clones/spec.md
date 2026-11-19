## ADDED Requirements

### Requirement: A fetch on a cloud host goes through the control plane's token

Where the configuration's control plane holds the App, the running runner SHALL open its forge tokens at start, and a fetch of a served repository, a new clone's first among them, SHALL go through the token it gives, by the call's own helper. Where it gives none, the fetch SHALL go with no credential, and where such a fetch fails after a refused ask, its line SHALL give git's words, then `; no forge token: ` and the ask's words (record 0053; core.33, core.34).

#### Scenario: A cloud host's fetch given the control plane's token

- **WHEN** a pass keeps `agentplot/flume` on a cloud host, its control plane giving a token for it, and its origin needs no credential
- **THEN** it is fetched and its view moved, and the control plane was asked for `agentplot/flume`'s token

#### Scenario: A repository the App's installation does not cover

- **WHEN** a pass keeps `agentplot/flume` on a cloud host, its control plane saying the App's installation does not cover it, and its origin needs no credential
- **THEN** it is fetched and its view moved, and no line names a forge token

#### Scenario: The running runner asks its control plane

- **WHEN** `boomdrive run --once` keeps `agentplot/flume` on a scratch host whose configuration names the hosted control plane at a loopback port with nothing on it, its partition `home` served `agentplot/flume` with no key of `home` in the host's store, and the fetch of `agentplot/flume` fails
- **THEN** a line reads `agentplot/flume: not fetched: <git's words>; no forge token: ` and words naming `boomdrive enroll home`
