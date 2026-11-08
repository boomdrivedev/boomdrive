# Verify

The verify stage of the change the-flake-configures-each-tool-by-its-own-module, on the branch `change/the-flake-configures-each-tool-by-its-own-module` at `148e472`, 2026-11-08.

**The work holds.** The gate passes as a whole. Every requirement and scenario of the three spec deltas has its case, which the gate ran and which holds. All 17 tasks are ticked. The code stage archived the change as task 3.4 asks, so this report sits in the archived directory, and the change stays archived with it.

## The whole checks

`devenv shell -- sluice exec main`, run once on `148e472`. sluice ran the check-fast tasks the commit affects against `main` (`14a9139`), 9 in all, and exited 0: "passed: 9 tasks", "recorded 9 results under their keys for 148e472".

| task | outcome |
|---|---|
| `nix:format` | passed |
| `nix:lint` | passed: `statix check flake.nix`, `statix check nix`, `deadnix --fail flake.nix nix` and `actionlint .github/workflows/cache.yml` |
| `nix:typecheck` | passed: "every case holds" |
| `nix:test` | passed: "every case holds" |
| `boomdrive:test` | passed. The family tests (`each_tools_version_is_its_tags`, `flumes_status_of_a_scratch_host`, `not_the_hosts_tools`, `a_submission_sluice_has_no_record_of`) ran with the set's Flume v0.18.0 and sluice v0.76.0 |
| `root:format` | passed: 210 files, 0 changed |
| `root:specs` | passed: 16 of 16 |
| `root:changes` | passed, no open change |
| `root:releases` | passed |

sluice also said `would conflict: README.md, flake.lock, flake.nix, nix/packages/flume-git-crates.nix with main`. This concerns where the branch lands, not these checks. The branch was cut from `bolt/the-boomdrive-flake-configures-the-whole-host`. The design's Dependencies already expect `main`'s chore the-sets-flume-and-sluice-are-their-latest-releases to touch `flake.nix` and `flake.lock`, and this change's later tags are kept. moon's warnings that it could not write its shared cache or archive outputs, and the OTLP exporter's failure to shut down, belong to the sandbox the gate runs in. They change no task's outcome.

## The specs

`nix/tests/default.nix` evaluates `outputs.nix`, `packages.nix`, `file.nix`, `darwin.nix`, `nixos.nix`, `tools.nix`, `cache.nix` and `workflow.nix` under `nix:test`, and `typecheck.nix` under `nix:typecheck`. `nix/check.sh` runs `activate.sh` too. Each scenario below is a case of these, named by the scenario. A scenario that names no platform has a case on nix-darwin and a case on NixOS.

### service-modules

- **The modules bring each tool's own module.** "Each tool's options are its own flake's" (`tools.nix`). Every option of `services.flume`, `services.sluice` and `services.crew` is declared under that tool's input, on both platforms. `flake.nix`'s `modulesOf` imports `inputs.<tool>.<platform>Modules.<tool>`. No file of `nix/modules` declares a tool's option.
- **The flake re-exports each tool's module.** "The tools' modules re-exported" (`tools.nix`).
- **The Boomdrive module is taken once however often it is imported.** "Imported twice" (`tools.nix`). `boomdrive` carries its `key`, and `default` is the same value.
- **The Boomdrive module turns on Flume's and sluice's modules and leaves crew's to the host.** Covered by "A host with the runner alone", "A host that keeps its own Flume file" and "The tools' user on NixOS" (`tools.nix`). `nix/modules/tools.nix` sets the `enable` of Flume's and sluice's modules with `mkDefault true` under `services.boomdrive.enable`. `nixos.nix` sets the three `user`s with `mkDefault`.
- **Each tool's package is the set's unless the host gives its own.** Covered by "The set's packages", "A host's own crew" and "No sluice of the set's" (`tools.nix`). `tools.nix` sets each `package` with `mkDefault`, outside the `mkIf`.
- **The Boomdrive module writes no file of the other tools'.** "The tools' activation is their own on nix-darwin" and "… on NixOS" (`tools.nix`).
- **The modules hold no option that installs another tool.** "The tools option is gone on nix-darwin" and "… on NixOS" (`darwin.nix`, `nixos.nix`), and "The README's words" (`cache.nix`).
- **The runner finds its tools on the profiles' PATH** (modified). Covered by "The set's tools", "A host's own tools" and "The user's profile first", each on nix-darwin (`darwin.nix`) and on NixOS (`nixos.nix`).
- **A changed file or package restarts the runner, and nothing else does** (modified). "A new file on nix-darwin", "A new package on nix-darwin" and "Nothing else changed on nix-darwin" are in `darwin.nix`. "Not running on NixOS", "Nothing changed on NixOS", "A new file on NixOS" and "Another user's activation" are in `activate.sh`.

### family-packages

- **Each package is made from the runner's tree or its tool's own flake at its release tag** (added). Covered by "Flume through its own flake", "sluice through its own flake", "crew through its own flake" and "The runner from this tree" (`packages.nix`). `nix/packages/flume.nix` and `nix/packages/flume-git-crates.nix` are gone. `nix/packages/git-crates.nix` holds the runner's crates alone.
- **Each package is made from its release tag's source** (removed). It is absent from `openspec/specs/family-packages/spec.md`, and no case names it.
- **The flake gives the family's host tools as packages** (modified). Covered by "Every system's packages", where the set holds `crew`, and "No tool of the host's own" (`typecheck.nix`).
- **The flake and its lock name one set** (modified). Covered by "A tag moved without the lock" and "A tool named by no release tag" (`packages.nix`). `flake.nix` names sluice `v0.76.0`, Flume `v0.18.0` and crew `v0.44.8`, each with its `follows` in one `inputs` set.

### binary-cache

- **The workflow builds and pushes every package of every system** (modified). Covered by "Each system has its job", "The runner is pushed with its tools", "A package that does not build" and "A branch's push" (`workflow.nix`).
- **A host substitutes the set, or builds the same paths** (modified). Covered by "A host's own nixpkgs" and "The host's trust" (`cache.nix`).

Every requirement the deltas add or modify is in `openspec/specs`, and the removed one is gone. `openspec list` finds no active change.

## The tasks

All 17 tasks, 1.1 to 3.4, are ticked `[x]`, and none is open. The gate ran each check a task names, and each passes in this run:

- 1.7 is true: `statix check flake.nix` passes, as `nix:lint`'s first command.
- 2.6 is true: `nix/modules/nixos.nix`, `nix/modules/tools.nix` and `nix/tests/lib.nix` each give the tools' settings under one `services` set, and `nix:lint` and `nix:test` pass.
- 3.4 is true: `root:specs` passes on the archived specs.

The words agree with the specs:

- core.46's sentence reads "the flake's modules run the runner", and its details say "each tool installed by its own module, its package the set's unless the host gives its own (record 0023)".
- CLAUDE.md and `openspec/config.yaml` name crew in the set, and each tool's own module beside `services.boomdrive`.
- The README names `services.crew` and each tool's `package`. It no longer names `services.boomdrive.tools` or `flume-git-crates.nix`, and it says Flume's git crates' hashes are Flume's own flake's.

## Outcome

Nothing goes back to the code stage. The change is archived at `openspec/changes/archive/2026-11-08-the-flake-configures-each-tool-by-its-own-module`, with this report.
