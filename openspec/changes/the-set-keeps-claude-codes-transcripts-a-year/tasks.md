# Tasks

Work through the tasks in order. A task is true once the check it names passes, run as it names it. The groups are headings for reading.

- **The checks.** Every evaluated case runs against no store, as `nix/check.sh` does, with core.41's neutral names. No check builds a package or reaches the network. A case named in a task is a key of `nix/tests/claude-code.nix` or `nix/tests/tools.nix`, which `sluice exec main -- moon run nix:test` prints as `nix: <case>: <what>` where it fails.
- **What no task adds or changes.**
  - No task adds a crate, a tool of the gate or a tracked file type (core.7, core.8).
  - The runner's code does not change.
  - `activate.sh`'s steps do not change, only its header's words.
  - Every `.nix` file written is formatted by nixfmt, as treefmt formats it.
  - Nothing outside the paths `proposal.md`'s Touches lists is changed.
- **What is already written.** This change's own commit holds record 0046 and core.18's and core.46's details. No task changes them. A task that relies on a choice the design names cites its Decision.

## 1. The option and the document

- [x] 1.1 `nix/tests/claude-code.nix`'s telemetry cases read the telemetry document alone, by its name on NixOS and its path on nix-darwin (design Decision 8; Task notes 1.1). *The document on NixOS* allows the transcripts document beside it under `claude-code`. *No collector on NixOS* and *Not without the runner* read no telemetry document. *In no home* holds every document's path to the platform's managed-settings folder, the telemetry document's among them. True when the cases "The document on NixOS", "The document on nix-darwin", "No collector on NixOS", "No collector on nix-darwin", "Not without the runner on NixOS", "Not without the runner on nix-darwin", "In no home on NixOS" and "In no home on nix-darwin" hold, with `sluice exec main -- moon run nix:test` passing.
- [x] 1.2 `nix/modules/claude-code.nix` declares `services.boomdrive.claude-code.cleanupPeriodDays`, of type `types.ints.positive` and `365` by default. Its description names the document, its two folders and that it is written wherever Flume is on (design Decision 3). The file gives the document's name, `10-boomdrive-transcripts.json`, and `transcriptsOf`, the document `{ cleanupPeriodDays = <days>; }` as a function of the option (design Decision 4). `nix/tests/claude-code.nix` gains the case. True when the cases "A span below one day on nix-darwin" and "A span below one day on NixOS" hold, with `sluice exec main -- moon run nix:test` passing.
- [ ] 1.3 Where `services.flume.enable` is on, whether or not the runner is, `nixos.nix` declares `environment.etc."claude-code/managed-settings.d/10-boomdrive-transcripts.json"`, with `text` the document as `builtins.toJSON` renders it and `mode = "0644"`. Otherwise it declares none (design Decisions 2 and 5; Task notes 1.3 and 1.4). True when the cases "The transcripts document on NixOS", "No Flume on NixOS" and "Flume without the runner on NixOS" hold, with `sluice exec main -- moon run nix:test` passing.
- [ ] 1.4 `darwin.nix` adds a second root line to `system.activationScripts.postActivation.text`, by `lib.mkBefore`, for any system that imports the module (design Decisions 2 and 6; Task notes 1.3 and 1.4):
  - with Flume on, `activate.sh managed` writes the document, `pkgs.writeText "10-boomdrive-transcripts.json"` of it, to `/Library/Application Support/ClaudeCode/managed-settings.d/10-boomdrive-transcripts.json`;
  - otherwise, `activate.sh managed` removes that path.

  `activate.sh`'s header names the managed step's two documents. `nix/tests/tools.nix`'s nix-darwin case drops each document's step by its path and holds each to one (design Decision 7). True when the cases "The transcripts document on nix-darwin", "No Flume on nix-darwin", "Flume without the runner on nix-darwin" and "The tools' activation is their own on nix-darwin" hold, with `sluice exec main -- moon run nix:test` passing.
- [ ] 1.5 `nix/tests/claude-code.nix` holds the document each system writes to its key and value. It reads the NixOS document from `/etc`'s `text`, and holds the nix-darwin step's store path to `pkgs.writeText` of the expected document (design Decisions 4 and 8). True when the cases "A year by default", "A span named" and "The span alone", each on nix-darwin and on NixOS, hold, with `sluice exec main -- moon run nix:test` passing.

## 2. The words and the specs

- [ ] 2.1 The README's **Installing with nix** gains a part on the transcripts' span, after the telemetry part (design Decision 9). Its nix block sets `services.boomdrive.claude-code.cleanupPeriodDays = 365;`. Its words name:
  - `10-boomdrive-transcripts.json` in `/Library/Application Support/ClaudeCode/managed-settings.d` and `/etc/claude-code/managed-settings.d`;
  - that the document comes wherever Flume is on, with the runner or without;
  - that every Claude Code session on the host keeps its transcript that long, whatever its home, so a hibernated stage can be resumed;
  - that a host's own document named after the set's sets the key over it;
  - that a shorter span, or turning Flume off, deletes every older transcript at the next session's start.

  `nix/tests/claude-code.nix` holds the README's words. True when the case "The README's transcripts" holds, with `sluice exec main -- moon run nix:test` passing.
- [ ] 2.2 CLAUDE.md and `openspec/config.yaml` say that the modules write Claude Code's managed transcripts document wherever they install Flume, and still touch no Claude Code home (design Decision 9). True when `sluice exec main -- moon run root:format` passes.
- [ ] 2.3 The spec delta archives into specs that pass openspec's strict check. That covers the requirement renamed *The modules write no file of Claude Code's but its managed telemetry and transcripts documents*. True when, after `devenv shell -- openspec archive the-set-keeps-claude-codes-transcripts-a-year --yes`, `sluice exec main -- moon run root:specs` passes.
