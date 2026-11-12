# Verify: the-sets-crew-pin-takes-its-new-names

The work on `change/the-sets-crew-pin-takes-its-new-names` was read at 934a470. The gate passes, the work meets every requirement and scenario of its change, and every task is ticked, so the change stays archived with this report.

- The gate ran once through sluice against `main` at 678ccfc. Every `check-fast` task it ran passed: `nix:test` and `nix:typecheck` held every case, and 578 Rust tests passed with none failed.
- The change's deltas modify 7 requirements, add 5, rename 1 and remove 2, across 26 scenarios. A case that passed in the gate asserts the outcome of each scenario.
- All 6 tasks are ticked. The code stage archived the change at 934a470, as task 2.1 says to. This report sits beside the archived change.
- The branch merges with no conflict into `main` at 678ccfc, and into `bolt/the-sets-crew-pin-takes-its-new-names` at fb0912d. The bolt has moved past the branch's base, 57b1d1e, taking sluice v0.86.0 in `flake.nix` and `flake.lock`. Its lock change touches the `sluice` node, and this branch's touches the `crew` node.

## Checks

- **The gate.** `sluice exec main` at 934a470 ran the `check-fast` tasks that 934a470 affects against `main` at 678ccfc. It exited 0, and sluice recorded 13 results under their keys for 934a470. moon completed 15 tasks.
  - Passed:
    - `nix:format`, `lint`, `typecheck` ("every case holds") and `test` ("every case holds", 1 m 9 s, run in full);
    - `boomdrive:test`, 578 tests passed and none failed, which the flake's inputs reach;
    - `root:format` (326 files, 0 changed), `specs` (23 of 23), `changes` (nothing open) and `releases`;
    - `site:install`, `check`, `prose`, `vocab`, `build` and `test`.
  - The other projects' tasks and `root`'s other tasks were not affected by 934a470, so sluice did not run them.
  - moon warned that it could not write its shared cache or its output archives outside the sandbox. devenv printed `Conflicting file` for `.mcp.json` and `.claude/settings.json`, which `.gitignore` ignores. None of these touched a task's result.
- The gate was run once, as this stage's brief says, and nothing was fixed here.

## Tasks

All 6 tasks of `tasks.md` are ticked `[x]`, and none is open. What they build is in the tree, as the design names it:

- **The pin** (1.1):
  - `flake.nix` names `github:agentplot/crew/v0.59.0`. `flake.lock` moves the `crew` node alone, to 6698fda at `v0.59.0`.
  - `flake.nix` takes `inputs.crew.packages.${system}.crew` unchanged, beside Flume's, with a comment citing core.44. `nix/packages/crew.nix` is removed, and nothing in the tree names it.
  - The breaking commit is 899f89d, `feat(nix)!: …`, as Task notes 1.1 spell it.
- **The runner's crew table from crew's drives** (1.2):
  - `options.nix` writes `partitions` from `lib.attrValues crew.settings.drives`, filtered on `machine == crew.host`. `attrValues` gives the drives in the order of their names, which are their labels. Each entry is `label` and `state_repository` alone, and the runner's key stays `partitions`.
  - The header comment, the `machine` option's description, `control_plane.crew`'s `defaultText` and `description`, and both failures' words name crew's drives as Task notes 1.2 give them. `machines` lists the drives' machines in label order.
  - `tools.nix`'s and `cache.nix`'s header comments say "its host, its drives and its hosts". `tools.nix` says the module writes none of crew's `hosts`, `runs` and `review`.
  - `settings.nix`'s fixtures use `driveOf` and `hosts` (`host-a` darwin, `host-b` nixos) by crew's names, and `partitionOf` is gone. `crewModule`, `crewFleet`, `crewOrder` and `crewElsewhere` are keyed by label. `crewOldRuns` and `crewOldReview` hold version 5's names.
  - `packages.nix`'s "crew through its own flake" holds the set's crew to the same `drvPath` as crew's flake's own package, at the lock's tag's version.
  - The cases in `darwin.nix`, `nixos.nix` and `tools.nix` that named crew's partitions now name crew's drives (Decision 7).
- **Version 6 and crew's old names** (1.3): `tools.nix`'s `loads`, `setsCrew`, `crewsKeys`, `oldRuns` and `oldReview` are as Decision 6 and Task notes 1.3 have them. Each runs on nix-darwin and on NixOS. The expected `version` is written in the test as 6 and is not read from crew.
- **The README** (1.4):
  - The block in **Installing with nix** is Task notes 1.4's, word for word.
  - The paragraph before it says the table is written in label order. It also says the module writes none of crew's `hosts`, `runs` or `review`.
  - The paragraph on a drive's `machine` names crew's drives.
  - The paragraph on overriding a tool says crew's version is crew's own flake's.
  - No nix block names `partitions`, ` flume = {`, ` every =` or ` hours =`. The `partitions` that remain are the runner's own tables, in its TOML examples and prose.
- **Format and lint** (1.5): `nix:format` and `nix:lint` passed in the gate.
- **The specs** (2.1): the deltas are archived into `openspec/specs/service-modules` and `family-packages`, and `root:specs` passes 23 of 23. `service-modules` holds the five added requirements and the renamed one. It no longer holds the two removed requirements, "…written from crew's partitions on this host" and "A host under crew that the runner cannot serve…".

The branch changes these paths against its base, 57b1d1e:

- the paths the proposal's Touches lists;
- the change's own files;
- the two specs its archive reconciled.

No crate, tool, tracked file type, configuration key of the runner's or protocol message is added. The runner's code is unchanged.

## Requirements and scenarios

Each case named below runs once per platform, as "<case> on nix-darwin" and "<case> on NixOS", where the spec asks for both.

| spec | requirement | scenario | asserted by |
|---|---|---|---|
| service-modules | The Boomdrive module turns on Flume's and sluice's modules and leaves crew's to the host | A host with the runner alone | `tools.nix` "A host with the runner alone" |
| | | A host that keeps its own Flume file | `tools.nix` "A host that keeps its own Flume file" |
| | | The tools' user on NixOS | `tools.nix` "The tools' user on NixOS", on `crewOn` (`settings.crewModule`, with drives and hosts) |
| | Each tool's package is the set's unless the host gives its own | The set's packages | `tools.nix` "The set's packages" |
| | | A host's own crew | `tools.nix` "A host's own crew" |
| | | No sluice of the set's | `tools.nix` "No sluice of the set's" |
| | A crew table written out wins whole | A table written out wins | `darwin.nix` and `nixos.nix` "A table written out wins", the table `garden` beside `crewFleet` |
| | No crew table beside another control plane | A hosted control plane beside crew | `darwin.nix` and `nixos.nix` "A hosted control plane beside crew" |
| | | No control plane beside crew | `darwin.nix` and `nixos.nix` "No control plane beside crew" |
| | The README writes no crew table beside crew's drives (renamed) | The README's crew table | `tools.nix` "The README's crew table" |
| | A drive of another machine gives Flume nothing | A drive of another machine | `drives.nix` "A drive of another machine" |
| | | A drive of this machine | `drives.nix` "A drive of this machine" |
| | Under crew, the runner's crew table is written from crew's drives on this host (added) | crew's drives on this host | `darwin.nix` and `nixos.nix` "crew's drives on this host", against `nix/tests/files/crew.toml` |
| | | crew's drives in label order | `tools.nix` "crew's drives in label order", on `crewOrder` |
| | Under crew, a host the runner cannot serve fails evaluation (added) | Under crew with neither | `tools.nix` "Under crew with neither" |
| | | No drive of crew's on this host | `tools.nix` "No drive of crew's on this host", naming `services.crew.host`, `host-a` and `host-b` |
| | A host on the set loads crew's teams file at version 6, by crew's names (added) | A host on the set's crew | `tools.nix` "A host on the set's crew" |
| | | crew's old way of running | `tools.nix` "crew's old way of running", on `crewOldRuns` |
| | | crew's old batch names | `tools.nix` "crew's old batch names", on `crewOldReview` |
| | The modules write none of crew's hosts, runs or review (added) | crew's keys stay the host's | `tools.nix` "crew's keys stay the host's" |
| | The README writes crew's settings by crew's names (added) | The README's crew settings | `tools.nix` "The README's crew settings" |
| family-packages | Each package is made from this tree, its tool's own flake at its release tag, or the flake's nixpkgs | Flume through its own flake | `packages.nix` "Flume through its own flake" |
| | | sluice through its own flake | `packages.nix` "sluice through its own flake" |
| | | crew through its own flake | `packages.nix` "crew through its own flake" |
| | | herdr through its own flake | `packages.nix` "herdr through its own flake" |
| | | The runner from this tree | `packages.nix` "The runner from this tree" |

The two removed requirements are replaced by the two added ones on crew's drives, as their reasons say. Their cases were renamed in this change, not dropped.

Task 1.1's cases, "Every system's packages" and "Each tag is the lock's", are `typecheck.nix`'s. They held in `nix:typecheck`. Task 1.4's cases that still hold, "The README's words", "The README's accounts" and "The README's lines", are `cache.nix`'s. They held in `nix:test`.

## Outside this repository

The change is breaking for a host that takes the set with crew on. Such a host, afterthought/swancloud's among them, must write its `services.crew.settings` by crew's version-6 names in the switch that takes this change. The proposal's Impact and the breaking commit 899f89d both say so, as crew record 0064's consequences already ask of it. This repository does nothing more for it.
