# Verify: the-set-keeps-claude-codes-transcripts-a-year

The work holds. The gate passes, all eight tasks are ticked, and a case proves each scenario of the delta.

The code stage archived the change under task 2.3, because `root:specs` can prove the delta only after the archive. This report sits beside the archive, on `change/the-set-keeps-claude-codes-transcripts-a-year` at `366ebc8`.

## The whole checks

`sluice exec main` ran once on `366ebc8`. It ran the checks that commit affects against `main` (`021a3f5`) and exited 0. All 9 passed, and sluice recorded their results under their keys for `366ebc8`:

- `nix`: `format`, `lint`, `typecheck` and `test` ("every case holds");
- `root`: `format`, `adrs`, `changes` and `specs`;
- `boomdrive`: `test`.

`root:specs` passed 27 items, with none failed. `root:changes` found no open change to validate. moon also warned that it could not write its output archives under `.moon/cache/outputs`, and that it skipped the Rust dependency install for want of a network. Both warnings come from the sandbox, and no check failed because of them.

## Tasks

All eight tasks are ticked, 1.1 to 2.3, and none is open.

- 1.1 to 1.5 and 2.1 rely on `nix:test`.
- 2.2 relies on `root:format`.
- 2.3 relies on `root:specs`.

Each of those checks passed in the run above.

The diff from the branch's base stays inside proposal.md's Touches. It adds no crate, gate tool or tracked file type, and the runner's code is unchanged. `activate.sh` changes only its header. The tasks say record 0046 and the details of core.18 and core.46 were written in the planning commit, `f8f5645e`, and they were.

## The delta, scenario by scenario

The cases are keys of `nix/tests/claude-code.nix` unless the table names another file.

### Added

| Scenario | Proven by |
|---|---|
| The transcripts document on NixOS | "The transcripts document on NixOS": `/etc`'s entry by name and target, mode `0644`, its text `builtins.toJSON` of `transcriptsOf` the option |
| The transcripts document on nix-darwin | "The transcripts document on nix-darwin": `runsOnceAmong` holds the lines naming the document's path to the one writing step, before every `sudo --user=` step, with no line naming `managed-settings.json` |
| Flume without the runner | "Flume without the runner on NixOS" and "… on nix-darwin": `t.nixosOf` and `t.darwinOf` of the set's modules with `services.flume.enable` on and the runner off |
| A year by default | "A year by default", on each platform: `{"cleanupPeriodDays":365}`. On NixOS this is `/etc`'s text, and on nix-darwin the step's store path, compared with `pkgs.writeText` of the expected bytes |
| A span named | "A span named", on each platform: `{"cleanupPeriodDays":90}`, read the same way |
| The span alone | "The span alone", on each platform: the document's keys are `cleanupPeriodDays` alone and its value is an integer, and the telemetry document of every signal holds no `cleanupPeriodDays` at its top or in its `env` |
| A span below one day | "A span below one day", on each platform, for `0` and `-1`: reading the option fails, the option is declared at `services.boomdrive.claude-code.cleanupPeriodDays`, and its type refuses the value |
| No Flume on NixOS | "No Flume on NixOS": no `/etc` entry by that name or target |
| No Flume on nix-darwin | "No Flume on nix-darwin": the one line naming the path is the removing step, run before the user's steps |
| The README's transcripts | "The README's transcripts": the nix block sets `services.boomdrive.claude-code.cleanupPeriodDays = 365;`, and the words of **Installing with nix** say each phrase the requirement names |

In "A span below one day", the failure is not read by its words, since `builtins.tryEval` gives no message. The case holds the option's location, and its type's refusal of the value. That refusal is what makes the module system's failure name the option.

### Modified

| Scenario | Proven by |
|---|---|
| The tools' activation is their own | `nix/tests/tools.nix`'s "The tools' activation is their own on nix-darwin": each document's step is dropped by its path and held to one, and the rest equals the tools' own modules' activation. Its NixOS case is unchanged |
| No Claude Code of the modules' | `nix/tests/apps.nix`'s `noClaudeCode`, unchanged: with a collector and with none, no step, job or unit names `claude`, `.claude`, `CLAUDE_CONFIG_DIR` or the set's marketplace. The transcripts step is among the steps it reads |
| The document on NixOS | "The document on NixOS": the telemetry entry as before, and no other entry under `claude-code` but the transcripts document |
| The document on nix-darwin | "The document on nix-darwin": `runsOnce` over the lines naming the telemetry path alone, with no line naming `managed-settings.json` |
| In no home | "In no home", on each platform: every managed document's path is in the platform's folder, the telemetry document's is among them, and none names a home |
| No collector on NixOS and on nix-darwin | the cases of those names, with `claudeEtcOf` and `managedLines` narrowed to the telemetry document |
| Not without the runner | "Not without the runner on NixOS" and "… on nix-darwin", read the same way |

The renamed requirement is in `openspec/specs/service-modules/spec.md` under its new name, and `root:specs` passes it.

## The modules

- `nix/modules/claude-code.nix` declares the option, `types.ints.positive`, default `365`. Its description names the document, the two folders and Flume, and the deletion at a session's start. The file gives `transcriptsName` and `transcriptsOf`.
- `nix/modules/nixos.nix` declares the `/etc` entry under `lib.mkIf config.services.flume.enable`, outside the runner's `lib.mkIf cfg.enable`. The runner's own config moves into `lib.mkMerge` unchanged.
- `nix/modules/darwin.nix` appends `keeping` to `managing` under one `lib.mkBefore`. It writes the document where Flume is on and removes it otherwise, and passes the path by `lib.escapeShellArg`.

## The words

- The README's **Installing with nix** says, after the telemetry part:
  - the option and its default of 365 days;
  - `10-boomdrive-transcripts.json` and its two folders;
  - that it is written wherever Flume is on, with the runner or without, whatever the collector;
  - that every session keeps its transcript a year, whatever its account and home;
  - that a host's own document named after the set's, such as `20-transcripts.json`, sets the key over it;
  - that a shorter span, or turning Flume off, deletes every older transcript at the next session's start.
- CLAUDE.md and `openspec/config.yaml` say the modules write the managed transcripts document wherever they install Flume, holding `cleanupPeriodDays` alone, a year by default, and still touch no Claude Code home.

## For the landing

`change/the-runners-macos-grant-outlasts-a-new-build`, which is not on `main`, also numbers its record 0046 (its `56fc8301`). That is record `0046-on-macos-the-runners-service-runs-a-copy-signed-with-the-hosts-own-identity-so-a-grant-holds-for-every-build.md`, and its rule is core.60. Whichever reaches `main` second will fail `root:adrs` until its record takes the next free number. This branch's gate is not affected, since neither record is on `main` (`021a3f5`).

## Archive

The change is at `openspec/changes/archive/2026-11-14-the-set-keeps-claude-codes-transcripts-a-year`, and no change of that name is open. `openspec/specs/service-modules/spec.md` holds the delta, and `root:specs` passes it under the strict check.
