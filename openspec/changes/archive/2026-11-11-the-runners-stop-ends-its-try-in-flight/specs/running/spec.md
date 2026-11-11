## ADDED Requirements

### Requirement: A stop of boomdrive run ends its try in flight

Sent SIGTERM or SIGINT, `boomdrive run` SHALL end each try of a call to a remote it has in flight as that try's limit would, and SHALL begin no such try, before it stops as it otherwise does: before the next repository it would keep, beginning no part's pass, letting each part end the pass in hand, and exiting 0 (record 0042 as amended on 2026-11-11; core.58).

#### Scenario: Stopped with a fetch in flight

- **WHEN** `boomdrive run` keeps `agentplot/flume`, with no clone of it, whose origin takes the connection and never answers, and is sent SIGTERM once the origin has taken the connection
- **THEN** it exits 0 within five seconds, well before the fetch's limit; its last two lines read `agentplot/flume: not cloned: git fetch with origin was stopped as the runner stopped` and `boomdrive: stopped`; and the origin's connection is closed within a second of its exit

### Requirement: boomdrive run --once stops as boomdrive run does

Sent SIGTERM or SIGINT, `boomdrive run --once` SHALL stop as `boomdrive run` does: end each try of a call to a remote it has in flight as that try's limit would, begin no such try, stop before the next repository it would keep, begin no part's pass and no settle, let each part end the pass in hand, say `boomdrive: stopped`, and exit 1 (record 0042 as amended on 2026-11-11; core.39).

#### Scenario: Stopped with a fetch in flight

- **WHEN** `boomdrive run --once` keeps `agentplot/flume`, with no clone of it, whose origin takes the connection and never answers, with a stub `flume` that notes each call it is given, and is sent SIGINT once the origin has taken the connection
- **THEN** it exits 1 within five seconds, well before the fetch's limit; its last two lines read `agentplot/flume: not cloned: git fetch with origin was stopped as the runner stopped` and `boomdrive: stopped`; it runs no `flume` command; and the origin's connection is closed within a second of its exit
