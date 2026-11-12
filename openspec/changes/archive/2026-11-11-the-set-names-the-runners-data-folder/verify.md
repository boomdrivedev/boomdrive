# Verify: the-set-names-the-runners-data-folder

The work on `change/the-set-names-the-runners-data-folder` was read at bc1b8bf. The gate passes, the work meets every requirement and scenario of its change, and every task is ticked, so the change stays archived with this report.

- The gate ran once through sluice against `main` at 678ccfc. Every `check-fast` task passed, with 937 tests passed and none failed.
- The change's delta modifies 1 requirement and adds 4, with 10 scenarios. A case or test that passed in the gate asserts the outcome of each scenario.
- All 7 tasks are ticked. The code stage archived the change at bc1b8bf, as task 4.1 says to. This report sits beside the archived change.
- The branch holds `bolt/the-set-gives-crew-each-kept-repositorys-folders` at 57b1d1e, its base. `main` has moved on to 678ccfc (#132), and the branch merges into it with no conflict.

## Checks

- **The gate.** `sluice exec main` at bc1b8bf ran the `check-fast` tasks that bc1b8bf affects against `main` at 678ccfc. It exited 0, and sluice recorded 37 results under their keys for bc1b8bf. moon completed 39 tasks.
  - Passed, every project's tasks:
    - `boomdrive`, `boomdrive-bounded`, `boomdrive-control`, `boomdrive-crew` and `boomdrive-hosted`: `format`, `lint`, `typecheck` and `test` each;
    - `nix:format`, `lint`, `typecheck` and `test`, the last saying "every case holds";
    - `root:format`, `toolchain`, `adrs`, `specs` (23 of 23), `changes`, `commits`, `releases` and `workflow`;
    - `site:install`, `check`, `prose`, `vocab`, `build`, `test` and `workflow`.
  - The 937 tests include `module_files.rs`'s `the_modules_data_folder_is_the_runners`, which passed.
  - moon warned that it could not write its shared cache or its output archives outside the sandbox. It also had no internet connection for a dependency install. None of these touched a task's result.
- The gate was run once, as this stage's brief says, and nothing was fixed here.

## Tasks

All 7 tasks of `tasks.md`, groups 1 to 4, are ticked `[x]`, and none is open. What they build is in the tree, as the design names it:

- **The rule** (1.1, 1.2):
  - `nix/modules/data-dir.nix` is `{ lib }: { home, dataHome }:`. It takes `dataHome` only where its string starts with `/`, else `home` joined with `.local/share`, and joins `boomdrive` to that. Each join adds a `/` only where the base does not end in one (design Decision 2). Its header names `config.rs`'s `data_dir` and `xdg` and the table. Asked for the home-based folder with no home, it throws, naming the missing home.
  - `nix/tests/data-dirs.json` holds the five rows of design Decision 4, with the neutral home `/h` (core.41).
  - `nix/tests/data-dir.nix` is `t: { … }`, and `nix/tests/default.nix`'s `test` takes it. Its case "The runner's own rule" holds the rule to every row and names a failing row by its `HOME`, `XDG_DATA_HOME` and what the rule gave.
  - `module_files.rs` gains `the_modules_data_folder_is_the_runners`. It reads the table from `CARGO_MANIFEST_DIR` with `serde_json`, builds each row's `Env` by `config::Env::read` from a lookup of `HOME` and `XDG_DATA_HOME` alone, and asserts `config::data_dir` is the row's folder, naming the row. The file's module documentation names the table. `crates/boomdrive/moon.yml`'s `test` takes `/nix/tests/data-dirs.json` as an input, and its comment names it.
- **The option** (2.1 to 2.3):
  - `darwin.nix` imports the rule as `dataDirOf` and declares `services.boomdrive.dataDir` beside `logFile`. It is outside `config = lib.mkIf cfg.enable`, of `lib.types.path` and `readOnly = true`. Its default is taken from the primary user's home and `launchd.user.agents.boomdrive.serviceConfig.EnvironmentVariables.XDG_DATA_HOME or null`. Its `defaultText` and `description` say the rule, that the option is read-only, and that an `XDG_DATA_HOME` from launchd's user domain is not seen. The header comment names the option.
  - `nixos.nix` does the same beside `user`. Its default is taken from `services.boomdrive.user`'s home and `systemd.user.services.boomdrive.environment.XDG_DATA_HOME or null`, and its words name the user manager's `environment.d`.
  - The nix cases of Task notes 2.1 and 2.2 are in `data-dir.nix`. `nix:format` and `nix:lint` passed in the gate.
- **The words** (3.1, 3.2):
  - The README's **Installing with nix** gains a paragraph after the one on the agent and the unit. It names `services.boomdrive.dataDir`, what it holds and its rule, says it is read-only, read wherever a host names the folder or a clone in it and set nowhere, and says an `XDG_DATA_HOME` from elsewhere is not seen. It shows the job's own `XDG_DATA_HOME` on each platform.
  - `root:format` passed with the table in the tree.
- **The specs** (4.1): the delta is archived into `openspec/specs/service-modules`, and `root:specs` passes 23 of 23.
  - The code stage changed the modified requirement's words from "alone of the module's environment" to "alone by the module" at 1684fc9. The planned words took the requirement to 507 characters, past the 500 that `openspec validate --strict` allows. The meaning is the same.

Record 0036's amendment of 2026-11-11 and core.57, with its details, are in the tree as the planning commit b6d545b wrote them.

`git diff b6d545b..bc1b8bf` changes only these paths:

- the files the proposal's Touches lists;
- the change's own files;
- the spec its archive reconciled.

The runner's code, its file's keys and the job's environment are unchanged. No flake input, tag, tool or tracked file type is added. Neither module, the rule nor its tests names the folder of before the rename.

## Requirements and scenarios

Each nix case is a case of `nix/tests/data-dir.nix`, run by `nix:test`.

| requirement | scenario | asserted by |
|---|---|---|
| On nix-darwin the runner is a launchd agent of the primary user's login (modified) | The agent | `nix/tests/darwin.nix`'s case, unchanged and passing; with a host's `XDG_DATA_HOME`, "A job given its own XDG_DATA_HOME on nix-darwin" holds the agent's `HOME` and `PATH` to the module's |
| | No home for the primary user | `nix/tests/darwin.nix`'s case, unchanged and passing |
| The modules name the runner's data folder by the runner's own rule | The data folder on nix-darwin | "The data folder on nix-darwin": `/Users/alice/.local/share/boomdrive` |
| | The data folder on NixOS | "The data folder on NixOS": `/home/alice/.local/share/boomdrive` |
| | The data folder of another user on NixOS | "The data folder of another user on NixOS": `bob`, `/home/bob/.local/share/boomdrive` |
| | The data folder with the runner not enabled | "The data folder with the runner not enabled": `t.darwinOf [ darwinModules.default ]` |
| | The runner's own rule | "The runner's own rule" for the nix half, and `module_files.rs` `the_modules_data_folder_is_the_runners` for the runner's, over the five rows of `data-dirs.json`: unset, absolute, absolute ending in `/`, empty and relative |
| The data folder follows the job's own XDG_DATA_HOME | A job given its own XDG_DATA_HOME | "A job given its own XDG_DATA_HOME on nix-darwin" and "… on NixOS": `/srv/data/boomdrive` |
| | An XDG_DATA_HOME the runner takes as unset | "An XDG_DATA_HOME the runner takes as unset on nix-darwin" and "… on NixOS": `data` and `""` each give the home's folder |
| No host sets the runner's data folder | A host that sets the data folder | "A host that sets the data folder on nix-darwin" and "… on NixOS": reading the option fails |
| The README names the runner's data folder | The README's data folder | "The README's data folder": `services.boomdrive.dataDir` and `read-only` in **Installing with nix** |

"At any priority" is nixpkgs' own rule. It refuses a read-only option with more than one definition, its default among them, whatever their priorities, so a host's `mkForce` fails as a plain definition does. The cases set it at the normal priority alone.

"The README's words" and "The README's accounts", which tasks 3.1 names as still holding, are cases of `nix:test` and passed.

## Outside this repository

The proposal names afterthought/swancloud's reading of `config.services.boomdrive.dataDir` in its doctor's manifest, its sluice drivers' jobs and its crew teams file. Once it takes a set that gives the option, it drops its own computation of the folder, along with the folder of before the rename. That work is the host's, and this change leaves it as it is.
