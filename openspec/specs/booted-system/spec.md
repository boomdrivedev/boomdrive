# booted-system Specification

## Purpose
The flake's boot of a NixOS system with the module on `aarch64-linux`, emulated on GitHub's arm runner, in which the service's user lingers, the runner's user unit starts and each tool of the set answers, run by the workflow `cache` as the check a box's pin reads (record 0020 as amended on 2026-11-14).

## Requirements

### Requirement: The flake gives the boot on aarch64-linux alone

The flake SHALL give `checks.aarch64-linux.boot`, a NixOS test made with the flake's own nixpkgs for `aarch64-linux`, and SHALL give no check for another system (record 0020 as amended on 2026-11-14).

#### Scenario: The boot's check

- **WHEN** the flake's checks are evaluated
- **THEN** `checks` gives `aarch64-linux` alone, holding `boot` alone, whose derivation evaluates

#### Scenario: Never on x86_64-linux

- **WHEN** the flake's checks are evaluated
- **THEN** they give nothing for `x86_64-linux` or `aarch64-darwin`

### Requirement: The boot's machine is a box that takes no work

The boot SHALL have one machine, `box`, of `aarch64-linux`. On it the Boomdrive module SHALL be enabled for the normal user `alice`, and crew's module on with the host `host-a` and the drives of `nix/tests/settings.nix`'s `crewModule`. The machine SHALL have no control plane and keep no repository, and its system profile SHALL hold git, the host's own (core.44).

#### Scenario: The boot's system

- **WHEN** the boot's machine is evaluated
- **THEN** it is of `aarch64-linux`, with `services.boomdrive.enable`, `services.boomdrive.user = "alice"`, `alice` a normal user who lingers, `services.crew.enable` with the host `host-a`, no `control_plane` in the runner's file, and git in the system profile

### Requirement: The boot needs no KVM and waits as long as an emulated boot takes

The boot SHALL require the build feature `nixos-test` alone, so that it builds where the machine has no `/dev/kvm` and QEMU emulates the guest. Its machine SHALL have 4 cores and 4096 MiB. Each wait of its script SHALL be allowed 30 minutes, and the whole test 90 minutes (record 0020 as amended on 2026-11-14).

#### Scenario: Emulated where there is no KVM

- **WHEN** the boot is evaluated
- **THEN** its required features are `nixos-test` alone, and its machine has 4 cores and 4096 MiB

#### Scenario: Its waits

- **WHEN** the boot is evaluated
- **THEN** its global timeout is 5400 seconds, and each wait of its script names a timeout of 30 minutes

### Requirement: The booted system's user lingers and the runner's unit starts from boot

The boot's script SHALL do these in order, before anything logs `alice` in:

- wait for `multi-user.target`;
- find `alice` lingering, as logind gives it;
- wait for `alice`'s manager, `user@<uid>.service`, as a system unit;
- wait for the runner's unit, `boomdrive.service`, active in that manager.

After every tool has answered, the script SHALL find the unit active, with no restart.

#### Scenario: The runner's unit from boot

- **WHEN** the boot's script is read
- **THEN** it waits for `multi-user.target`, finds `Linger=yes` for `alice`, waits for `alice`'s `user@<uid>.service` and waits for `boomdrive.service` in `alice`'s manager, in that order, each before the script's first command run as `alice`

#### Scenario: The runner stays up

- **WHEN** the boot's script is read
- **THEN** after the last tool's answer, it finds `boomdrive.service` in `alice`'s manager active with `NRestarts=0`

### Requirement: Each tool of the set answers in the booted system

The boot's script SHALL run, as `alice` by a login shell, each tool of the set the modules install on the machine, by its program's name. Each tool but crew SHALL be run with `--version`. crew SHALL be run as `crew _release`, its own answer of its version. The script SHALL fail where a command fails, or where its answer does not hold the version of the flake's `aarch64-linux` package of that tool.

#### Scenario: Each tool's answer

- **WHEN** the boot is evaluated
- **THEN** its answers are `boomdrive`, `flume`, `sluice`, `crew`, `timber`, `herdr`, `wt`, `ghostty`, `portless`, `pendingyou`, `plannotator` and `plannotator-tui`, each with its command and the version of the flake's `aarch64-linux` package of that tool, and its script runs each answer

#### Scenario: Every tool installed answers

- **WHEN** the system profile of the boot's machine is read
- **THEN** each package of it that is one of the flake's `aarch64-linux` packages of a tool has an answer

### Requirement: The workflow boots every commit a box may take

The workflow `cache` SHALL have the job `boot`, named `boot (aarch64-linux)`, on `ubuntu-24.04-arm` and no other runner, which at every run of the workflow builds `.#checks.aarch64-linux.boot`, printing its log. Its check on a commit SHALL be what a box's pin reads: a box takes only a commit whose `boot (aarch64-linux)` passed (afterthought/swancloud record 0059, decision 3).

#### Scenario: The boot's job

- **WHEN** the workflow is read
- **THEN** its job `boot` is named `boot (aarch64-linux)`, runs on `ubuntu-24.04-arm`, has no `needs`, no `if` and no matrix, sets up the family's cache with `skipPush: true`, and runs `nix build --no-link --print-build-logs .#checks.aarch64-linux.boot`

#### Scenario: The boot never on x86_64-linux

- **WHEN** the workflow is read
- **THEN** the job `boot` names no system but `aarch64-linux` and no runner but `ubuntu-24.04-arm`

### Requirement: The boot reads no token and pushes nothing

The job `boot` SHALL substitute what the family's cache holds, build the rest, and wait on no other job. It SHALL be given no environment and no secret but GitHub's own `GITHUB_TOKEN`, which install-nix-action takes to fetch the flake's inputs, and no token of the cache's. It SHALL push nothing (core.4, core.47).

#### Scenario: No token for the boot

- **WHEN** the workflow is read
- **THEN** the job `boot` names no `environment`, no `secrets.` but the `GITHUB_TOKEN` install-nix-action takes, no `CACHIX_AUTH_TOKEN`, no `op://` and no `cachix push`
