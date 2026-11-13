# Tasks

Work through the tasks in order. A task is true once the check it names passes, run as it names it. The groups are headings for reading.

- **The checks.** Every evaluated case runs against no store, as `nix/check.sh` does, with core.41's neutral names. No check builds a package or reaches the network.
- **What no task adds or changes.**
  - No task adds a crate, a tool of the gate or a tracked file type (core.7, core.8).
  - The runner's code does not change.
  - Every `.nix` file written is formatted by nixfmt, as treefmt formats it.
  - Nothing outside the paths `proposal.md`'s Touches lists is changed.
- **What is already written.** This change's own commit holds:
  - record 0013's amendment of 2026-11-12, the set's sessions in Claude Code's managed settings;
  - core.18's, core.28's and core.46's details.

  No task changes them. A task that relies on a choice the design names cites its Decision.

## 1. The option and the document

- [x] 1.1 `nix/modules/claude-code.nix` is added, and `darwin.nix` and `nixos.nix` import it. It declares `services.boomdrive.claude-code.telemetry` (design Decision 2; Task notes 1.1):
  - `collector`, `null` by default, an `http://` address on the host's loopback;
  - `metrics`, `logs` and `traces`, `true`, `true` and `false` by default.

  A collector named with every signal off fails evaluation, naming the three. The file also gives `documentOf`, the document as a function of the option (design Decision 3). `nix/tests/claude-code.nix` is added to `nix/tests/default.nix`'s `test`. True when `sluice exec main -- moon run nix:test` passes, proving the service-modules delta's "A collector off the host" and "A collector with no signal", on nix-darwin and on NixOS.
- [x] 1.2 Where the runner is on and a collector is named, `nixos.nix` declares `environment.etc."claude-code/managed-settings.d/10-boomdrive-telemetry.json"`, with `text` the document as `builtins.toJSON` renders it and `mode = "0644"`. Otherwise it declares none (design Decisions 6 and 7). True when `sluice exec main -- moon run nix:test` passes, proving the service-modules delta's "The document on NixOS", "No collector on NixOS" and "Not without the runner" on NixOS.
- [x] 1.3 `activate.sh managed <path> [<source>]` writes the source to the path, whole, mode 0644, only where it differs, replacing a link and making a missing folder 0755. Given no source, it removes the file or link at the path and nothing else. It says a line for each write, removal or failure, and exits 0. Its header names the step, and its cases are in `nix/tests/activate.sh` (design Decision 9; Task notes 1.3). True when `sluice exec main -- moon run nix:test` passes, proving the service-modules delta's:
  - "A fresh host", "Nothing differs", "A link where the document goes" and "A folder it cannot write";
  - "The document removed" and "Nothing to remove".
- [x] 1.4 `darwin.nix` adds one root line to `system.activationScripts.postActivation.text`, before the user's steps, for any system that imports the module (design Decisions 6 and 8):
  - with the runner on and a collector named, `activate.sh managed` writes the document, the document being `pkgs.writeText "10-boomdrive-telemetry.json"`, to `/Library/Application Support/ClaudeCode/managed-settings.d/10-boomdrive-telemetry.json`;
  - otherwise, `activate.sh managed` removes that path.

  True when `sluice exec main -- moon run nix:test` passes, proving the service-modules delta's "The document on nix-darwin", "No collector on nix-darwin" and "Not without the runner" on nix-darwin.
- [x] 1.5 `nix/tests/claude-code.nix` holds the document each system writes to its keys and values (design Decisions 3 to 5). It reads the NixOS document from `/etc`'s `text`, and holds the nix-darwin step's store path to `pkgs.writeText` of the expected document (design Decision 12). True when `sluice exec main -- moon run nix:test` passes, proving the service-modules delta's:
  - "In no home" and "The telemetry keys alone";
  - "Metrics and logs by default", "Traces named", "Each signal's endpoint" and "No general endpoint";
  - "The content settings off" and "The account and repository attributes on";

  each on nix-darwin and on NixOS.

## 2. What the modules hold to

- [ ] 2.1 `nix/tests/apps.nix`'s `noClaudeCode` reads a system with `services.boomdrive` and `services.crew` on and a collector named, beside the one with none (design Decision 10). True when `sluice exec main -- moon run nix:test` passes, proving the service-modules delta's "No Claude Code of the modules'" on nix-darwin and on NixOS.
- [ ] 2.2 `nix/tests/tools.nix`'s nix-darwin case drops the managed document's line, beside the runner's link, Pending You's folder and the plugins' links, and holds that line to one (design Decision 10). True when `sluice exec main -- moon run nix:test` passes, proving the service-modules delta's "The tools' activation is their own".

## 3. The words and the specs

- [ ] 3.1 The README's **Installing with nix** gains a part on the sessions' telemetry (design Decision 11). Its nix block sets `services.boomdrive.claude-code.telemetry.collector = "http://localhost:4318";`, and its words name:
  - `metrics`, `logs` and `traces`, and that traces are Claude Code's beta, off unless named;
  - `10-boomdrive-telemetry.json` in `/Library/Application Support/ClaudeCode/managed-settings.d` and `/etc/claude-code/managed-settings.d`;
  - that every Claude Code session on the host exports to that collector alone, whatever its home, and with none of its words;
  - that nothing is written without a collector or the runner;
  - that a host's own document named after the set's sets a key over it;
  - that a Desktop session's launch environment and `workspace.host_paths` are beyond the document.

  `nix/tests/claude-code.nix` holds the README's words. True when `sluice exec main -- moon run nix:test` passes, proving the service-modules delta's "The README's telemetry".
- [ ] 3.2 CLAUDE.md and `openspec/config.yaml` say that the modules write Claude Code's managed telemetry document where the host names a collector, and still touch no Claude Code home (design Decision 11). True when `sluice exec main -- moon run root:format` passes.
- [ ] 3.3 The spec delta archives into specs that pass openspec's strict check. That covers the requirement renamed *The modules write no file of Claude Code's but its managed telemetry document*. True when, after `devenv shell -- openspec archive every-claude-code-session-exports-to-the-hosts-collector --yes`, `sluice exec main -- moon run root:specs` passes.
