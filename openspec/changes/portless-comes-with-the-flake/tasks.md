# Tasks

Work through the tasks in order. A task is true once the check it names passes, run as it names it. The groups are headings for reading.

- Every evaluated case runs against no store, as `nix/check.sh` does, with core.41's neutral names. No check builds a package, runs Node.js or pnpm, or reaches the network. Two steps reach it, each in the devenv shell outside the sandbox: locking portless's input, and building portless once before task 1.1's check (Task notes 1.1).
- No task adds a crate, a tool of the gate or a tracked file type (core.7, core.8). The runner's code does not change. Every `.nix` file written is formatted by nixfmt, as treefmt formats it. Nothing outside the paths proposal.md's Touches lists is changed.
- Record 0023's amendment of 2026-11-11 and core.13's, core.44's and core.46's details were written in the commit that wrote this change, and no task changes them. A task that relies on a choice the design names cites its Decision.

## 1. portless in the set

- [x] 1.1 portless's source is an input at `v0.17.0`, `flake = false`, and the lock locks it. `packages.<system>.portless` is `nix/packages/portless.nix`'s build of its workspace's `packages/portless` alone, by `pnpm --filter=portless exec tsup` with the flake's nixpkgs' pnpm 11. Its fetch is `fetchPnpmDeps` with the one hash of design Decision 3. It is run by a wrapper of the flake's `nodejs_24` with openssl, procps, which and hostname after the host's `PATH`, given as `passthru.runtimeInputs` and `passthru.node`. Its version is its tagged `package.json`'s. The flake's header names it. The type check's set holds `portless` (design Decisions 1 to 4; Task notes 1.1). True when `sluice exec main -- moon run nix:typecheck` passes, proving the family-packages delta's "Every system's packages" with `portless` among the ten, "No tool of the host's own", and each tag the lock's.
- [x] 1.2 `tags.nix` holds portless to a release tag (design Decision 1; Task notes 1.2). True when `sluice exec main -- moon run nix:test` passes, proving the family-packages delta's "portless named by no release tag".
- [x] 1.3 The test holds the set's portless to its source and its tag, its fetch to one hash for every system under pnpm 11, its Node.js to the flake's `nodejs_24`, and its `PATH` to the flake's openssl, procps, which and hostname with no git (design Decisions 1 to 4; Task notes 1.3). True when `sluice exec main -- moon run nix:test` passes, proving the family-packages delta's "portless from its source", "portless other than its tag", "One fetch for every system", "The flake's Node.js" and "portless runs the host's git".

## 2. The modules, the shell and the cache

- [ ] 2.1 `nix/modules/apps.nix` declares `services.boomdrive.portless.package`, the set's `portless` by default. With the runner enabled it installs it unless it is `null`, and without the runner it does not (design Decision 5; Task notes 2.1). True when `sluice exec main -- moon run nix:test` passes, proving the service-modules delta's "portless with the runner", "A host's own portless", "No portless of the set's" and "No portless without the runner" on nix-darwin and on NixOS.
- [ ] 2.2 No launchd job, systemd unit or activation step of a system with the runner enabled names the set's portless (design Decision 6; Task notes 2.2). True when `sluice exec main -- moon run nix:test` passes, proving the service-modules delta's "No proxy of the modules'" on nix-darwin and on NixOS.
- [ ] 2.3 The flake's shell holds the set's portless after herdr (design Decision 7; Task notes 2.3). True when `sluice exec main -- moon run nix:test` passes, proving the way-in delta's "Every system's shell" with the six tools.
- [ ] 2.4 The modules' default portless is the flake's own whatever the host's `pkgs` (design Decisions 2 and 5; Task notes 2.4). True when `sluice exec main -- moon run nix:test` passes, proving the binary-cache delta's "A host's own nixpkgs" on nix-darwin and on NixOS, and "The runner is pushed with its tools" with `portless` among each system's packages.

## 3. The words

- [ ] 3.1 The README's **Installing with nix** names portless in the set, `#portless` on the profile line, `services.boomdrive.portless.package` and its `null`, the proxy as portless's own, started by its first `portless run` at a terminal or by `portless service install` and started once on a host, `portless trust` as the person's, and portless in the shell (design Decision 6; Task notes 3.1). True when `sluice exec main -- moon run nix:test` passes, proving the service-modules delta's "The README's words" and "The README's proxy".
- [ ] 3.2 CLAUDE.md and `openspec/config.yaml` name portless in the flake's set and among what the modules install with the runner (Task notes 3.2). True when `sluice exec main -- moon run root:format` passes.

## 4. The specs

- [ ] 4.1 The spec deltas, each holding the base's words where `worktrunk-comes-with-the-flake` landed first (design Sequencing; Task notes 4.1), archive into specs that pass openspec's strict check. True when, after `devenv shell -- openspec archive portless-comes-with-the-flake --yes`, `sluice exec main -- moon run root:specs` passes.
