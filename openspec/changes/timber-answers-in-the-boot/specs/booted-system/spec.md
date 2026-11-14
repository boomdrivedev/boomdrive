## MODIFIED Requirements

### Requirement: Each tool of the set answers in the booted system

The boot's script SHALL run, as `alice` by a login shell, each tool of the set the modules install on the machine, by its program's name. Each tool but crew SHALL be run with `--version`. crew SHALL be run as `crew _release`, its own answer of its version. The script SHALL fail where a command fails, or where its answer does not hold the version of the flake's `aarch64-linux` package of that tool.

#### Scenario: Each tool's answer

- **WHEN** the boot is evaluated
- **THEN** its answers are `boomdrive`, `flume`, `sluice`, `crew`, `timber`, `herdr`, `wt`, `ghostty`, `portless`, `pendingyou`, `plannotator` and `plannotator-tui`, each with its command and the version of the flake's `aarch64-linux` package of that tool, and its script runs each answer

#### Scenario: Every tool installed answers

- **WHEN** the system profile of the boot's machine is read
- **THEN** each package of it that is one of the flake's `aarch64-linux` packages of a tool has an answer
