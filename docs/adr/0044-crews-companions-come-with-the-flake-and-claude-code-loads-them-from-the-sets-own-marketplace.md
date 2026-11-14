---
number: 44
title: Crew's companions come with the flake, and Claude Code loads them from the set's own marketplace
status: accepted
date: 2026-11-11
decision-makers:
- the user
- agentplot-design
- the construct stage of change/crews-companions-come-with-the-flake
---

# Crew's companions come with the flake, and Claude Code loads them from the set's own marketplace

Status: accepted. Points 1 to 5 are agentplot-design's rulings of 2026-11-11, given to the construct stage of `change/crews-companions-come-with-the-flake` and accepted by agentplot-design as its own; point 3 carries the user's answer of that day to their card, and point 5 agentplot-design's ruling after the user's question. Points 6 to 8 are the construct stage's own choices. Each is cheap to change.

Deciders: agentplot-design, answering the construct stage's two stop-shorts of 2026-11-11 ("Neither crew nor Flume: Claude Code loads them itself"; "build everything except the step that installs into Claude Code homes"), and ruling on the wake mod after the user's question that day (point 5); the user, answering their card `req_020ff29dc00c49ec98e8` on 2026-11-11 ("Every session on those accounts, mine included"), and approving proposal 144, which moves the install into Claude Code's homes into the unit `the-companions-load-in-claude-codes-sessions` (point 3); the construct stage of `change/crews-companions-come-with-the-flake`, for points 6 to 8.

Sources: record 0023, whose amendment of 2026-11-08 names Pending You and Plannotator among the companions and keeps Claude Code the person's own; records 0009 and 0034 (point 1, the set links herdr's plugins with herdr's own command); record 0022 as amended on 2026-11-11 (Claude Code's files are copies, never links into the store); rules core.13, core.18, core.44, core.45 and core.46; agentplot/sluice core.51 (no module writes Claude Code's folder); agentplot/blueprints `design/principles.md` 10 and 14; Flume v0.21.2's `crates/flume/src/host/config.rs`, which gives every account's `env` value as `<set>` (Flume core.30 and core.32); Pending You 0.41.0's `mod/hooks/register.js` and `mod/hooks/wake.js`; afterthought/swancloud's `modules/home-pendingyou.nix` and `modules/home-plannotator.nix` as deneb runs them; the OpenSpec change `crews-companions-come-with-the-flake`, whose design gives each point below.

## Context and problem statement

crew's sessions use two companions. Pending You posts the cards a session waits on a person for, and wakes the session when the card is answered. Plannotator reviews a plan or a document, in the browser or in a herdr pane beside the agent's. Today no host gets either from the flake. afterthought/swancloud installs them on deneb itself, and its `claude` shim gives each Claude Code session of crew's partitions Pending You's hooks, MCP server, skill and wake mod through `--settings`, `--mcp-config`, `--plugin-dir` and `CLAUDE_CODE_PLUGIN_DIRS`.

The flake writes none of Claude Code's files (record 0023; sluice core.51). Claude Code 2.1.319 loads hooks, MCP servers and plugins beyond its own settings only from those flags and that variable. crew's standing roles run `claude` from `PATH` with crew's own arguments, and Flume starts every place and every role it runs with its own plugin, its own settings and the account's environment, none of which a kind or a host's table can add to.

## Considered options

1. **Claude Code loads them itself** (chosen). The set installs the companions as Claude Code plugins with Claude Code's own commands, at user scope, into each Claude Code home, from a marketplace of the set's own.
2. **crew owns it.** crew's teams file names the plugin folders its sessions load, and a Flume setting carries them to the agents Flume starts. One fact sits in two launchers, and sessions neither starts miss it. A kind carries nothing of one host (Flume 32), so crew's kinds cannot name plugin folders.
3. **`CLAUDE_CODE_PLUGIN_DIRS` in each Flume pool account's environment.** It misses crew's standing roles and a laptop with no pool.
4. **Installing alone**, the wiring a later unit of crew's. It leaves the companions unloaded.

## Decision outcome

Option 1.

1. **Claude Code loads the companions itself.** The set installs them as Claude Code plugins with `claude plugin marketplace add` and `claude plugin install`, at user scope, from a marketplace of the set's own. Every session under a home loads them with no launcher passing a flag. Neither crew nor Flume gains a setting. Claude Code writing its own settings by its own command is no other tool writing its file (principle 10), as herdr's own `plugin link` records the set's herdr plugins (record 0034, point 1).
2. **The set builds the companions.** It builds:
   - the packages: Pending You 0.41.0, Plannotator 0.28.61 and plannotator-tui 0.9.9;
   - their herdr plugins: Pending You for Herdr, and Plannotator's Annotate, whose `doc` pane `plannotator-tui herdr open` opens;
   - Pending You's own folder, `~/.config/pendingyou`: the launchers its hooks and MCP server run it through, the manifest its `init` would write, and its wake mod, `mod`, as its `init` copies it there;
   - one sign-in command, `pendingyou-signin`, which signs Claude Code's connection and herdr's in once for the computer;
   - the set's marketplace, one store folder of regular files and no links into the store (record 0022 as amended on 2026-11-11), holding Pending You's plugin and Plannotator's.
3. **The set installs them into each Claude Code home.** Which sessions get the companions was the user's to say, on their card `req_020ff29dc00c49ec98e8`, and they answered: "Every session on those accounts, mine included". On deneb, the homes Flume's accounts run are also the homes of the user's own sessions, so user scope there reaches every session. Flume names each account's configuration home in its own status, a `home` on each account, the `CLAUDE_CONFIG_DIR` its environment gives, else none. That is a path and no secret, so the environment stays masked (Flume core.30 and core.32) and the home is a field of its own (principle 14). Activation, for each home, adds or updates the set's marketplace, installs or updates both plugins, and adds Pending You's MCP server, all with Claude Code's own commands, idempotently, changing nothing else in the home; and each session's environment puts Pending You's folder's wake mod on `CLAUDE_CODE_PLUGIN_DIRS` (point 5).
4. **Pending You's MCP server is not in the plugin.** Claude Code renames a plugin's server `mcp__plugin_<plugin>_<server>__*`, and Pending You's hooks, its permissions and its words call `mcp__pendingyou__*`. Activation adds it by `claude mcp add-json --scope user pendingyou`, with the folder's headers helper, in each home. Pending You's plugin holds its hooks and its skill.
5. **The wake mod is written into Pending You's folder, and loaded from there.** agentplot-design's ruling, after the user's question: "The set writes the wake mod into Pending You's folder (~/.config/pendingyou/mod), as swancloud does, and loads it from there in every session by putting that folder on CLAUDE_CODE_PLUGIN_DIRS. That is an environment variable, not a Claude Code file, so presence, the loaded note and herdr's pane marks hold." The mod reads its root and its name from Claude Code's mod API, `$.plugin.root` and `$.plugin.name` (`cliConfigOf` in `wake.js`): only a copy named `pendingyou-wake` whose root ends in `/mod` finds Pending You's folder, and the folder's copy is one. So each activation writes the set's Pending You's `mod` into the folder, as its `init` copies it there, and the set's Pending You plugin carries no wake mod. A session that loads the folder's copy is woken when its card is answered, says presence while it is open, writes its loaded note under the folder and marks herdr's pane.
6. **Where each companion comes from.**
   - **Pending You.** pendingyou-cli tags no release and locks no dependency, so the set takes the package Pending You publishes on npm, `pendingyou-0.41.0.tgz`, as an input the lock names by its hash. It runs on the flake's Node.js 24. This departs from core.44's derivation of a tagged source until pendingyou-cli tags its releases.
   - **Plannotator and plannotator-tui.** Neither publishes a flake. numtide/llm-agents.nix builds each from its source at its release tag, `v0.28.61` and `v0.9.9`. So the set takes both from llm-agents.nix's flake at a commit that carries them, which the lock names. Plannotator's Claude Code plugin is that same source's `apps/hook`, as Plannotator's own marketplace names it. This departs from core.44's "the tool's own flake" until each publishes one.
   - **The herdr plugins.** Pending You for Herdr and Annotate tag none of their own releases. Each is named by the commit the lock locks, and the test holds it to the version of the tool it runs: Pending You for Herdr's `bin/run.sh` names the set's Pending You, and Annotate's `plannotator-tui.version` names the set's plannotator-tui. Annotate's `bin` holds its runtime, built from its `rust` source at that commit, and the set's plannotator-tui, so herdr's link runs no build step of its. This departs from core.45's release tag for these two.
7. **The Boomdrive module stands as Pending You's module for its folder.** Pending You gives no module. So the Boomdrive module's own options install Pending You, and its activation, as the runner's user, writes into Pending You's folder only what `pendingyou init` writes there: the two launchers, the manifest, the wake mod and the sign-in's Claude folder. Each is a regular file, written whole and only where it differs. The sign-ins, the computer's key and Pending You's state stay the command line's own. This departs from core.18 until Pending You gives a module.
8. **With the runner and crew.** The modules install the companions, link their herdr plugins and write Pending You's folder only where `services.boomdrive` and `services.crew` are both on. Each package is the set's by default, a host's own replaces it, and `null` drops it.

Not taken: a loader that hands the mod a Claude Code API whose `plugin.root` is the folder's. That fakes Claude Code's mod API and breaks with its next change. The wake mod in the set's Pending You plugin, which runs from that plugin's copy in Claude Code's cache, finds no folder, and says no presence, writes no loaded note and marks no herdr pane. The source of Pending You's command line built by `tsc` with nothing locked. Plannotator's command from its release binaries, which core.44 rules out where a source build exists.

### Consequences

- core.18's, core.44's, core.45's and core.46's details name what this record allows, in the commit that writes the change.
- Pending You's skill is kept in this tree as Pending You serves it, since the address its `init` fetches it from is not versioned.

## Amendment, 2026-11-13: the set's marketplace lists timber's plugin

Status: decided on 2026-11-13 by the construct stage of the change `timber-comes-with-boomdrives-flake`, within record 0023's amendment of that day, which brings timber with its herdr and Claude Code plugins into the set. A writing agent's choice, cheap to change; the change's design gives it as its Decision 4.

- **A third plugin.** `boomdrive-claude-plugins` lists `timber`: timber's own `plugins/claude-code` at the tag the lock names, copied whole, at the set's timber's version. Point 2's marketplace is otherwise as it stands, one store folder of regular files.
- **Its install waits with the companions'.** No Claude Code home gets it until point 3's unit, `the-companions-load-in-claude-codes-sessions`, installs the set's marketplace. That unit finds `timber` among the marketplace's plugins; what it installs of them, and for which sessions, is its own design.
- **Not taken:** timber's own marketplace, `timber`, carried as a second package, which each home would add beside the set's.
