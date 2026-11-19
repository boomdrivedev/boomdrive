# Tasks

Work through the tasks in order. A task is true once the check it names passes, run as it names it.

- Every case evaluates against no store, as `nix/check.sh` does, with core.41's neutral names. Only 1.1's lock reaches the network.
- No task moves an input other than adding Rex's, or adds a crate, a tool the gate runs or a tracked file type (core.7, core.8). The runner's code and its file do not change (core.5). Nothing outside the paths proposal.md's Touches lists is changed but this change's own files.

## 1. The set's Rex

- [x] 1.1 `flake.nix` names Rex's flake by `https://pkgs.superlogical.com/rex/nix/preview/0.1.0-preview.14.tar.gz`, its `nixpkgs` following the flake's, and the lock gains its `rex` node alone, locked by the hash Flume v0.29.0's `devenv.lock` locks (design Decision 1). The flake gives `rex`, Rex's flake's own package, for each system (design Decision 2). `nix/tests/typecheck.nix` lists `rex` in the set. True when `sluice exec main -- moon run nix:typecheck` passes.
- [x] 1.2 `nix/tests/packages.nix` proves "Rex through its own flake", "Rex other than its URL's release", "Rex is Flume's", "Rex other than Flume's" and "Rex moved without the lock" (design Decisions 2 and 3). True when `sluice exec main -- moon run nix:test` passes.
- [x] 1.3 `services.boomdrive.rex.package` installs the set's `rex` with the runner, a host's own replacing it and `null` installing none, and names Rex in no Flume setting (design Decision 4). `nix/tests/apps.nix` proves "Rex with the runner", "A host's own Rex", "No Rex of the set's", "No Rex without the runner" and "No Rex server of the modules'", and `nix/tests/cache.nix` "A host's own nixpkgs" with `rex` among the flake's own paths. True when `sluice exec main -- moon run nix:test` passes.
- [x] 1.4 The flake's shell holds Rex (design Decision 5), which `nix/tests/way-in.nix`'s "Every system's shell" proves, and the boot asks Rex its version by `rex version`, which `nix/tests/boot.nix`'s "Each tool's answer" and "Every tool installed answers" prove. True when `sluice exec main -- moon run nix:test` passes.
- [x] 1.5 The nix files this change changes keep to the nix project's format and lint. True when `sluice exec main -- moon run nix:format` and `sluice exec main -- moon run nix:lint` pass.

## 2. The words

- [ ] 2.1 The README names Rex among the set, `services.boomdrive.rex.package` and `github:boomdrivedev/boomdrive#rex`; `CLAUDE.md` and `openspec/config.yaml` name Rex among the set. True when `sluice exec main -- moon run root:format` passes.
- [ ] 2.2 Record 0023 holds its amendment of 2026-11-18, how Rex comes with the flake, and core.44's, core.45's and core.46's details say what they allow of Rex. True when `sluice exec main -- moon run root:format` passes.

## 3. The specs

- [ ] 3.1 The change validates and archives into specs that pass OpenSpec's strict check (core.61). True when `sluice exec main -- moon run root:specs root:changes` passes.
