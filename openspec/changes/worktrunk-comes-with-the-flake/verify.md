# Verify: worktrunk-comes-with-the-flake

The work on `change/worktrunk-comes-with-the-flake` was read at 81d02b8. The gate fails: `nix:test` fails one case, "The runner from this tree". So the change is not archived, and it goes back to its code stage.

Read against its change, the work otherwise holds. Every requirement and scenario of the deltas has a case in the tree. Every task is ticked. Every path the branch changes is one the proposal's Touches lists, or one of the change's own files.

## Checks

`sluice exec main` at 81d02b8 ran the `check-fast` tasks that 81d02b8 affects, against `main` at bb240e8. It exited 1. moon completed 15 tasks and failed 1, and sluice recorded 14 results under their keys for 81d02b8.

- **Passed:**
  - `nix:format`, `nix:lint` and `nix:typecheck` ("every case holds");
  - `root:adrs`, `root:changes`, `root:format`, `root:releases` and `root:specs` (23 passed, 0 failed);
  - `boomdrive:test`: 578 tests passed and none failed;
  - `site:install`, `site:check`, `site:vocab`, `site:prose`, `site:build` and `site:test`.
- **Failed:** `nix:test`, with one line:

  ```
  nix: The runner from this tree: packages.aarch64-darwin.boomdrive's source takes target
  ```

- moon warned that it could not write its shared cache or its output archives outside the sandbox. Neither touched a task's result.
- The gate was run once, as this stage's brief says, and nothing was fixed here.

### Why `nix:test` failed

The failing case predates this change. It came with the flake (da3aabbb), and this change does not touch it or `nix/packages/boomdrive.nix`. It fails here because two tasks of the same gate run raced:

- **`target/` appeared during the run.** The place had no `target/` when the gate started. `boomdrive:test` ran beside `nix:test` and made `target/` at the tree's root at 16:54, while `nix:test`'s evaluation (16:53:57 to 16:55:15) was under way.
- **The case read the root's entries twice.** It lists the tree's top-level entries with `builtins.readDir ../..` and asks the runner's source filter about each. The filter is `lib.fileset.toSource`'s, whose tree of the root was read separately.
- **The two reads disagreed.** The filter's tree did not hold `target`, and the case's listing did. Where an entry is missing from its tree, the filter of the locked nixpkgs (`lib/fileset/internal.nix`, `_toSourceFilter`) falls back to the root, `tree.${head components} or tree`, and answers `true`. So the source "takes target", and the case fails.
- **Every other run passes.** Where `target/` is present for the whole evaluation, or absent for all of it, the two reads agree: the filter leaves `target` out, and the case holds.

The code stage chooses the fix. Either:

- the case stops racing the runner's build: it lists only the entries the filter's own tree saw, or leaves out what `.gitignore` ignores, such as `/target`;
- or the gate is run again on a tree whose `target/` stands throughout.

Either way, the gate must pass before this stage archives the change.

## Tasks

All 9 tasks, 1.1 to 4.1, are ticked `[x]`, and none is open. What each names is in the tree:

- **1.1:**
  - `flake.nix` takes `worktrunk` at `github:max-sixty/worktrunk/v0.80.0`, following the flake's `nixpkgs` and `rust-overlay`.
  - The lock gains `worktrunk` (rev `b49ca7e`, `original.ref` `v0.80.0`), `crane`, `flake-utils` and `systems`, and no second `nixpkgs` or `rust-overlay`.
  - `packagesOf` gives `worktrunk` from `nix/packages/worktrunk.nix`, which is `package.overrideAttrs { VERGEN_GIT_DESCRIBE = tag; }`.
  - `typecheck.nix`'s set holds `worktrunk`.
  - `nix:typecheck` passed.
- **1.2:**
  - `packages.nix` holds "worktrunk through its own flake", "worktrunk is sluice's", "worktrunk other than sluice's" and "worktrunk named by no release tag", as the task notes describe them.
  - `tags.nix`'s list of tools named by a release tag holds `worktrunk`.
  - These cases ran in the failing `nix:test` and printed no failure.
- **2.1:** `nix/modules/apps.nix` declares `services.boomdrive.worktrunk.package` as `installed "worktrunk"`, and puts it in `environment.systemPackages` after Ghostty's. `nix/tests/apps.nix` holds the four cases on each platform.
- **2.2:** `cache.nix`'s `names` holds `worktrunk`.
- **2.3:** `nix/shell.nix` holds `set.worktrunk` after `set.herdr`. `way-in.nix`'s `shellTools` holds the six.
- **3.1:**
  - The README's **Installing with nix** names worktrunk in the set and `#worktrunk` on the profile line.
  - It names `services.boomdrive.worktrunk.package` with `null`, a host's own `wt`, the shell's worktrunk, and `wt config shell install` as the person's own.
  - `cache.nix`'s "The README's words" reads the option.
- **3.2:** CLAUDE.md and `openspec/config.yaml` name worktrunk in the set and among what the modules install. `root:format` passed.
- **4.1:**
  - The code stage archived the change at 62096e36, and moved it back out at 81d02b8 so that this stage archives it once. `openspec/specs` at 81d02b8 is as it was at the branch's base, 57b1d1e.
  - At 81d02b8, `root:specs` passed 23 of 23 with the change open.
  - The archive this task names has not been made, since the gate fails.

## Requirements and scenarios

Each scenario's case is in the tree. Those under `nix:typecheck` passed. Those under `nix:test` printed no failure, but that task failed on another case, so it has not passed as a whole.

| spec | requirement | scenario | case |
|---|---|---|---|
| family-packages | The flake gives the family's host tools as packages | Every system's packages | `typecheck.nix`, the set of ten and `default` |
| family-packages | The flake gives the family's host tools as packages | No tool of the host's own | `typecheck.nix` |
| family-packages | The flake and its lock name one set | A tag moved without the lock; A tool named by no release tag; herdr named by no release tag | `packages.nix`, unchanged |
| family-packages | The flake and its lock name one set | worktrunk named by no release tag | `packages.nix`, with `tags.nix` holding `worktrunk` |
| family-packages | Each package is made from this tree, its tool's own flake at its release tag, or the flake's nixpkgs | worktrunk through its own flake | `packages.nix`: source, `cargoArtifacts`, `wt`, version `0.80.0`, `VERGEN_GIT_DESCRIBE` `v0.80.0` |
| family-packages | Each package is made from this tree, … | The runner from this tree | `packages.nix`, **failed** in this run (above) |
| family-packages | Each package is made from this tree, … | Flume, sluice, crew and herdr through their own flakes | `packages.nix`, unchanged |
| family-packages | worktrunk is the release sluice's worktree driver was checked against | worktrunk is sluice's | `packages.nix`, reading sluice's `devenv.nix` at its locked tag |
| family-packages | worktrunk is the release sluice's worktree driver was checked against | worktrunk other than sluice's | `packages.nix`, `worktrunkHeld` with `0.81.0` and `0.80.0` |
| service-modules | The modules hold no option that installs a tool its own module installs | The tools option is gone | `apps.nix`, unchanged |
| service-modules | The modules hold no option that installs a tool its own module installs | The README's words | `cache.nix`, with `services.boomdrive.worktrunk.package` |
| service-modules | The modules install worktrunk, replaceable or dropped | worktrunk with the runner; A host's own worktrunk; No worktrunk of the set's; No worktrunk without the runner | `apps.nix`, each on nix-darwin and NixOS |
| binary-cache | The workflow builds and pushes every package of every system | Each system has its job; The runner is pushed with its tools; A package that does not build; A branch's push | `workflow.nix`, which holds the workflow to build every package the flake lists, `worktrunk` among them |
| binary-cache | A host substitutes the set, or builds the same paths | A host's own nixpkgs | `cache.nix`, `names` holding `worktrunk` |
| binary-cache | A host substitutes the set, or builds the same paths | The host's trust | `cache.nix`, unchanged |
| way-in | The flake gives a shell that starts herdr | Every system's shell | `way-in.nix`, six packages |
| way-in | The flake gives a shell that starts herdr | Entering the shell; A command through the shell; Inside herdr already | `nix/tests/way-in.sh`, unchanged |

The code stage reworded one sentence of the way-in delta, keeping its meaning, so the requirement stays within openspec's strict length.

## Beyond the change

- **Records and rules.** Record 0023's amendment of 2026-11-11 and core.44's and core.46's details are in the tree, as 6fc38d1 wrote them.
- **The runner's code.** It does not change, and no crate, gate tool or tracked file type is added.
- **The sibling unit has landed.** `bolt/the-flake-brings-its-companion-tools` has moved past this branch's base: it has landed `portless-comes-with-the-flake` (73cb3ae). The design's Sequencing says the unit that lands second merges the shared lines to hold both tools:
  - the flake, its lock, `apps.nix` and the tests' lists;
  - the README, CLAUDE.md and `openspec/config.yaml`;
  - core.44 and core.46, and record 0023;
  - the modified requirements.

  This unit lands second. That merge belongs to its landing, and this stage has not checked it.
