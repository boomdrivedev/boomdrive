# control-plane-interface

## ADDED Requirements

### Requirement: A report's answer names which statuses were taken

A report's answer SHALL name each launch whose carried status the control plane took with the report it answers, and no other, so the runner confirms an end only once its control plane has it (core.2, core.54; record 0030 as amended on 2026-11-09). An answer that names no statuses taken SHALL read as naming none.

#### Scenario: A status taken and one left

- **WHEN** the fake control plane is reported launches `a` and `b`, each with a carried status, and launch `c` with none, and the test leaves `b`'s status
- **THEN** its answer names `a`'s status taken, and neither `b`'s nor `c`'s

#### Scenario: An answer from before

- **WHEN** a report's answer is written by `boomdrive-control`'s types naming a status taken, and an answer's JSON names no statuses taken
- **THEN** the first round-trips unchanged; and the second reads as naming none, and is written back with no `statuses_taken`
