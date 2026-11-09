# crew-control-plane

## ADDED Requirements

### Requirement: A report's answer names taken each status places.rec holds

Once the runner's branch is written, or holds crew's files already, the answer to the report SHALL name taken each launch whose status `places.rec` holds, and no other (record 0030 as amended on 2026-11-09).

#### Scenario: A status places.rec holds is taken

- **WHEN** the report carries a place's status as a fixture's text, beside a request answered `held` with no status
- **THEN** `places.rec`'s `Status` for its request is that text, and the report's answer names that launch's status taken and not the held request's
