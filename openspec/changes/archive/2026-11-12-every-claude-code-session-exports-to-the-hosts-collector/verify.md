# Verify: every-claude-code-session-exports-to-the-hosts-collector

The verify stage, 2026-11-12, on `change/every-claude-code-session-exports-to-the-hosts-collector` at `2617012`, cut from `bolt/claude-codes-telemetry-reaches-the-hosts-collector`.

**Verdict: the whole checks pass and the work holds.** The change was already archived at `2617012` by the code stage, as task 3.3 has it, so this report goes beside the archived change.

## The whole checks

`sluice exec main` ran once, on `2617012`, against `main` (`e2d9dc2`). It exited 0: 9 tasks passed, and sluice recorded their results under their keys for `2617012`.

| task | outcome |
|---|---|
| `nix:format` | passed |
| `nix:lint` | passed |
| `nix:typecheck` | passed |
| `nix:test` | passed, "every case holds", `claude-code.nix`'s cases among them |
| `root:format` | passed |
| `root:specs` | passed, 26 of 26 specs, `service-modules` among them |
| `root:adrs` | passed |
| `root:changes` | passed |
| `boomdrive:test` | passed, every test of the crate |

The other projects' tasks are not affected by the branch, and sluice did not run them. moon's warnings that it could not archive task outputs in the sandbox, and that it skipped installing dependencies with no network, changed no result.

## The tasks

All 10 tasks, 1.1 to 3.3, are ticked. Each one names `nix:test`, `root:format` or `root:specs` as its check, and each of those passed above.

## The spec delta, requirement by requirement

Each scenario is a named case in `nix/tests/claude-code.nix`, `nix/tests/activate.sh`, `nix/tests/apps.nix` or `nix/tests/tools.nix`. Each case runs on nix-darwin and NixOS wherever the scenario names both. The cases spell out the expected documents key by key and do not compute them with `documentOf`.

| requirement | where it is met | proven by |
|---|---|---|
| Where the host names a collector, the modules write the document | `nixos.nix`: `environment.etc."claude-code/managed-settings.d/10-boomdrive-telemetry.json"`, mode `0644`, under `mkIf cfg.enable`. `darwin.nix`: one root line in `postActivation`, ahead of the user's steps, that writes the `pkgs.writeText` document | "The document on NixOS": the only entry under `claude-code`, mode and text checked. "The document on nix-darwin": the step runs once, before every `sudo --user=` line, and no line names `managed-settings.json` |
| Every session reads it, whatever its home | the path is the platform's managed-settings folder alone | "In no home", with crew on |
| The document holds the telemetry keys alone | `claude-code.nix`'s `documentOf` returns `{ env = … }` and nothing else | "The telemetry keys alone": `env` alone, every value a string, every key on the list of 20 |
| Each signal is exported as the host names it | `metrics`, `logs` and `traces` default to `true`, `true` and `false`. Exporters are `otlp` or `none`. `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA` is set only when traces are on | "Metrics and logs by default", "Traces named" |
| Each signal on goes to the collector by its own endpoint alone | `removeSuffix "/"`, then `/v1/<signal>` and `http/protobuf`, for each signal on. No general endpoint, header or resource attribute is set | "Each signal's endpoint", with `http://127.0.0.1:4318/`. "No general endpoint" |
| The collector is on the host | `types.strMatching` allows only an `http://` address on `localhost`, `127.0.0.1` or `[::1]`, with a port and an optional path. An assertion fails when every signal is off | "A collector off the host": `example.com`, `https`, a user and a query each refused, naming the option, and three loopback forms taken. "A collector with no signal": the failure names `metrics`, `logs` and `traces` |
| No session's words leave in what it exports | the six content keys are pinned to `"0"` | "The content settings off" |
| Each session carries its account and repository | the three `OTEL_METRICS_INCLUDE_*` keys are pinned to `"1"` | "The account and repository attributes on" |
| With no collector, or without the runner, no document | NixOS declares no entry. nix-darwin's step runs for every system that imports the module, outside `mkIf cfg.enable`, and removes the document | "No collector on NixOS", "No collector on nix-darwin", and "Not without the runner" on each platform |
| The managed step writes the document whole, and nothing else | `activate.sh`'s `document`: makes the folder `0755`, writes a temporary file beside the path at `0644`, renames it over the path and replaces a link. It skips the write where the bytes and mode already match. It says one line and exits 0 | "A fresh host", "Nothing differs", "A link where the document goes" (the link's target unchanged), "A folder it cannot write", plus a case that rewrites a document whose bytes or mode differ |
| The managed step removes the document alone | `managed` with no source removes only the file or link at the path | "The document removed": the host's own document and `managed-settings.json` unchanged, and a link removed with its target kept. "Nothing to remove", with and without the folder. Usage exits 2 |
| The README names the sessions' telemetry | the README's **Installing with nix** gains the part on the sessions' telemetry | "The README's telemetry" |
| *(modified)* The Boomdrive module writes no file of the other tools' | the managed step is the only line the module adds to nix-darwin's activation | "The tools' activation is their own": the step runs exactly once, and with it removed the activation equals the tools' own |
| *(renamed and modified)* The modules write no file of Claude Code's but its managed telemetry document | no step, job or unit names `claude`, `.claude`, `CLAUDE_CONFIG_DIR` or the marketplace | "No Claude Code of the modules'", run with a collector named and with none |

## What the change holds to

- **Paths.** Every path the branch changes is in the proposal's Touches. The one exception is `openspec/specs/service-modules/spec.md`, which the archive of task 3.3 writes.
- **Code and tooling.** The runner's code is unchanged. No crate, tool of the gate or tracked file type is added (core.7, core.8). `nix:format` and `root:format` both pass.
- **Records and rules.** Record 0013's amendment of 2026-11-12, the set's sessions in Claude Code's managed settings, and the details of core.18, core.28 and core.46 are in the planning commit `b38adb0c`. They agree with what the modules do.
- **Specs.** The archived spec has the renamed requirement and none of the old one. `root:specs` validates it strictly.

## Findings

None.
