## ADDED Requirements

### Requirement: With the runner on and a collector named, the modules turn Flume's and sluice's telemetry on to it

With `services.boomdrive.enable` on and `services.boomdrive.claude-code.telemetry.collector` set, the modules SHALL give `services.flume.settings.telemetry` and `services.sluice.settings.telemetry` each `send = true` and `collector`, the host's collector with one trailing `/` dropped. Each key SHALL be given apart at the modules' default priority, and the modules SHALL give no other key of either tool's `telemetry`.

#### Scenario: The host's collector reaches Flume and sluice

- **WHEN** a system turns on `services.boomdrive` and names the collector `http://localhost:4318`
- **THEN** Flume's `settings.telemetry` and sluice's, each less its `null` keys, are each `{ send = true; collector = "http://localhost:4318"; }`, and the system fails no assertion, on nix-darwin and on NixOS

#### Scenario: A collector's trailing slash

- **WHEN** a system turns on `services.boomdrive` and names the collector `http://127.0.0.1:4318/`
- **THEN** Flume's and sluice's `settings.telemetry.collector` are each `http://127.0.0.1:4318`, on nix-darwin and on NixOS

### Requirement: Each tool keeps which of its signals it sends

The modules SHALL give neither Flume's nor sluice's `telemetry` its `traces`, `metrics`, `logs`, `service` or `attributes`, nor sluice's its `logs_tail_bytes`, whatever `services.boomdrive.claude-code.telemetry`'s `metrics`, `logs` and `traces` say. A key a host sets in a tool's own `telemetry` SHALL be kept beside the modules' keys, and a host's `send` there SHALL win.

#### Scenario: Each tool keeps its signals

- **WHEN** a system turns on `services.boomdrive`, names a collector with `traces` on and `logs` off, and sets `services.flume.settings.telemetry.traces = false` and `services.sluice.settings.telemetry.logs = "tail"`
- **THEN** Flume's `settings.telemetry`, less its `null` keys, is `send`, `collector` and `traces = false` alone, and sluice's is `send`, `collector` and `logs = "tail"` alone, on nix-darwin and on NixOS

#### Scenario: A host turns a tool's sending off

- **WHEN** a system turns on `services.boomdrive`, names a collector, and sets `services.sluice.settings.telemetry.send = false`
- **THEN** sluice's `settings.telemetry.send` is false, its `collector` is still the host's, Flume's `send` is true, and the system fails no assertion, on nix-darwin and on NixOS

### Requirement: No tool of the set sends to a collector other than the host's

Where the modules give the tools' telemetry, evaluation SHALL fail unless Flume's and sluice's `settings.telemetry.collector`, each with one trailing `/` dropped, is the host's collector with one trailing `/` dropped, whatever priority the host gives it. The failure SHALL name the tool's option and `services.boomdrive.claude-code.telemetry.collector`.

#### Scenario: The same collector named again

- **WHEN** a system turns on `services.boomdrive`, names the collector `http://localhost:4318`, and sets `services.flume.settings.telemetry.collector = "http://localhost:4318/"`
- **THEN** the system fails no assertion, on nix-darwin and on NixOS

#### Scenario: A collector apart from the host's

- **WHEN** a system turns on `services.boomdrive`, names the collector `http://localhost:4318`, and sets `services.flume.settings.telemetry.collector = "http://localhost:4317"`, and again with `services.sluice.settings.telemetry.collector` so set
- **THEN** evaluation fails, naming the tool's option and `services.boomdrive.claude-code.telemetry.collector`, on nix-darwin and on NixOS

#### Scenario: A forced collector apart from the host's

- **WHEN** a system turns on `services.boomdrive`, names the collector `http://localhost:4318`, and sets `services.sluice.settings.telemetry.collector = lib.mkForce "http://localhost:4317"`
- **THEN** evaluation fails, naming `services.sluice.settings.telemetry.collector` and `services.boomdrive.claude-code.telemetry.collector`, on nix-darwin and on NixOS

### Requirement: With no collector, or without the runner, the modules give no tool's telemetry

Where `services.boomdrive.claude-code.telemetry.collector` is `null`, or `services.boomdrive.enable` is off, the modules SHALL give no key of Flume's or sluice's `settings.telemetry`.

#### Scenario: No tool's telemetry without a collector

- **WHEN** a system turns on `services.boomdrive` and names no collector
- **THEN** Flume's and sluice's `settings.telemetry` are each `null`, on nix-darwin and on NixOS

#### Scenario: No tool's telemetry without the runner

- **WHEN** a system imports the set's modules, turns `services.flume` and `services.sluice` on, names a collector, and leaves `services.boomdrive.enable` off
- **THEN** Flume's and sluice's `settings.telemetry` are each `null`, on nix-darwin and on NixOS

### Requirement: The README says the collector is Flume's and sluice's too

The README's **Installing with nix** SHALL say that the collector the host names is Flume's and sluice's too, each turned on to send to it through `services.flume.settings.telemetry` and `services.sluice.settings.telemetry`. It SHALL say that which of its signals each tool sends stays that tool's own setting, that `send = false` there turns that tool's telemetry off, and that a tool's `collector` other than the host's fails evaluation.

#### Scenario: The README's collector

- **WHEN** the README's **Installing with nix** is read
- **THEN** it names `services.flume.settings.telemetry`, `services.sluice.settings.telemetry` and `send = false`, and says that a tool's collector other than the host's fails evaluation

## MODIFIED Requirements

### Requirement: The Boomdrive module writes no file of the other tools'

The modules' own activation SHALL link the runner's file, write Pending You's folder where crew's companions are installed, run herdr's own `plugin link` for the plugins `services.boomdrive.herdr.plugins` lists, write or remove each of Claude Code's two managed documents on nix-darwin, and do nothing else of a tool's. Flume's, sluice's and crew's files, crew's source in Flume's folder and Flume's and sluice's `[telemetry]` among them, SHALL each be written by that tool's own module, as it writes them without Boomdrive's.

#### Scenario: The tools' activation is their own

- **WHEN** a system with `services.boomdrive` and crew turned on is set beside one that imports Flume's, sluice's and crew's own modules alone, each turned on with the same settings, packages and user, and Flume's with crew's source declared as the modules declare it
- **THEN** on nix-darwin their activations are the same but for the runner's own link, Pending You's folder, herdr's plugins' links and the managed telemetry and transcripts documents' steps, each run once, and on NixOS their user activations are the same but for the runner's own

#### Scenario: The tools' telemetry is their own

- **WHEN** a system with `services.boomdrive` and crew turned on names the collector `http://localhost:4318`, and is set beside one that imports Flume's, sluice's and crew's own modules alone, as above, with Flume's and sluice's `settings.telemetry` each `{ send = true; collector = "http://localhost:4318"; }`
- **THEN** on nix-darwin their activations are the same but for what the scenario above leaves aside, and on NixOS their user activations' Flume and sluice snippets are each the same
