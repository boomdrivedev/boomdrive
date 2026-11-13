# Design

## Context

Record 0013's amendment of 2026-11-12 was accepted by the user that day, on their card:

- **Boomdrive's set carries the wiring.** Every Claude Code session on a drive's accounts exports Claude Code's own telemetry to the collector its host names. That covers a Flume place's session, a standing role's and the person's own. With no collector named, a session exports nothing. The host names which of metrics, logs and traces its sessions export.
- **Only to the host's collector.** The session holds no backend's key, and the collector holds each backend's.
- **Claude Code's own export.** The set installs no vendor's plugin.
- **No session's words leave.** The export holds no prompt, response, tool input, command, file path or tool output, and the settings that add them stay off.
- **Each session names where it came from.** Every agent commits under the same person's name.

core.28 reads with it. The brief of this change names the way: Claude Code's host-wide managed settings, holding the telemetry keys alone, and each session carrying its account and repository.

What Claude Code does, as read on 2026-11-12 in its documentation (`code.claude.com/docs/en/managed-settings`, `settings`, `env-vars` and `monitoring-usage`) and in the 2.1.322 binary:

- **Its managed settings.** They are read from `/Library/Application Support/ClaudeCode/managed-settings.json` on macOS and `/etc/claude-code/managed-settings.json` on Linux and WSL. Each path is a system folder outside every configuration home, which `CLAUDE_CONFIG_DIR` does not move.
  - **Drop-ins.** "Claude Code merges `managed-settings.json` first, then every `*.json` file in the directory in alphabetical order," the directory being `managed-settings.d` beside it. Nested blocks such as `env` "merge key by key", and a later file's single value replaces an earlier one's. The documentation's own example names `10-telemetry.json`. 2.1.322's binary reads `managed-settings.d`.
  - **Precedence.** "Nothing in your own settings files or `--settings` overrides a managed key." An `env` entry "overwrites the same variable exported in your shell". The exception is a launch environment the Claude Desktop app or a self-hosted runner builds, which wins for the variables it sets.
  - **Several admin sources.** Where a host has more than one, a server's or an MDM profile's beside the file, the first that delivers a policy key wins. `env` is the exception, "merged per variable across the admin sources" (2.1.223 or later).
- **Its telemetry.** It is administered "for all users through the managed settings file", by `env`:
  - `CLAUDE_CODE_ENABLE_TELEMETRY` turns telemetry on.
  - `OTEL_METRICS_EXPORTER`, `OTEL_LOGS_EXPORTER` and `OTEL_TRACES_EXPORTER` choose each signal's exporter, `none` turning it off. A developer's own setting can turn a signal off or move it to the console, "so set the selectors in managed settings too if you need them locked".
  - Each signal's endpoint and protocol, `OTEL_EXPORTER_OTLP_<SIGNAL>_ENDPOINT` and `_PROTOCOL`, override the general ones. Claude Code "has no default protocol".
  - Traces are a beta and need `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA=1`.
- **Its content settings.** `OTEL_LOG_USER_PROMPTS`, `OTEL_LOG_ASSISTANT_RESPONSES`, `OTEL_LOG_TOOL_DETAILS` (Bash commands, MCP server and tool names, skill names and tool input), `OTEL_LOG_TOOL_CONTENT` and `OTEL_LOG_RAW_API_BODIES`. Each is off by default. Responses fall back to prompts' setting where unset. `ENABLE_BETA_TRACING_DETAILED` with `BETA_TRACING_ENDPOINT` "changes where your logs and traces go", and is "only for debugging, not in production".
- **Its standard attributes**, on every metric and event:
  - `session.id`, by `OTEL_METRICS_INCLUDE_SESSION_ID`, default on;
  - `organization.id` and `user.email` where signed in;
  - `user.account_uuid` and `user.account_id`, by `OTEL_METRICS_INCLUDE_ACCOUNT_UUID`, default on;
  - `vcs.repository.url.full`, `vcs.owner.name`, `vcs.repository.name` and `vcs.provider.name`, "the session repository's identity, derived from its `origin` remote", by `OTEL_METRICS_INCLUDE_REPOSITORY`, default off (2.1.269 or later). They are left out where the session has no `origin`, and "flow only to your own exporters".
- **What a session's tools inherit.** "Claude Code doesn't pass `OTEL_*` environment variables to the subprocesses it spawns", but a `TRACEPARENT` while tracing. The settings reference says an `env` value reaches every subprocess. The documentation doesn't reconcile the two.
- **No module.** No nix-darwin or NixOS module, at the lock's nix-darwin and nixpkgs, writes Claude Code's managed settings. Claude Code has no command that writes them.

The flake today:

- **Activation.** The modules' activation is `nix/modules/activate.sh`, run as the runner's user. Its steps are `link`, `keep`, `plugins` and `pendingyou`.
- **No Claude Code file.** The service-modules spec has the modules write none of Claude Code's files (core.46's details).
- **sluice's telemetry.** sluice's own module names its collector in `[telemetry]`: an `http://` address of a host, a port and at most a path. sluice sends to its `/v1/traces`, `/v1/metrics` and `/v1/logs`, OTLP over HTTP, protobuf, with each signal a boolean.

## Goals / Non-Goals

**Goals:**

- Every Claude Code session on the host exports Claude Code's own metrics, logs and traces, each as the host names it, to the collector the host names and nowhere else. That holds whatever the session's account and configuration home.
- The document the set writes holds the telemetry keys alone.
- Each session carries its account and repository, and no prompt, response, tool input, command, file path or tool output.
- With no collector named, the set writes nothing.

**Non-Goals:**

- **The collector.** Running it, and choosing each signal's backend, are the host's: on the user's hosts, swancloud's (its record 0061 as amended).
- **Turning traces on for one seat first,** and reading Dash0's Insights from them. That is swancloud's (its record 0061 as amended, *One seat first*).
- **The runner's own OTLP export** (record 0013), which is not built.
- **sluice's export,** which is sluice's module's.
- **Installing Claude Code,** which stays the person's own.
- **Dash0's agent plugin, or any vendor's** (record 0013 as amended).

## Decisions

### 1. One drop-in document of Claude Code's managed settings

The set writes one document, `10-boomdrive-telemetry.json`, into Claude Code's managed-settings folder:

- `/Library/Application Support/ClaudeCode/managed-settings.d` on nix-darwin;
- `/etc/claude-code/managed-settings.d` on NixOS.

Every session on the host reads it, whatever its account and its configuration home, since the folder is the system's and no home's. Its keys win over every user and project setting. Its `env` is written into each session's environment over the shell's.

The name follows the documentation's `10-telemetry.json`. A host's own document named after it, in Claude Code's order, sets a key over the set's on purpose. A host's own `managed-settings.json`, read first, keeps every key the set's document does not set.

The modules write nothing else of Claude Code's: no `managed-settings.json`, no home and no `claude` (Decision 10).

Alternatives:

- **The whole `managed-settings.json`.** Not taken: a host's own policy, such as an organization's, is held there.
- **Each home's `settings.json`.** Not taken: a new account's home misses it, and it is a Claude Code home's file (core.46).
- **The variables in each Flume account's environment, or in the launcher.** Not taken: crew's standing roles and the person's own terminal miss them.
- **`OTEL_*` in the runner's service environment.** Not taken: no session inherits the runner's.

### 2. The option, after sluice's `[telemetry]`

`nix/modules/claude-code.nix`, imported by `darwin.nix` and `nixos.nix`, declares `services.boomdrive.claude-code.telemetry`:

- **`collector`**, `null` by default. The collector's address: `http://`, a host of the machine's own loopback (`localhost`, `127.0.0.1` or `[::1]`), a port, and at most a path, with no user, query or fragment. Its type is a `strMatching` of that, so any other address fails evaluation, naming the option.
- **`metrics`**, `true` by default.
- **`logs`**, `true` by default.
- **`traces`**, `false` by default.

The namespace is the tool's, as `services.boomdrive.plannotator-tui` is. The words are sluice's: `collector`, and a boolean for each signal. A collector with no signal on fails evaluation: "name metrics, logs or traces, or no collector".

Why each choice:

- **On the host.** Record 0013 as amended: "A session exports to the collector on its own host and nowhere else, and the collector holds each backend's key."
- **`http://` alone.** One receiver on the host serves sluice and Claude Code alike. A loopback address needs no TLS.
- **Traces off by default.** They are Claude Code's beta, and swancloud reads one seat before every seat.

Alternatives:

- **Any address.** Not taken: it lets a backend's address into every session, with its key in the headers next.
- **gRPC, or a protocol option.** Not taken: sluice speaks HTTP alone.
- **Every signal on by default.** Not taken: it turns beta traces on everywhere.
- **A list of signals.** Not taken: a list given twice concatenates in Nix's module system, where three booleans merge plainly.
- **One host-wide `services.boomdrive.collector` for the runner and sluice too.** Not taken now: the runner's export is not built, and sluice's collector is its module's own key. When the runner's is built, giving the host's one collector to each from one option is a change of its own (core.57).

### 3. The document

The document is `{ "env": { … } }`, and nothing else beside `env`. Every value is a string. The function `documentOf` in `nix/modules/claude-code.nix` makes it from the option:

| key | value |
|---|---|
| `CLAUDE_CODE_ENABLE_TELEMETRY` | `"1"` |
| `OTEL_METRICS_EXPORTER`, `OTEL_LOGS_EXPORTER`, `OTEL_TRACES_EXPORTER` | `"otlp"` for a signal on, `"none"` for one off |
| `OTEL_EXPORTER_OTLP_<SIGNAL>_ENDPOINT` | `<collector>/v1/<signal>`, for each signal on only |
| `OTEL_EXPORTER_OTLP_<SIGNAL>_PROTOCOL` | `"http/protobuf"`, for each signal on only |
| `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` | `"1"`, with traces on only |
| `OTEL_METRICS_INCLUDE_SESSION_ID`, `OTEL_METRICS_INCLUDE_ACCOUNT_UUID`, `OTEL_METRICS_INCLUDE_REPOSITORY` | `"1"` |
| `OTEL_LOG_USER_PROMPTS`, `OTEL_LOG_ASSISTANT_RESPONSES`, `OTEL_LOG_TOOL_DETAILS`, `OTEL_LOG_TOOL_CONTENT`, `OTEL_LOG_RAW_API_BODIES`, `ENABLE_BETA_TRACING_DETAILED` | `"0"` |

`<SIGNAL>` is `METRICS`, `LOGS` or `TRACES`, and `<signal>` is `metrics`, `logs` or `traces`. `<collector>` is the option's address with any trailing `/` dropped, so `http://localhost:4318/` gives `http://localhost:4318/v1/metrics`.

- **Each signal by its own endpoint, never the general one.** The document sets no `OTEL_EXPORTER_OTLP_ENDPOINT` and no `OTEL_EXPORTER_OTLP_PROTOCOL`. Should a tool a session runs inherit the session's `env`, which the documentation leaves open, it finds no endpoint to send its traces to. That is swancloud record 0061's amendment of 2026-11-09, the fleet's own cause fixed at the session. A signal on is sent to its own endpoint, whatever general endpoint a person's settings name.
- **A signal off is pinned.** `none` in managed settings means a person's own settings send that signal nowhere: "nowhere else".
- **The path.** Each endpoint is the collector's address with `/v1/<signal>`, as OTLP over HTTP takes a signal's own endpoint whole, and as sluice sends.
- **No header, no helper.** The document names no `OTEL_EXPORTER_OTLP_*HEADERS`, no `otelHeadersHelper`, no `BETA_TRACING_ENDPOINT` and no `OTEL_RESOURCE_ATTRIBUTES`. The collector holds each backend's key. A static resource attribute is one value for the whole host.
- **The rendering.** `builtins.toJSON` gives the bytes, keys in Nix's attribute order, so one option gives one document byte for byte.

### 4. No words: the content settings pinned off

The five content settings and `ENABLE_BETA_TRACING_DETAILED` are each pinned `"0"`. They are not left unset, so no person's own setting turns one on, and no session's prompt, response, tool input, command, file path or tool output leaves in what it exports (core.28). Spans then go as Claude Code redacts them by default. `ENABLE_BETA_TRACING_DETAILED` is pinned off since it moves logs and traces to `BETA_TRACING_ENDPOINT`, which is not the collector.

What the document cannot hold:

- **`workspace.host_paths`.** Claude Code puts it on an event only where its launch environment sets `CLAUDE_CODE_WORKSPACE_HOST_PATHS`, a value of the Claude Desktop app's workspaces. A launch environment wins over settings, so the document cannot clear it. Nothing the set or the family runs sets it.
- **A Desktop session's launch environment.** It wins for the variables it sets, and the document cannot reach those.

Both are named in the README.

### 5. Account and repository, by Claude Code's own attributes

Each session carries its account and repository through Claude Code's own standard attributes, which the document turns on:

- **Its account.** `user.account_uuid` and `user.account_id` (`OTEL_METRICS_INCLUDE_ACCOUNT_UUID`), and `organization.id` and `user.email`, which Claude Code adds wherever the session is signed in.
- **Its session.** `session.id` (`OTEL_METRICS_INCLUDE_SESSION_ID`).
- **Its repository.** `vcs.repository.url.full`, `vcs.owner.name`, `vcs.repository.name` and `vcs.provider.name`, from the session's repository's `origin` (`OTEL_METRICS_INCLUDE_REPOSITORY`). A Flume place's session works in a worktree of the runner's clone, whose `origin` is the forge's address. The forge's token is never in that address (core.33).

Each flag is pinned `"1"`, so a person's setting cannot drop it.

The drive is not an attribute. No host-wide setting can name a session's drive, and the host's accounts give each repository's drive (record 0028). So a backend reads the drive from the repository.

Alternatives:

- **`OTEL_RESOURCE_ATTRIBUTES`.** Not taken: it is one value for every session on the host, so it names no session's account or repository.
- **A launch wrapper per session.** Not taken: the person's own terminal and crew's standing roles each run `claude` from `PATH`, and one wrapper per launcher is what Decision 1 avoids.

### 6. Nothing without a collector, or without the runner

The document is written where `services.boomdrive.enable` is on and `collector` is not `null`. Otherwise:

- **On NixOS,** no `/etc` entry is declared. NixOS removes a file of `/etc` it wrote and no longer declares.
- **On nix-darwin,** nothing removes a file the configuration no longer names. So each activation of a system that imports the module removes `10-boomdrive-telemetry.json` from the folder where it stands, whether or not the runner is on, and nothing else there.

So a host that stops naming a collector, or turns the runner off, keeps no document of the set's.

Why with the runner: record 0013 as amended speaks of the sessions on a drive's accounts, and a host's drives are the runner's.

### 7. On NixOS, an `/etc` entry copied

`nixos.nix` declares `environment.etc."claude-code/managed-settings.d/10-boomdrive-telemetry.json"`, with `text` the document and `mode = "0644"`. The mode makes NixOS copy the file rather than link it into the store, as Claude Code's files are copies (record 0022 as amended on 2026-11-11). No activation step of the modules runs for it.

### 8. On nix-darwin, a root step of the activation

nix-darwin writes only under `/etc`. So `darwin.nix` adds to `system.activationScripts.postActivation.text`, run as root, one line, before the user's steps:

- with the document: `/bin/sh ${./activate.sh} managed <folder>/10-boomdrive-telemetry.json <document> || true`, the document being `pkgs.writeText "10-boomdrive-telemetry.json" (builtins.toJSON document)`;
- without it: `/bin/sh ${./activate.sh} managed <folder>/10-boomdrive-telemetry.json || true`.

The path, which holds a space, is passed by `lib.escapeShellArg`.

### 9. `activate.sh managed <path> [<source>]`

- **Given a source,** it writes `<source>` to `<path>` whole, as a regular file of mode 0644, by a temporary file beside it renamed over the path. It writes only where no regular file there has the same bytes and mode. A link standing at the path is replaced. It makes the path's folder, mode 0755, where it is missing. It says one line naming the path where it wrote it.
- **Given none,** it removes the file or link at `<path>` where one stands, and says one line naming it. Where none stands, it says nothing.
- **Nothing else.** It reads, writes and removes nothing else in the folder.
- **Failures.** A write or a removal that fails says one line naming the path and why, and the step exits 0, as every step of the activation does.

Its cases are in `nix/tests/activate.sh`, run as the case's own user in a scratch folder.

### 10. What the modules hold to

The service-modules spec's requirement *The modules write none of Claude Code's files* becomes *The modules write no file of Claude Code's but its managed telemetry document*. No activation step, launchd job or systemd unit runs `claude`, or names a Claude Code home, `CLAUDE_CONFIG_DIR` or the set's marketplace, with a collector named or not.

The nix-darwin activation of the tools' own case is the same as the tools' own but for four steps:

- the runner's link;
- Pending You's folder;
- the plugins' links;
- the managed document's step.

### 11. The words, and the rules written now

Written in this change's own commit, by this stage:

- **Record 0013's amendment of 2026-11-12, the set's sessions in Claude Code's managed settings.** It holds this design's choices (Decisions 1 to 6), each with the path not taken. It says that the drive is read from the repository.
- **core.18's details.** They allow the Boomdrive module to write Claude Code's managed telemetry document, as root, since Claude Code gives no module and no command writes its managed settings.
- **core.28's details.** They rule out a session's export with a content setting on, to a collector off the host, or by a general endpoint. They allow Claude Code's own account, session and repository attributes.
- **core.46's details.** The Claude Code file they rule out leaves the managed telemetry document aside.

The build writes:

- **The README.** Under **Installing with nix**:
  - the option and its four keys;
  - the two paths and the document's name;
  - that each session on the host exports to the collector alone, whatever its home;
  - that nothing is written without a collector or the runner;
  - that traces are Claude Code's beta, off unless named;
  - that a host's own document named after the set's sets a key over it;
  - that a Desktop session's launch environment and `workspace.host_paths` are beyond the document (Decision 4).
- **CLAUDE.md and `openspec/config.yaml`.** They say the modules write Claude Code's managed telemetry document where the host names a collector, and still touch no Claude Code home.

### 12. Tests

`nix/tests/claude-code.nix`, added to `nix/tests/default.nix`'s `test`, holds the delta's scenarios on the document, the option and the README. It evaluates `t.darwinSystem` and `t.nixosSystem`:

- the NixOS document is `config.environment.etc."claude-code/managed-settings.d/10-boomdrive-telemetry.json"`, its `text` read by `builtins.fromJSON`;
- the nix-darwin document is the store path the activation's `managed` line names, compared with `pkgs.writeText` of the expected document, as `nix/tests/darwin.nix` compares the runner's file;
- the README's words are read through `t.nixBlocks` and the README's text, as `nix/tests/tools.nix` reads them.

Beside it:

- `nix/tests/apps.nix`'s `noClaudeCode` adds a system with a collector named;
- `nix/tests/tools.nix`'s nix-darwin case drops the managed line beside the other three;
- `nix/tests/activate.sh` gains the `managed` cases.

The tests use core.41's neutral names.

## Risks / Trade-offs

- **Traces are a beta.** A release of Claude Code can change their shape or their switch. The document's keys are read again at each Claude Code the user's hosts move to. Traces are off unless named.
- **A Claude Code that reads no drop-in folder** reads no document, and its sessions export nothing. That fails safe: nothing leaves.
- **The documentation disagrees on whether `env` reaches a subprocess.** The document names no general endpoint, so a tool that inherits it has no endpoint for its traces. A tool that reads `OTEL_TRACES_EXPORTER` and a per-signal endpoint and sends its own traces reaches only the host's collector. Under swancloud's record 0061 as amended, that collector forwards Claude Code's spans alone.
- **Another admin source.** On a host whose organization delivers its own managed settings, that source's `env` wins for each variable it sets. That is the organization's choice.
- **A host's own document named after the set's** sets a key over it. That is the host's own configuration, named in the README.
- **Desktop.** Its launch environment, and `workspace.host_paths`, are beyond the document (Decision 4).
- **No live session in the gate.** The gate holds the document, not what a session sends. Whether a session's export carries what Decisions 3 to 5 say is read on one host by the change that turns it on there, swancloud's.

## Migration Plan

No host carries a document of the set's before this change.

- **A host that names no collector** gets nothing.
- **A host that names one** gets the document at its next switch. Each new session reads it, and a running session keeps what it read at its start.
- **On the user's hosts,** swancloud names the collector once its pin carries this change. It drops its own Claude Code telemetry settings, so each session has one endpoint per signal.
- **Rolling back** is naming no collector, or a pin before this change. Each removes the document: NixOS by its `/etc`, nix-darwin by Decision 6's step, which a pin before this change does not have. On such a pin, remove the document by hand.

## Task notes

- **1.1.** `documentOf` and the option live in one file, so the test reads the function and the module reads the same. The type's pattern accepts `http://localhost:4318`, `http://127.0.0.1:4318/`, `http://[::1]:4318/otlp`. It refuses `https://localhost:4318`, `http://collector.example.com:4318`, `http://user@localhost:4318` and `http://localhost:4318/?x=1`.
- **1.3.** The step's folder on nix-darwin is `/Library/Application Support/ClaudeCode/managed-settings.d`. In the shell cases the folder is a scratch path, as the other cases' homes are.
