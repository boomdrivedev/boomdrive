# Verify: the-set-states-the-runners-folders

The work on `change/the-set-states-the-runners-folders` was read at 80dabf0. The gate passes, the work meets every requirement and scenario of its change, and every task is ticked, so the change stays archived with this report.

- The gate ran once through sluice against `main` at 678ccfc. Every `check-fast` task passed, with 938 tests passed and none failed.
- The change's delta modifies 1 requirement and adds 3, with 10 scenarios. A case or test that passed in the gate asserts the outcome of each scenario.
- All 8 tasks are ticked. The code stage archived the change at 80dabf0, as task 4.1 says to. This report sits beside the archived change.
- The branch holds `bolt/the-set-gives-crew-each-kept-repositorys-folders` at 65de6eb, its base. `main` is at 678ccfc (#132), and the branch merges into it with no conflict.

## Checks

- **The gate.** `sluice exec main` at 80dabf0 ran the `check-fast` tasks that 80dabf0 affects against `main` at 678ccfc. It exited 0, and sluice recorded 37 results under their keys for 80dabf0. moon completed 39 tasks.
  - Passed, every project's tasks:
    - `boomdrive`, `boomdrive-bounded`, `boomdrive-control`, `boomdrive-crew` and `boomdrive-hosted`: `format`, `lint`, `typecheck` and `test` each;
    - `nix:format`, `lint`, `typecheck` and `test`, the last two saying "every case holds";
    - `root:format`, `toolchain`, `adrs`, `specs` (23 of 23), `changes`, `commits` (521 commits, no errors), `releases` and `workflow`;
    - `site:install`, `check`, `prose`, `vocab`, `build`, `test` and `workflow`.
  - The 938 tests include `module_files.rs`'s `the_modules_clone_folders_are_the_runners` and `the_modules_data_folder_is_the_runners`, which passed.
  - moon warned that it could not write its shared cache or its output archives outside the sandbox. It also had no internet connection for a dependency install. None of these touched a task's result.
- The gate was run once, as this stage's brief says, and nothing was fixed here.

## Tasks

All 8 tasks of `tasks.md`, groups 1 to 4, are ticked `[x]`, and none is open. What they build is in the tree, as the design names it:

- **The rule** (1.1 to 1.3):
  - `nix/modules/clone-dir.nix` is `{ dataDir, address }:`, giving `"${toString dataDir}/repositories/${address}"` (design Decision 2). Its header names `clone.rs`'s `container` as the rule it follows, says why one `/` before `repositories` is the runner's join, and names the table.
  - `nix/tests/clone-dirs.json` holds the three rows of design Decision 5, with the neutral data folders `/d/boomdrive` and `/x/data/boomdrive` (core.41). The third row's `Example-1/a.b_c` keeps a capital, a hyphen, a `.` and a `_` as written.
  - `nix/tests/clone-dirs.nix` is `t: { … }`, and `nix/tests/default.nix`'s `test` takes it. Its case "The runner's own clone rule" holds the rule to every row and names a failing row by its `dataDir`, its `address` and what the rule gave.
  - `module_files.rs` gains `the_modules_clone_folders_are_the_runners`. It reads the table from `CARGO_MANIFEST_DIR` with `serde_json`, refuses an empty table, parses each row's address by `config::Address::parse`, and asserts `clone::container` of the row's `dataDir` is the row's folder, naming the row. The file's module documentation names the table. `crates/boomdrive/moon.yml`'s `test` takes `/nix/tests/clone-dirs.json` as an input, and its comment names it.
  - `root:format` passed with the table in the tree.
- **The option** (2.1 to 2.3):
  - `nix/modules/clones.nix` is `{ config, lib, ... }:`. It takes `cloneDirOf` from `clone-dir.nix` and `drivesOf` from `accounts.nix`, and declares `services.boomdrive.cloneDirs` as `attrsOf path`, `readOnly = true`. Its default is one entry for each address of every drive, keyed by the address. Its `defaultText` and `description` are as design Decision 1 has them, and its header cites record 0036 as amended on 2026-11-11.
  - `darwin.nix` and `nixos.nix` each import `./clones.nix` beside `./drives.nix`. Each `dataDir` description says a clone's folder is `services.boomdrive.cloneDirs`'s, and each header comment names `cloneDirs` beside `dataDir` (design Decision 6).
  - The nix cases of Task notes 2.1 are in `clone-dirs.nix`, each scenario the spec runs on either platform a case on each. `nix:typecheck`, `nix:format` and `nix:lint` passed in the gate.
- **The words** (3.1):
  - The README's **Installing with nix** gains a paragraph after the data folder's. It names `services.boomdrive.cloneDirs`, its entries keyed by `<owner>/<name>` and each folder `<dataDir>/repositories/<owner>/<name>`. It says the option holds only the repositories `services.boomdrive.accounts` names, none the local layer alone adds, and that it is read-only, read wherever a host names a clone's folder and set nowhere. It shows a reader's line in a nix block.
  - `dataDir`'s paragraph now says a clone's folder in it is `services.boomdrive.cloneDirs`'s.
- **The specs** (4.1): the delta is archived into `openspec/specs/service-modules`, and `root:specs` passes 23 of 23.

Record 0036's second amendment of 2026-11-11 and core.57, with its details, are in the tree as the planning commit f7a8de9 wrote them.

`git diff f7a8de9..80dabf0` changes only these paths:

- the files the proposal's Touches lists;
- the change's own files;
- the spec its archive reconciled.

The runner's code, its file's keys, its data folder, its layout and the job's environment are unchanged. No flake input, tag, tool or tracked file type is added. No other tool's option is set.

## Requirements and scenarios

Each nix case is a case of `nix/tests/clone-dirs.nix`, run by `nix:test`.

| requirement | scenario | asserted by |
|---|---|---|
| The README names the runner's data folder (modified) | The README's data folder | `nix/tests/data-dir.nix`'s "The README's data folder", unchanged and passing: `services.boomdrive.dataDir` and `read-only` in **Installing with nix** |
| The modules state each kept repository's clone folder by the runner's own rule | Each kept repository's clone folder on nix-darwin | "Each kept repository's clone folder on nix-darwin": both folders under `/Users/alice/.local/share/boomdrive/repositories`, compared whole |
| | Each kept repository's clone folder on NixOS | "Each kept repository's clone folder on NixOS": both folders under `/home/alice/.local/share/boomdrive/repositories`, compared whole |
| | The clone folder of another machine's drive | "The clone folder of another machine's drive on nix-darwin" and "… on NixOS": the drive `garden` on `host-b`, crew's host `host-a`, gives `example/alpha` under the system's own `dataDir` |
| | The clone folders follow the data folder | "The clone folders follow the data folder on nix-darwin" and "… on NixOS": `/srv/data/boomdrive/repositories/agentplot/flume` |
| | No repository kept | "No repository kept on nix-darwin" and "… on NixOS": `{ }` |
| | The clone folders with the runner not enabled | "The clone folders with the runner not enabled": `t.darwinOf [ darwinModules.default home ]` |
| | The runner's own clone rule | "The runner's own clone rule" for the nix half, and `module_files.rs` `the_modules_clone_folders_are_the_runners` for the runner's, over the three rows of `clone-dirs.json` |
| No host sets a clone's folder | A host that sets a clone's folder | "A host that sets a clone's folder on nix-darwin" and "… on NixOS": reading the option fails |
| The README names each kept repository's clone folder | The README's clone folders | "The README's clone folders": `services.boomdrive.cloneDirs` in **Installing with nix** |

"At any priority" is nixpkgs' own rule. It refuses a read-only option with more than one definition, its default among them, whatever their priorities, so a host's `mkForce` fails as a plain definition does. The cases set an entry at the normal priority alone.

"The data folder on nix-darwin" and "The data folder on NixOS", which task 2.1 names as still holding, and "The README's words" and "The README's accounts", which task 3.1 names, are cases of `nix:test` and passed.

## Outside this repository

The proposal names afterthought/swancloud's reading of `config.services.boomdrive.cloneDirs` in its doctor's manifest and its sluice drivers' jobs. Once it takes a set that carries the option, it computes no clone's folder (its record 0059, decision 4). That work is the host's, and this change leaves it as it is. crew's kept repositories read `cloneDirs` in the change `the-set-gives-crew-each-kept-repositorys-folders`, which builds on this one.
