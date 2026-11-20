## ADDED Requirements

### Requirement: An end is confirmed only while Flume still lists it owed

Of the keys a pass would confirm, the runner SHALL confirm only those that Flume's listing, read once after the answer to the pass's report where any key is to be confirmed, still gives pending for `boomdrive` at the place. Where that listing cannot be read, it SHALL confirm nothing that pass, and a later pass SHALL confirm what Flume still owes (core.54).

#### Scenario: A record Flume forgot before the confirm

- **WHEN** the runner carries `P@<at>` with its landing pending for `boomdrive`, and Flume forgets the record while the pass's report is sent and taken
- **THEN** no `flume confirm` is run and nothing is said of it; and the next pass carries the status not listed and confirms nothing

#### Scenario: Flume's listing unread at the confirm

- **WHEN** the runner carries `P@<at>` with its landing pending for `boomdrive`, its report is taken, and Flume's listing cannot be read after it
- **THEN** nothing is confirmed that pass; and the next pass, Flume's listing read, confirms the landing's key
