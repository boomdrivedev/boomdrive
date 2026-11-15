## MODIFIED Requirements

### Requirement: Every pull carries the host's tools, repositories and leases

The runner SHALL hand every pull a hello holding:

- its interface's version, and `boomdrive --version`'s text, as the runner's version;
- each tool's version: the first line of `flume --version`, `sluice --version` and `git --version`, as each printed it, or the tool named missing;
- the address of each repository the configuration names whose clone is made;
- each lease it holds, by id and number.

It SHALL run no multiplexer's `--version`, and name none among its tools (core.29).

#### Scenario: Repositories kept

- **WHEN** the configuration names `agentplot/flume` and `example/crew`, and only `agentplot/flume`'s clone is made
- **THEN** the hello names `agentplot/flume` alone

#### Scenario: A tool missing

- **WHEN** no program the runner runs as `sluice` exists
- **THEN** the hello names `sluice` as missing, and the pass goes on

#### Scenario: No multiplexer's version

- **WHEN** a hello is made with programs given for `flume`, `sluice`, `git` and `herdr`
- **THEN** its tools are `flume`, `git` and `sluice` alone, and the program given for `herdr` never runs
