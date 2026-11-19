# Proposal

## Why

The brief: "crew runs only the drives a host sets up for it, and an account needs a crew state repository only for one of those, so a drive kept just to read, like Lime Energy's on deneb, needs nothing more." Its sources are the signal `2026-11-18-boomdrive-planner-45e1d8d2/01-a-drive-kept-only-to-read` and record 0032 as amended on 2026-11-18 (`e6c35535`), which decides it.

Today the set gives crew every drive of an account that names its `state_repository`, and under crew it fails every account that names a drive and no `state_repository` (record 0032 as amended on 2026-11-15). So a drive the host never set up in crew becomes a crew drive with no main level. crew's own module then fails the host, since that drive lacks crew's own keys. And a drive kept only to read, whose account holds no drive crew runs, still needs a state repository. Lime Energy's repositories are kept on deneb as the drive `wes`, with no main level and no agents (afterthought/swancloud record 0059 as amended on 2026-11-13). Today deneb cannot name that drive in its accounts without both.

## What Changes

- **crew is given only the drives its settings set up.** With crew's module on, whether or not the runner is, the set gives a drive's `machine` and its account's `state_repository` only where the host's crew settings set up the drive, `services.crew.settings.drives.<drive>`. The drive's account must also name its `state_repository`. The set adds no drive to crew's settings. A drive of the accounts that crew's settings do not set up is given nothing, so crew has no drive of its name. It stays a drive like any other: its repositories kept, and its Flume stream given where it runs.
- **The runner's crew table holds only the drives crew runs.** Under crew, with crew's module on and no table written, each partition is a drive of this host under an account that names its `state_repository`, as now, and the drive must also be one crew's settings set up. The failure where no such drive is this host's names the machines of those drives alone.
- **An account needs a state repository only for a drive crew runs.** Under crew, with the runner and crew's module on, evaluation fails for an account that names no `state_repository` and holds a drive crew's settings set up, naming that account's option. An account none of whose drives crew's settings set up needs none.
- **Unchanged:** a host's second write of a given drive's `machine` or `state` in crew's settings still fails crew's own check, naming `services.boomdrive.accounts`. So does a drive of crew's that no account names.
- **The README** says what crew is given, and that an account needs a state repository only for a drive crew's settings set up.
- **Record 0032** is amended with this change's forms. **core.57**, and **core.18**'s and **core.46**'s details, are amended with it. All are written in this change's own commit.

No host moves. A host that evaluates today gives crew the same drives and the runner the same table: before this change, crew's own module already failed any drive it was given and not set up.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `service-modules`: with crew's module on, the modules give a drive's machine and its account's state repository only where crew's settings set up the drive, and add no drive to crew's. The runner's crew table holds only this host's drives that crew's settings set up. Under crew, only an account holding a drive crew's settings set up must name its state repository. The README says so.

## Impact

- `nix/modules/crew-drives.nix`: in place of defining `services.crew.settings.drives`, it adds a module to the type of crew's drives. That module gives `machine` and `state` to a drive crew's settings set up, under an account that names its state repository. Its definitions are placed at `services.boomdrive.accounts`.
- `nix/modules/accounts.nix`: its header comment says what `crewDrivesOf` is now read for. The function stays as it is.
- `nix/modules/options.nix`: the runner's crew table, the "no drive" failure and the state repository's failure read the drives crew's settings set up. The header comment, the `machine` and `state_repository` options' descriptions and the table's `defaultText` and `description` say so.
- `nix/tests/settings.nix`: the fixture `reading`. `nix/tests/crew-drives.nix`: the placement check reads each given value's definitions, and the case of a drive crew's settings do not set up holds it given nothing. `nix/tests/tools.nix`: the cases of the table and the state repository.
- `README.md`: **Installing with nix**.
- `docs/adr/0032-…`, `docs/architecture/core.md`: record 0032's forms of 2026-11-18, core.57, and core.18's and core.46's details, written with this change.
- Nothing else changes:
  - The runner's code, its file's keys and `nix/tests/files` stay as they are.
  - No flake input or tag moves. crew `v0.68.0`, the lock's, still takes a drive's `machine` and `state` once, from the host's settings or another module.
  - Flume's streams (`drives.nix`) and crew's kept repositories (`kept.nix`) are given as now, for every drive of this host and every kept address.
  - The quick start's drive is set up in crew, so its file and test stay as they are.
  - `boomdrive setup`, on a host Nix does not manage, still sets up one crew drive by `crew drive set`.
- Outside this repository, nothing must move. afterthought/swancloud may name deneb's drive `wes` under its account in `services.boomdrive.accounts`, with no crew keys and no state repository, once it takes a set carrying this change.

## Touches

```
nix/modules/crew-drives.nix
nix/modules/accounts.nix
nix/modules/options.nix
nix/tests/settings.nix
nix/tests/crew-drives.nix
nix/tests/tools.nix
README.md
docs/adr/0032-a-drives-machine-and-its-accounts-crew-state-are-named-once-in-the-hosts-accounts-and-crew-is-given-them.md
docs/architecture/core.md
openspec/specs/service-modules/spec.md
```
