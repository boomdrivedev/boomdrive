# Proposal

## Why

flume's record 0069 was accepted by the user on 2026-11-14. Under it, a stage whose kind hibernates it ends its agent once its prompt cache no longer pays. A caller's `flume resume`, a message or the answer it waits on then resumes that agent's own session (flume 212, 213). A resume carries the session from its transcript. At each session's start, under any home, Claude Code deletes every transcript older than that home's `cleanupPeriodDays`, thirty days where none is set. So a stage hibernated past thirty days starts afresh, and its conversation is lost (flume C.4, S127). flume keeps no copy of a transcript, since the host keeps them. Record 0069 names this repository for it: "the host's setup sets `cleanupPeriodDays` in every home".

No home on the user's hosts sets it today. On deneb on 2026-11-11 the oldest transcript was last written on 2026-10-13 (record 0069's sources). afterthought-design ruled on 2026-11-14, relayed by boomdrive-operator: wherever the set installs Flume, Claude Code keeps every session's transcript 365 days unless the host names another span. The set does it by a document of its own in Claude Code's `managed-settings.d`, beside `10-boomdrive-telemetry.json`, holding `cleanupPeriodDays` alone. deneb writes about 2.7 GB of transcripts every 30 days.

The set already writes one document of Claude Code's managed settings, `10-boomdrive-telemetry.json` (record 0013 as amended on 2026-11-12). Every session on the host reads that folder, whatever its account and configuration home, and no user or project setting overrides a key it sets. A second document beside it reaches every home at once, a new account's among them, and is written into no home (core.46).

This change is the unit `the-set-keeps-claude-codes-transcripts-a-year` of the bolt `claude-code-keeps-transcripts-a-year`.

## What Changes

- **One option.** `services.boomdrive.claude-code.cleanupPeriodDays` is named as Claude Code names its key. It is a whole number of days, `365` by default. A span below one day fails evaluation, as Claude Code refuses one.
- **One more document of Claude Code's managed settings.** Wherever `services.flume.enable` is on, the modules write `10-boomdrive-transcripts.json` beside `10-boomdrive-telemetry.json`. That is `/Library/Application Support/ClaudeCode/managed-settings.d` on nix-darwin and `/etc/claude-code/managed-settings.d` on NixOS. It is a regular file, mode 0644, and holds `cleanupPeriodDays` alone, as a JSON number.
- **With Flume.** It is written wherever Flume is on, with the runner or without, and whether or not a collector is named. Where Flume is off, NixOS declares no entry for it, and each nix-darwin activation removes it where it stands.
- **Activation.** nix-darwin's root step runs the same `activate.sh managed` once for each document. The step does not change, and its header names both documents.
- **The telemetry document's requirements** allow the transcripts document beside it. The modules still write no other file of Claude Code's.
- **Words.** The README names the option, the document and its two folders. It says that the document comes with Flume, and that a host's own document named after it sets the key over it. CLAUDE.md and `openspec/config.yaml` say the modules write it wherever they install Flume.

Record 0046 and core.18's and core.46's details are written in this change's own commit (design Decision 9).

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `service-modules`: wherever Flume is on, the modules write Claude Code's managed transcripts document, holding `cleanupPeriodDays` alone, 365 days unless the host names another span. The telemetry document's requirements, and the modules' hold on Claude Code's files, allow it.

## Impact

- `nix/modules/claude-code.nix`: the option, its document's name, and the document as a function of it.
- `nix/modules/darwin.nix` and `nix/modules/nixos.nix`: nix-darwin's root line and NixOS's `/etc` entry.
- `nix/modules/activate.sh`: its header names both documents. Its steps do not change.
- `nix/tests/claude-code.nix` and `nix/tests/tools.nix`.
- `README.md`, `CLAUDE.md` and `openspec/config.yaml`.
- The runner's code does not change, and runs no Claude Code (core.1, core.13). No crate, tool of the gate or tracked file type is added (core.7, core.8).
- Flume's statement does not change. C.4 has the host set `cleanupPeriodDays` for every home, longer than any hibernation lasts. The set is how a host of the set does that.
- **Disk.** A host keeps a year of transcripts, about twelve times what thirty days keep. On deneb that is about 33 GB.
- **Outside this repository.** afterthought/swancloud takes the document with its pin. A host that sets `cleanupPeriodDays` in a home's settings or in its own `managed-settings.json` finds the set's document set over both. It names its span in the option instead.

## Touches

```
nix/**
README.md
CLAUDE.md
openspec/config.yaml
openspec/changes/the-set-keeps-claude-codes-transcripts-a-year/**
docs/adr/0046-the-set-keeps-claude-codes-transcripts-a-year-wherever-it-installs-flume.md
docs/architecture/core.md
```
