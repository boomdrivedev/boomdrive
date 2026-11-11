# Verify

The verify stage of the change the-flake-configures-each-tool-by-its-own-module, on the branch `change/the-flake-configures-each-tool-by-its-own-module` at `f33981b`, 2026-11-08. The branch holds its base, `bolt/the-boomdrive-flake-configures-the-whole-host` at `c61379f`, merged in (`70dd77e`, task 1.8).

**The work holds.** The gate passes as a whole. Every requirement and scenario of the three spec deltas has its case, which the gate ran and which holds, beside the base's accounts and drives. All 18 tasks are ticked. The code stage archived the change over its base as task 3.4 asks, so this report sits in the archived directory, and the change stays archived with it.

## The whole checks

`devenv shell -- sluice exec main`, run once on `f33981b`. sluice ran the check-fast tasks the commit affects against `main` (`904c8fa`), 21 in all, and exited 0: "passed: 21 tasks", "recorded 21 results under their keys for f33981b".

| project | tasks | outcome |
|---|---|---|
| `nix` | `format`, `lint`, `typecheck`, `test` | passed; `typecheck` and `test` each "every case holds" |
| `boomdrive` | `format`, `lint`, `typecheck`, `test` | passed. The family tests (`each_tools_version_is_its_tags`, `flumes_status_of_a_scratch_host`, `not_the_hosts_tools`, `a_submission_sluice_has_no_record_of`) ran with the set's Flume v0.18.0 and sluice v0.76.0 |
| `boomdrive-crew` | `format`, `lint`, `typecheck`, `test` | passed |
| `boomdrive-hosted` | `format`, `lint`, `typecheck`, `test` | passed |
| `root` | `format`, `specs`, `changes`, `adrs`, `releases` | passed: `specs` 16 of 16, `changes` no open change, treefmt 214 files, 0 changed |

sluice also said `would conflict: … with main`, naming `README.md`, `flake.nix`, `flake.lock`, `nix/packages/flume-git-crates.nix`, `docs/architecture/core.md`, `openspec/specs/views/spec.md`, record 0028 and runner files of the base's accounts and drives. This concerns where the bolt lands on `main`, not these checks: the branch lands on its base, which already holds those files as merged here. moon's warnings that it could not write its shared cache or archive outputs, and the OTLP exporter's failure to shut down, belong to the sandbox the gate runs in. They change no task's outcome.

## The specs

`nix/tests/default.nix` evaluates `outputs.nix`, `packages.nix`, `file.nix`, `darwin.nix`, `nixos.nix`, `tools.nix`, `cache.nix` and `workflow.nix` under `nix:test`, and `typecheck.nix` under `nix:typecheck`. `nix/check.sh` runs `activate.sh` too. Each scenario below is a case of these, named by the scenario. A scenario that names no platform has a case on nix-darwin and a case on NixOS.

### service-modules

- **The modules bring each tool's own module.** "Each tool's options are its own flake's" (`tools.nix`). `flake.nix`'s `modulesOf` imports `inputs.<tool>.<platform>Modules.<tool>`, and no file of `nix/modules` declares an option of `services.flume`, `services.sluice` or `services.crew`.
- **The flake re-exports each tool's module.** "The tools' modules re-exported" (`tools.nix`).
- **The Boomdrive module is taken once however often it is imported.** "Imported twice" (`tools.nix`).
- **The Boomdrive module turns on Flume's and sluice's modules and leaves crew's to the host.** "A host with the runner alone", "A host that keeps its own Flume file" and "The tools' user on NixOS" (`tools.nix`).
- **Each tool's package is the set's unless the host gives its own.** "The set's packages", "A host's own crew" and "No sluice of the set's" (`tools.nix`).
- **The Boomdrive module writes no file of the other tools'.** "The tools' activation is their own" (`tools.nix`), on nix-darwin and on NixOS.
- **The modules hold no option that installs another tool.** "The tools option is gone" (`darwin.nix`, `nixos.nix`), and "The README's words" (`cache.nix`). `services.boomdrive.tools` is no option, and its words in `nix/modules/options.nix` name each tool's `package` and `enable`.
- **The runner finds its tools on the profiles' PATH** (modified). "The set's tools", "A host's own tools" and "The user's profile first" (`darwin.nix`, `nixos.nix`).
- **A changed file or package restarts the runner, and nothing else does** (modified). "A new file on nix-darwin", "A new package on nix-darwin" and "Nothing else changed on nix-darwin" in `darwin.nix`; "Not running on NixOS", "Nothing changed on NixOS", "A new file on NixOS" and "Another user's activation" in `activate.sh`.

The cases that evaluate a whole system give the runner's settings under the base's `accounts`, and crew's module its own `settings.crewModule` (`nix/tests/settings.nix`), so the base's "The options are the runner's file, its accounts always written" and "A flat list of before fails evaluation" hold under the modules that bring each tool's own: their cases, "Only accounts", "The flat list in settings" and "A stream in a drive's settings" (`file.nix`), pass in the same run.

### family-packages

- **Each package is made from the runner's tree or its tool's own flake at its release tag** (added). "Flume through its own flake", "sluice through its own flake", "crew through its own flake" and "The runner from this tree" (`packages.nix`). `nix/packages/flume.nix` and `nix/packages/flume-git-crates.nix` are gone, and `nix/packages/crew.nix` gives crew its tag's version.
- **Each package is made from its release tag's source** (removed). It is absent from `openspec/specs/family-packages/spec.md`, and no case names it.
- **The flake gives the family's host tools as packages** (modified). "Every system's packages", whose set holds `crew`, and "No tool of the host's own" (`typecheck.nix`).
- **The flake and its lock name one set** (modified). "A tag moved without the lock" and "A tool named by no release tag" (`packages.nix`). `flake.nix` names sluice `v0.76.0`, Flume `v0.18.0` and crew `v0.44.8`, each with its follows in one `inputs` set, newer than the base's v0.74.3 and v0.17.1.

### binary-cache

- **The workflow builds and pushes every package of every system** (modified). "Each system has its job", "The runner is pushed with its tools", "A package that does not build" and "A branch's push" (`workflow.nix`).
- **A host substitutes the set, or builds the same paths** (modified). "A host's own nixpkgs" and "The host's trust" (`cache.nix`).

Each requirement the deltas add or modify stands in `openspec/specs` word for word as the delta gives it, beside the base's requirements, and the removed one is gone. `openspec list` finds no active change.

## The tasks

All 18 tasks, 1.1 to 3.4, are ticked `[x]`, and none is open. Each check a task names ran in this gate and passes:

- 1.7: `statix check flake.nix` passes, as `nix:lint`'s first command.
- 1.8: `nix:test` and `root:adrs` pass with the base merged. `flake.nix` and `flake.lock` keep this change's tags, `nix/packages/flume-git-crates.nix` stays gone, `nix/tests/settings.nix` names crew's own settings beside the base's accounts and drives, and core.46's details keep "each tool installed by its own module, its package the set's unless the host gives its own (record 0023)" beside "the managed layer written by nix while a control plane writes the local layer (core.17, record 0009 as amended 2026-11-06)".
- 2.6: `nix:lint` and `nix:test` pass.
- 3.4: `root:specs` passes on the specs archived over the base.

The words agree with the specs:

- core.46's sentence reads "the flake's modules run the runner as the user's service".
- CLAUDE.md and `openspec/config.yaml` name crew in the set, and each tool's own module beside `services.boomdrive`.
- The README names `services.flume.package`, `services.sluice.package` and `services.crew.package`, names no `services.boomdrive.tools` or `flume-git-crates.nix`, says Flume's git crates' hashes are Flume's own flake's, and names each repository under its drive, `accounts.<account>.drives.<drive>.repositories`.

## Outcome

Nothing goes back to the code stage. The change is archived at `openspec/changes/archive/2026-11-08-the-flake-configures-each-tool-by-its-own-module`, with this report.
