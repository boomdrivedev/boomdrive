# Verify: a-drives-machine-and-state-are-named-once

The work holds. The gate passes, all eleven tasks are ticked, and a case proves each scenario of both deltas on nix-darwin and on NixOS.

The code stage archived the change under task 3.3, since `root:specs` proves the deltas only once they are archived. This report sits beside the archive, on `change/a-drives-machine-and-state-are-named-once` at `4f91a96`.

## The whole checks

`sluice exec main` ran once on `4f91a96`. It ran the checks that commit affects against `main` (`23e4181`) and exited 0. All 12 passed, and sluice recorded their results under their keys for `4f91a96`:

- `nix`: `format`, `lint`, `typecheck` and `test` ("every case holds");
- `root`: `format`, `adrs`, `changes` and `specs`;
- `site`: `check`, `build` and `test`, after `site:install`;
- `boomdrive`: `test`.

`root:specs` passed 27 items, with none failed. `root:changes` found no open change to validate. moon warned that it could not write its output archives under `.moon/cache/outputs`. The warning comes from the sandbox, and no check failed because of it.

## Tasks

All eleven tasks are ticked, 1.1 to 3.3, and none is open.

- 1.1, 1.2, 1.3, 2.1, 2.2 and 3.1 rely on `nix:test`. 3.1 also relies on treefmt over `README.md`, which `root:format` runs.
- 3.2 relies on `nix:format` and `nix:lint`.
- 3.3 relies on `root:specs`, after the archive.

Each of those checks passed in the run above.

The diff from `main` stays inside proposal.md's Touches and the change's own folder. It adds no crate, gate tool or tracked file type. The runner's code, `nix/tests/files` and `flake.lock` are unchanged, and no option of crew's is declared. The quick start's words under `site/src/content/` are unchanged. Record 0032's amendment of 2026-11-15, core.57, and the details of core.18 and core.46 were written in the planning commit, `6eb1dbf2`, as the tasks say.

## The service-modules delta, scenario by scenario

A case named "… on nix-darwin" or "… on NixOS" has its twin on the other platform. `onEach` in `tools.nix` and `crew-drives.nix` makes both cases from one check.

### Modified

| Scenario | Proven by |
|---|---|
| Only accounts | `nix/tests/file.nix`'s "Only accounts", unchanged |
| A host under crew | `file.nix`'s "A host under crew", plus `darwin.nix`'s and `nixos.nix`'s "A host under crew on …". `settings.crew`'s account `own` names `example/state`, and the file is still `nix/tests/files/crew.toml` byte for byte, so the state repository is no key of it |
| A host under a hosted control plane | `file.nix`'s and `nixos.nix`'s cases of that name, unchanged |
| The runner reads each file | `crates/boomdrive/tests/module_files.rs`, unchanged, over the unchanged `nix/tests/files` |
| A table written out wins | "A table written out wins on …": the table is `garden` alone, and the file is its rendering, beside `settings.crewKeys` |
| The README's crew table | `tools.nix`'s "The README's crew table": some nix block sets `type = "crew"`, and none names `control_plane.crew` or `partitions`. The words name `services.crew.host` and say a table written out wins |

### Removed

Neither removed requirement is in `openspec/specs/service-modules/spec.md`. Their cases, "crew's drives on this host", "crew's drives in label order" and "No drive of crew's on this host", are renamed to the added requirements' scenarios below. The fixtures `crewFleet` and `crewOrder` are gone.

### Added

| Scenario | Proven by |
|---|---|
| A state repository of no address | `file.nix`'s "A state repository of no address": `state_repository = "example"` fails evaluation of `services.boomdrive.accounts` |
| Each drive's machine and state repository, given to crew | `nix/tests/crew-drives.nix`'s case of that name. `garden` and `home` get `host-a` and `example/state`, `away` gets `host-b` and `example/work`, and `field` gets `host-a` and `example/work`. crew's teams file evaluates and no assertion fails. A definition of `services.crew.settings` is placed at `services.boomdrive.accounts` |
| Given without the runner | "Given without the runner on …": the platform's `default` module with `services.boomdrive.enable` unset gives the same drives |
| An account naming no state repository gives crew nothing | "An account naming no state repository gives crew nothing on …": crew's `home` keeps its own `host-a` and `example/state`, the file evaluates and no assertion fails |
| A drive crew's settings do not configure | "A drive crew's settings do not configure on …": `crewAccounts` beside `crewKeys`, which configures `home` alone, and crew's teams file fails to evaluate |
| A machine written twice | "A machine written twice on …": `home`'s `machine` is also written as `host-a`, and the file fails to evaluate. crew's check refuses the second definition even when its value is the same. The definition is placed at `services.boomdrive.accounts` |
| A state written twice | "A state written twice on …": the same check for `home`'s `state`, `example/state` |
| The accounts' drives on this host | "The accounts' drives on this host on …": `settings.crewUnwritten` beside `crewKeys` writes `nix/tests/files/crew.toml` byte for byte, with no failure |
| The accounts' drives in their order | `tools.nix`'s "The accounts' drives in their order on …": the table is `host-a` with `garden` and `home` at `example/state`, then `field` at `example/work`, each holding `label` and `state_repository` alone. `away`, on `host-b`, is left out |
| Under crew with neither | `tools.nix`'s "Under crew with neither on …": the failure names `settings.control_plane.crew` and `services.crew` |
| No drive of the accounts on this host | `tools.nix`'s "No drive of the accounts on this host on …": `settings.elsewhere`, whose one drive `away` is on `host-b`, fails naming `services.crew.host`, `host-a` and `host-b` |
| An account under crew with no state repository | `tools.nix`'s "An account under crew with no state repository on …": `settings.stateless` beside `crewModule` fails naming `services.boomdrive.accounts.own.state_repository` |
| A drive of crew's no account names | `tools.nix`'s "A drive of crew's no account names on …": `crewStray` fails naming `stray` and `services.boomdrive.accounts` |
| The README's machine and state | `tools.nix`'s "The README's machine and state": the block naming `services.crew` names `state_repository = "example/state"` before `services.crew = {`, and neither `state =` nor `machine =` after it. The words name `services.boomdrive.accounts.<account>.state_repository` |

## The quick-start delta

| Scenario | Proven by |
|---|---|
| The drive on NixOS | `nix/tests/quick-start.nix`'s "quick start: NixOS's drive". The system evaluates with no failure, and the runner's accounts are `me`'s `home` keeping `your-github-name/your-repository`. The crew table is `your-host` with `home` at `your-github-name/drive-state`, and crew's `home` gets `your-host` and that state repository. Read as it stands, the file names the account's `state_repository` and writes neither `machine` nor `state` in crew's drive `home`. The Flume, stream, source and cache checks are unchanged |

## The modules

- `nix/modules/accounts.nix`: `drivesOf` gives each drive its account's `state_repository`, null where the account names none. `crewDrivesOf host` gives each drive under an account that names a state repository as `{ machine, state }`, its machine `host` where the drive names none.
- `nix/modules/crew-drives.nix`: its `_file` is `services.boomdrive.accounts`. Under `lib.mkIf crew.enable` alone, it gives `services.crew.settings.drives` from `crewDrivesOf` at the options' own priority. `darwin.nix` and `nixos.nix` import it beside `sources.nix`.
- `nix/modules/options.nix`:
  - It declares an account's `state_repository`, of the address type the runner's addresses take, which `fileOf` does not write.
  - It writes the runner's crew table from `hereOf crew.host` over the accounts' drives that name a state repository.
  - The "no drive" failure names the accounts' machines.
  - Two assertions are new, `stateless` and `unnamed`. Each holds only under crew, with crew's module on.

## The words

- The README's **Installing with nix** says:
  - an account names its crew state repository once, which the runner's file does not hold;
  - with crew's module on, whether or not the runner is, each such account's drives give crew's drive of that name its `machine` and its `state`;
  - a host that writes either in crew's settings fails evaluation, naming `services.boomdrive.accounts`;
  - the module writes the runner's crew table from the accounts' drives on this host in the accounts' order, and a table written out wins;
  - under crew, an account with a drive and no state repository, or a drive of crew's that no account names, fails.

  Its nix block moves `example/state` to `accounts.own.state_repository` and drops `state` and `machine` from crew's drive.
- The `machine` option's description and the README's words about it name `hostname -s` in place of crew's drives. They also say crew's drive is given the machine.

## For the landing

The change is breaking for a host under crew with crew's module on, as the proposal says. afterthought/swancloud's hosts move in the switch that takes a set carrying it: each account names its `state_repository`, each drive of crew's is named under its account, and each drive's `machine` and `state` leave `services.crew.settings`.

## Archive

The change is at `openspec/changes/archive/2026-11-15-a-drives-machine-and-state-are-named-once`, and no change of that name is open. `openspec/specs/service-modules/spec.md` and `openspec/specs/quick-start/spec.md` hold the deltas, and `root:specs` passes them under the strict check.
