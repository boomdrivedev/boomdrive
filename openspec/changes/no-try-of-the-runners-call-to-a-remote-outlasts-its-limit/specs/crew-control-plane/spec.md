## ADDED Requirements

### Requirement: No listing of the runner branches outlasts its limit

crew's implementation SHALL list `<label>/runner/*` by a `git ls-remote` run as every git call of the runner's to a remote is: in a process group of its own, stopped at thirty seconds with everything it started (record 0042; core.58).

#### Scenario: The listing's limit is thirty seconds

- **WHEN** crew's implementation makes the store of a partition
- **THEN** its listing's limit is thirty seconds

### Requirement: A listing stopped at its limit leaves its partition unread

A listing stopped at its limit SHALL fail with the words `git ls-remote with <url> was still going after <limit>, the most a try may take, and was stopped`, naming the state repository's URL, after git's last line on standard error where git wrote one. Its partition SHALL be unread with those words, and pulled again as any part that could not be read (record 0042).

#### Scenario: A state repository that never answers the listing

- **WHEN** crew's implementation lists `home/runner/*` of a state repository that takes the connection and never answers
- **THEN** the listing ends within its limit and grace and a margin, and fails with `git ls-remote with <url> was still going after <limit>, the most a try may take, and was stopped`, naming the state repository's URL
