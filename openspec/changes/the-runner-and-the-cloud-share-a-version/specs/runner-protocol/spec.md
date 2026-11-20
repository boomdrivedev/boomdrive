## ADDED Requirements

### Requirement: The runner names the versions it speaks in one file of data

`crates/boomdrive-hosted/protocol/spoken.json` SHALL name each version directory under `protocol/` once, lowest first, as `{"version": N, "spoken": bool}` under `versions`, and nothing else, marking spoken exactly the versions the runner's code checks, sends and offers. The crate SHALL read it, and each upgrade SHALL offer one version, the newest spoken first (record 0056, point 4 as amended; core.42; principle 16).

#### Scenario: The file names each version directory

- **WHEN** `spoken.json` is read and `protocol/` listed
- **THEN** it holds `versions` alone, naming versions 1 to 4 once each, lowest first, and versions 1 to 3 are not spoken

#### Scenario: The runner offers what the file marks spoken

- **WHEN** the crate reads `spoken.json`
- **THEN** the versions it marks spoken are version 4 alone, and the upgrade offers `boomdrive.v4`

### Requirement: The gate refuses a tree that shares no version with the cloud's main

The gate's check `boomdrive-hosted:cloud-protocol` SHALL read `workers/control-plane/runner-protocol/pin.json` from boomdrive-cloud's `main` as GitHub holds it, never from a cache, and SHALL fail where the versions it marks spoken share none with those `spoken.json` marks spoken, naming both lists. Where it cannot read the pin, or the pin does not list versions, it SHALL fail naming what it could not read, and pass nothing (record 0056, points 1 and 5; core.42).

#### Scenario: A version shared passes

- **WHEN** the check runs against a cloud whose `main` pins versions 3 and 4 spoken, the runner marking 4 and 5 spoken
- **THEN** it passes, naming version 4 as shared

#### Scenario: No version shared is refused

- **WHEN** the runner marks version 5 alone spoken and the cloud's `main` pins 3 and 4 spoken, or pins 4 not spoken and 5 spoken while the runner marks 4 alone
- **THEN** the check fails, naming the versions each side speaks and that they share none

#### Scenario: A pin that cannot be read is refused

- **WHEN** the cloud's repository cannot be fetched, its `main` holds no pin, or its pin is not JSON or does not list versions
- **THEN** the check fails, naming `workers/control-plane/runner-protocol/pin.json` and what it could not read
