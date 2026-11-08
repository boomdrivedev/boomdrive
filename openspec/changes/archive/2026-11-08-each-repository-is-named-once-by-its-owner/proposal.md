# Proposal

## Why

Record 0023, accepted by the user on 2026-11-06, has a host name each repository once, under its owner, and the module give each repository its stream in Flume's module, with no folder written. In the user's words, "it just has one flat list of repos, but we should be able to configure the streams and the repos as well", and, of a repository's stream, "One block by owner". Record 0028, accepted by the user on 2026-11-07, puts the owner under its account's drive, and the drive gives the stream:

- `services.boomdrive.accounts.<account>.drives.<drive>.repositories.<owner> = [ <names> ]`. A repository belongs to one drive.
- Each drive has its stream, named after the drive unless the drive names another. Its room, priority and each label's reserve are the drive's `stream` keys, and the module gives a default for each, deneb's: four agents, a reserve of one for operation and two for planning.
- A drive whose `machine` names another computer gets no stream and no agents on this one, and this host keeps its repositories to read. In the user's words, "Willdan work on DeNEB is just for reading and the work stays on the box".

Today the module writes the runner's file from `services.boomdrive.settings` alone, each drive's repositories a list of addresses, and gives Flume nothing: a host writes each repository in the runner's file, and again in Flume's, by hand. Flume's set tag, v0.18.0, names a repository's `[repos."<owner>/<name>"]` by its address (flume record 0055), so the module can give Flume each address's stream with no folder.

This change is the unit `each-repository-is-named-once-by-its-owner` of the bolt `the-boomdrive-flake-configures-the-whole-host`, after the unit that brings each tool's own module.

## What Changes

- **The accounts by owner.** `services.boomdrive.accounts` names each account, its drives, and each drive's repositories by owner. The module writes the runner's `accounts` from it, each address `<owner>/<name>`, owners in name order and each owner's names in the order given.
- **One place for the accounts.** `services.boomdrive.settings` holds the rest of the runner's file. A host's own `settings.accounts` fails evaluation, naming `services.boomdrive.accounts`, as a flat `settings.repositories` does.
- **Each drive's stream in Flume's module.** For each drive of this host, the module gives Flume's `settings.streams.<stream>` the drive's `agents`, `priority` and each label's `reserve`, with the module's defaults, and gives each of the drive's addresses `settings.repos."<owner>/<name>".stream`. Each is given at the options' default priority, so a host's own `services.flume.settings` wins.
- **A drive on another machine.** A drive's `machine`, compared with `services.crew.host`, gives no stream and no rule where it names another host. Its repositories are still kept.
- **Named once.** A drive's name across the accounts, a repository on the host, and a stream across the drives are each given once, or evaluation fails, naming what is given twice.
- **Words.**
  - The README's install with nix names each drive's repositories by owner and its stream.
  - CLAUDE.md says the module's `accounts` and `settings` are the runner's file.
  - Record 0028 gains this change's choices as an amendment, and core.46's details allow `services.boomdrive.accounts`, both in the commit that writes this change.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `service-modules`: `services.boomdrive.accounts` names each drive's repositories by owner, and the module writes the runner's `accounts` from it; a host's own `settings.accounts` fails evaluation; each drive of this host gives Flume its stream, with the module's defaults, and each address its drive's stream; a drive of another machine gives Flume nothing; a drive's name, a repository and a stream are each given once.

## Impact

- `nix/modules/`:
  - `options.nix` declares `accounts` and no longer declares `settings.accounts`.
  - `accounts.nix`, added, turns `accounts` into the runner's `accounts` table.
  - `drives.nix`, added, gives Flume each drive's stream and each address's rule.
  - `darwin.nix` and `nixos.nix` render the file with it and import `drives.nix`.
  - `toml.nix`'s header names both options.
- `nix/tests/`:
  - `drives.nix`, added to `default.nix`'s `test`.
  - `settings.nix`, `file.nix`, `darwin.nix`, `nixos.nix` and `cache.nix`.
  - The header line of each file under `nix/tests/files`.
- `README.md` and `CLAUDE.md`.
- `docs/adr/0028-…` and `docs/architecture/core.md`, in the commit that writes this change.
- The runner's code does not change, and neither does its file: every rendered file but its header line is the text it was, which `module_files.rs` reads.
- No flake input or tag moves. No crate, tool or tracked file type is added (core.7, core.8).
- Outside this repository: afterthought/swancloud passes deneb's accounts and drives as these options once it takes the flake (record 0028).

## Touches

```
nix/modules/**
nix/tests/**
README.md
CLAUDE.md
docs/adr/0028-a-hosts-configuration-names-its-accounts-each-account-its-drives-and-each-drive-its-repositories.md
docs/architecture/core.md
```
