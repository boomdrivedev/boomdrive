# Service modules

## MODIFIED Requirements

### Requirement: On nix-darwin the runner is a launchd agent of the primary user's login

With `services.boomdrive.enable`, the nix-darwin module SHALL declare `launchd.user.agents.boomdrive`, loaded in `system.primaryUser`'s login. It SHALL run `<package>/bin/boomdrive run --config <the file>` behind `/bin/wait4path /nix/store`, the shell handing its process to the runner by `exec`, so an agent loaded before the Nix store's volume is mounted waits for it, and a stop's signal reaches the runner. It SHALL start at load, be kept alive at most once in 30 seconds, run as interactive, leave its process group when stopped, have only `HOME` and `PATH` in its environment, and log to `logFile`. Evaluation SHALL fail where the primary user has no home.

#### Scenario: The agent

- **WHEN** the module is enabled for the primary user `alice`, whose home is `/Users/alice`
- **THEN** the agent runs the package's `boomdrive run --config` with the file, `RunAtLoad`, `KeepAlive`, `ThrottleInterval` 30, `ProcessType` `Interactive` and `AbandonProcessGroup`, with `HOME` `/Users/alice`, and logs to `/Users/alice/Library/Logs/boomdrive.log`

#### Scenario: The agent waits for the Nix store

- **WHEN** the module is enabled, with no `settings` or with a crew control plane's
- **THEN** the agent's program is `/bin/sh -c` with a line that runs `/bin/wait4path /nix/store`, then `exec`s the package's `boomdrive run --config` with the file

#### Scenario: No home for the primary user

- **WHEN** the module is enabled and `users.users.alice.home` is not set
- **THEN** evaluation fails, naming the primary user and the option

### Requirement: A changed file or package restarts the runner, and nothing else does

On nix-darwin, the agent's program SHALL name the package and the file, so nix-darwin, which reloads an agent whose plist changed, reloads it when either changes and leaves it running when neither does. On NixOS, the module's user activation SHALL, for the service's user alone, start the unit where it is not running, restart it where the running runner's command line is not the unit's, and do nothing else.

#### Scenario: A new file on nix-darwin

- **WHEN** two configurations differ only in `settings.fetch_interval`
- **THEN** their agents' programs differ, in the file alone, and their plists differ

#### Scenario: A new package on nix-darwin

- **WHEN** two configurations differ only in `package`
- **THEN** their agents' programs differ, in the package alone, and their plists differ

#### Scenario: Nothing else changed on nix-darwin

- **WHEN** two configurations differ only in `tools`
- **THEN** their agents' plists are the same, byte for byte

#### Scenario: Not running on NixOS

- **WHEN** the activation runs for `alice` and the unit is not running
- **THEN** it starts the unit

#### Scenario: Nothing changed on NixOS

- **WHEN** the activation runs for `alice` and the running runner's command line is the unit's
- **THEN** it neither starts nor restarts the unit

#### Scenario: A new file on NixOS

- **WHEN** the activation runs for `alice` and the running runner's command line names another file
- **THEN** it restarts the unit

#### Scenario: Another user's activation

- **WHEN** the activation runs for `bob`
- **THEN** it starts, restarts and links nothing
