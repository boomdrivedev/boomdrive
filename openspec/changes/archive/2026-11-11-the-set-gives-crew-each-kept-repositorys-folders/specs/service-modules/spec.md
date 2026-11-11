## ADDED Requirements

### Requirement: With the runner and crew's module on, the modules give crew each kept repository's folders

With `services.boomdrive.enable` and `services.crew.enable`, each module SHALL give `services.crew.settings.repositories` one entry for each address of `services.boomdrive.cloneDirs`, and no other. Each entry's `dir` SHALL be that address's folder in `services.boomdrive.cloneDirs`, and its `checkout` SHALL be the runner's view of the address.

#### Scenario: crew's kept repositories on nix-darwin

- **WHEN** the module is enabled for the primary user `alice`, whose home is `/Users/alice`, crew's module is on with its host and partitions, and the account `own`'s drive `home` keeps the owner `agentplot`'s `flume` and `sluice`
- **THEN** `services.crew.settings.repositories` gives `agentplot/flume` the `dir` `/Users/alice/.local/share/boomdrive/repositories/agentplot/flume` and the `checkout` `/Users/alice/Code/agentplot/flume`, gives `agentplot/sluice` the same under its own name, and gives nothing else

#### Scenario: crew's kept repositories on NixOS

- **WHEN** the module is enabled for the user `alice`, whose home is `/home/alice`, crew's module is on with its host and partitions, and the account `own`'s drive `home` keeps the owner `agentplot`'s `flume` and `sluice`
- **THEN** `services.crew.settings.repositories` gives `agentplot/flume` the `dir` `/home/alice/.local/share/boomdrive/repositories/agentplot/flume` and the `checkout` `/home/alice/Code/agentplot/flume`, gives `agentplot/sluice` the same under its own name, and gives nothing else

#### Scenario: crew's kept repository of another machine's drive

- **WHEN** crew's module is on with the host `host-a`, and the account `work`'s drive `garden`, on the machine `host-b`, keeps the owner `example`'s `alpha`
- **THEN** `services.crew.settings.repositories` gives `example/alpha` the `dir` `services.boomdrive.cloneDirs` gives it and the `checkout` `Code/example/alpha` under the user's home, on either platform

#### Scenario: crew's checkouts under the host's repository root

- **WHEN** crew's module is on, the drive `home` keeps `agentplot/flume`, and `services.boomdrive.settings.repository_root` is `~/src`, and again `/srv/code`
- **THEN** `services.crew.settings.repositories` gives `agentplot/flume` the `checkout` `src/agentplot/flume` under the user's home in the first, and `/srv/code/agentplot/flume` in the second, on either platform

### Requirement: crew's checkout is the runner's view by the runner's own rule

The view the modules give crew SHALL be the runner's by the runner's own rule, `<repository_root>/<owner>/<name>`. Its root SHALL be `services.boomdrive.settings.repository_root`, or `~/Code` where that is unset, and `~/` SHALL be the home of the runner's user.

#### Scenario: The runner's own view rule

- **WHEN** the module's view rule and the runner's `config::view_path` are given one home, one address and one `repository_root`, the root unset, beginning with `~/`, `~/` alone, absolute, or absolute ending in `/`
- **THEN** each gives the same folder, as text

#### Scenario: A repository root the runner refuses

- **WHEN** the module's view rule and the runner's reading of its file are given a `repository_root` that is neither absolute nor begins with `~/`, such as `src` or `$HOME/Code`
- **THEN** the rule's evaluation fails and the runner refuses the file

### Requirement: Without the runner, crew's module or a kept repository, the modules give crew's kept repositories nothing

The modules SHALL give `services.crew.settings.repositories` nothing where `services.boomdrive.enable` or `services.crew.enable` is not set, or where `services.boomdrive.cloneDirs` names no address, so the host's own value stands, and crew's file names none where the host names none.

#### Scenario: crew's module off

- **WHEN** the module is enabled, the drive `home` keeps `agentplot/flume` and `agentplot/sluice`, and crew's module is not turned on
- **THEN** `services.crew.settings.repositories` is `null`, on either platform

#### Scenario: The runner not enabled

- **WHEN** the platform's module is imported with crew's module on and the drive `home`, and `services.boomdrive.enable` is not set
- **THEN** `services.crew.settings.repositories` is `null`, on either platform

#### Scenario: No repository kept beside crew

- **WHEN** the module is enabled, crew's module is on, and `services.boomdrive.accounts` names no repository
- **THEN** `services.crew.settings.repositories` is `null`, on either platform

### Requirement: No host writes a kept repository's folders

The modules SHALL give each entry of crew's kept repositories at their own priority, so evaluation of `services.crew.settings.repositories` fails where a host gives an address of `services.boomdrive.cloneDirs` another `dir` or another `checkout`. A host's own entry for an address `services.boomdrive.cloneDirs` does not name SHALL stand beside the modules' entries.

#### Scenario: A host that gives a kept repository another folder

- **WHEN** crew's module is on, the drive `home` keeps `agentplot/flume`, and a host gives `services.crew.settings.repositories."agentplot/flume".dir = "/srv/flume"`, and again `.checkout = "/srv/flume"`
- **THEN** evaluation of `services.crew.settings.repositories` fails in each, on either platform

#### Scenario: A host's own kept repository

- **WHEN** crew's module is on, the drive `home` keeps `agentplot/flume` and `agentplot/sluice`, and a host gives `services.crew.settings.repositories."example/own"` the `dir` `/srv/own` and the `checkout` `/srv/own/main`
- **THEN** `services.crew.settings.repositories` gives `example/own` as the host gave it, beside `agentplot/flume` and `agentplot/sluice` as the module gives them, on either platform

### Requirement: The README names crew's kept repositories

The README's **Installing with nix** SHALL say that, with the runner and crew's module on, the module gives `services.crew.settings.repositories` each address of `services.boomdrive.cloneDirs`, its `dir` the clone's folder and its `checkout` the runner's view. It SHALL say a host writes none, an entry it gives another folder failing evaluation, and that a repository of the host's own that the runner does not keep stays the host's to name.

#### Scenario: The README's kept repositories

- **WHEN** the README's **Installing with nix** is read
- **THEN** it names `services.crew.settings.repositories`
