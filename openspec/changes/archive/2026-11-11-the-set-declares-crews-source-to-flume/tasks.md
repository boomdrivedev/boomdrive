# Tasks

Work through the tasks in order. A task is true once the check it names passes, run as it names it. The groups are headings for reading.

- Every case evaluates against no store, as `nix/check.sh` does, with core.41's neutral names. No task builds a package, writes to the store, or reaches the network.
- No task moves a flake input or its tag, or adds a crate, a tool or a tracked file type (core.7, core.8). The runner's code and its file do not change (core.5). Nothing outside the paths proposal.md's Touches lists is changed.
- core.57's amendment is written in the commit that writes this change. No task changes it.

## 1. crew's source in Flume's module

- [x] 1.1 `nix/modules/sources.nix`, imported by `darwin.nix` and `nixos.nix` after `./drives.nix`, gives `services.flume.sources.crew` its `kinds_dir`, `~/.local/share/flume/kinds/crew`, and its `roles`, `~/.local/state/crew/flume/roles.toml`, each by `lib.mkDefault`, under `services.crew.enable` alone. The header comments of both platform modules name it. `nix/tests/sources.nix` is added to `default.nix`'s `test`, and `lib.nix`'s `toolsOf` declares the same source in Flume's own module (design Decisions 1 to 4 and 6; Task notes 1.1). True when `sluice exec main -- moon run nix:test` passes, proving "crew's source with crew on", "No source of crew's with crew off" and "crew's source without the runner", each on nix-darwin and on NixOS, with "The tools' activation is their own" still holding on both.
- [x] 1.2 `nix/tests/sources.nix` holds the source's `roles` and `kinds_dir` to the `ROLES_FILE` and `KINDS_DIR` of crew's `plugin/lib/source.py` at the lock's crew, each under `~/` (design Decision 5; Task notes 1.2). True when `sluice exec main -- moon run nix:test` passes, proving "The paths the set's crew writes".
- [x] 1.3 A key a host sets in `services.flume.sources.crew` wins over the module's, and the other key keeps the module's (design Decision 4; Task notes 1.3). True when `sluice exec main -- moon run nix:test` passes, proving "A host's own kinds folder for crew" on nix-darwin and on NixOS.
- [x] 1.4 The nix files this change adds or changes keep to the nix project's format and lint (Task notes 1.4). True when `sluice exec main -- moon run nix:format` and `sluice exec main -- moon run nix:lint` pass.

## 2. The words

- [x] 2.1 The README's **Installing with nix** says that, with crew's module on, the module declares crew's source to Flume, `services.flume.sources.crew`, naming `~/.local/state/crew/flume/roles.toml` and `~/.local/share/flume/kinds/crew`, that Flume's module writes it and a host writes no file for it, and that a key the host sets there wins. No nix block of it sets `services.flume.sources` (Task notes 2.1). True when `sluice exec main -- moon run nix:test` passes, proving "The README's words on crew's source", with "The README's words" and "The README's lines" still holding.
- [x] 2.2 CLAUDE.md's paragraph on the flake says the module gives Flume's module crew's source of kinds where crew is on (Task notes 2.2). True when `sluice exec main -- moon run root:format` passes.
- [x] 2.3 The change's delta archives into specs that pass openspec's strict check. True when, after `devenv shell -- openspec archive the-set-declares-crews-source-to-flume --yes`, `sluice exec main -- moon run root:specs` passes.
