# Verify

The verify stage of the change the-flake-configures-each-tool-by-its-own-module, on the branch `change/the-flake-configures-each-tool-by-its-own-module` at `a8a1382`, 2026-11-08.

**The work falls short: the gate fails on `nix:lint`.** Every requirement and scenario of the three spec deltas has its case, and those cases pass. Every task is ticked. The code stage had already archived the change, as task 3.4 asks, so this report sits in the archived directory. The change goes back to its code stage for the one finding below.

## The whole checks

`devenv shell -- sluice exec main`, run once on `a8a1382`. sluice ran the check-fast tasks the commit affects, 9 in all. It exited 1: `failed: nix:lint`.

| task | outcome |
|---|---|
| `nix:format` | passed |
| `nix:lint` | **failed** |
| `nix:typecheck` | passed: "every case holds" |
| `nix:test` | passed: "every case holds" |
| `boomdrive:test` | passed, the family tests among them, with the set's Flume v0.18.0 and sluice v0.76.0 |
| `root:format` | passed, 210 files, 0 changed |
| `root:specs` | passed, 16 of 16 |
| `root:changes` | passed, no open change |
| `root:releases` | passed |

`nix:lint` runs `statix check flake.nix`, which fails twice with statix's warning 20, "Avoid repeated keys in attribute sets":

- `flake.nix:30`, the input `sluice`: `inputs.nixpkgs.follows`, `inputs.rust-overlay.follows` and `inputs.nix-darwin.follows` repeat the key `inputs`.
- `flake.nix:36`, the input `flume`: the same three keys.

statix suggests `inputs = { nixpkgs.follows = …; rust-overlay.follows = …; nix-darwin.follows = …; };` for each. crew's input has two `follows`, not three, and passes. Tasks 1.1 and 1.2 made these lines. Each task named only `nix:typecheck` or `nix:test` as its check, so neither ran the lint.

sluice also said `would conflict: README.md, flake.lock, flake.nix, nix/packages/flume-git-crates.nix with main`. That concerns where the branch lands, not these checks. The branch was cut from `bolt/the-boomdrive-flake-configures-the-whole-host`. The design's Dependencies already expect `main`'s chore the-sets-flume-and-sluice-are-their-latest-releases to touch `flake.nix` and `flake.lock`, with this change's later tags kept.

## The specs

Each scenario below has its case under `nix/tests`, and `nix:typecheck` and `nix:test` hold every case.

### service-modules

- **The modules bring each tool's own module.** "Each tool's options are its own flake's" (`tools.nix`). Every option of `services.flume`, `services.sluice` and `services.crew` is declared under that tool's input, on both platforms. `flake.nix`'s `modulesOf` imports `inputs.<tool>.<platform>Modules.<tool>`, and no file of `nix/modules` declares a tool's option.
- **The flake re-exports each tool's module.** "The tools' modules re-exported": the names are `boomdrive crew default flume sluice`, and sluice's module alone declares `services.sluice` and no `services.boomdrive`.
- **The Boomdrive module is taken once however often it is imported.** "Imported twice": `default` and `boomdrive` together evaluate with no failed assertion. `boomdrive` carries `key = "github:agentplot/boomdrive#<platform>Modules.boomdrive"`.
- **The Boomdrive module turns on Flume's and sluice's modules and leaves crew's to the host.** Covered by "A host with the runner alone", "A host that keeps its own Flume file" and "The tools' user on NixOS". `nix/modules/tools.nix` sets each `enable` with `mkDefault true` under `services.boomdrive.enable`, and `nixos.nix` sets the three `user`s with `mkDefault`.
- **Each tool's package is the set's unless the host gives its own.** Covered by "The set's packages", "A host's own crew" and "No sluice of the set's". `tools.nix` sets each `package` with `mkDefault`, outside the `mkIf`, so the default holds whether or not the runner is enabled.
- **The Boomdrive module writes no file of the other tools'.** "The tools' activation is their own" checks it on nix-darwin and on NixOS against `toolsDarwinSystem` and `toolsNixosSystem`. Apart from the runner's own link, the activations are the same.
- **The modules hold no option that installs another tool.** "The tools option is gone" (`darwin.nix`, `nixos.nix`): `mkRemovedOptionModule`'s words name `services.flume.package` and `services.sluice.package`. "The README's words" (`cache.nix`): the README names all three `package` options and no `services.boomdrive.tools`.
- **The runner finds its tools on the profiles' PATH** (modified). Covered by "The set's tools", "A host's own tools" (both `package`s `null`) and "The user's profile first". The modules put `cfg.package` alone in `environment.systemPackages`.
- **A changed file or package restarts the runner, and nothing else does** (modified). "Nothing else changed on nix-darwin" compares systems that differ only in `services.flume.package`. The NixOS scenarios are `activate.sh`'s cases, as before.

### family-packages

- **Each package is made from the runner's tree or its tool's own flake at its release tag.** Covered by "Flume through its own flake" (the `drvPath` is Flume's flake's), "sluice through its own flake", "crew through its own flake" (`nix/packages/crew.nix` gives crew's tag version, and its source, name and home page are crew's) and "The runner from this tree". `nix/packages/flume.nix` and `flume-git-crates.nix` are gone.
- **Each package is made from its release tag's source** (removed). No case names it any more.
- **The flake gives the family's host tools as packages** (modified). Covered by "Every system's packages", where `typecheck.nix`'s set holds `crew`, and "No tool of the host's own".
- **The flake and its lock name one set** (modified). Covered by "A tag moved without the lock" and "A tool named by no release tag". `tags.nix` holds crew, Flume and sluice to `v<n>.<n>.<n>`. `flake.nix` names `v0.76.0`, `v0.18.0` and `v0.44.8`, each the lock's.

### binary-cache

- **The workflow builds and pushes every package of every system** (modified). Covered by "Each system has its job", "The runner is pushed with its tools", "A package that does not build" and "A branch's push" (`workflow.nix`). The workflow builds every package `nix eval` lists, crew among them, and needed no change.
- **A host substitutes the set, or builds the same paths** (modified). "A host's own nixpkgs" turns crew on and holds the profile to the flake's own `boomdrive`, `flume`, `sluice` and `crew`. "The host's trust" holds too.

## The tasks

All 15 tasks, 1.1 to 3.4, are ticked `[x]`, and each has its commit, `384ea1f` to `a8a1382`. Each check a task names passes in this run. Task 3.4 moved the over-long requirement's last sentences and its two scenarios, word for word, into a requirement of its own, "The modules hold no option that installs another tool". `root:specs` passes after the archive.

The words agree with the specs:

- core.46's sentence reads "the flake's modules run the runner".
- core.46's details say "each tool installed by its own module, its package the set's unless the host gives its own (record 0023)".
- CLAUDE.md and `openspec/config.yaml` name crew in the set, and each tool's own module beside `services.boomdrive`.
- The README names `services.crew` and each tool's `package`, and no longer names `services.boomdrive.tools` or `flume-git-crates.nix`.

## What goes back to the code stage

- `flake.nix`'s inputs `sluice` and `flume` each write their three `follows` under one `inputs = { … };`, so `statix check flake.nix` passes. Then `sluice exec main` passes as a whole. Nothing else this report checked needs to change.
