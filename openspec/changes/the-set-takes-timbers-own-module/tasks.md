# Tasks

Work through the tasks in order. A task is true once the check it names passes, run as it names it. The groups are headings for reading.

- **The checks.** Every evaluated case runs against no store, as `nix/check.sh` does, with core.41's neutral names. No check builds a package or reaches the network. A case named in a task is a key of `nix/tests/tools.nix` or `nix/tests/apps.nix`, which `sluice exec main -- moon run nix:test` prints as `nix: <case>: <what>` where it fails. Each task leaves every case of the project's test holding, so the order below matters: timber's module is turned on in the same task that stops the Boomdrive module installing timber.
- **What no task adds or changes.**
  - No task adds a crate, a tool of the gate or a tracked file type (core.7, core.8).
  - The runner's code does not change.
  - The lock does not change: timber's `v0.7.0` gives the module.
  - No case reads more of the README than it reads today (blueprints principle 16).
  - Every `.nix` file written is formatted by nixfmt, as treefmt formats it.
  - Nothing outside the paths `proposal.md`'s Touches lists is changed.
- **What is already written.** This change's own commit holds record 0023's amendment of 2026-11-15 and core.18's and core.46's details. No task changes them. A task that relies on a choice the design names cites its Decision.

## 1. timber's own module

- [x] 1.1 `flake.nix`'s `tools` gains `timber`, after `crew`, so the platform's `boomdrive` module imports timber's `<platform>Modules.timber` and the flake gives it again as `darwinModules.timber` and `nixosModules.timber`. The comments of `flake.nix` that name the tools' modules name timber's (design Decision 1). `nix/tests/tools.nix`'s `tools` gains `timber`, *The tools' modules re-exported* expects `boomdrive`, `crew`, `default`, `flume`, `sluice` and `timber`, and the case *timber's own module beside the set's* is added, on each platform (Task notes 1.1). True when the cases "Each tool's options are its own flake's", "The tools' modules re-exported", "Imported twice" and "timber's own module beside the set's", each on nix-darwin and on NixOS, hold, with `sluice exec main -- moon run nix:test` passing.
- [x] 1.2 `nix/modules/nixos.nix` sets `services.timber.user` to the runner's user by default, beside Flume's, sluice's and crew's, and its header names timber (design Decision 4). `nix/tests/tools.nix`'s `usersOf` reads timber's `user`, and *The tools' user on NixOS* expects it `alice` in both systems. True when the case "The tools' user on NixOS" holds, with `sluice exec main -- moon run nix:test` passing.
- [x] 1.3 `nix/modules/tools.nix` sets `services.timber.package` to the set's `timber` by default, whether or not the runner is enabled, beside Flume's, sluice's and crew's, and, where `services.boomdrive.enable` is on, `services.timber.enable` true by default, beside Flume's and sluice's. Its header names timber's module (design Decisions 2 and 3). `nix/modules/apps.nix` no longer declares `timber.package` or puts it in the system profile, and its header says timber comes by its own module, which the Boomdrive module turns on, and keeps its plugin's words (design Decisions 5 to 7). The tests change with it (design Decision 8; Task notes 1.3):
  - `nix/tests/lib.nix`'s `toolsOf` gains timber's own module after crew's, turned on with the set's `timber`, and `t.toolsNixosSystem` gives it the user `alice`;
  - `nix/tests/tools.nix`'s `fileNames` gains `timber-config.toml`, *A host with the runner alone* expects timber's `enable` true, *The set's packages* expects the set's `timber` in the profile, and *The tools' activation is their own on NixOS* holds the `timber` snippet to the tools' own;
  - `nix/tests/apps.nix`'s timber cases set `services.timber.package` in place of the removed option, *timber with the runner* expects timber's `enable` true, *A host's own timber* and *No timber of the set's* hold timber's link to the set's, *No timber without the runner* expects timber's `enable` false, and *A host that keeps its own timber file* is added, on each platform.

  True when the cases "A host with the runner alone", "The set's packages", "timber with the runner", "A host's own timber", "No timber of the set's", "A host that keeps its own timber file", "No timber without the runner" and "No serving process of the modules'", each on nix-darwin and on NixOS, and "The tools' activation is their own on nix-darwin" and "The tools' activation is their own on NixOS" hold, with `sluice exec main -- moon run nix:test` passing.
- [x] 1.4 `nix/modules/options.nix` removes `services.boomdrive.timber.package` by `lib.mkRemovedOptionModule`, beside `services.boomdrive.tools`. Its words say timber's own module installs timber, and to set `services.timber.package` to the host's own package or `null`, or `services.timber.enable` to `false` to keep timber's file the host's own. The file's header names it (design Decision 6; Task notes 1.4). `nix/tests/apps.nix` gains the case *The timber option is gone*, on each platform. True when the cases "The timber option is gone on nix-darwin" and "The timber option is gone on NixOS" hold, with `sluice exec main -- moon run nix:test` passing.

## 2. The words

- [x] 2.1 The README's **Installing with nix** says what design Decision 9 lists: timber's module among the tools' own modules, re-exported, with its file at `~/.config/timber/config.toml`; timber's module turned on with the runner and its herdr plugin linked with Flume's and sluice's; five apps of the runner's module's own, not six; `services.timber.package = null;` beside Flume's and sluice's, and `services.timber.enable = false` to keep timber's file the host's own; and `services.timber.package` in the serving process's paragraph. The README no longer names `services.boomdrive.timber.package`. No check reads these words. `nix/tests/cache.nix`'s *The README's words* drops `services.boomdrive.timber.package` from its list and gains nothing (principle 16). True when the case "The README's words" holds, with `sluice exec main -- moon run nix:test` passing.

## 3. The specs

- [ ] 3.1 The change's delta passes openspec's strict check. True when `sluice exec main -- moon run root:changes` passes.
- [ ] 3.2 The delta archives into specs that pass openspec's strict check. That covers the requirement renamed *The Boomdrive module turns on Flume's, sluice's and timber's modules and leaves crew's to the host*. True when, after `devenv shell -- openspec archive the-set-takes-timbers-own-module --yes`, `sluice exec main -- moon run root:specs` passes.
