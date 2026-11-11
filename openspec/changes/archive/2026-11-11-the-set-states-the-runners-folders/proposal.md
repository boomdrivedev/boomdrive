# Proposal

## Why

The brief: "Each kept repository's clone folder is a read-only value of the runner's Boomdrive module, derived by the runner's own rule under the runner's data folder and read as logFile is, which no host sets; its readers are crew's kept repositories, the doctor's manifest and, until sluice's host process replaces them, the sluice drivers' jobs, so none of them computes a clone's folder."

The set now names the runner's data folder, `services.boomdrive.dataDir` (record 0036 as amended on 2026-11-11). A clone's folder under it is still each reader's to join: `<dataDir>/repositories/<owner>/<name>`, by the runner's own layout. A host's example configuration of 2026-11-11 joins it three times: in its doctor's manifest, in its sluice drivers' jobs and in crew's teams file. The change `the-set-gives-crew-each-kept-repositorys-folders` would join it a fourth time for crew's `dir`. Each join is the runner's layout written again, and none fails when the runner's moves.

agentplot/blueprints `design/principles.md` 14 has the set derive every value the recipe knows, its folders among them. afterthought/swancloud's record 0059, decision 4, has the fleet read the runner's layout from the flake and never compute it, and its amendment of 2026-11-11 waits on this change to end its exception. Record 0034's point 6 reads a fact of the runner's job from one home and sets it nowhere, as `logFile` is.

## What Changes

- **Each of the runner's modules states each kept repository's clone folder.** The nix-darwin and NixOS modules each declare `services.boomdrive.cloneDirs`, read-only, whether or not the runner is enabled. It holds one entry for each address of every drive of every account in `services.boomdrive.accounts`, another machine's drives among them. Each entry is keyed by its address as written, `<owner>/<name>`, and its value is the runner's clone folder, `<dataDir>/repositories/<owner>/<name>`: the folder `flume launch --repository` names.
- **No host sets it.** A host that sets `services.boomdrive.cloneDirs`, or an entry of it, fails evaluation, as one that sets `dataDir` does.
- **One file holds the rule.** `nix/modules/clone-dir.nix` gives a clone's folder from a data folder and an address. The module that declares the option, `nix/modules/clones.nix`, takes it, and both platform modules import that module.
- **One table holds the module's rule to the runner's.** `nix/tests/clone-dirs.json` lists cases, each a data folder, an address and the folder. The flake's tests hold `clone-dir.nix` to each case. The runner's test `the_modules_clone_folders_are_the_runners` holds `clone::container` to the same cases.
- **The README names the option** in **Installing with nix**, beside `dataDir`: what it holds, that it is read-only, and that a host reads it wherever it names a clone's folder. `dataDir`'s paragraph and its option's description then send a reader of a clone's folder to `cloneDirs`.
- **Record 0036 is amended** a second time on 2026-11-11, with each kept repository's clone folder, and **core.57** with it. Both are written with this change.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `service-modules`: the modules state each kept repository's clone folder as a read-only option by the runner's own rule, and the README names it. The README's data folder sends a reader of a clone's folder to it.

## Impact

- `nix/modules/clone-dir.nix`: new. A clone's folder from a data folder and an address, by the runner's rule.
- `nix/modules/clones.nix`: new. Declares `services.boomdrive.cloneDirs` from `services.boomdrive.accounts` and `services.boomdrive.dataDir`.
- `nix/modules/darwin.nix` and `nix/modules/nixos.nix`: import `./clones.nix`. Their header comments and `dataDir`'s description name `cloneDirs` for a clone's folder.
- `nix/tests/clone-dirs.json`: new. The table of cases both rules are held to.
- `nix/tests/clone-dirs.nix`: new. The service-modules spec's clone-folder scenarios on each platform, the table's cases, and the README's words. `nix/tests/default.nix` takes it.
- `crates/boomdrive/tests/module_files.rs`: the runner's own `container` held to the table. `crates/boomdrive/moon.yml`: the test task takes the table as an input.
- `README.md`: **Installing with nix** names `services.boomdrive.cloneDirs`, and its data folder's paragraph sends a reader of a clone's folder to it.
- `docs/adr/0036-…` and `docs/architecture/core.md`: record 0036's second amendment of 2026-11-11 and core.57, written with this change.
- Nothing else changes. The runner's code, its file's keys, its data folder, its layout and the job's environment are as they are. No flake input, tool or tag moves. No other tool's option is set: crew's kept repositories read `cloneDirs` in the change `the-set-gives-crew-each-kept-repositorys-folders`, which builds on this one.
- Outside this repository: afterthought/swancloud reads `config.services.boomdrive.cloneDirs` in its doctor's manifest and its sluice drivers' jobs, once it takes a set that carries it, and computes no clone's folder (its record 0059, decision 4).

## Touches

```
nix/modules/clone-dir.nix
nix/modules/clones.nix
nix/modules/darwin.nix
nix/modules/nixos.nix
nix/tests/clone-dirs.json
nix/tests/clone-dirs.nix
nix/tests/default.nix
crates/boomdrive/tests/module_files.rs
crates/boomdrive/moon.yml
README.md
docs/adr/0036-the-set-gives-crew-each-kept-repositorys-folders-and-a-host-writes-only-its-own.md
docs/architecture/core.md
openspec/specs/service-modules/spec.md
```
