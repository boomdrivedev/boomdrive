## ADDED Requirements

### Requirement: A stop of the runner ends its fetch in flight

A fetch of a clone in flight when the runner stops SHALL be ended as its limit ends it: its group asked to end, given two seconds, then killed, so nothing it started outlives it. A fetch begun once the runner has stopped SHALL run no process (record 0042 as amended on 2026-11-11; core.58).

#### Scenario: What a halted call started ends with it

- **WHEN** a call run to a limit of a minute starts a process that starts another, neither of which ever ends, and its halt is set 300 ms in
- **THEN** both are ended within five seconds, the call is said halted, and what each pipe gave until then is kept

#### Scenario: A halted call is given its grace

- **WHEN** a call is halted while it runs, and, asked to end, says so and exits
- **THEN** what it said as it ended is kept, and the call is said halted

#### Scenario: A call begun after its halt

- **WHEN** a call is run whose halt is already set
- **THEN** it runs no process, returns at once with no exit and nothing printed, and is said halted

### Requirement: A fetch ended by the runner's stop is named

A fetch ended by the runner's stop, or begun after it, SHALL fail with the words `git fetch with origin was stopped as the runner stopped`, after git's last line on standard error where git wrote one. Its repository SHALL be named not fetched, or not cloned with nothing left in the data directory, as for any failed fetch (record 0042 as amended on 2026-11-11).

#### Scenario: A halted fetch's words

- **WHEN** a call to `origin`, `git fetch`, writes `fatal: half` on standard error and is halted while it runs
- **THEN** it fails with `fatal: half; git fetch with origin was stopped as the runner stopped`

#### Scenario: A fetch begun after the stop

- **WHEN** a call to `origin`, `git fetch`, is made once its halt is set
- **THEN** it fails with `git fetch with origin was stopped as the runner stopped` alone
