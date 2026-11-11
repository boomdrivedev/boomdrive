# Design

## Context

Record 0034, accepted on 2026-11-09, point 2: "Flume's sources are Flume's module's, and crew's source is declared by the set." A host's sources of kinds are Flume's managed layer, one file `~/.config/flume/sources/<name>.toml` each, beside `config.toml` (Flume rule core.45, record 0040). Flume's module writes them from options named as the file names them. "Where crew is in the set, the Boomdrive module declares crew's source through Flume's module, naming the roles file crew writes and its kinds folder (crew record 0057, point 1). Not taken: crew declaring its own source, which would write into Flume's folder; a host keeping its own writer of Flume's managed layer."

crew record 0057, point 1: "crew writes `~/.local/state/crew/flume/roles.toml`, and nothing else does. […] The fleet declares the source alone, `sources/crew.toml` naming this file and the kinds folder." Point 5 names the kinds folder, `~/.local/share/flume/kinds/crew`. crew's `plugin/lib/source.py`, at the set's tag v0.50.2 and on crew's main, names both under the home of the user crew runs as:

```python
ROLES_FILE = Path.home() / ".local/state/crew/flume/roles.toml"
KINDS_DIR = Path.home() / ".local/share/flume/kinds/crew"
```

crew writes the roles file only on a host where one of its partitions keeps a main level. It makes the kinds folder only when the file has an entry.

Flume's module at the lock's tag, v0.21.2 (`65911c2`), declares `services.flume.sources`, an attribute set of sources by name (Flume's change `flumes-module-writes-the-hosts-sources`, landed as #38 on 2026-11-09):

- Each source is a table with `kinds_dir`, required, and `roles`, null unless set. Each is an absolute path or one under `~`, and any other key is free-form, for Flume to judge.
- Each source is rendered at evaluation as `flume-source-<name>.toml`, and linked at `~/.config/flume/sources/<name>.toml` by the activation's sources step, run as the user after the link of `config.toml`. A file there that is no link is moved aside once. The step removes only the links it made.
- A source's name is ASCII letters, digits, `_` and `-`, and not `flume`, `repo` or `launch`. A string shaped as a credential fails evaluation.

Flume's configuration reference at that tag says a source's file may give only `kinds_dir` and `roles`. A `kinds_dir` that does not exist is no fault of the file, though a launch naming the source is refused, naming the folder as missing. A roles file that cannot be read declares no role.

The flake today:

- `nix/modules/darwin.nix` and `nixos.nix` import `options.nix`, `tools.nix`, `apps.nix` and `drives.nix`, and Flume's, sluice's and crew's own modules come with them (flake.nix's `modulesOf`).
- `tools.nix` sets each tool's `package`, and Flume's and sluice's `enable` with the runner's.
- `drives.nix` gives Flume's `settings.streams` and `settings.repos` from the runner's drives, each by `lib.mkDefault`, under `services.boomdrive.enable`.
- Nothing gives `services.flume.sources`.
- `nix/tests/lib.nix`'s `toolsDarwinSystem` and `toolsNixosSystem` turn on the tools' own modules alone, crew among them, for "The tools' activation is their own", which compares their activations with the runner's system's.

## Goals / Non-Goals

**Goals:**

- A host that takes the set with crew on has crew's source declared in Flume's module, and writes no file for it.
- The source names the roles file and kinds folder the set's crew writes.
- A host's own value for a key of the source wins.

**Non-Goals:**

- crew's roles file and kinds folder, which crew writes (crew record 0057). The set writes neither and makes neither folder.
- Any other source of kinds. A host declares its own through `services.flume.sources`.
- crew's kept repositories, which record 0036 gives crew through its own module in the unit `the-set-gives-crew-each-kept-repositorys-folders`.
- herdr's settings, the capture file, sluice's telemetry and restarts (record 0034, points 1 and 3 to 5).

## Decisions

### 1. `nix/modules/sources.nix` declares crew's source

A plain module, `{ config, lib, ... }:`, taking no `packages`, imported as `./sources.nix` by `darwin.nix` and `nixos.nix` beside `./drives.nix`. Its header comment says what it gives Flume's module, cites record 0034, point 2, and crew record 0057, point 1, and says it writes no file (core.18, core.46). Its config:

```nix
config = lib.mkIf config.services.crew.enable {
  services.flume.sources.crew = {
    kinds_dir = lib.mkDefault "~/.local/share/flume/kinds/crew";
    roles = lib.mkDefault "~/.local/state/crew/flume/roles.toml";
  };
};
```

Not taken:

- **Declaring it in `tools.nix`.** That file sets each tool's package and enable, and says it gives "no key of a tool's `settings`". A file named for Flume's option keeps each file to one thing, as `drives.nix` does for Flume's streams.
- **Declaring it in `drives.nix`.** That file gives Flume what the runner's drives say, under the runner's `enable`. crew's source is no fact of a drive.

### 2. With crew's module on, whether or not the runner is

The source is declared where `services.crew.enable` is true, and nowhere else. crew's roles run in Flume wherever crew and Flume both run, and the runner is not needed for that. A host that runs crew and Flume from the set with the runner off would otherwise write the source itself, which record 0034, point 2, rules out. With Flume's module off, the source is declared and nothing writes it, as with any of Flume's options.

Not taken:

- **Under `services.boomdrive.enable` too, as `drives.nix` gives its values.** That would tie crew's source to the runner, which neither record asks for.
- **Only where one of crew's partitions names this host as its `machine`.** crew already writes the roles file only on such a host. Flume reads a roles file that cannot be read as declaring no role, and a missing kinds folder is no fault of the source's file. The condition would copy crew's own into the set, to save a source that declares nothing.

### 3. The source `crew`, its two keys, each under `~`

The source's name is `crew`, the name crew's record and its code give the file, `sources/crew.toml`. It holds `kinds_dir` and `roles` and no other key, since Flume's file takes no other. Each path is written under `~`, as crew names it under `Path.home()`. Flume expands `~` to the home of the user it runs as. On nix-darwin, Flume and crew both run as the primary user. On NixOS, `nixos.nix` gives both the runner's user by default.

Not taken:

- **Absolute paths built from the user's home.** That takes a different lookup on each platform: the primary user's home on nix-darwin, and crew's user's on NixOS. It helps only a NixOS host that gives crew and Flume two users, where Flume would then read another user's home. Such a host sets its own value (Decision 4).

### 4. Each key by `lib.mkDefault`

Each key is given at the options' default priority, as `drives.nix` and `tools.nix` give theirs, so a host's own value in `services.flume.sources.crew` wins key by key. This covers a host whose crew runs under another user, and a host that follows its own crew input at a commit that moves the paths (record 0034, point 7). The source's other options are left as Flume's module has them.

### 5. The paths are named in the module and held to crew's by a test

The module names the two paths as text. A case of `nix/tests/sources.nix` reads `plugin/lib/source.py` from the lock's crew, `t.inputs.crew`, and finds the lines `ROLES_FILE = Path.home() / "<path>"` and `KINDS_DIR = Path.home() / "<path>"`. It holds the source's `roles` and `kinds_dir` to be `~/<path>` of each. A crew tag that moves either path fails the gate in the change that moves the lock (core.45). A file with no such line fails the case, naming the line it lacks.

Not taken:

- **Reading the paths from crew's source at evaluation, in the module.** Every host's evaluation would then turn on the text of crew's Python, and the module would need crew's source passed in beside `packages`.
- **An option of crew's module giving the paths.** No record asks crew for one. Record 0034, point 2, gives the declaration to the set, and crew record 0057, point 1, gives it to the fleet.

### 6. The tests

`nix/tests/sources.nix`, added to `default.nix`'s `test`, runs each case on nix-darwin and on NixOS, as `drives.nix` does, unless it names neither:

- "crew's source with crew on", "No source of crew's with crew off" and "A host's own kinds folder for crew" read `config.services.flume.sources`. They read the activation as `tools.nix` reads it: on nix-darwin, the text of `system.activationScripts.postActivation`; on NixOS, the `flume` snippet of `system.userActivationScripts`. Each looks for the rendered file's name, `flume-source-crew.toml`.
- "crew's source without the runner" evaluates `t.darwinOf` and `t.nixosOf` with the platform's `default` module and crew on, the runner left off, as `drives.nix`'s "With the runner off" does.
- "The paths the set's crew writes" is one case, of no platform: it compares the nix-darwin system's source with crew's `source.py` (Decision 5).
- "The README's words on crew's source" reads `t.installing` and `t.nixBlocks`.

`nix/tests/lib.nix`'s `toolsOf` gives Flume's own module `sources.crew` with the same two values, so `toolsDarwinSystem` and `toolsNixosSystem` declare what the runner's system declares. "The tools' activation is their own" then still holds: the Boomdrive module adds to the tools' activations only the runner's link and herdr's plugins' links.

Every case uses core.41's neutral names: the host `host-a`, `settings.crewModule`'s partitions, and the folder `/srv/kinds/crew`.

### Rules relied on

- **core.18, core.46**: the Boomdrive module sets Flume's module options and writes no file of Flume's. Flume's module renders and links the source.
- **core.45**: the set's crew and Flume are the lock's tags, which a test holds the paths to.
- **core.5**: the runner's file names no source, kind or stream, and does not change.
- **core.13**: nothing is started. Flume reads the sources at its next command.
- **core.7, core.8, core.41**: no tool, crate or file type is added, and the tests use neutral names.

### Rules amended

- **core.57** names crew's source among the facts the set gives through a tool's module option, from another tool's own layout, citing record 0034, point 2. Its details rule out a host's own `sources/crew.toml`, or a writer of it, beside the set. They allow a host's own key set through Flume's option. They cite crew record 0057, point 1. The amendment is written in the commit that writes this change.

### Rules departed from

None.

## Dependencies

- Flume's `services.flume.sources`, from Flume's change `flumes-module-writes-the-hosts-sources`. It is in v0.21.2, the tag the lock names (`65911c2`), so no input moves.
- crew's roles file and kinds folder at v0.50.2, the tag the lock names, which crew record 0057 placed.

## What this change asks of others

Nothing of another repository. A host that takes the set with crew on drops its own `~/.config/flume/sources/crew.toml`, and any writer of it, in the switch that takes this change. Flume's module moves a file of the host's own there aside once.

## Risks / Trade-offs

- **A host whose crew and Flume run as two users on NixOS** gets a source under Flume's user's home, where crew writes nothing. Flume then declares no role of crew's and refuses a launch naming the source. The host sets its own `roles` and `kinds_dir` (Decision 4).
- **A host that follows its own crew input** at a commit that moves the paths gets the set's paths, which the test checks only at the set's crew. The host sets its own (Decision 4; record 0034, point 7).
- **A file of the host's own at `~/.config/flume/sources/crew.toml`** is moved to `crew.toml.before-nix` by Flume's module at the first switch, and the module's link takes its place. Where that file named the same two paths, nothing changes for Flume. Where `crew.toml.before-nix` already stands, Flume's sources step fails naming both, links the other sources, and stops nothing else of the switch; the host removes one of them and switches again.

## Migration Plan

Each host takes the change at its next switch. On a host with crew on, Flume's activation links `~/.config/flume/sources/crew.toml`, and Flume reads it at its next command. Rolling back the flake's revision removes the link, since the sources step removes only its own links. A host that wrote the file by hand moves `crew.toml.before-nix`, where Flume's module moved it, back into place.

## Open Questions

None.

## Task notes

**1.1** `nix/modules/sources.nix` as Decision 1's sketch, its header comment in `drives.nix`'s manner. `darwin.nix` and `nixos.nix` add `./sources.nix` to `imports` after `./drives.nix`, and their header comments' sentence on `drives.nix` gains "and sources.nix declares crew's source to Flume's module (record 0034)". `nix/tests/sources.nix` is added to `default.nix`'s `test`, after `./drives.nix`. Its `platforms` follow `drives.nix`'s, `system` and `off`, plus an `activationOf` that gives Flume's activation text as `tools.nix` reads it (Decision 6). crew is turned on as `tools.nix`'s `crewOf settings.crewModule` turns it on: `services.crew` enabled with `host = "host-a"` and `settings = settings.crewModule`. `lib.nix`'s `toolsOf` adds `sources.crew` to its `services.flume`, with the two values of Decision 1 written as plain strings, and its comment says so. "crew's source with crew on" holds the source to the attribute set `{ kinds_dir = …; roles = …; }`, holds `flume-source-crew.toml` in the activation, and holds `t.failures` empty. "No source of crew's with crew off" turns on the runner alone and holds `services.flume.sources ? crew` false and `flume-source-crew.toml` out of the activation. "crew's source without the runner" holds the same attribute set.

**1.2** In `nix/tests/sources.nix`, a function of a line's prefix, `ROLES_FILE` or `KINDS_DIR`, finds the line of `builtins.readFile "${t.inputs.crew}/plugin/lib/source.py"` that matches `<prefix> = Path.home\(\) / "([^"]+)"`, by `builtins.match` over `lib.splitString "\n"`. It gives `~/` and the path, or null where no line matches. The case fails naming the line where it is null, and naming both values where they differ.

**1.3** "A host's own kinds folder for crew" sets `services.flume.sources.crew.kinds_dir = "/srv/kinds/crew"` beside crew on, and holds `kinds_dir`, `roles` and `t.failures` as the scenario says.

**1.4** statix refuses a key repeated in one attribute set, so each of `sources.nix`'s values sits under one `services.flume.sources.crew = { … };`. deadnix refuses an argument the module does not read, so `sources.nix` takes `config` and `lib` alone.

**2.1** The README's **Installing with nix**, after the sentence "a host that runs crew turns on crew's, with its `host` and its partitions.", says that with crew's module on the module declares crew's source to Flume, `services.flume.sources.crew`, naming the roles file crew writes, `~/.local/state/crew/flume/roles.toml`, and crew's kinds folder, `~/.local/share/flume/kinds/crew`. It says Flume's module writes the source at `~/.config/flume/sources/crew.toml`, so a host writes no file for it, and a key the host sets there wins. No nix block gains `services.flume.sources`. "The README's words on crew's source" is a case in `nix/tests/sources.nix`, of no platform.

**2.2** CLAUDE.md's paragraph on the flake, after "each drive's stream given to Flume's module", gains ", and, with crew on, crew's source of kinds (record 0034)".
