# Verify: worktrunk-comes-with-the-flake

The work on `change/worktrunk-comes-with-the-flake` was read at 9bc8936. The gate passes, and the work holds against its change:

- every requirement and scenario of the deltas has a case in the tree, and each passed in the gate;
- every task, 1.1 to 4.1, is ticked;
- every path the branch changes is one the proposal's Touches lists, or one of the change's own files.

So this stage archives the change.

## Checks

`sluice exec main` at 9bc8936 ran the `check-fast` tasks that 9bc8936 affects, against `main` at bb240e8. It exited 0. moon completed 39 tasks, and sluice recorded 37 results, every one a pass, under their keys for 9bc8936.

- **nix:** `nix:format`, `nix:lint`, `nix:typecheck` and `nix:test`, each "every case holds".
- **root:**
  - `root:adrs` (41 records), `root:changes` (1 passed), `root:commits` (no errors in 514 commits);
  - `root:format` (328 files, none changed), `root:releases`, `root:specs` (23 passed, 0 failed), `root:toolchain` and `root:workflow`.
- **The crates:** each one's `format`, `lint`, `typecheck` and `test`. The tests:
  - `boomdrive`: 578 passed, 0 failed;
  - `boomdrive-hosted`: 173 passed, 0 failed;
  - `boomdrive-crew`: 135 passed, 0 failed;
  - `boomdrive-control`: 44 passed, 0 failed;
  - `boomdrive-bounded`: 6 passed, 0 failed.
- **site:** `site:install`, `site:check`, `site:vocab`, `site:prose`, `site:build`, `site:test` and `site:workflow`.
- **Warnings.** moon could not write its shared cache or its output archives outside the sandbox, and sluice named `.mcp.json` and `.claude/settings.json` as conflicting files. None touched a task's result.
- The gate was run once, as this stage's brief says, and nothing was fixed here.

### "The runner from this tree"

The case failed at 81d02b8, when `boomdrive:test` made `target/` while `nix:test` evaluated. Task 1.3 is its fix, at 9bc8936:

- **The listing.** The case's listing of the root leaves out the top-level entries `.gitignore` names, `target` among them. git tracks none of them, so the flake's own tree never holds one.
- **The inputs.** `nix:test` reads `.gitignore` among its inputs.
- **What still holds.** The case still holds the source to `Cargo.toml`, `Cargo.lock` and `crates`, and to leaving out every other entry of the root but those `.gitignore` names. It still holds every file under `crates/` as taken.

`target/` stood at the root when this run began, and the case passed. With the fix, it no longer turns on whether `target/` appears while it evaluates.

## Tasks

All 9 tasks, 1.1 to 4.1, are ticked `[x]`, and none is open. What each names is in the tree:

- **1.1:**
  - `flake.nix` takes `worktrunk` at `github:max-sixty/worktrunk/v0.80.0`, following the flake's `nixpkgs` and `rust-overlay`.
  - The lock gains `worktrunk` (rev `b49ca7e`, `original.ref` `v0.80.0`), with worktrunk's own `crane` and `flake-utils`, and no second `nixpkgs` or `rust-overlay`.
  - `packagesOf` gives `worktrunk` from `nix/packages/worktrunk.nix`, which gives the package its tag as `VERGEN_GIT_DESCRIBE`.
  - `typecheck.nix`'s set holds `worktrunk`.
  - `nix:typecheck` passed.
- **1.2:**
  - `packages.nix` holds "worktrunk through its own flake", "worktrunk is sluice's", "worktrunk other than sluice's" and "worktrunk named by no release tag".
  - `tags.nix`'s list of tools named by a release tag holds `worktrunk`.
  - `nix:test` passed.
- **1.3:** "The runner from this tree" leaves out what `.gitignore` names at the top, and `nix/moon.yml` gives `nix:test` the input `/.gitignore`. `nix:test` passed.
- **2.1:** `nix/modules/apps.nix` declares `services.boomdrive.worktrunk.package` as `installed "worktrunk"`, and puts it in `environment.systemPackages`. `nix/tests/apps.nix` holds the four cases on each platform. `nix:test` passed.
- **2.2:** `cache.nix`'s `names` holds `worktrunk`. `nix:test` passed.
- **2.3:** `nix/shell.nix` holds `set.worktrunk`. `way-in.nix`'s `shellTools` holds the six. `nix:test` passed.
- **3.1:**
  - The README's **Installing with nix** names worktrunk in the set and `github:agentplot/boomdrive#worktrunk`.
  - It names `services.boomdrive.worktrunk.package` with `null`, a host's own worktrunk, the shell's worktrunk, and `wt config shell install` as the person's own.
  - `cache.nix`'s "The README's words" reads the option, and `nix:test` passed.
- **3.2:** CLAUDE.md and `openspec/config.yaml` name worktrunk in the set and among what the modules install. `root:format` passed.
- **4.1:** The code stage archived the change at 62096e36 and moved it back out at 81d02b8, so that this stage archives it once. With the change open, `root:specs` passed 23 of 23. This stage archives it, the report with it.

## Requirements and scenarios

Each scenario's case is in the tree, and passed under the task named.

| spec | requirement | scenario | case | task |
|---|---|---|---|---|
| family-packages | The flake gives the family's host tools as packages | Every system's packages | `typecheck.nix`, the ten and `default` | `nix:typecheck` |
| family-packages | The flake gives the family's host tools as packages | No tool of the host's own | `typecheck.nix` | `nix:typecheck` |
| family-packages | The flake and its lock name one set | A tag moved without the lock; A tool named by no release tag; herdr named by no release tag | `packages.nix`, unchanged | `nix:test` |
| family-packages | The flake and its lock name one set | worktrunk named by no release tag | `packages.nix`, with `tags.nix` holding `worktrunk` | `nix:test` |
| family-packages | Each package is made from this tree, its tool's own flake at its release tag, or the flake's nixpkgs | worktrunk through its own flake | `packages.nix`: source, `cargoArtifacts`, `wt`, version `0.80.0`, `VERGEN_GIT_DESCRIBE` `v0.80.0` | `nix:test` |
| family-packages | Each package is made from this tree, … | The runner from this tree | `packages.nix`, its listing leaving out what `.gitignore` names | `nix:test` |
| family-packages | Each package is made from this tree, … | Flume, sluice, crew and herdr through their own flakes | `packages.nix`, unchanged | `nix:test` |
| family-packages | worktrunk is the release sluice's worktree driver was checked against | worktrunk is sluice's | `packages.nix`, reading sluice's `devenv.nix` at its locked tag | `nix:test` |
| family-packages | worktrunk is the release sluice's worktree driver was checked against | worktrunk other than sluice's | `packages.nix`, `worktrunkHeld` with `0.81.0` and `0.80.0` | `nix:test` |
| service-modules | The modules hold no option that installs a tool its own module installs | The tools option is gone | `apps.nix`, unchanged | `nix:test` |
| service-modules | The modules hold no option that installs a tool its own module installs | The README's words | `cache.nix`, with `services.boomdrive.worktrunk.package` | `nix:test` |
| service-modules | The modules install worktrunk, replaceable or dropped | worktrunk with the runner; A host's own worktrunk; No worktrunk of the set's; No worktrunk without the runner | `apps.nix`, each on nix-darwin and NixOS | `nix:test` |
| binary-cache | The workflow builds and pushes every package of every system | Each system has its job; The runner is pushed with its tools; A package that does not build; A branch's push | `workflow.nix`, which holds the workflow to build every package the flake lists, `worktrunk` among them | `nix:test` |
| binary-cache | A host substitutes the set, or builds the same paths | A host's own nixpkgs | `cache.nix`, `names` holding `worktrunk` | `nix:test` |
| binary-cache | A host substitutes the set, or builds the same paths | The host's trust | `cache.nix`, unchanged | `nix:test` |
| way-in | The flake gives a shell that starts herdr | Every system's shell | `way-in.nix`, six packages | `nix:test` |
| way-in | The flake gives a shell that starts herdr | Entering the shell; A command through the shell; Inside herdr already | `nix/tests/way-in.sh`, unchanged | `nix:test` |

The code stage reworded one sentence of the way-in delta, keeping its meaning, so the requirement stays within openspec's strict length.

## Beyond the change

- **Records and rules.** Record 0023's amendment of 2026-11-11 and core.44's and core.46's details are in the tree, as 6fc38d1 wrote them.
- **The runner's code.** It does not change, and no crate, gate tool or tracked file type is added.
- **The sibling unit has landed.** `bolt/the-flake-brings-its-companion-tools` has moved past this branch's base, 57b1d1e: it has landed `portless-comes-with-the-flake` (73cb3ae). The design's Sequencing says the unit that lands second merges the shared lines to hold both tools:
  - the flake, its lock, `apps.nix` and the tests' lists;
  - the README, CLAUDE.md and `openspec/config.yaml`;
  - core.44 and core.46, and record 0023;
  - the modified requirements.

  This unit lands second. That merge belongs to its landing, and this stage has not checked it.
