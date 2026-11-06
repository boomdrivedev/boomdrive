# Verify: boomdrive-publishes-a-flake-of-the-family-and-the-runners-service

The work on `change/boomdrive-publishes-a-flake-of-the-family-and-the-runners-service`, read at 375d3a9, meets its change, so the change is archived with this report.

- Every task is ticked.
- The gate run through sluice passes every `check-fast` task, the `nix` project's four among them.
- The change has 17 requirements and 44 scenarios. A case of `nix:typecheck`, `nix:test` or `boomdrive:test` asserts the outcome of every scenario, except one. "The runner is pushed with its tools" happens on GitHub once the change lands, and the gate holds the workflow's text to it.
- A trial archive adds the three specs, and all 15 specs then pass `openspec validate --specs --strict`.
- The change's record is 0020. `main` has moved to e3aa0f8 since the branch was cut, landing #14 with record 0021, so each record keeps a number of its own. `git merge-tree` merges `main` and the branch with no conflict.

## Tasks

All 28 tasks of `tasks.md`, groups 1 to 9, are ticked `[x]`, and none is open. What they build is in the tree, as the design and its Task notes name it:

- **The flake and its project** (group 1).
  - `flake.nix` holds Task notes 1.1's `nixConfig` and inputs, and `flake.lock` locks them.
  - `nix/moon.yml` declares the `nix` project, `language: 'nix'`, with `format`, `lint`, `typecheck` and `test` tagged `check-fast`. `.moon/workspace.yml` lists it.
  - `devenv.nix` pins nix 2.34.8, nixfmt 1.5.0, statix 0.5.8-unstable-2026-07-17, deadnix 1.3.2 and actionlint 1.7.12, each by an assertion, and `check-toolchain.sh` checks each, statix by its store path's name.
  - `nix/inputs.nix` gives the lock's inputs. `nix/check.sh` evaluates `nix/tests` with `--offline --read-only` against `dummy://`, with nix's configuration and XDG folders under a scratch folder.
  - Task 1.4's rulings of 2026-11-06 are in the proposal, the design (The user's rulings), record 0020 (`status: accepted`) and core.47.
- **The package set** (group 2).
  - `nix/packages/boomdrive.nix` takes `Cargo.toml`, `Cargo.lock` and `crates/` by `lib.fileset`, and takes weir's git crates by `git-crates.nix`.
  - `flume.nix` builds Flume's tag, its weir crates by `flume-git-crates.nix`.
  - `sluice.nix` gives sluice's own flake's package `version` and `SLUICE_VERSION` from the tag `flake.nix` names, read by `tag.nix`, and only where the tree says `0.0.0`.
  - `herdr-plugin.nix` copies the tool's `plugins/herdr` with `dontFixup`.
  - `nix/tests/tags.nix` holds each tag to the lock's.
- **The set proven** (group 3).
  - `devenv.nix` gives `BOOMDRIVE_SET_FLUME` and `BOOMDRIVE_SET_SLUICE` by `lib.getExe` of the flake's own packages, puts neither on `PATH`, and sets `cachix.pull = [ "boomdrive" ]` with no push.
  - `crates/boomdrive/tests/family.rs` runs them on the scratch host.
  - The set's tags are Flume `v0.10.0` and sluice `v0.68.0`, and the lock's revisions are those tags' commits on GitHub (448dd92 and c7ee329).
- **The file** (group 4).
  - `nix/modules/toml.nix` renders `settings`, and `nix/tests/files/` holds the three texts.
  - `treefmt.nix` excludes `nix/tests/files/**` and says why.
  - `nix/modules/options.nix` declares `services.boomdrive`, whose `package` and `tools` default to the flake's own `packages.<system>`. Its credential assertion names every path it finds.
  - `crates/boomdrive/tests/module_files.rs` reads each rendered file with `config::read`.
- **The nix-darwin module** (group 5). `nix/modules/darwin.nix` declares `launchd.user.agents.boomdrive` and `system.requiresPrimaryUser`, with an assertion on the primary user's home. Its `postActivation` runs `activate.sh link` as the primary user, under `sudo --user`, with `|| true`.
- **The NixOS module** (group 6). `nix/modules/nixos.nix` declares `systemd.user.services.boomdrive` with `restartIfChanged = false`, `alice`'s `linger`, and the systemd-256 and declared-user assertions. Its `userActivationScripts.boomdrive` runs `link`, then `keep` with procps and systemd on `PATH`, for the service's user alone.
- **The binary cache** (group 7).
  - `.github/workflows/cache.yml` follows Task notes 7.2.
  - Each action is pinned by a commit, and each is the commit its commented tag names on GitHub: `actions/checkout` v7.0.1, `cachix/install-nix-action` v31.11.3, `1password/load-secrets-action` v5.0.1 and `cachix/cachix-action` v17.
  - Cachix's public API gives `boomdrive` as public, at `https://boomdrive.cachix.org`, with the one signing key `nixConfig` names.
- **The README and the project's notes** (group 8).
  - The README gains **Installing with nix**, which covers every point of Task notes 8.1.
  - `CLAUDE.md` names the `nix` project and that it owns the flake and `.github/workflows/cache.yml`.
  - `openspec/config.yaml`'s stack names the flake and the family's cache.
- **The change's checks** (group 9). See Checks.

Nothing outside the proposal's Touches changed: `git diff --stat main...HEAD` lists only those paths and the change's own files. The runner's own source, `crates/*/src`, is unchanged.

## Checks

- **The gate.** `sluice exec main` at 375d3a9, against `main` at e3aa0f8, passed all 25 `check-fast` tasks. This covers every task task 9.1 names: `nix:format`, `nix:lint`, `nix:typecheck` and `nix:test`, `boomdrive:format`, `boomdrive:lint`, `boomdrive:typecheck` and `boomdrive:test`, and `root:format`, `root:toolchain`, `root:adrs`, `root:specs` and `root:changes`.
  - `nix:typecheck` and `nix:test` each said `every case holds`.
  - `root:toolchain` found each of the nix project's pins: nix 2.34.8, nixfmt 1.5.0, deadnix 1.3.2, actionlint 1.7.12 and statix 0.5.8-unstable-2026-07-17.
  - 637 tests passed, with none failed or ignored. Among them are `family.rs`'s four and `module_files.rs`'s four.
  - sluice did not record the result, because this report's draft was in the worktree beyond the head. The landing's own gate records its own.
- **Task 9.2.** On a copy of `openspec/`, the pinned openspec 1.14.3 validated the change strictly and archived it, adding 17 requirements across three new specs: binary-cache 4, family-packages 6 and service-modules 7. `openspec validate --specs --strict` then passed all 15 specs.
- **Real builds, beyond the gate.** These go past what the gate proves, which is evaluation alone (design Decision 14).
  - The devenv shell's set runs as its tags: `flume 0.10.0` and `sluice 0.68.0`, at `/nix/store/…-flume-0.10.0` and `/nix/store/…-sluice-0.68.0`.
  - `nix build .#flume-herdr-plugin .#sluice-herdr-plugin` on this host built `flume-herdr-plugin-0.10.0` and `sluice-herdr-plugin-0.68.0`. Each holds its tool's `herdr-plugin.toml`, `bin` and `lib`.
  - A dry run of `nix build .#packages.aarch64-darwin.boomdrive` leaves only the runner's own derivation to build. Its vendored crates are already in the store, weir's git crates among them, by the hash `git-crates.nix` holds.
  - The runner's package itself was not built here. The host stood at a load near 80 with other agents' suites, and the design gives that build to the workflow, on every system, at the landing push (Decision 14; Risks, "The runner's package is first built after it lands").
- **`main` since the cut.** #14 (e3aa0f8) changes `flume::Flume::status_of`, which `family.rs` does not call, and core.5's details. It takes record 0021. `git merge-tree --write-tree main HEAD` gives a tree with no conflict.

## Requirements and scenarios

Each scenario's case. `nix:typecheck` and `nix:test` cases are named as `nix/tests/<file>` and the case's name. `boomdrive:test` cases are named by `crates/boomdrive/tests/<file>` and the function.

### Family packages

| scenario | case |
|---|---|
| Every system's packages | `typecheck.nix` "Every system's packages": the three systems alone, each with the five packages and `default`, each a derivation whose `drvPath` evaluates, and `default`'s `drvPath` is `boomdrive`'s |
| No tool of the host's own | `typecheck.nix` "No tool of the host's own": no package named, `pname`d or `mainProgram`ed git or herdr |
| A tag moved without the lock | `packages.nix` "A tag moved without the lock": one failure naming `sluice`, `v0.68.0` and `v0.67.0`; `typecheck.nix` "Each tag is the lock's" over the tree's own flake and lock |
| Flume at its tag | `packages.nix` "Flume at its tag": the source is the lock's Flume, the version `0.10.0`, on each system |
| sluice through its own flake | `packages.nix` "sluice through its own flake": the source, `pname`, `cargoBuildFlags` and homepage of sluice's flake's package, with version and `SLUICE_VERSION` `0.68.0`; and "sluice's version from its tree", left unchanged where the tree says 9.9.9 |
| The runner from this tree | `packages.nix` "The runner from this tree": the source's root is the tree, its filter takes `Cargo.toml`, `Cargo.lock` and `crates` of the top-level entries and leaves out the rest, and takes every file under `crates/` |
| An overridden input | `packages.nix` "An overridden input": Flume's input replaced by `nix/tests/tree-9.9.9`, the package made from it, version `9.9.9` |
| weir moved without its hash | `packages.nix` "weir moved without its hash": one failure for each of weir's three crates, each naming the crate, the moved revision and the entry's; and "The runner's git crates" and "Flume's git crates" over the real locks |
| Flume's plugin moves with Flume | `packages.nix` "Flume's plugin moves with Flume": `src` is the lock's Flume's `plugins/herdr`, version `0.10.0`, `dontFixup`; and "sluice's plugin moves with sluice" |
| Each tool's version | `family.rs` `each_tools_version_is_its_tags`: the hello's reader gives `flume 0.10.0` and `sluice 0.68.0`, the tags read from `flake.lock` |
| Flume's status of a scratch host | `family.rs` `flumes_status_of_a_scratch_host`: no place, and the host part's `running`, `bound`, `room` and `streams` sliced as Flume printed them, read by `facts` |
| A submission sluice has no record of | `family.rs` `a_submission_sluice_has_no_record_of`: sluice exits 1 with `error.kind` `not_found`, which `sluice::Cli::entry` reads as `Reply::Refused` in sluice's own words, the reader's form of an entry sluice lacks |
| Not the host's tools | `family.rs` `not_the_hosts_tools`: a `PATH` that finds stand-ins answering `0.0.1-host`, and the readers run the set's, no stand-in called; every case of the file checks the same |

### Binary cache

| scenario | case |
|---|---|
| The flake's nixConfig | `cache.nix` "The flake's nixConfig": exactly the one substituter and the one key |
| The README's lines | `cache.nix` "The README's lines": a nix block of **Installing with nix** names each as a nix string |
| Each system has its job | `workflow.nix` "Each system has its job": one job `build`, on `matrix.runner`, the matrix's systems the flake's, each on its spec's runner |
| The runner is pushed with its tools | `workflow.nix` "The runner is pushed with its tools": the build names every package of `nix eval`'s `packages.$SYSTEM`, writes their paths, and the last step pushes them to `boomdrive` alone. The run itself is GitHub's, after landing (design, After deploy, step 6) |
| A package that does not build | `workflow.nix` "A package that does not build": `fail-fast: false`, the push after the build, `skipPush: true`, no `continue-on-error`, `bash`, which GitHub runs with `-eo pipefail` |
| A branch's push | `workflow.nix` "A branch's push": `push` with branches `main` and tags `v*` alone, and `workflow_dispatch` |
| No service account token | `workflow.nix` "No service account token": `environment: cachix`, the check before the load and the build, the secret given to the check and the load alone, `exit 1` |
| No token | `workflow.nix` "No token": the load is `cachix-token` reading `op://GitHub-Actions/CACHIX_AUTH_TOKEN/credential`, the one `op://` of the file, the check after it and before cachix and the build, the output given to the check, cachix and the push alone, and no step echoes a token; "Each action pinned by its commit" |
| A host's own nixpkgs | `cache.nix` "A host's own nixpkgs on nix-darwin" and "… on NixOS": with an overlay giving `boomdrive`, `flume` and `sluice` of the host's own, the profile holds the flake's three and none of the overlay's |
| The host's trust | `cache.nix` "The host's trust on nix-darwin" and "… on NixOS": no substituter, trusted substituter or key setting, and no `extraOptions`, names `boomdrive.cachix.org` |

### Service modules

| scenario | case |
|---|---|
| Only repositories | `file.nix` "Only repositories": `empty.toml` byte for byte, `repositories = []` alone after the header; `module_files.rs` `the_empty_file_keeps_nothing_with_the_runners_defaults` |
| A host under crew | `file.nix` "A host under crew", `crew.toml`; `darwin.nix` "A host under crew on nix-darwin" and `nixos.nix` "A host under crew on NixOS", each module's `--config` the store file of that text |
| A host under a hosted control plane | `file.nix` "A host under a hosted control plane", `hosted.toml`; `nixos.nix` "A host under a hosted control plane on NixOS" |
| The runner reads each file | `module_files.rs`: `each_rendered_file_is_one_read_here`, and the three files read by `config::read` with their repositories, crew's table opened by `planes::open`, and the hosted table's partitions named by `planes::enroll` |
| A token in the hosted table | `file.nix` "A token in the hosted table": names `control_plane.hosted.token` and `boomdrive enroll`, finds a list element's `private_key` and `state_password` too, and does not take `monkey` for a key |
| The agent | `darwin.nix` "The agent": no assertion fails, `Label` `org.nixos.boomdrive`, `ProgramArguments` the set's runner with `run --config <file>`, `RunAtLoad`, `KeepAlive`, `ThrottleInterval` 30, `ProcessType` `Interactive`, `AbandonProcessGroup`, `HOME` and `PATH` alone, `HOME` `/Users/alice`, and both logs `/Users/alice/Library/Logs/boomdrive.log` |
| No home for the primary user | `darwin.nix` "No home for the primary user": the failure names `alice` and `users.users.alice.home` |
| The unit | `nixos.nix` "The unit": `ExecStart` the set's runner with the file, `wantedBy` `default.target`, the unit's lines `ConditionUser=alice`, `Restart=always`, `RestartSec=30s`, `StartLimitIntervalSec=0` and `KillMode=process`, and `alice` lingers while `bob` does not; "No such user on NixOS" |
| A hosted control plane on an old systemd | `nixos.nix` "A hosted control plane on an old systemd": systemd overridden to 255, the failure says `systemd-creds`, `256` and `255` |
| A new file on nix-darwin | `darwin.nix` "A new file on nix-darwin": a `fetch_interval` of `5m` changes the fourth argument alone |
| Not running on NixOS | `activate.sh` "Not running on NixOS": `systemctl --user --no-block start boomdrive.service`, no restart; `nixos.nix` "The user activation on NixOS" |
| Nothing changed on NixOS | `activate.sh` "Nothing changed on NixOS": `ps` of the main PID read, neither start nor restart, nothing said |
| A new file on NixOS | `activate.sh` "A new file on NixOS": restart, naming what ran; and "A new package on NixOS" |
| Another user's activation | `activate.sh` "Another user's activation": `bob`'s `keep` calls nothing, says nothing and makes nothing in the home; `nixos.nix` "The user activation on NixOS": `link` and `keep` only under `[ "$(id -un)" = alice ]` |
| An empty home | `activate.sh` "An empty home": the link made, one line, nothing moved aside, neither `systemctl` nor `ps` called |
| A person's own file | `activate.sh` "A person's own file": the file and, in a second case, a folder moved to `config.toml.before-nix`, the link made, both paths named |
| A file moved aside before | `activate.sh` "A file moved aside before": exit 1, both files as they were, both paths named on standard error; each module runs `link` with `\|\| true` (`darwin.nix` "The activation links the file on nix-darwin", `nixos.nix` "The user activation on NixOS") |
| Already linked | `activate.sh` "Already linked": exit 0, nothing said, nothing moved; and "A link to another file" replaced |
| The set's tools | `darwin.nix` "The set's tools on nix-darwin" and `nixos.nix` "The set's tools on NixOS": the profile holds the set's runner, Flume and sluice |
| A host's own tools | `darwin.nix` "A host's own tools on nix-darwin" and `nixos.nix` "A host's own tools on NixOS": with `tools = [ ]`, the runner and no set's Flume or sluice |
| The user's profile first | `darwin.nix` "The user's profile first on nix-darwin" and `nixos.nix` "The user's profile first on NixOS": `/etc/profiles/per-user/alice/bin` and the home's `.nix-profile/bin` before `/run/current-system/sw/bin`, no store folder, and on NixOS `PATH` the unit's one variable of its own |

## What waits on others

None of this holds the change back. Each is named in the design.

- **The user's steps** (design, After deploy): the environment `cachix`, the cache's token in 1Password, and the service account whose token is the environment's secret. Steps 1 to 5 come before the landing, so the landing push finds the token. Steps 6 and 7 watch the first run and the cache.
- **The user's ruling on devenv's own push** (design, The user's to rule on, 1). Until they rule, `devenv.nix` pulls only, which core.47 asks of it.
- **What the change asks of other tools and of a host's configuration** (design, What this change asks of others): it goes to the orchestrator, and nothing of it is in this tree.
