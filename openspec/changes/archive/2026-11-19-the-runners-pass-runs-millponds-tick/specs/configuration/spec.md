## ADDED Requirements

### Requirement: The configuration names the inboxes the host keeps

The configuration file's top-level `inboxes` SHALL be a list of the names millpond's host configuration gives the inboxes this host keeps, each a string that is not empty and named once, in the order the runner ticks them; none where the file names none. Any other value SHALL be refused, naming the file and the line. The local layer SHALL NOT set it (record 0033).

#### Scenario: The inboxes a host keeps

- **WHEN** the file says `inboxes = ["home", "work"]`
- **THEN** the runner ticks `home`, then `work`, after each settle; and a file naming no `inboxes` ticks none

#### Scenario: An inbox named twice or not named

- **WHEN** the file's `inboxes` names `home` twice, holds an empty name, or is a string
- **THEN** the file is refused, naming its line and `inboxes`
