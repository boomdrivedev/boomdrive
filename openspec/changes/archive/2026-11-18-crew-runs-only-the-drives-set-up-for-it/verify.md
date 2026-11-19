# Verify

The verify stage's report on `change/crew-runs-only-the-drives-set-up-for-it` at `3869b30`, cut from `bolt/a-host-chooses-the-drives-crew-runs`.

The work holds. The whole checks pass, every task is done, the change passes OpenSpec's strict check, and every requirement and scenario of its spec is met.

## The whole checks

`devenv shell -- sluice exec main` was run once, on `3869b30` as it stood. It exited 0.

sluice ran the checks `3869b30` affects against `main` (`8e4015d`), seven tasks of `#check-fast`, and all seven passed. It recorded their results under their keys for `3869b30`.

| task | outcome |
|---|---|
| `nix:format` | passed |
| `nix:lint` | passed |
| `nix:typecheck` | passed: "every case holds" |
| `nix:test` | passed: "every case holds" |
| `boomdrive:test` | passed: every suite ok, none failed |
| `root:format` | passed: 494 files formatted, 0 changed |
| `root:changes` | passed: 1 open change, valid under `openspec validate --strict`, archiving to specs that pass it |

moon warned, for each task, that it failed to package the task's outputs into an archive under `.moon/cache/outputs`. That is no failure of a check: every task passed.

## OpenSpec

- `openspec instructions apply --change crew-runs-only-the-drives-set-up-for-it --json` gives `state` `all_done`, with 8 tasks, 8 complete and 0 remaining.
- `openspec validate crew-runs-only-the-drives-set-up-for-it --strict` says: "Change 'crew-runs-only-the-drives-set-up-for-it' is valid".

## The spec against the work

Each requirement of `specs/service-modules/spec.md`, and the code or case that meets it. `nix:test` ran every case named here, on nix-darwin and on NixOS, and passed.

- **With crew's module on, the modules give each drive crew's settings set up its machine and its account's state repository.**
  - `nix/modules/crew-drives.nix` declares `services.crew.settings` again. Its `drives` is an `attrsOf` of a submodule. That submodule gives `machine` and `state` from `crewDrivesOf` under `lib.mkIf (crew.enable && given ? ${name})`, and defines no drive.
  - The scenarios "Each drive's machine and state repository, given to crew", "Given without the runner", "An account naming no state repository gives crew nothing" and "A drive crew's settings do not configure" are cases of `nix/tests/crew-drives.nix`.
  - The last of these, `unconfigured`, holds crew's drives to be `home` alone. It also holds crew's teams file to evaluate and the system to fail no assertion.
- **A host writes neither a drive's machine nor its state in crew's settings.**
  - Both `_file`s are `services.boomdrive.accounts`.
  - "A machine written twice" and "A state written twice" each hold crew's settings failing.
  - Each also reads the given value's place from `valueMeta` by `placedFor`.
- **The modules bring each tool's own module.**
  - `outsideOf` in `nix/tests/tools.nix` fails an option with no declaration in its tool's input.
  - It also fails an option declared again outside that input, unless the declaration is crew-drives.nix's of `services.crew.settings` or `services.crew.settings.drives`.
  - It checks those two declarations' sub-options the same way.
  - crew-drives.nix's declaration sets no `description`, `default` or `example`.
  - "Each tool's options are its own flake's" passes.
- **Under crew, the runner's crew table is written from the accounts' drives on this host.**
  - In `nix/modules/options.nix`, `written` filters on `drive.state_repository != null && crewRuns drive`, where `crewRuns` reads `crew.settings.drives ? ${drive.name}`.
  - "The accounts' drives on this host", "The accounts' drives in their order" and "Only the drives crew's settings set up" (`setUp`) pass.
- **Under crew, a host the runner cannot serve from its accounts fails evaluation.**
  - `crewMachines` takes the same filter, and the failure's words are Decision 4's.
  - "Under crew with neither", "No drive of the accounts on this host" and "No drive crew runs on this host" (`elsewhereHere`, with `elsewhereHome`) pass.
- **Under crew, each account with a drive crew's settings set up names its state repository.**
  - `stateless` lists only an account with no `state_repository` that holds a drive of crew's settings, and the failure's words are Decision 3's.
  - "An account under crew with no state repository" and "An account whose drives crew's settings do not set up" (`reading`) pass.
  - The second holds crew's drives to be `home`, the table to be the one partition `home`, and Flume's streams to be `home` and `field`.
- **The README writes no crew table beside crew's drives**, and **the README names each drive's machine and its account's state repository once.**
  - **Installing with nix** says the module writes `[control_plane.crew]` from this host's drives that crew's settings set up, and names `services.crew.host`. It says a table written out wins.
  - It says each drive crew's settings set up is given its `machine` and its account's `state_repository`, and a drive they do not set up is given nothing.
  - It says an account needs a `state_repository` only for such a drive, and names `services.boomdrive.accounts.<account>.state_repository`.
  - Its nix block is unchanged: `state_repository = "example/state"` comes before `services.crew = {`, with no `state =` or `machine =` after it.
  - The paragraph on a drive's `machine` says the same.

Record 0032's amendment of 2026-11-18 on the module's forms, core.57, and core.18's and core.46's details are written as the design's Decision 7 has them. The tree changes only the paths the proposal's Touches lists. `openspec/specs/service-modules/spec.md` is left to the archive.

## Found

Nothing falls short.
