# Tasks

Work through the tasks in order. A task is true once the check it names passes, run as it names it. The groups are headings for reading.

- **The checks.** Every evaluated case runs against no store, as `nix/check.sh` does, with core.41's neutral names. No check builds a package or reaches the network. The three steps that reach it are design Task notes 0's, each in the devenv shell outside the sandbox.
- **What no task adds or changes.** No task adds a crate, a tool of the gate or a tracked file type (core.7, core.8). The runner's code does not change. Every `.nix` file written is formatted by nixfmt, as treefmt formats it. Nothing outside the paths proposal.md's Touches lists is changed.
- **What is already written.** Record 0044, record 0023's amendment of 2026-11-11, and core.13's, core.18's, core.44's, core.45's and core.46's details are written in this change's commits, record 0044's points 3 and 5 with the user's answer and agentplot-design's ruling on the wake mod, and no task changes them. The install into Claude Code's homes is the unit `the-companions-load-in-claude-codes-sessions` (design Decision 12), and no task here builds it. A task that relies on a choice the design names cites its Decision.

## 1. The companions in the set

- [x] 1.1 `flake.nix` gains design Decision 1's four inputs, `pendingyou`, `llm-agents`, `herdr-pendingyou` and `herdr-annotate`, and the lock locks them. `packages.<system>.pendingyou` is `nix/packages/pendingyou.nix`'s copy of the npm package's `dist`, `mod` and `package.json`. It is run by a `makeWrapper` of the flake's `nodejs_24`, with `passthru.mod` and `passthru.node`, and its version is its `package.json`'s. The flake's header names the companions. The type check's set holds `pendingyou` (design Decisions 1 and 2). True when `sluice exec main -- moon run nix:typecheck` passes, with each input's ref the lock's.
- [x] 1.2 `packages.<system>.plannotator` and `packages.<system>.plannotator-tui` are llm-agents.nix's packages of those names, unchanged, and the type check's set holds both (design Decision 3; Task notes 1.2). True when `sluice exec main -- moon run nix:typecheck` passes.
- [x] 1.3 `packages.<system>.pendingyou-herdr-plugin` is `nix/packages/pendingyou-herdr-plugin.nix`'s whole copy of its input, its version its `herdr-plugin.toml`'s. `packages.<system>.plannotator-herdr-plugin` is `nix/packages/plannotator-herdr-plugin.nix`'s copy of Annotate. Its `bin` holds the runtime built from the input's `rust` folder with that folder's `Cargo.lock`, and the set's plannotator-tui's program copied, each beside its version file, with `passthru.runtime` and `passthru.tui`. The type check's set holds both (design Decision 4; Task notes 1.3). True when `sluice exec main -- moon run nix:typecheck` passes.
- [x] 1.4 `nix/tests/companions.nix`, added to `nix/tests/default.nix`'s test, holds the packages to their releases (design Decisions 2 to 4). True when `sluice exec main -- moon run nix:test` passes, proving the family-packages delta's:
  - "Pending You from its npm package" and "Pending You other than its package";
  - "Pending You's Node.js";
  - "Plannotator and plannotator-tui through llm-agents.nix" and "Plannotator other than its tag";
  - "Pending You for Herdr names the set's Pending You" and "Pending You for Herdr other than the set's";
  - "Annotate holds its runtime and the set's plannotator-tui" and "Annotate other than the set's plannotator-tui".

## 2. The set's marketplace

- [ ] 2.1 `nix/packages/pendingyou-skill.md` is the text `https://www.pendingyou.com/skill-stub.md` serves, unchanged. `packages.<system>.boomdrive-claude-plugins` is `nix/packages/claude-plugins.nix`'s marketplace `boomdrive`. It holds:
  - the plugin `pendingyou`: Pending You's hooks, naming the launcher by `$HOME`, and the skill. No wake mod and no MCP server.
  - the plugin `plannotator`: the `apps/hook` of the set's Plannotator's `src`.

  Each file is copied with links followed, the build fails on any link, and `passthru.plugins` and `passthru.hooks` are given. The type check's set holds it (design Decision 5; Task notes 2.1). True when `sluice exec main -- moon run nix:typecheck` passes, proving the family-packages delta's "Every system's packages" with the sixteen.
- [ ] 2.2 `nix/tests/companions.nix` holds the marketplace to its plugins, its hooks to the set's Pending You's `dist/claude.js`, and its build to no link (design Decision 5; Task notes 2.1). True when `sluice exec main -- moon run nix:test` passes, proving the family-packages delta's "The marketplace's two plugins", "Pending You's hooks", "Pending You's hooks against its own", "No wake mod in the plugin", "No MCP server in the plugin", "Plannotator's plugin from its command's source" and "No link into the store".

## 3. The modules

- [x] 3.1 `nix/modules/companions.nix`, imported by `darwin.nix` and `nixos.nix`, declares `services.boomdrive.pendingyou.package`, `services.boomdrive.plannotator.package` and `services.boomdrive.plannotator-tui.package`. With the runner and crew on, it installs each that is not `null`, and `pendingyou-signin` where Pending You's is not. `apps.nix`'s `herdr.plugins` default gains the two companions' plugins where crew is on (design Decisions 6 and 9). True when `sluice exec main -- moon run nix:test` passes, proving the service-modules delta's:
  - "The companions with the runner and crew", "A host's own Plannotator", "No companions without crew" and "No companions without the runner";
  - "The companions' plugins with crew";
  - "The sign-in's two commands";

  each on nix-darwin and on NixOS where it says so.
- [ ] 3.2 `nix/modules/pendingyou-folder.nix` gives the folder's four files and the given Pending You's wake mod, `mod`, for a home and a package. `darwin.nix` and `nixos.nix` run `activate.sh pendingyou <source> <home>/.config/pendingyou` as the runner's user, before the plugins' step, where the companions are installed and Pending You's package is not `null` (design Decisions 7 and 8). True when `sluice exec main -- moon run nix:test` passes, proving the service-modules delta's "The folder written on nix-darwin", "The folder written on NixOS", "The launchers run the given Pending You", "The wake mod in the folder", "A crew session's stop check" and "No Pending You of the set's".
- [ ] 3.3 `activate.sh pendingyou` writes each file of its source whole, with its mode, only where it differs, the wake mod's `hooks/wake.js`, `hooks/register.js` and `hooks/hooks.json` first. It replaces a link, makes a missing folder 0700, touches nothing else, says a line for each file written or not written, and exits 0. Its cases are in `nix/tests/activate.sh` (design Decision 8). True when `sluice exec main -- moon run nix:test` passes, proving the service-modules delta's "A fresh home", "The wake mod's order", "Nothing differs", "The command line's own files", "A link where a launcher goes" and "A file it cannot write".
- [x] 3.4 The tests hold the modules to no `claude`, Claude Code home or marketplace in any step, job or unit. They hold the tools' activation to their own but for the runner's link, Pending You's folder and the plugins' links (design Decisions 7, 8 and 12). True when `sluice exec main -- moon run nix:test` passes, proving the service-modules delta's "No Claude Code of the modules'" and "The tools' activation is their own".
- [x] 3.5 The cache's cases hold the companions to the flake's own whatever the host's `pkgs`, and to the packages each system's job builds (design Decisions 1 and 6). True when `sluice exec main -- moon run nix:test` passes, proving the binary-cache delta's "A host's own nixpkgs" and "The runner is pushed with its tools".

## 4. The shell and the words

- [x] 4.1 The flake's shell holds the set's `pendingyou`, `plannotator` and `plannotator-tui` after portless (design Decision 10). True when `sluice exec main -- moon run nix:test` passes, proving the way-in delta's "Every system's shell" with the nine tools.
- [x] 4.2 The README's **Installing with nix** names (design Decision 11):
  - the companions and their herdr plugins;
  - `#pendingyou`, `#plannotator` and `#plannotator-tui` on the profile's line;
  - the three options and their `null`, and that they come with the runner where crew is on;
  - `~/.config/pendingyou` and what stays the command line's;
  - `pendingyou-signin` as the person's one sign-in;
  - `boomdrive-claude-plugins`, installed into no Claude Code home yet.

  True when `sluice exec main -- moon run nix:test` passes, proving the service-modules delta's "The README's words" and "The README's companions".
- [x] 4.3 CLAUDE.md and `openspec/config.yaml` name the companions in the flake's set and among what the modules install with the runner and crew. True when `sluice exec main -- moon run root:format` passes.

## 5. The install that waits, and the specs

- [ ] 5.1 The spec deltas, each holding the base's words where `worktrunk-comes-with-the-flake` landed first (design Sequencing; Task notes 4.1), archive into specs that pass openspec's strict check. True when, after `devenv shell -- openspec archive crews-companions-come-with-the-flake --yes`, `sluice exec main -- moon run root:specs` passes.
