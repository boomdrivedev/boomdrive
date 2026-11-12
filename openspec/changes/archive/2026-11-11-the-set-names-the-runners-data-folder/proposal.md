# Proposal

## Why

The brief: "The set derives the runner's data folder by the runner's own rule, $XDG_DATA_HOME/boomdrive as the runner's job sets it, else ~/.local/share/boomdrive under its user's home, and gives it as a read-only option of the runner's module that no host sets."

A host's example configuration of 2026-11-11, which evaluates against the flake bolt at `572919e4`, marks a gap: its doctor's manifest names the runner's data folder and a clone in it, and the flake gives no value for that folder. So the host keeps computing the folder in its own Nix. It also keeps the folder's name from before the rename (record 0035) for a host not yet moved. The host's sluice drivers' jobs and the repositories of crew's teams file compute the same folder. No unit planned a value of the flake's for it. The user, reading that example on 2026-11-11: "what about all of the gaps still in that example?"

agentplot/blueprints `design/principles.md` 14 has the set derive or default every value the recipe knows, its folders among them, and a host write only its own. Record 0036 has the set give crew each kept repository's folders, each `dir` under the runner's data folder. The runner takes no folder but its own rule's (`crates/boomdrive/src/config.rs`, `data_dir`: "and no other"). So the folder is a value the set derives, and no host's setting.

## What Changes

- **Each of the runner's modules names the runner's data folder.** The nix-darwin and NixOS modules each declare `services.boomdrive.dataDir`, read-only, beside the job it is derived from, as the nix-darwin module declares `logFile`. Its value is the runner's own rule: `$XDG_DATA_HOME/boomdrive` where the runner's job gives an absolute `XDG_DATA_HOME`, else `~/.local/share/boomdrive` under the runner's user's home. The job's `XDG_DATA_HOME` is the one its declaration gives: the agent's `EnvironmentVariables` on nix-darwin, the unit's `environment` on NixOS. With neither set, as the modules leave them, the folder is `~/.local/share/boomdrive`.
- **No host sets it.** A host that sets `services.boomdrive.dataDir` fails evaluation, as nixpkgs fails a read-only option set beside its value.
- **One file holds the rule.** `nix/modules/data-dir.nix` gives the folder from a home and an `XDG_DATA_HOME`, and both modules take it.
- **One table holds the module's rule to the runner's.** `nix/tests/data-dirs.json` lists cases, each a `HOME`, an `XDG_DATA_HOME` (set, unset, empty, relative or ending in `/`) and the folder. The flake's tests hold `data-dir.nix` to each case, and the runner's test `the_modules_data_folder_is_the_runners` holds `config::data_dir` to the same cases.
- **The README names the option** in **Installing with nix**: what it holds, that it is read-only, and that a host reads it wherever it names the folder or a clone in it.
- **Record 0036 is amended**, on 2026-11-11, with the runner's data folder, and **core.57** with it. Both are written with this change.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `service-modules`: the modules name the runner's data folder as a read-only option, by the runner's own rule from the job's own environment, and the README names it. The nix-darwin agent's environment is the module's `HOME` and `PATH`, beside any variable the host gives the job.

## Impact

- `nix/modules/data-dir.nix`: new. The runner's data folder from a home and an `XDG_DATA_HOME`, by the runner's rule.
- `nix/modules/darwin.nix`: declares `services.boomdrive.dataDir` from the primary user's home and the agent's `EnvironmentVariables`.
- `nix/modules/nixos.nix`: declares `services.boomdrive.dataDir` from `services.boomdrive.user`'s home and the unit's `environment`.
- `nix/tests/data-dirs.json`: new. The table of cases both rules are held to.
- `nix/tests/data-dir.nix`: new. The service-modules spec's data-folder scenarios on each platform, the table's cases, and the README's words. `nix/tests/default.nix` takes it.
- `crates/boomdrive/tests/module_files.rs`: the runner's own rule held to the table. `crates/boomdrive/moon.yml`: the test task takes the table as an input.
- `README.md`: **Installing with nix** names `services.boomdrive.dataDir`.
- `docs/adr/0036-…` and `docs/architecture/core.md`: record 0036's amendment of 2026-11-11 and core.57, written with this change.
- Nothing else changes. The runner's code, its file's keys, its data folder and the job's environment are as they are. No flake input, tool or tag moves. No other tool's option is set: crew's kept repositories read the option in the change `the-set-gives-crew-each-kept-repositorys-folders`, which builds on this one.
- Outside this repository: afterthought/swancloud reads `config.services.boomdrive.dataDir` in its doctor's manifest, its sluice drivers' jobs and its crew teams file's repositories, once it takes a set that gives it, and drops its own computation of the folder, the folder of before the rename with it (record 0035).

## Touches

```
nix/modules/data-dir.nix
nix/modules/darwin.nix
nix/modules/nixos.nix
nix/tests/data-dirs.json
nix/tests/data-dir.nix
nix/tests/default.nix
crates/boomdrive/tests/module_files.rs
crates/boomdrive/moon.yml
README.md
docs/adr/0036-the-set-gives-crew-each-kept-repositorys-folders-and-a-host-writes-only-its-own.md
docs/architecture/core.md
openspec/specs/service-modules/spec.md
```
