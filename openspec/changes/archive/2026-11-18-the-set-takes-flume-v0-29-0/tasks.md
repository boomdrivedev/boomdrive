# Tasks

Work through the tasks in order. A task is true once the check it names passes, run as it names it.

- Every case evaluates against no store, as `nix/check.sh` does, with core.41's neutral names. Only 1.1's lock reaches the network.
- No task moves an input other than Flume's, or adds a crate, a tool or a tracked file type (core.7, core.8). The runner's code and its file do not change (core.5). Nothing outside the paths proposal.md's Touches lists is changed but this change's own files.

## 1. The set's Flume at v0.29.0

- [x] 1.1 `flake.nix` names Flume `v0.29.0`, and the lock moves its `flume` node alone, locked at the tag's commit `7e9be28c` (core.44, core.45). True when `sluice exec main -- moon run nix:typecheck` passes.
- [x] 1.2 A drive's label option is `owed`, given Flume as `owed`; `reserve` is declared hidden and fails evaluation, naming `owed`, whether or not the runner is on; `stream.agents` is from 1 (design Decisions 1 to 3). `nix/tests/drives.nix` reads `owed` and `reserved` (Flume record 0063). True when `sluice exec main -- moon run nix:test` passes, proving "A drive's stream by default", "A drive that writes only what it changes", "A host's own room and rule", "A label's old word" and "A stream of no agents", each on nix-darwin and on NixOS.
- [x] 1.3 The README's **Installing with nix** names a label's `owed`, the old word's failure and a label's `reserved` as Flume's own. True when `sluice exec main -- moon run root:format` passes.
- [x] 1.4 Record 0028 holds its amendment of 2026-11-18. True when `sluice exec main -- moon run root:format` passes.
- [x] 1.5 The nix files this change changes keep to the nix project's format and lint. True when `sluice exec main -- moon run nix:format` and `sluice exec main -- moon run nix:lint` pass.

## 2. The specs

- [x] 2.1 The change validates and archives into specs that pass OpenSpec's strict check (core.61). True when `sluice exec main -- moon run root:specs root:changes` passes.
