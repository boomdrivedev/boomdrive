# Binary cache

## MODIFIED Requirements

### Requirement: The workflow builds and pushes every package of every system

The workflow `cache` SHALL run on a push to `main`, on a push of a tag `v*`, and by hand, and on no other push. For each system the flake gives, a job on GitHub's hosted runner of that system SHALL build every package the flake gives for it, the runner's among them, and push their store paths and runtime closures to the cache. A job whose build fails SHALL push nothing.

#### Scenario: Each system has its job

- **WHEN** the workflow is read beside the flake's packages
- **THEN** it has one job for each system the flake gives, `x86_64-linux` on `ubuntu-24.04`, `aarch64-linux` on `ubuntu-24.04-arm`, and `aarch64-darwin` on `macos-15`, and none for another

#### Scenario: The runner is pushed with its tools

- **WHEN** a commit lands on `main`
- **THEN** each system's job builds `boomdrive`, `boomdrive-launcher`, `boomdrive-claude-plugins`, `flume`, `sluice`, `crew`, `herdr`, `ghostty`, `portless`, `worktrunk`, `timber`, `pendingyou`, `plannotator`, `plannotator-tui` and the five herdr plugins of that commit, and pushes them to the cache

#### Scenario: A package that does not build

- **WHEN** one package of a system fails to build
- **THEN** that system's job fails and pushes nothing, and the other systems' jobs push theirs

#### Scenario: A branch's push

- **WHEN** a branch other than `main` is pushed
- **THEN** the workflow does not run

### Requirement: A host substitutes the set, or builds the same paths

The modules' default packages SHALL be the flake's own, made with its locked nixpkgs whatever the host's `pkgs`, so they are the store paths the cache holds: the runner's, each tool's module's, and those of the modules' own options, the herdr plugins among them. A host whose nix trusts the cache SHALL substitute them, and one that does not SHALL build the same paths from the source the lock names. The modules SHALL add no substituter or key to a host's nix settings.

#### Scenario: A host's own nixpkgs

- **WHEN** a module is enabled, with crew turned on, in a system whose `pkgs` carries an overlay of the host's own
- **THEN** the system profile holds the flake's own `boomdrive`, `flume`, `sluice`, `crew`, `herdr`, `ghostty`, `portless`, `worktrunk`, `timber`, `boomdrive-launcher`, `pendingyou`, `plannotator` and `plannotator-tui`, the store paths the flake gives, and the plugins it links are the flake's own `timber-herdr-plugin`, `pendingyou-herdr-plugin` and `plannotator-herdr-plugin` beside Flume's and sluice's

#### Scenario: The host's trust

- **WHEN** a module is enabled
- **THEN** the system's nix substituters and trusted public keys name nothing of the cache's
