# clones

## MODIFIED Requirements

### Requirement: A place has ended once the status carried of it says Flume holds it no more

A place SHALL have ended once the status the report carries of it stands not listed, or reads the place's phase `tidied` or `gone` and does not name `boomdrive` among those its record is pending for. A place with no status carried SHALL not have ended. So a place whose record Flume holds for the runner SHALL not have ended until the runner has collected it (record 0030).

#### Scenario: The place tidied

- **WHEN** the runner drains `agentplot/flume` for `P` alone, and the status the report carries of `P` then reads its record `P@<at>` tidied and landed
- **THEN** the next pass removes its view and its container

#### Scenario: The place no longer listed

- **WHEN** the runner drains `agentplot/flume` for `P` alone, and the status the report carries of `P` then stands not listed
- **THEN** the next pass removes its view and its container

#### Scenario: The place's end held for the runner

- **WHEN** the runner drains `agentplot/flume` for `P` alone, and the status the report carries of `P` reads its record `P@<at>` tidied and landed, pending for `boomdrive`
- **THEN** the next pass keeps its view and its container, named as held by `P`; and once the carried status reads `P@<at>` pending for `boomdrive` no more, the pass after removes them
