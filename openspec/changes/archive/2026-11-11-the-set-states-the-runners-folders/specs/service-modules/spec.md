## MODIFIED Requirements

### Requirement: The README names the runner's data folder

The README's **Installing with nix** SHALL name `services.boomdrive.dataDir` and what it holds. It SHALL say the option is read-only and is read wherever a host names the folder, a clone's folder in it being read from `services.boomdrive.cloneDirs`. It SHALL say a host gives the job its own `XDG_DATA_HOME`, since one the job takes from elsewhere is not seen.

#### Scenario: The README's data folder

- **WHEN** the README's **Installing with nix** is read
- **THEN** it names `services.boomdrive.dataDir` and says it is read-only

## ADDED Requirements

### Requirement: The modules state each kept repository's clone folder by the runner's own rule

Each module SHALL declare `services.boomdrive.cloneDirs`, whether or not the runner is enabled. It SHALL hold one entry for each address of every drive of every account in `services.boomdrive.accounts`, a drive of another machine's among them, and no other. Each entry SHALL be keyed by its address as written, `<owner>/<name>`, and its value SHALL be the runner's clone folder of it by the runner's own rule, `<services.boomdrive.dataDir>/repositories/<owner>/<name>`.

#### Scenario: Each kept repository's clone folder on nix-darwin

- **WHEN** the module is enabled for the primary user `alice`, whose home is `/Users/alice`, and the account `own`'s drive `home` keeps the owner `agentplot`'s `flume` and `sluice`
- **THEN** `services.boomdrive.cloneDirs` is `agentplot/flume` at `/Users/alice/.local/share/boomdrive/repositories/agentplot/flume` and `agentplot/sluice` at `/Users/alice/.local/share/boomdrive/repositories/agentplot/sluice`, and nothing else

#### Scenario: Each kept repository's clone folder on NixOS

- **WHEN** the module is enabled for the user `alice`, whose home is `/home/alice`, and the account `own`'s drive `home` keeps the owner `agentplot`'s `flume` and `sluice`
- **THEN** `services.boomdrive.cloneDirs` is `agentplot/flume` at `/home/alice/.local/share/boomdrive/repositories/agentplot/flume` and `agentplot/sluice` at `/home/alice/.local/share/boomdrive/repositories/agentplot/sluice`, and nothing else

#### Scenario: The clone folder of another machine's drive

- **WHEN** crew's host is `host-a`, and the account `work`'s drive `garden`, on the machine `host-b`, keeps the owner `example`'s `alpha`
- **THEN** `services.boomdrive.cloneDirs` gives `example/alpha` its folder under `services.boomdrive.dataDir`, on either platform

#### Scenario: The clone folders follow the data folder

- **WHEN** the host gives the runner's job `XDG_DATA_HOME` `/srv/data`, and the drive `home` keeps `agentplot/flume`
- **THEN** `services.boomdrive.cloneDirs` gives `agentplot/flume` at `/srv/data/boomdrive/repositories/agentplot/flume`, on either platform

#### Scenario: No repository kept

- **WHEN** `services.boomdrive.accounts` names no repository
- **THEN** `services.boomdrive.cloneDirs` is empty, on either platform

#### Scenario: The clone folders with the runner not enabled

- **WHEN** the nix-darwin module is imported for the primary user `alice`, the drive `home` keeps `agentplot/flume`, and `services.boomdrive.enable` is not set
- **THEN** `services.boomdrive.cloneDirs` gives `agentplot/flume` at `/Users/alice/.local/share/boomdrive/repositories/agentplot/flume`

#### Scenario: The runner's own clone rule

- **WHEN** the module's rule and the runner's `clone::container` are given one data folder and one address, its owner and name each as written
- **THEN** each gives the same folder

### Requirement: No host sets a clone's folder

`services.boomdrive.cloneDirs` SHALL be read-only, so evaluation of it fails where a host sets it, or an entry of it, at any priority.

#### Scenario: A host that sets a clone's folder

- **WHEN** a host sets `services.boomdrive.cloneDirs."agentplot/flume" = "/srv/flume"`
- **THEN** evaluation of the option fails, on either platform, as nixpkgs fails a read-only option set beside its value

### Requirement: The README names each kept repository's clone folder

The README's **Installing with nix** SHALL name `services.boomdrive.cloneDirs`, what it holds and how each entry is keyed. It SHALL say the option is read-only and is read wherever a host names a clone's folder, and that it holds only the repositories `services.boomdrive.accounts` names.

#### Scenario: The README's clone folders

- **WHEN** the README's **Installing with nix** is read
- **THEN** it names `services.boomdrive.cloneDirs`
