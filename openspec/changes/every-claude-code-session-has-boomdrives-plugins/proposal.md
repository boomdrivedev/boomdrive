# Proposal

## Why

The brief:

> Every Claude Code session on the accounts Boomdrive runs, your own included, has every Claude Code plugin Boomdrive's tools ship, with Pending You's server and the add-on that wakes an agent when you answer, so each agent and your own session show in herdr that they are there; nothing else in Claude Code changes.

The set builds the plugins already. `boomdrive-claude-plugins` is a Claude Code marketplace named `boomdrive` holding Pending You's, Plannotator's and timber's plugins. The modules write Pending You's folder, with its wake mod, its launchers and its headers helper. But no Claude Code home is installed into, so no session loads any of it. Record 0044 leaves the install to this unit:

- **Point 1.** Claude Code loads the plugins itself. The set installs them with `claude plugin marketplace add` and `claude plugin install`, at user scope.
- **Point 3.** Which sessions get them was the user's call. They answered on their card `req_020ff29dc00c49ec98e8`: "Every session on those accounts, mine included". Activation installs into each home Flume's status names, idempotently, and changes nothing else there. It adds Pending You's MCP server with Claude Code's own command.
- **Point 4.** Pending You's MCP server is not in the plugin. Claude Code renames a plugin's server, and Pending You's hooks and words call `mcp__pendingyou__*`.
- **Point 5.** Each session loads the folder's wake mod by `CLAUDE_CODE_PLUGIN_DIRS` in its environment. That is an environment variable, not a Claude Code file. So presence, the loaded note and herdr's pane marks hold.
- **The amendment of 2026-11-18.** The marketplace lists every Claude Code plugin a tool of the set ships. That is Flume's `flume`, and sluice's `sluice` and `sluice-board`, at the set's versions. This unit installs every plugin the marketplace lists. Nothing has built those three plugins into the marketplace yet.

The homes come from Flume's status. Flume's statement, C.4's row `home`, gives the field: "Status gives it with the account … So a tool that installs into each account's home, a companion's plugin among them, reads the homes from status and never from the host's configuration". No Flume release gives it yet. Flume's `AccountStatus`, at the set's v0.26.0 and at Flume's main, holds "never a value of its environment or a path". This change's construct stage stopped short on that. boomdrive-design answered on 2026-11-18:

> (a). Record 0044 point 3 holds: the set installs into each account's Claude Code folder with Claude Code's own commands, and reads those folders from Flume's status, as Flume's statement C.4 'home' has it. Not (b): a third managed-settings document locks the plugins on and replaces a person's own CLAUDE_CODE_PLUGIN_DIRS, against 0044 points 1 and 3 and core.18. I queued the Flume unit flumes-status-names-each-accounts-claude-code-folder. Plan this change with a task that moves the set's Flume to the release carrying it and installs per folder as 0044 point 3 says. Its code waits on that release, which the planner orders ahead of it.

This change is the unit `every-claude-code-session-has-boomdrives-plugins` of the bolt `every-machine-takes-boomdrives-tools`.

## What Changes

- **Flume first.** The set takes the Flume release that carries `flumes-status-names-each-accounts-claude-code-folder`, whose status gives each account's `home`. The lock moves with it. This change's code waits on that release.
- **The marketplace lists every tool's plugin.** `boomdrive-claude-plugins` gains three plugins:
  - `flume`, Flume's `plugins/claude`;
  - `sluice`, sluice's `plugins/sluice`;
  - `sluice-board`, sluice's `plugins/sluice-board`.

  Each is copied whole from the tree the set builds its tool from, since neither flake builds the plugin. The build fails where a plugin's manifest says another version than the set's tool. The marketplace then holds six plugins.
- **Every home is installed into.** With the runner and Flume on, each activation runs one step as the runner's user. It runs after Flume's own link of its file, after Pending You's folder and after herdr's links. It reads the homes from the set's Flume's `flume --json status`: each `claude` account's `home`, and Claude Code's default home. In each home it runs only Claude Code's own commands:
  - `claude plugin marketplace add`, naming the build's marketplace, which adds `boomdrive` or points it at the new build;
  - `claude plugin install` for each plugin of a tool the host installs that the home lacks;
  - `claude plugin update` for each such plugin whose installed version is not the marketplace's;
  - `claude plugin uninstall` for each plugin of the `boomdrive` marketplace whose tool the host no longer installs;
  - `claude mcp add-json --scope user pendingyou`, where Pending You's folder is written and the home names no server `pendingyou`.

  It leaves every other plugin, marketplace, server and setting of the home as it is. It stops nothing: a missing `claude` or `flume`, a status it cannot read and a command Claude Code refuses are each said in one line.
- **Every session loads the wake mod.** Where the modules write Pending You's folder, the login environment, `environment.extraInit`, puts `~/.config/pendingyou/mod` on `CLAUDE_CODE_PLUGIN_DIRS`. It does so for a user whose folder holds the mod, once, ahead of any value already there. Flume reads each agent's environment from that login environment, and a person's own shell starts from it.
- **Words.** The README's install with nix says the plugins are installed into each home Flume's accounts use and the default home. It names what each switch runs there, what it leaves, and the wake mod's variable. CLAUDE.md and `openspec/config.yaml` say the same. No check reads them (blueprints principle 16).
- **Records and rules.** Record 0044 gains an amendment of 2026-11-18 holding boomdrive-design's answer and this design's choices. core.18's and core.46's details name the step and the login environment. Both are written in the commit that writes this change.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `service-modules`: each activation installs the set's marketplace and the plugins of the tools the host installs, with Pending You's server, into every Claude Code home of Flume's accounts and the default home, by Claude Code's own commands. The login environment puts the wake mod on `CLAUDE_CODE_PLUGIN_DIRS`. The modules still write no file of Claude Code's, and the README says the plugins are installed.
- `family-packages`: the set's marketplace lists Flume's, sluice's and sluice-board's plugins beside the companions' and timber's, each at its tool's version in the set.

## Impact

- `flake.nix`, `flake.lock`: Flume's input at the release that gives each account's `home`.
- `nix/packages/claude-plugins.nix`: the three plugins.
- `nix/modules/activate.sh`: the step, `activate.sh claude`.
- `nix/modules/sessions.nix`, new: which plugins a host installs and removes, the server's file, and the login environment's lines, which are `nix/modules/plugin-dirs.sh`, new.
- `nix/modules/darwin.nix`, `nix/modules/nixos.nix`, `nix/modules/companions.nix`, `nix/modules/pendingyou-folder.nix`.
- `nix/tests/`: `activate.sh`, `sessions.nix` (new, in `default.nix`), `companions.nix`, `apps.nix`, `tools.nix`, `lib.nix`.
- `README.md`, `CLAUDE.md`, `openspec/config.yaml`.
- `docs/adr/0044-…md` and `docs/architecture/core.md`, in the commit that writes this change.
- **What does not change.**
  - The runner's code does not change and runs no `claude`.
  - No crate, gate tool or tracked file type is added (core.7, core.8).
  - No managed-settings document is added.
  - No module writes a file of Claude Code's: Claude Code's commands write what they write in each home.
- **On a host.**
  - The first switch that takes this change adds the marketplace and the plugins in each home, and Pending You's server where none is named.
  - A home that installed the same plugin from another marketplace, as `plannotator@plannotator`, keeps both until the person uninstalls their own. The README says so.
  - A session started before the switch loads the plugins at its next start, or after `/reload-plugins`.
- **Outside this repository.** afterthought/swancloud can drop its `claude` shim's flags and `CLAUDE_CODE_PLUGIN_DIRS` for Pending You, and its own installs of Flume's and Plannotator's plugins, once it takes this change.
