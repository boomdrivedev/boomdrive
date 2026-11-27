## MODIFIED Requirements

### Requirement: The runner hears Flume's status from flume host

While `boomdrive run` carries a control plane's parts, it SHALL hold one subscription to flume host, `{"ask":"subscribe","repositories":[…]}`. The ask SHALL name the folder of each clone that a pass reads, and no `repositories` where it names none. It SHALL keep the latest `document` as flume host sent it, and read each pass's status from it, running no `flume --json status`. A read that names other folders SHALL make it ask again. A document that differs from the one held of the same ask SHALL wake every part. `boomdrive run --once` SHALL ask `{"ask":"status"}`, naming the same folders, for each read. A runner that carries no part SHALL hold none (core.13).

#### Scenario: Passes read the subscription

- **WHEN** the runner carries `home`, `poll_interval` is a second, and a stub flume host answers on the socket
- **THEN** three control passes run, the stub flume host is asked one subscription, which names the kept clone's folder, and the stub `flume` is asked no `status`

#### Scenario: A change heard wakes the parts

- **WHEN** the runner carries `home`, `poll_interval` is an hour, and its first control pass ended
- **AND** the stub flume host sends a document that differs from the one held
- **THEN** `home` makes a control pass within seconds

#### Scenario: A runner that carries no part

- **WHEN** the configuration names no control plane
- **THEN** the runner makes no connection to flume host's socket

#### Scenario: The asks name the folders read

- **WHEN** a status ask names `/r` and `/s`, and a subscription is read first naming `/r`, then naming `/r` and `/s`
- **THEN** the status ask is `{"ask":"status","repositories":["/r","/s"]}`, and the subscription asks `{"ask":"subscribe","repositories":["/r"]}`
- **AND** the subscription asks again on a new connection naming `/r` and `/s`, each read gives the document of its own ask, and no part wakes
