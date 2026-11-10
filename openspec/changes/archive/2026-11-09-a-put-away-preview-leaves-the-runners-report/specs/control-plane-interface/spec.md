## MODIFIED Requirements

### Requirement: A report's answer names what is let go and what is refused

A report's answer SHALL name the ids the control plane carries no more, which the runner SHALL drop from its report, and the ids whose reports it refused because their leases are no longer the runner's, with its words. A report that cannot be sent SHALL give `Unsent`, and the runner SHALL keep its report to send at the next pass.

#### Scenario: A report not sent

- **WHEN** the fake control plane refuses a report
- **THEN** the runner's report file holds every answer of the pass, and the next pass reports them

#### Scenario: A launch let go

- **WHEN** the fake control plane answers a report naming launch `a` as carried no more
- **THEN** `a` leaves the runner's report, and later reports do not carry it

#### Scenario: A preview let go

- **WHEN** the runner has brought up the preview `home/30`, and the fake control plane answers a report naming `home/30` as carried no more
- **THEN** a line reads `home/30: let go by the control plane`; `home/30` leaves the runner's report, and later reports do not carry it; and no `sluice release` is run
