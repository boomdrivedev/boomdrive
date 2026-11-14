## ADDED Requirements

### Requirement: Wherever the set installs Flume, the modules write Claude Code's managed transcripts document

With `services.flume.enable` on, in a system that imports the set's modules, the modules SHALL write `10-boomdrive-transcripts.json`, whole, as a regular file of mode 0644, into Claude Code's managed-settings folder beside the telemetry document: `/Library/Application Support/ClaudeCode/managed-settings.d` on nix-darwin and `/etc/claude-code/managed-settings.d` on NixOS. They SHALL write it whether or not `services.boomdrive.enable` is on and whether or not a collector is named.

#### Scenario: The transcripts document on NixOS

- **WHEN** a NixOS system turns on `services.boomdrive`, Flume on by default, and names no collector
- **THEN** `/etc` holds `claude-code/managed-settings.d/10-boomdrive-transcripts.json`, mode `0644`, its text the document

#### Scenario: The transcripts document on nix-darwin

- **WHEN** a nix-darwin system turns on `services.boomdrive`, Flume on by default, and names no collector
- **THEN** its activation runs once, as root and before every step of the runner's user, the step that writes the document to `/Library/Application Support/ClaudeCode/managed-settings.d/10-boomdrive-transcripts.json`

#### Scenario: Flume without the runner

- **WHEN** a system imports the set's modules, turns `services.flume` on and leaves `services.boomdrive.enable` off
- **THEN** NixOS's `/etc` holds `claude-code/managed-settings.d/10-boomdrive-transcripts.json`, and nix-darwin's activation runs the step that writes the document

### Requirement: The managed transcripts document holds the span alone

The document SHALL hold `cleanupPeriodDays` and no other key, its value a JSON number: `services.boomdrive.claude-code.cleanupPeriodDays`, `365` by default.

#### Scenario: A year by default

- **WHEN** a system turns on `services.boomdrive` and names no span
- **THEN** the document is `{"cleanupPeriodDays":365}`, on nix-darwin and on NixOS

#### Scenario: A span named

- **WHEN** a system turns on `services.boomdrive` and names `services.boomdrive.claude-code.cleanupPeriodDays = 90`
- **THEN** the document is `{"cleanupPeriodDays":90}`, on nix-darwin and on NixOS

#### Scenario: The span alone

- **WHEN** a system turns on `services.boomdrive` and names a collector with every signal on
- **THEN** the transcripts document's keys are `cleanupPeriodDays` alone, a number, and the telemetry document holds no `cleanupPeriodDays`, on nix-darwin and on NixOS

### Requirement: The span is a whole number of days, one or more

`services.boomdrive.claude-code.cleanupPeriodDays` SHALL be a whole number of one or more; any other value SHALL fail evaluation, naming the option.

#### Scenario: A span below one day

- **WHEN** a system names `services.boomdrive.claude-code.cleanupPeriodDays` as `0`, or as `-1`
- **THEN** evaluation fails, naming `services.boomdrive.claude-code.cleanupPeriodDays`, on nix-darwin and on NixOS

### Requirement: Without Flume, the set writes no managed transcripts document

Where `services.flume.enable` is off, the modules SHALL write no managed transcripts document. On NixOS they SHALL declare no `/etc` entry for it. On nix-darwin each activation of a system that imports the module SHALL remove `10-boomdrive-transcripts.json` from Claude Code's managed-settings folder where it stands, and nothing else there.

#### Scenario: No Flume on NixOS

- **WHEN** a NixOS system turns on `services.boomdrive` and turns `services.flume` off
- **THEN** `/etc` holds no `claude-code/managed-settings.d/10-boomdrive-transcripts.json`

#### Scenario: No Flume on nix-darwin

- **WHEN** a nix-darwin system turns on `services.boomdrive` and turns `services.flume` off
- **THEN** its activation runs once, as root, the step that removes `/Library/Application Support/ClaudeCode/managed-settings.d/10-boomdrive-transcripts.json`, and no step writes it

### Requirement: The README names the transcripts' span

The README's install with nix SHALL name `services.boomdrive.claude-code.cleanupPeriodDays` and its default of 365 days, the document's name and its two folders, and that it is written wherever Flume is on. It SHALL say that every Claude Code session on the host keeps its transcript that long, whatever its home, that a host's own document named after the set's sets the key over it, and that a shorter span, or turning Flume off, deletes the older transcripts at the next session's start.

#### Scenario: The README's transcripts

- **WHEN** the README's install with nix is read
- **THEN** a nix block names `services.boomdrive.claude-code.cleanupPeriodDays`, and its words name `365`, `10-boomdrive-transcripts.json`, `/Library/Application Support/ClaudeCode/managed-settings.d` and `/etc/claude-code/managed-settings.d`, and say that the document comes wherever Flume is on

## MODIFIED Requirements

### Requirement: The Boomdrive module writes no file of the other tools'

The modules' own activation SHALL link the runner's file, write Pending You's folder where crew's companions are installed, run herdr's own `plugin link` for the plugins `services.boomdrive.herdr.plugins` lists, write or remove each of Claude Code's two managed documents on nix-darwin, and do nothing else of a tool's. Flume's, sluice's and crew's files, crew's source in Flume's folder among them, SHALL each be written by that tool's own module, as it writes them without Boomdrive's.

#### Scenario: The tools' activation is their own

- **WHEN** a system with `services.boomdrive` and crew turned on is set beside one that imports Flume's, sluice's and crew's own modules alone, each turned on with the same settings, packages and user, and Flume's with crew's source declared as the modules declare it
- **THEN** on nix-darwin their activations are the same but for the runner's own link, Pending You's folder, herdr's plugins' links and the managed telemetry and transcripts documents' steps, each run once, and on NixOS their user activations are the same but for the runner's own

### Requirement: The modules write no file of Claude Code's but its managed telemetry and transcripts documents

No activation step, launchd job or systemd unit of the modules SHALL run `claude`, name a Claude Code home or write a file of Claude Code's, whatever the host turns on, but the managed telemetry document in Claude Code's managed-settings folder where the host names a collector, and the managed transcripts document there wherever Flume is on.

#### Scenario: No Claude Code of the modules'

- **WHEN** a system turns on `services.boomdrive` and `services.crew`, with a collector named and with none
- **THEN** no activation step, launchd job or systemd unit of the system runs `claude` or names `.claude`, `CLAUDE_CONFIG_DIR` or the set's marketplace, on nix-darwin and on NixOS

### Requirement: Where the host names a collector, the modules write Claude Code's managed telemetry document

With `services.boomdrive.enable` on and `services.boomdrive.claude-code.telemetry.collector` set, the modules SHALL write `10-boomdrive-telemetry.json`, whole, as a regular file of mode 0644, into Claude Code's managed-settings folder: `/Library/Application Support/ClaudeCode/managed-settings.d` on nix-darwin and `/etc/claude-code/managed-settings.d` on NixOS. They SHALL write no other file of Claude Code's managed settings but the managed transcripts document.

#### Scenario: The document on NixOS

- **WHEN** a NixOS system turns on `services.boomdrive` and names the collector `http://localhost:4318`
- **THEN** `/etc` holds `claude-code/managed-settings.d/10-boomdrive-telemetry.json`, mode `0644`, its text the document, and no other entry under `claude-code` but `claude-code/managed-settings.d/10-boomdrive-transcripts.json`

#### Scenario: The document on nix-darwin

- **WHEN** a nix-darwin system turns on `services.boomdrive` and names the collector `http://localhost:4318`
- **THEN** its activation runs, as root, the step that writes the document to `/Library/Application Support/ClaudeCode/managed-settings.d/10-boomdrive-telemetry.json`, and no step writes `managed-settings.json`

### Requirement: Every session on the host reads the document, whatever its home

Each managed document's path SHALL be the platform's managed-settings folder alone, in no Claude Code home, so every Claude Code session on the host reads it, whatever its account and configuration home.

#### Scenario: In no home

- **WHEN** a system names a collector, with crew's module on
- **THEN** the telemetry document's path and the transcripts document's are each in the platform's managed-settings folder, and none names a home, `.claude` or `CLAUDE_CONFIG_DIR`, on nix-darwin and on NixOS

### Requirement: With no collector, or without the runner, the set writes no managed telemetry document

Where `services.boomdrive.claude-code.telemetry.collector` is `null`, or `services.boomdrive.enable` is off, the modules SHALL write no managed telemetry document. On NixOS they SHALL declare no `/etc` entry for it. On nix-darwin each activation of a system that imports the module SHALL remove `10-boomdrive-telemetry.json` from Claude Code's managed-settings folder where it stands, and nothing else there.

#### Scenario: No collector on NixOS

- **WHEN** a NixOS system turns on `services.boomdrive` and names no collector
- **THEN** `/etc` holds no `claude-code/managed-settings.d/10-boomdrive-telemetry.json`

#### Scenario: No collector on nix-darwin

- **WHEN** a nix-darwin system turns on `services.boomdrive` and names no collector
- **THEN** its activation runs, as root, the step that removes `/Library/Application Support/ClaudeCode/managed-settings.d/10-boomdrive-telemetry.json`, and no step writes the telemetry document

#### Scenario: Not without the runner

- **WHEN** a system names a collector and leaves `services.boomdrive.enable` off
- **THEN** NixOS's `/etc` holds no `claude-code/managed-settings.d/10-boomdrive-telemetry.json`, and nix-darwin's activation runs the step that removes the telemetry document

## RENAMED Requirements

- FROM: `### Requirement: The modules write no file of Claude Code's but its managed telemetry document`
- TO: `### Requirement: The modules write no file of Claude Code's but its managed telemetry and transcripts documents`
