# Proposal

## Why

crew's sessions use two companions:

- **Pending You** posts the cards a session waits on a person for. It wakes the session when one is answered, and herdr shows the cards on each pane.
- **Plannotator** reviews a plan in the browser. plannotator-tui reviews a document in a herdr pane beside the agent's, and crew's conductors open each unit's change in it.

Record 0023's amendment of 2026-11-08 names both among the companions the flake brings. Today no host gets either from the flake. afterthought/swancloud installs both on deneb itself. Its `claude` shim also gives each session of crew's partitions Pending You's hooks, MCP server, skill and wake mod. It does that through Claude Code's flags and `CLAUDE_CODE_PLUGIN_DIRS`.

Record 0044, agentplot-design's ruling of 2026-11-11, decides how the flake brings them:

- **Claude Code loads them itself.** The set installs them as Claude Code plugins with Claude Code's own commands, at user scope, from a marketplace of the set's own. Neither crew nor Flume gains a setting.
- **The rest is built now.** That is the packages, their herdr plugins, Pending You's own folder, one sign-in command, and the set's marketplace.
- **The install into Claude Code's homes waits.** The user's card `req_020ff29dc00c49ec98e8` asks which sessions get the companions. Their answer decides whether Flume first names each account's configuration home, or crew and Flume pass the plugins at launch.
- **Pending You's MCP server is not in the plugin.** A plugin's server is renamed `mcp__plugin_<plugin>_<server>__*`, and Pending You's hooks and words call `mcp__pendingyou__*`. The waiting step adds it with `claude mcp add-json`.
- **The wake mod rides in Pending You's plugin.** It reads its root from Claude Code's mod API, so it cannot run as the folder's copy. A session it wakes says no presence and marks no herdr pane. The record names that loss.

This change is the unit `crews-companions-come-with-the-flake` of the bolt `the-flake-brings-its-companion-tools`, beside `portless-comes-with-the-flake` and `worktrunk-comes-with-the-flake`.

## What Changes

- **The packages.**
  - `pendingyou` is Pending You's command line 0.41.0, as Pending You publishes it on npm, run on the flake's Node.js 24.
  - `plannotator` 0.28.61 and `plannotator-tui` 0.9.9 come through numtide/llm-agents.nix's flake, which builds each from its source at its release tag.
- **Their herdr plugins.**
  - `pendingyou-herdr-plugin` is recordplane/herdr-pendingyou at the commit naming Pending You 0.41.0.
  - `plannotator-herdr-plugin` is plannotator/herdr-annotate, Annotate, at the commit naming plannotator-tui 0.9.9. Its `bin` holds its runtime, built from its own `rust` source, and the set's plannotator-tui, so herdr's link runs no build step of its.
- **The set's marketplace.** `boomdrive-claude-plugins` is one store folder of regular files, no link into the store. It is a Claude Code marketplace named `boomdrive` holding two plugins:
  - `pendingyou`: Pending You's hooks, which run the launcher in Pending You's folder, its wake mod and its skill. It has no MCP server.
  - `plannotator`: the `apps/hook` of Plannotator's own source, as Plannotator's marketplace names it.
- **The modules, with the runner and crew.** With `services.boomdrive` and `services.crew` both on, they:
  - install `services.boomdrive.pendingyou.package`, `services.boomdrive.plannotator.package` and `services.boomdrive.plannotator-tui.package`, each the set's by default, a host's own replacing it, `null` dropping it;
  - add the two herdr plugins to `services.boomdrive.herdr.plugins`;
  - write Pending You's folder at each activation, as the runner's user. It holds the two launchers, the manifest, and the sign-in's Claude folder, each a regular file written only where it differs. Pending You's sign-ins, key and state stay its own.
  - install `pendingyou-signin`, the one command the person runs to sign Claude Code's connection and herdr's in.
- **Claude Code stays the person's own.** No activation step runs `claude` or writes into a Claude Code home. The install into the homes, Pending You's MCP server with it, is the change's last task, which waits on the user's answer and on the unit that answer puts first.
- **The shell** holds the set's `pendingyou`, `plannotator` and `plannotator-tui`.
- **The cache** builds and pushes the new packages with the rest.
- **Words.**
  - The README names the companions, their options, `pendingyou-signin`, and the install that waits.
  - CLAUDE.md and `openspec/config.yaml` name the companions.
  - Record 0044, record 0023's pointer to it, and core.13's, core.18's, core.44's, core.45's and core.46's details are in the commit that writes this change.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `family-packages`: the set gains `pendingyou`, `plannotator`, `plannotator-tui`, `pendingyou-herdr-plugin`, `plannotator-herdr-plugin` and `boomdrive-claude-plugins`. It covers where each comes from, how each is held to its release, and what the marketplace holds.
- `service-modules`: with the runner and crew, the modules install the companions, link their herdr plugins, write Pending You's folder and install the sign-in command. None of Claude Code's homes is touched.
- `binary-cache`: the companions are built and pushed with the rest, and the modules install the flake's own.
- `way-in`: the shell's `PATH` holds the set's Pending You, Plannotator and plannotator-tui.

## Impact

- `flake.nix` and `flake.lock`: four inputs, Pending You's npm package, numtide/llm-agents.nix, recordplane/herdr-pendingyou and plannotator/herdr-annotate. The six packages, and the header's words.
- `nix/packages/`: recipes for Pending You, the two herdr plugins and the marketplace, and Pending You's skill as Pending You serves it.
- `nix/modules/apps.nix`, `nix/modules/companions.nix` (added), `nix/modules/darwin.nix`, `nix/modules/nixos.nix` and `nix/modules/activate.sh`.
- `nix/shell.nix`.
- `nix/tests/`: `typecheck.nix`, `packages.nix`, `companions.nix` (added), `default.nix`, `apps.nix`, `tools.nix`, `way-in.nix`, `cache.nix` and `activate.sh`.
- `README.md`, `CLAUDE.md` and `openspec/config.yaml`.
- The runner's code does not change, and runs no companion (core.1, core.13).
- No crate, tool of the gate or tracked file type is added (core.7, core.8). The gate evaluates the recipes and never builds them.
- crew and Flume do not change. The waiting step's first unit is one of theirs: Flume naming each account's configuration home, or crew and Flume passing the plugins at launch.
- Outside this repository: afterthought/swancloud, once deneb's pin carries the waiting step, drops its own Pending You and Plannotator installs and its shim's Pending You flags and settings (record 0044's consequences). agentplot-design tells afterthought.

## Touches

```
flake.nix
flake.lock
nix/**
README.md
CLAUDE.md
openspec/config.yaml
docs/adr/0023-the-boomdrive-flake-configures-the-whole-host-each-tool-by-its-own-module-and-each-repository-named-once-by-its-owner.md
docs/adr/0044-crews-companions-come-with-the-flake-and-claude-code-loads-them-from-the-sets-own-marketplace.md
docs/architecture/core.md
```
