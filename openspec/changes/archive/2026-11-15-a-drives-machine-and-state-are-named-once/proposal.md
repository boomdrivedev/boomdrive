# Proposal

## Why

The brief: "A host names each drive's machine and its account's crew state repository once, in its Boomdrive accounts, and the set passes each drive's machine and its account's state to crew's module, so services.crew.settings names neither, and the runner's crew table takes them from there too." Its sources are record 0032, which decides it, and record 0028, which puts a drive's machine on the drive and the crew state repository on its account.

Today a host writes both facts twice. `services.boomdrive.accounts.<account>.drives.<drive>.machine` decides whether this host gives the drive a Flume stream (drives.nix). crew's own `services.crew.settings.drives.<drive>` names the drive's `machine` and `state` again. The runner's crew table is then read from crew's drives whose `machine` is `services.crew.host` (options.nix). Nothing keeps the two the same. A drive this host gives a stream, whose crew drive names another machine, gets agents nobody serves, and the reverse serves a drive with no stream (record 0032's context). The account names no crew state repository anywhere in the set, though record 0028 puts it there.

crew's module at the tag the lock names, `v0.59.0`, already takes the two from another module. It declares a drive's `machine` and `state` as written once, "by the host's settings or by another module that gives it by the drive's label", and fails evaluation where both write one (crew's spec, "Another module gives a drive its machine and its state repository"). The change the-sets-crew-pin-takes-its-new-names moved the set to that crew. It left this to the change that builds record 0032, which this is.

## What Changes

- **Each account names its crew state repository.** `services.boomdrive.accounts.<account>.state_repository` is declared, unset by default: an address `<owner>/<name>`, as the runner's crew table and `boomdrive setup --state-repository` name it (core.12, core.46). It is no key of the runner's file (core.5).
- **The set gives crew each drive's machine and state.** With crew's module on, whether or not the runner is, the modules give `services.crew.settings.drives.<drive>` for each drive of each account that names its `state_repository`, this host's and others'. Its `machine` is the drive's `machine`, or `services.crew.host` where the drive names none. Its `state` is the account's `state_repository`. They give no other key of crew's drive, and nothing of an account that names no `state_repository`.
- **A host writes neither in crew's settings.** The modules give both at the options' own priority, in a definition whose place is `services.boomdrive.accounts`. A host that also writes a given drive's `machine` or `state` in crew's settings fails evaluation by crew's own check. crew's failure lists `services.boomdrive.accounts` among the places that write it.
- **The runner's crew table is read from the accounts.** Under crew, with crew's module on and no table written, `[control_plane.crew]` holds `services.crew.host` and each drive of this host under an account that names its `state_repository`. A drive of this host names no `machine`, or names `services.crew.host`. The table lists them account by account in the order of their names, each account's drives in the order of theirs. Each partition is the drive's name as `label`, with its account's `state_repository`. A table written out still wins, whole.
- **Under crew, both facts are named in the accounts or nowhere.** With the runner, a crew control plane and crew's module on, evaluation fails in two cases. An account that names a drive and no `state_repository` fails, naming that account's option. A drive of crew's settings that no account names fails, naming it and `services.boomdrive.accounts`. Where no drive of this host is left, the failure names the machines the accounts' drives name.
- **The quick start's NixOS drive file** names the account `me`'s state repository in `services.boomdrive.accounts` and neither the drive's machine nor its state in crew's settings. Its test holds the file to that (site.8).
- **The README** writes the account's `state_repository` in its nix block, and neither `state` nor `machine` in crew's. Its words say what the module gives crew, and what fails.
- **Record 0032 is amended** with this change's forms. **core.57**, and **core.18**'s and **core.46**'s details, are amended with it. All are written in this change's own commit.

**BREAKING**: a host under crew with crew's module on fails evaluation until it moves, in the switch that takes this change. It names each account's `state_repository` in `services.boomdrive.accounts`, names every drive of crew's there, and drops each such drive's `machine` and `state` from `services.crew.settings`. No code carries a host across (record 0032: "I want the system to be built the way it should be and then have agents fucking fix the data transition as necessary").

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `service-modules`: each account names its crew state repository. With crew's module on, the modules give crew each drive's machine and its account's state repository, and a host that writes either in crew's settings fails evaluation. The runner's crew table is written from the accounts' drives on this host. Under crew, an account with no state repository and a drive of crew's no account names each fail evaluation. The README writes them so.
- `quick-start`: NixOS's drive file names the account's state repository, and neither the drive's machine nor its state in crew's settings.

## Impact

- `nix/modules/accounts.nix`: each drive `drivesOf` lists carries its account's `state_repository`. A new `crewDrivesOf` gives crew's drives' `machine` and `state` from the accounts.
- `nix/modules/crew-drives.nix`: new. With crew's module on, it gives `services.crew.settings.drives`, its place `services.boomdrive.accounts`.
- `nix/modules/options.nix`: an account's `state_repository` option. The runner's crew table is read from the accounts. Two assertions are added, and the third's words change. Its header comment and the `machine` option's description name what crew is given.
- `nix/modules/darwin.nix` and `nix/modules/nixos.nix`: each imports `crew-drives.nix`, and its header comment names it.
- `nix/tests/settings.nix`: crew's own keys for a drive, apart from its machine and state. The accounts' fixtures of this change.
- `nix/tests/file.nix`: the account's state repository, by its type.
- `nix/tests/crew-drives.nix`: new, the cases of what crew is given, on each platform. `nix/tests/default.nix` takes it.
- `nix/tests/darwin.nix`, `nix/tests/nixos.nix` and `nix/tests/tools.nix`: the crew table's cases read the accounts. They gain the cases of the two new failures and the README's words.
- `nix/tests/quick-start.nix` and `site/src/start/nixos/boomdrive.nix`: the quick start's NixOS drive.
- `README.md`: **Installing with nix**.
- `docs/adr/0032-…`, `docs/architecture/core.md`: record 0032's amendment of 2026-11-15, core.57, and core.18's and core.46's details, written with this change.
- Nothing else changes:
  - The runner's code, its file's keys and `nix/tests/files` are as they are. The account's state repository is no key of the runner's file.
  - No flake input, tool or tag moves. crew `v0.59.0` already takes a drive's `machine` and `state` from another module.
  - No option of crew's is declared here.
  - `boomdrive setup`, on a host Nix does not manage, still writes crew's drive by `crew drive set`.
  - The quick start's words, `site/src/content/start/nixos-drive.md`, still hold: the file still declares the drive, crew's host and its state repository. Any rewording is the content kind's (site.5). This change carries out "The quick start" of `site/docs/site-design.md` as its site map has it: "a step changes only with its test passing".
- Outside this repository, afterthought/swancloud moves its hosts in the switch that takes a set carrying this change:
  - each account names its `state_repository` in `services.boomdrive.accounts`;
  - each drive of crew's is named under its account there;
  - each drive's `machine` and `state` leave `services.crew.settings`.

## Touches

```
nix/modules/accounts.nix
nix/modules/crew-drives.nix
nix/modules/options.nix
nix/modules/darwin.nix
nix/modules/nixos.nix
nix/tests/settings.nix
nix/tests/file.nix
nix/tests/crew-drives.nix
nix/tests/default.nix
nix/tests/darwin.nix
nix/tests/nixos.nix
nix/tests/tools.nix
nix/tests/quick-start.nix
site/src/start/nixos/boomdrive.nix
README.md
docs/adr/0032-a-drives-machine-and-its-accounts-crew-state-are-named-once-in-the-hosts-accounts-and-crew-is-given-them.md
docs/architecture/core.md
openspec/specs/service-modules/spec.md
openspec/specs/quick-start/spec.md
```
