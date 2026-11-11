# Proposal

## Why

The brief: "With crew's module on, the Boomdrive module gives crew's repositories an entry for each address the host's runner keeps, every drive's of every account, its dir the runner's clone and its checkout the runner's view, a check of the set failing where either differs from the runner's own, so a host writes none." Its sources are record 0036's point 1, core.57 and agentplot/blueprints `design/principles.md` 14.

crew's teams file names each repository its host keeps, with its `dir`, the folder crew names as the kit's and cuts worktrees in, and its `checkout`, the working tree it names as the kit's checkout (crew record 0061). crew's module at the tag the lock names declares them as `services.crew.settings.repositories`. On a runner's host both are the runner's: the clone's folder, which the set now states as `services.boomdrive.cloneDirs`, and the view, `<repository_root>/<owner>/<name>`. Nothing of the set gives them. A host's draft configuration of 2026-11-09 built the map in its own Nix from its accounts and the runner's layout, with the folder of before the rename. Principle 14 has the set derive every value the recipe knows, and core.57 names crew's kept repositories among them.

## What Changes

- **The modules give crew each kept repository's folders.** With the runner enabled and crew's module on, the nix-darwin and NixOS modules each give `services.crew.settings.repositories` one entry for each address of `services.boomdrive.cloneDirs`: every address of every drive of every account, another machine's drives among them. Its `dir` is the address's `cloneDirs` folder. Its `checkout` is the runner's view, by the runner's own rule, under the `repository_root` the runner's file is given.
- **One file holds the view rule.** `nix/modules/view-dir.nix` gives the runner's view from a home, a `repository_root` and an address, as the runner's `config::view_path` does: `~/Code` where the root is unset, a root beginning with `~/` under the home, an absolute one as written, each join Rust's `PathBuf::push`. A root the runner refuses fails evaluation, naming it. `nix/modules/kept.nix` makes crew's map from `cloneDirs`, the home and the root, and each platform module gives it with its own user's home.
- **One table holds the module's view rule to the runner's.** `nix/tests/view-dirs.json` lists cases, each a home, a `repository_root`, an address and the folder, or no folder where the runner refuses the root. The flake's tests hold `view-dir.nix` to each case. The runner's test `the_modules_views_are_the_runners` holds `config::view_path` to the same cases. `cloneDirs` is already held to `clone::container` by `nix/tests/clone-dirs.json`.
- **A host writes none.** Each entry is given at the module's own priority, so a host that gives a kept repository another `dir` or `checkout` fails evaluation, as nixpkgs fails an option given two values. A host's own repository that the runner does not keep stays beside the module's entries.
- **Nothing is given where nothing is kept.** With the runner off, crew's module off, or no repository kept, crew's `repositories` is the host's alone, and crew's file names none where the host names none.
- **The README names it** in **Installing with nix**, after `cloneDirs`'s paragraph.
- **Record 0036 is amended** a third time on 2026-11-11 with these forms, and **core.57**'s details with it. Both are written with this change.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `service-modules`: with the runner and crew's module on, the modules give crew's kept repositories each kept address's clone folder and view, by the runner's own rules, and the README names them.

## Impact

- `nix/modules/view-dir.nix`: new. The runner's view of one address, by the runner's rule.
- `nix/modules/kept.nix`: new. crew's kept repositories from `cloneDirs`, a home and a `repository_root`.
- `nix/modules/darwin.nix` and `nix/modules/nixos.nix`: each gives `services.crew.settings.repositories` with the runner enabled and crew's module on. Their header comments name it.
- `nix/tests/view-dirs.json`: new. The table of cases both view rules are held to.
- `nix/tests/kept.nix`: new. The service-modules spec's crew scenarios on each platform, the table's cases, and the README's words. `nix/tests/default.nix` takes it.
- `crates/boomdrive/tests/module_files.rs`: the runner's own `config::view_path` held to the table. `crates/boomdrive/moon.yml`: the test task takes the table as an input.
- `README.md`: **Installing with nix** names crew's kept repositories as the module gives them.
- `docs/adr/0036-…` and `docs/architecture/core.md`: record 0036's third amendment of 2026-11-11 and core.57's details, written with this change.
- Nothing else changes. The runner's code, its file's keys, its data folder, its layout and its views are as they are. No flake input, tool or tag moves: crew's module at the lock's tag, v0.50.2, already declares `repositories`, and crew there reads it. No option of crew's is declared here (the service-modules spec's "Each tool's options are its own flake's").
- Outside this repository: afterthought/swancloud drops its own `services.crew.settings.repositories` once it takes a set that carries this change. A value of its that names another folder for a kept address then fails its evaluation.

## Touches

```
nix/modules/view-dir.nix
nix/modules/kept.nix
nix/modules/darwin.nix
nix/modules/nixos.nix
nix/tests/view-dirs.json
nix/tests/kept.nix
nix/tests/default.nix
crates/boomdrive/tests/module_files.rs
crates/boomdrive/moon.yml
README.md
docs/adr/0036-the-set-gives-crew-each-kept-repositorys-folders-and-a-host-writes-only-its-own.md
docs/architecture/core.md
openspec/specs/service-modules/spec.md
```
