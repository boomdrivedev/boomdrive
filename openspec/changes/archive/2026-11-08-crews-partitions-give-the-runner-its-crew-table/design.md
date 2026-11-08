# Design

## Context

Record 0023 (accepted 2026-11-06) writes crew's partitions once, in crew's module:

- `services.crew.settings.partitions` lists every partition of the fleet with its `machine`.
- `services.crew.host` names this host to crew.
- "Where the control plane is crew and no `[control_plane.crew]` is written, the Boomdrive module writes it: the host from `services.crew.host`, and each partition whose `machine` is this host. A table written out wins. The runner's file still names its partitions (core.17), and neither the runner nor crew changes."

Its talk's design (`design/the-flakes-configuration/design.md`) spells the table out: "`host` from `services.crew.host`, and a `label` and `state_repository` for each partition whose `machine` is this host". Rule core.18's details already allow the Boomdrive module "writing the runner's `[control_plane.crew]` from crew's partitions whose `machine` is this host, a table written out winning (record 0023)".

What stands today:

- **The runner's crew table.** crew's implementation reads `[control_plane.crew]`: `host`, a crew name, and `partitions`, at least one, each a `label` given once and a `state_repository` `owner/name`. It refuses any other key (the crew-control-plane spec, "crew's table names the host and the partitions"). Under `type = "crew"` with no table, it refuses at start for want of `host`.
- **The runner's module.** `nix/modules/options.nix` declares `settings.control_plane` as `nullOr` a submodule holding `type`, `poll_interval`, `crew` and `hosted`. `crew` is `nullOr (attrsOf toml)`, null by default and left out of the file where null. `darwin.nix` and `nixos.nix` render `cfg.settings` by `toml.nix`.
- **crew's module.** It comes from crew's flake at `v0.44.8`, and is imported by the Boomdrive module since `the-flake-configures-each-tool-by-its-own-module`. It declares `services.crew.enable`, `host` (a crew name, required) and `settings.partitions`, a non-empty list in which each partition declares `label`, `state` and `machine`, among others. Neither `host` nor `partitions` has a default. The Boomdrive module leaves `services.crew.enable` to the host (that change's Decision 5).
- **The README.** Its install with nix writes the partition `home` twice: once in `services.crew.settings.partitions`, and once in `services.boomdrive.settings.control_plane.crew` with the host.

This change is the bolt's unit `crews-partitions-give-the-runner-its-crew-table`.

## Goals / Non-Goals

**Goals:**

- Under crew, a host writes its partitions once, in crew's module, and the runner's file gets its crew table from them.
- A table a host writes out is the file's table, unchanged.
- A host under crew that the runner could not serve fails at evaluation, not at the runner's start.

**Non-Goals:**

- Any change to the runner's code, its file's keys, crew's implementation or any file under `nix/tests/files`.
- Any option of crew's module, or of `services.boomdrive` beside `settings`.
- The hosted control plane's partitions, which are enrollments in the runner's file alone (record 0023).
- Relating crew's partitions to the runner's accounts and drives. The runner does not relate them today, and record 0028's options for accounts are the unit `each-repository-is-named-once-by-its-owner`'s.

## Decisions

### 1. The table is `control_plane.crew`'s default

`options.nix` gives `control_plane.crew` a default, computed in the control plane's own submodule:

- where that submodule's `type` is `crew` and crew's module is on, the table written from crew's partitions (Decision 3);
- else `null`, as now.

What follows from it:

- `settings.control_plane.crew` reads the table, so `nix eval` of a host's `services.boomdrive.settings` shows what the file holds.
- `darwin.nix` and `nixos.nix` render `cfg.settings` as now. One `settings` still renders one text on either platform.
- The default is evaluated only where the host names a control plane at all, since `control_plane` stays null otherwise.
- The default reads the host's `services.crew` only where it is declared. So `options.nix` evaluated alone, as `nix/tests/file.nix` evaluates it, writes no table.

Alternatives:

- **A definition of `settings.control_plane.crew` set from `tools.nix` by `mkDefault`, under `mkIf`.** Not taken:
  - The condition reads `settings.control_plane.type`, the option being defined, and nix recurses for ever.
  - A definition made without the condition gives `control_plane` a value on every host. A host that names no control plane would then fail for want of `type`.
- **The table filled in at render time, in `darwin.nix` and `nixos.nix`.** Not taken:
  - `settings.control_plane.crew` would read null where the file holds a table.
  - Each platform's module would carry the fill.
- **The runner reading crew's `teams.json`.** Record 0023 did not take it. core.17 rules it out, and it would tie the runner to crew's file versions.

### 2. Only where crew's module is on

The table is written only where `services.crew.enable` is true. crew's facts are the host's own where it turns crew on, with its `host` and its partitions.

Alternatives:

- **Wherever crew's `host` and `partitions` are defined, whether or not crew's module is on.** Not taken:
  - A host that names crew's facts and keeps crew off has said it does not want crew's file. Reading its facts for the runner anyway is a second meaning for one option.
  - `the-flake-configures-each-tool-by-its-own-module`'s Decision 5 already has a host under crew that runs no crew command write its table out.
- **The Boomdrive module turning crew on wherever the control plane is crew.** That change's Decision 5 did not take it: the runner reads crew's state repository, not crew's file.

### 3. Which partitions, and what of each

- The table's `host` is `services.crew.host`.
- Its `partitions` are those of `services.crew.settings.partitions` whose `machine` equals `services.crew.host`, in that list's order.
- Each is `{ label; state_repository = state; }`, and nothing else of the partition: crew's implementation refuses any other key.
- crew's module types `host` and `machine` alike as crew names, lower-case letters, digits and `-`, so they are compared as written.

Alternatives:

- **Every partition of the fleet.** Not taken. The runner would serve another machine's partitions and write its runner branch on them. It would then claim their launches that name no host (the crew-control-plane spec, "A launch naming no host is claimed before it is given"). Record 0028 gives a drive whose `machine` is another computer no agents on this one.
- **The partitions in label order.** Not taken: crew's order is the order the host wrote, and the runner reads its partitions in their order.

### 4. A table written out wins whole

A host's own `settings.control_plane.crew` is a definition above the option's default, so the module system takes it in place of the default, whole. A table the host writes with `host` alone is the file's table, and the runner judges it.

Alternatives:

- **The host's table merged with crew's partitions.** Not taken:
  - Record 0023 says a table written out wins.
  - A merge would serve a partition the host had left out on purpose.
  - Partitions are added to a running host through the local layer, whose `partitions` replace the managed layer's whole (core.17).

### 5. Nothing beside another control plane

The default is `null` where `type` is not `crew`. With no `control_plane`, no default is evaluated. So under a hosted control plane, and on a host the runner serves no control plane for, the file is what it is today, whatever crew's module holds. core.19 has the runner serve one control plane, and a crew table beside a hosted one would be a second.

### 6. Two hosts under crew fail evaluation

With `services.boomdrive.enable`, `options.nix` adds two assertions:

- **Neither.** `type` is `crew`, and `control_plane.crew` is null: the host wrote none and crew's module is off. The words name `settings.control_plane.crew` and `services.crew`, and say the module writes the table from crew's partitions once crew is turned on with its host and partitions.
- **No partition on this host.** `type` is `crew`, the table is the module's, and it holds no partition. The words name `services.crew.host` and its value, and each `machine` the partitions name.

The runner refuses both files at start today, for want of a `host` or of a partition. Under the host's service manager it does so every 30 seconds, and the switch that wrote the file succeeds. Failing the switch names the option to fix, as the flat `repositories` of before does.

Alternatives:

- **The runner's refusal at start, as now.** Not taken: the module can tell at evaluation, and the switch is where the host is being written.
- **An empty `partitions` written, for the local layer to fill.** Not taken:
  - Under crew, no control plane writes the runner's local layer.
  - A host that means its local layer to name the partitions writes the table out, with its `host`, and Decision 4 takes it as written.
- **A table the host writes out held to the same checks.** Not taken: the runner judges a table written out, and the local layer may replace its `partitions` (core.17).

### 7. The runner and its files do not change

The file the module writes is one the runner already reads.

- For the delta's fleet, `home` on `host-a` and `away` on `host-b`, beside the settings of "A host under crew", the module writes `nix/tests/files/crew.toml`, byte for byte.
- So `crates/boomdrive/tests/module_files.rs`, which reads each file there, proves the runner reads it.
- No file is added to `nix/tests/files`, so that test's list of files is unchanged.

### 8. The tests

Every case is an evaluated case under `nix/tests`, run by `nix:test` against no store (record 0020's Decision 14). Each case's names are core.41's: the hosts `host-a` and `host-b`, users `alice` and `bob`, and repositories under `example/`.

- **`settings.nix`** gains:
  - `crewUnwritten`: the settings of `crew` with `control_plane = { type = "crew"; }` and no table;
  - `crewFleet`: crew's settings with the partitions `home`, on `host-a` at `example/state`, and `away`, on `host-b` at `example/away`;
  - `crewOrder`: the partitions `garden` on `host-a` at `example/garden`, `away` on `host-b` at `example/away`, and `home` on `host-a` at `example/state`;
  - `crewElsewhere`: the one partition `away` on `host-b`.

  Each partition holds every key crew's module asks for at its tag, as `crewModule`'s does, made by one function of label, machine and state.
- **`darwin.nix` and `nixos.nix`** each gain, on their platform, through their own `writes` and `runs`:
  - "crew's partitions on this host";
  - "A table written out wins";
  - "A hosted control plane beside crew";
  - "No control plane beside crew".
- **`tools.nix`** gains, on each platform by `onEach`:
  - "crew's order";
  - "Under crew with neither";
  - "No partition on this host".

  It also gains "The README's crew table".
- **`lib.nix`** gains `installing` and `nixBlocks`, the README's install with nix and its nix blocks, which `cache.nix` takes from it in place of its own.

### Rules relied on

- core.18: its details allow the Boomdrive module writing the runner's `[control_plane.crew]` from crew's partitions whose `machine` is this host, a table written out winning. The module writes no file of crew's, and sets no option of crew's module.
- core.46: the runner's file is written from one option per key, `settings.control_plane.crew` among them, and the Boomdrive module writes no other tool's file.
- core.17: the runner's file still names its partitions, and a person can write the same table by hand. The local layer's `partitions` still replace the managed layer's.
- core.2: the table is the one crew's implementation reads, and no code of the runner's changes.
- core.19: no crew table beside a hosted control plane (Decision 5).
- core.4: the table holds names and addresses alone. The credential check still runs over `settings`, the written table among them.
- core.5: no stream or kind is written.
- core.44: crew's module and its options are as crew's flake gives them at the tag the lock names.
- core.41: neutral names in every case.
- core.7 and core.8: no crate, tool or tracked file type is added.

### Rules amended

None. core.18's details already name the table written from crew's partitions.

### Rules departed from

None.

## Dependencies

- crew's flake at `v0.44.8`, the lock's, whose module declares `services.crew.host` and `services.crew.settings.partitions` with `label`, `state` and `machine`.
- crew record 0054 renames crew's "partition" to "drive". When a tag of crew's renames the module's `partitions`, the change that moves crew's tag moves Decision 3's reading with it. The runner's table keeps its `partitions` (record 0028's amendment of 2026-11-07, the keys of the runner's file).

## What this change asks of others

- **afterthought/swancloud**, once it takes the flake, can drop deneb's `control_plane.crew`. deneb's partitions in `services.crew.settings.partitions` and its `services.crew.host` give the same table (record 0023).

## Risks / Trade-offs

- **A switch that passed may now fail.** A host under crew that wrote no table and kept crew off got a file the runner refused at start. Now its switch fails, naming what to set (Decision 6). No host is known to do so: afterthought/swancloud does not yet import these modules.
- **One fact may be offered in two places.** Record 0028's sketch has an account's `state` and a drive's `machine` among `services.boomdrive.accounts`' options. This change reads both from crew's partitions alone, where record 0023 writes them once. The unit `each-repository-is-named-once-by-its-owner` gives the accounts' options, and its design says how they and crew's partitions stay one fact.
- **A table written out stays as written.** A host that writes its table and edits crew's partitions later keeps the table it wrote. It drops the table to have the module write it.

## Migration Plan

A host that writes both today keeps its own table, which wins (Decision 4). To have the module write it, the host removes `settings.control_plane.crew`, once `nix eval` of its `services.boomdrive.settings.control_plane.crew` with the table removed gives the table it wrote.

## Open Questions

None.

## Task notes

**1.1** In `options.nix`:

- Bind the system's `config` before the submodule shadows it, for example `outer = config;`.
- Let `crew` be `outer.services.crew or { enable = false; }`, so a system with no crew module reads it as off.
- `written` is `null` unless `crew.enable`. Where crew is on, it is:

  ```nix
  {
    inherit (crew) host;
    partitions = map (p: { inherit (p) label; state_repository = p.state; })
      (lib.filter (p: p.machine == crew.host) crew.settings.partitions);
  }
  ```

- `controlPlane` becomes `types.submodule ({ config, ... }: { … })`.
- Its `crew` option keeps its type, gains `default = if config.type == "crew" then written else null;` and a `defaultText` saying so in words, and its description says a table written out wins.
- The header comment says so too, citing record 0023 and this change.

`settings.nix` gains `crewUnwritten`, `crewFleet`, `crewOrder` and `crewElsewhere`, as Decision 8 names them. A `partitionOf = label: machine: state: { … }` holds `crewModule`'s other keys, and `crewModule` keeps its one partition `home` on `host-a`.

In `darwin.nix` and `nixos.nix`, "crew's partitions on this host on <platform>" is a system of `crewUnwritten` with crew on, host `host-a` and `crewFleet`. It fails no assertion, and `writes` (or `runs`) holds its file to `files/crew.toml`.

In `tools.nix`, "crew's order" holds `config.services.boomdrive.settings.control_plane.crew`, for `crewOrder`, to `{ host = "host-a"; partitions = [ { label = "garden"; state_repository = "example/garden"; } { label = "home"; state_repository = "example/state"; } ]; }`, on each platform.

**1.2** No code beyond 1.1's. The case: `crewFleet` with crew on, beside `settings.crew` with its `control_plane.crew` replaced by `{ host = "host-a"; partitions = [ { label = "garden"; state_repository = "example/garden"; } ]; }`. Its `settings.control_plane.crew` is that table, and `writes`/`runs` holds the file to `render` of its `settings`.

**1.3** No code beyond 1.1's. The cases: `settings.hosted`, and `{ }`, each with crew on, host `host-a` and `crewModule`. Each fails no assertion, and its file is `files/hosted.toml` or `files/empty.toml`.

**2.1** The assertion in `options.nix`'s `assertions`: `!(crewPlane && cfg.settings.control_plane.crew == null)`, where `crewPlane` is `(cfg.settings.control_plane.type or null) == "crew"`. The case: a system of `crewUnwritten` with crew off. Its failures name `settings.control_plane.crew` and `services.crew`.

**2.2** The assertion: `!(crewPlane && written != null && cfg.settings.control_plane.crew == written && written.partitions == [ ])`. Its words name `services.crew.host`, `crew.host`, and the partitions' machines joined by `, `. The case: `crewUnwritten` with crew on, host `host-a` and `crewElsewhere`. Its failures name `services.crew.host`, `host-a` and `host-b`.

**3.1** In the README's **Installing with nix**:

- The example's `control_plane` becomes `control_plane.type = "crew";`.
- The sentence before it says that, where `control_plane.type` is `crew` and no `control_plane.crew` is written, the module writes it from crew's partitions: `host` from `services.crew.host`, and each partition whose `machine` is that host, by its `label` and `state`. It says a table written out wins, whole.

Move `installing` and `nixBlocks` from `cache.nix` to `lib.nix`, and have `cache.nix` read them from `t`. "The README's crew table" in `tools.nix`:

- some block of `t.nixBlocks` has the infix `type = "crew"`;
- none has `state_repository`;
- `t.installing` has `services.crew.host` and `written out wins`.

**3.2** nixfmt, statix and deadnix over the files the change touched, as the project's own tasks run them.

**3.3** `openspec archive` writes the delta's requirements into `openspec/specs/service-modules/spec.md`, and the change moves under `archive/`.
