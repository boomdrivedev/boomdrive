# install Specification

## Purpose
Boomdrive's install is the script that each release attaches. On a Mac, it adds the set at the release's tag to a person's Nix profile. At the person's asking, it installs Nix and sets Nix to use the family's cache. Then it runs the enroll command in its flags, with the token on its standard input. Record 0012 as amended on 2026-11-27, core.31, core.37 and core.47 govern it.

## Requirements

### Requirement: GitHub serves the install with each release

`install.sh` SHALL be attached to each GitHub release of Boomdrive, so GitHub serves it at `https://github.com/boomdrivedev/boomdrive/releases/latest/download/install.sh` and at `releases/download/v<version>/install.sh`. It SHALL give the release's version in `version=`. It SHALL run as the argument of `sh -c`, so its standard input is free for the token (record 0012 as amended on 2026-11-27).

#### Scenario: The command the page copies

- **WHEN** a person runs `echo <token> | sh -c "$(curl -fsSL <address>)" install --yes --control-plane <url> --drive <drive> --repository <owner/name>`
- **THEN** the install adds the set at the release's tag and runs `boomdrive enroll` with those flags

### Requirement: The install takes its own flag and the enroll command's

The install SHALL take `--yes`, and `--control-plane`, `--drive` and one or more `--repository`, which it SHALL pass to `boomdrive enroll` in their order. It SHALL take the three enroll flags together or none of them. Any other flag, and any part of the three alone, SHALL be a usage error, exit 2, before the install runs anything.

#### Scenario: A flag that is not the install's

- **WHEN** the install is given `--token <token>`
- **THEN** it exits 2 and runs nothing

#### Scenario: Part of the enroll command

- **WHEN** the install is given `--drive home` alone
- **THEN** it exits 2, names `--control-plane` and `--repository`, and runs nothing

#### Scenario: No enroll command

- **WHEN** the install is given `--yes` alone, on a host whose Nix uses the cache
- **THEN** it adds the set and runs no `boomdrive enroll`

### Requirement: The install runs on macOS on Apple silicon

The install SHALL go on only on macOS on Apple silicon. On Linux it SHALL stop with exit 2, naming `services.boomdrive`. On an Intel Mac or another system it SHALL stop with exit 2. It SHALL install nothing where it stops. Each stop SHALL say that the token is not spent, where the install was given an enroll command.

#### Scenario: Linux

- **WHEN** the install runs on Linux, with an enroll command
- **THEN** it exits 2, names `services.boomdrive` and says that the token is not spent, and installs nothing

#### Scenario: A Mac that is not Apple silicon

- **WHEN** the install runs on macOS on `x86_64`
- **THEN** it exits 2, names Apple silicon, and installs nothing

### Requirement: The install stops where Nix or another source gives the runner

Where the runner's file is a link into `/nix/store`, the install SHALL stop with exit 2, install nothing, and name the enroll command without the install. Where `boomdrive` on the `PATH` gives the release's version, the install SHALL add nothing and run the enroll command. Where it gives another version, the install SHALL stop with exit 2, naming both versions.

#### Scenario: Nix sets up the runner

- **WHEN** `~/.config/boomdrive/config.toml` is a link into `/nix/store`
- **THEN** the install exits 2, names `boomdrive enroll` with the enroll command's flags, and installs nothing

#### Scenario: Another runner on the PATH

- **WHEN** `boomdrive --version` gives `boomdrive 0.1.0`, and the install is of another version
- **THEN** the install exits 2, names both versions, and installs nothing

#### Scenario: This release's runner on the PATH

- **WHEN** `boomdrive --version` gives the install's version
- **THEN** the install runs no `nix` and runs the enroll command, with the token on its standard input

### Requirement: The install asks before it installs Nix

Where the host has no `nix`, the install SHALL say that it will install Nix at the pinned release. It SHALL say that it will set Nix to use the family's cache, and name the cache's URL and key. It SHALL go on only with `--yes`, or a yes on the terminal that is its standard input. Otherwise it SHALL stop with exit 2, name `--yes`, and download nothing.

#### Scenario: No Nix, and no answer

- **WHEN** the host has no `nix`, the install is not given `--yes`, and its standard input is the token
- **THEN** it names Nix's release, the cache and `--yes`, exits 2, and fetches nothing

### Requirement: The install runs Nix's installer at the pinned digest alone

The install SHALL run Nix's installer only where its SHA-256 is the digest that the install pins. The installer's flags SHALL be `--daemon --yes --nix-extra-conf-file`, and the file SHALL turn on flakes and name the cache and its key.

#### Scenario: An installer whose digest is not the pinned one

- **WHEN** the host has no `nix`, the install is given `--yes`, and the installer it fetches gives another digest
- **THEN** it exits 1, names that digest, and does not run the installer

#### Scenario: No Nix, and yes

- **WHEN** the host has no `nix`, the install is given `--yes`, and the installer gives the pinned digest
- **THEN** it runs the installer with a configuration that turns on flakes and names the cache and its key. Then it adds the set and runs the enroll command

### Requirement: The install asks before Nix uses the cache

Where `nix config show` does not name the cache and its key, the install SHALL show the two lines that name them. It SHALL name the file that it adds them to, and the restart of Nix's daemon. It SHALL go on only with `--yes`, or a yes on the terminal that is its standard input. On yes, it SHALL add the lines with `sudo tee -a` and restart Nix's daemon. Where it cannot write the file, it SHALL stop with exit 2 and add nothing to the profile.

#### Scenario: Nix that uses the cache

- **WHEN** `nix config show` names the cache and its key
- **THEN** the install runs no `sudo`, adds the set and runs the enroll command

#### Scenario: Nix that does not use the cache, and no answer

- **WHEN** `nix config show` does not name the cache, and the install is not given `--yes`
- **THEN** it names `/etc/nix/nix.conf` and the two lines, exits 2, and changes nothing

#### Scenario: Nix that does not use the cache, and yes

- **WHEN** `nix config show` does not name the cache, and the install is given `--yes`
- **THEN** it adds the two lines to `/etc/nix/nix.conf`, restarts `system/org.nixos.nix-daemon`, adds the set and runs the enroll command

#### Scenario: A Nix configuration the install cannot write

- **WHEN** the install is given `--yes`, and `sudo tee -a` fails
- **THEN** it exits 2, restarts no daemon, and adds nothing to the profile

### Requirement: The install writes the lines where the host's Nix reads them

The install SHALL add the cache's lines to `/etc/nix/nix.custom.conf` and restart `system/systems.determinate.nix-daemon` where `determinate-nixd` is on the `PATH`, and to `/etc/nix/nix.conf` and restart `system/org.nixos.nix-daemon` otherwise.

#### Scenario: Determinate's Nix

- **WHEN** `determinate-nixd` is on the `PATH`, `nix config show` does not name the cache, and the install is given `--yes`
- **THEN** it adds the lines to `/etc/nix/nix.custom.conf` and restarts `system/systems.determinate.nix-daemon`

### Requirement: The install adds the set at its tag and gives the token to enroll alone

The install SHALL add `boomdrive`, `flume`, `sluice`, `herdr` and `herdr-plugins` of `github:boomdrivedev/boomdrive/v<version>`, and `nixpkgs#git` from that flake's inputs, with one `nix profile add`. It SHALL then run `boomdrive enroll` from the profile with the enroll command's flags, as its last command. No other program that the install runs SHALL read its standard input. So the token goes to `boomdrive enroll` alone, and no process has it in its arguments (core.31).

#### Scenario: The token on enroll's standard input alone

- **WHEN** the install adds the set and runs the enroll command, with the token on its standard input
- **THEN** `boomdrive enroll` reads the token on its standard input, and no program that the install ran has the token in its arguments
