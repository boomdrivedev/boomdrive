# control-plane-interface

## RENAMED Requirements

- FROM: `### Requirement: A report's answer names what is let go and what is refused`
- TO: `### Requirement: A report's answer names what is let go, what is refused and which statuses were taken`

## MODIFIED Requirements

### Requirement: A report's answer names what is let go, what is refused and which statuses were taken

A report's answer SHALL name the ids the control plane carries no more, which the runner SHALL drop from its report, and the ids whose reports it refused because their leases are no longer the runner's, with its words. It SHALL name each launch whose carried status the control plane took with the report it answers, and no other, so the runner confirms an end only once its control plane has it (core.2, core.54; record 0030 as amended on 2026-11-09). An answer that names no statuses taken SHALL read as naming none. A report that cannot be sent SHALL give `Unsent`, and the runner SHALL keep its report to send at the next pass.

#### Scenario: A report not sent

- **WHEN** the fake control plane refuses a report
- **THEN** the runner's report file holds every answer of the pass, and the next pass reports them

#### Scenario: A launch let go

- **WHEN** the fake control plane answers a report naming launch `a` as carried no more
- **THEN** `a` leaves the runner's report, and later reports do not carry it

#### Scenario: A status taken and one left

- **WHEN** the fake control plane is reported launches `a` and `b`, each with a carried status, and launch `c` with none, and the test leaves `b`'s status
- **THEN** its answer names `a`'s status taken, and neither `b`'s nor `c`'s

#### Scenario: An answer from before

- **WHEN** a report's answer is written by `boomdrive-control`'s types naming a status taken, and an answer's JSON names no statuses taken
- **THEN** the first round-trips unchanged; and the second reads as naming none, and is written back with no `statuses_taken`
