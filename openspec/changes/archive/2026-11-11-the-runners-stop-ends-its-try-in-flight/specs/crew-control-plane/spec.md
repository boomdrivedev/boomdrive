## ADDED Requirements

### Requirement: A stop of the runner ends its listing in flight

crew's implementation SHALL run its listing of `<label>/runner/*` to the runner's stop as well as to its limit. A listing in flight when the runner stops SHALL be ended as its limit ends it, and a listing begun once the runner has stopped SHALL run no process (record 0042 as amended on 2026-11-11; core.58).

#### Scenario: A listing in flight when the runner stops

- **WHEN** crew's implementation lists `home/runner/*` of a state repository that takes the connection and never answers, its limit thirty seconds, and the runner stops 300 ms into the listing
- **THEN** the listing ends within five seconds

### Requirement: A listing ended by the runner's stop leaves its partition unread

A listing ended by the runner's stop, or begun after it, SHALL fail with the words `git ls-remote with <url> was stopped as the runner stopped`, naming the state repository's URL, after git's last line on standard error where git wrote one. Its partition SHALL be unread with those words (record 0042 as amended on 2026-11-11).

#### Scenario: A listing's words at the runner's stop

- **WHEN** crew's implementation lists `home/runner/*` of a state repository that takes the connection and never answers, and the runner stops 300 ms into the listing
- **THEN** the listing fails with words ending `git ls-remote with <url> was stopped as the runner stopped`, naming the state repository's URL

#### Scenario: A pull after the runner's stop

- **WHEN** a part of crew's implementation pulls after the runner has stopped, its state repository's `home/main` holding a launch for this host
- **THEN** the pull is unread with `git ls-remote with <url> was stopped as the runner stopped`
