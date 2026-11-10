## ADDED Requirements

### Requirement: No try of a fetch outlasts its limit

Every fetch of a clone, a pass's, a landing's and a new clone's first, SHALL run in a process group of its own and SHALL be stopped at its limit, thirty seconds, the limit agentplot/sluice record 0109 sets for every try of a call to a remote. At the limit its group SHALL be asked to end, given two seconds, then killed, so nothing it started outlives it (record 0042; core.58).

#### Scenario: What a call started ends with it

- **WHEN** a call is run to its limit, and starts a process that starts another, neither of which ever ends
- **THEN** at its limit both are ended, the call is said stopped, and what each pipe gave until then is kept

#### Scenario: A call asked to end is given its grace

- **WHEN** a call run to its limit, asked to end at its limit, says so and exits
- **THEN** what it said as it ended is kept, and the call is said stopped

#### Scenario: A call that will not end is killed

- **WHEN** a call run to its limit ignores being asked to end
- **THEN** it is killed once its grace has passed, and the call is said stopped

#### Scenario: A call that ends within its limit

- **WHEN** a call run to its limit exits by itself before it
- **THEN** its exit and everything it printed are given, and the call is not said stopped

#### Scenario: The limit is thirty seconds

- **WHEN** the runner fetches a clone with the git it runs on a host
- **THEN** the fetch's limit is thirty seconds and its grace two

### Requirement: A fetch stopped at its limit is named, and the pass goes on

A fetch stopped at its limit SHALL fail with the words `git fetch with origin was still going after <limit>, the most a try may take, and was stopped`, after git's last line on standard error where git wrote one. Its repository SHALL be named not fetched, or not cloned with nothing left in the data directory, as for any failed fetch. Every other repository SHALL still be kept in that pass, and the next pass SHALL fetch it again (record 0042).

#### Scenario: A fetch whose origin never answers

- **WHEN** a pass keeps `agentplot/flume` and `agentplot/sluice`, both cloned, and `agentplot/flume`'s origin takes the connection and never answers
- **THEN** the pass ends within its limit and grace and a margin; a line reads `agentplot/flume: not fetched: git fetch with origin was still going after <limit>, the most a try may take, and was stopped`; and `agentplot/sluice` is fetched and its view moved in the same pass

#### Scenario: A clone whose origin never answers

- **WHEN** a pass keeps `agentplot/flume`, with no clone of it, and `agentplot/sluice`, and `agentplot/flume`'s origin takes the connection and never answers
- **THEN** the pass ends within its limit and grace and a margin; a line reads `agentplot/flume: not cloned: git fetch with origin was still going after <limit>, the most a try may take, and was stopped`; no folder of `agentplot/flume`'s is left in the data directory; and `agentplot/sluice` is kept in the same pass
