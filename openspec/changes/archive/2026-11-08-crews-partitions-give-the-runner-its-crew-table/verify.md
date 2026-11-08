# Verify

The verify stage of the change crews-partitions-give-the-runner-its-crew-table, on the branch `change/crews-partitions-give-the-runner-its-crew-table` at `1de25e5`, 2026-11-08. The branch was cut from `bolt/the-boomdrive-flake-configures-the-whole-host` at `4f39b90`, which it holds unchanged beneath its own commits.

**The work holds.** The gate passes as a whole. Every requirement and scenario of the service-modules delta has its case, which the gate ran and which holds. All 8 tasks are ticked. The code stage archived the change as task 3.3 asks, so this report sits in the archived directory, and the change stays archived with it.

## The whole checks

`devenv shell -- sluice exec main`, run once on `1de25e5`. sluice ran the check-fast tasks the commit affects against `main` (`8911d44`), 21 in all, and exited 0: "passed: 21 tasks", "recorded 21 results under their keys for 1de25e5".

| project | tasks | outcome |
|---|---|---|
| `nix` | `format`, `lint`, `typecheck`, `test` | passed; `typecheck` and `test` each "every case holds" |
| `boomdrive` | `format`, `lint`, `typecheck`, `test` | passed. `tests/module_files.rs` read every file under `nix/tests/files`, `crew.toml` among them. The family tests (`each_tools_version_is_its_tags`, `flumes_status_of_a_scratch_host`, `not_the_hosts_tools`, `a_submission_sluice_has_no_record_of`) ran with the set's tools |
| `boomdrive-crew` | `format`, `lint`, `typecheck`, `test` | passed |
| `boomdrive-hosted` | `format`, `lint`, `typecheck`, `test` | passed |
| `root` | `format`, `specs`, `changes`, `adrs`, `releases` | passed: `specs` 16 of 16, `service-modules` among them; `changes` no open change; `adrs` 28 records; treefmt 215 files, 0 changed |

sluice also said `would conflict: … with main`, naming `README.md`, record 0023, record 0028, `docs/architecture/core.md`, `flake.nix`, `flake.lock`, `nix/packages/flume-git-crates.nix`, `openspec/specs/views/spec.md` and runner files of the bolt's accounts and drives. This concerns where the bolt lands on `main`, not these checks: the branch lands on its base, which it holds unchanged. moon's warnings that it could not write its shared cache or archive outputs, that it had no internet connection, and the OTLP exporter's failure to shut down, belong to the sandbox the gate runs in. They change no task's outcome.

## The specs

`nix/tests/default.nix` gathers the cases of `outputs.nix`, `packages.nix`, `file.nix`, `darwin.nix`, `nixos.nix`, `tools.nix`, `cache.nix` and `workflow.nix` under `nix:test`, one attribute set merged by `//`. No case's name is given by two of those files, so none of this change's cases is shadowed. `tools.nix`'s `onEach` names each of its cases `<scenario> on <platform>`, one on nix-darwin and one on NixOS.

### service-modules

- **Under crew, the runner's crew table is written from crew's partitions on this host.**
  - "crew's partitions on this host": "crew's partitions on this host on nix-darwin" (`darwin.nix`) and "… on NixOS" (`nixos.nix`). The system is `settings.crewUnwritten`, "A host under crew" with `control_plane = { type = "crew"; }`, beside crew on with host `host-a` and `settings.crewFleet`, `home` on `host-a` at `example/state` and `away` on `host-b` at `example/away`. It fails no assertion, and the agent's or the unit's `--config` is `nix/tests/files/crew.toml`, byte for byte.
  - "crew's order": "crew's order on <platform>" (`tools.nix`). For `settings.crewOrder`, `garden` on `host-a`, `away` on `host-b` and `home` on `host-a`, `settings.control_plane.crew` equals the host `host-a` with `garden` at `example/garden` then `home` at `example/state`, each `label` and `state_repository` alone.
  - `nix/modules/options.nix` reads `config.services.crew or { enable = false; }` at the system's level, before the control plane's submodule binds its own `config`. `written` is crew's `host` and the partitions whose `machine` is that host, in crew's order, each `{ label; state_repository = state; }`, or `null` where crew is off. It is `control_plane.crew`'s default where the submodule's `type` is `crew`, and `null` otherwise, with a `defaultText` saying so (design Decisions 1 to 3).
- **A crew table written out wins whole.** "A table written out wins on nix-darwin" and "… on NixOS". With crew on and `crewFleet`, the settings of "A host under crew" whose table is `host-a` with `garden` at `example/garden` alone: no assertion fails, `settings.control_plane.crew` is that table, and the file is `toml.nix`'s rendering of those settings (Decision 4).
- **No crew table beside another control plane.**
  - "A hosted control plane beside crew on nix-darwin" and "… on NixOS": `settings.hosted` beside crew on with `crewModule`, its partition `home` on `host-a`. No assertion fails, and the file is `nix/tests/files/hosted.toml`.
  - "No control plane beside crew on nix-darwin" and "… on NixOS": `settings.empty` beside the same crew. No assertion fails, and the file is `nix/tests/files/empty.toml` (Decision 5).
- **A host under crew that the runner cannot serve fails evaluation.**
  - "Under crew with neither on <platform>" (`tools.nix`): `crewUnwritten` with crew off fails, its failures naming `settings.control_plane.crew` and `services.crew`. The assertion is `!(crewPlane && cfg.settings.control_plane.crew == null)`.
  - "No partition on this host on <platform>" (`tools.nix`): `crewUnwritten` with crew on, host `host-a` and `settings.crewElsewhere`, the one partition `away` on `host-b`, fails, its failures naming `services.crew.host`, `host-a` and `host-b`. The assertion holds only where the table is the module's `written` and its `partitions` are empty, and its message, built only where crew is on, names `services.crew.host`, its value and the partitions' machines, each once (Decision 6).
  - Both assertions sit in the Boomdrive module's `config`, under `services.boomdrive.enable`, beside its others.
- **The README writes no crew table beside crew's partitions.** "The README's crew table" (`tools.nix`): a nix block of the README's **Installing with nix** holds `type = "crew"`, none holds `state_repository`, and the section names `services.crew.host` and says "written out wins". The README's example now sets `control_plane.type = "crew";` beside `services.crew` with `host = "host-a"` and the partition `home`, and the words before it say how the module writes the table. `lib.nix` gives `installing` and `nixBlocks`, which `cache.nix` takes from it; the binary-cache spec's "The README's lines" and the service-modules spec's "The README's words" still hold under `nix:test`.

The delta stands in `openspec/specs/service-modules/spec.md`, its six requirements appended after the spec's existing ones, and `root:specs` passes it strictly.

## The tasks

All 8 of `tasks.md` are ticked, each by the commit that names it:

| task | commit |
|---|---|
| 1.1 | `c35666d` feat(nix): under crew, the runner's crew table is written from crew's partitions on this host |
| 1.2 | `baab834` test(nix): a crew table written out wins whole |
| 1.3 | `56d987b` test(nix): no crew table beside a hosted control plane or none |
| 2.1 | `695befe` feat(nix): under crew, neither a crew table nor crew's module fails evaluation |
| 2.2 | `522ba6f` feat(nix): under crew, no partition on this host fails evaluation |
| 3.1 | `5ca6ba8` docs(readme): the install with nix writes no crew table beside crew's partitions |
| 3.2 | `e35ed84` nix files format and lint clean, which the gate's `nix:format` and `nix:lint` confirm |
| 3.3 | `1de25e5` archive into the specs, which the gate's `root:specs` confirms |

## The bounds

- The files the change touched, from `4f39b90` to `1de25e5`, are `README.md`, record 0023, `nix/modules/options.nix`, `nix/tests/cache.nix`, `darwin.nix`, `lib.nix`, `nixos.nix`, `settings.nix` and `tools.nix`, the change's own directory and `openspec/specs/service-modules/spec.md`. Each is within proposal.md's Touches.
- No file under `crates/` or `nix/tests/files` changed, and no crate, tool or tracked file type was added (design Decision 7; core.2, core.7, core.8).
- Record 0023's amendment of 2026-11-08 names the change's choices, each with the path not taken, and no rule changed, as the design says (core.18's details already name the table).
- Every case uses core.41's neutral names: the hosts `host-a` and `host-b`, and repositories under `example/` and `agentplot/`.
