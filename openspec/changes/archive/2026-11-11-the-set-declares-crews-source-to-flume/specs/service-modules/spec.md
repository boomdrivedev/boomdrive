## ADDED Requirements

### Requirement: With crew on, the modules declare crew's source to Flume

Where `services.crew.enable` is true, whether or not `services.boomdrive.enable` is, the modules SHALL give Flume's module the source `crew`, `services.flume.sources.crew`, with `kinds_dir` set to `~/.local/share/flume/kinds/crew` and `roles` set to `~/.local/state/crew/flume/roles.toml`, and no other key (record 0034, point 2; crew record 0057, point 1).

#### Scenario: crew's source with crew on

- **WHEN** a system turns on `services.boomdrive` and crew, with its host and partitions, and sets no option of `services.flume.sources`
- **THEN** `services.flume.sources.crew` is exactly `kinds_dir = "~/.local/share/flume/kinds/crew"` and `roles = "~/.local/state/crew/flume/roles.toml"`, Flume's activation links a source rendered as `flume-source-crew.toml`, and the system fails no assertion, on nix-darwin and on NixOS

#### Scenario: crew's source without the runner

- **WHEN** a system imports the platform's `default` module, leaves `services.boomdrive.enable` unset, and turns on crew with its host and partitions
- **THEN** `services.flume.sources.crew` names the same `kinds_dir` and `roles`, on nix-darwin and on NixOS

### Requirement: With crew off, no source of crew's

Where `services.crew.enable` is false, the modules SHALL give Flume's module no source of crew's.

#### Scenario: No source of crew's with crew off

- **WHEN** a system turns on `services.boomdrive` and leaves crew off
- **THEN** `services.flume.sources` holds no `crew`, and Flume's activation links no `flume-source-crew.toml`, on nix-darwin and on NixOS

### Requirement: crew's source names what the set's crew writes

The `roles` and `kinds_dir` the modules give crew's source SHALL be the roles file and the kinds folder the set's crew writes, each under the home of the user it runs as, written under `~`.

#### Scenario: The paths the set's crew writes

- **WHEN** crew's `plugin/lib/source.py`, at the crew the lock names, is read for its `ROLES_FILE` and `KINDS_DIR`, each `Path.home() / "<path>"`
- **THEN** the source's `roles` is `~/` followed by `ROLES_FILE`'s path, and its `kinds_dir` is `~/` followed by `KINDS_DIR`'s

### Requirement: A host's own value in crew's source wins

The modules SHALL give each key of crew's source at the options' default priority, so a value a host sets in `services.flume.sources.crew` wins over the module's, and the key it does not set keeps the module's.

#### Scenario: A host's own kinds folder for crew

- **WHEN** a system turns on `services.boomdrive` and crew, with its host and partitions, and sets `services.flume.sources.crew.kinds_dir = "/srv/kinds/crew"`
- **THEN** the source's `kinds_dir` is `/srv/kinds/crew`, its `roles` is `~/.local/state/crew/flume/roles.toml`, and the system fails no assertion, on nix-darwin and on NixOS

### Requirement: The README names crew's source

The README's **Installing with nix** SHALL say that, with crew's module on, the module declares crew's source to Flume, `services.flume.sources.crew`, naming the roles file crew writes and its kinds folder, and that a host writes no file for it. No nix block in it SHALL set `services.flume.sources`.

#### Scenario: The README's words on crew's source

- **WHEN** the README's **Installing with nix** is read
- **THEN** it says `services.flume.sources.crew`, `~/.local/state/crew/flume/roles.toml` and `~/.local/share/flume/kinds/crew`, and no nix block in it sets `services.flume.sources`

## MODIFIED Requirements

### Requirement: The Boomdrive module writes no file of the other tools'

The modules' own activation SHALL link the runner's file, run herdr's own `plugin link` for the plugins `services.boomdrive.herdr.plugins` lists, and do nothing else of a tool's. Flume's, sluice's and crew's files, crew's source in Flume's folder among them, SHALL each be written and linked by that tool's own module, from its own `settings`, and for Flume its `sources`, as it writes them when imported without Boomdrive's.

#### Scenario: The tools' activation is their own

- **WHEN** a system with `services.boomdrive` and crew turned on is set beside one that imports Flume's, sluice's and crew's own modules alone, each turned on with the same settings, packages and user, and Flume's with crew's source declared as the modules declare it
- **THEN** on nix-darwin their activations are the same but for the runner's own link and herdr's plugins' links, and on NixOS their user activations are the same but for the runner's own
