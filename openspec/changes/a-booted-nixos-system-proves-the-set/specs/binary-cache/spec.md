## MODIFIED Requirements

### Requirement: The workflow builds and pushes every package of every system

The workflow `cache` SHALL run on a push to `main`, on a push of a tag `v*`, and by hand, and on no other push. For each system the flake gives, a job on GitHub's hosted runner of that system SHALL build every package the flake gives for it, the runner's among them, and push their store paths and runtime closures to the cache. A job whose build fails SHALL push nothing. Beside the systems' build jobs, the workflow's one other job SHALL be the boot, `boot (aarch64-linux)`, which pushes nothing (booted-system).

#### Scenario: Each system has its job

- **WHEN** the workflow is read beside the flake's packages
- **THEN** its jobs are `build` and `boot` alone, and `build` has one job for each system the flake gives, `x86_64-linux` on `ubuntu-24.04`, `aarch64-linux` on `ubuntu-24.04-arm`, and `aarch64-darwin` on `macos-15`, and none for another

#### Scenario: The runner is pushed with its tools

- **WHEN** a commit lands on `main`
- **THEN** each system's job builds `boomdrive`, `boomdrive-launcher`, `boomdrive-claude-plugins`, `flume`, `sluice`, `crew`, `herdr`, `ghostty`, `portless`, `worktrunk`, `pendingyou`, `plannotator`, `plannotator-tui` and the four herdr plugins of that commit, and pushes them to the cache

#### Scenario: A package that does not build

- **WHEN** one package of a system fails to build
- **THEN** that system's job fails and pushes nothing, and the other systems' jobs push theirs

#### Scenario: A branch's push

- **WHEN** a branch other than `main` is pushed
- **THEN** the workflow does not run

### Requirement: The push's token is the workflow's alone

The workflow SHALL read the cache's auth token only from 1Password, through a service account whose token is the secret `OP_SERVICE_ACCOUNT_TOKEN` of the repository's environment `cachix`, which admits `main` and tags `v*` alone. Each system's build job SHALL fail before it builds, naming what is missing, where either token is empty. No other job SHALL read either token. No file of the tree, option, store path or log SHALL hold either token.

#### Scenario: No service account token

- **WHEN** the workflow runs and the environment gives no `OP_SERVICE_ACCOUNT_TOKEN`
- **THEN** each system's build job fails before it builds, naming `OP_SERVICE_ACCOUNT_TOKEN`, and the boot, which reads no token, runs as it would

#### Scenario: No token

- **WHEN** the workflow runs and the `CACHIX_AUTH_TOKEN` it loads from 1Password is empty
- **THEN** each system's build job fails before it builds, naming `CACHIX_AUTH_TOKEN`, and the boot, which reads no token, runs as it would
