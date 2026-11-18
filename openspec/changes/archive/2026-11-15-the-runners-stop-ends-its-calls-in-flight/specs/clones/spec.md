## ADDED Requirements

### Requirement: A stop of the runner ends its fetch in flight

Every fetch of a clone, a pass's, a landing's and a new clone's first, SHALL also end at the runner's stop, through the runner's halt, `weir-bounded`'s, which the runner's git carries. A fetch in flight when the halt is set SHALL have its group asked to end, given two seconds, then killed, as at its limit, so nothing it started outlives it. A fetch begun once the halt is set SHALL run no git (record 0042 as amended on 2026-11-12; core.58).

#### Scenario: A fetch in flight at the stop

- **WHEN** a fetch whose remote never answers runs with a limit of sixty seconds, and its git's halt is set 200 ms in
- **THEN** it fails within five seconds, its limit not reached

#### Scenario: A fetch begun after the stop

- **WHEN** a fetch begins with its git's halt already set
- **THEN** it fails at once and runs no git

#### Scenario: The runner's git carries one halt

- **WHEN** a git is made by `Git::default()`, and another by `halted_by` a halt
- **THEN** the first's halt is never set, and the second's is that halt, set when it is set; `trying_for` keeps a git's halt

### Requirement: A fetch ended at the stop fails with a halted call's words

A fetch ended at the runner's stop, or begun after it, SHALL fail with `weir-bounded`'s words for a halted call, `git fetch with origin was stopped as its caller asked`, after git's last line on standard error and `; ` where git wrote one, and never with the limit's words. Its standard output SHALL not be read as an answer (record 0042 as amended on 2026-11-12).

#### Scenario: A halted fetch's words

- **WHEN** a fetch writes `fatal: half` on standard error and is halted before it ends
- **THEN** it fails with `fatal: half; git fetch with origin was stopped as its caller asked`

#### Scenario: A fetch begun after the stop says so alone

- **WHEN** a fetch begins with its git's halt already set
- **THEN** it fails with `git fetch with origin was stopped as its caller asked` alone

### Requirement: A fetch ended at the stop is named as any failed fetch

A repository whose fetch was ended at the runner's stop, or begun after it, SHALL be named not fetched, or not cloned with nothing left in the data directory, with the halted words, as for any failed fetch, and the runner's next start SHALL fetch it again (record 0042 as amended on 2026-11-12).

#### Scenario: A new clone halted in flight

- **WHEN** the runner is stopped while its first fetch of `agentplot/flume`, with no clone of it, is in flight
- **THEN** a line reads `agentplot/flume: not cloned: git fetch with origin was stopped as its caller asked`, and no folder of `agentplot/flume`'s is left in the data directory
