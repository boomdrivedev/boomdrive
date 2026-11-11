# Verify

This is the verify stage of the change each-repository-is-named-once-by-its-owner. It ran on the branch `change/each-repository-is-named-once-by-its-owner` at `bbb0895`, 2026-11-08. The branch holds its base, `bolt/the-boomdrive-flake-configures-the-whole-host`, at `999f6b1` (task 1.5).

**The work holds.**

- The gate passes as a whole.
- Every requirement and scenario of the delta has its case, and the gate ran each one and it held.
- All 14 tasks are ticked.
- The code stage archived the change over its base, as task 3.3 asks. This report sits in the archived directory, and the change stays archived with it.

## The whole checks

`devenv shell -- sluice exec main` ran once on `bbb0895`. sluice ran the check-fast tasks the commit affects against `main` (`8911d44`), 21 in all, and exited 0. It printed "passed: 21 tasks" and "recorded 21 results under their keys for bbb0895".

| project | tasks | outcome |
|---|---|---|
| `nix` | `format`, `lint`, `typecheck`, `test` | Passed. `typecheck` and `test` each said "every case holds". |
| `boomdrive` | `format`, `lint`, `typecheck`, `test` | Passed. `tests/module_files.rs` ran 4 tests and all passed: `each_rendered_file_is_one_read_here`, `the_empty_file_keeps_nothing_with_the_runners_defaults`, `the_crew_file_opens_crews_implementation` and `the_hosted_file_names_its_partitions`. The family tests ran with the set's own tools: `each_tools_version_is_its_tags`, `flumes_status_of_a_scratch_host`, `not_the_hosts_tools` and `a_submission_sluice_has_no_record_of`. |
| `boomdrive-crew` | `format`, `lint`, `typecheck`, `test` | Passed. |
| `boomdrive-hosted` | `format`, `lint`, `typecheck`, `test` | Passed. |
| `root` | `format`, `specs`, `changes`, `adrs`, `releases` | Passed. `specs` passed 16 of 16. `changes` found no open change. treefmt formatted 219 files and changed none. `adrs` found 28 records. |

sluice also printed `would conflict: … with main`. It named the README, runner files and docs from the bolt's earlier units, and `flake.nix` and `flake.lock`. That warning is about where the bolt lands on `main`, not about these checks. The branch lands on its base, which already holds those files.

moon warned that it could not store blobs or archive task outputs into its cache, and the OTLP exporter failed to shut down. Both come from the sandbox the gate runs in, and neither changes a task's outcome.

## The base

The branch's merge base with `bolt/the-boomdrive-flake-configures-the-whole-host` is `999f6b1`, the bolt's tip, so the change holds its base. No conflict marker is left in the tree.

`nix/modules/options.nix` holds this change's `services.boomdrive.accounts` beside the base's crew table, which is written from crew's partitions on this host. Its two assertions hold beside this change's own.

`nix/tests/settings.nix` gives the base's crew values with `accounts` beside `settings`, each drive's repositories by owner. Those values are `crew`, `crewUnwritten`, `crewModule`, `crewFleet`, `crewOrder` and `crewElsewhere`.

`openspec/specs/service-modules/spec.md` keeps every requirement of the base's. It adds the delta's nine added requirements and holds the four it modifies.

## The specs

`nix/tests/default.nix` evaluates these files, among others, under `nix:test`:

- `file.nix`
- `darwin.nix`
- `nixos.nix`
- `tools.nix`
- `drives.nix`
- `cache.nix`

Each scenario below is a case in one of them, named by the scenario. Where a scenario names no platform, `drives.nix`'s `onEach` runs it on nix-darwin and on NixOS.

### service-modules

- **The options are the runner's file, its accounts always written** (modified).
  - "Only accounts", "A host under crew" and "A host under a hosted control plane" (`file.nix`): `fileOf` of `services.boomdrive` renders each file under `nix/tests/files` byte for byte. `darwin.nix` and `nixos.nix` hold the agent's and the unit's `--config` to `crew.toml`, `hosted.toml` and `empty.toml`.
  - "The runner reads each file": `module_files.rs` is unchanged, and each file under `nix/tests/files` changes in its header line alone.
- **A flat list of before fails evaluation** (modified).
  - "The flat list in settings" (`file.nix`): the failure names `settings.repositories` and `services.boomdrive.accounts.<account>.drives.<drive>.repositories.<owner>`.
  - "A stream in a drive's settings" (`file.nix`): an assertion fails, naming `settings.accounts` and `services.boomdrive.accounts`.
  - "A key a drive does not hold" (`file.nix`): `accounts.own.drives.home.pool` fails the render.
- **Under crew, the runner's crew table is written from crew's partitions on this host** (modified).
  - "crew's partitions on this host" (`darwin.nix`, `nixos.nix`): `crewUnwritten`, whose accounts are in `accounts`, beside `crewFleet`, renders `crew.toml` byte for byte on each platform.
  - "crew's order" (`tools.nix`, on each platform) holds.
  - The base's other crew cases hold: "A table written out wins" (`darwin.nix`, `nixos.nix`) and "No partition on this host" (`tools.nix`).
- **No crew table beside another control plane** (modified). "A hosted control plane beside crew" and "No control plane beside crew" (`darwin.nix`, `nixos.nix`) render `hosted.toml` and `empty.toml` byte for byte beside crew's module on each platform, with no failing assertion.
- **Each repository is named once, under its drive and its owner.**
  - "Owners turned into addresses" (`file.nix`) renders `repositories = ["agentplot/flume", "example/beta", "example/alpha"]`.
  - "An address of no repository" (`file.nix`): the failure names `example/inner/alpha`.
- **Each drive of this host gives Flume its stream.** The cases are "A drive's stream by default", "A drive that writes only what it changes" and "A stream named other than its drive" (`drives.nix`).
  - Every leaf is given by `lib.mkDefault`.
  - The default reserves are the stream submodule's own `mkDefault`, so a label the drive adds sits beside them.
- **A drive's stream is Flume's alone, and only with the runner.**
  - "With the runner off" (`drives.nix`): Flume's `streams` and `repos` read null.
  - "A drive's stream changes nothing of the runner's" (`drives.nix`): the runner's files are compared as text, and on nix-darwin the plists too.
- **Each repository is given its drive's stream in Flume's settings.** "Each address given its drive's stream" (`drives.nix`): Flume's `repos` are exactly the three addresses.
- **A drive of another machine gives Flume nothing.** The cases are "A drive of another machine" and "A drive of this machine" (`drives.nix`). In the first, the runner's file still holds `[accounts.work.drives.garden]` with `example/alpha`.
- **A drive that names a machine needs crew's host.** "No host to compare with" (`drives.nix`): the failure names `services.crew.host` and `accounts.work.drives.garden.machine`.
- **A host's own Flume settings win over the module's.** "A host's own room and rule" (`drives.nix`):
  - `agents` is 6.
  - The reserves are 1 and 2, with `keep_free` 1 beside them.
  - The repository's stream is `other`.
  - No assertion fails.
- **A drive, a repository and a stream are each given once.**
  - "A drive's name under two accounts" and "A repository in two drives" (`file.nix`). The second case also covers an address kept twice by one drive.
  - "One stream for two drives" (`drives.nix`).
- **The README names each drive's repositories by owner.** "The README's accounts" (`cache.nix`) holds, with "The README's words" still holding. The README's **Installing with nix**:
  - names `services.boomdrive.accounts`;
  - names each drive's stream keys and their defaults;
  - names `machine` against `services.crew.host` and `services.flume.settings.streams`;
  - sets `control_plane.type = "crew"` with no table;
  - names no `settings.accounts`.

Each of the delta's 13 requirements is in `openspec/specs/service-modules/spec.md`, word for word as the delta has it. `openspec list` finds no active change.

## The tasks

All 14 tasks, 1.1 to 1.5 and 2.1 to 3.3, are ticked `[x]`, and none is open. Each check a task names ran in this gate and passed:

- 1.1 to 1.3, 2.1 to 2.5 and 3.1: `nix:test`.
- 1.4: `boomdrive:test`'s `module_files`.
- 1.5: `nix:test` and `root:changes`.
- 2.6: `nix:lint`.
- 3.2: `root:format`.
- 3.3: `root:specs`, on the archived specs.

The change keeps to its constraints:

- Every path changed since `999f6b1` is under the proposal's Touches or the change's own directory.
- The runner's code, its protocol and `module_files.rs` are unchanged.
- No flake input or tag moved, and no crate, tool or tracked file type was added.
- Record 0028's amendment of 2026-11-08 and core.46's details came in the commit that writes the change, `dec0f5c`, and core.46 cites the amendment.
