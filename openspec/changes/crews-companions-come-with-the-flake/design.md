# Design

## Context

Record 0044 holds agentplot-design's rulings of 2026-11-11 and this stage's choices:

- **Claude Code loads the companions itself.** The set installs them as plugins of a marketplace of its own (points 1 and 2).
- **The install into Claude Code's homes waits** on the user's card `req_020ff29dc00c49ec98e8` (point 3).
- **Pending You's MCP server is not in its plugin** (point 4).
- **The wake mod rides in Pending You's plugin**, and presence and herdr's pane marks are lost (point 5).
- **Where each companion comes from** (point 6).
- **The Boomdrive module stands as Pending You's module for its folder** (point 7).
- **The companions come with the runner and crew** (point 8).

The flake today, after `portless-comes-with-the-flake`:

- **The set** is `boomdrive`, `boomdrive-launcher`, `crew`, `flume`, `flume-herdr-plugin`, `ghostty`, `herdr`, `portless`, `sluice` and `sluice-herdr-plugin`.
- **`nix/modules/apps.nix`** holds the Boomdrive module's own options for what it installs with the runner: `herdr.package`, `herdr.plugins`, `ghostty.package`, `launcher.package` and `portless.package`.
- **Activation.** `nix/modules/activate.sh` is the modules' activation, run as the runner's user. Its `link` links the runner's file, and its `plugins` runs herdr's own `plugin link` for each plugin `herdr.plugins` lists.
- **The tests.** `nix/check.sh test` evaluates `nix/tests` against no store, and runs `nix/tests/activate.sh`'s shell cases.

What the companions are, as read on 2026-11-11:

- **Pending You 0.41.0** is npm's `pendingyou`, from recordplane/pendingyou-cli, under Apache-2.0.
  - **Its package.** It has no runtime dependency, and runs on Node.js 20 or later. It holds `dist/`, the command line, and `mod/`, the wake mod: a Claude Code plugin named `pendingyou-wake` whose `hooks/hooks.json` names one module, `./register.js`.
  - **Its source.** pendingyou-cli tags no release. Its commit `93a302e8` says 0.41.0, and it locks no dependency: its build is `tsc` from an unlocked TypeScript.
  - **Its folder.** `init` writes Pending You's folder, `~/.config/pendingyou`: `bin/pendingyou-hook`, the launcher its Claude Code hooks run (`dist/shim.js`); `bin/pendingyou-mcp-headers`, its MCP server's headers helper; and `claude-code.json`, the manifest (`installClaude` in `dist/claude.js`). The folder also holds the command line's own sign-ins, the computer's key and its state.
  - **Its hooks.** `init` gives Claude Code its `CLAUDE_HOOKS` and `PERMISSION_HOOKS` (`dist/claude.js`), each line as `hookLine` writes it (`dist/hooks.js`): `<launcher> <sub> --app claude-code || true`.
  - **Its skill.** `init` saves the text `https://www.pendingyou.com/skill-stub.md` serves, which is not versioned.
  - **Its server.** The MCP server is `pendingyou`, `https://www.pendingyou.com/mcp`, signed in through the folder's helper. Its hooks, its permissions and its words call it as `mcp__pendingyou__*`.
  - **Its sign-in.** `pendingyou login --app claude-code` makes a connection that can post cards only where the Claude folder it reads names that server with the helper. `pendingyou app login herdr` signs herdr's popup in.
- **The wake mod** reads its root and name from Claude Code's mod API, `$.plugin.root` and `$.plugin.name`. Only a copy named `pendingyou-wake` whose root ends in `/mod` finds Pending You's folder (`cliConfigOf` in `mod/hooks/wake.js`). Such a copy says presence while its session is open, writes a loaded note under the folder, and marks herdr's pane. Any other copy wakes its session on an answer and does none of those.
- **Pending You for Herdr** is recordplane/herdr-pendingyou, under Apache-2.0, with no tag.
  - **Its commit.** Commit `d930a00715c2284254f8cb938fe683dca17163ac` is its 0.2.0 that names Pending You 0.41.0.
  - **What it runs.** Every command runs `bin/run.sh`. That runs the folder's launcher once the launcher answers, else `npx -y pendingyou@0.41.0`. Its start hook runs `pendingyou herdr watch --ensure`.
  - **No build steps.** It has none, so herdr's link takes it as it is.
- **Plannotator 0.28.61** is backnotprop/plannotator, under MIT and Apache-2.0, tagged `v0.28.61` on 2026-11-10.
  - **No flake.** It publishes none, and the flake's nixpkgs carries none.
  - **Its Claude Code plugin** is its source's `apps/hook`, as its own marketplace, `.claude-plugin/marketplace.json`, names it. That is `.claude-plugin/plugin.json`, and `hooks/hooks.json`, whose hooks run `plannotator` from `PATH`, with its mod, `hooks/mod/register.ts`, which imports only its own folder.
- **plannotator-tui 0.9.9** is plannotator/plannotator-tui, under MIT, tagged `v0.9.9`. It publishes no flake.
- **numtide/llm-agents.nix** builds both from source at their tags, at its commit `3c4f3e7ccf54c7279659536160f9c58de38073c1` of 2026-11-11:
  - `plannotator` with bun2nix, for `x86_64-linux`, `aarch64-linux` and `aarch64-darwin`;
  - `plannotator-tui` with `rustPlatform`.
- **Annotate** is plannotator/herdr-annotate, under MIT, the herdr plugin `annotate`. `plannotator-tui herdr open` opens Annotate's `doc` pane by default (`crates/plannotator-tui/src/herdr/launch.rs`).
  - **Its commit.** Commit `a9973b43c1fc90044a9f15755e310e12b8465abb`, its `main`, says plugin 0.9.9, `plannotator-tui.version` 0.9.9 and `herdr-annotate.version` 0.2.3. Its tags name its runtime alone, and `rust-lite-v0.2.3`'s tree pins plannotator-tui 0.9.4.
  - **Its build steps** fetch two prebuilt binaries into `bin/`. Each step exits 0 where `bin/<name>.exe` runs and `bin/<name>.version` holds the pinned version (`scripts/fetch-herdr-annotate.sh`, `scripts/fetch-plannotator-tui.sh`).
  - **Its runtime's source** is its `rust` folder, with its `Cargo.lock`.
- **Claude Code 2.1.319** installs a plugin by `claude plugin marketplace add` and `claude plugin install`. It runs its own copy of each plugin, from `<home>/plugins/cache/<marketplace>/<plugin>/<version>`.

This change is the unit `crews-companions-come-with-the-flake` of the bolt `the-flake-brings-its-companion-tools`.

## Goals / Non-Goals

**Goals:**

- The set gains Pending You, Plannotator and plannotator-tui, their herdr plugins, and its marketplace of their Claude Code plugins. Each is held to its release, and the cache builds and pushes it.
- Each piece comes with the runner where crew's module is on:
  - the three commands and the sign-in command are installed;
  - the two herdr plugins are linked;
  - Pending You's folder is written.

  Each package can be replaced or dropped.
- The flake's shell holds the three commands.
- Nothing runs `claude` or writes into a Claude Code home, and the runner's code does not change.

**Non-Goals:**

- **The install into Claude Code's homes,** with Pending You's MCP server. It waits (Decision 12; task 5.1).
- **Presence, the loaded note and herdr's pane marks** from the wake mod (record 0044, point 5).
- **plannotator-tui's own settings.** Its `~/.config/plannotator-tui/config.toml`, where a review pane opens, is the person's, as herdr's `config.toml` is (record 0034, point 1).
- **Plannotator's skills.** Plannotator installs them apart from its plugin.
- **plannotator-tui's own herdr plugin, `plannotator-tui`.** `plannotator-tui herdr open` opens Annotate's pane, not this one.
- **An `ensure` action** that restarts a running herdr's Pending You watcher after a switch. Bringing a running herdr's plugins up to date stays the host's (record 0034, point 1).
- **Any change of crew's or Flume's.**

## Decisions

### 1. Four inputs, each locked by its hash

`flake.nix` gains:

```nix
pendingyou = {
  url = "https://registry.npmjs.org/pendingyou/-/pendingyou-0.41.0.tgz";
  flake = false;
};
llm-agents = {
  url = "github:numtide/llm-agents.nix/3c4f3e7ccf54c7279659536160f9c58de38073c1";
  inputs.nixpkgs.follows = "nixpkgs";
};
herdr-pendingyou = {
  url = "github:recordplane/herdr-pendingyou/d930a00715c2284254f8cb938fe683dca17163ac";
  flake = false;
};
herdr-annotate = {
  url = "github:plannotator/herdr-annotate/a9973b43c1fc90044a9f15755e310e12b8465abb";
  flake = false;
};
```

- **Locked by hash.** The lock names each by its hash. A tarball's unpacked tree is Pending You's `package/`.
- **One nixpkgs.** llm-agents.nix follows the flake's nixpkgs, as every input of the set does, so the set has one nixpkgs and the cache holds what the flake builds (core.47). Its build of Plannotator under the flake's nixpkgs is tried once before task 1.2's check (Task notes 1.2). Where it fails, the input keeps its own nixpkgs, and the task's summary says so.
- **No tag for `tags.nix`.** None of the four is in `tags.nix`'s list of tools named by a release tag. Decisions 3 and 4 hold each to its release instead (core.45's details as amended).

Alternatives:

- **Pending You from its source at commit `93a302e8`.** Not taken: the source locks no dependency, so its `tsc` is whichever TypeScript a build finds.
- **Plannotator's command from its release binaries.** Not taken: core.44 rules out a prebuilt binary where a source build exists, and llm-agents.nix builds it from source.
- **A recipe of this flake's for Plannotator.** Not taken: it would copy llm-agents.nix's bun2nix lock of the workspace, thousands of lines kept here.
- **Annotate at `rust-lite-v0.2.3`.** Not taken: that tree pins plannotator-tui 0.9.4, and its manifest is plugin 0.9.4.

### 2. Pending You's package

`packages.<system>.pendingyou` is `nix/packages/pendingyou.nix`, called with the set's `pkgs` and the input as `src`.

- **It copies.** It copies `dist`, `mod` and `package.json` to `$out/lib/node_modules/pendingyou`. Nothing is built.
- **It runs on the flake's Node.js 24.** `$out/bin/pendingyou` is a `makeWrapper` of the flake's `nodejs_24` running `dist/cli.js`, the one Node.js portless runs on.
- **Its version** is its `package.json`'s.
- **What it gives.** `passthru.mod` is its `mod` folder and `passthru.node` its Node.js. `meta` gives `mainProgram` `pendingyou`, license `asl20` and home page `https://www.pendingyou.com`.

### 3. Plannotator and plannotator-tui through llm-agents.nix

`packages.<system>.plannotator` and `packages.<system>.plannotator-tui` are llm-agents.nix's packages of those names for that system, unchanged.

- **The source's tag.** The test holds each one's `src` to its release tag, `v0.28.61` and `v0.9.9`, and its version to the tag without the `v`.
- **The same source.** Plannotator's Claude Code plugin is taken from that same `src` (Decision 5).
- **A moved lock.** A lock that moves llm-agents.nix to a commit carrying another version moves both packages with it. The test names the version that moved.

### 4. The herdr plugins

**`pendingyou-herdr-plugin`** is `nix/packages/pendingyou-herdr-plugin.nix`.

- **Copied whole.** It copies the input whole, its version its `herdr-plugin.toml`'s. Nothing in it is rewritten, as nothing in the set's other plugins is.
- **The launcher first.** Its `bin/run.sh` runs the folder's launcher, which each activation writes before it links the plugins (Decision 8). Its `npx` fallback runs only where no folder was written.
- **Its version.** The test holds the version `run.sh` names, `pendingyou@<version>`, to the set's `pendingyou`.

**`plannotator-herdr-plugin`** is `nix/packages/plannotator-herdr-plugin.nix`, Annotate.

- **Copied, `bin` filled.** It copies the input and fills `bin`, so herdr's link runs its build steps and each exits 0 having fetched nothing:
  - `herdr-annotate.exe` is Annotate's runtime, a `rustPlatform.buildRustPackage` of the input's `rust` folder, with `cargoLock.lockFile` its own `Cargo.lock`;
  - `herdr-annotate.version` holds the input's `herdr-annotate.version`;
  - `plannotator-tui.exe` is the set's plannotator-tui's program, copied;
  - `plannotator-tui.version` holds its version.
- **Its version.** The test holds the input's `plannotator-tui.version` to the set's `plannotator-tui`.
- **What it gives.** `passthru.runtime` is the runtime and `passthru.tui` the plannotator-tui it was given.

### 5. The set's marketplace, `boomdrive-claude-plugins`

`packages.<system>.boomdrive-claude-plugins` is `nix/packages/claude-plugins.nix`, given the set's `pendingyou`, Plannotator's `src` and Pending You's skill. Its output is one folder:

```
.claude-plugin/marketplace.json     name "boomdrive", owner "agentplot", plugins pendingyou and plannotator, each source "./plugins/<name>"
plugins/pendingyou/.claude-plugin/plugin.json   name "pendingyou", version the set's Pending You's
plugins/pendingyou/hooks/hooks.json             Pending You's hooks and "modules": ["./register.js"]
plugins/pendingyou/hooks/register.js            the wake mod's, from the set's Pending You's mod/hooks
plugins/pendingyou/hooks/wake.js                the same
plugins/pendingyou/skills/pendingyou/SKILL.md   Pending You's skill
plugins/plannotator/…                           apps/hook of Plannotator's source, whole
```

- **Pending You's hooks.**
  - **The lines.** They are `init`'s `CLAUDE_HOOKS` then `PERMISSION_HOOKS`, in their order:
    - SessionStart `pickup`;
    - UserPromptSubmit `handoff`;
    - Stop `stopcheck`;
    - SessionEnd `presence`;
    - PermissionRequest `permission`;
    - PostToolUse and PostToolUseFailure `permission-done`;
    - SessionEnd `permission-done`;
    - Notification `notify`, matching `permission_prompt`.
  - **The command.** Each is `"$HOME/.config/pendingyou/bin/pendingyou-hook" <sub> --app claude-code || true`. The command names the folder's launcher by the session's `HOME` rather than a host's path, so one marketplace serves every host and the cache holds it.
  - **Timeouts.** Each has `init`'s 10-second timeout, and the two SessionEnd hooks have none.
  - **Where they stand.** They are kept in the recipe beside a comment naming `dist/claude.js`, and the test holds them to it (Task notes 2.1).
- **The wake mod.** The plugin carries the mod's module, so Claude Code loads it with the plugin. Named `pendingyou`, its copy wakes its session on an answer and finds no folder (record 0044, point 5).
- **The skill** is `nix/packages/pendingyou-skill.md`, the text `https://www.pendingyou.com/skill-stub.md` served on the day task 2.1 writes it, unchanged. Its first line is Pending You's own. The recipe's comment names its address and day. treefmt excludes `*.md` already, so no formatter rewrites it.
- **No MCP server.** The plugin holds no `.mcp.json` and names no server (record 0044, point 4).
- **Plannotator's plugin** is its source's `apps/hook`, copied whole, as Plannotator's own marketplace gives it. Its version is its `plugin.json`'s, which the test holds to the set's `plannotator`.
- **Regular files only.** Every file is copied with links followed (`cp -RL`), so the folder holds no link into the store (record 0022 as amended on 2026-11-11). The build fails where `find "$out" -type l` finds one.
- **The derivation** is `runCommand`: nothing is compiled. `passthru.plugins` gives each plugin's name and version, and `passthru.hooks` Pending You's hooks as an attribute set, for the test.

Alternatives:

- **The marketplace made by the module with the host's home in each hook line.** Not taken: it would be no package of the set, so the cache would hold none, and each host would build its own.
- **The wake mod as a plugin of its own, `pendingyou-wake`.** Not taken: by that name its copy still finds no folder, its root being Claude Code's cache, and one plugin is one install.

### 6. The modules' options and installs, with the runner and crew

`nix/modules/companions.nix`, imported by `darwin.nix` and `nixos.nix` beside `apps.nix`, declares three options, each made by `installed` as `apps.nix` makes its own:

- `services.boomdrive.pendingyou.package`;
- `services.boomdrive.plannotator.package`;
- `services.boomdrive.plannotator-tui.package`.

Each is the set's for the host's system by default, with its `defaultText`.

- **Where they go.** With `services.boomdrive.enable` and `services.crew.enable` both true, each that is not `null` goes in `environment.systemPackages`. So does the sign-in command, where Pending You's package is not `null` (Decision 9).
- **Without either.** Without the runner or without crew, none does.
- **The herdr plugins.** `apps.nix`'s `herdr.plugins` default gains `set.pendingyou-herdr-plugin` and `set.plannotator-herdr-plugin`, after Flume's and sluice's, where crew's module is on. Its `defaultText` and description say so.
- **Why crew.** The companions are crew's (record 0044, point 8). A host that runs Flume without crew gets none of them.

### 7. Pending You's folder

`nix/modules/pendingyou-folder.nix` is a function, not a module, which `darwin.nix` and `nixos.nix` call in their `let` with the runner's user's `home` and `services.boomdrive.pendingyou.package`. It gives one store folder of four files:

- `bin/pendingyou-hook`, mode 0755. It runs as `init`'s launcher does (`dist/shim.js`), with what deneb's adds for crew:
  - it exports `PENDINGYOU_CONFIG_DIR` as `<home>/.config/pendingyou`;
  - for `permission-done`, it exits at once where `permission-cards/` holds no card;
  - for `stopcheck` in a session crew started (`CREW_AGENT` set), it runs the check and throws its verdict away, since an agent's last words are for its conductor and not a card;
  - otherwise it `exec`s the given package's `pendingyou`.
- `bin/pendingyou-mcp-headers`, mode 0755. It exports `PENDINGYOU_CONFIG_DIR` and `exec`s `pendingyou mcp-headers "$@"`.
- `claude-code.json`, mode 0644. It is the manifest in `installClaude`'s shape. It names the origin `https://www.pendingyou.com`, the helper at the folder's path, the hook lines of Decision 5 by the folder's path, permission cards on, `mcpAdded` false, and no skill and no command-line copy.
- `signin/.claude.json`, mode 0644. It names the server `pendingyou`, of type `http`, at `https://www.pendingyou.com/mcp`, with the folder's helper as its `headersHelper`. It is the Claude folder the sign-in command signs in with.

The folder holds no copy of the wake mod (record 0044, point 5).

Alternatives:

- **The four files computed by `activate.sh`.** Not taken: JSON written by `sh` with no `jq`, and the launchers' package would still be the module's.
- **`init` run at activation.** Not taken: it writes Claude Code's settings and sets up one Claude folder of the several (core.46's details as amended).

### 8. The activation writes the folder, before it links the plugins

`activate.sh` gains `pendingyou <source> <folder>`.

- **Each file whole.** For each file of `<source>`, by its path under it, it writes `<folder>/<path>` with the source's mode, by a copy into a temporary file beside it renamed over the path. It writes only where no regular file there has the same bytes and mode, and replaces a link standing there.
- **New folders.** It makes the folder with mode 0700 where it is missing, and each folder under it with 0755.
- **Nothing else.** It reads, writes and removes nothing else under `<folder>`: the sign-ins, the key, `permission-cards/` and the state are the command line's.
- **What it says.** It says one line where it wrote a file, naming it, and nothing where none differed.
- **A failure.** A file it cannot write is said in one line naming it, stops the files after it, and stops nothing else of the activation. The step exits 0, as `plugins` does.

`darwin.nix` and `nixos.nix` run it as the runner's user, as they run `plugins`, where the companions are installed and Pending You's package is not `null`. Their folder is `<home>/.config/pendingyou`, and they run it before the plugins' step, so Pending You for Herdr finds the launcher at its first link.

### 9. One sign-in command

`pendingyou-signin`, made in `companions.nix` by `writeShellApplication` from `services.boomdrive.pendingyou.package`, does three things:

- it exports `PENDINGYOU_CONFIG_DIR` as `$HOME/.config/pendingyou`;
- it runs `pendingyou login --app claude-code "$@"` with `CLAUDE_CONFIG_DIR` the folder's `signin`;
- it runs `pendingyou app login herdr` with the same arguments but `--force`, which that command does not take.

The person runs it once for the computer. Each approval is theirs, in the browser or by a code on the phone. The modules run it never.

### 10. The shell

`nix/shell.nix`'s packages gain the set's `pendingyou`, `plannotator` and `plannotator-tui`, after portless.

### 11. The words

- **The README's Installing with nix:**
  - names the companions in the set;
  - names the three options, their `null`, and that they come with the runner where crew's module is on;
  - names Pending You's folder and what stays the command line's;
  - names `pendingyou-signin` as the person's one sign-in;
  - says that the companions' Claude Code plugins are not yet installed into any Claude Code home, which waits on the user's answer (record 0044, point 3).
- **CLAUDE.md and `openspec/config.yaml`** name the companions in the flake's set and among what the modules install with the runner.

### 12. The install into Claude Code's homes waits

Task 5.1 is the install. It waits on the user's answer to `req_020ff29dc00c49ec98e8` and on the unit that answer puts first:

- **Every session on the accounts.** Flume names each account's configuration home in its status. Then activation installs into each home, for each `home` Flume's status gives:
  - `claude plugin marketplace add` of `boomdrive-claude-plugins`, or `update` where it is added;
  - `claude plugin install` or `update` of `pendingyou@boomdrive` and `plannotator@boomdrive`, at user scope;
  - `claude mcp add-json --scope user pendingyou` with the folder's helper, where the home names none.

  Each runs with `CLAUDE_CONFIG_DIR` that home, idempotently, and changes nothing else. The person's default home is installed into only behind an option that is off.
- **crew's and Flume's sessions alone.** crew and Flume pass the plugins at launch. This change's task then gives crew the marketplace's folders.

The code stage leaves task 5.1 unticked and stops short at it, naming it, until the answer and that unit's tag are in. The task writes its own spec delta then, from the answer.

## Rules relied on

- **core.1, core.13.** The runner runs no companion. Pending You's watcher is started by herdr's start of its plugin (core.13's details as amended).
- **core.4, core.31, core.32.** No sign-in, key or token is in the store. Pending You's sign-ins are the command line's, made by the person's `pendingyou-signin`.
- **core.7, core.8.** No crate, gate tool or file type is added. `*.md` is excluded by treefmt already.
- **core.18, core.46.** No module writes Claude Code's files. Pending You's folder is written by the Boomdrive module standing as Pending You's (record 0044, point 7; core.18's and core.46's details as amended).
- **core.44, core.45.** Each companion is held to its release by the test. Points 6 of record 0044 and the amended details name where each departs.
- **core.47.** The modules install the flake's own packages, which the cache holds.

## Risks / Trade-offs

- **The wake mod's loss.** A session waits on its card without presence or a marked pane (record 0044, point 5). agentplot-design raises it with Pending You.
- **llm-agents.nix's pace.** A Plannotator release reaches the set only once llm-agents.nix carries it, or the input moves to a commit that does.
- **The marketplace's `$HOME`.** A session whose `HOME` is not the runner's user's home finds no launcher, and its hooks do nothing, each line ending `|| true`.
- **The `npx` fallback.** Pending You for Herdr's `npx` fallback fetches from npm only where the folder was never written: a host whose Pending You is `null` but whose plugin list keeps the plugin.

## Sequencing

`worktrunk-comes-with-the-flake` modifies the same requirements: the package lists, the README's words and the shell's tools. Whichever of the two lands second writes its deltas over the other's landed words, holding them (Task notes 4.1).

## Task notes

- **0.** Every evaluated case runs against no store, as `nix/check.sh` does, with core.41's neutral names. No check builds a package or reaches the network. Three steps reach it, each in the devenv shell outside the sandbox:
  - locking the four inputs;
  - fetching Pending You's skill;
  - building each new package once on this host, before the task that adds it is ticked.
- **1.2.** Build `packages.aarch64-darwin.plannotator` once with llm-agents.nix following the flake's nixpkgs. Where it fails, drop the `follows`, say so in the summary, and build again.
- **1.3.** Annotate's runtime: build once, and take the version its `--version` prints into the summary.
- **2.1.** Fetch the skill once with `curl -fsS https://www.pendingyou.com/skill-stub.md`. The test reads `dist/claude.js` of the set's Pending You's source as text, and holds each `{ event, sub }` of `PERMISSION_HOOKS`, and each event of `HOOKS`, to `passthru.hooks`.
- **4.1.** Each delta holds the base's words where `worktrunk-comes-with-the-flake` landed first.
