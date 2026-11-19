# Tasks

Work through the tasks in order. A task is true once the check it names passes, run as it names it. The groups are headings for reading.

- **The checks.** Every evaluated case runs against no store, as `nix/check.sh` does, with core.41's neutral names. No check builds a package or reaches the network. The two steps that reach it are design Task notes 0's, each in the devenv shell outside the sandbox. A case named in a task is a key of a file of `nix/tests/`, or a case of `nix/tests/activate.sh`, which `sluice exec main -- moon run nix:test` prints as `nix: <case>: <what>` where it fails.
- **What no task adds or changes.**
  - No task adds a crate, a tool of the gate or a tracked file type (core.7, core.8).
  - The runner's code does not change.
  - No case reads a Markdown file (record 0049).
  - Every `.nix` file written is formatted by nixfmt, as treefmt formats it.
  - Nothing outside the paths `proposal.md`'s Impact lists is changed.
- **What is already written.** This change's own commit holds record 0044's amendment of 2026-11-18, this design's, and core.18's and core.46's details. No task changes them. A task that relies on a choice the design names cites its Decision.

## 1. Flume first

- [x] 1.1 **The set takes Flume v0.31.0, the release that carries `flumes-status-names-each-accounts-claude-code-folder`**, on a branch that holds the set's sluice 0.97.2 (design Decision 1 and Sequencing; Task notes 1.1):
  - `flake.nix`'s `flume` input names that release's tag, and the lock moves the `flume` node alone;
  - the set's herdr moves where that tag's tree pins another;
  - each option of Flume's module the set gives takes that release's names;
  - `flake.nix`'s comments naming Flume's tag name the new one.

  True when `sluice exec main -- moon run nix:typecheck` passes, with each input's ref the lock's, and `sluice exec main -- moon run nix:test` passes.

## 2. The set's marketplace

- [x] 2.1 The set's marketplace, `boomdrive-claude-plugins`, builds with Flume v0.31.0: its eight plugins each copied whole from its tool's pin, `flume` at `0.31.0`, its build holding each manifest to its tool's version (design Decision 7; Task notes 0). True when `packages.aarch64-darwin.boomdrive-claude-plugins` builds once, in the devenv shell outside the sandbox, and `sluice exec main -- moon run nix:typecheck` passes.
- [x] 2.2 The set's cases of the marketplace hold with Flume v0.31.0 (design Decision 10). True when the cases "The marketplace's plugins", "No link into the store", "Each tool's plugins from its tool's pin", "Each tool's plugins say its tool's version" and "A plugin other than its tool" hold, with `sluice exec main -- moon run nix:test` passing.

## 3. The install into each home

- [ ] 3.1 `nix/modules/activate.sh` gains the verb `claude`, as design Decisions 2 to 4 and 6 have it, and its header names it. `nix/tests/activate.sh` gains its cases, with stubs of `flume` and `claude` (Task notes 3.1):
  - *Two accounts and the default home*, *An account with no home*, *Two accounts sharing one home*, *An account of another program* and *Flume's status cannot be read*;
  - *A fresh home*, *Nothing differs*, *A new build of a plugin*, *A plugin to remove* and *The home's own plugins*;
  - *A home with no server*, *A home that names one* and *No server given*;
  - *No Claude Code*, *A command refused* and *Every command held*.

  True when those cases hold, with `sluice exec main -- moon run nix:test` passing.
- [ ] 3.2 `nix/modules/pendingyou-folder.nix` gives `passthru.server`, the server its `signin/.claude.json` names. `nix/modules/sessions.nix`, new, gives the plugins to install and to remove for a system's config, and the server's file where Pending You's folder is written (design Decisions 5 and 6). True when `sluice exec main -- moon run nix:typecheck` passes.
- [ ] 3.3 `nix/modules/darwin.nix` runs `activate.sh claude` in `postActivation` by `lib.mkAfter`. `nix/modules/nixos.nix` runs it last of the runner's user's steps before `keep`, and its user activation gains `deps = [ "flume" ]`. Each runs it where the runner and Flume, with a package, are on, with the profiles' `PATH` and `$HOME/.local/bin`, and each header names it, as `flake.nix`'s comment on the marketplace and `nix/modules/companions.nix`'s header say the modules install it (design Decision 3). `nix/tests/sessions.nix`, new, read by `nix/tests/default.nix`, holds the cases:
  - *The step on nix-darwin*, *The step on NixOS*, *No step without the runner* and *No step without Flume's package*;
  - *Every plugin with the runner and crew*, *No companions' plugins without crew* and *A tool turned off*;
  - *The server's file*.

  True when those cases hold, each on nix-darwin and on NixOS where it says so, with `sluice exec main -- moon run nix:test` passing.
- [ ] 3.4 `nix/tests/apps.nix`'s *No Claude Code of the modules'* allows `activate.sh claude`'s step alone, run once. `nix/tests/activate.sh` gains *The step's commands*, holding the verb to the six commands the requirement names. `nix/tests/tools.nix`'s *The tools' activation is their own on nix-darwin* takes the step out with the runner's other steps (design Decision 10). True when the cases "No Claude Code of the modules'", "The step's commands", "The tools' activation is their own on nix-darwin" and "The tools' activation is their own on NixOS" hold, with `sluice exec main -- moon run nix:test` passing.

## 4. The wake mod in every session

- [ ] 4.1 `nix/modules/plugin-dirs.sh`, new, holds design Decision 8's lines. `nix/modules/sessions.nix` puts them in `environment.extraInit` where Pending You's folder is written, on nix-darwin and on NixOS. `nix/tests/activate.sh` gains the cases *A fresh environment*, *A person's own value*, *Run twice* and *No folder*. `nix/tests/sessions.nix` gains *The lines with the folder* and *No lines without the folder* (Task notes 4.1). True when those cases hold, with `sluice exec main -- moon run nix:test` passing.

## 5. The words

- [ ] 5.1 The README's **Installing with nix** says what design Decision 9 lists, and no longer says no Claude Code home is installed into. No check reads these words (record 0049). True when `sluice exec main -- moon run root:format` passes.
- [ ] 5.2 CLAUDE.md and `openspec/config.yaml` say the modules install the set's marketplace and the tools' plugins into each Claude Code home of Flume's accounts and the default home, by Claude Code's own commands, with Pending You's server and the wake mod's variable. They no longer say no module runs `claude` or touches a Claude Code home. True when `sluice exec main -- moon run root:format` passes.

## 6. The specs

- [ ] 6.1 The change's deltas pass openspec's strict check. True when `sluice exec main -- moon run root:changes` passes.
- [ ] 6.2 The deltas archive into specs that pass openspec's strict check. Each holds the base's words where `the-set-takes-flume-v0-29-0` or `boomdrives-tools-install-on-a-machine-without-the-runner` landed first (design Sequencing). True when, after `devenv shell -- openspec archive every-claude-code-session-has-boomdrives-plugins --yes`, `sluice exec main -- moon run root:specs` passes.
