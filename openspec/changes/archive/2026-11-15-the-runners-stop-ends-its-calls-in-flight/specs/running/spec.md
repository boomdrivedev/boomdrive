## ADDED Requirements

### Requirement: A stop of the runner ends its calls in flight

Sent SIGTERM or SIGINT, `boomdrive run` and `boomdrive run --once` SHALL end each try of a call to a remote in flight as its limit would, through the runner's halt, `weir-bounded`'s, begin no such try, keep no further repository, begin no part's pass and no settle, and let each part end the pass in hand. `boomdrive run` SHALL then exit 0, and `boomdrive run --once` SHALL say `boomdrive: stopped` and exit 1 (record 0042 as amended on 2026-11-12; core.39, core.58).

#### Scenario: Stopped with a fetch in flight

- **WHEN** `boomdrive run` keeps `agentplot/flume`, with no clone of it, from an origin that takes the connection and never answers, and is sent SIGTERM once the origin has taken it
- **THEN** it exits 0 within five seconds; the lines after its start are `agentplot/flume: not cloned: git fetch with origin was stopped as its caller asked` and `boomdrive: stopped`; no folder of `agentplot/flume`'s is left in the data directory; and the origin's connection is closed within a second of its exit

#### Scenario: Stopped with a part's read in flight

- **WHEN** `boomdrive run` carries crew's partition `home`, whose state repository takes the connection and never answers, and is sent SIGTERM once `home`'s read of `home/main` has taken it
- **THEN** it exits 0 within five seconds; one line reads `boomdrive: home: the control plane could not be read: ` followed by words ending `git fetch with <url> was stopped as its caller asked`, naming the state repository's URL; its last line is `boomdrive: stopped`; it runs no settle; and the state repository's connection is closed within a second of its exit

#### Scenario: run --once stopped with a fetch in flight

- **WHEN** `boomdrive run --once` keeps `agentplot/flume`, with no clone of it, from an origin that takes the connection and never answers, and is sent SIGINT once the origin has taken it
- **THEN** it exits 1 within five seconds; the lines after its start are `agentplot/flume: not cloned: git fetch with origin was stopped as its caller asked` and `boomdrive: stopped`; it runs no `flume`; and the origin's connection is closed within a second of its exit
