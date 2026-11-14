# Design

## Context

flume's record 0069, accepted by the user on 2026-11-14, gives a stage whose kind hibernates it a session that outlives its agent:

- **Hibernate.** Once its agent has been quiet for its hibernate time, the life of its prompt cache by default, the agent is ended. Its session is kept for a resume, and its room and account are freed (flume 212).
- **Resume.** A caller's `flume resume`, a message, a role's delivery or the answer it waits on starts one fresh agent carrying the hibernated agent's conversation. Where its transcript is gone, the stage starts afresh, with flume's words that its conversation could not be carried (flume 213).
- **The host keeps transcripts.** flume keeps no copy. C.4: "Every Claude Code start under any home cleans the shared folder of transcripts older than its home's `cleanupPeriodDays`, thirty days where it sets none, so a host whose sessions hibernate sets it in every home, once, longer than any hibernation lasts." S127 is a conversation hibernated forty days on a host that sets none: it starts afresh.
- **Elsewhere.** Record 0069's consequences: "the host's setup sets `cleanupPeriodDays` in every home (boomdrivedev/boomdrive)".

afterthought-design ruled on 2026-11-14, relayed by boomdrive-operator. Wherever the set installs Flume, Claude Code keeps every session's transcript 365 days unless the host names another span. It does so by a document of the set's beside `10-boomdrive-telemetry.json` in Claude Code's `managed-settings.d`, holding `cleanupPeriodDays` alone. deneb writes about 2.7 GB of transcripts every 30 days.

What Claude Code does, as read on 2026-11-14 in the 2.1.322 binary, the release the user's hosts run:

- **The key.** `cleanupPeriodDays` is a whole number, positive and optional: "Number of days to retain chat transcripts before automatic cleanup (default: 30). Minimum 1. Use a large value for long retention". A value below one is a settings error. Its tip reads: "cleanupPeriodDays must be at least 1. To keep transcripts for a long time, set a large number (e.g. 3650 for ~10 years)."
- **The sweep.** Its cutoff is the merged settings' `cleanupPeriodDays`, else thirty days. Where the policy settings set the key, the sweep runs with their value. "Cleanup deletes data only on this computer, not anything stored by Anthropic."
- **Who may set it.** `cleanupPeriodDays` "is only honored from the organization's managed settings sources (server-managed, MDM, managed-settings.json)" when an embedding program hands managed settings of its own. The managed-settings file, with its drop-ins, is one of those sources.
- **Desktop.** `desktopSessionCleanupPeriodDays`, the ceiling for transcripts of Claude Desktop and Cowork, is "Ignored when cleanupPeriodDays is managed by org policy."

What Claude Code's managed settings do is design Context of the change `every-claude-code-session-exports-to-the-hosts-collector`, archived on 2026-11-12:

- **The folder.** `managed-settings.json` is merged first, then every `*.json` in `managed-settings.d` beside it, in alphabetical order. A later file's value replaces an earlier one's.
- **Precedence.** No user or project setting overrides a managed key.
- **Several admin sources.** Where a host also has a server's or an MDM profile's, the first that delivers a policy key wins.
- **The paths.** They are system folders outside every configuration home, which `CLAUDE_CONFIG_DIR` does not move.

The flake today:

- **The telemetry document.** `nix/modules/claude-code.nix` declares `services.boomdrive.claude-code.telemetry` and gives `documentOf`, the name `10-boomdrive-telemetry.json` and the platforms' folders.
  - **NixOS.** `nixos.nix` declares the document's `/etc` entry, mode 0644, with the runner on and a collector named.
  - **nix-darwin.** `darwin.nix` adds one root line, `mkBefore`, to `system.activationScripts.postActivation.text` for any system that imports the module. It runs `activate.sh managed <path> <source>` to write and `activate.sh managed <path>` to remove.
- **Flume's switch.** `tools.nix` turns `services.flume.enable` on by default with the runner, and a host's `false` wins. A host that imports the set's modules may also turn Flume on without the runner. `services.flume.package` is the set's either way.
- **The spec.** The service-modules spec has the modules write no other file of Claude Code's managed settings than the telemetry document. With no collector, NixOS declares no `/etc` entry under `claude-code`.

## Goals / Non-Goals

**Goals:**

- Wherever the set installs Flume, every Claude Code session on the host keeps its transcript 365 days, or the span the host names, whatever its account and configuration home.
- The document holds `cleanupPeriodDays` alone, so the host's own `managed-settings.json` keeps every other key.
- Where Flume is off, the set leaves no document of the kind.

**Non-Goals:**

- **A copy of a transcript.** flume keeps none (record 0069), and the set keeps none.
- **Hibernation and resume.** They are flume's (212, 213).
- **Sharing one `projects` folder between account homes.** It stays the host's choice, made once (flume C.4).
- **Claude Desktop's own sweep,** `desktopSessionCleanupPeriodDays`, which the set does not write.
- **Installing Claude Code,** which stays the person's own.

## Decisions

### 1. One drop-in document of its own, beside the telemetry document

The set writes `10-boomdrive-transcripts.json` into Claude Code's managed-settings folder, the same folder as the telemetry document:

- `/Library/Application Support/ClaudeCode/managed-settings.d` on nix-darwin;
- `/etc/claude-code/managed-settings.d` on NixOS.

Every session on the host reads it, whatever its account and home. Its key wins over every home's `settings.json`, so the sweep that every start runs over the shared `projects` folder uses its span. That is what C.4 asks of a host whose sessions hibernate, "in every home, once", and a home made later reads it too.

The name follows the telemetry document's. The two hold no key in common, so their order is moot. A host's own document named after it in Claude Code's order, such as `20-transcripts.json`, sets the key over the set's on purpose.

The Boomdrive module stays the one writer of the set's documents in Claude Code's managed settings, as record 0013's amendment of 2026-11-12 made it.

Alternatives:

- **The key in the telemetry document.** Not taken. That document holds telemetry keys alone, and is written only with the runner and a collector. Transcripts are kept whether or not a collector is named.
- **The whole `managed-settings.json`.** Not taken: a host's own policy is held there.
- **Each home's `settings.json`, as record 0069's words read.** Not taken. It is a Claude Code home's file, which no module of the set writes (core.46), and a home made after the switch misses it. Managed settings reach every home at once, which is what the words ask.
- **Flume's module writing it.** Not taken. Claude Code's managed settings are no file of Flume's (core.18). Two writers of that folder would be two owners of one thing.

### 2. Written wherever the set installs Flume

The document is written where `services.flume.enable` is on, in a system that imports the set's modules:

- **With the runner,** Flume is on by default (`tools.nix`), so a host of the runner keeps transcripts unless it turns Flume off.
- **Without the runner,** a host that turns Flume on through the set's modules keeps them too.
- **Whatever the collector.** It is written whether or not `services.boomdrive.claude-code.telemetry.collector` names one.

Why Flume: hibernation and resume are Flume's (212, 213). A host where Flume runs is one whose sessions may hibernate. That is the ruling's own condition.

A host that imports Flume's own module alone, `darwinModules.flume` or `nixosModules.flume`, imports no module of the set's, and gets no document.

Alternatives:

- **With the runner, as the telemetry document is.** Not taken: a host that runs Flume alone through the set's modules would lose its hibernated sessions at thirty days.
- **Every system that imports the modules.** Not taken: it would write Claude Code's settings on a host where nothing of the set resumes a session.

### 3. The option, named as Claude Code names its key

`nix/modules/claude-code.nix` declares `services.boomdrive.claude-code.cleanupPeriodDays`:

- **Its type** is `types.ints.positive`, a whole number of one or more. `0`, a negative number or a fraction fails evaluation, naming the option.
- **Its default** is `365`, the ruling's year.
- **Its description** names the document, its two folders, that it is written wherever Flume is on, and that Claude Code deletes a transcript older than the span at a session's start.

The name is Claude Code's own key. core.46 has a module write a tool's file from one option per key, named as the file names it, and core.12 allows a field of another tool's format spelled as that tool spells it. It sits in `services.boomdrive.claude-code` beside `telemetry`, since Claude Code gives no module and the Boomdrive module writes its managed documents.

Alternatives:

- **A null that writes nothing.** Not taken. A host whose stages hibernate would then lose them at thirty days (S127). A host that wants Claude Code's own span names `30`. A host that wants its own document names one after the set's.
- **`types.ints.unsigned`, taking `0`.** Not taken: Claude Code refuses it as a settings error.
- **A duration, such as `"365d"`.** Not taken. Claude Code's key is a number of days, and one option per key holds what the key holds.
- **An upper bound.** Not taken: Claude Code sets none, and its own tip names 3650.

### 4. The document

The document is `{ "cleanupPeriodDays": <days> }` and nothing else. `<days>` is a JSON number, as Claude Code's schema takes it, never a string. The function `transcriptsOf` in `nix/modules/claude-code.nix` makes it from the option, and `builtins.toJSON` renders it, so one option gives one document byte for byte. The file also gives the document's name, `10-boomdrive-transcripts.json`, beside the telemetry document's.

Why 365: it is the ruling's span. A year is longer than any hibernation the fleet keeps, as C.4 asks.

### 5. On NixOS, an `/etc` entry copied

`nixos.nix` declares `environment.etc."claude-code/managed-settings.d/10-boomdrive-transcripts.json"`, with `text` the document and `mode = "0644"`. The mode makes NixOS copy the file rather than link it into the store, as with the telemetry document.

It is declared where `services.flume.enable` is on, whether or not the runner is, so it stands outside the module's `lib.mkIf cfg.enable`. Where Flume is off, no entry is declared, and NixOS removes the file it wrote. No activation step runs for it.

### 6. On nix-darwin, a second root line

`darwin.nix` adds a second root line to `system.activationScripts.postActivation.text`, by `lib.mkBefore`, for any system that imports the module, beside the telemetry document's line:

- **With Flume on,** `/bin/sh ${./activate.sh} managed <folder>/10-boomdrive-transcripts.json <document> || true`, the document being `pkgs.writeText "10-boomdrive-transcripts.json"` of the document.
- **Otherwise,** `/bin/sh ${./activate.sh} managed <folder>/10-boomdrive-transcripts.json || true`, which removes the set's document where it stands, and nothing else in the folder.

nix-darwin removes no file it wrote outside `/etc`, so a host that turns Flume off keeps no document of the set's. The path, which holds a space, is passed by `lib.escapeShellArg`.

`activate.sh managed` does not change: it writes or removes the one path it is given. Its header names both documents. Its cases in `nix/tests/activate.sh` hold for either path.

### 7. What the modules hold to

The service-modules spec's requirements on Claude Code's files make room for the second document:

- ***The modules write no file of Claude Code's but its managed telemetry document*** becomes ***The modules write no file of Claude Code's but its managed telemetry and transcripts documents***.
- ***Where the host names a collector, the modules write Claude Code's managed telemetry document*** writes no other file of Claude Code's managed settings but the transcripts document. Its NixOS scenario allows that entry beside the telemetry document.
- ***With no collector, or without the runner, the set writes no managed telemetry document*** declares no `/etc` entry for the telemetry document. Before, it declared none under `claude-code`, and the transcripts document is now there wherever Flume is on.
- ***Every session on the host reads the document, whatever its home*** holds each managed document's path to the platform's folder.
- ***The Boomdrive module writes no file of the other tools'***: nix-darwin's activation writes or removes each of the two documents.

The tools' own case, *The tools' activation is their own*, compares the runner's nix-darwin activation with the tools' own modules alone. It drops both documents' steps, each run once. On NixOS neither document is a user activation's.

No step, job or unit runs `claude`, or names a Claude Code home, `CLAUDE_CONFIG_DIR` or the set's marketplace. The document holds a number.

### 8. Tests

`nix/tests/claude-code.nix` holds the delta's transcripts scenarios. It evaluates `t.darwinSystem`, `t.nixosSystem`, and `t.darwinOf` and `t.nixosOf` of the modules for the cases with the runner off:

- **NixOS** reads `config.environment.etc."claude-code/managed-settings.d/10-boomdrive-transcripts.json"`, its `text` by `builtins.fromJSON`.
- **nix-darwin** reads the root line naming the transcripts document's path. Its store path is compared with `pkgs.writeText` of the expected document, as the telemetry cases compare theirs.
- **The README** is read through `t.nixBlocks` and `t.installing`.

The telemetry cases read their own document by its name and path: NixOS's `/etc` entry by name, and nix-darwin's root line by its path. Today they read every entry under `claude-code` and every `managed` line. *In no home* reads every document's path.

`nix/tests/tools.nix`'s nix-darwin case drops each document's step, by its path, and holds each to one.

The tests use core.41's neutral names.

### 9. The words, and the rules written now

Written in this change's own commit, by this stage:

- **Record 0046,** the set keeps Claude Code's transcripts a year wherever it installs Flume. It holds afterthought-design's ruling and Decisions 1 to 4, each with the path not taken.
- **core.18's details.** They allow the Boomdrive module to write, as root, the managed transcripts document beside the telemetry document, holding `cleanupPeriodDays` alone, wherever the set installs Flume.
- **core.46's details.** The Claude Code file they rule out leaves the managed transcripts document aside, and they allow it as they allow the telemetry document.

The build writes:

- **The README.** Under **Installing with nix**, after the telemetry part:
  - the option, `services.boomdrive.claude-code.cleanupPeriodDays`, and its default of 365 days;
  - the document's name and its two folders;
  - that it is written wherever Flume is on, with the runner or without;
  - that every session on the host keeps its transcript that long, whatever its home, so a hibernated stage can be resumed;
  - that a host's own document named after the set's sets the key over it;
  - that Claude Code deletes, at a session's start, any transcript older than the span, so a shorter span or turning Flume off deletes what is older at the next start.
- **CLAUDE.md and `openspec/config.yaml`.** They say the modules write Claude Code's managed transcripts document wherever they install Flume, and still touch no Claude Code home.

## Risks / Trade-offs

- **Disk.** A year of transcripts is about twelve times what thirty days keep: on deneb, about 33 GB at today's rate. A host short of room names a shorter span, longer than any hibernation it keeps.
- **A shorter span, or a rollback, deletes.** Claude Code sweeps at each session's start. A host that names a shorter span, turns Flume off or moves to a pin before this change has every transcript older than the new span deleted at its next session's start, and its stages hibernated that long start afresh. The README says so.
- **Several admin sources.** On a host whose organization delivers its own managed settings, the first source that delivers `cleanupPeriodDays` wins. That is the organization's choice.
- **The host's own `managed-settings.json`** is read before the drop-ins, so the set's document sets the key over it. A host that wants its own span names it in the option, or in a document named after the set's.
- **Desktop.** 2.1.322 ignores `desktopSessionCleanupPeriodDays` where `cleanupPeriodDays` is managed. No session of the family's runs in Claude Desktop or Cowork.
- **A Claude Code that reads no `managed-settings.d`** sweeps at its home's own span, as today.
- **No live session in the gate.** The gate holds the document, not what a session's sweep keeps. A host reads it by its oldest transcript, past thirty days after the switch.

## Migration Plan

No host carries a document of the set's for transcripts before this change.

- **A host with Flume on** gets the document at its next switch. Each session that starts after it sweeps at 365 days. A transcript already swept is gone.
- **A host with Flume off** gets nothing.
- **On the user's hosts,** afterthought/swancloud takes the document with its pin. It drops any `cleanupPeriodDays` of its own, so one span holds.
- **Rolling back** is turning Flume off, naming another span, or a pin before this change.
  - NixOS removes its `/etc` entry, and nix-darwin's step of Decision 6 removes the document.
  - A pin before this change has no such step, so on nix-darwin the document is removed by hand.
  - Either way the next session sweeps at the span then in force (Risks).

## Task notes

- **1.1.** The telemetry cases' helpers read the telemetry document alone. `claudeEtcOf` filters to the entry named `10-boomdrive-telemetry.json`, and `managedLines` to the lines naming `darwinPath`. *The document on NixOS* holds that `claude-code` holds no entry but the telemetry and the transcripts documents. *In no home* holds that each document's path is in the platform's managed-settings folder and names no home, and that the telemetry document's is among them.
- **1.3 and 1.4.** For the runner off and Flume on, Flume's module wants its user on NixOS: the case gives `services.flume.user = "alice"`, as `t.toolsNixosSystem` does. On nix-darwin, `t.darwinOf` gives the primary user.
- **1.4.** The transcripts line's path is `/Library/Application Support/ClaudeCode/managed-settings.d/10-boomdrive-transcripts.json`.
