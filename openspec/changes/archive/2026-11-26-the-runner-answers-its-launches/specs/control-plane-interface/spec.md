## ADDED Requirements

### Requirement: A control plane says whether the runner alone reads it

`ControlPlane` SHALL say, by `read_through_the_runner`, whether the readers of its places reach the host only through the runner. Each implementation SHALL say it. The hosted implementation SHALL say true. Crew's implementation SHALL say false. See core.54 and record 0030 as amended on 2026-11-26.

#### Scenario: A hosted control plane is read through the runner

- **WHEN** the hosted parts `home` and `work` are open
- **THEN** each part says true

#### Scenario: Crew's control plane is read on the host

- **WHEN** crew's part is open
- **THEN** it says false
