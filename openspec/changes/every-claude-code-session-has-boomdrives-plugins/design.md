# Design

## Context

Record 0044 decides how Claude Code loads the set's plugins:

- **Point 1.** With Claude Code's own commands, at user scope, from the set's marketplace `boomdrive`.
- **Point 3.** Into each home Flume's status names, with Pending You's MCP server.
- **Point 5.** The wake mod from Pending You's folder, by `CLAUDE_CODE_PLUGIN_DIRS` in each session's environment.

Its amendments of 2026-11-18 add every tool's own plugin to the marketplace and give each companion a switch of its own. boomdrive-design's answer of 2026-11-18, quoted in the proposal, holds point 3: the homes come from Flume's status, which Flume v0.31.0 gives.

What stands today:

- **The marketplace.** `nix/packages/claude-plugins.nix` builds `boomdrive`, one store folder of regular files, with eight plugins, each at the set's version of its tool: `pendingyou`, `plannotator`, `timber`, `flume`, `sluice`, `sluice-board`, `crew` and `worktrunk` (the fix `the-sets-marketplace-holds-every-tools-claude-code-plugins`).
- **Pending You's folder.** `activate.sh pendingyou` writes it as the set's user, where `services.boomdrive.pendingyou.enable` is on with a package, with the runner or without. Its `signin/.claude.json` names Pending You's server: `type` `http`, `url` `https://www.pendingyou.com/mcp`, and `headersHelper` the folder's `bin/pendingyou-mcp-headers`.
- **What no module does.** No step runs `claude`. The test *No Claude Code of the modules'* holds every step to that.
- **Flume's status.** `flume --json status` gives `.status.host.pools[].accounts[]`, each with its `name` and `program`. From v0.31.0, each also gives `home`, its configuration home, as Flume's statement, C.4's row `home`, has it: Claude Code's `CLAUDE_CONFIG_DIR` its variables give, read against the login environment, and `null` where they give none. Where it cannot be read, `home` is `null` and `home_unread` says why.
- **Flume's agents.** Each agent's environment is the host user's login environment, read by `$SHELL -l -c 'env -0'`, with the account's variables over it (Flume's 29). Flume starts each place's agent with its own plugin by `--plugin-dir`.

What Claude Code 2.1.322 does, tried in a scratch home on 2026-11-18:

- **`claude plugin marketplace add <folder> --json`.** The first time, it adds a `directory` marketplace. Run again with the same folder, it answers "already on disk". Run with another folder of the same name, it re-points the marketplace and keeps its plugins: "Plugins already installed from it now update from the new source". Each exits 0. It writes `plugins/known_marketplaces.json` and `extraKnownMarketplaces` in the home's `settings.json`.
- **Install.** `claude plugin install <p>@boomdrive --json` installs at user scope, and answers "already installed" where the plugin is. A plugin of a `directory` marketplace "loads in place" from the marketplace's folder, which is read at each session start.
- **The list.** `claude plugin list --json` gives each plugin's `id`, its `version` as installed, and the marketplace folder's `folderVersion`.
- **Update.** `claude plugin update <p>@boomdrive --scope user --json` moves the installed version to the folder's. It answers `up_to_date` where they are one.
- **Uninstall.** `claude plugin uninstall <p>@boomdrive --json` removes the plugin. It exits 1 with `not_installed` where the home lacks it.
- **The server.** `claude mcp add-json --scope user pendingyou '<json>'` writes the server into `$CLAUDE_CONFIG_DIR/.claude.json`. It exits 1 and changes nothing where the home names one already.
- **What it reads.** `CLAUDE_CODE_PLUGIN_DIRS` takes absolute paths, `:`-separated, and loads each as a plugin for the session. A plugin loaded by `--plugin-dir` replaces an installed plugin of the same name for that session.

## Goals / Non-Goals

**Goals:**

- Every Claude Code session on a host's Flume accounts, and in Claude Code's default home, loads every plugin of a tool the set installs there, with Pending You's server and wake mod.
- The modules write no file of Claude Code's. Claude Code's own commands write what they write.
- Nothing else in a home changes.

**Non-Goals:**

- **Flume's field.** That is Flume's unit `flumes-status-names-each-accounts-claude-code-folder`.
- **A managed-settings document for plugins or servers.** That is design's (b), which boomdrive-design declined.
- **Homes no Flume account uses**, but the default home.
- **The tools' installs on a machine without the runner.** That is the bolt's unit `boomdrives-tools-install-on-a-machine-without-the-runner`.
- **Removing a home's own install of a plugin from another marketplace**, such as `plannotator@plannotator`.

## Decisions

### 1. The set takes the Flume that gives each account's home

`flake.nix`'s Flume input moves to v0.31.0, the release carrying `flumes-status-names-each-accounts-claude-code-folder`, whose `flume --json status` gives each account's `home`. The lock moves with it (core.45). Its tree pins herdr 0.9.5, as v0.29.0's did, so the set's herdr stays (the family-packages spec, herdr is the release Flume's terminal profile was checked against), and its module renames no option the set gives it. Flume past v0.29.0 stops pushing sluice, so it comes only beside sluice 0.97 or later: the set's sluice 0.97.2, from the fix `the-set-takes-each-tools-latest-release`.

Alternative: reading the homes from `services.flume.settings`' accounts. Not taken: Flume's C.4 says a tool "reads the homes from status and never from the host's configuration".

### 2. The homes: each `claude` account's, and the default

The step reads the homes from the set's Flume, by `<flume> --json status`, as `.status.host.pools[].accounts[]`:

- **An account's home.** Each account whose `program` is `claude` and whose `home` is a path gives that path. One whose `home` is `null`, whether or not `home_unread` says why, gives none.
- **The default home.** Claude Code's default home, `CLAUDE_CONFIG_DIR` unset, is always a home. It holds Flume's agents for an account that gives no home, an agent under no account, and a person's own session started with no `CLAUDE_CONFIG_DIR`.

Each home is installed into once, in the order status gives it, the default home first. An account of another program is skipped. Where Flume's status cannot be read, the step says so in one line and installs into the default home alone.

Alternative: the default home only where an account gives none. Not taken: a person's own session in the default home is theirs on every host, and the user's answer includes their own sessions.

### 3. One step, `activate.sh claude`, after Flume's link and Pending You's folder

```
activate.sh claude <flume> <jq> <marketplace> <server> <plugin>... [-- <plugin>...]
```

- **The arguments.** `<flume>` is the host's Flume's program, `services.flume.package`'s `flume`. `<jq>` is the flake's `jq`. `<marketplace>` is the store folder of `boomdrive-claude-plugins`. `<server>` is a store file holding Pending You's server's JSON, or `-` for none. The plugins before `--` are installed, and those after it are removed.
- **Where it runs.** It runs as the runner's user with `HOME` that user's home. Its `PATH` is the profiles' folders the runner's service runs with, then `$HOME/.local/bin`, where Claude Code's own installer puts `claude`. It finds `claude` there. Where none is found, it says so in one line and runs nothing.
- **When it runs.** It runs where `services.boomdrive.enable` is on and `services.flume.enable` is on with a package. It runs after Flume's own step, so Flume reads the switch's file:
  - on nix-darwin, after Pending You's folder and herdr's links in `postActivation`, by `lib.mkAfter`;
  - on NixOS, last of the runner's user's steps before `keep`, in `system.userActivationScripts.boomdrive`, which gains `deps = [ "flume" ]`.
- **What it says.** It exits 0. Each home it cannot read and each command Claude Code refuses or fails is one line, naming the home, the command and Claude Code's words (the `message` of its `--json` answer). Nothing is said where every command held.

Alternatives:

- **A launchd job or a systemd unit.** Not taken: the homes change only with Flume's file, which only a switch changes.
- **The runner at its pass.** Not taken: the runner runs no tool's housekeeping of its own accord (core.13), and runs only what its control plane asks (core.1).

### 4. In each home, only Claude Code's own commands, and only for `boomdrive`

For each home, with `CLAUDE_CONFIG_DIR` set to it, or unset for the default, and standard input from `/dev/null`, the step runs in this order:

1. `claude plugin marketplace add <marketplace> --json`. It adds `boomdrive`, or points it at this build's folder.
2. `claude plugin list --json`, read with `<jq>` for the ids `<p>@boomdrive` and their `version`.
3. For each plugin to install:
   - `claude plugin install <p>@boomdrive --json` where the list lacks it;
   - `claude plugin update <p>@boomdrive --scope user --json` where its installed `version` is not the marketplace's, read from `<marketplace>/.claude-plugin/marketplace.json`.
4. For each plugin to remove that the list holds: `claude plugin uninstall <p>@boomdrive --json`.
5. Where `<server>` is given and the home's own `.claude.json` names no `mcpServers.pendingyou`: `claude mcp add-json --scope user pendingyou <the server's JSON>`. The home's `.claude.json` is `$CLAUDE_CONFIG_DIR/.claude.json`, else `$HOME/.claude.json`.

So a switch that changes nothing runs two commands in each home: the marketplace's add and the list. The step never names another marketplace, another plugin or another server, and never removes the marketplace or Pending You's server. Claude Code writing its own files by its own commands is no module writing them (record 0044, point 1; core.18).

Alternatives:

- **`install` and `update` at every switch.** Not taken: that is two `claude` starts more for each plugin and home at every switch, for nothing.
- **A copy of the marketplace at a stable path in the home.** Not taken: `marketplace add` re-points the marketplace and keeps its plugins, and a copy would be one more folder the set writes in the home.
- **`claude mcp get pendingyou` to learn whether the server is named.** Not taken: it reaches the server, by its helper, to check its health.

### 5. The plugins of the tools the host installs

The module splits the marketplace's plugins into those installed and those removed:

| plugin | installed where |
|---|---|
| `flume` | `services.flume.enable`, with a package |
| `sluice`, `sluice-board` | `services.sluice.enable`, with a package |
| `crew` | `services.crew.enable`, with a package |
| `timber` | `services.timber.enable`, with a package |
| `worktrunk` | `services.boomdrive.worktrunk.enable`, with a package |
| `pendingyou` | `services.boomdrive.pendingyou.enable`, with a package |
| `plannotator` | `services.boomdrive.plannotator.enable`, with a package |

With the runner, Flume's, sluice's and timber's modules and worktrunk are on by default, and the companions where crew is on too, each a host's own value winning (record 0023 and record 0044 as amended on 2026-11-18). Each plugin not installed is removed, so a tool turned off leaves no plugin that calls it. With the runner and crew on and no package `null`, every plugin the marketplace lists is installed, as the amendment of 2026-11-18 asks.

Alternative: every plugin on every host with the runner. Not taken: Pending You's hooks without its folder, and Flume's without Flume, would run for nothing in every session.

### 6. Pending You's server, the folder's

`nix/modules/pendingyou-folder.nix` gives `passthru.server`, the JSON of the server its `signin/.claude.json` names. The module writes it into a store file, which is `<server>`, where the folder is written: Pending You's switch on, its package not `null`. Elsewhere `<server>` is `-`.

The step adds it only where the home names no `pendingyou`, and never removes or rewrites one. A person's own `pendingyou` stays theirs.

### 7. The marketplace as the set builds it

The step installs from `boomdrive-claude-plugins` as it stands: each plugin copied whole from its tool's pin, and held by the build to the set's version of its tool (record 0044's amendment of 2026-11-18). The set's Flume moving to v0.31.0 moves the `flume` plugin with it.

Each plugin loads in place from the store, needing no package beside it but its tool's, which Decision 5 installs it beside.

In a place's agent, Flume's `--plugin-dir` copy replaces `flume@boomdrive` for that session. So Flume's hooks run once.

### 8. The wake mod on the login environment's `CLAUDE_CODE_PLUGIN_DIRS`

Where the modules write Pending You's folder, `environment.extraInit` gains these lines, on nix-darwin and on NixOS:

```sh
if [ -d "$HOME/.config/pendingyou/mod" ]; then
  case ":${CLAUDE_CODE_PLUGIN_DIRS-}:" in
  *":$HOME/.config/pendingyou/mod:"*) ;;
  *) CLAUDE_CODE_PLUGIN_DIRS="$HOME/.config/pendingyou/mod${CLAUDE_CODE_PLUGIN_DIRS:+:$CLAUDE_CODE_PLUGIN_DIRS}"
     export CLAUDE_CODE_PLUGIN_DIRS ;;
  esac
fi
```

- **Where they run.** Each platform's set-environment script runs them, which `/etc/zshenv` and the shells' profiles read. So they run in every login shell, the one Flume reads each agent's environment from among them, and in a person's own shell.
- **What they keep.** A value already there keeps its paths, after the mod's. A value that names the mod already is left as it is.
- **A user with no folder.** Their environment is left as it is.

They are a file of their own, `nix/modules/plugin-dirs.sh`, which `nix/modules/sessions.nix` reads into `extraInit` and `nix/tests/activate.sh` runs by `sh`, so the module holds one copy (Task notes 4.1).

Alternatives:

- **The variable in each Flume account's `env`.** Not taken: it misses a person's own session, and the set does not write a host's accounts.
- **`environment.variables`.** Not taken: it would replace a person's own value, and name a folder a user without one lacks.

### 9. The words

- **The README's install with nix.** It says that each switch installs the set's marketplace and the plugins of the tools it installs into every Claude Code home Flume's accounts use, and the default home. It names the commands and says what each leaves. It says Pending You's server is added where the home names none. It names `CLAUDE_CODE_PLUGIN_DIRS` and the wake mod. It says that a home's own install of the same plugin from another marketplace stays until the person removes it, and that a running session loads the plugins at its next start or after `/reload-plugins`.
- **CLAUDE.md and `openspec/config.yaml`.** "None runs `claude` or writes into a Claude Code home" and "touching no Claude Code home" give way to the install by Claude Code's own commands.

### 10. The tests

- **The step's cases.** They are in `nix/tests/activate.sh`, with stubs of `flume` and `claude` (Task notes 3.1).
- **The modules' evaluated cases.** They are in `nix/tests/sessions.nix`, new, read by `nix/tests/default.nix`.
- **The marketplace's cases.** They are the set's, in `nix/tests/companions.nix` and `nix/tests/plugins.nix`, and hold with Flume v0.31.0.
- **The cases this change modifies.** *No Claude Code of the modules'* allows the one step. *The tools' activation is their own* takes it out with the runner's other steps.

## Rules relied on

- **core.1, core.13.** The runner runs no `claude` and keeps no process. The step is the activation's, at a switch.
- **core.18, core.46.** No module writes a file of Claude Code's. Claude Code's own commands write in each home, as herdr's own `plugin link` records the set's herdr plugins. core.18's and core.46's details name the step and the login environment.
- **core.28.** The marketplace is a folder on the host, and no command of the step names a network address. Pending You's server's address is Claude Code's to reach in a session, as before.
- **core.41.** The tests name `alice` and scratch homes.
- **core.44, core.45.** Flume's tag moves with the lock. Each plugin is at its tool's version, which the build holds.
- **core.47.** The marketplace is the flake's own package, which the cache holds.

## Risks / Trade-offs

- **A switch's time.** Each home costs two `claude` starts at a switch that changes nothing, and one more for each plugin that changes. Flume's status reads every place once.
- **A session running during the switch.** Claude Code's commands write the home's files while its sessions read them, as when a person runs them by hand.
- **Two copies of one plugin.** A home that installed `plannotator@plannotator`, or `flume` from Flume's own marketplace, loads both copies' hooks until the person uninstalls theirs. The step changes nothing else in a home (record 0044, point 3), so the README tells the person.
- **An older Claude Code.** A `claude` without `--json` on these commands fails each, said in one line. The plugins wait for a newer Claude Code.
- **Another home.** A session under a `CLAUDE_CONFIG_DIR` that no Flume account names gets none of it. The default home and Flume's accounts' homes are the ones the user named.
- **The login environment.** A shell that reads no set-environment script, or a person's own rc that sets `CLAUDE_CODE_PLUGIN_DIRS` after it, loads no wake mod.

## Sequencing

- **The Flume unit first.** `flumes-status-names-each-accounts-claude-code-folder` is released in Flume v0.31.0, before this change's code (boomdrive-design, 2026-11-18).
- **sluice before Flume.** No Flume past v0.29.0 goes into the set ahead of sluice 0.97 or later. This branch takes `main`, holding the fix `the-set-takes-each-tools-latest-release`, sluice 0.97.2, and the fix `the-sets-marketplace-holds-every-tools-claude-code-plugins`, before task 1.1 moves Flume.
- **The other unit of the bolt.** `boomdrives-tools-install-on-a-machine-without-the-runner` landed on the bolt first, changing the same modules and the README's install. This change holds its words and cases: the set's user, each companion's switch, and the steps without the runner.

## Task notes

- **0.** Every evaluated case runs against no store, as `nix/check.sh` does, with core.41's neutral names. No check builds a package or reaches the network. Two steps reach it, each in the devenv shell outside the sandbox: locking Flume's new tag, and building `packages.aarch64-darwin.boomdrive-claude-plugins` once, before task 2.1 is ticked.
- **1.1.** The field is `home`, as v0.31.0's `AccountStatus` names it, with `home_unread` beside it where it cannot be read.
- **3.1.**
  - The `flume` stub prints the case's `status.json` for `--json status`, or exits 1 where the case names no status.
  - The `claude` stub records each call with the `CLAUDE_CONFIG_DIR` it sees, `unset` where none.
  - For `plugin list --json`, it prints the case's `plugins.json` for that home.
  - It refuses the command the case's `refuse` names, printing `{"outcome":"failed","message":"<words>"}` and exiting 1.
  - The homes' `.claude.json` files are written by the case.
  - `jq` is the shell's own, found by its path.
- **4.1.** `nix/tests/activate.sh` sources `nix/modules/plugin-dirs.sh` by `sh` in a scratch `HOME`, with and without the folder, and with `CLAUDE_CODE_PLUGIN_DIRS` unset, set to another path and naming the mod, then prints the variable.
