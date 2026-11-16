# Proposal

## Why

A host of the set names one collector, `services.boomdrive.claude-code.telemetry.collector`, and every Claude Code session on it exports there (record 0013 as amended on 2026-11-12). Flume and sluice each send their own telemetry, OTLP over HTTP, protobuf, to a collector on the host, once their own `[telemetry]` turns it on (flume 216 to 220 and record 0067; sluice 151). The set gives neither one today, so a host that wants a place's trace, sluice's runs and its sessions in one backend writes the same address three times, and each tool sends nothing until it does.

core.57, amended on 2026-11-14 (#210, 6808fbf3), makes the host's collector sluice's and Flume's: "the collector the host names for its Claude Code sessions, `services.boomdrive.claude-code.telemetry.collector`, is sluice's and Flume's collector, each turned on to send to it through its own module's telemetry settings, while which of its signals each tool sends stays that tool's setting". It also rules out "a host's configuration naming sluice's or Flume's collector apart from its Claude Code sessions' where the set names one, or a tool of the set sending to a collector other than the one the host names". The decision is boomdrive-design's, from its talk with the user of 2026-11-13 on telemetry for Flume and sluice (flume record 0067). This change builds it.

Once it is built, a place's trace, sluice's spans of a check run under the stage that ran it, and the stage's agent's own turns reach one collector. Flume hands a stage's start to its agent in `TRACEPARENT`, and sluice nests under it (flume record 0067).

## What Changes

- **The host's collector reaches Flume and sluice.** With the runner on and a collector named, the Boomdrive module gives Flume's own module `services.flume.settings.telemetry` and sluice's `services.sluice.settings.telemetry`, each `send = true` and `collector`, the host's address with a trailing `/` dropped. Each is given at the module's default priority, and no other key is given. Each tool's module writes its own `[telemetry]` from them, Flume's into its managed layer and sluice's into its file.
- **Each tool keeps its signals.** Neither tool's `traces`, `metrics`, `logs`, `service` or `attributes` is given, so each sends what its own defaults and the host's own settings for it say. Claude Code's `metrics`, `logs` and `traces` switches reach Claude Code's document alone.
- **One collector.** A tool's `collector` that is not the host's, its trailing `/` dropped on both sides, fails evaluation, naming the tool's option and `services.boomdrive.claude-code.telemetry.collector`. That holds whatever priority the host gives it, `lib.mkForce` included. A host's `send = false` in a tool's own settings turns that tool's telemetry off, and is kept.
- **Nothing otherwise.** With no collector named, or with the runner off, the module gives neither tool any key of `telemetry`.
- **One base.** `nix/modules/claude-code.nix` gives `baseOf`, the collector with its trailing `/` dropped. The Claude Code document and both tools take it, so all three send to the same `/v1/<signal>`.
- **Words.** The collector option's description, the README's telemetry part, CLAUDE.md and `openspec/config.yaml` say that the collector is Flume's and sluice's too, and that each tool's signals stay its own settings.

Record 0013's amendment of 2026-11-15 and core.57's details are written in this change's own commit (design Decision 8).

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `service-modules`: with the runner on and a collector named, the modules turn Flume's and sluice's own telemetry on to that collector. Each tool keeps which of its signals it sends, and evaluation fails where a tool's collector is not the host's. Flume's and sluice's `[telemetry]` is each written by that tool's own module.

## Impact

- `nix/modules/collector.nix`, new: the tools' `telemetry` keys and the assertion.
- `nix/modules/claude-code.nix`: `baseOf`, and the collector option's description.
- `nix/modules/darwin.nix` and `nix/modules/nixos.nix`: each imports `collector.nix` beside `sources.nix`, and its header names it.
- `nix/tests/collector.nix`, new, and `nix/tests/default.nix`. `nix/tests/lib.nix` gives the tools' own systems with extra settings.
- `README.md`, `CLAUDE.md` and `openspec/config.yaml`.
- The runner's code does not change, and starts no tool's process (core.1, core.13). No crate, tool of the gate or tracked file type is added (core.7, core.8).
- Neither Flume's nor sluice's statement changes. Flume's 216 and sluice's 151 leave `[telemetry]` to the host's own settings, and the set gives it as the host's (core.57).
- **On a host.** A host that names a collector finds Flume and sluice sending to it at its next switch. Flume sends a place's trace, its records and its metrics. sluice sends its traces, its metrics and, as logs, which task of a failed check failed and where its output is kept, never the output. A host that wants less sets a tool's own signals or `send = false`.
- **Outside this repository.** afterthought/swancloud takes the change with its pin. A host that already names the same collector in a tool's own settings keeps evaluating, and one that names another fails until it drops it.

## Touches

```
nix/**
README.md
CLAUDE.md
openspec/config.yaml
openspec/changes/the-hosts-collector-reaches-sluice-and-flume/**
docs/adr/0013-health-goes-by-the-protocol-and-opt-in-otlp-and-nothing-else-leaves-the-host.md
docs/architecture/core.md
```
