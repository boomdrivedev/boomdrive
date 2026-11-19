# Tasks

## 1. The set and the module

- [x] 1.1 `flake.nix` names millpond `v0.1.0`; the set's `millpond` is its flake's package with its git crates by hash (Decision 1). True when `sluice exec main -- moon run nix:typecheck nix:test` passes, proving "millpond at its tag" and "Every system's packages".
- [x] 1.2 The module installs millpond, renders and links its host configuration, and gives the runner's `inboxes` (Decisions 2 and 3). True when `sluice exec main -- moon run nix:test` passes, proving "The set's millpond", "No millpond", "Two inboxes", "No inbox" and "An inbox named twice".
- [x] 1.3 The nix files keep to the project's format and lint. True when `sluice exec main -- moon run nix:format nix:lint` passes.

## 2. The specs

- [x] 2.1 The change validates and archives into specs that pass OpenSpec's strict check (core.61). True when `sluice exec main -- moon run root:specs root:changes` passes.
