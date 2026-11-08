# running

## RENAMED Requirements

- FROM: `### Requirement: Each turn of passes ends with a settle of every place`
- TO: `### Requirement: Each pass ends with a settle of every place`

## MODIFIED Requirements

### Requirement: Each pass ends with a settle of every place

`boomdrive run` SHALL end each keeping pass, on its loop, and each part's control pass, on the part's thread, with one `flume --json settle --every <bound>`, awaited before that thread waits. The first keeping pass SHALL share the settles of the parts' first control passes. A pass that ends while another's settle is under way SHALL share it and run none. A keeping pass whose configuration did not read SHALL settle all the same, and a runner told to stop SHALL begin none (core.13; record 0029).

#### Scenario: Passes back to back share one settle

- **WHEN** `fetch_interval` and `poll_interval` are an hour, and the runner's first keeping pass and its part's first control pass run back to back with a stub `flume`
- **THEN** one `flume --json settle --every 3600s` runs, after every other `flume` command of the control pass, and no other settle runs before the next pass

#### Scenario: Each control pass ends with a settle

- **WHEN** `fetch_interval` is an hour and `poll_interval` a second, and the runner makes three control passes of its part with a stub `flume`
- **THEN** each control pass's `flume` commands end with one `flume --json settle --every 1s`, before the next control pass reads Flume's status

#### Scenario: Each keeping pass ends with a settle

- **WHEN** the configuration names no control plane, `fetch_interval` is one second, and the runner makes three keeping passes with a stub `flume`
- **THEN** each keeping pass is followed by one `flume --json settle --every 1s`, and the stub is asked nothing else

#### Scenario: A pass on a broken configuration settles

- **WHEN** the configuration of a running runner that keeps a repository each second is made invalid, with a stub `flume`
- **THEN** each pass names the file and keeps nothing, and is followed by one `flume --json settle --every 1s` all the same

#### Scenario: A pass ends while a settle is under way

- **WHEN** one pass's settle is under way, and another pass ends
- **THEN** the other pass runs no settle, and returns without waiting for the one under way to end
