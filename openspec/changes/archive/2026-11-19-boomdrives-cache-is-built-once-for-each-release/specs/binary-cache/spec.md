## MODIFIED Requirements

### Requirement: The workflow builds and pushes every package of every system

The workflow `cache` SHALL run on a push of a tag `v*` and by hand, and on no push of a branch. For each system the flake gives, a job on GitHub's hosted runner of that system SHALL build every package the flake gives for it, the runner's among them, and push their store paths and runtime closures to the cache. A job whose build fails SHALL push nothing. Its one other job SHALL be the boot, `boot (aarch64-linux)`, which pushes nothing (booted-system).

#### Scenario: Each system has its job

- **WHEN** the workflow is read beside the flake's packages
- **THEN** its jobs are `build` and `boot` alone, and `build` has one job for each system the flake gives, `x86_64-linux` on `ubuntu-24.04`, `aarch64-linux` on `ubuntu-24.04-arm`, and `aarch64-darwin` on `macos-15`, and none for another

#### Scenario: The runner is pushed with its tools

- **WHEN** a commit lands on `main` and is released as a tag
- **THEN** the workflow runs once, at the tag, and each system's job builds `boomdrive`, `boomdrive-launcher`, `boomdrive-claude-plugins`, `flume`, `sluice`, `crew`, `herdr`, `rex`, `ghostty`, `portless`, `worktrunk`, `timber`, `pendingyou`, `plannotator`, `plannotator-tui` and the five herdr plugins of that commit, of the tag, and pushes them to the cache

#### Scenario: A package that does not build

- **WHEN** one package of a system fails to build
- **THEN** that system's job fails and pushes nothing, and the other systems' jobs push theirs

#### Scenario: A branch's push

- **WHEN** a branch is pushed, `main` among them
- **THEN** the workflow does not run
