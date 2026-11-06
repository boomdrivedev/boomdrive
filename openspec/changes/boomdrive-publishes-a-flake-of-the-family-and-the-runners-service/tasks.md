# Tasks

Build the groups in order. Each group stands on the ones before it, is one session's work for an agent that has read this change (proposal, design, specs) and its Task notes, and ends green on the moon tasks it names. A task is true once the moon task it names passes, run as it names it.

- Run each check as `devenv shell -- moon run <project>:<task>`, or through sluice while iterating, as CLAUDE.md says.
- Every test runs in scratch folders, with neutral names and paths (core.41). No test reads the user's configuration or home, writes to the store, or reaches the network. `flake.lock` is made outside the sandbox, by `nix flake lock`.
- The `nix` project's tasks evaluate and never build (design Decision 14). Only the shell realises the set's Flume and sluice, and only the workflow builds the runner's package.
- Every option, key, secret and variable a task adds names what it holds (core.12), as design Decisions 5, 8 and 14 name them.
- No task pushes to the cache, runs the workflow, or reads or asks for the cache's token. The user adds the token (design, After deploy).
- Nothing outside the paths proposal.md's Touches lists is changed. The runner's own code does not change.

## 1. The flake and its project

- [x] 1.1 `flake.nix` holds the `nixConfig` and inputs of Task notes 1.1 and outputs with no package yet, and `flake.lock` locks them. `nix/moon.yml` declares the `nix` project, `language: 'nix'`, with `format`, `lint`, `typecheck` and `test` tagged `check-fast`. `.moon/workspace.yml` lists it. `devenv.nix` pins nix, nixfmt, statix and actionlint, and `check-toolchain.sh` holds them (design Decisions 1 and 14; Task notes 1.3). True when `devenv shell -- moon run nix:format nix:lint root:toolchain root:format` passes.
- [x] 1.2 `nix/inputs.nix` gives the lock's inputs, `devenv.nix` sets `BOOMDRIVE_FLAKE_INPUTS` from it, and `nix/check.sh` evaluates `nix/tests` against no store, with no network and no nix daemon (design Decision 14; Task notes 1.2 and 1.4). True when `devenv shell -- moon run nix:typecheck nix:test` passes, `nix/tests` holding one case that evaluates the flake's outputs.
- [x] 1.3 Run this group's checks. True when `devenv shell -- moon run nix:format nix:lint nix:typecheck nix:test root:toolchain root:format` passes.
- [x] 1.4 The change carries the user's rulings of 2026-11-06: the cache `boomdrive`, signed by Cachix's own key; its token loaded from 1Password through a service account; the user's steps as commands; and sluice `v0.68.0` in the first set (design, The user's rulings and After deploy). The proposal, the design, the binary-cache and family-packages specs, record 0020, accepted, and core.47 say so, and devenv's own push waits on the user (design, The user's to rule on). True when `devenv shell -- moon run root:format root:adrs root:changes` passes.

## 2. The package set

- [x] 2.1 `packages.<system>.boomdrive` and `default` make the runner from this tree, its git crates' hashes in `nix/packages/git-crates.nix` beside their revisions (design Decision 3; Task notes 2.1). True when `devenv shell -- moon run nix:typecheck nix:test` passes, proving the family-packages spec's "The runner from this tree" and "weir moved without its hash", the latter over a `Cargo.lock` whose weir revision differs.
- [x] 2.2 `packages.<system>.flume` makes Flume at its tag (design Decision 3; Task notes 2.2). True when `devenv shell -- moon run nix:test` proves the family-packages spec's "Flume at its tag" and "An overridden input".
- [x] 2.3 `packages.<system>.sluice` is sluice's flake's, at its tag's version (design Decision 3; Task notes 2.3). True when `devenv shell -- moon run nix:test` proves the family-packages spec's "sluice through its own flake".
- [x] 2.4 `flume-herdr-plugin` and `sluice-herdr-plugin` are their tools' plugins (design Decision 3; Task notes 2.4). True when `devenv shell -- moon run nix:test` proves the family-packages spec's "Flume's plugin moves with Flume", and the same of sluice's.
- [x] 2.5 `nix:typecheck` fails where a tag in `flake.nix` is not the lock's (Task notes 2.5). True when `devenv shell -- moon run nix:typecheck nix:test` passes, `nix:typecheck` proving "Every system's packages" and "No tool of the host's own", and `nix:test` proving "A tag moved without the lock" over a lock that differs.
- [x] 2.6 Run this group's checks. True when `devenv shell -- moon run nix:format nix:lint nix:typecheck nix:test` passes.

## 3. The set proven

- [x] 3.1 `devenv.nix` gives the set's Flume and sluice by path, on no `PATH`, from the flake's own packages, and pulls from the family's cache (design Decision 6; Task notes 3.1). `crates/boomdrive/tests/family.rs` runs them on a scratch host (Task notes 3.2). The set's tags are Flume `v0.10.0` and sluice `v0.68.0`, or the newest release of each the test passes with, as design Decision 6 says. True when `devenv shell -- moon run boomdrive:test` proves the family-packages spec's "Each tool's version", "Flume's status of a scratch host", "A submission sluice has no record of" and "Not the host's tools".
- [x] 3.2 Run this group's checks. True when `devenv shell -- moon run boomdrive:format boomdrive:lint boomdrive:typecheck boomdrive:test nix:typecheck nix:test` passes.

## 4. The file

- [x] 4.1 `nix/modules/toml.nix` renders `settings`, and `nix/tests/files/empty.toml`, `crew.toml` and `hosted.toml` hold the three texts. `treefmt.nix` excludes `nix/tests/files/**`, saying why (design Decision 9; Task notes 4.1). True when `devenv shell -- moon run nix:test root:format` passes, `nix:test` proving the service-modules spec's "Only repositories", "A host under crew" and "A host under a hosted control plane" over `options.nix` evaluated alone.
- [x] 4.2 `nix/modules/options.nix` declares `services.boomdrive`, its packages the flake's own, and refuses a credential's key (design Decisions 8 and 12; Task notes 4.2). `crates/boomdrive/tests/module_files.rs` reads each rendered file. True when `devenv shell -- moon run nix:test boomdrive:test` passes, `nix:test` proving "A token in the hosted table" and `boomdrive:test` proving "The runner reads each file".
- [x] 4.3 Run this group's checks. True when `devenv shell -- moon run nix:format nix:lint nix:typecheck nix:test boomdrive:test root:format` passes.

## 5. The nix-darwin module

- [x] 5.1 `darwinModules.default` declares the agent and the tools (design Decisions 10 and 11; Task notes 5.1). True when `devenv shell -- moon run nix:test` proves the service-modules spec's "The agent", "No home for the primary user", "A new file on nix-darwin", "The set's tools", "A host's own tools", "The user's profile first", and "A host under crew" on nix-darwin.
- [x] 5.2 `nix/modules/activate.sh link` makes the link, run by the module's `postActivation` as the primary user (design Decision 9; Task notes 5.2). True when `devenv shell -- moon run nix:test root:format` passes, `nix:test` proving "An empty home", "A person's own file" and "Already linked", and that the module's activation runs it as the primary user with the module's file.
- [x] 5.3 Run this group's checks. True when `devenv shell -- moon run nix:format nix:lint nix:typecheck nix:test root:format` passes.

## 6. The NixOS module

- [x] 6.1 `nixosModules.default` declares the unit, the lingering user and the tools (design Decisions 10 and 11; Task notes 6.1). True when `devenv shell -- moon run nix:test` proves the service-modules spec's "The unit", "A hosted control plane on an old systemd", "The set's tools", "A host's own tools", "The user's profile first", and "A host under crew" on NixOS.
- [x] 6.2 `nix/modules/activate.sh keep` starts and restarts the unit, run by the module's user activation after its link (design Decision 10; Task notes 5.2). True when `devenv shell -- moon run nix:test` proves "Not running on NixOS", "Nothing changed on NixOS", "A new file on NixOS" and "Another user's activation", and that the user activation runs `link`, then `keep` with the unit's own command line.
- [ ] 6.3 Run this group's checks. True when `devenv shell -- moon run nix:format nix:lint nix:typecheck nix:test root:format` passes.

## 7. The binary cache

- [ ] 7.1 The flake's `nixConfig` and the modules' packages hold to the cache (design Decision 4; Task notes 7.1). True when `devenv shell -- moon run nix:test` proves the binary-cache spec's "The flake's nixConfig", "A host's own nixpkgs" and "The host's trust", on nix-darwin and on NixOS.
- [ ] 7.2 `.github/workflows/cache.yml` builds and pushes every package of every system (design Decision 5; Task notes 7.2). True when `devenv shell -- moon run nix:lint nix:test root:format` passes, `nix:lint` running actionlint on the workflow, and `nix:test` proving over the workflow's text "Each system has its job" and "A branch's push", and that the job names `environment: cachix`, `fail-fast: false` and `skipPush: true`, loads the cache's token from 1Password by `1password/load-secrets-action`, fails naming `OP_SERVICE_ACCOUNT_TOKEN` and then `CACHIX_AUTH_TOKEN` before its build step, and pushes only after it, as "The runner is pushed with its tools", "A package that does not build", "No service account token" and "No token" need.
- [ ] 7.3 Run this group's checks. True when `devenv shell -- moon run nix:format nix:lint nix:typecheck nix:test root:format` passes.

## 8. The README and the project's notes

- [ ] 8.1 README gains **Installing with nix** (Task notes 8.1). True when `devenv shell -- moon run boomdrive:test nix:test` passes, `tests/readme.rs` reading the README's TOML as before, and `nix:test` proving the binary-cache spec's "The README's lines".
- [ ] 8.2 `CLAUDE.md` names the `nix` project and what it owns, and `openspec/config.yaml` names the flake and the family's cache in its stack (Task notes 8.2). True when `devenv shell -- moon run root:format root:specs root:changes` passes.

## 9. The change's checks

- [ ] 9.1 Run the checks of every moon project this change touches. True when `devenv shell -- moon run nix:format nix:lint nix:typecheck nix:test boomdrive:format boomdrive:lint boomdrive:typecheck boomdrive:test root:format root:toolchain root:adrs root:specs root:changes` passes.
- [ ] 9.2 The spec deltas archive into specs that pass openspec's strict check, each requirement's text within openspec's 500 characters. True when, after `openspec archive boomdrive-publishes-a-flake-of-the-family-and-the-runners-service --yes` on a copy of `openspec/` with the pinned openspec, `openspec validate --specs --strict` passes all 15 specs there, and `devenv shell -- moon run root:specs root:changes` passes on the tree.
