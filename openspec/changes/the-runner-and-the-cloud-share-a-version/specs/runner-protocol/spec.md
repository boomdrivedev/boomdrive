## ADDED Requirements

### Requirement: The runner names the versions it speaks in one file of data

`crates/boomdrive-hosted/protocol/spoken.json` SHALL name each version directory under `protocol/` once, lowest first, as `{"version": N, "spoken": bool}` under `versions`, and nothing else, marking spoken exactly the versions the runner's code checks, sends and offers. The crate's test SHALL hold the file to the directories and to the code (core.42; principle 16).

#### Scenario: The file names each version directory

- **WHEN** `spoken.json` is read and `protocol/` listed
- **THEN** it holds `versions` alone, naming versions 1 to 5 once each, lowest first, and versions 1 to 4 are not spoken

#### Scenario: The runner speaks what the file marks spoken

- **WHEN** the crate's test reads `spoken.json`
- **THEN** the versions it marks spoken are version 5 alone, the version the crate's code speaks, and the upgrade offers `boomdrive.v5`

### Requirement: The gate refuses a runner that offers a version the cloud's main does not speak

The gate's check `boomdrive-hosted:cloud-protocol` SHALL read `workers/control-plane/runner-protocol/pin.json` from boomdrive-cloud's `main` as the forge holds it, never from a cache, and SHALL fail where `spoken.json` marks spoken none, or a version the pin does not mark spoken, naming both lists. Where it cannot read the pin, it SHALL fail naming what it could not read (Boom: `the-cloud-leads-the-runners-protocol`; core.42).

#### Scenario: A runner whose versions the cloud speaks passes

- **WHEN** the check runs against a cloud whose `main` pins versions 4 and 5 spoken, the runner marking 5 alone, or 4 and 5, spoken
- **THEN** it passes, saying the cloud speaks each version the runner offers

#### Scenario: A runner that offers a version the cloud does not speak is refused

- **WHEN** the runner marks version 5 spoken and the cloud's `main` pins 3 and 4 spoken, whether or not the runner also marks 4, or the runner marks 4 alone and the cloud's `main` pins 5 alone, or the runner marks none
- **THEN** the check fails, naming the versions each side speaks and each version the runner offers that the cloud does not speak

#### Scenario: A pin that cannot be read is refused

- **WHEN** the cloud's repository cannot be fetched, its `main` holds no pin, or its pin is not JSON or does not list versions
- **THEN** the check fails, naming `workers/control-plane/runner-protocol/pin.json` and what it could not read
