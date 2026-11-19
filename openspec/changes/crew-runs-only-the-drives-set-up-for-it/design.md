# Design

## Context

Record 0032 as amended on 2026-11-18 decides it: "crew runs a drive only where the host sets it up in crew". A drive is crew's when the host's crew settings set up its main level: its own keys under `services.crew.settings.drives.<drive>`, its session, roles and blueprints. The set gives such a drive its `machine` and its account's `state_repository`. It gives a drive the host does not set up in crew nothing: "crew has no drive of that name, and the drive is a drive like any other". Under crew, "an account with a drive the host sets up in crew and no `state_repository` fails evaluation", and "an account with no such drive needs none". See proposal.md, Why.

In this tree, from the change a-drives-machine-and-state-are-named-once:

- `nix/modules/crew-drives.nix` defines `services.crew.settings.drives` as `crewDrivesOf crew.host (drivesOf accounts)` under `lib.mkIf crew.enable`, with `_file = "services.boomdrive.accounts"`. So every drive under an account that names its `state_repository` becomes a name of crew's drives.
- `nix/modules/accounts.nix`'s `crewDrivesOf host drives` gives `{ machine, state }` by the drive's name, for each drive whose account names its `state_repository`.
- `nix/modules/options.nix`:
  - `written`, the runner's crew table, holds each drive of this host (`hereOf crew.host drives`) under an account that names its `state_repository`.
  - `crewMachines` names the machines of the drives under such accounts, for the "no drive" failure.
  - `stateless` lists each account with `drives != { }` and no `state_repository`.
  - `unnamed` lists the drives of crew's that no account names.
- `nix/tests/crew-drives.nix` reads what crew is given. Its `placed` finds the module's definition among `options.services.crew.settings.definitionsWithLocations`. Its case "A drive crew's settings do not configure" holds crew's teams file failing, since a drive given and not set up lacks crew's own keys.
- `nix/tests/tools.nix` holds the table's and the failures' cases, `accountsOrder`, `elsewhere`, `stateless` and `stray`, with the fixtures of `nix/tests/settings.nix`.
- `nix/tests/tools.nix`'s case "Each tool's options are its own flake's" reads each option of `services.<tool>` by its `declarationPositions`, which name the file that declares it whatever that module's `_file`, and fails one declared in a file outside its tool's input. It carries the service-modules requirement "The modules bring each tool's own module", which carries record 0023's each tool by its own module.

crew `v0.68.0`, the lock's (`nix/modules/options.nix` of crew's tree):

- `settings.drives` is `types.attrsOf drive`. A drive's `machine` and `state` are each `types.unique`, written once, "by the host's settings or by another module that gives it by the drive's label".
- A drive's `blueprints`, `session`, `folder`, `accounts`, `review` and `runs` have no default. So a drive crew is given and the host does not set up fails crew's evaluation.

The nixpkgs the lock names gives each submodule option a `valueMeta`, whose `configuration.options` holds the submodule's options with their `definitionsWithLocations`. For an `attrsOf` option, `valueMeta.attrs.<name>` holds each element's.

## Goals / Non-Goals

**Goals:**

- crew is given a drive's machine and state only where the host's crew settings set up the drive. The set adds no name to crew's drives.
- The runner's crew table, the "no drive" failure and the state repository's failure read the drives crew's settings set up.
- A drive of the accounts that crew's settings do not set up, under an account with no state repository, evaluates. Its repositories are kept and its stream is given, as for any drive.

**Non-Goals:**

- crew's module, which is unchanged. Its own rule that a drive given and not set up fails still stands. The set no longer gives such a drive.
- Flume's streams (`drives.nix`) and crew's kept repositories (`kept.nix`), which are given as now, by every drive of this host and every kept address.
- The failure for a drive of crew's that no account names, which stands as it is.
- A switch on a drive, or repositories kept outside a drive, each of which record 0032 turned down.
- The runner's code, its file's keys and its crew implementation (core.2, core.5).

## Decisions

### 1. The set adds to the type of crew's drives, and defines no drive

`crew-drives.nix` declares `services.crew.settings` again, a `types.submodule` whose `drives` is `types.attrsOf (types.submodule <the giving module>)`. The module system merges the declaration into crew's own: a submodule's modules are concatenated, and `attrsOf`'s element types merge with them. So each of crew's drives is evaluated with the giving module beside crew's own. The declaration sets no `description`, `default` or `example`, which only one declaration may set.

The giving module, a function of the drive's `name`, gives its `machine` and `state` from `given`. Each is under `lib.mkIf on`, and `on` is `crew.enable && given ? ${name}`. `given` is `crewDrivesOf crew.host (drivesOf config.services.boomdrive.accounts)`, as now, read only where crew's module is on. The module's `config` names `machine` and `state` as attributes of its own. The module system reads a module's attribute names before its arguments are known, and `name` comes from `_module.args`, so names under a `lib.mkIf` of `given.${name}` would recurse. Both the outer module and the giving module set `_file = "services.boomdrive.accounts"`, so each given value is placed there. A host's second write still fails crew's unique check. crew's failure lists `services.boomdrive.accounts` among the places that write the value, and the module adds no message of its own.

What this gives:

- A drive crew's settings set up, under an account that names its `state_repository`, is given its `machine`, the drive's or `services.crew.host` where it names none, and its account's `state_repository` as its `state`.
- A drive of the accounts that crew's settings do not set up is never evaluated by crew's type, so it is given nothing, and crew has no drive of its name.
- A drive crew's settings set up whose account names no `state_repository` is given nothing. Its machine and state are crew's settings' own, as before.
- The values are given with crew's module on, whether or not the runner is, as before.

The extension adds no key to crew's file and declares no option crew does not. It sets crew's `machine` and `state` of each drive, as core.46 and core.57 have the set set crew's module options. Its declaration of `services.crew.settings` lists `services.boomdrive.accounts` among that option's `declarations`, and its `declarationPositions` name `nix/modules/crew-drives.nix`.

That declaration is one of an option of crew's in this tree, so the change amends the service-modules requirement "The modules bring each tool's own module" and its scenario to admit it. The requirement carries record 0023's "each tool by its own module": each tool's options are the tool's own, a host can use the tool without Boomdrive, and this tree carries no copy of a tool's module. The declaration keeps all three. Every option is still declared in crew's input. The platform's crew module, re-exported, is crew's own with nothing added. And the declaration gives crew's values as core.57 has the set do. So the requirement says each option of `services.flume`, `services.sluice`, `services.crew` and `services.timber` is declared in its tool's input, and this tree declares one again only to add, to that option's submodule type, a module giving facts the set knows, declaring no option the tool's input does not and setting no description, default or example. Its scenario names crew-drives.nix's declaration of `services.crew.settings` and its `drives` as the one such declaration.

A prototype with the lock's nixpkgs `lib` checked each of these:

- crew's drives held only the host's names;
- a drive given and not set up was absent;
- a host's second write of `machine` failed, listing ``In `services.boomdrive.accounts'`` and the host's place;
- `valueMeta.configuration.options.drives.valueMeta.attrs.home.configuration.options.machine.definitionsWithLocations` read the definition's place, with or without that failure.

Not taken:

- **Reading crew's drives' names to choose what to give.** Giving a drive's value defines the drive, so its names would depend on the giving, and evaluation recurses.
- **Reading the host's own definitions of `services.crew.settings`, less the set's, for the drives they name.** A host's definitions may hold `lib.mkIf`, `lib.mkMerge` or a module function under `drives`. Collecting names from them would mean redoing the module system's merge, and a form it missed would leave a drive set up and given nothing.
- **An option of crew's own that takes the accounts' values and applies them to the drives crew's settings set up**, which would leave this tree declaring no option of crew's. That is a crew release for what the module system already gives here, through crew's own type.
- **`lib.mkDefault` on the given values.** It changes a value's priority, not whether the drive exists, so the set would still add the drive.

### 2. The runner's crew table holds the drives crew runs

In `options.nix`, `crewRuns` is `drive: crew.settings.drives ? ${drive.name}`. It is read only where crew's module is on. `written`'s partitions are the drives of `hereOf crew.host drives` with a `state_repository` and with `crewRuns`. Their form is as now: the drive's name as `label` and its account's `state_repository` alone, in the accounts' order. crew's drives' names are now the host's settings' alone (Decision 1), so reading them here recurses through nothing. The table's `defaultText` and `description`, and `options.nix`'s header comment, say "that crew's settings set up".

A table written out wins whole, as before.

Not taken:

- **Every drive of this host under an account that names a state repository, as before.** The table would then name a partition crew has no drive for, and crew's implementation would pull a `<label>/main` no crew role writes.
- **Dropping the state repository's filter, since the failure of Decision 3 catches a drive with none.** The table would then render a partition with no `state_repository` alongside that failure.

### 3. An account needs a state repository only for a drive crew runs

`stateless` lists each account whose `state_repository` is `null` and which holds a drive with `crewRuns`. It is read only where `crewPlane && crew.enable` holds, as now. Its failure's words: "services.boomdrive.settings.control_plane.type is crew, and services.boomdrive.accounts.<account>.state_repository is not set: under crew, an account with a drive crew's settings set up names its crew state repository, which the module gives that drive and the runner's crew table (records 0028 and 0032 as amended on 2026-11-18)." Where several accounts fail, the words list each, with "are".

Not taken:

- **An account naming no state repository is no account of crew's**, so the check would be dropped. Record 0032 turned it down: "That drops the check that catches a forgotten state repository, and one account may hold a drive crew runs and one it does not."

### 4. The "no drive" failure names the drives crew runs

`crewMachines` reads the drives with a `state_repository` and with `crewRuns`, each machine once, in the accounts' order. Where there is none, the words say "crew's settings set up no drive of the accounts under a state repository". The failure's words: "services.boomdrive.settings.control_plane.crew is written from the drives of services.boomdrive.accounts that services.crew.settings.drives sets up, whose machine is services.crew.host, ${host}, or that name none, under an account that names its state_repository, and no drive is: ${machines}. Set services.crew.host to this host's machine, or give a drive crew's settings set up this host as its machine (record 0032)." Its condition is as now.

### 5. The tests

`nix/tests/settings.nix` gains two fixtures, by core.41's names:

- `elsewhereHome`: `elsewhere`, whose account `work` keeps `away` on `host-b`, beside the account `own`, naming the state repository `example/state`, whose drive `home`, naming no machine, keeps agentplot's `flume`. It goes with `crewElsewhere`, crew's keys for `away` alone.
- `reading`: the runner's value under crew with no table. Its account `own`, naming `example/state`, has the drive `home` keeping agentplot's `flume`. Its account `work`, naming no state repository, has the drive `field` keeping agentplot's `sluice`. It goes with `crewKeys`, crew's keys for `home` alone.

`nix/tests/crew-drives.nix`:

- `placed` becomes `placedFor key evaluated`. It holds a definition placed at `services.boomdrive.accounts` among crew's drive `home`'s `key`, read from `valueMeta` as in Decision 1, reading no value. "Each drive's machine and state repository, given to crew" holds it for `machine` and `state`. "A machine written twice" holds it for `machine`, and "A state written twice" for `state`.
- "A drive crew's settings do not configure" reads the system of `crewAccounts` and `crewKeys`. It holds crew's drives to be `home` alone, with `host-a` and `example/state`, crew's teams file to evaluate and the system to fail no assertion.
- The header comment says the set adds to crew's drive type, giving the drives crew's settings set up.

`nix/tests/tools.nix`'s case "Each tool's options are its own flake's" fails an option of `services.<tool>` with no declaration in its tool's input, and one declared again in a file outside it but crew-drives.nix's declaration of `services.crew.settings` and its `drives`. It checks the options of that declaration's submodule types the same way, so every other option of crew's settings, and every option of a drive of crew's, is held to crew's input alone.

`nix/tests/tools.nix` gains three cases:

- "Only the drives crew's settings set up": the system of `crewAccounts` and `crewKeys` under crew with no table. It fails no assertion, and its table is `host-a` with the one partition `home` at `example/state`.
- "No drive crew runs on this host": `elsewhereHome` with `crewElsewhere`. It fails, by `failsNaming`, naming `services.crew.host`, `host-a` and `host-b`.
- "An account whose drives crew's settings do not set up": `reading` with `crewKeys`. It fails no assertion. crew's drives' names are `home` alone, the table is `host-a` with the one partition `home` at `example/state`, and Flume's streams hold `home` and `field`.

Every other case that stands still holds, with its name.

### 6. The README

In **Installing with nix**, the paragraph before the nix block says:

- an account names its crew state repository once, `services.boomdrive.accounts.<account>.state_repository`, where it holds a drive crew runs;
- with crew's module on, whether or not the runner is, each drive crew's settings set up, `services.crew.settings.drives.<drive>`, under an account that names its `state_repository`, is given its `machine`, `services.crew.host` where the drive names none, and its account's `state_repository` as its `state`, whichever machine it runs on;
- a drive crew's settings do not set up is given nothing: crew has no drive of its name, and the drive's repositories are kept, and its stream given where it runs, as any drive's;
- the runner's `[control_plane.crew]` holds each drive of this host that crew's settings set up, under an account that names its `state_repository`;
- under crew, an account with a drive crew's settings set up and no `state_repository`, or a drive of crew's that no account names, fails evaluation, and an account none of whose drives crew runs needs no `state_repository`.

The paragraph on a drive's `machine` says the module gives it to crew's drive of that name where crew's settings set the drive up, under an account that names its `state_repository`. The nix block, and `nix/tests/examples/host-flake.nix`, its copy, stay as they are (record 0049).

### 7. The records and rules

Record 0032 gains "Amendment, 2026-11-18: the module's forms" after the amendment it carries out. It holds Decisions 1, 2, 3 and 4, each a choice of this stage's, cheap to change, with the path not taken. core.57's rule names crew's drives as each drive the host sets up in crew's own settings. Its details rule out a drive the set makes crew's that crew's settings do not set up. They allow a drive crew's settings do not set up, given nothing, its account naming no state repository where it holds no drive crew runs. core.18's details read the runner's crew table from the drives crew's settings set up. core.46's details give crew each such drive's machine and state, and admit crew-drives.nix's declaration of `services.crew.settings` and its `drives`. The service-modules requirement "The modules bring each tool's own module" and its scenario are amended in this change's delta (Decision 1), and record 0032's forms name it. The records and rules are written with this change's design, and no task changes them.

### Rules relied on

- **core.57**: a fact the set knows from the host's accounts is given through the tool's own module option, and a host writes it nowhere. Amended here to name the drives crew's settings set up.
- **core.46, core.18**: the Boomdrive module sets crew's module options, `machine` and `state`, and writes no file of crew's. It declares crew's `services.crew.settings` again only to add the module that gives them. crew's module renders the teams file.
- **core.5**: the runner's file names its accounts, drives and repositories and no state repository, so it does not change. A drive crew does not run is kept like any drive.
- **core.19**: a runner serves the one control plane its `type` names, so the failures of Decisions 3 and 4 hold only under crew.
- **core.2**: no code outside the crew implementation names crew's records. The runner's crew implementation reads `[control_plane.crew]` as before.
- **core.12**: no key is added.
- **core.41**: neutral names in the tests and the README.
- **core.7, core.8**: no tool, crate or file type is added.
- **core.48**: no host moves, so the build's commits are `feat(nix):` and `docs(readme):`, none breaking.

### Rules amended

core.57 (rule and details), core.18's details and core.46's details, with record 0032's forms of 2026-11-18. The service-modules requirement "The modules bring each tool's own module" and its scenario, in this change's delta (Decision 1).

### Rules departed from

None.

## Dependencies

- crew `v0.68.0`, the lock's: a drive's `machine` and `state` written once, by the host's settings or another module; its drives an `attrsOf` of a submodule.
- The lock's nixpkgs: submodule declarations merge, and `valueMeta` gives a submodule's options to the tests.
- Record 0032 as amended on 2026-11-18.

## What this change asks of others

Nothing must move. afterthought/swancloud may name deneb's drive `wes` under its account in `services.boomdrive.accounts`, with no crew keys and no state repository, once it takes a set carrying this change.

## Risks / Trade-offs

- [A host misspells a drive's name in crew's settings, so the drive it meant goes unset up] → The misspelled name is a drive of crew's that no account names, which still fails under crew, naming it.
- [A crew release changes `drives` from an `attrsOf` of a submodule, or renames `machine` or `state`] → The declaration stops merging, or gives an option crew no longer declares, and evaluation fails in the change that moves crew's tag, before any host switches.
- [`services.crew.settings` lists `services.boomdrive.accounts` among its declarations, so crew's option documentation names it] → It is the option the values come from, so the place a reader is sent is where the fix is.
- [Under crew, a drive of this host crew does not run has a stream and no partition] → That is the drive record 0032 keeps: its repositories kept, a stream given where it runs, no main level.

## Migration Plan

None: a host that evaluates today gives crew the same drives and the runner the same table. To roll back, a host returns the flake's revision, and a drive it names with no crew keys must then leave its accounts or be set up in crew.

## Task notes

**1.1** In `tools.nix`, `again` names the declarations this tree may make again, by the option's name, each with its file:

```nix
again = lib.genAttrs [
  "services.crew.settings"
  "services.crew.settings.drives"
] (_: toString ../modules/crew-drives.nix);
```

`outsideOf` fails an option none of whose files is under its tool's input, or one of whose files outside it is not `again`'s file for its name. For each option `again` names, it checks the same way the options of `option.type.getSubOptions option.loc`, less `_module`. `filesOf` stays as it is.

**1.2** `crew-drives.nix` as Decision 1:

```nix
{ config, lib, ... }:
let
  inherit (import ./accounts.nix { inherit lib; }) drivesOf crewDrivesOf;
  crew = config.services.crew;
  given = crewDrivesOf crew.host (drivesOf config.services.boomdrive.accounts);
in
{
  _file = "services.boomdrive.accounts";
  options.services.crew.settings = lib.mkOption {
    type = lib.types.submodule {
      options.drives = lib.mkOption {
        type = lib.types.attrsOf (
          lib.types.submodule (
            { name, ... }:
            let
              on = crew.enable && given ? ${name};
            in
            {
              _file = "services.boomdrive.accounts";
              config = {
                machine = lib.mkIf on given.${name}.machine;
                state = lib.mkIf on given.${name}.state;
              };
            }
          )
        );
      };
    };
  };
}
```

Its header comment cites record 0032 as amended on 2026-11-18. It says why the module adds to crew's type rather than defining crew's drives, and why `config` names `machine` and `state` itself. `accounts.nix`'s header comment says `crewDrivesOf` gives what crew-drives.nix gives each drive crew's settings set up. In `nix/tests/crew-drives.nix`, `placedFor` is:

```nix
placedFor =
  key: evaluated:
  lib.any (definition: definition.file == "services.boomdrive.accounts")
    evaluated.options.services.crew.settings.valueMeta.configuration.options.drives.valueMeta.attrs.home.configuration.options.${key}.definitionsWithLocations;
```

`unconfigured` reads `system (crewWith settings.crewAccounts settings.crewKeys)` with `holdsDrives`, `loads` and `t.failures`, as `stateless` does.

**2.1** `crewRuns` sits beside `crewPlane` in `options.nix`'s `let`. `written`'s filter is `drive: drive.state_repository != null && crewRuns drive`. `crewMachines` takes the same filter. `elsewhereHome` is `elsewhere // { accounts = elsewhere.accounts // { own = { state_repository = "example/state"; drives.home.repositories.agentplot = [ "flume" ]; }; }; }`. In `tools.nix`, the case functions `setUp` and `elsewhereHere` sit after `accountsOrder` and `elsewhere`, each taken by `onEach` after its neighbour.

**2.2** `stateless` in `options.nix` is `lib.attrNames (lib.filterAttrs (_: account: account.state_repository == null && lib.any (drive: crew.settings.drives ? ${drive}) (lib.attrNames account.drives)) cfg.accounts)`. In `tools.nix`, the case function `reading` reads crew's drives' names as `lib.attrNames evaluated.config.services.crew.settings.drives` and Flume's streams as `evaluated.config.services.flume.settings.streams`, holding `? home` and `? field`. It sits after `stateless`.

**3.1** statix refuses a key repeated in one attribute set, and deadnix a binding nobody reads, so `placed` goes with its last use.
