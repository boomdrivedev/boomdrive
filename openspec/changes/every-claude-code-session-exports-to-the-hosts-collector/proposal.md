# Proposal

## Why

Record 0013's amendment of 2026-11-12, accepted by the user that day on their card, has every Claude Code session on a drive's accounts export Claude Code's own telemetry to the collector its host names and nowhere else, with none of a session's words in it. The set carries the wiring, so a person turns it on by naming a collector, as for the runner. core.28 reads with it. On the user's hosts today only the Bedrock account's sessions export anything, through afterthought/swancloud's own settings, and none exports traces. The user wants their sessions in Dash0, whose AI SDLC Insights reads a session from its traces. swancloud's record 0061, as amended (1cb0e725), has their hosts' collectors send Claude Code's traces to Dash0 alone. Which backend gets each signal is the collector's choice, and swancloud's. The set names no backend.

The set writes nothing of the kind today. A session's telemetry is set per home, per launcher or per shell. A new account's home, a crew standing role or the person's own terminal each misses it.

Claude Code reads its managed settings, in a system folder of the platform's, in every session on the host, whatever its account and configuration home. No user or project setting overrides a key they set. They take a folder of drop-in documents, `managed-settings.d`, each merged key by key after `managed-settings.json`. So one document of the set's own, holding the telemetry keys alone, reaches every session, and the host's own `managed-settings.json` stays the host's.

This change is the unit `every-claude-code-session-exports-to-the-hosts-collector` of the bolt `claude-codes-telemetry-reaches-the-hosts-collector`.

## What Changes

- **One option.** `services.boomdrive.claude-code.telemetry` names the collector and the signals:
  - `collector`: the collector's `http://` address on the host's own loopback, as sluice's `[telemetry]` names its own. It is `null` by default, and then nothing is written.
  - `metrics`, `logs` and `traces`: whether each signal is exported. Metrics and logs are on by default. Traces, Claude Code's beta, are off unless the host turns them on.

  A collector off the host fails evaluation, and so does a collector with no signal on.
- **One document of Claude Code's managed settings.** With the runner on and a collector named, the modules write `10-boomdrive-telemetry.json` into Claude Code's managed-settings folder. That is `/Library/Application Support/ClaudeCode/managed-settings.d` on nix-darwin and `/etc/claude-code/managed-settings.d` on NixOS. It is a regular file, mode 0644, and holds an `env` of telemetry keys and nothing else:
  - telemetry turned on;
  - each signal on goes to `<collector>/v1/<signal>` as OTLP over HTTP, protobuf, by its own endpoint. A signal off is pinned `none`. No general endpoint is set, so no tool a session runs inherits one;
  - traces, where on, with Claude Code's beta switch;
  - every content setting pinned off: prompts, responses, tool details, tool content, raw API bodies and detailed beta tracing;
  - Claude Code's own account, session and repository attributes turned on, so each session carries its account and repository.
- **Nothing without a collector.** With no collector named, or the runner off, the modules write no document. NixOS drops the file it wrote. On nix-darwin each activation removes the set's own document where it stands, and nothing else in the folder.
- **Activation.** `activate.sh managed` writes the document whole, only where it differs, or removes it. nix-darwin's activation runs it as root.
- **Words.** The README names the option, the two paths, that nothing is written without a collector, and that traces are off unless named. CLAUDE.md and `openspec/config.yaml` say the modules write Claude Code's managed telemetry document where a collector is named.

Record 0013's amendment of 2026-11-12, the set's sessions in Claude Code's managed settings, and core.18's, core.28's and core.46's details are written in this change's own commit (design Decision 11).

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `service-modules`: where the host names a collector, the modules write Claude Code's managed telemetry document for every session on the host. It holds the telemetry keys alone: each signal as the host names it, to the collector on the host and nowhere else, with the content settings off and the account and repository attributes on. Without a collector nothing is written. The modules write no other file of Claude Code's.

## Impact

- `nix/modules/claude-code.nix` (added): the option, its checks, and the document as a function of it.
- `nix/modules/darwin.nix` and `nix/modules/nixos.nix`: the import, nix-darwin's activation step and NixOS's `/etc` entry.
- `nix/modules/activate.sh`: the `managed` step.
- `nix/tests/`: `claude-code.nix` (added), `default.nix`, `activate.sh`, `apps.nix` and `tools.nix`.
- `README.md`, `CLAUDE.md` and `openspec/config.yaml`.
- The runner's code does not change, and runs no Claude Code (core.1, core.13). No crate, tool of the gate or tracked file type is added (core.7, core.8).
- The set installs no Claude Code: Claude Code stays the person's own. A Claude Code that reads no `managed-settings.d` reads no document and exports nothing.
- Outside this repository, in afterthought/swancloud and none of this change's:
  - the collector on each host forwards each signal to its backend, Claude Code's traces to Dash0 alone (its record 0061 as amended);
  - turning traces on for one seat first, and reading Insights in Dash0, is its own;
  - once a host's pin carries this change, it drops its own Claude Code telemetry settings, the Bedrock account's endpoint in `lib/agent-config.nix` among them, so each session has one endpoint per signal.

## Touches

```
nix/**
README.md
CLAUDE.md
openspec/config.yaml
openspec/changes/every-claude-code-session-exports-to-the-hosts-collector/**
docs/adr/0013-health-goes-by-the-protocol-and-opt-in-otlp-and-nothing-else-leaves-the-host.md
docs/architecture/core.md
```
