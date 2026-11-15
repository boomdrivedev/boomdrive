# Design

## Context

core.57, as amended on 2026-11-14 (#210, 6808fbf3), gives the host's one collector to Flume and sluice: "the collector the host names for its Claude Code sessions, `services.boomdrive.claude-code.telemetry.collector`, is sluice's and Flume's collector, each turned on to send to it through its own module's telemetry settings, while which of its signals each tool sends stays that tool's setting". Its details add: "Also rules out: a host's configuration naming sluice's or Flume's collector apart from its Claude Code sessions' where the set names one, or a tool of the set sending to a collector other than the one the host names (core.28; flume record 0067)." The decision is boomdrive-design's, from its talk with the user of 2026-11-13 on telemetry for Flume and sluice (flume record 0067, its Deciders).

What each tool reads, at the tags the set pins:

- **Flume, v0.24.0.** Its module's `services.flume.settings` is Flume's managed file, `~/.config/flume/config.toml`. Its `telemetry` is `null` unless given, a table of `send`, `collector`, `service`, `traces`, `metrics`, `logs` and `attributes`, each `null` unless given (flume C.7). Flume sends nothing unless `send` is true. A table that sends must name a collector: `http://`, then `localhost`, an address of `127.0.0.0/8` or `[::1]`, a port and at most a path, which the module's type holds. `traces`, `metrics` and `logs` are each on where unset. Flume sends to `<collector>/v1/<signal>`, OTLP over HTTP, protobuf, and takes every `OTEL_EXPORTER_OTLP_*` out of its environment first (flume 216, record 0067).
- **sluice, v0.86.0.** Its module's `services.sluice.settings` is sluice's file, `~/.config/sluice/config.toml`. Its `telemetry` is `null` unless given, a table of `send`, `collector`, `service`, `traces`, `metrics`, `logs`, `logs_tail_bytes` and `attributes`. `send` true needs a `collector`, any `http://` address, and sluice refuses a table that sends and names none. `traces` and `metrics` are on where unset. `logs` is `"summary"` where unset: which task of a failed check failed and where its output is kept, never the output. sluice also sends to `<collector>/v1/<signal>` (sluice 151).
- **Neither tool** drops a trailing `/` from the collector before it joins `/v1/<signal>`.

The flake today:

- **The collector.** `nix/modules/claude-code.nix` declares `services.boomdrive.claude-code.telemetry`: `collector`, `null` or an `http://` address of `localhost`, `127.0.0.1` or `[::1]` with a port and at most a path; and `metrics`, `logs` and `traces`, Claude Code's switches. Its `documentOf` drops one trailing `/` from the collector and joins `/v1/<signal>` for each signal on. `darwin.nix` and `nixos.nix` write the document only with the runner on and a collector named.
- **The other tools' options.** `tools.nix` sets each tool's `package`, and with the runner turns Flume's and sluice's modules on, a host's `false` winning. Its header holds that it sets no key of a tool's `settings`. `drives.nix` gives Flume each drive's stream, and `sources.nix` gives Flume crew's source, each at the module's default priority, so a host's own value wins key by key.
- **The spec.** The service-modules spec has each of Flume's, sluice's and crew's files written by that tool's own module. *The tools' activation is their own* compares a system of the runner's with one of the tools' own modules alone, given the same settings.

Every address the set's collector type takes is one Flume's type takes, and sluice takes any `http://` address.

## Goals / Non-Goals

**Goals:**

- With the runner on and a collector named, Flume and sluice each send their own telemetry to that collector, through their own modules, with no setting of the host's beside it.
- Which of its signals each tool sends, and as what, stays that tool's own setting.
- No tool of the set sends to a collector other than the host's one.

**Non-Goals:**

- **What a tool sends.** Each tool's spans, records, metrics and logs are its own (flume 216 to 220; sluice 151).
- **The runner's own export.** The runner exports nothing yet (record 0013, its Consequences). This change does not add it.
- **crew, timber, Pending You or Plannotator.** None of them has a telemetry setting the set could give.
- **A backend.** The set names no backend, and the collector chooses where each signal goes (record 0013 as amended on 2026-11-12).

## Decisions

### 1. One module of the set's, beside sources.nix

A new module, `nix/modules/collector.nix`, gives Flume's and sluice's `settings.telemetry` and holds the assertion of Decision 6. `darwin.nix` and `nixos.nix` import it beside `sources.nix`, and each header names it.

Not taken:

- **`tools.nix`.** It sets packages and switches, and its header holds that it sets no key of a tool's `settings`.
- **`claude-code.nix`'s module.** It is Claude Code's document and options. The tools' settings are another tool's, as `sources.nix` gives Flume crew's.

### 2. With the runner on and a collector named

The module gives the tools' keys under the same condition as the telemetry document: `services.boomdrive.enable` on and `services.boomdrive.claude-code.telemetry.collector` set. Otherwise it gives no key of either tool's `telemetry`, and each stays `null`, as without the set.

It gives the keys whether or not each tool's module is on. A tool's module that is off renders nothing of its `settings`, so the keys do nothing there.

Not taken:

- **Whenever a collector is named.** That would send Flume's and sluice's telemetry where the host's sessions send none.
- **Only where each tool's module is on.** That adds a condition that changes nothing a host gets.

### 3. `send` and `collector`, and no other key

The module gives each tool two keys, each by `lib.mkDefault`:

- `services.flume.settings.telemetry.send = true` and `services.sluice.settings.telemetry.send = true`;
- `services.flume.settings.telemetry.collector` and `services.sluice.settings.telemetry.collector`, each the host's collector by Decision 4.

Each key is given apart, as `sources.nix` gives crew's source, so a key a host sets in a tool's own `telemetry` merges beside the module's. A host's `send = false` there turns that tool's telemetry off, and wins.

Not taken:

- **The whole table by `lib.mkDefault`.** Any key a host set in it, such as `traces`, would drop the module's whole table.
- **`send` pinned true.** The rule turns each tool on and leaves which of its signals it sends to the tool. A host that wants none of a tool's signals says so in that tool's own setting.

### 4. One base for all three

`claude-code.nix` gives `baseOf`, the collector with one trailing `/` dropped, as `documentOf` computes it today. `documentOf` takes it, and `collector.nix` gives each tool `baseOf` of the host's collector. So the sessions, Flume and sluice send to the same `<base>/v1/<signal>`, and neither tool joins `//v1`.

Not taken: the host's address as written, which sends a tool's signals to `//v1/<signal>` where the host names a trailing `/`.

### 5. Each tool keeps its signals

The module gives neither tool `traces`, `metrics`, `logs`, `logs_tail_bytes`, `service` or `attributes`. Each tool sends what its own defaults and the host's own settings for it say:

- Flume sends its traces, metrics and logs, and names itself `flume`.
- sluice sends its traces, its metrics and its `summary` logs, and names itself `sluice`.

Claude Code's `metrics`, `logs` and `traces` switches reach Claude Code's document alone.

Not taken: Claude Code's switches given to the tools. They are one program's choices, and they would turn Flume's traces off wherever Claude Code's beta traces are off.

### 6. One collector, by an assertion

Where the module gives the keys, evaluation fails unless each tool's `collector`, its trailing `/` dropped, is the host's base. That holds whatever priority the host gives the tool's `collector`, and covers a host's `null` there, which leaves a table that sends with no collector. The failure names the tool's option and `services.boomdrive.claude-code.telemetry.collector`, with both values.

A host naming the same address in a tool's own settings, with or without its trailing `/`, evaluates.

Not taken:

- **The `collector` at the module's own priority,** so a host's other value conflicts. nixpkgs' failure for that tells the host to use `lib.mkForce`, which then sends the tool elsewhere, as core.57 rules out.
- **The host's own collector winning,** which core.57 rules out.

### 7. Tests

A new file, `nix/tests/collector.nix`, holds the delta's scenarios, each on nix-darwin and on NixOS. `nix/tests/default.nix` imports it. Its cases' names are the delta's scenarios', none of them a name `claude-code.nix` holds, since `default.nix` merges every file's cases by name.

- **Systems.** It evaluates `t.darwinSystem` and `t.nixosSystem` with a collector named. For the runner off, it evaluates `t.darwinOf` and `t.nixosOf` of the modules, with Flume and sluice turned on.
- **What it reads.** It reads `config.services.flume.settings.telemetry` and `config.services.sluice.settings.telemetry`, each with its `null` keys dropped. A failure is read through `t.failures`, by words as `claude-code.nix`'s `failsNaming` reads them.
- **The tools' own files.** *The tools' telemetry is their own* sets a system of the runner's, with crew and a collector, beside one of the tools' own modules alone, given the same `telemetry` keys. On nix-darwin it compares their activations' lines, less what *The tools' activation is their own* drops. On NixOS it compares Flume's and sluice's user activation snippets. Each tool's activation links the file its module rendered, at a store path made from its settings, so equal lines show that each tool's module wrote the same `[telemetry]`.
- **The tools' own systems.** `nix/tests/lib.nix` gives `toolsDarwinOf` and `toolsNixosOf`, each the tools' own system with one module of extra settings beside it. `toolsDarwinSystem` and `toolsNixosSystem` are each of `{ }`.
- **The README** is read through `t.installing` and `t.nixBlocks`.

The tests use core.41's neutral names.

### 8. The words, and the rules written now

Written in this change's own commit, by this stage:

- **Record 0013's amendment of 2026-11-15.** It records boomdrive-design's decision and Decisions 2 to 6, each with the path not taken.
- **core.57's details.** They allow a host's `send` and signals in a tool's own `telemetry`, the same collector named there again, and no tool's telemetry given where the runner is off or no collector is named. Their sources add record 0013 as amended on 2026-11-15.

The build writes:

- **The collector option's description.** It says the collector is Flume's and sluice's too, with the runner on, and that each tool's signals stay its own settings.
- **The README.** Under **Installing with nix**, after the telemetry document's part and before the part on its absence:
  - that the same collector is Flume's and sluice's, each turned on to send to it through its own `services.flume.settings.telemetry` and `services.sluice.settings.telemetry`;
  - that which of its traces, metrics and logs each tool sends stays that tool's own setting, at its own defaults;
  - that `send = false` there turns that tool's telemetry off;
  - that a tool's `collector` other than the host's fails evaluation.
- **CLAUDE.md and `openspec/config.yaml`.** They say that with the runner and a collector the modules also turn Flume's and sluice's telemetry on to it, by `services.flume.settings.telemetry` and `services.sluice.settings.telemetry`.

## Risks / Trade-offs

- **More goes to the collector.** A host that names a collector for its sessions now gets Flume's and sluice's telemetry too, every signal of each by its defaults, at its next switch. A host that wants less sets a tool's signals or `send = false`.
- **What each tool sends is that tool's rules'.** Flume sends no words or paths (flume record 0067). sluice's `summary` logs say where a failed check's output is kept, and its `tail` sends the output's end, scrubbed (sluice 151). The set turns neither further on. The collector is on the host and chooses what reaches a backend.
- **A host's `null` table.** A host that sets `services.flume.settings.telemetry = null`, or sluice's, where the module gives keys, fails evaluation: nixpkgs refuses an option defined both `null` and not. Such a host sets `send = false` instead.
- **A tool that cannot reach the collector** loses that signal and changes nothing it does (flume 216; sluice 151).
- **No live collector in the gate.** The gate holds the settings and each tool's rendered file, not what a collector receives. A host reads it at its collector.

## Migration Plan

No host of the set gives Flume or sluice a `[telemetry]` from the set before this change.

- **A host that names a collector** gets Flume's and sluice's telemetry on at its next switch.
- **A host that names none** gets nothing.
- **A host that names the same collector in a tool's own settings** evaluates as before, and may drop its own.
- **A host that names another collector in a tool's own settings** fails evaluation until it drops that, or names that collector as the host's.
- **On the user's hosts,** afterthought/swancloud takes the change with its pin. It drops any `telemetry` of Flume's or sluice's of its own beside the set's, so one collector holds.
- **Rolling back** is naming no collector, or a pin before this change. Each tool's module renders its file again without `[telemetry]`, and each tool sends nothing.

## Task notes

- **1.1.** `baseOf` is `lib.removeSuffix "/"`, as `documentOf`'s `base` is today. `documentOf` calls it in place of its own `base`.
- **1.2.** For the runner off, Flume's and sluice's modules want their user on NixOS: the case gives `services.flume.user` and `services.sluice.user` `"alice"`, as `t.toolsNixosSystem` does. On nix-darwin, `t.darwinOf` gives the primary user.
- **1.2 and 1.3.** A tool's `telemetry` value holds every key its module declares, `null` where none is given. The cases drop the `null` keys with `lib.filterAttrs` before comparing, so *the module gives no other key* reads as the whole of what is left.
- **1.4.** The assertion's message names the tool's option by `lib.showOption`, the tool's value and the host's, so `failsNaming` finds `services.flume.settings.telemetry.collector` or `services.sluice.settings.telemetry.collector`, and `services.boomdrive.claude-code.telemetry.collector`.
- **1.5.** `toolsDarwinOf` and `toolsNixosOf` pass their module through `toolsOf`'s `extra`. `toolsNixosOf` imports it beside the module that gives each tool `alice`.
