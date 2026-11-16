# Verify: the-set-takes-timbers-own-module

The work holds. The gate passes, all seven tasks are ticked, and a case proves each scenario of the delta.

The code stage archived the change under task 3.2, because `root:specs` can prove the delta only after the archive. This report sits beside the archive, on `change/the-set-takes-timbers-own-module` at `55a52b1`.

## The whole checks

`sluice exec main` ran once on `55a52b1`. It ran the checks that commit affects against `main` (`23e4181`) and exited 0. All 14 passed, and sluice recorded their results under their keys for `55a52b1`:

- `nix`: `format`, `lint`, `typecheck` and `test` ("every case holds");
- `root`: `format`, `adrs`, `changes`, `specs` and `releases`;
- `site`: `build`, `prose`, `test` and `check`;
- `boomdrive`: `test`.

`root:specs` passed 27 items, with none failed. `root:changes` found no open change to validate. moon warned that it could not write its output archives under `.moon/cache/outputs`, and that it skipped the Rust dependency install for want of a network. Both warnings come from the sandbox, and no check failed because of them.

## Tasks

All seven tasks are ticked, 1.1 to 3.2, and none is open.

- 1.1 to 1.4 and 2.1 rely on `nix:test`.
- 3.1 relies on `root:changes`. It passed before the archive, and finds no open change after it.
- 3.2 relies on `root:specs`.

Each of those checks passed in the run above.

The diff from the branch's base, `9bba58b` (`bolt/a-hosts-set-takes-its-tools-new-releases`), stays inside proposal.md's Touches, beside the change's own files and the reconciled spec:

- `flake.nix`, `nix/modules/{apps,nixos,options,tools}.nix` and `nix/tests/{apps,cache,lib,tools}.nix`;
- `README.md`;
- record 0023 and `docs/architecture/core.md`.

It adds no crate, gate tool or tracked file type. The runner's code and `flake.lock` are unchanged. The tasks say record 0023's amendment of 2026-11-15 and core.18's and core.46's details were written in the planning commit, `45a7952`, and they were.

Outside the archives and record 0023, `services.boomdrive.timber.package` is named only where its failure is: the case that proves it, the requirement that says so, and core.46's details.

## The delta, scenario by scenario

The cases are keys of `nix/tests/tools.nix` or `nix/tests/apps.nix`, each on nix-darwin and on NixOS unless it names a platform.

### Added

| Scenario | Proven by |
|---|---|
| The timber option is gone | `apps.nix`'s "The timber option is gone": setting `services.boomdrive.timber.package = null` fails, and the failure, `lib.mkRemovedOptionModule`'s assertion read by `t.failures`, names `services.timber.package` and `services.timber.enable` |

### Modified

| Scenario | Proven by |
|---|---|
| Each tool's options are its own flake's | `tools.nix`'s "Each tool's options are its own flake's", its `tools` holding `timber`: every option of `services.timber` is declared in timber's input |
| The tools' modules re-exported | `tools.nix`'s "The tools' modules re-exported": `boomdrive`, `crew`, `default`, `flume`, `sluice` and `timber`, and sluice's module alone declares `services.sluice` and no `services.boomdrive` |
| Imported twice | `tools.nix`'s "Imported twice", unchanged |
| timber's own module beside the set's | `tools.nix`'s "timber's own module beside the set's": the platform's `default` and timber's flake's own `<platform>Modules.timber` evaluate, failing no assertion |
| A host with the runner alone | `tools.nix`'s "A host with the runner alone": Flume's, sluice's and timber's `enable` true, crew's false, no failure |
| A host that keeps its own Flume file | `tools.nix`'s "A host that keeps its own Flume file", unchanged |
| The tools' user on NixOS | `tools.nix`'s "The tools' user on NixOS": all four `alice`, then sluice's `bob` beside Flume's, crew's and timber's `alice` |
| The set's packages | `tools.nix`'s "The set's packages": the profile holds the set's `boomdrive`, `crew`, `flume`, `sluice` and `timber` |
| A host's own crew, No sluice of the set's | the cases of those names, unchanged |
| The tools' activation is their own | `tools.nix`'s "… on nix-darwin": the activation less the runner's own steps equals that of the tools' own modules, timber's now among them in `lib.nix`'s `toolsOf`. "… on NixOS": the user activation's snippets less the runner's are the tools' own, and `crew`, `flume`, `sluice` and `timber` each equal the tool's own and are not empty |
| The tools option is gone | the case of that name, unchanged |
| The README's words | `nix/tests/cache.nix`'s "The README's words", which no longer looks for `services.boomdrive.timber.package` and looks for nothing new (principle 16) |
| timber with the runner | `apps.nix`'s "timber with the runner": no failure, timber's `enable` true and the set's timber in the profile |
| A host's own timber | `apps.nix`'s "A host's own timber": the host's package in the profile, not the set's, and timber's link the one the set's timber gets (`linksAsTheSets`) |
| No timber of the set's | `apps.nix`'s "No timber of the set's": `services.timber.package = null` evaluates, no set's timber in the profile, and timber's link as with the set's |
| A host that keeps its own timber file | `apps.nix`'s "A host that keeps its own timber file": `services.timber.enable = false` evaluates, links no file of timber's and holds no set's timber |
| No timber without the runner | `apps.nix`'s "No timber without the runner": timber's `enable` false and no set's timber in the profile |

timber's link, as `apps.nix`'s `timberLinkOf` reads it, is the activation's lines naming `timber-config.toml` on nix-darwin and the user activation's `timber` snippet on NixOS, as Task notes 1.3 ask.

The renamed requirement is in `openspec/specs/service-modules/spec.md` under its new name, *The Boomdrive module turns on Flume's, sluice's and timber's modules and leaves crew's to the host*, and `root:specs` passes it. *timber's serving process is timber's own* is unchanged, and "No serving process of the modules'" holds.

## The modules

- `flake.nix`'s `tools` is `flume`, `sluice`, `crew` and `timber`, so `modulesOf` imports timber's module after crew's and gives it again. Its comments name timber's module and the amendment of 2026-11-15.
- `nix/modules/tools.nix` sets `services.timber.package` to the set's `timber` by `lib.mkDefault`, whether or not the runner is on. With the runner it sets `services.timber.enable` by `lib.mkDefault true`, beside Flume's and sluice's.
- `nix/modules/nixos.nix` sets `services.timber.user` to the runner's user by `lib.mkDefault`.
- `nix/modules/apps.nix` no longer declares `timber.package` or puts it in the profile. Its header says timber comes by its own module, and timber's herdr plugin stays in the plugins' default.
- `nix/modules/options.nix` removes `services.boomdrive.timber.package` by `lib.mkRemovedOptionModule`, beside `services.boomdrive.tools`. Its words name `services.timber.package`, `null`, `services.timber.enable = false` and timber's file.

## The words

The README's **Installing with nix** says what design Decision 9 lists:

- timber's module among the tools' own, re-exported, taken once when a host also imports it from timber's flake;
- its file, `~/.config/timber/config.toml`, written from `services.timber.settings`, the header alone where none is set, and moved aside once;
- `services.boomdrive.enable` turning on Flume's, sluice's and timber's modules;
- `services.timber.package = null;` beside Flume's and sluice's, and `services.timber.enable = false` to keep timber's file the host's own;
- timber's module turned on with the runner and its herdr plugin linked with Flume's and sluice's;
- five apps of the runner's module's own, listed by name;
- `services.timber.package` in the serving process's paragraph.

It no longer names `services.boomdrive.timber.package`.

## Archive

The change is at `openspec/changes/archive/2026-11-15-the-set-takes-timbers-own-module`, and no change of that name is open. `openspec/specs/service-modules/spec.md` holds the delta, and `root:specs` passes it under the strict check.
