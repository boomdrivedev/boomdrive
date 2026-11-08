# Tasks

Build the tasks in order. A task is true once the check it names passes, run as it names it. The groups are headings, for reading.

- Every case evaluates against no store, as `nix/check.sh` does, with core.41's neutral names. No task builds a package, writes to the store, or reaches the network.
- No task moves a flake input or its tag, or adds a crate, a tool or a tracked file type (core.7, core.8). The runner's code does not change. Nothing outside the paths proposal.md's Touches lists is changed.
- Record 0028's amendment of 2026-11-08 and core.46's details are written in the commit that writes this change. No task changes them.

## 1. The runner's accounts by owner

- [x] 1.1 `services.boomdrive.accounts` names each account's drives, each drive's `machine`, `stream` and repositories by owner, and `settings` no longer declares `accounts`. `nix/modules/accounts.nix` gives `drivesOf` and `fileOf`, and `darwin.nix` and `nixos.nix` render `fileOf`. `toml.nix`'s header line names both options, and each file under `nix/tests/files` takes it. `settings.nix`'s `crew` and `hosted` name their accounts by owner beside their `settings` (design Decisions 1 and 7; Task notes 1.1). True when `sluice exec main -- moon run nix:test` passes, proving the service-modules spec's "Only accounts", "A host under crew", "A host under a hosted control plane" and "Owners turned into addresses".
- [x] 1.2 A host's own `settings.accounts`, the flat `settings.repositories`, a key a drive does not hold, and an address of no GitHub repository each fail evaluation, naming `services.boomdrive.accounts` or the address (design Decisions 1 and 2; Task notes 1.2). True when `sluice exec main -- moon run nix:test` passes, proving "A stream in a drive's settings", "The flat list in settings", "A key a drive does not hold" and "An address of no repository".
- [x] 1.3 A drive's name under two accounts, and an address kept by two drives or twice by one, each fail evaluation, naming what is given twice (design Decision 6; Task notes 1.3). True when `sluice exec main -- moon run nix:test` passes, proving "A drive's name under two accounts" and "A repository in two drives".
- [x] 1.4 The runner reads each rendered file, changed in its header line alone, with the accounts, drives, repositories and control plane its `accounts` and `settings` named (Task notes 1.4). True when `devenv shell -- cargo test -p boomdrive --test module_files` passes, proving "The runner reads each file".

## 2. Each drive's stream in Flume's module

- [x] 2.1 `nix/modules/drives.nix`, imported by `darwin.nix` and `nixos.nix`, gives Flume's `settings.streams.<stream>` each drive's `agents`, `priority` and each label's `reserve`, with the module's defaults, by `lib.mkDefault`, under `services.boomdrive.enable`, and no `streams` where no drive is named (design Decisions 3 and 4; Task notes 2.1). True when `sluice exec main -- moon run nix:test` passes, proving "A drive's stream by default", "A drive that writes only what it changes", "With the runner off" and "A drive's stream changes nothing of the runner's", each on nix-darwin and on NixOS where the scenario names no platform, with "The tools' activation is their own" and "A host with the runner alone" still holding.
- [x] 2.2 `drives.nix` gives Flume's `settings.repos."<owner>/<name>".stream` the drive's stream for each address each drive keeps, by its address and never a folder, and no `repos` where no drive is named (design Decision 4; Task notes 2.2). True when `sluice exec main -- moon run nix:test` passes, proving "Each address given its drive's stream" and "A stream named other than its drive".
- [ ] 2.3 A host's own value in `services.flume.settings` wins over the module's, and a stream's key the module does not give, set there, sits beside the module's (design Decision 4; Task notes 2.3). True when `sluice exec main -- moon run nix:test` passes, proving "A host's own room and rule".
- [ ] 2.4 A drive whose `machine` names a host other than `services.crew.host` gives Flume no stream and no repository's table, its repositories still in the runner's file, and a drive that names a `machine` where `services.crew.host` is not set fails evaluation (design Decision 5; Task notes 2.4). True when `sluice exec main -- moon run nix:test` passes, proving "A drive of another machine", "A drive of this machine" and "No host to compare with".
- [ ] 2.5 A stream given by two drives of this host fails evaluation, naming the stream and the drives (design Decision 6; Task notes 2.5). True when `sluice exec main -- moon run nix:test` passes, proving "One stream for two drives".
- [ ] 2.6 The nix files this change adds or changes keep to the nix project's lint (Task notes 2.6). True when `sluice exec main -- moon run nix:lint` passes.

## 3. The words

- [ ] 3.1 The README's **Installing with nix** names the accounts in `services.boomdrive.accounts`, each drive's repositories by owner and its stream's keys and defaults, a drive's `machine` against `services.crew.host`, and Flume's own `services.flume.settings.streams` for any other key of a stream, and no `settings.accounts` (Task notes 3.1). True when `sluice exec main -- moon run nix:test` passes, proving "The README's accounts", with "The README's words" still holding.
- [ ] 3.2 CLAUDE.md's paragraph on the flake says the module's `accounts` and `settings` are the runner's file, each drive's stream given to Flume's module (Task notes 3.2). True when `sluice exec main -- moon run root:format` passes.
- [ ] 3.3 The spec delta archives into specs that pass openspec's strict check. True when, after `devenv shell -- openspec archive each-repository-is-named-once-by-its-owner --yes`, `sluice exec main -- moon run root:specs` passes.
