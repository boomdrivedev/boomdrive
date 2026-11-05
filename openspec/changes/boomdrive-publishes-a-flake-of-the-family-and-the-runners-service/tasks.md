# Tasks

Build the groups in order. Each group stands on the ones before it, is one session's work for an agent that has read this change (proposal, design, specs) and its Task notes, and ends green on the moon tasks it names. A task is true once the moon task it names passes, run as it names it.

- Run each check as `devenv shell -- moon run <project>:<task>`, or through sluice while iterating, as CLAUDE.md says.
- Every test runs in scratch folders, with neutral names and paths (core.41). No test reads the user's configuration or home, writes to the store, or reaches the network. `flake.lock` is made outside the sandbox, by `nix flake lock`.
- The `nix` project's tasks evaluate and never build (design Decision 12). Only the shell builds the set's Flume and sluice.
- Every option, key and variable a task adds names what it holds (core.12), as design Decisions 6 and 12 name them.
- Nothing outside the paths proposal.md's Touches lists is changed. The runner's own code does not change.

## 1. The flake and its project

- [ ] 1.1 `flake.nix` holds the inputs of Task notes 1.1 and outputs with no package yet, and `flake.lock` locks them. `nix/moon.yml` declares the `nix` project, `language: 'nix'`, with `format`, `lint`, `typecheck` and `test` tagged `check-fast`. `.moon/workspace.yml` lists it. `devenv.nix` pins nix, nixfmt and statix, and `check-toolchain.sh` holds them (design Decisions 1 and 12; Task notes 1.3). True when `devenv shell -- moon run nix:format nix:lint root:toolchain root:format` passes.
- [ ] 1.2 `nix/inputs.nix` gives the lock's inputs, `devenv.nix` sets `BOOMDRIVE_FLAKE_INPUTS` from it, and `nix/check.sh` evaluates `nix/tests` against a read-only store with no network (design Decision 12; Task notes 1.2 and 1.4). True when `devenv shell -- moon run nix:typecheck nix:test` passes, `nix/tests` holding one case that evaluates the flake's outputs.
- [ ] 1.3 Run this group's checks. True when `devenv shell -- moon run nix:format nix:lint nix:typecheck nix:test root:toolchain root:format` passes.

## 2. The package set

- [ ] 2.1 `packages.<system>.boomdrive` and `default` build the runner from this tree (design Decision 3; Task notes 2.1). True when `devenv shell -- moon run nix:typecheck nix:test` passes, proving the family-packages spec's "The runner from this tree" and that the package's `outputHashes` name `Cargo.lock`'s git packages.
- [ ] 2.2 `packages.<system>.flume` builds Flume at its tag (design Decision 3; Task notes 2.2). True when `devenv shell -- moon run nix:test` proves the family-packages spec's "Flume at its tag" and "An overridden input".
- [ ] 2.3 `packages.<system>.sluice` is sluice's flake's, at its tag's version (design Decision 3; Task notes 2.3). True when `devenv shell -- moon run nix:test` proves the family-packages spec's "sluice through its own flake".
- [ ] 2.4 `flume-herdr-plugin` and `sluice-herdr-plugin` are their tools' plugins (design Decision 3; Task notes 2.4). True when `devenv shell -- moon run nix:test` proves the family-packages spec's "Flume's plugin moves with Flume", and the same of sluice's.
- [ ] 2.5 `nix:typecheck` fails where a tag in `flake.nix` is not the lock's (Task notes 2.5). True when `devenv shell -- moon run nix:typecheck nix:test` passes, `nix:typecheck` proving "Every system's packages" and "No tool of the host's own", and `nix:test` proving "A tag moved without the lock" over a lock that differs.
- [ ] 2.6 Run this group's checks. True when `devenv shell -- moon run nix:format nix:lint nix:typecheck nix:test` passes.

## 3. The set proven

- [ ] 3.1 `devenv.nix` gives the set's Flume and sluice by path, on no `PATH`, built from the flake's own packages (design Decision 4; Task notes 3.1). `crates/boomdrive/tests/family.rs` runs them on a scratch host (Task notes 3.2). The set's tags are Flume `v0.10.0` and sluice `v0.67.0`, or the newest release of each the test passes with, as design Decision 4 says. True when `devenv shell -- moon run boomdrive:test` proves the family-packages spec's "Each tool's version", "Flume's status of a scratch host", "A submission sluice has no record of" and "Not the host's tools".
- [ ] 3.2 Run this group's checks. True when `devenv shell -- moon run boomdrive:format boomdrive:lint boomdrive:typecheck boomdrive:test nix:typecheck nix:test` passes.

## 4. The file

- [ ] 4.1 `nix/modules/toml.nix` renders `settings`, and `nix/tests/files/empty.toml`, `crew.toml` and `hosted.toml` hold the three texts. `treefmt.nix` excludes `nix/tests/files/**`, saying why (design Decision 7; Task notes 4.1). True when `devenv shell -- moon run nix:test root:format` passes, `nix:test` proving the service-modules spec's "Only repositories", "A host under crew" and "A host under a hosted control plane" over `options.nix` evaluated alone.
- [ ] 4.2 `nix/modules/options.nix` declares `services.boomdrive` and refuses a credential's key (design Decisions 6 and 10; Task notes 4.2). `crates/boomdrive/tests/module_files.rs` reads each rendered file. True when `devenv shell -- moon run nix:test boomdrive:test` passes, `nix:test` proving "A token in the hosted table" and `boomdrive:test` proving "The runner reads each file".
- [ ] 4.3 Run this group's checks. True when `devenv shell -- moon run nix:format nix:lint nix:typecheck nix:test boomdrive:test root:format` passes.

## 5. The nix-darwin module

- [ ] 5.1 `darwinModules.default` declares the agent and the tools (design Decisions 8 and 9; Task notes 5.1). True when `devenv shell -- moon run nix:test` proves the service-modules spec's "The agent", "No home for the primary user", "A new file on nix-darwin", "The set's tools", "A host's own tools", "The user's profile first", and "A host under crew" on nix-darwin.
- [ ] 5.2 `nix/modules/activate.sh link` makes the link, run by the module's `postActivation` as the primary user (design Decision 7; Task notes 5.2). True when `devenv shell -- moon run nix:test root:format` passes, `nix:test` proving "An empty home", "A person's own file" and "Already linked", and that the module's activation runs it as the primary user with the module's file.
- [ ] 5.3 Run this group's checks. True when `devenv shell -- moon run nix:format nix:lint nix:typecheck nix:test root:format` passes.

## 6. The NixOS module

- [ ] 6.1 `nixosModules.default` declares the unit, the lingering user and the tools (design Decisions 8 and 9; Task notes 6.1). True when `devenv shell -- moon run nix:test` proves the service-modules spec's "The unit", "A hosted control plane on an old systemd", "The set's tools", "A host's own tools", "The user's profile first", and "A host under crew" on NixOS.
- [ ] 6.2 `nix/modules/activate.sh keep` starts and restarts the unit, run by the module's user activation after its link (design Decision 8; Task notes 5.2). True when `devenv shell -- moon run nix:test` proves "Not running on NixOS", "Nothing changed on NixOS", "A new file on NixOS" and "Another user's activation", and that the user activation runs `link`, then `keep` with the unit's own command line.
- [ ] 6.3 Run this group's checks. True when `devenv shell -- moon run nix:format nix:lint nix:typecheck nix:test root:format` passes.

## 7. The README and the project's notes

- [ ] 7.1 README gains **Installing with nix** (Task notes 7.1). True when `devenv shell -- moon run boomdrive:test` passes with `tests/readme.rs` reading the README's TOML as before.
- [ ] 7.2 `CLAUDE.md` names the `nix` project, and `openspec/config.yaml` names the flake in its stack (Task notes 7.2). True when `devenv shell -- moon run root:format root:specs root:changes` passes.

## 8. The change's checks

- [ ] 8.1 Run the checks of every moon project this change touches. True when `devenv shell -- moon run nix:format nix:lint nix:typecheck nix:test boomdrive:format boomdrive:lint boomdrive:typecheck boomdrive:test root:format root:toolchain root:adrs root:specs root:changes` passes.
- [ ] 8.2 The spec deltas archive into specs that pass openspec's strict check, each requirement's text within openspec's 500 characters. True when, after `openspec archive boomdrive-publishes-a-flake-of-the-family-and-the-runners-service --yes` on a copy of `openspec/` with the pinned openspec, `openspec validate --specs --strict` passes all 14 specs there, and `devenv shell -- moon run root:specs root:changes` passes on the tree.
