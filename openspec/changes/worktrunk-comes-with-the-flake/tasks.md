# Tasks

Build the tasks in order. A task is true once the check it names passes, run as it names it. The groups are headings, for reading.

- Every evaluated case runs against no store, as `nix/check.sh` does, with core.41's neutral names. No task builds a package, writes to the store, or reaches the network, but to lock worktrunk's input, which `nix flake lock` does in the devenv shell, outside the sandbox.
- No task builds or runs worktrunk.
- No task adds a crate, a tool of the gate or a tracked file type (core.7, core.8). The runner's code does not change, and nothing outside the paths proposal.md's Touches lists is changed.
- Record 0023 and core.md's details were amended in the commit that wrote this change, and no task changes them.

## 1. worktrunk in the set

- [x] 1.1 worktrunk's flake is an input at `v0.80.0`, following the flake's `nixpkgs` and `rust-overlay`, and the lock locks it. `packages.<system>.worktrunk` is its flake's package, given its tag as `VERGEN_GIT_DESCRIBE` by `nix/packages/worktrunk.nix`. The type check's set holds `worktrunk` (design Decisions 1 and 2; Task notes 1.1). True when `sluice exec main -- moon run nix:typecheck` passes, proving:
  - the family-packages delta's "Every system's packages", with `worktrunk` among the ten;
  - "No tool of the host's own";
  - each tag the lock's.
- [x] 1.2 The test holds the set's worktrunk to its flake's package, its source, dependencies, program and version, and to its tag as its `git describe`. It holds it to the worktrunk sluice's `devenv.nix` pins at sluice's locked tag. `tags.nix` holds worktrunk to a release tag (design Decisions 1 and 2; Task notes 1.2). True when `sluice exec main -- moon run nix:test` passes, proving the family-packages delta's "worktrunk through its own flake", "worktrunk is sluice's", "worktrunk other than sluice's" and "worktrunk named by no release tag".

## 2. The modules and the shell

- [x] 2.1 `nix/modules/apps.nix` declares `services.boomdrive.worktrunk.package`, the set's `worktrunk` by default. With the runner enabled it installs it where it is not `null`, and without the runner it installs none (design Decision 3; Task notes 2.1). True when `sluice exec main -- moon run nix:test` passes, proving the service-modules delta's "worktrunk with the runner", "A host's own worktrunk", "No worktrunk of the set's" and "No worktrunk without the runner" on nix-darwin and on NixOS.
- [x] 2.2 The modules' default worktrunk is the flake's own whatever the host's `pkgs` (design Decision 3; core.47; Task notes 2.2). True when `sluice exec main -- moon run nix:test` passes, proving the binary-cache delta's "A host's own nixpkgs" with `worktrunk` on nix-darwin and on NixOS.
- [x] 2.3 `devShells.<system>.default` holds the set's worktrunk beside the runner, Flume, sluice, crew and herdr (design Decision 4; Task notes 2.3). True when `sluice exec main -- moon run nix:test` passes, proving the way-in delta's "Every system's shell" with six packages.

## 3. The words

- [ ] 3.1 The README's **Installing with nix** names:
  - worktrunk in the set, and `#worktrunk` on the profile line;
  - `services.boomdrive.worktrunk.package`, and setting it `null` or to a host's own;
  - worktrunk in the shell;
  - worktrunk's shell integration as the person's own `wt config shell install`.

  `cache.nix`'s "The README's words" reads the option (Task notes 3.1). True when `sluice exec main -- moon run nix:test` passes, proving the service-modules delta's "The README's words".
- [ ] 3.2 CLAUDE.md and `openspec/config.yaml` name worktrunk in the flake's set and among what its modules install (Task notes 3.2). True when `sluice exec main -- moon run root:format` passes.

## 4. The specs

- [ ] 4.1 The spec deltas archive into specs that pass openspec's strict check. True when, after `devenv shell -- openspec archive worktrunk-comes-with-the-flake --yes`, `sluice exec main -- moon run root:specs` passes.
