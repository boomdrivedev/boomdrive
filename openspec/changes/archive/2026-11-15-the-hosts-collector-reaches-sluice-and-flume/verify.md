# Verify

The verify stage of `the-hosts-collector-reaches-sluice-and-flume`, on `change/the-hosts-collector-reaches-sluice-and-flume` at `e5cd526c`, cut from `bolt/a-hosts-set-takes-its-tools-new-releases` at `9bba58b5`, 2026-11-15.

Verdict: the whole checks pass and the work holds. Every requirement and scenario of the delta is met, and every task is ticked done.

## The whole checks

`sluice exec main`, run once on `e5cd526c`, exited 0. sluice ran the checks the branch affects against `main` (`23e4181`): 16 tasks completed, 14 results recorded under their keys.

| project | tasks |
|---|---|
| `nix` | `format`, `lint`, `typecheck`, `test` ("nix: test: every case holds") |
| `boomdrive` | `test` |
| `root` | `format`, `adrs`, `specs`, `changes`, `releases` |
| `site` | `install`, `check`, `build`, `test`, `prose`, `vocab` |

moon warned that it could not archive the outputs of `nix:lint` and `nix:format` into its cache, and that it skipped the Rust toolchain's dependency install with no network. Neither warning failed a task.

## Tasks

All eight are ticked `[x]`. Each one's check passes:

- **1.1 to 1.5 and 2.1.** Each is true when `nix:test` passes with its named cases, and it does. Every case those tasks name is in the tree:
  - In `nix/tests/collector.nix`: each scenario of the delta on nix-darwin and on NixOS, "The tools' telemetry is their own" on each platform, and "The README's collector".
  - In `nix/tests/claude-code.nix`: "Each signal's endpoint", "Metrics and logs by default" and "The README's telemetry".
  - In `nix/tests/tools.nix`: "The tools' activation is their own" on each platform.
- **2.2.** `grep -c 'services.sluice.settings.telemetry' CLAUDE.md openspec/config.yaml` counts 1 in each file. `root:format` runs treefmt over `openspec/config.yaml` and passes.
- **2.3.** The change is archived at `openspec/changes/archive/2026-11-15-the-hosts-collector-reaches-sluice-and-flume`. `openspec validate service-modules --type spec --strict --no-interactive` passes ("Specification 'service-modules' is valid"), and so does `root:specs`. The requirement *The Boomdrive module writes no file of the other tools'* carries its second scenario, *The tools' telemetry is their own*.

The code stage archived the change in task 2.3, as that task asks. This report therefore sits in the archived directory, beside the rest of the change.

## Requirements and scenarios

- **With the runner on and a collector named, the modules turn Flume's and sluice's telemetry on to it.**
  - Under `services.boomdrive.enable` and a collector that is not `null`, `nix/modules/collector.nix` gives each tool `send` and `collector`, each apart by `lib.mkDefault`, and gives no other key.
  - `collector` is `baseOf` of the host's collector.
  - `darwin.nix` and `nixos.nix` import the module beside `sources.nix`, and each header names it.
  - The scenarios *The host's collector reaches Flume and sluice* and *A collector's trailing slash* hold on both platforms.
- **Each tool keeps which of its signals it sends.**
  - Neither tool is given `traces`, `metrics`, `logs`, `logs_tail_bytes`, `service` or `attributes`.
  - A host's own keys merge beside the module's, and a host's `send` wins over the module's `mkDefault`.
  - The scenarios *Each tool keeps its signals* and *A host turns a tool's sending off* hold.
- **No tool of the set sends to a collector other than the host's.**
  - The assertion holds a tool's collector to `own != null && baseOf own == baseOf host`. It reads the merged value, so it covers `lib.mkForce`.
  - Its message names the option through `lib.showOption`, together with both values and `services.boomdrive.claude-code.telemetry.collector`.
  - The scenarios *The same collector named again*, *A collector apart from the host's* (Flume and sluice) and *A forced collector apart from the host's* hold.
- **With no collector, or without the runner, the modules give no tool's telemetry.**
  - The module's `config` sits under `lib.mkIf` of both conditions.
  - The scenarios *No tool's telemetry without a collector* and *No tool's telemetry without the runner* hold.
- **The README says the collector is Flume's and sluice's too.**
  - A paragraph in **Installing with nix** names both options and `send = false`, and says that a tool's `collector` other than the host's fails evaluation. It sits after the telemetry document's part and before "Without a collector, or without the runner".
  - The scenario *The README's collector* holds.
- **The Boomdrive module writes no file of the other tools' (modified).**
  - The requirement's text now names Flume's and sluice's `[telemetry]` among the files each tool's own module writes.
  - The scenario *The tools' telemetry is their own* compares the runner's system, with crew and a collector, against the tools' own modules given the same telemetry: the activation lines on nix-darwin and Flume's and sluice's user activation snippets on NixOS.
  - At archive, the code stage shortened the requirement's opening sentence to openspec's strict length and kept its meaning (task 2.3).

## Bounds the tasks set

- **Unchanged:**
  - No crate and none of the runner's code.
  - `tools.nix`, `sources.nix` and `activate.sh`.
- **Added:** no tool of the gate and no tracked file type.
- **What the branch changes:**
  - The paths the proposal's Touches lists.
  - `openspec/specs/service-modules/spec.md`, which task 2.3's archive writes.
- **Already written:** record 0013's amendment of 2026-11-15 and core.57's details, with its new "Not ruled out", are in the change's own commit (`02e1e9c3`), as design Decision 8 says.
