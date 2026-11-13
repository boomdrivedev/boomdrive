## MODIFIED Requirements

### Requirement: The workflow builds and pushes every package of every system

The workflow `cache` SHALL run on a push to `main`, on a push of a tag `v*`, and by hand, and on no other push. For each system the flake gives, a job on GitHub's hosted runner of that system SHALL build every package the flake gives for it, the runner's among them, and push their store paths and runtime closures to the cache. A job whose build fails SHALL push nothing. Before Nix is installed, the `x86_64-linux` job SHALL remove from its runner the preinstalled trees that no package's build uses and dpkg does not own, the Android SDK, .NET, GHC, Swift, Rust's own and the tool cache among them, so the store and the builds have room for the set, and SHALL say the free space of `/` before and after; no other system's job SHALL remove anything.

#### Scenario: Each system has its job

- **WHEN** the workflow is read beside the flake's packages
- **THEN** it has one job for each system the flake gives, `x86_64-linux` on `ubuntu-24.04`, `aarch64-linux` on `ubuntu-24.04-arm`, and `aarch64-darwin` on `macos-15`, and none for another

#### Scenario: The x86_64-linux job has room

- **WHEN** the workflow is read
- **THEN** its first step, run only where the system is `x86_64-linux`, removes `/usr/local/lib/android`, `/usr/share/dotnet`, `/opt/ghc`, `/usr/local/.ghcup`, `/usr/share/swift`, `~/.cargo`, `~/.rustup` and `/opt/hostedtoolcache` and says the free space of `/` before and after, before Nix is installed, and no other step has a condition

#### Scenario: The runner is pushed with its tools

- **WHEN** a commit lands on `main`
- **THEN** each system's job builds `boomdrive`, `boomdrive-launcher`, `boomdrive-claude-plugins`, `flume`, `sluice`, `crew`, `herdr`, `ghostty`, `portless`, `worktrunk`, `pendingyou`, `plannotator`, `plannotator-tui` and the four herdr plugins of that commit, and pushes them to the cache

#### Scenario: A package that does not build

- **WHEN** one package of a system fails to build
- **THEN** that system's job fails and pushes nothing, and the other systems' jobs push theirs

#### Scenario: A branch's push

- **WHEN** a branch other than `main` is pushed
- **THEN** the workflow does not run
