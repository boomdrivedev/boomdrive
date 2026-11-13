## ADDED Requirements

### Requirement: Where the host names a collector, the modules write Claude Code's managed telemetry document

With `services.boomdrive.enable` on and `services.boomdrive.claude-code.telemetry.collector` set, the modules SHALL write `10-boomdrive-telemetry.json`, whole, as a regular file of mode 0644, into Claude Code's managed-settings folder: `/Library/Application Support/ClaudeCode/managed-settings.d` on nix-darwin and `/etc/claude-code/managed-settings.d` on NixOS. They SHALL write no other file of Claude Code's managed settings.

#### Scenario: The document on NixOS

- **WHEN** a NixOS system turns on `services.boomdrive` and names the collector `http://localhost:4318`
- **THEN** `/etc` holds `claude-code/managed-settings.d/10-boomdrive-telemetry.json`, mode `0644`, its text the document, and no other entry under `claude-code`

#### Scenario: The document on nix-darwin

- **WHEN** a nix-darwin system turns on `services.boomdrive` and names the collector `http://localhost:4318`
- **THEN** its activation runs, as root, the step that writes the document to `/Library/Application Support/ClaudeCode/managed-settings.d/10-boomdrive-telemetry.json`, and no step writes `managed-settings.json`

### Requirement: Every session on the host reads the document, whatever its home

The document's path SHALL be the platform's managed-settings folder alone, in no Claude Code home, so every Claude Code session on the host reads it, whatever its account and configuration home.

#### Scenario: In no home

- **WHEN** a system names a collector, with crew's module on
- **THEN** the document's path names no home, `.claude` or `CLAUDE_CONFIG_DIR`, on nix-darwin and on NixOS

### Requirement: The managed telemetry document holds the telemetry keys alone

The document SHALL hold `env` and no other key, each key of its `env` one of Claude Code's telemetry keys and each value a string.

#### Scenario: The telemetry keys alone

- **WHEN** a system names a collector and turns on metrics, logs and traces
- **THEN** the document's keys are `env` alone, with string values, and each key of its `env` is one of `CLAUDE_CODE_ENABLE_TELEMETRY`, `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA`, `ENABLE_BETA_TRACING_DETAILED`, `OTEL_METRICS_EXPORTER`, `OTEL_LOGS_EXPORTER`, `OTEL_TRACES_EXPORTER`, `OTEL_EXPORTER_OTLP_METRICS_ENDPOINT`, `OTEL_EXPORTER_OTLP_LOGS_ENDPOINT`, `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`, `OTEL_EXPORTER_OTLP_METRICS_PROTOCOL`, `OTEL_EXPORTER_OTLP_LOGS_PROTOCOL`, `OTEL_EXPORTER_OTLP_TRACES_PROTOCOL`, `OTEL_METRICS_INCLUDE_SESSION_ID`, `OTEL_METRICS_INCLUDE_ACCOUNT_UUID`, `OTEL_METRICS_INCLUDE_REPOSITORY`, `OTEL_LOG_USER_PROMPTS`, `OTEL_LOG_ASSISTANT_RESPONSES`, `OTEL_LOG_TOOL_DETAILS`, `OTEL_LOG_TOOL_CONTENT` and `OTEL_LOG_RAW_API_BODIES`, on nix-darwin and on NixOS

### Requirement: Each signal is exported as the host names it

`services.boomdrive.claude-code.telemetry.metrics`, `logs` and `traces` SHALL each say whether that signal is exported, `true`, `true` and `false` by default. The document SHALL set `CLAUDE_CODE_ENABLE_TELEMETRY` to `"1"`, each signal's exporter to `"otlp"` where it is on and `"none"` where it is off, and `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` to `"1"` with traces on and not at all with traces off.

#### Scenario: Metrics and logs by default

- **WHEN** a system names a collector and names no signal
- **THEN** the document's `OTEL_METRICS_EXPORTER` and `OTEL_LOGS_EXPORTER` are `otlp`, its `OTEL_TRACES_EXPORTER` is `none`, and it has no `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA`

#### Scenario: Traces named

- **WHEN** a system names a collector and turns `traces` on and `logs` off
- **THEN** the document's `OTEL_TRACES_EXPORTER` is `otlp`, its `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` is `1`, and its `OTEL_LOGS_EXPORTER` is `none`

### Requirement: Each signal on goes to the collector by its own endpoint alone

For each signal on, the document SHALL set that signal's endpoint to the collector's address, any trailing `/` dropped, followed by `/v1/<signal>`, and its protocol to `"http/protobuf"`. For a signal off it SHALL set neither. It SHALL set no general endpoint, protocol or header, no headers helper, no detailed tracing endpoint and no resource attributes.

#### Scenario: Each signal's endpoint

- **WHEN** a system names the collector `http://127.0.0.1:4318/` and turns `traces` on and `logs` off
- **THEN** the document's metrics endpoint is `http://127.0.0.1:4318/v1/metrics` and its traces endpoint `http://127.0.0.1:4318/v1/traces`, each protocol `http/protobuf`, and it has no logs endpoint or logs protocol

#### Scenario: No general endpoint

- **WHEN** a system names a collector and turns on every signal
- **THEN** the document holds no `OTEL_EXPORTER_OTLP_ENDPOINT`, `OTEL_EXPORTER_OTLP_PROTOCOL`, key ending in `HEADERS`, `otelHeadersHelper`, `BETA_TRACING_ENDPOINT` or `OTEL_RESOURCE_ATTRIBUTES`

### Requirement: The collector is on the host

`services.boomdrive.claude-code.telemetry.collector` SHALL be `null` or an `http://` address whose host is `localhost`, `127.0.0.1` or `[::1]`, with a port and at most a path, and no user, query or fragment; any other value SHALL fail evaluation, naming the option. A collector named with every signal off SHALL fail evaluation, naming `metrics`, `logs` and `traces`.

#### Scenario: A collector off the host

- **WHEN** a system names the collector `http://collector.example.com:4318`, or `https://localhost:4318`
- **THEN** evaluation fails, naming `services.boomdrive.claude-code.telemetry.collector`

#### Scenario: A collector with no signal

- **WHEN** a system names a collector and turns `metrics`, `logs` and `traces` off
- **THEN** evaluation fails, naming `metrics`, `logs` and `traces`

### Requirement: No session's words leave in what it exports

The document SHALL set each of `OTEL_LOG_USER_PROMPTS`, `OTEL_LOG_ASSISTANT_RESPONSES`, `OTEL_LOG_TOOL_DETAILS`, `OTEL_LOG_TOOL_CONTENT`, `OTEL_LOG_RAW_API_BODIES` and `ENABLE_BETA_TRACING_DETAILED` to `"0"`, whatever the host turns on.

#### Scenario: The content settings off

- **WHEN** a system names a collector and turns on every signal
- **THEN** each of the six keys is `0` in the document, on nix-darwin and on NixOS

### Requirement: Each session carries its account and repository

The document SHALL set each of `OTEL_METRICS_INCLUDE_SESSION_ID`, `OTEL_METRICS_INCLUDE_ACCOUNT_UUID` and `OTEL_METRICS_INCLUDE_REPOSITORY` to `"1"`, so each session's metrics and events carry Claude Code's own session, account and repository attributes.

#### Scenario: The account and repository attributes on

- **WHEN** a system names a collector
- **THEN** each of the three keys is `1` in the document, on nix-darwin and on NixOS

### Requirement: With no collector, or without the runner, the set writes no managed telemetry document

Where `services.boomdrive.claude-code.telemetry.collector` is `null`, or `services.boomdrive.enable` is off, the modules SHALL write no managed telemetry document. On NixOS they SHALL declare no `/etc` entry under `claude-code`. On nix-darwin each activation of a system that imports the module SHALL remove `10-boomdrive-telemetry.json` from Claude Code's managed-settings folder where it stands, and nothing else there.

#### Scenario: No collector on NixOS

- **WHEN** a NixOS system turns on `services.boomdrive` and names no collector
- **THEN** `/etc` holds no entry under `claude-code`

#### Scenario: No collector on nix-darwin

- **WHEN** a nix-darwin system turns on `services.boomdrive` and names no collector
- **THEN** its activation runs, as root, the step that removes `/Library/Application Support/ClaudeCode/managed-settings.d/10-boomdrive-telemetry.json`, and no step writes a document

#### Scenario: Not without the runner

- **WHEN** a system names a collector and leaves `services.boomdrive.enable` off
- **THEN** NixOS's `/etc` holds no entry under `claude-code`, and nix-darwin's activation runs the step that removes the document

### Requirement: The managed step writes the document whole, and nothing else

`activate.sh managed <path> <source>` SHALL write `<source>` to `<path>` whole, as a regular file of mode 0644, by a temporary file beside it renamed over the path, only where no regular file there has the same bytes and mode, replacing a link there. It SHALL make the path's folder, mode 0755, where it is missing, and say one line naming the path where it wrote it. A write that fails SHALL say one line naming the path, and the step SHALL exit 0.

#### Scenario: A fresh host

- **WHEN** the step runs with a source and a path whose folder is missing
- **THEN** the folder is made with mode 0755, the path holds the source's bytes as a regular file of mode 0644, one line names the path, and the step exits 0

#### Scenario: Nothing differs

- **WHEN** the step runs with a source and the path already holds its bytes with mode 0644
- **THEN** nothing is written, nothing is said, and the step exits 0

#### Scenario: A link where the document goes

- **WHEN** the step runs with a source and a link stands at the path
- **THEN** the path holds the source's bytes as a regular file of mode 0644, and the link's target is unchanged

#### Scenario: A folder it cannot write

- **WHEN** the step runs with a source and the path's folder cannot be written
- **THEN** one line names the path not written, and the step exits 0

### Requirement: The managed step removes the document alone

`activate.sh managed <path>` SHALL remove the file or link at `<path>` where one stands, saying one line naming it, and say nothing where none stands. It SHALL read, write and remove nothing else in the folder, and exit 0.

#### Scenario: The document removed

- **WHEN** the step runs with no source, the path holds a file, and the folder holds another document
- **THEN** the path holds nothing, the other document is unchanged, one line names the path, and the step exits 0

#### Scenario: Nothing to remove

- **WHEN** the step runs with no source and nothing stands at the path
- **THEN** nothing is changed, nothing is said, and the step exits 0

### Requirement: The README names the sessions' telemetry

The README's install with nix SHALL name `services.boomdrive.claude-code.telemetry` and its `collector`, `metrics`, `logs` and `traces`, the document's name and its two folders, and that every Claude Code session on the host exports to the collector alone, whatever its home. It SHALL say that nothing is written without a collector or the runner, and that traces are Claude Code's beta, off unless named.

#### Scenario: The README's telemetry

- **WHEN** the README's install with nix is read
- **THEN** a nix block names `services.boomdrive.claude-code.telemetry` with a `collector`, and its words name `metrics`, `logs`, `traces`, `10-boomdrive-telemetry.json`, `/Library/Application Support/ClaudeCode/managed-settings.d` and `/etc/claude-code/managed-settings.d`, and say that traces are off unless named

## MODIFIED Requirements

### Requirement: The Boomdrive module writes no file of the other tools'

The modules' own activation SHALL link the runner's file, write Pending You's folder where crew's companions are installed, run herdr's own `plugin link` for the plugins `services.boomdrive.herdr.plugins` lists, write or remove Claude Code's managed telemetry document on nix-darwin, and do nothing else of a tool's. Flume's, sluice's and crew's files, crew's source in Flume's folder among them, SHALL each be written by that tool's own module, as it writes them without Boomdrive's.

#### Scenario: The tools' activation is their own

- **WHEN** a system with `services.boomdrive` and crew turned on is set beside one that imports Flume's, sluice's and crew's own modules alone, each turned on with the same settings, packages and user, and Flume's with crew's source declared as the modules declare it
- **THEN** on nix-darwin their activations are the same but for the runner's own link, Pending You's folder, herdr's plugins' links and the managed telemetry document's step, and on NixOS their user activations are the same but for the runner's own

### Requirement: The modules write no file of Claude Code's but its managed telemetry document

No activation step, launchd job or systemd unit of the modules SHALL run `claude`, name a Claude Code home or write a file of Claude Code's, whatever the host turns on, but the managed telemetry document in Claude Code's managed-settings folder where the host names a collector.

#### Scenario: No Claude Code of the modules'

- **WHEN** a system turns on `services.boomdrive` and `services.crew`, with a collector named and with none
- **THEN** no activation step, launchd job or systemd unit of the system runs `claude` or names `.claude`, `CLAUDE_CONFIG_DIR` or the set's marketplace, on nix-darwin and on NixOS

## RENAMED Requirements

- FROM: `### Requirement: The modules write none of Claude Code's files`
- TO: `### Requirement: The modules write no file of Claude Code's but its managed telemetry document`
