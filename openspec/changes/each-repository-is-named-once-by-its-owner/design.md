# Design

## Context

Record 0023 (accepted 2026-11-06) has a host name each repository once, under its owner, and the Boomdrive module write each address into the runner's file and give each address its stream in Flume's module, with no folder written. Record 0028 (accepted 2026-11-07) moves the owner under its account's drive, and the drive gives the stream:

```nix
services.boomdrive.accounts.personal.drives.agentplot.repositories.agentplot = [ "flume" "sluice" ];
```

- A repository belongs to one drive. A drive's name is given once on the host, across its accounts.
- Each drive has its stream, named after the drive unless the drive names another. Its room, priority and each label's reserve are the drive's `stream` keys, with the module's defaults, deneb's: four agents, a reserve of one for operation and two for planning.
- A drive whose `machine` names another computer gets no stream and no agents on this one, and this host keeps its repositories to read.
- Record 0028's amendment of 2026-11-07, the keys of the runner's file: each drive's `repositories` is a list of addresses in the order kept, and the module turns its owners' names into addresses. The drive's stream and machine, and the account's state repository, are the modules' options and never keys of the runner's file (core.5).

The flake today, after the unit `the-flake-configures-each-tool-by-its-own-module`:

- `nix/modules/options.nix` declares `services.boomdrive.settings`, the runner's file, whose `accounts` is typed and always written, each drive's `repositories` a list of addresses.
- `darwin.nix` and `nixos.nix` render `settings` with `toml.nix`.
- `tools.nix` sets Flume's, sluice's and crew's module options: each `package` the set's by default, and Flume's and sluice's `enable` with the runner's.
- Nothing gives Flume a stream or a repository's table.

Flume's module at the set's tag, v0.18.0, declares `settings.streams.<stream>`, with `agents`, `priority` and `labels.<label>.reserve` and `keep_free` among its keys, and `settings.repos."<repository>".stream`, a repository named by its address `<owner>/<name>` (flume record 0055). Each key is null unless set, and left out of Flume's file. Its tables of named entries merge across modules. A label's `keep_free` is zero where Flume's file gives none (flume record 0033).

crew's module at its tag, v0.44.8, declares `services.crew.host`, "this host, as `hostname -s` names it and the partitions' `machine` names a host", with no default.

This change is the bolt's unit `each-repository-is-named-once-by-its-owner`. Its later units bring herdr and Ghostty, and give the runner its crew table from crew's partitions.

## Goals / Non-Goals

**Goals:**

- A host names each repository once, under its account's drive and its owner, and writes no address and no folder.
- The module writes the runner's `accounts` from it.
- Each drive of this host gives Flume its stream, with the module's defaults, and each of its addresses that stream.
- A drive of another machine gives Flume nothing, and its repositories are still kept.

**Non-Goals:**

- An account's `state`, its crew state repository, and the runner's crew table. The bolt's unit `crews-partitions-give-the-runner-its-crew-table` brings them. Until it does, an account holding `state` fails evaluation, as any key the module does not declare.
- Any key of a stream but its room, priority and each label's reserve. Every other key is Flume's own option.
- Flume's kinds, pools, accounts and host settings, which the host sets in `services.flume.settings`.
- Any change to the runner's code, its file's keys, its local layer, or Flume's.
- A flake input or tag moved.

## Decisions

### 1. `services.boomdrive.accounts` names each drive's repositories by owner

`options.nix` declares `services.boomdrive.accounts`, beside `settings`:

```nix
services.boomdrive.accounts.<account>.drives.<drive> = {
  machine = null;                  # crew's name of the host the drive's work runs on; null is this host
  stream = {
    name = "<drive>";              # the drive's own name
    agents = 4;
    priority = null;               # Flume's own where null
    labels.<label>.reserve = <n>;  # operation 1 and planning 2 by default
  };
  repositories.<owner> = [ <names> ];
};
```

- An account holds `drives` alone, and a drive these three keys alone, so any other key fails evaluation, naming its option.
- `repositories.<owner>` is a list of names, each typed `[A-Za-z0-9._-]+`. The address `<owner>/<name>` is checked against the runner's form, `[A-Za-z0-9-]+/[A-Za-z0-9._-]+`, by an assertion naming it, since an attribute set's names take no type.
- `machine` takes crew's form of a host's name, `[a-z0-9][a-z0-9-]*`, as `services.crew.host` does.

The module writes the runner's `accounts` from it: each drive's `repositories` the drive's addresses, owner by owner in the order of the owners' names, each owner's names in the order given. Nix keeps no order of an attribute set's names, so the owners' names give the order. A drive's workspace lists its folders in the same order (core.43).

`nix/modules/accounts.nix`, added, is the one place that turns `accounts` into what the others read, so both platforms render one text:

- `drivesOf accounts`: each drive as `{ account, name, machine, stream, addresses }`, in the accounts' and drives' name order;
- `fileOf cfg`: the table the runner's file is rendered from, `cfg.settings` with `accounts` from `cfg.accounts`, every account's `drives` and every drive's `repositories` always written.

`darwin.nix` and `nixos.nix` render `fileOf cfg` in place of `cfg.settings`. `toml.nix`'s header line names both options: `# Written by the boomdrive flake's module from services.boomdrive.accounts and services.boomdrive.settings; a change goes there.`

Alternatives:

- **A drive's repositories as a list of addresses**, as `settings.accounts` holds them. Not taken: records 0023 and 0028 name each repository by its owner, and the user asked for "One block by owner".
- **An option per owner with its own order key.** Not taken: the order by owner name is stable, and a person who wants another order has none to keep.

### 2. The accounts are named in one place

`settings` no longer declares `accounts`. A host's own `settings.accounts`, which `settings`' free-form type would take, fails evaluation, naming `services.boomdrive.accounts`. The flat `settings.repositories` of before fails as now, its words naming `services.boomdrive.accounts.<account>.drives.<drive>.repositories.<owner>`.

Alternatives:

- **Both, merged.** `settings.accounts` kept beside `accounts`, the two concatenated. Not taken: a repository could be named in two places, and one named only in `settings` would get no stream in Flume.
- **`settings.accounts` read-only.** Not taken: nix's own refusal of a second definition names no replacement.

### 3. A drive's stream: its keys and their defaults

A drive's `stream` holds `name`, `agents`, `priority` and `labels.<label>.reserve`, the keys record 0028 names:

| key | default | given to Flume as |
|---|---|---|
| `name` | the drive's name | the stream's name in `streams` and each `repos` rule |
| `agents` | 4 | `streams.<stream>.agents` |
| `priority` | null | `streams.<stream>.priority`, left out where null, so Flume's 0 holds |
| `labels.<label>.reserve` | `operation` 1, `planning` 2 | `streams.<stream>.labels.<label>.reserve` |

The stream's submodule sets each default label's `reserve` by `lib.mkDefault` in its `config`, so a drive that sets `labels.review.reserve` keeps `operation` and `planning`, and one that sets `labels.planning.reserve = 1` changes that alone. A label's `reserve` has no default of its own.

Every other key of a stream, a label's `keep_free` among them, is Flume's own option, `services.flume.settings.streams.<stream>`, and merges with what the module gives (Decision 4). deneb's `keep_free = 0` is Flume's own default, so the module gives none.

Flume refuses a stream whose reserves pass its bound (flume record 0033). The module does not check it: Flume owns the rule and refuses at a start, naming the stream.

Alternatives:

- **A drive holding every key of Flume's stream.** Not taken: each would be a second option for one of Flume's keys, which core.46 has written by one option.
- **`priority` given a default of the module's.** Not taken: record 0028's defaults are deneb's room and reserves, and deneb's priorities differ by drive.
- **A label's `keep_free` in the drive.** Not taken: record 0028 names a label's reserve alone, and Flume's own option holds it.

### 4. What the module gives Flume

`nix/modules/drives.nix`, added, imported by `darwin.nix` and `nixos.nix` beside `tools.nix`, gives Flume's options under `services.boomdrive.enable`, for each drive of this host (Decision 5):

- `services.flume.settings.streams.<stream>`: `agents`, `priority` and each label's `reserve`;
- `services.flume.settings.repos."<owner>/<name>".stream`, the drive's stream, for each address the drive keeps.

Each value is given by `lib.mkDefault`, as `tools.nix` gives each tool's package, so a host's own value in `services.flume.settings` wins, and a key the module does not give, set there, sits beside the module's. Where no drive of this host is named, the module gives Flume no `streams` and no `repos` at all, so Flume's file is the one it renders of the host's own settings. Each address is Flume's key as written, and no folder is given (flume record 0055).

The module writes no file of Flume's: Flume's own module renders and links its file from its `settings` (core.18, core.46).

Alternatives:

- **The module's values at the normal priority.** Not taken: a host's own value for the same key would conflict, and nix would refuse both.
- **The Boomdrive module writing Flume's `[repos]` itself.** Not taken: core.18 has a tool's file written by its own module alone.
- **Giving Flume the drives whether or not the runner is enabled.** Not taken: the module's other values are given under `enable`, and a host with the runner off keeps Flume's file as its own settings make it.

### 5. A drive's machine, against crew's host

A drive's `machine` names the host its work runs on, as crew's partitions' `machine` names one. A drive whose `machine` is null or equals `services.crew.host` is a drive of this host. One whose `machine` names another host gives Flume nothing, and its repositories are still written in the runner's file, so this host keeps them to read (record 0028).

Where a drive names a `machine` and `options.services.crew.host.isDefined` is false, evaluation fails, naming `services.crew.host` and the drive's `accounts.<account>.drives.<drive>.machine`.

Alternatives:

- **`networking.hostName`.** Not taken: NixOS gives it `nixos` by default and nix-darwin leaves it unset, so a drive could lose its stream with no word. crew's `host` has no default, names the host as crew's machines do, and is the one the bolt's crew unit reads.
- **An option of the runner's own for this host's name.** Not taken: a second option for what `services.crew.host` names.

### 6. Each name given once

Evaluation fails, naming what is given twice:

- in `options.nix`: a drive's name under two accounts, naming the drive and the accounts; an address kept by two drives or twice by one, naming the address and the drives;
- in `drives.nix`: a stream given by two drives of this host, naming the stream and the drives.

The runner refuses the first two at its start (record 0028's amendment of 2026-11-07). Checking them at evaluation fails the switch, before a file the runner refuses is linked, and before two drives' values meet in one of Flume's options, where nix would refuse them with no word of the drives.

### 7. The tests

Every case is evaluated under `nix/tests` against no store, by `nix:test` (record 0020's Decision 14), with core.41's names: the accounts `own`, `home` and `work`, the drives `home` and `garden`, the owners `agentplot` and `example`, and the hosts `host-a` and `host-b`.

- **`settings.nix`**: `crew` and `hosted` each become `{ accounts; settings; }`, the value of `services.boomdrive` less `enable`, the accounts by owner. `empty` stays `{ }`.
- **`file.nix`** evaluates `options.nix` with `services.boomdrive` set from them, and renders `fileOf`. It holds the modified scenarios but "The runner reads each file", and "Owners turned into addresses", "An address of no repository", "A drive's name under two accounts" and "A repository in two drives".
- **`darwin.nix` and `nixos.nix`** set `services.boomdrive = settings.crew` and `settings.hosted` where they set `settings` now.
- **`drives.nix`**, added to `default.nix`'s `test`, holds the stream, repository, machine, host's-own and one-stream scenarios, each on nix-darwin and on NixOS unless it names a platform, as `tools.nix`'s `onEach` runs them.
- **`cache.nix`** gains "The README's accounts" beside "The README's words".
- **`crates/boomdrive/tests/module_files.rs`** is unchanged: each file under `nix/tests/files` changes in its header line alone, a comment the runner does not read.

### Rules relied on

- core.5: the runner's file names accounts, drives and repositories, and no stream; a drive's stream in the flake is given to Flume's module and never to the runner's file (Decisions 1 and 4).
- core.18: the Boomdrive module sets Flume's module options and writes no file of Flume's (Decision 4).
- core.46: each tool's module writes its own file from its options, and the Boomdrive module sets the others' options. Its details are amended (below).
- core.44 and core.45: Flume at the set's tag, v0.18.0, names a repository by its address, so no tag moves.
- core.43: a drive's workspace lists its folders in its repositories' order, which Decision 1 gives.
- core.12: a drive's `stream` names Flume's stream, and its `machine` crew's host, each as its tool means it.
- core.17: the runner's local layer is untouched. The managed layer is the one nix writes.
- core.4: no option this change adds holds a credential.
- core.13: the module starts nothing.
- core.7 and core.8: no crate, tool or tracked file type is added.
- core.41: neutral names in every case.

### Rules amended

core.46's details, in the commit that writes this change, with record 0028's amendment of 2026-11-08: not ruled out, the runner's `accounts` written from `services.boomdrive.accounts`, which names each drive's repositories by owner and gives the drive's stream and machine to Flume's module, never to the runner's file.

### Rules departed from

None.

## Dependencies

The bolt's unit `the-flake-configures-each-tool-by-its-own-module` is this branch's base, with Flume's module at v0.18.0 and crew's at v0.44.8. No release of another tool is needed.

## What this change asks of others

- **afterthought/swancloud**, once it takes the flake, passes deneb's accounts and drives as `services.boomdrive.accounts`, every drive's stream as its keys, the rest of each stream in `services.flume.settings.streams`, and drops the `[repos]` rules it writes into Flume's file by folder (record 0028). An account's `state` waits for the bolt's crew unit.

## Risks / Trade-offs

- **A drive's room below its reserves.** A drive that sets `agents` under 3 and keeps the default reserves gets a stream Flume refuses at a start, naming it. The README says a drive's reserves are within its room.
- **A drive's repositories reordered.** A host that wrote `settings.accounts` in an order of its own gets its owners in name order, and its drive's workspace's folders with them.
- **crew's host for a host without crew.** A host whose drive names a machine sets `services.crew.host` whether or not crew runs there.

## Migration Plan

A host that writes `settings.accounts` moves each drive to `services.boomdrive.accounts`, each address `<owner>/<name>` written as `repositories.<owner> = [ "<name>" ]`. A host that writes `[repos]` rules or streams in Flume's settings for its drives drops them, or keeps only what differs from the drive's. Nothing changes for a host until it moves its Boomdrive input.

## Open Questions

None.

## Task notes

**1.1** `nix/modules/accounts.nix`, `{ lib }:`, with a header comment in `toml.nix`'s manner, gives `drivesOf` and `fileOf` (Decision 1). In `options.nix`:

- `accounts` is declared beside `settings`, its submodules as Decision 1's sketch, the drive's submodule a function of its `name`, so `stream.name` defaults to it.
- `settings`' `accounts` option goes, and the `drive` and `account` types become the new option's.
- The header comment says `accounts` names each drive's repositories by owner and the module writes the runner's `accounts` from it.

`darwin.nix` and `nixos.nix` render `(import ./accounts.nix { inherit lib; }).fileOf cfg`. In `toml.nix`, the header line is Decision 1's, and each file under `nix/tests/files` takes it as its first line. `settings.nix`, `file.nix`, `darwin.nix` and `nixos.nix` as Decision 7 has them. `file.nix`'s `rendered` renders `fileOf` of the evaluated `services.boomdrive`. "Owners turned into addresses" renders the drive of the scenario and holds its `[accounts.own.drives.home]` lines.

**1.2** In `options.nix`'s assertions:

- `!(cfg.settings ? accounts)`, its message naming `settings.accounts` and `services.boomdrive.accounts`;
- the flat list's message naming `services.boomdrive.accounts.<account>.drives.<drive>.repositories.<owner>`;
- each address of `drivesOf cfg.accounts` matching `[A-Za-z0-9-]+/[A-Za-z0-9._-]+`, the message naming those that do not.

In `file.nix`:

- "A stream in a drive's settings" keeps its `settings.accounts.own.drives.home.stream`, and now expects an assertion's failure naming `settings.accounts` and `services.boomdrive.accounts`, where it expected the render to fail.
- "A key a drive does not hold" sets `accounts.own.drives.home.pool` and expects the render's `tryEval` to fail.

**1.3** In `options.nix`'s assertions, from `drivesOf cfg.accounts`: each drive name under one account, and each address kept once, each message naming as Decision 6 has it.

**1.4** The runner's test reads each file of `nix/tests/files`, whose header line alone changed. A failure there is a change in a file's settings, which 1.1 put back.

**2.1** `nix/modules/drives.nix`, a plain module taking no `packages`, imported as `./drives.nix` by `darwin.nix` and `nixos.nix` beside `tools.nix`. Under `lib.mkIf cfg.enable`, with `here` the drives of `drivesOf cfg.accounts` whose `machine` is null (2.4 adds the host's), it gives `services.flume.settings.streams` a `lib.mkMerge` of one attribute set for each drive of `here`, every leaf by `lib.mkDefault`, wrapped in `lib.mkIf (here != [ ])`. Its header comment cites record 0028 and core.18. Each attribute set gives its keys under one `services.flume.settings = { … };`, since statix refuses a key repeated.

In `nix/tests/drives.nix`, Flume's values are read from `config.services.flume.settings`, whose keys the drive does not give read null. "A drive's stream changes nothing of the runner's" compares two systems' rendered `fileOf` text and, on nix-darwin, `darwin.nix`'s `plistOf`. "With the runner off" evaluates `t.darwinOf` and `t.nixosOf` with the platform's `default` module and the accounts, `services.boomdrive.enable` unset, and holds `services.flume.settings.streams` and `.repos` null.

**2.2** `drives.nix` gives `services.flume.settings.repos`, `lib.genAttrs` of each drive's `addresses` to `{ stream = lib.mkDefault <stream>; }`, merged and guarded as 2.1's streams.

**2.3** No code beyond 2.1's and 2.2's `lib.mkDefault`. The case sets Flume's values at the normal priority and checks Flume's settings and `t.failures`.

**2.4** `here` keeps a drive whose `machine` is null, or whose `machine` equals `config.services.crew.host` where `options.services.crew.host.isDefined`. An assertion fails where a drive's `machine` is set and the host is not defined, as Decision 5 words it. The cases set `services.crew.host` alone, crew's module left off: the host is read whether or not crew is on.

**2.5** An assertion in `drives.nix`: each stream name of `here` given by one drive, naming the stream and its drives.

**2.6** The new and changed nix files keep to statix and deadnix: no attribute set's key repeated, as `services.flume.settings.streams` and `services.flume.settings.repos` would be beside each other, and no binding unused.

**3.1** README, **Installing with nix**:

- The host names its accounts in `services.boomdrive.accounts` and writes the rest of the runner's file as `services.boomdrive.settings`.
- The example's `settings.accounts` becomes `accounts.own.drives.home.repositories.agentplot = [ "flume" "sluice" ];`.
- A paragraph says each drive's repositories are named by owner, the drive's stream is given to Flume's module with its defaults and its keys, a drive of another machine gives Flume nothing and compares its `machine` with `services.crew.host`, and any other key of a stream is set in `services.flume.settings.streams.<stream>`. It says a drive's reserves are within its room.
- `settings.accounts is always written` reads that `accounts` is always written, and that `settings.accounts` fails evaluation as `settings.repositories` does.

`cache.nix`'s "The README's accounts" reads the section as "The README's words" does.

**3.2** CLAUDE.md's paragraph on the flake: "as nix-darwin and NixOS modules, whose `accounts` and `settings` are the runner's file, each drive's stream given to Flume's module".
