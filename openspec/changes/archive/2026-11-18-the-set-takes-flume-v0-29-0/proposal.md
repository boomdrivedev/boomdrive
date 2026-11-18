# Proposal

## Why

The set pins Flume v0.26.0. Flume's newest release is v0.29.0, whose watch, unwatched command and wait settle only the places where something is pending (Flume's 208 and 225), the fix for a host's settle that froze it. A host that takes Flume through the set, deneb's among them, gets that fix only once the set takes v0.29.0, and a host that pins its own Flume to have it, afterthought/swancloud's, can drop that pin once it has.

v0.29.0 renames a label's share of its stream (Flume record 0063, accepted by the user on 2026-11-09). A label's `reserve` is now `owed`, and its `keep_free` is now `reserved`. Flume's module declares the new words, and Flume refuses a stream's starts where its file names an old one. The set gives each drive's labels to Flume's module as `reserve` today, so a host on the set at v0.29.0 would start nothing in its drives' streams. Flume's record names Boomdrive's drive option `owed` with it. Flume's module also reads a stream's `agents` from 1.

## What Changes

- **The pin.** `flake.nix` names Flume `v0.29.0`, and the lock moves its `flume` node alone (core.44, core.45).
- **A label's share is `owed`.** A drive's `stream.labels.<label>.owed`, `operation` 1 and `planning` 2 by default, is given Flume's module as the label's `owed`. The agents a label holds for itself alone stay Flume's own setting, now `reserved`, set in `services.flume.settings.streams.<stream>`.
- **The old word fails evaluation.** A drive's label that sets `reserve` fails evaluation, naming its option and `owed`, whether or not the runner is on, and gives Flume nothing.
- **A stream's agents from 1.** A drive's `stream.agents` is a whole number from 1, as Flume's option reads it.
- **The README** names a label's `owed`, the old word's failure, and a label's `reserved` as Flume's own.
- **Record 0028** is amended on 2026-11-18 with a label's share, the old word's failure and a stream's agents from 1.

**BREAKING**: a host whose `services.boomdrive.accounts` sets a drive's `stream.labels.<label>.reserve` fails evaluation at the switch that takes this change, and writes `owed` in its place in that switch. A host whose `services.flume.settings.streams.<stream>.labels.<label>` sets `reserve` or `keep_free` writes `owed` or `reserved`, which Flume v0.29.0 refuses otherwise.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `service-modules`: each drive gives Flume each label's `owed`, its stream's `agents` read from 1; a host's own `reserved` is kept beside the module's; and a drive's label that sets `reserve` fails evaluation, naming `owed`.

## Impact

- `flake.nix`, `flake.lock`: Flume at `v0.29.0`, taken unchanged through its own flake. Its flake and lock are as at v0.26.0, and its tree still pins herdr 0.9.5, the set's herdr.
- `nix/modules/options.nix`: a label's option `owed`; `reserve` declared hidden, null by default, for the failure; `stream.agents` from 1.
- `nix/modules/drives.nix`: each label given as `owed`, a label that sets `reserve` given nothing; the failure, outside the runner's `enable`.
- `nix/tests/drives.nix`: its cases read `owed` and `reserved`, and gain "A label's old word" and "A stream of no agents".
- `README.md`'s **Installing with nix**.
- `docs/adr/0028-…`: its amendment of 2026-11-18.
- The runner's code and its file do not change: the runner's floor stays Flume 0.21.0 (core.38), and its `flume settle --every` is the same command at v0.29.0 (core.13). No crate, tool or tracked file type is added (core.7, core.8). No rule changes.
- Outside this repository: a host on the set that sets a drive label's `reserve`, or Flume's own `reserve` or `keep_free`, writes the new words in the switch that takes this change.

## Touches

```
flake.nix
flake.lock
nix/modules/options.nix
nix/modules/drives.nix
nix/tests/drives.nix
README.md
docs/adr/0028-a-hosts-configuration-names-its-accounts-each-account-its-drives-and-each-drive-its-repositories.md
```
