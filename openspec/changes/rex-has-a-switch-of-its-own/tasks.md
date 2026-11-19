# Tasks

A task is done once the check it names passes, run as it names it.

- Every case evaluates against no store, as `nix/check.sh` does, with core.41's neutral names. No case builds a package or reaches the network.
- No task adds a crate, a tool the gate runs or a tracked file type (core.7, core.8). The runner's code, the set's packages, the lock, the cache's workflow, the boot and the shell do not change. Nothing outside the paths `proposal.md`'s Touches lists is changed but this change's own files.
- The merge of `main` at `f954b77` into the bolt holds tasks 1.1 and 1.2's code, which the merge needed, and record 0023's amendment "Rex has a switch of its own" is written with this change.

## 1. Rex's switch

- [x] 1.1 `nix/modules/apps.nix` declares `services.boomdrive.rex.enable`, `switch "Rex"`, and puts `services.boomdrive.rex.package` in the system profile by `installing "rex"`, with the runner or without, its header comment and its option's description saying so (design Decision 1). Done when *Rex with the runner*, *A host's own Rex*, *No Rex of the set's*, *No Rex without the runner* and *No Rex server of the modules'*, each on nix-darwin and on NixOS, hold, with `sluice exec main -- moon run nix:test` passing.
- [x] 1.2 `nix/tests/apps.nix` gains *Rex without the runner*, on each platform, and *Every tool turned on without the runner* turns on Rex's switch and expects the set's `rex` (design Decision 2). Done when both, each on nix-darwin and on NixOS, hold, with `sluice exec main -- moon run nix:test` passing.
- [x] 1.3 The nix files keep to the nix project's format and lint. Done when `sluice exec main -- moon run nix:format nix:lint` passes.

## 2. The specs

- [ ] 2.1 The change validates and archives into specs that pass OpenSpec's strict check (core.61). Done when `sluice exec main -- moon run root:specs root:changes` passes.
