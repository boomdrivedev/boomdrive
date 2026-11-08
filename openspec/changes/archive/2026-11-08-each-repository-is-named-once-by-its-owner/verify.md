# Verify

The verify stage of the change each-repository-is-named-once-by-its-owner, on the branch `change/each-repository-is-named-once-by-its-owner` at `dbbc95c`, 2026-11-08. The branch was cut from `bolt/the-boomdrive-flake-configures-the-whole-host` at `4f39b90`.

**The work holds.** The gate passes as a whole. Every requirement and scenario of the delta has its case, and the gate ran each case and it held. All 14 tasks are ticked. The code stage archived the change as task 3.3 asks, so this report sits in the archived directory, and the change stays archived with it.

## The whole checks

`devenv shell -- sluice exec main`, run once on `dbbc95c`. sluice ran the check-fast tasks the commit affects against `main` (`8911d44`), 21 in all, and exited 0: "passed: 21 tasks", "recorded 21 results under their keys for dbbc95c".

| project | tasks | outcome |
|---|---|---|
| `nix` | `format`, `lint`, `typecheck`, `test` | passed; `typecheck` and `test` each "every case holds" |
| `boomdrive` | `format`, `lint`, `typecheck`, `test` | passed. `tests/module_files.rs` ran 4 tests and all passed: `each_rendered_file_is_one_read_here`, `the_empty_file_keeps_nothing_with_the_runners_defaults`, `the_crew_file_opens_crews_implementation` and `the_hosted_file_names_its_partitions`. The family tests (`each_tools_version_is_its_tags`, `flumes_status_of_a_scratch_host`, `not_the_hosts_tools`, `a_submission_sluice_has_no_record_of`) ran with the set's own tools |
| `boomdrive-crew` | `format`, `lint`, `typecheck`, `test` | passed |
| `boomdrive-hosted` | `format`, `lint`, `typecheck`, `test` | passed |
| `root` | `format`, `specs`, `changes`, `adrs`, `releases` | passed: `specs` 16 of 16, `changes` no open change, treefmt 218 files, 0 changed |

sluice also printed `would conflict: … with main`, naming the README, runner files and docs from the bolt's earlier units. That is about where the bolt lands on `main`, not about these checks: the branch lands on its base, which already holds those files. moon warned that it could not archive task outputs into its cache, and the OTLP exporter failed to shut down. Both come from the sandbox the gate runs in and change no task's outcome.

## The specs

`nix/tests/default.nix` evaluates `file.nix`, `darwin.nix`, `nixos.nix`, `tools.nix`, `drives.nix` (added by this change) and `cache.nix`, among others, under `nix:test`. Each scenario below is a case of these, named by the scenario. Where a scenario names no platform, `drives.nix`'s `onEach` runs it on nix-darwin and on NixOS.

### service-modules

- **The options are the runner's file, its accounts always written** (modified).
  - "Only accounts", "A host under crew" and "A host under a hosted control plane" (`file.nix`): `fileOf` of `services.boomdrive` renders each file under `nix/tests/files` byte for byte. `darwin.nix` and `nixos.nix` hold the agent's and the unit's `--config` to `crew.toml`, `hosted.toml` and `empty.toml`.
  - "The runner reads each file": `module_files.rs`, unchanged. Each file changed in its header line alone.
- **A flat list of before fails evaluation** (modified).
  - "The flat list in settings" (`file.nix`): the failure names `settings.repositories` and `services.boomdrive.accounts.<account>.drives.<drive>.repositories.<owner>`.
  - "A stream in a drive's settings" (`file.nix`): an assertion fails, naming `settings.accounts` and `services.boomdrive.accounts`.
  - "A key a drive does not hold" (`file.nix`): `accounts.own.drives.home.pool` fails the render.
- **Each repository is named once, under its drive and its owner.**
  - "Owners turned into addresses" (`file.nix`) renders `repositories = ["agentplot/flume", "example/beta", "example/alpha"]`.
  - "An address of no repository" (`file.nix`): the failure names `example/inner/alpha`.
- **Each drive of this host gives Flume its stream.** "A drive's stream by default", "A drive that writes only what it changes" and "A stream named other than its drive" (`drives.nix`). Every leaf is given by `lib.mkDefault`. The default reserves are the stream submodule's own `mkDefault`, so a label the drive adds sits beside them.
- **A drive's stream is Flume's alone, and only with the runner.**
  - "With the runner off" (`drives.nix`): Flume's `streams` and `repos` read null.
  - "A drive's stream changes nothing of the runner's" (`drives.nix`): the runner's files are compared as text, and on nix-darwin the plists too.
- **Each repository is given its drive's stream in Flume's settings.** "Each address given its drive's stream" (`drives.nix`): Flume's `repos` are exactly the three addresses.
- **A drive of another machine gives Flume nothing.** "A drive of another machine" and "A drive of this machine" (`drives.nix`). In the first, the runner's file still holds `[accounts.work.drives.garden]` with `example/alpha`.
- **A drive that names a machine needs crew's host.** "No host to compare with" (`drives.nix`): the failure names `services.crew.host` and `accounts.work.drives.garden.machine`.
- **A host's own Flume settings win over the module's.** "A host's own room and rule" (`drives.nix`): `agents` is 6, the reserves are 1 and 2 with `keep_free` 1 beside them, the repository's stream is `other`, and no assertion fails.
- **A drive, a repository and a stream are each given once.**
  - "A drive's name under two accounts" and "A repository in two drives" (`file.nix`). The second case also covers an address kept twice by one drive.
  - "One stream for two drives" (`drives.nix`).
- **The README names each drive's repositories by owner.** "The README's accounts" (`cache.nix`), with "The README's words" still holding. The README's **Installing with nix** names `services.boomdrive.accounts`, each drive's stream keys and their defaults, `machine` against `services.crew.host`, and `services.flume.settings.streams`, and no `settings.accounts`.

Each requirement the delta adds or modifies is in `openspec/specs/service-modules/spec.md` exactly as the delta words it. `openspec list` finds no active change.

## The tasks

All 14 tasks, 1.1 to 3.3, are ticked `[x]`, and none is open. Each check a task names ran in this gate and passed:

- 1.1 to 1.3, 2.1 to 2.5 and 3.1: `nix:test`.
- 1.4: `boomdrive:test`'s `module_files`.
- 2.6: `nix:lint`.
- 3.2: `root:format`.
- 3.3: `root:specs`, on the specs archived.

The change keeps to its constraints:

- Every changed path is under the proposal's Touches or the change's own directory.
- The runner's code and `module_files.rs` are unchanged, no flake input or tag moved, and no crate, tool or tracked file type was added.
- Record 0028's amendment of 2026-11-08 and core.46's details came in the commit that writes the change, `dec0f5c`, and core.46 cites the amendment.
