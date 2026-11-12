# Verify: the-set-gives-crew-each-kept-repositorys-folders

The work on `change/the-set-gives-crew-each-kept-repositorys-folders` was read at 6e36d6e. The gate passes, the work meets every requirement and scenario of its change, and every task is ticked, so the change stays archived with this report.

- The gate ran once through sluice against `main` at 678ccfc. Every `check-fast` task passed, with 939 tests passed and none failed.
- The change's delta adds 5 requirements, with 12 scenarios. A case or test that passed in the gate asserts the outcome of each scenario.
- All 9 tasks are ticked. The code stage archived the change at 6e36d6e, as task 4.1 says to. This report sits beside the archived change.
- The branch holds `bolt/the-set-gives-crew-each-kept-repositorys-folders` at 5655a57, its base. `main` is at 678ccfc (#132), and the branch merges into it with no conflict.

## Checks

- **The gate.** `sluice exec main` at 6e36d6e ran the `check-fast` tasks that 6e36d6e affects against `main` at 678ccfc. It exited 0, and sluice recorded 37 results under their keys for 6e36d6e. moon completed 39 tasks.
  - Passed, every project's tasks:
    - `boomdrive`, `boomdrive-bounded`, `boomdrive-control`, `boomdrive-crew` and `boomdrive-hosted`: `format`, `lint`, `typecheck` and `test` each;
    - `nix:format`, `lint`, `typecheck` and `test`, the last two saying "every case holds";
    - `root:format`, `toolchain`, `adrs`, `specs` (23 of 23), `changes`, `commits` (532 commits, no errors), `releases` and `workflow`;
    - `site:install`, `check`, `prose`, `vocab`, `build`, `test` and `workflow`.
  - The 939 tests include `module_files.rs`'s `the_modules_views_are_the_runners`, with `the_modules_clone_folders_are_the_runners` and `the_modules_data_folder_is_the_runners`. All three passed.
  - moon warned that it could not write its shared cache or its output archives outside the sandbox. It also had no internet connection for a dependency install. None of these touched a task's result.
- The gate was run once, as this stage's brief says, and nothing was fixed here.

## Tasks

All 9 tasks of `tasks.md`, groups 1 to 4, are ticked `[x]`, and none is open. What they build is in the tree, as the design names it:

- **The view rule** (1.1 to 1.3):
  - `nix/modules/view-dir.nix` is `{ lib }: { home, root, address }:`. Each join is Rust's `PathBuf::push`: an absolute name replaces the base, and a `/` is added only where the base lacks one. The root is `Code` under the home where `root` is null, joined under the home where it begins with `~/`, and taken as written where it is absolute. Any other root throws, naming `services.boomdrive.settings.repository_root` and its value, and a null home throws where it is needed (design Decision 3). Its header names `config.rs`'s `view_path` and `root_path` and the table.
  - `nix/tests/view-dirs.json` holds the nine rows of design Decision 5, with the neutral homes `/h` and `/h/` (core.41).
  - `nix/tests/kept.nix` is `t: { … }`, and `nix/tests/default.nix`'s `test` takes it. Its cases "The runner's own view rule" and "A repository root the runner refuses" read each row with `builtins.tryEval`. A failing row is named by its `home`, `repository_root` and `address`, and what the rule gave.
  - `module_files.rs` gains `the_modules_views_are_the_runners`. It reads the table from `CARGO_MANIFEST_DIR` with `serde_json` and refuses an empty table. It parses each row's file, `accounts = {}` with the row's root written as a TOML string, by `config::parse` with the row's home. It asserts the parse is refused where the row's `folder` is null. Elsewhere it asserts `view_path(&config, &address).to_str()` is `Some(folder)`, text against text, naming the row. The file's module documentation names the table. `crates/boomdrive/moon.yml`'s `test` takes `/nix/tests/view-dirs.json` as an input, and its comment names it.
  - `root:format` passed with the table in the tree.
- **crew's kept repositories** (2.1 to 2.4):
  - `nix/modules/kept.nix` is `{ lib }: { cloneDirs, home, root }:`. It maps each address of `cloneDirs` to `{ dir; checkout; }`, the `dir` read from `cloneDirs` and the `checkout` from `view-dir.nix` (design Decision 2). Its header cites record 0036's point 1 and crew record 0061.
  - `darwin.nix` and `nixos.nix` each import it as `keptOf`. Under `lib.mkIf cfg.enable`, each gives `services.crew.settings.repositories` the map of `cfg.cloneDirs`, its own `home` and `cfg.settings.repository_root`. The map is given at the module's own priority, under `lib.mkIf (config.services.crew.enable && cfg.cloneDirs != { })` (design Decisions 1, 4 and 6). Each header comment names crew's kept repositories beside `cloneDirs`. On NixOS the entry sits beside `crew.user` in the module's `services` set.
  - The nix cases of Task notes 2.1, 2.2 and 2.3 are in `kept.nix`, each scenario the spec runs on either platform a case on each. `nix:test` passed with every case of `tools.nix`, `darwin.nix` and `nixos.nix` still holding, "The tools' activation is their own" among them. `nix:format` and `nix:lint` passed.
- **The words** (3.1): the README's **Installing with nix** gains a paragraph after `cloneDirs`'s, with no nix block (design Decision 7).
  - It names `services.crew.settings.repositories`, one entry for each address of `services.boomdrive.cloneDirs`, its `dir` the clone's folder and its `checkout` the runner's view, `<repository_root>/<owner>/<name>`, with the root `~/Code` where unset.
  - It says a host writes none, since an entry the host gives a kept repository with another `dir` or `checkout` fails evaluation.
  - It says a repository of the host's own that the runner does not keep stays the host's to name.
  - "The README's words", "… accounts", "… data folder", "… clone folders" and "… crew table" still hold.
- **The specs** (4.1): the delta is archived into `openspec/specs/service-modules`, and `root:specs` passes 23 of 23.

Record 0036's third amendment of 2026-11-11 and core.57's details are in the tree as the planning commit ada3883e wrote them.

`git diff 5655a57..6e36d6e` changes only these paths:

- the files the proposal's Touches lists;
- the change's own files;
- the spec its archive reconciled.

The runner's code, its file's keys, its data folder, its layout and its views are unchanged. No flake input, tag, tool or tracked file type is added. No option of crew's is declared.

## Requirements and scenarios

Each nix case is a case of `nix/tests/kept.nix`, run by `nix:test`. A case named "… on nix-darwin" and "… on NixOS" runs the scenario on each platform.

| requirement | scenario | asserted by |
|---|---|---|
| With the runner and crew's module on, the modules give crew each kept repository's folders | crew's kept repositories on nix-darwin | "crew's kept repositories on nix-darwin": `agentplot/flume` and `agentplot/sluice`, each `dir` under `/Users/alice/.local/share/boomdrive/repositories` and each `checkout` under `/Users/alice/Code`, compared whole |
| | crew's kept repositories on NixOS | "crew's kept repositories on NixOS": the same under `/home/alice`, compared whole |
| | crew's kept repository of another machine's drive | "crew's kept repository of another machine's drive on nix-darwin" and "… on NixOS": crew's host `host-a` and the drive `garden` on `host-b` give `example/alpha` the system's own `cloneDirs."example/alpha"` and `<home>/Code/example/alpha`, compared whole |
| | crew's checkouts under the host's repository root | "crew's checkouts under the host's repository root on nix-darwin" and "… on NixOS": `~/src` gives `<home>/src/agentplot/flume`, and `/srv/code` gives `/srv/code/agentplot/flume` |
| crew's checkout is the runner's view by the runner's own rule | The runner's own view rule | "The runner's own view rule" for the nix half, and `the_modules_views_are_the_runners` for the runner's, over the seven rows of `view-dirs.json` that name a folder: the root unset, `~/src`, `~/`, `~//srv/code`, `/srv/code` and `/srv/code/`, and a home ending in `/` |
| | A repository root the runner refuses | "A repository root the runner refuses" for the nix half, and `the_modules_views_are_the_runners` for the runner's, over the rows `src` and `$HOME/Code`: the rule's evaluation fails and `config::parse` refuses the file |
| Without the runner, crew's module or a kept repository, the modules give crew's kept repositories nothing | crew's module off | "crew's module off on nix-darwin" and "… on NixOS": `null` |
| | The runner not enabled | "The runner not enabled on nix-darwin" and "… on NixOS": the platform's module imported with crew on and the drive `home`, `services.boomdrive.enable` unset, gives `null` |
| | No repository kept beside crew | "No repository kept beside crew on nix-darwin" and "… on NixOS": `null` |
| No host writes a kept repository's folders | A host that gives a kept repository another folder | "A host that gives a kept repository another folder on nix-darwin" and "… on NixOS": a host's `dir`, and again its `checkout`, of `/srv/flume` for `agentplot/flume` makes crew's map fail to evaluate |
| | A host's own kept repository | "A host's own kept repository on nix-darwin" and "… on NixOS": `example/own` as the host gave it, beside the module's `agentplot/flume` and `agentplot/sluice`, compared whole |
| The README names crew's kept repositories | The README's kept repositories | "The README's kept repositories": `services.crew.settings.repositories` in **Installing with nix** |

The runner's half compares text, not `Path`s, so a doubled or trailing `/` that crew's text would hold fails it (design Decision 5). The nix cases read crew's map with `builtins.tryEval` over `builtins.deepSeq`, so a failing evaluation is told apart from a `null` value (Task notes 2.1).

## Outside this repository

afterthought/swancloud drops its own `services.crew.settings.repositories` once it takes a set that carries this change, as the proposal names. An entry of its that names another folder for a kept address then fails its evaluation. That work is the host's, and this change leaves it as it is.
