# Tasks

Work through the tasks in order. A task is true once the check it names passes, run as it names it. The groups are headings for reading.

- **The checks.** Every evaluated case runs against no store, as `nix/check.sh` does, with core.41's neutral names. No check builds a package or reaches the network. The steps that reach it are design Task notes 0's, each in the devenv shell outside the sandbox: locking timber's input, and building `timber` and the marketplace once on this host before the task that adds each is ticked.
- **What no task adds or changes.**
  - No task adds a crate, a tool of the gate or a tracked file type (core.7, core.8).
  - The runner's code does not change.
  - Every `.nix` file written is formatted by nixfmt, as treefmt formats it.
  - Nothing outside the paths proposal.md's Touches lists is changed.
- **What is already written.** These were written in the commit that wrote this change, and no task changes them:
  - record 0023's amendment of 2026-11-13 and record 0044's;
  - core.13's, core.44's and core.46's details.

  No task installs timber's Claude Code plugin into a Claude Code home: that is the unit `the-companions-load-in-claude-codes-sessions` (design Decision 4). A task that relies on a choice the design names cites its Decision.

## 1. timber in the set

- [x] 1.1 `flake.nix` takes timber's flake at `v0.6.0`, following the flake's `nixpkgs` and `rust-overlay`, and the lock locks it as the one node `timber`. `packages.<system>.timber` is timber's flake's `default`, unchanged. `packages.<system>.timber-herdr-plugin` is `nix/packages/herdr-plugin.nix` of the set's timber, outside the `plugins` that `herdr-plugins` is called with. The flake's header and description name timber. The type check's set holds both, and `tags.nix` holds timber to a release tag (design Decisions 1 to 3 and 9; Task notes 0 and 1.1). True when `sluice exec main -- moon run nix:typecheck` passes, proving the family-packages delta's "Every system's packages", with the twenty, and "No tool of the host's own", with each tag the lock's.
- [x] 1.2 `nix/tests/packages.nix` holds the set's timber to its flake's package, its program and its tag's version. It holds timber's herdr plugin to its source's `plugins/herdr` at timber's version, its floor to the set's herdr, and timber to a release tag. `nix/tests/workflow.nix` holds each system's job to build both (design Decisions 1 to 3; Task notes 1.2). True when `sluice exec main -- moon run nix:test` passes, proving:
  - the family-packages delta's "timber through its own flake", "timber's plugin moves with timber", "herdr is Flume's" and "timber named by no release tag";
  - the binary-cache delta's "The runner is pushed with its tools".

## 2. timber's plugins

- [x] 2.1 `nix/packages/claude-plugins.nix` takes the set's timber. The marketplace `boomdrive` lists a third plugin, `timber`, at `./plugins/timber` and timber's version: its source's `plugins/claude-code`, copied with links followed, its manifest's version checked against timber's in the build, and `passthru.sources.timber` given. `nix/tests/companions.nix`'s marketplace case expects the three (design Decision 4; Task notes 0 and 2.1). True when `sluice exec main -- moon run nix:test` passes, proving the family-packages delta's "The marketplace's three plugins" and "No link into the store".
- [x] 2.2 `nix/tests/timber.nix`, added to `nix/tests/default.nix`'s test, holds both of timber's plugin manifests to the set's timber, and the marketplace's `timber` plugin to timber's own source (design Decisions 4 and 5; Task notes 2.2). True when `sluice exec main -- moon run nix:test` passes, proving the family-packages delta's "timber's plugins say the set's timber", "A timber plugin other than the set's timber" and "timber's Claude Code plugin from its command's source".

## 3. The modules and the shell

- [x] 3.1 `nix/modules/apps.nix` declares `services.boomdrive.timber.package`, the set's `timber` by default. With the runner enabled it installs it where it is not `null`, whether or not crew is on. Without the runner it installs none, and no step, job or unit of the modules runs the set's timber (design Decision 6; Task notes 3.1). True when `sluice exec main -- moon run nix:test` passes, proving:
  - the service-modules delta's "timber with the runner", "A host's own timber", "No timber of the set's", "No timber without the runner" and "No serving process of the modules'", each on nix-darwin and on NixOS;
  - the binary-cache delta's "A host's own nixpkgs", with `timber`.
- [x] 3.2 `services.boomdrive.herdr.plugins`' default is Flume's, sluice's and timber's plugins, then crew's companions' where crew is on (design Decision 7; Task notes 3.2). True when `sluice exec main -- moon run nix:test` passes, proving:
  - the service-modules delta's "The plugins linked on nix-darwin", "The plugins linked on NixOS", "The companions' plugins with crew" and "No companions without crew";
  - the binary-cache delta's "A host's own nixpkgs", with `timber-herdr-plugin`.
- [x] 3.3 `devShells.<system>.default` holds the set's timber beside the runner, Flume, sluice and crew (design Decision 8; Task notes 3.3). True when `sluice exec main -- moon run nix:test` passes, proving the way-in delta's "Every system's shell" with the eleven tools.

## 4. The words

- [x] 4.1 The README's **Installing with nix** names (design Decision 10; Task notes 4.1):
  - timber in the set and its two plugins;
  - `#timber` on the profile line's list;
  - `timber-herdr-plugin` among the plugins a host links by hand;
  - `services.boomdrive.timber.package` and its `null`;
  - `timber serve` as timber's own, started by its pane or command, or a host's own service;
  - timber's plugin in the marketplace, installed into no Claude Code home yet;
  - timber in the shell.

  `nix/tests/cache.nix` reads the option and timber's words. True when `sluice exec main -- moon run nix:test` passes, proving the service-modules delta's "The README's words" and "The README's timber".
- [ ] 4.2 CLAUDE.md and `openspec/config.yaml` name timber among the tools the flake pins together, its plugin in the set's marketplace, and timber among what the modules install with the runner (design Decision 10; Task notes 4.2). True when `sluice exec main -- moon run root:format` passes.

## 5. The specs

- [ ] 5.1 The spec deltas archive into specs that pass openspec's strict check. True when, after `devenv shell -- openspec archive timber-comes-with-boomdrives-flake --yes`, `sluice exec main -- moon run root:specs` passes.
