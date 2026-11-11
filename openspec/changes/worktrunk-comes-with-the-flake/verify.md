# Verify: worktrunk-comes-with-the-flake

The work on `change/worktrunk-comes-with-the-flake` was read at dc8c4c1, which merges `bolt/the-flake-brings-its-companion-tools` at 73cb3ae, with `portless-comes-with-the-flake` landed. The gate passes, and the work holds against its change:

- every requirement and scenario of the deltas has a case in the tree, and each passed in the gate;
- every task, 1.1 to 4.2, is ticked;
- every path the branch changes from 73cb3ae is one the proposal's Touches lists, or one of the change's own files;
- each delta's modified requirement is the bolt's, as `openspec/specs` holds it at 73cb3ae, with worktrunk added and nothing of portless's dropped.

So this stage archives the change.

## Checks

`sluice exec main` at dc8c4c1 ran the `check-fast` tasks that dc8c4c1 affects, against `main` at bb240e8. It exited 0. moon completed 39 tasks, and sluice recorded 37 results, every one a pass, under their keys for dc8c4c1.

- **nix:** `nix:format`, `nix:lint`, `nix:typecheck` and `nix:test`, each "every case holds".
- **root:**
  - `root:adrs` (41 records), `root:changes` (1 passed, 0 failed), `root:commits` (no errors in 531 commits);
  - `root:format` (330 files, none changed), `root:releases`, `root:specs` (23 passed, 0 failed), `root:toolchain` and `root:workflow`.
- **The crates:** each one's `format`, `lint`, `typecheck` and `test`. The tests:
  - `boomdrive`: 578 passed, 0 failed;
  - `boomdrive-hosted`: 173 passed, 0 failed;
  - `boomdrive-crew`: 135 passed, 0 failed;
  - `boomdrive-control`: 44 passed, 0 failed;
  - `boomdrive-bounded`: 6 passed, 0 failed.
- **site:** `site:install`, `site:check` (0 errors), `site:vocab`, `site:prose`, `site:build` (11 pages), `site:test` and `site:workflow`.
- **Warnings.** moon could not write its shared cache or its output archives outside the sandbox, and found no network for the rust toolchain's install. sluice named `.mcp.json` and `.claude/settings.json` as conflicting files. None touched a task's result.
- The gate was run once, as this stage's brief says, and nothing was fixed here.

## Tasks

All 10 tasks, 1.1 to 4.2, are ticked `[x]`, and none is open. What each names is in the tree:

- **1.1:**
  - `flake.nix` takes `worktrunk` at `github:max-sixty/worktrunk/v0.80.0`, following the flake's `nixpkgs` and `rust-overlay`.
  - The lock's `worktrunk` node locks rev `b49ca7e` with `original.ref` `v0.80.0`, its `nixpkgs` and `rust-overlay` following the root's, and brings worktrunk's own `crane` and `flake-utils`, with no second `nixpkgs` or `rust-overlay`.
  - `packagesOf` gives `worktrunk` from `nix/packages/worktrunk.nix`, which gives the package its tag as `VERGEN_GIT_DESCRIBE` and changes nothing else.
  - `typecheck.nix`'s set holds `worktrunk` beside `portless`, eleven packages.
  - `nix:typecheck` passed.
- **1.2:**
  - `packages.nix` holds "worktrunk through its own flake", "worktrunk is sluice's", "worktrunk other than sluice's" and "worktrunk named by no release tag".
  - "worktrunk is sluice's" reads sluice's `devenv.nix` at the tag the lock names for sluice, `v0.78.1`, and fails naming the file where it cannot read one pin.
  - `tags.nix`'s list of tools named by a release tag holds `worktrunk` beside `portless`.
  - `nix:test` passed.
- **1.3:** "The runner from this tree" leaves out the top-level entries `.gitignore` names, which git never tracks, and `nix/moon.yml` gives `nix:test` the input `/.gitignore`. The case still holds the source to `Cargo.toml`, `Cargo.lock` and `crates`, and every file under `crates/` as taken. `nix:test` passed.
- **2.1:** `nix/modules/apps.nix` declares `services.boomdrive.worktrunk.package` as `installed "worktrunk"`, and puts it in `environment.systemPackages` beside portless's. `nix/tests/apps.nix` holds the four cases on each platform, under names of their own beside portless's. `nix:test` passed.
- **2.2:** `cache.nix`'s `names` holds `worktrunk` beside `portless`. `nix:test` passed.
- **2.3:** `nix/shell.nix` holds `set.worktrunk` beside `set.portless`, and `way-in.nix`'s `shellTools` holds the seven. `nix:test` passed.
- **3.1:**
  - The README's **Installing with nix** names worktrunk in the set, through its own flake at the release sluice's tag pins, and `github:agentplot/boomdrive#worktrunk` on the profile line.
  - It names `services.boomdrive.worktrunk.package` among the five options, with `null`, a host's own `wt`, the shell's worktrunk, and `wt config shell install` as the person's own.
  - `cache.nix`'s "The README's words" reads the option, and `nix:test` passed.
- **3.2:** CLAUDE.md and `openspec/config.yaml` name worktrunk in the set and among what the modules install, beside portless. `root:format` passed.
- **4.1** and **4.2:** The change stands beside portless: the set gives eleven packages and the shell seven, the modules install portless and worktrunk with the runner, and the README, CLAUDE.md, `openspec/config.yaml`, record 0023, core.44 and core.46 name both. With the change open, `root:changes` and `root:specs` passed. This stage archives it once, the report with it.

## Requirements and scenarios

Each scenario's case is in the tree, and passed under the task named.

| spec | requirement | scenario | case | task |
|---|---|---|---|---|
| family-packages | The flake gives the family's host tools as packages | Every system's packages | `typecheck.nix`, the eleven and `default` | `nix:typecheck` |
| family-packages | The flake gives the family's host tools as packages | No tool of the host's own | `typecheck.nix` | `nix:typecheck` |
| family-packages | The flake and its lock name one set | A tag moved without the lock; A tool named by no release tag; herdr named by no release tag; portless named by no release tag | `packages.nix`, unchanged | `nix:test` |
| family-packages | The flake and its lock name one set | worktrunk named by no release tag | `packages.nix`, with `tags.nix` holding `worktrunk` | `nix:test` |
| family-packages | Each package is made from this tree, its tool's own flake or source at its release tag, or the flake's nixpkgs | Flume, sluice, crew and herdr through their own flakes; portless from its source; portless other than its tag | `packages.nix`, unchanged | `nix:test` |
| family-packages | Each package is made from this tree, … | worktrunk through its own flake | `packages.nix`: source, `cargoArtifacts`, `wt`, version `0.80.0`, `VERGEN_GIT_DESCRIBE` `v0.80.0` | `nix:test` |
| family-packages | Each package is made from this tree, … | The runner from this tree | `packages.nix`, its listing leaving out what `.gitignore` names | `nix:test` |
| family-packages | worktrunk is the release sluice's worktree driver was checked against | worktrunk is sluice's | `packages.nix`, reading sluice's `devenv.nix` at its locked tag | `nix:test` |
| family-packages | worktrunk is the release sluice's worktree driver was checked against | worktrunk other than sluice's | `packages.nix`, `worktrunkHeld` with `0.81.0` and `0.80.0` | `nix:test` |
| service-modules | The modules hold no option that installs a tool its own module installs | The tools option is gone | `darwin.nix` and `nixos.nix`, unchanged | `nix:test` |
| service-modules | The modules hold no option that installs a tool its own module installs | The README's words | `cache.nix`, with `services.boomdrive.portless.package` and `services.boomdrive.worktrunk.package` | `nix:test` |
| service-modules | The modules install worktrunk, replaceable or dropped | worktrunk with the runner; A host's own worktrunk; No worktrunk of the set's; No worktrunk without the runner | `apps.nix`, each on nix-darwin and NixOS | `nix:test` |
| binary-cache | The workflow builds and pushes every package of every system | Each system has its job; The runner is pushed with its tools; A package that does not build; A branch's push | `workflow.nix`, which holds the workflow to build every package the flake lists, `worktrunk` among them | `nix:test` |
| binary-cache | A host substitutes the set, or builds the same paths | A host's own nixpkgs | `cache.nix`, `names` holding `portless` and `worktrunk` | `nix:test` |
| binary-cache | A host substitutes the set, or builds the same paths | The host's trust | `cache.nix`, unchanged | `nix:test` |
| way-in | The flake gives a shell that starts herdr | Every system's shell | `way-in.nix`, seven packages | `nix:test` |
| way-in | The flake gives a shell that starts herdr | Entering the shell; A command through the shell; Inside herdr already | `nix/tests/way-in.sh`, unchanged | `nix:test` |

Each modified requirement was read against `openspec/specs` at 73cb3ae. Each differs only by worktrunk's name, its added scenarios, and the counts eleven and seven. The way-in requirement's sentence on a command and a shell inside herdr is reworded, keeping its meaning, so it stays within openspec's strict length.

## Beyond the change

- **Records and rules.** Record 0023's amendment of 2026-11-11 for worktrunk, and core.44's and core.46's details naming portless and worktrunk, are in the tree.
- **The runner's code.** It does not change, and no crate, gate tool or tracked file type is added.
- **Outside this repository.** afterthought/swancloud, once it takes the flake, drops its hosts' own worktrunk for the set's, or sets `services.boomdrive.worktrunk.package` to its own or to `null` (record 0023). That is its own work, which this change names and does not do.
