# Verify

The verify stage of `every-claude-code-session-has-boomdrives-plugins`, on `change/every-claude-code-session-has-boomdrives-plugins` at `837e333a`, cut from `bolt/every-machine-takes-boomdrives-tools`, which it holds at `382923de`.

**Outcome: the whole checks pass. The work meets every requirement and scenario of the change's specs, and every task is ticked. The change is archived.**

## The whole checks

`sluice exec main` ran once on `837e333` and exited 0. It ran the checks `837e333` affects, against `main` at `f954b77`, and printed `passed: 13 tasks`. moon completed 14 tasks, counting `site:install`.

| project | tasks |
|---|---|
| `boomdrive` | test: 738 passed, 0 failed |
| `nix` | format, lint, typecheck, mac-sh, test (`nix: test: every case holds`) |
| `root` | format (497 files, 0 changed), specs (27 passed, 0 failed), changes (1 open), releases |
| `site` | install, check (0 errors), build, test |

No crate changes on this branch since the base, so the other crates' tasks were not among the affected checks.

`root:changes` archived this open change, alone, into a scratch copy of the specs. Every spec it wrote passed `openspec validate --strict` (core.61).

sluice said three things that are not checks:

- **Not recorded.** sluice did not record the run under its keys. This stage's draft of this report was in the worktree, beyond `837e333`, while the run went on. The run itself passed.
- **Would conflict with main.** sluice named `CLAUDE.md`, `README.md`, `docs/architecture/core.md`, `nix/modules/apps.nix`, `nix/tests/apps.nix` and `openspec/config.yaml` as conflicting with `main`. `main` moved after the bolt was cut, with `the-set-carries-rex-matched-to-flume` (#1) and `sluice-policy-names-its-signers` (#284). This branch lands on `bolt/every-machine-takes-boomdrives-tools`, which it holds whole, with no conflict. The bolt takes `main` before it lands.
- **moon's cache.** moon warned at each task that it could not package the task's outputs into its cache. That is moon's cache, not a check, and it failed no task.

## The tasks

Every task of tasks.md, 1.1 to 6.2, is ticked. Each one's named check is in the tree and ran in the gate's run:

| task | what holds it |
|---|---|
| 1.1 | `flake.nix`'s `flume` input names `github:boomdrivedev/flume/v0.31.0`, and the lock moves the `flume` node alone, to `22b5982`; `nix:typecheck` and `nix:test` |
| 2.1 | `boomdrive-claude-plugins` as `nix/packages/claude-plugins.nix` builds it, `flume` at `0.31.0`; `nix:typecheck` |
| 2.2 | the cases "The marketplace's plugins", "No link into the store", "Each tool's plugins from its tool's pin", "Each tool's plugins say its tool's version" and "A plugin other than its tool", in `nix:test` |
| 3.1 | `nix/modules/activate.sh`'s verb `claude`, named in its header and usage; `nix/tests/activate.sh`'s sixteen cases of the homes, the commands, the server and the step's lines, with stubs of `flume` and `claude`, and *claude's usage* |
| 3.2 | `pendingyou-folder.nix`'s `passthru.server`, which `signin/.claude.json` names; `nix/modules/sessions.nix`'s `runs`, `pluginsOf`, `serverOf`, `argumentsOf` and `module` |
| 3.3 | `darwin.nix`'s `postActivation` step by `lib.mkAfter`, and `nixos.nix`'s step before `keep` with `deps = [ "flume" ]`, each where `sessions.runs`; `nix/tests/sessions.nix`'s cases, read by `nix/tests/default.nix` |
| 3.4 | `apps.nix`'s *No Claude Code of the modules'*, which allows the one step, run once; `activate.sh`'s *The step's commands*; `tools.nix`'s and `collector.nix`'s nix-darwin cases, which take the step out; `nixos.nix`'s *The user activation on NixOS*, which holds it between the plugins' links and `keep` |
| 4.1 | `nix/modules/plugin-dirs.sh`, read into `environment.extraInit` by `sessions.nix`'s `module`; `activate.sh`'s *A fresh environment*, *A person's own value*, *Run twice* and *No folder*; `sessions.nix`'s *The lines with the folder* and *No lines without the folder* |
| 5.1 | the README's **Installing with nix**; `root:format` |
| 5.2 | CLAUDE.md and `openspec/config.yaml`; `root:format` |
| 6.1 | `root:changes` |
| 6.2 | `root:changes`, which archives this open change alone into a scratch copy of the specs and holds every spec it writes to `openspec validate --strict` (core.61) |

## The specs

### service-modules

- **With the runner and Flume, each activation installs the set's plugins into every Claude Code home of Flume's accounts.** `sessions.runs` is `services.boomdrive.enable` and Flume's switch with a package. On nix-darwin the step is the last of `postActivation`, by `lib.mkAfter`. It runs as the primary user by `sudo --user`, with `HOME` that user's home and `PATH` the agent's own, then `$HOME/.local/bin`. On NixOS it is the runner's user's step after the folder and the plugins' links and before `keep`, inside the test of that user, and the snippet gains `deps = [ "flume" ]`. *The step on nix-darwin* holds Flume's link, Pending You's folder and herdr's links, each once, before it. *The step on NixOS* holds its order, its guard, its `deps` and that no other user's snippet runs it. *No step without the runner* and *No step without Flume's package* read every activation text and user snippet of the system.
- **The plugins installed are those of the tools the host installs.** `pluginsOf` splits the marketplace's own list by Decision 5's table, each tool's switch with a package, and removes every other plugin it lists. *Every plugin with the runner and crew*, *No companions' plugins without crew* (with `-` for the server) and *A tool turned off* hold each split on both platforms.
- **The homes.** The step reads `.status.host.pools[]?.accounts[]?`, keeps each whose `program` is `claude` and whose `home` is a non-empty string, drops repeats in status's order, and runs the default home, with `CLAUDE_CONFIG_DIR` unset, first. A status Flume refuses, or one `jq` cannot read, is one line and the default home alone. The five cases of the homes hold each scenario.
- **Only Claude Code's own commands, for the set's marketplace.** In each home: `plugin marketplace add <marketplace> --json`, `plugin list --json`, then `install`, `update --scope user` and `uninstall`, each `<plugin>@boomdrive` with `--json`, as the list and the marketplace's versions say. The list is read for ids ending `@boomdrive` alone, so *The home's own plugins* names neither `plannotator@plannotator` nor `foo@other`. Claude Code 2.1.322's `claude plugin list --json`, read on this host, is the array of objects with `id` and `version` the step reads. *A fresh home*, *Nothing differs*, *A new build of a plugin* and *A plugin to remove* hold the rest.
- **Pending You's server.** `serverOf` writes `passthru.server`, the same value `signin/.claude.json` names, to a store file where the folder is written, and `-` elsewhere. The step adds it by `claude mcp add-json --scope user pendingyou`, after the plugins' commands, only where `${CLAUDE_CONFIG_DIR:-$HOME}/.claude.json` names no `mcpServers.pendingyou`, and runs no command that removes or rewrites one. *A home with no server*, *A home that names one*, *No server given* and *The server's file* (on both platforms, with alice's helper path) hold it.
- **The step stops nothing.** `claude` is looked for before Flume is run, so *No Claude Code* runs neither. Every `ask` is followed by `|| true`, and a refusal is one line naming the home, the command and the answer's `message`, else Claude Code's standard error, else its output. *A command refused* and *Every command held* hold it, and each case checks exit 0.
- **The login environment's wake mod.** `plugin-dirs.sh` puts `$HOME/.config/pendingyou/mod` ahead of any value, once, and exports it, only where the folder is there. `sessions.nix`'s `module` adds it to `environment.extraInit` where Pending You's switch is on with a package, which is where the folder is written. The four cases of the lines and the two of the module hold it.
- **The modules write no file of Claude Code's but its managed documents.** *No Claude Code of the modules'* now reads every step's text less the one line of `activate.sh claude`, holds that no text runs it twice and that the system runs exactly one such line, and still finds no other `claude`, `.claude`, `CLAUDE_CONFIG_DIR` or marketplace. *The step's commands* holds the verb to the six commands and to one place in the file that runs `claude`.
- **The Boomdrive module writes no file of the other tools'.** *The tools' activation is their own* and *The tools' telemetry is their own* take the step out on nix-darwin with the runner's other steps and hold it run once. On NixOS the runner's user's snippet is the runner's own, and *The user activation on NixOS* holds the step's line within it.
- **The README.** **Installing with nix** names `pendingyou-signin`, `boomdrive-claude-plugins`, `~/.config/pendingyou`, `CLAUDE_CODE_PLUGIN_DIRS` and the commands each switch runs. It says the folder's sign-ins, key and state stay the command line's, that each switch changes nothing else in a home, and that a home's own `plannotator@plannotator` stays until the person uninstalls it. It no longer says no Claude Code home is installed into.

### family-packages

- **The set's marketplace lists every Claude Code plugin a tool of the set ships.** The marketplace's eight plugins stand as the base built them. With Flume at v0.31.0, *The marketplace's plugins* finds `flume` at `0.31.0`, and *No link into the store* holds the build.

## The words and records

- CLAUDE.md and `openspec/config.yaml` say the modules install the set's marketplace and the tools' plugins into each home by Claude Code's own commands, with Pending You's server and the wake mod's variable. Neither still says no module runs `claude` or touches a Claude Code home.
- Record 0044's amendment of 2026-11-18 names Flume v0.31.0, the eight plugins with `crew` and `worktrunk`, and the wake mod by the login environment. core.18's and core.46's details name `activate.sh claude` and the lines of `environment.extraInit`.
- No crate, gate tool or tracked file type is added, and the runner's code is unchanged since the base.
