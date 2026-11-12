# Verify: the-set-declares-crews-source-to-flume

The work on `change/the-set-declares-crews-source-to-flume` was read at 7b51648. The gate passes, the work meets every requirement and scenario of its change, and every task is ticked, so the change stays archived with this report.

- The gate ran once through sluice against `main` at 678ccfc. Every `check-fast` task that 7b51648 affects passed: 578 tests passed and none failed, and every nix case holds.
- The change's delta adds 5 requirements with 6 scenarios, and modifies 1 requirement and its 1 scenario. A case that passed in the gate asserts the outcome of each scenario.
- All 7 tasks are ticked. The code stage archived the change at 7b51648, as task 2.3 says to. This report sits beside the archived change.
- The branch was cut from `bolt/the-set-declares-crews-source-to-flume` at 57b1d1e. `main` has moved on by #132, whose three commits touch only devenv's copies of Claude Code's files, and the branch merges into `main` with no conflict.

## Checks

- **The gate.** `sluice exec main` at 7b51648 ran the `check-fast` tasks that 7b51648 affects against `main` at 678ccfc. It exited 0, and sluice recorded 8 results under their keys for 7b51648. moon completed 8 tasks, and all 8 passed:
  - `nix:format`, `nix:lint`, `nix:typecheck` and `nix:test`. `nix:typecheck` and `nix:test` each said "every case holds".
  - `boomdrive:test`, with 578 tests passed and none failed.
  - `root:format`, which formatted 329 files and changed none.
  - `root:specs`, 23 of 23, with `service-modules` among them.
  - `root:changes`, which found no open change.
- The tasks that the change does not affect were not run by sluice.
- moon warned that it could not write its shared cache or its output archives outside the sandbox. It also had no internet connection for a dependency install. sluice said `.mcp.json` and `.claude/settings.json` conflict, from devenv's files, which #132 on `main` changes. None of these touched a task's result.
- The gate was run once, as this stage's brief says, and nothing was fixed here.

## Tasks

All 7 tasks of `tasks.md`, groups 1 and 2, are ticked `[x]`, and none is open. What they build is in the tree, as the design names it:

- **crew's source in Flume's module** (1.1 to 1.4):
  - `nix/modules/sources.nix` takes `config` and `lib` alone. Under `lib.mkIf config.services.crew.enable` it gives `services.flume.sources.crew` its `kinds_dir`, `~/.local/share/flume/kinds/crew`, and its `roles`, `~/.local/state/crew/flume/roles.toml`, each by `lib.mkDefault`, under one attribute set.
  - Its header comment cites record 0034, point 2, crew record 0057, point 1, core.18, core.46 and core.5. It says the module writes no file.
  - `darwin.nix` and `nixos.nix` import `./sources.nix` after `./drives.nix`, and their header comments name it with record 0034.
  - `nix/tests/sources.nix` is in `default.nix`'s `test`, after `./drives.nix`.
  - `lib.nix`'s `toolsOf` declares the same two values as plain strings in Flume's own module, and its comment says so.
  - `sources.nix`'s `crewsPath` reads `plugin/lib/source.py` from `t.inputs.crew`. It matches `<prefix> = Path.home() / "<path>"` line by line. It fails naming the line where none matches, and naming both values where they differ.
  - `nix:format` and `nix:lint` pass.
- **The words** (2.1 to 2.3):
  - The README's **Installing with nix**, after the sentence on crew's `host` and partitions, names `services.flume.sources.crew` and its two paths. It says Flume's module writes the source at `~/.config/flume/sources/crew.toml`, that a host writes no file for it, and that a key the host sets there wins. No nix block of it sets `services.flume.sources`.
  - CLAUDE.md's paragraph on the flake gains ", and, with crew on, crew's source of kinds (record 0034)".
  - The delta is archived into `openspec/specs/service-modules`, and `root:specs` passes 23 of 23.

core.57 and its details are amended in 07304c9, the commit that writes the change, as the tasks' preamble asks. The rule names crew's source among the facts the set gives through a tool's module option, citing records 0036 and 0034, point 2. Its details rule out a host's own `sources/crew.toml`, or a writer of it, beside the set. They allow a host's own key through Flume's option, and cite crew record 0057, point 1.

Against the base, 57b1d1e, the branch changes only these paths:

- the files the proposal's Touches lists;
- the change's own files;
- the spec its archive reconciled.

No flake input or tag moves. No crate, tool or tracked file type is added. The runner's code and its file do not change.

## Requirements and scenarios

Every case is in `nix/tests/sources.nix` unless the row names another file. A case named "on each" runs once on nix-darwin and once on NixOS.

| requirement | scenario | asserted by |
|---|---|---|
| With crew on, the modules declare crew's source to Flume | crew's source with crew on | "crew's source with crew on", on each: the source is exactly the two keys, the activation holds `flume-source-crew.toml`, and no assertion fails |
| | crew's source without the runner | "crew's source without the runner", on each: the platform's `default` module, the runner unset, the same source |
| With crew off, no source of crew's | No source of crew's with crew off | "No source of crew's with crew off", on each: no `crew` in `services.flume.sources`, and no `flume-source-crew.toml` in the activation |
| crew's source names what the set's crew writes | The paths the set's crew writes | "The paths the set's crew writes": `roles` and `kinds_dir` equal `~/` and `ROLES_FILE`'s and `KINDS_DIR`'s paths in crew's `source.py` at the lock's crew |
| A host's own value in crew's source wins | A host's own kinds folder for crew | "A host's own kinds folder for crew", on each: `kinds_dir` is `/srv/kinds/crew`, `roles` keeps the module's, and no assertion fails |
| The README names crew's source | The README's words on crew's source | "The README's words on crew's source": the three strings, and no nix block setting `services.flume.sources` |
| The Boomdrive module writes no file of the other tools' (modified) | The tools' activation is their own | `nix/tests/tools.nix`, "The tools' activation is their own", on each, against `lib.nix`'s tools' systems, which now declare crew's source |

"The README's words" and "The README's lines", in `nix/tests/cache.nix`, still hold. Every case passed in `nix:test`.

## Outside this repository

A host that takes the set with crew on, afterthought/swancloud's among them, drops its own `~/.config/flume/sources/crew.toml` and any writer of it in the switch that takes this change. Flume's module moves a file of the host's own at that path aside to `crew.toml.before-nix` once. The proposal and design say this, and this change asks nothing else of another repository.
