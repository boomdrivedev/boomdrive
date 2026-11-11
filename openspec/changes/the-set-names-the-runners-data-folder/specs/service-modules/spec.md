## MODIFIED Requirements

### Requirement: On nix-darwin the runner is a launchd agent of the primary user's login

With `services.boomdrive.enable`, the nix-darwin module SHALL declare `launchd.user.agents.boomdrive`, loaded in `system.primaryUser`'s login. It SHALL run `<package>/bin/boomdrive run --config <the file>`, start at load, be kept alive at most once in 30 seconds, run as interactive, leave its process group when stopped, be given `HOME` and `PATH` alone by the module, beside any variable the host gives the job, and log to `logFile`. Evaluation SHALL fail where the primary user has no home.

#### Scenario: The agent

- **WHEN** the module is enabled for the primary user `alice`, whose home is `/Users/alice`
- **THEN** the agent runs the package's `boomdrive run --config` with the file, `RunAtLoad`, `KeepAlive`, `ThrottleInterval` 30, `ProcessType` `Interactive` and `AbandonProcessGroup`, with `HOME` `/Users/alice`, and logs to `/Users/alice/Library/Logs/boomdrive.log`

#### Scenario: No home for the primary user

- **WHEN** the module is enabled and `users.users.alice.home` is not set
- **THEN** evaluation fails, naming the primary user and the option

## ADDED Requirements

### Requirement: The modules name the runner's data folder by the runner's own rule

Each module SHALL declare `services.boomdrive.dataDir`, whether or not the runner is enabled, as the runner's data folder by the runner's own rule. That is `$XDG_DATA_HOME/boomdrive` where the runner's job gives an absolute `XDG_DATA_HOME`, else `~/.local/share/boomdrive` under the runner's user's home: the primary user's on nix-darwin, `services.boomdrive.user`'s on NixOS. It SHALL never name the folder of before the rename (record 0035).

#### Scenario: The data folder on nix-darwin

- **WHEN** the module is enabled for the primary user `alice`, whose home is `/Users/alice`, and the job is given no `XDG_DATA_HOME`
- **THEN** `services.boomdrive.dataDir` is `/Users/alice/.local/share/boomdrive`

#### Scenario: The data folder on NixOS

- **WHEN** the module is enabled for the user `alice`, whose home is `/home/alice`, and the job is given no `XDG_DATA_HOME`
- **THEN** `services.boomdrive.dataDir` is `/home/alice/.local/share/boomdrive`

#### Scenario: The data folder of another user on NixOS

- **WHEN** the module is enabled for the user `bob`, whose home is `/home/bob`
- **THEN** `services.boomdrive.dataDir` is `/home/bob/.local/share/boomdrive`

#### Scenario: The data folder with the runner not enabled

- **WHEN** the nix-darwin module is imported for the primary user `alice` and `services.boomdrive.enable` is not set
- **THEN** `services.boomdrive.dataDir` is `/Users/alice/.local/share/boomdrive`

#### Scenario: The runner's own rule

- **WHEN** the module's rule and the runner's `data_dir` are given one `HOME` and one `XDG_DATA_HOME`, the latter unset, absolute, absolute ending in `/`, empty or relative
- **THEN** each gives the same folder

### Requirement: The data folder follows the job's own XDG_DATA_HOME

The `XDG_DATA_HOME` the data folder follows SHALL be the one the job's declaration gives: the agent's `EnvironmentVariables` on nix-darwin and the unit's `environment` on NixOS. An empty or relative one SHALL be taken as unset, as the runner takes it.

#### Scenario: A job given its own XDG_DATA_HOME

- **WHEN** the host gives the runner's job `XDG_DATA_HOME` `/srv/data`, on nix-darwin in `launchd.user.agents.boomdrive.serviceConfig.EnvironmentVariables` and on NixOS in `systemd.user.services.boomdrive.environment`
- **THEN** `services.boomdrive.dataDir` is `/srv/data/boomdrive` on either platform, and the nix-darwin agent's `HOME` and `PATH` are the module's

#### Scenario: An XDG_DATA_HOME the runner takes as unset

- **WHEN** the host gives the runner's job `XDG_DATA_HOME` `data`, or an empty one
- **THEN** `services.boomdrive.dataDir` is `~/.local/share/boomdrive` under the user's home, on either platform

### Requirement: No host sets the runner's data folder

`services.boomdrive.dataDir` SHALL be read-only, so evaluation of it fails where a host sets it, at any priority.

#### Scenario: A host that sets the data folder

- **WHEN** a host sets `services.boomdrive.dataDir = "/srv/boomdrive"`
- **THEN** evaluation of the option fails, on either platform, as nixpkgs fails a read-only option set beside its value

### Requirement: The README names the runner's data folder

The README's **Installing with nix** SHALL name `services.boomdrive.dataDir` and what it holds. It SHALL say the option is read-only and is read wherever a host names the folder or a clone in it. It SHALL say a host gives the job its own `XDG_DATA_HOME`, since one the job takes from elsewhere is not seen.

#### Scenario: The README's data folder

- **WHEN** the README's **Installing with nix** is read
- **THEN** it names `services.boomdrive.dataDir` and says it is read-only
