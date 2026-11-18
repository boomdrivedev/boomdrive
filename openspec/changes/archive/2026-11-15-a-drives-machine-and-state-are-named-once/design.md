# Design

## Context

Record 0032 decides that a host names a drive's machine and its account's crew state repository once, in `services.boomdrive.accounts`. Boomdrive's module gives crew both through crew's own module options, and builds the runner's crew table from the same accounts. A table written out still wins whole. A host that also writes either in crew's settings fails its switch, naming `services.boomdrive.accounts`. No code carries a host from one shape to the other.

In this tree:

- `nix/modules/options.nix` declares an account as its `drives` alone, and a drive as its `machine`, `stream` and `repositories`. A drive's `machine` is compared with `services.crew.host`: a drive naming none, or naming that host, is this host's, and gives Flume its stream (`drives.nix`, record 0028's amendment of 2026-11-08).
- `options.nix` writes the runner's `[control_plane.crew]`, under crew with crew's module on and no table written, from `crew.settings.drives` whose `machine` is `services.crew.host`, in label order, each as `label` and `state` as `state_repository`. It fails where crew's module is off and no table is written, and where no drive of crew's is this host's.
- `nix/modules/accounts.nix` is "the one place that reads" the accounts. Its `drivesOf` lists each drive as `{ account, name, machine, stream, addresses }`, accounts and then drives in the order of their names. Its `hereOf host` keeps this host's drives.
- `nix/modules/kept.nix` and `sources.nix` already give crew's module values the set knows (core.57). Each platform module gives crew's kept repositories at the options' own priority, so a host's other folder fails evaluation.
- `nix/tests/settings.nix` writes crew's drives through `driveOf label machine state`, each with its `machine` and `state`. Its fixtures `crewModule`, `crewFleet`, `crewOrder` and `crewElsewhere` turn crew on with them in `tools.nix`, `darwin.nix`, `nixos.nix`, `kept.nix`, `sources.nix`, `apps.nix`, `cache.nix`, `companions.nix`, `claude-code.nix`, `boot.nix` and `lib.nix`'s `toolsOf`. None of their accounts names a crew state repository.
- `site/src/start/nixos/boomdrive.nix`, the quick start's NixOS drive, writes crew's drive `home` with its `machine` and `state`. Its account `me` names neither. `nix/tests/quick-start.nix` reads crew's drives' `state`, `machine` and `blueprints` from the evaluated system.
- The README's **Installing with nix** writes crew's drive `home` with `state` and `machine`, and says the module writes the runner's crew table from crew's drives.

crew `v0.59.0`, the lock's (`nix/modules/options.nix` of crew's tree):

- A drive's `state` and `machine` are each `types.unique`. Each one's message says it is "written once: where another module gives it, the host's configuration writes it nowhere else in services.crew.settings, and each place that writes it is named below".
- crew's spec, "Another module gives a drive its machine and its state repository": each is written once, by the host's settings or by another module that gives them under the drive's label. Written twice at one priority, the values equal or not, evaluation fails. "A drive given and not configured" fails too, as a drive that lacks a key crew requires.
- The module system's failure for a `types.unique` option lists each definition's place, its module's `_file`, under "Definition values".

## Goals / Non-Goals

**Goals:**

- Each account names its crew state repository, and each drive its machine, in `services.boomdrive.accounts` alone.
- With crew's module on, crew's drives take their `machine` and `state` from the accounts. A host that writes either in crew's settings fails evaluation, naming `services.boomdrive.accounts`.
- The runner's crew table is read from the accounts.
- Under crew, a host whose accounts and crew's drives do not agree fails evaluation, naming what to write where.

**Non-Goals:**

- crew's module. It already takes both from another module, and its unique check is crew's own.
- crew's `hosts`, `runs`, `review` and other keys, which stay the host's (core.57; record 0036, point 4).
- The runner's code, its file's keys and its crew implementation. The account's state repository is no key of the runner's file (core.5; record 0028's amendment of 2026-11-07, "Nothing else in an account or a drive"). The runner's crew table keeps `partitions` and `state_repository`.
- `boomdrive setup` on a host Nix does not manage, which writes crew's drive by `crew drive set` (record 0009 as amended on 2026-11-09 and 2026-11-11).
- A hosted control plane's accounts, which name no crew state repository.
- The quick start's words. `site/src/content/start/nixos-drive.md` still holds: the file still declares "the account `me`'s drive `home` keeping your repository, with crew's drive `home`, its host and its state repository". A rewording is the content kind's (site.5).

## Decisions

### 1. The account's `state_repository`

`options.nix`'s account submodule gains `state_repository`. It is `types.nullOr (types.strMatching "[A-Za-z0-9-]+/[A-Za-z0-9._-]+")`, `null` by default, the pattern the module already holds an address to. Its description: "The account's crew state repository, `<owner>/<name>`, whose branch `<drive>/main` holds each of its drives' plan and run record: under crew, named here once, and given to crew's drives and the runner's crew table; none where unset, and the account's drives give crew nothing." `fileOf` writes no key of it, so the runner's file is as it was.

Not taken:

- **`state`**, as record 0032's option 1 and record 0028's example write it, and as crew's file names a drive's. core.12's details rule out "a key such as `state` that names less than what it holds, a repository". The runner's own key for this value is `state_repository`, in its crew table, and setup takes it as `--state-repository`. So core.46's "one option per key, each named as the file names it" names it so too.
- **On each drive.** Record 0028 puts it on the account: "Under crew, an account names its crew state repository". A drive is a branch of its account's state.

### 2. The accounts give one list

`accounts.nix`'s `drivesOf` gives each drive its account's `state_repository` beside `machine` and `stream`. It reads the account as `{ drives, state_repository ? null, ... }`. A new `crewDrivesOf host drives` gives crew's drives as an attribute set by the drive's name. It holds each drive whose `state_repository` is set, as `{ machine; state; }`: `machine` is the drive's, or `host` where the drive names none, and `state` is its account's `state_repository`. `options.nix`'s table and the new module both read `drivesOf`, so the file stays "the one place that reads" the accounts.

### 3. crew is given each drive's machine and state, with crew's module on

A new module, `nix/modules/crew-drives.nix`, which both platform modules import beside `drives.nix`, `clones.nix` and `sources.nix`, gives `services.crew.settings.drives` the value of `crewDrivesOf config.services.crew.host (drivesOf config.services.boomdrive.accounts)`, under `lib.mkIf config.services.crew.enable`. It gives:

- every drive of every account that names its `state_repository`, this host's and others', as record 0032 has it: "each drive of every account, this host's and others'";
- no other key of a drive: crew's own keys stay the host's, named by the drive (record 0032);
- nothing of an account that names no `state_repository`: such an account is no account of crew's (record 0028), as a hosted control plane's are not, and its drives' machines, if given alone, would split a crew drive between two writers.

It gives them whether or not the runner is enabled, as `sources.nix` gives crew's source. The accounts are the host's one place for both facts, and crew reads its drives whether or not the runner runs.

A drive the module gives crew, which the host's crew settings do not configure, fails crew's own evaluation, as crew's spec has it ("A drive given and not configured"). The set judges nothing of crew's drives itself.

Not taken:

- **Only with the runner**, as Flume's streams are given. A host that turns the runner off would have to write both facts in crew's settings again. Turning it back on, it would then write them twice.
- **The accounts' drives given machine alone where the account names no state repository.** crew would need the drive's `state` from crew's settings, so one crew drive would have two writers.

### 4. At the options' own priority, placed at `services.boomdrive.accounts`

The module gives `machine` and `state` at the options' own priority, with no `lib.mkDefault`. A host that also writes a given drive's `machine` or `state` in `services.crew.settings` then fails evaluation by crew's unique check. That holds "A host that also writes a drive's machine or state in crew's settings fails its switch" (record 0032).

The module sets `_file = "services.boomdrive.accounts";`. The module system takes it as the place of every definition the module makes, so crew's failure lists it under "Definition values":

```
error: The option `services.crew.settings.drives.home.machine' is defined multiple times while it's expected to be unique.
A drive's machine is written once: where another module gives it, the host's configuration writes it nowhere else in services.crew.settings, and each place that writes it is named below.
Definition values:
- In `services.boomdrive.accounts': "host-a"
- In `/etc/nixos/configuration.nix': "host-a"
```

The module makes no definition but crew's drives, so no other message names that place. Its key, by which the module system takes it once, is still its path.

A host's `lib.mkForce` of a drive's `machine` or `state` wins over the module's, on purpose, as core.57 has it for crew's map.

Not taken:

- **`lib.mkDefault`**, under which a host's own value would win with no word, and both writers would stand.
- **An assertion of the set's over crew's settings.** crew's unique check already fails a second write. Two tools judging one fact is what the family rule in CLAUDE.md forbids. An assertion would also never run, since crew's check throws first.
- **The module's own path as the place.** It names a file in the store, where the fix is `services.boomdrive.accounts`.

### 5. The runner's crew table is read from the accounts, in their order

In `options.nix`, `written`, the table the module writes under crew with crew's module on and no table written, is:

```nix
{
  inherit (crew) host;
  partitions = map (drive: {
    label = drive.name;
    inherit (drive) state_repository;
  }) (lib.filter (drive: drive.state_repository != null) (hereOf crew.host drives));
}
```

`drives` is `options.nix`'s own `drivesOf cfg.accounts`. Its entries are this host's drives, those that name no machine or name `services.crew.host`, under an account that names its `state_repository`. They come in `drivesOf`'s order: accounts in the order of their names, each account's drives in the order of theirs, as the runner's file lists them. The runner's crew implementation reads each partition by its label, so nothing turns on the order (crew-control-plane spec). The table's `defaultText` and `description` say this.

A table written out wins whole, as before.

Not taken:

- **Label order across the accounts**, a sort that changes nothing the runner reads, in place of the one order the accounts already have.
- **crew's drives, as before.** They now hold the accounts' machines and states, so they would give the same table where crew's settings name no other drive. But the table would then read crew's settings for a fact of the accounts, which record 0032 has read "from the same accounts".

### 6. Under crew, both facts are in the accounts or nowhere

With the runner enabled, `control_plane.type = "crew"` and crew's module on, `options.nix` adds two assertions. They hold whether or not a table is written out:

- **An account with a drive and no state repository.** Each account whose `drives` is not empty and whose `state_repository` is `null` fails: "services.boomdrive.settings.control_plane.type is crew, and services.boomdrive.accounts.<account>.state_repository is not set: under crew, each account names its crew state repository, which the module gives crew's drives and the runner's crew table (records 0028 and 0032)." It lists each such account's option.
- **A drive of crew's that no account names.** Each name of `services.crew.settings.drives` that no drive of `services.boomdrive.accounts` has fails: "services.crew.settings.drives names <drives>, which no account of services.boomdrive.accounts names: under crew, each drive is named once, under its account in services.boomdrive.accounts.<account>.drives.<drive>, with its machine where it runs on another host, and the module gives crew its machine and its account's state_repository (record 0032)."

The failure where no drive is this host's keeps its condition. Its words read the accounts: "services.boomdrive.settings.control_plane.crew is written from the drives of services.boomdrive.accounts whose machine is services.crew.host, ${host}, or that name none, under an account that names its state_repository, and no drive is: they name ${machines}. Set services.crew.host to this host's machine, or give a drive of services.boomdrive.accounts this host as its machine (record 0032)." `machines` are the `machine`s the drives of accounts naming a state repository name, each once, in `drivesOf`'s order. Where no such drive is named at all, the words say "the accounts name no drive under a state repository" in place of "they name …". The failure where crew's module is off and no table is written keeps naming `settings.control_plane.crew` and `services.crew`. Its words say to turn crew's module on "with its host, its drives' own keys and its hosts".

Not taken:

- **Either left to stand.** A drive of this host whose account names no state repository gets a stream and no partition. A drive of crew's that no account names gets crew's agents and no stream or partition. These are the mismatches record 0032 ends.
- **The two failures with any control plane.** Under another control plane the accounts are that control plane's, and crew's settings are crew's own: "A hosted control plane beside crew" and "No control plane beside crew" stand as they are.
- **The two failures without crew's module.** Where crew's module is off, crew has no drives, and the table, if written out, holds the runner's own keys.

### 7. The tests write crew's keys apart from the accounts' facts

`nix/tests/settings.nix`:

- `driveOf label` gives crew's own keys of a drive alone: `blueprints`, `session`, `folder`, `repos`, `accounts`, `runs` and `review`, as now.
- `writtenDriveOf label machine state` is `driveOf label // { inherit machine state; }`. It is crew's drive for a system whose accounts give crew nothing, which writes both in crew's own settings. That is still allowed (Decision 3).
- `crewModule` keeps its meaning, `home` by `writtenDriveOf "home" "host-a" "example/state"`. Every system that turns crew on with it names no account's state repository, so the module gives crew nothing there, and those cases do not change. So do `crewOldRuns` and `crewOldReview`.
- `crew`'s account `own` names `state_repository = "example/state"`, and `crewUnwritten` with it. The file it renders, `nix/tests/files/crew.toml`, does not change: it holds no account's state repository.
- `crewKeys` is `{ inherit hosts; drives.home = driveOf "home"; }`, crew's keys for `crew`'s account's drive.
- `crewAccounts` gives the accounts `own`, `state_repository = "example/state"`, with the drives `garden` (`machine = "host-a"`) and `home` (no machine), and `work`, `state_repository = "example/work"`, with the drives `away` (`machine = "host-b"`) and `field` (no machine). `crewDrives` is `hosts` with crew's keys for each of the four.
- `elsewhere` is the runner's value under crew with no table, whose one account `work`, `state_repository = "example/away"`, has the one drive `away`, `machine = "host-b"`. `crewElsewhere` is `hosts` with crew's keys for `away`.
- `stateless` is the runner's value under crew with no table, whose account `own` keeps `home` and names no state repository. It goes with `crewModule`.
- `crewStray` is `crewKeys` with a second drive, `stray`, by `writtenDriveOf "stray" "host-a" "example/state"`.
- `crewFleet` and `crewOrder` are removed with their last use.

Each case's name is its scenario's:

- `darwin.nix` and `nixos.nix`: "crew's drives on this host" becomes "The accounts' drives on this host", the system `crewWith settings.crewUnwritten settings.crewKeys`. "A table written out wins" takes `settings.crewKeys` in place of `crewFleet`.
- `tools.nix`: "crew's drives in label order" becomes "The accounts' drives in their order", the system of `crewAccounts` and `crewDrives` under crew with no table. "No drive of crew's on this host" becomes "No drive of the accounts on this host", the system of `elsewhere` and `crewElsewhere`. "An account under crew with no state repository" and "A drive of crew's no account names" are new.
- `crew-drives.nix`, new: the giving module's cases on each platform, its `loads` the one `tools.nix` has. "Each drive's machine and state repository, given to crew" reads crew's drives from the system of `crewAccounts` and `crewDrives`, each holding `machine` and `state` alone, and holds the module's definition placed at `services.boomdrive.accounts` among `options.services.crew.settings.definitionsWithLocations`. "Given without the runner" reads the same of the platform's `default` module with the runner off. "An account naming no state repository gives crew nothing" is `crewModule` beside the account `own` keeping `agentplot/flume` with no state repository. "A drive crew's settings do not configure" is `crewAccounts` beside `crewKeys`. "A machine written twice" and "A state written twice" are `crewAccounts` beside `crewDrives` with `home`'s `machine` `host-a`, or its `state` `example/state`, added. Each holds crew's settings to fail, by `loads`, and the module's definition placed at `services.boomdrive.accounts`.
- `file.nix`: "A state repository of no address" evaluates `options.nix` alone with `accounts.own.state_repository = "example"`, and holds the accounts to fail, by `builtins.tryEval` of their `deepSeq`.

Every name is core.41's.

### 8. The quick start's NixOS drive

`site/src/start/nixos/boomdrive.nix` writes `services.boomdrive.accounts.me` as `{ state_repository = "your-github-name/drive-state"; drives.home.repositories.your-github-name = [ "your-repository" ]; }`. crew's drive `home` keeps every key it has now but `state` and `machine`. Its header comment says the set gives crew's drive its machine and state from the account. The system it makes gives crew's `home` the same `machine`, `your-host`, and `state`, `your-github-name/drive-state`, as before, and the runner's file and crew table are the same.

`nix/tests/quick-start.nix`'s case "quick start: NixOS's drive" holds two more things of the file as it stands, through its `written`. The account `me` names `your-github-name/drive-state` as its `state_repository`. crew's drive `home` writes neither `machine` nor `state`. Its reading of the evaluated system's crew drives is unchanged, so it holds what the set gives.

The step's words stay as they are (Non-Goals). site.8 holds: the file is the one the page shows, and its test runs it as it stands. "The quick start" of `site/docs/site-design.md` has a step change "only with its test passing", which it does in the task that changes it.

### 9. The README

**Installing with nix**'s nix block writes `accounts.own = { state_repository = "example/state"; drives.home.repositories.boomdrivedev = [ "flume" "sluice" ]; };`. Its crew drive `home` drops `state` and `machine`. Its paragraph before the block says:

- the module writes `[control_plane.crew]` from the accounts' drives of this host under an account that names its `state_repository`, in the accounts' order, each by its name as `label` with its account's `state_repository`, and a table written out wins;
- with crew's module on, each drive of an account that names its `state_repository` gives crew's drive of the same name its `machine`, `services.crew.host` where the drive names none, and its account's `state_repository` as its `state`;
- a host writes neither in `services.crew.settings`, and one that does fails evaluation, crew's failure naming `services.boomdrive.accounts`;
- under crew, an account with a drive and no `state_repository`, or a drive of crew's that no account names, fails evaluation;
- crew's `hosts`, `runs`, `review` and other keys of a drive are still the host's.

The paragraph on a drive's `machine`, after the stream's block, says the module gives it to crew's drive of that name too.

`tools.nix`'s "The README's crew table" holds that no nix block names `control_plane.crew` or `partitions`, in place of `state_repository`, which the accounts now name. "The README's machine and state" is new. In the nix block that names `services.crew`, it finds `state_repository = "example/state"` before `services.crew = {`, and neither `state =` nor `machine =` after it. It finds `services.boomdrive.accounts.<account>.state_repository` in the README's words.

### 10. The records and rules

Record 0032 gains an amendment of 2026-11-15 with Decisions 1, 3, 4, 5 and 6, each a choice of this stage's, cheap to change, with the path not taken. core.57 names crew's drives' machine and state among what the set gives crew. Its details rule out a host's second write and allow an account naming no state repository. core.18's details read the runner's crew table from the accounts. core.46's details have the accounts give crew each drive's machine and state. All are written in this change's own commit, and no task changes them.

### Rules relied on

- **core.57**: a fact the set knows from the host's accounts is given through the tool's own module option, and a host writes it nowhere. Amended here to name crew's drives' machine and state.
- **core.46, core.18**: the Boomdrive module sets crew's module options and writes no file of crew's. crew's module renders the teams file.
- **core.12**: a key names what it holds, so the account's key is `state_repository` (Decision 1).
- **core.5**: the runner's file names its accounts, drives and repositories and no state repository, so it does not change.
- **core.19**: a runner serves the one control plane its `type` names, so under crew every account the runner serves is crew's (Decision 6).
- **core.2**: no code outside the crew implementation names crew's records. The modules give crew's module options and write the runner's file. The runner's crew implementation reads `[control_plane.crew]` as before.
- **core.41**: neutral names in the tests, the README and the quick start.
- **core.7, core.8**: no tool, crate or file type is added. The new `.nix` files are nixfmt's.
- **core.48**: the build's commit that moves the table and adds the two failures is a breaking change, `feat(nix)!:`, whose subject names the host's move.
- **site.8**: the quick start's file changes with its test, which runs it as it stands.

### Rules amended

core.57 (rule and details), core.18's details and core.46's details, with record 0032's amendment of 2026-11-15.

### Rules departed from

None. Decision 1 departs from the key `state` that record 0032's option 1 and record 0028's example write. The amendment records it, on core.12.

## Dependencies

- crew `v0.59.0`, the lock's: a drive's `machine` and `state` written once, by the host's settings or another module (crew's spec, "Another module gives a drive its machine and its state repository").
- Records 0028 and 0032.

## What this change asks of others

afterthought/swancloud moves its hosts in the switch that takes a set carrying this change. Each account names its `state_repository` in `services.boomdrive.accounts`. Each drive of crew's is named under its account there, with its `machine` where it runs on another host. Each such drive's `machine` and `state` leave `services.crew.settings`. Record 0032 has agents make that move, and no code of the set's.

No other repository changes.

## Risks / Trade-offs

- **A host's switch fails until it moves.**
  - Under crew with crew's module on, an account with drives and no `state_repository` fails, naming the option.
  - A drive of crew's that no account names fails, naming it.
  - Once an account names its `state_repository`, a drive's `machine` or `state` still in crew's settings fails by crew's check, naming `services.boomdrive.accounts`.
  - Each failure is at evaluation, before anything on the host changes.
- **Every account's drive goes to crew.** An account that names a state repository gives crew all its drives, another machine's among them, and crew's settings must give each its own keys. That is crew's teams file's own rule, which lists every drive (crew's "A drive given and not configured").
- **The place `services.boomdrive.accounts` is a word, not a file.** A person reading crew's failure finds the option, which is where the fix is. The module's file still shows in a trace.
- **A host's `lib.mkForce`** of a drive's machine or state passes, on purpose (core.57).

## Migration Plan

Each host takes the change at its next switch, with its accounts and crew's settings moved in the same switch (What this change asks of others). crew's module then renders the same teams file, and the runner's file and crew table are the same. To roll back, the host returns both the flake's revision and its settings.

## Open Questions

None.

## Task notes

**1.1** In `options.nix`, `state_repository` sits in `account`'s `options` beside `drives`. `account` becomes a submodule whose `options` holds both. `accounts.nix`'s header comment says each drive carries its account's `state_repository`. `drivesOf` reads `{ drives, state_repository ? null, ... }` and inherits it into each drive. `file.nix`'s case "A state repository of no address" uses its `evaluated` with `{ accounts.own.state_repository = "example"; }`. It holds `!(builtins.tryEval (builtins.deepSeq (evaluated given).services.boomdrive.accounts true)).success`.

**1.2** `crewDrivesOf` is `host: drives: lib.listToAttrs (map (drive: lib.nameValuePair drive.name { machine = if drive.machine == null then host else drive.machine; state = drive.state_repository; }) (lib.filter (drive: drive.state_repository != null) drives))`. `crew-drives.nix` is:

```nix
{ config, lib, ... }:
let
  inherit (import ./accounts.nix { inherit lib; }) drivesOf crewDrivesOf;
  crew = config.services.crew;
in
{
  _file = "services.boomdrive.accounts";
  config = lib.mkIf crew.enable {
    services.crew.settings.drives = crewDrivesOf crew.host (drivesOf config.services.boomdrive.accounts);
  };
}
```

Its header comment cites record 0032 and crew's spec, and says why `_file` names the option (Decision 4). `darwin.nix` and `nixos.nix` import `./crew-drives.nix` after `./sources.nix`, and each header comment names it beside `sources.nix`. `settings.nix` gains `writtenDriveOf`, `crewKeys`, `crewAccounts` and `crewDrives`, and `driveOf` loses its `machine` and `state`. Every fixture that used `driveOf label machine state` uses `writtenDriveOf` (Decision 7), so every case that stood still stands. `nix/tests/crew-drives.nix` follows `sources.nix`'s layout: `platforms` with `system` and `off`, and `onEach`. Its placed check is `lib.any (definition: definition.file == "services.boomdrive.accounts") evaluated.options.services.crew.settings.definitionsWithLocations`, which reads no definition's value. `default.nix`'s `test` takes it after `./sources.nix`.

**1.3** In `quick-start.nix`, `written.services.boomdrive.accounts.me.state_repository` is `"your-github-name/drive-state"`. Neither `written.services.crew.settings.drives.home ? machine` nor `? state` holds. The file's header comment says "the account me, with its state repository, whose drive home is kept under crew, crew's drive home given its machine and state by the set".

**2.1** `written`, `machines` and the "no drive" failure's words as Decisions 5 and 6. `machines` reads `drives` filtered by `state_repository != null`, read only where crew's module is on. The `machine` option's description says: "The host the drive's work runs on, as `hostname -s` names it; this host, `services.crew.host`, where unset. A drive of another host gives Flume nothing, and this host keeps its repositories to read. With crew's module on, crew's drive of this name is given it as its `machine` (record 0032)." `options.nix`'s header comment says the crew table is written from the accounts' drives on this host under an account that names its state repository, in the accounts' order. The fixtures and the renamed cases are Decision 7's. In `tools.nix`, the case function `crewOrder`, renamed with its case, has its `ordered` become the partitions `garden` and `home` at `example/state`, then `field` at `example/work`. This task's commit is the breaking one: `feat(nix)!: the runner's crew table is read from the accounts, each account naming its crew state repository, so a host under crew names each drive's machine and state there and drops them from crew's settings`.

**2.2** The two assertions sit after the "no drive" one. `stateless` is the names of `cfg.accounts` whose `drives` is not `{ }` and whose `state_repository` is `null`. `unnamed` is `lib.subtractLists (map (drive: drive.name) drives) (lib.attrNames crew.settings.drives)`. Each condition reads `crewPlane && crew.enable` before the list, so crew's settings are read only where crew's module is on. The cases use `failsNaming`:

- "An account under crew with no state repository" names `services.boomdrive.accounts.own.state_repository`.
- "A drive of crew's no account names" names `stray` and `services.boomdrive.accounts`.

**3.1** The README as Decision 9. "The README's machine and state" splits the block that names `services.crew` at `services.crew = {`.

**3.2** statix refuses a key repeated in one attribute set, so each fixture's `drives` is one attribute set. deadnix refuses a binding nobody reads, so `crewFleet` and `crewOrder` go with their last use.
