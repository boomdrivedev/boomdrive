# Design

## Context

The set pins crew `v0.50.2` (`flake.nix`, `flake.lock`). crew's teams file there is version 5 (`plugin/lib/crew.py`, `VERSION = 5`). Its drives are a list, `partitions`, each with a `partition` and a `label`, `flume` for how its roles run, and `review`'s `every` and `hours`. crew's tree carries no version at that tag, so `nix/packages/crew.nix` gives crew's flake's package the tag's version with `overrideAttrs` (core.44's details: "a version given to a tool's package where its tagged tree does not carry it, until it does").

crew `v0.59.0` (`6698fdae`) is crew's newest release. At that tag:

- **Its teams file is version 6** (`VERSION = 6`). crew record 0054 renamed "partition" to "drive". crew record 0064 has crew read every fact of a host's drives from its own file alone. Its amendment of 2026-11-09, accepted, renamed two keys on the user's words: "I don't understand what this section is about and why we're turning these things off", and "a comment should explain the units/options". crew's module, `nix/modules/options.nix`, declares:
  - `settings.drives`, an attribute set keyed by label, which crew's `json.nix` writes as the file's list in label order. Each drive has `label`, read-only, its name; and `blueprints`, `state`, `machine`, `session`, `folder`, `repos`, `accounts`, `review` and `runs`, each required with no default. `roles` is optional.
  - A drive's `runs`: exactly `design`, `planner`, `reviewer`, `ops` and `operator`, each `"standing"` or `"on-demand"`.
  - A drive's `review`: exactly `after_changes`, a whole number of at least 1, and `after_hours`, a number above 0. It declares no free-form type, so `every` or `hours` fails evaluation.
  - A drive's `accounts`: the Claude account each of those five roles runs under.
  - The drive itself is free-form JSON, so a `flume` key passes the module. With no `runs`, though, evaluation fails, since `runs` has no default.
  - `settings.hosts`, required and non-empty: each host crew reaches or reads, by its `hostname -s` name, with exactly `ssh` and `kind` (`darwin` or `nixos`).
  - `settings.version`, read-only, from `crew.py`.
  - The header comment: "Which keys agree with which (a machine with no host, a blueprints repo not kept, two accounts in one session) is crew's to judge at load."
- **Its tree carries its version.** `plugin/release.json` says `{"version": "0.59.0"}`. crew's flake reads it through `nix/version.nix`, so `packages.<system>.crew.version` is `0.59.0`.
- **Its flake's inputs are unchanged**: `nixpkgs` and `nix-darwin`, which `flake.nix` already follows. Its `darwin.nix` and `nixos.nix` modules are unchanged since `v0.50.2`.
- **`plugin/lib/source.py`** still names `ROLES_FILE` and `KINDS_DIR` as at `v0.50.2`.

In this tree:

- `nix/modules/options.nix` writes the runner's `[control_plane.crew]` from `crew.settings.partitions`: those whose `machine` is `services.crew.host`, in list order, as `label` and `state_repository`. Its second failure lists the partitions' machines. Its header comment and the drive's `machine` description name crew's partitions.
- `nix/tests/settings.nix` writes crew's fixtures, `crewModule`, `crewFleet`, `crewOrder` and `crewElsewhere`, by version 5's names, through `partitionOf`. `tools.nix`, `darwin.nix`, `nixos.nix`, `cache.nix` and `lib.nix`'s `toolsOf` turn crew on with them.
- `nix/tests/packages.nix`'s "crew through its own flake" holds the set's crew to crew's source, name and home page, and its version to the tag's.
- The README's **Installing with nix** turns crew on with `settings.partitions`, `review = { every = 5; hours = 4; }` and `flume = { … false … }`. Its words name crew's partitions three times.

Record 0032: "Boomdrive's module sets crew's options as crew's file names them, so crew's rename of 'partition' to 'drive' (crew record 0054) moves in the change that moves crew's version."

## Goals / Non-Goals

**Goals:**

- The set's crew is `v0.59.0`, as crew's flake builds it, reporting its own version.
- A host on the set loads crew's teams file at version 6, its settings written by crew's names.
- The runner's crew table is written from crew's drives.

**Non-Goals:**

- Giving crew each drive's `machine` and `state` from `services.boomdrive.accounts` (record 0032). That is the unit `each-drives-machine-and-state-are-written-once`, which builds on this pin. Here the host still writes both in crew's drives.
- Giving crew each kept repository's `dir` and `checkout` (record 0036, point 1), the unit `the-set-gives-crew-each-kept-repositorys-folders`, which also builds on this pin.
- Renaming the runner's own table's `partitions` (crew-control-plane spec). That is the runner's key, read by its crew implementation, and no key of crew's file.
- Any other tool's pin.

## Decisions

### 1. The pin is crew `v0.59.0`

`flake.nix`'s crew `url` is `github:agentplot/crew/v0.59.0`, and the lock moves with it in the same task (core.45). It reads teams file version 6 with `runs` and `review`'s `after_changes` and `after_hours`, and its tree carries its version. It is crew's newest release, and record 0036, point 3, has the set pin each tool "at its latest release that runs with the others". The crew input's `follows` stay as they are, since crew's flake takes the same two inputs.

Not taken:

- **`v0.54.0`, the first to read version 6.** It reads `flume` and `every`, which crew's amendment of 2026-11-09 renamed, and its tree carries no version.
- **`v0.58.5`, the first to name `runs` and carry its version.** It is not crew's latest release (record 0036, point 3).

### 2. crew's package is crew's flake's own, unchanged

`flake.nix` gives `crew = inputs.crew.packages.${system}.crew;`, as it gives Flume, and `nix/packages/crew.nix` is removed. core.44's allowance for a version given to a package lasts only until the tagged tree carries its version, and crew's does now. The override also changed the derivation, since `__intentionallyOverridingVersion` is an attribute of it. So the set's crew was a path crew's own flake never builds. Unchanged, it is that path.

`packages.nix`'s "crew through its own flake" holds `crew.drvPath` to crew's flake's own `packages.<system>.crew.drvPath`, as "Flume through its own flake" does, and its version to the lock's tag without the `v`.

Not taken:

- **Keeping the override, giving the same version.** It is a recipe kept for nothing, and it moves the store path off crew's own (core.44: "a recipe kept here for a tool whose own flake builds it").

### 3. The runner's crew table is read from crew's drives, in label order

In `options.nix`, `written`'s entries come from `lib.attrValues crew.settings.drives`, filtered by `drive.machine == crew.host`. Each is `{ inherit (drive) label; state_repository = drive.state; }`. `machines`, in the second failure's words, comes from the same values. The two failures say "drive" where they said "partition", and name `services.crew.settings.drives`. `label` is crew's read-only option, the drive's name.

The order is the labels', which is the only order crew's version-6 settings have, and the order crew's file lists them. The runner's crew implementation reads each partition by its label, so nothing turns on the order (crew-control-plane spec).

Not taken:

- **Keeping "crew's order" as a list.** crew's settings hold no list of drives at version 6.

### 4. crew's own keys stay the host's

The module writes no key of crew's `hosts`, and none of a drive's `runs`, `review`, `session`, `folder`, `repos`, `accounts` or `blueprints`. Until record 0032's unit lands, it writes no `machine` or `state` either. Each is a fact only the host knows, set through crew's own options (core.57: a host "gives through a tool's own settings only what is its own"; record 0036, point 4).

Not taken:

- **Giving `hosts` from the accounts' machines.** The set knows no host's `ssh` name, and no other host's `kind`. Giving this host's `kind` alone, from the platform module, would split one table between two writers.
- **A check of the set that every drive's `machine` is among `hosts`.** crew's own header gives that judgement to crew, at load. Two tools judging one fact is what the family rule in CLAUDE.md forbids.

### 5. The tests write crew's settings by crew's names

`nix/tests/settings.nix` replaces `partitionOf` with `driveOf label machine state`. It gives a drive every key crew's module requires at its tag:

```nix
{
  inherit machine state;
  blueprints = [ "example/blueprints" ];
  session = label;
  folder = "/srv/crew/${label}";
  repos = [ ];
  accounts = {
    design = "example";
    planner = "example";
    reviewer = "example";
    ops = "example";
    operator = "example";
  };
  runs = {
    design = "standing";
    planner = "standing";
    reviewer = "on-demand";
    ops = "on-demand";
    operator = "standing";
  };
  review = {
    after_changes = 5;
    after_hours = 4;
  };
}
```

`hosts` is `host-a`, `{ ssh = "host-a"; kind = "darwin"; }`, and `host-b`, `{ ssh = "host-b"; kind = "nixos"; }`. Every fixture names both, so every drive's machine is among them.

- `crewModule` is `{ inherit hosts; drives.home = driveOf "home" "host-a" "example/state"; }`.
- `crewFleet` gives `home` and `away`, on `host-b` at `example/away`.
- `crewOrder` gives `garden`, `away` and `home`.
- `crewElsewhere` gives `away` alone.

Two fixtures carry crew's old names. `crewOldRuns` is `crewModule` whose `home` drops `runs` and names `flume`, each of the five roles `false`. `crewOldReview` is `crewModule` whose `home` has `review = { every = 5; hours = 4; }`.

Both runs values are written so a case can tell the module wrote them as given. Every name is core.41's: `host-a`, `host-b`, `example`, and the folder `/srv/crew/<label>`. `repos` is empty: crew's description has it empty "where a host's `repositories` names them", and no case loads crew.

### 6. New cases hold the set's crew to version 6 and crew's names

In `tools.nix`, each on nix-darwin and on NixOS through `onEach`:

- **"A host on the set's crew"** evaluates `system crewOn`. It holds `t.failures` empty, and `config.services.crew.settings.version` equal to `6`. It holds that `builtins.deepSeq` of `config.services.crew.settings` under `builtins.tryEval` succeeds.
- **"crew's keys stay the host's"** evaluates the same system. It holds `settings.hosts` equal to the fixture's `hosts`, and `settings.drives.home.runs` and `.review` equal to the fixture's, so the module added and changed nothing of them (Decision 4).
- **"crew's old way of running"** and **"crew's old batch names"** evaluate `system (crewOf settings.crewOldRuns)` and `system (crewOf settings.crewOldReview)`. Each holds that the same `tryEval` of `deepSeq` fails. The module system throws for an option used with no value, and for a definition of an option a submodule does not declare. `tryEval` catches both, so the case reads success alone.

The version is written as `6`, not read from crew. A crew release that moves the file's version then fails this case in the change that moves the pin (core.45), so the host's move is seen there.

Beside "The README's crew table", **"The README's crew settings"** finds a nix block of `t.nixBlocks` naming `services.crew`. It holds each of `drives`, `runs`, `after_changes`, `after_hours`, `hosts`, `ssh =` and `kind =` in that block. It holds that no nix block names `partitions`, ` flume = {`, `every =` or `hours =`. The leading space in ` flume = {` tells a drive's key from `services.flume = {`.

### 7. The cases on crew's partitions take crew's drives

Each case's name is its scenario's. OpenSpec's strict check refuses a scenario renamed inside a MODIFIED requirement, so the delta removes the two requirements whose scenarios are renamed. It adds them again under new names: "Under crew, the runner's crew table is written from crew's drives on this host", and "Under crew, a host the runner cannot serve fails evaluation". Each renamed scenario renames its case:

- "crew's partitions on this host on nix-darwin" and "… on NixOS" become "crew's drives on this host on …", in `darwin.nix` and `nixos.nix`.
- "crew's order" becomes "crew's drives in label order", in `tools.nix`.
- "No partition on this host" becomes "No drive of crew's on this host", in `tools.nix`.

Their comments say drive where they said partition. The runner's own table in `darwin.nix` and `nixos.nix`, `garden`, keeps `partitions`, since that is the runner's key. `nix/tests/files/crew.toml` does not change: the runner's file is the same.

### 8. The README

**Installing with nix**'s nix block writes `services.crew.settings` by crew's names. It writes one drive, `home`, with every key crew requires, `runs` and `review` each carrying a comment that says what its values mean. It writes `hosts.host-a`. Its words say:

- A host that runs crew turns on crew's module "with its `host`, its drives and the hosts crew reaches".
- The module writes `[control_plane.crew]` from crew's drives, holding each whose `machine` is `services.crew.host`.
- A drive's `machine` names another computer "as crew's drives name a host".
- The module writes none of crew's `hosts`, `runs` or `review`, which are the host's own.

### Rules relied on

- **core.44**: each tool through its own flake at its tag, reporting its tag's version, with no recipe kept here for a tool whose flake builds it.
- **core.45**: the tag moves only with the lock, in a change the gate passes.
- **core.46, core.18**: the Boomdrive module sets crew's module options and writes no file of crew's. crew's module renders the teams file.
- **core.57**: the module gives only what the set knows. A host's own facts go through the tool's own option.
- **core.5**: the runner's file names no stream or kind, and does not change.
- **core.41**: neutral names in the tests and the README.
- **core.7, core.8**: no tool, crate or file type is added.
- **core.48**: the build's commit that moves the pin is a breaking change, `feat(nix)!:`, whose subject names the host's move, as record 0035's commits did.

### Rules amended

None.

### Rules departed from

None.

## Dependencies

- crew `v0.59.0`, `6698fdaef7c6eb2b1ec34808f1d7a9c016d85622`: teams file version 6, `runs`, `review`'s `after_changes` and `after_hours`, `hosts`, and `plugin/release.json` at `0.59.0`.
- crew records 0054, 0061, 0064 and its amendment of 2026-11-09, and 0065.

## What this change asks of others

A host that takes the set with crew on, afterthought/swancloud's among them, writes its `services.crew.settings` by version 6's names in the switch that takes this change. crew record 0064's consequences already ask that of swancloud.

No other repository changes. The units `each-drives-machine-and-state-are-written-once` and `the-set-gives-crew-each-kept-repositorys-folders` follow this one, since each sets an option crew declares only from version 6.

## Risks / Trade-offs

- **A host's switch fails until it moves its settings.**
  - Version 5's `partitions` leaves crew's `drives` and `hosts` undefined, so evaluation fails naming them.
  - `review.every` fails, naming the option.
  - A drive with `flume` and no `runs` fails, naming `runs`.
  - Each failure is at evaluation, before anything on the host changes. No code carries a host across (record 0032: "I want the system to be built the way it should be and then have agents fucking fix the data transition as necessary").
- **crew's teams file on a host moves from version 5 to 6** when crew's module renders it at the switch. crew v0.59.0 refuses a version-5 file. A host that keeps its own crew build older than v0.54.0, through `services.crew.package` (record 0034, point 7), would refuse the version-6 file the module writes. Record 0036, point 3, has a host keep no build of its own.
- **crew's store path changes** with the pin. The family's cache holds the new path once the cache workflow runs on `main` (core.47).

## Migration Plan

Each host takes the change at its next switch, with its `services.crew.settings` written by version 6's names in the same switch. crew's module then links a version-6 `~/.config/crew/teams.json`, which crew v0.59.0 reads at its next command. To roll back, the host returns both the flake's revision and its crew settings.

## Open Questions

None.

## Task notes

**1.1** In `flake.nix`, crew's `url` becomes `github:agentplot/crew/v0.59.0`. Lock it in the devenv shell, outside the sandbox, with `devenv shell -- nix flake update crew` (nix 2.34 spells the old `--update-input` this way). Read the lock's diff: it moves only the `crew` node. `flake.nix`'s `crew = import ./nix/packages/crew.nix { … };` becomes `crew = inputs.crew.packages.${system}.crew;`, beside `flume`'s, with a comment like Flume's. `git rm nix/packages/crew.nix`. The header comment of `flake.nix` is unchanged. Enter the devenv shell once more before the check, so `BOOMDRIVE_FLAKE_INPUTS` holds the new crew. The task's commit is the breaking one, `feat(nix)!: the set takes crew v0.59.0, whose teams file a host writes by crew's names: drives, runs, review's after_changes and after_hours, and hosts`. nix:test fails from this task until 1.2's fixtures move, which is why 1.1's check is the type check alone.

**1.2** `settings.nix` as Decision 5. Its header comment says crew's fixtures are written by crew's names at the lock's crew, citing crew record 0064 as amended on 2026-11-09, and lists `host-a` and `host-b` as crew's hosts. `options.nix` as Decision 3, including:

- its header comment's sentence, "from crew's drives whose `machine` is `services.crew.host`";
- the `machine` option's description, "as crew's drives' `machine` names one";
- `control_plane.crew`'s `defaultText` and `description`, "each of `services.crew.settings.drives` whose `machine` is that host, in label order";
- the first failure's words, "with its host, its drives and its hosts";
- the second's: "…is written from crew's drives whose machine is services.crew.host, ${crew.host}, and no drive's is: they name ${machines}. Set services.crew.host to this host's machine, or give one of services.crew.settings.drives this host as its machine (record 0023)."

`tools.nix`'s header comment says "with its `host`, its drives and its hosts". The case renames are Decision 7's. `packages.nix`'s crew case is Decision 2's, its comment saying crew's tree carries its version. `cache.nix`'s comment on crew says "its host, its drives and its hosts".

**1.3** The cases of Decision 6, in `tools.nix`. `crewOf` and `crewOn` already exist there. A `loads` function, `evaluated: (builtins.tryEval (builtins.deepSeq evaluated.config.services.crew.settings true)).success`, serves all three. The fixture's `hosts` and drive `home` are read from `settings.crewModule`.

**1.4** The README's nix block of **Installing with nix** gives:

```nix
            services.crew = {
              enable = true;
              host = "host-a";
              settings = {
                drives.home = {
                  blueprints = [ "example/blueprints" ];
                  state = "example/state";
                  machine = "host-a";
                  session = "home";
                  folder = "/Users/alice/Code/agentplot";
                  repos = [ "agentplot/flume" "agentplot/sluice" ];
                  accounts = { design = "home"; planner = "home"; reviewer = "home"; ops = "home"; operator = "home"; };
                  # Each main-level role "standing", its agent kept running in its pane,
                  # or "on-demand", run by Flume in the drive's stream while work waits for it.
                  runs = { design = "standing"; planner = "standing"; reviewer = "on-demand"; ops = "on-demand"; operator = "standing"; };
                  # A batch of changes is due for your read once 5 wait, or 4 hours after the last.
                  review = { after_changes = 5; after_hours = 4; };
                };
                hosts.host-a = { ssh = "host-a"; kind = "darwin"; };
              };
            };
```

Its paragraph before the block, and the paragraph on a drive's `machine`, say Decision 8's words. "The README's crew settings" sits in `tools.nix`'s attribute set beside "The README's crew table".

**1.5** statix refuses a key repeated in one attribute set, so each fixture's `drives` is one attribute set. deadnix refuses a binding nobody reads, so `partitionOf` goes with its last use.
