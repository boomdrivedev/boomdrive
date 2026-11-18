## MODIFIED Requirements

### Requirement: On a cloud host forge-token refuses until the protocol carries a token

Where the configuration's control plane is hosted, `boomdrive forge-token` SHALL refuse every operation with exit 1, printing nothing on standard output. Standard error SHALL say that the hosted control plane mints the host's forge tokens and that version 3 of its protocol carries none yet. The words SHALL be the hosted implementation's own (record 0007 as amended on 2026-11-09; record 0052; core.2, core.33).

#### Scenario: A get on a cloud host

- **WHEN** the configuration names the hosted control plane and git asks `get` for a served repository
- **THEN** it exits 1, standard output is empty, standard error says the hosted control plane mints this host's forge tokens and version 3 of its protocol carries none yet, and nothing is minted

#### Scenario: An erase on a cloud host

- **WHEN** the configuration names the hosted control plane and git asks `erase`
- **THEN** it exits 1 with the same words, and standard output is empty
