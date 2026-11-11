# Design

## Context

Record 0023's amendment of 2026-11-08 puts portless in the set, on the user's card "Boomdrive flake: beyond Pending You and Plannotator, which of your tools should it also install?" ("worktrunk (wt), portless (the .localhost dev addresses)"):

- **What it is for.** The flake installs portless, which gives each worktree's dev server a `.localhost` address and whose routes crew's board reads.
- **As herdr is.** It is the set's by default and overridable as herdr is: a host replaces it through the modules' options, or drops it for its own.

The flake today, after the change `herdr-and-ghostty-come-with-the-flake-and-an-app-opens-the-agents`:

- **The set** is `boomdrive`, `boomdrive-launcher`, `crew`, `flume`, `flume-herdr-plugin`, `ghostty`, `herdr`, `sluice` and `sluice-herdr-plugin`. Each tool's source is an input the lock names by its hash. Flume, sluice, crew and herdr come through their own flakes at release tags, and `nix/tests/tags.nix` holds each to one.
- **`nix/modules/apps.nix`** holds the Boomdrive module's own options for what it installs with the runner that no tool's own module installs: `herdr.package`, `herdr.plugins`, `ghostty.package` and `launcher.package`. Each is the set's by default, replaced by a host's own, and none where `null`. Without the runner it installs none.
- **The shell**, `nix/shell.nix`, holds the set's runner, Flume, sluice, crew and herdr.
- **The cache's workflow** builds every package `nix eval` lists for each system, so a new package needs no change of the workflow's.

What portless is, as read on 2026-11-11:

- **Its repository.** portless is vercel-labs/portless, under Apache-2.0. Its latest release is `v0.17.0`, tagged on 2026-11-08. It publishes no flake, and the flake's locked nixpkgs carries no portless.
- **Its workspace.** The repository is a pnpm 11 workspace (`packageManager` `pnpm@11.1.3`). It holds the command line, `packages/portless`, beside its docs site, tests and examples, which bring Next.js and their own dependencies.
- **Its package.** `packages/portless/package.json` says version `0.17.0` and needs Node.js 24 or later. It has no runtime dependency: tsup bundles `src/cli.ts` and `src/index.ts` into `dist/`. Its `build` also runs `tsc` for the library's declarations, which the program does not need.
- **What it runs.** It runs `openssl` for its certificates, and `ps` and `which`. It runs git in a worktree to name its servers, `security` on macOS and a distribution's `update-ca-certificates` or `update-ca-trust` to trust its certificate authority, and `sudo` to bind a port below 1024.
- **Its proxy.** portless serves `https://<name>.localhost` through one proxy for the machine, on port 443 by default.
  - **Its first run.** The first `portless run` starts the proxy. It asks for `sudo` where the port needs root, and makes a certificate authority that it adds to the system's trust store.
  - **With no terminal.** Where the port needs root and no terminal is open, `portless run` starts no proxy and exits 1, saying how to start one (its `ensureProxyRunning`).
  - **As a service.** `portless service install` installs the proxy as a root service of launchd or systemd. It records the Node.js and the script it was run with, by their paths.
- **Its worktree names.** In a linked worktree, `portless run` serves the branch's last segment as a subdomain, `fix-ui.myapp.localhost`, beside the main checkout's `myapp.localhost`.
- **crew's read.** crew's board reads the host's running servers from `portless list` (crew's `gather.routes`), and says "portless did not answer" where it cannot run it.

What the flake's locked nixpkgs gives to build it: `pnpm_11` 11.27.0, `fetchPnpmDeps` (fetcher version 5), `pnpmConfigHook` and `nodejs_24` 24.21.0.

- **The fetch.** Under pnpm 11, `fetchPnpmDeps` sets pnpm's `force_ignores_platform`, so the fetch holds every platform's optional packages and is the same on every system.
- **A trial build.** On 2026-11-11 a recipe as Decision 2 has it built `v0.17.0` on `aarch64-darwin`:
  - its fetch's hash was `sha256-e9nKmcWZFM1aFxW03omeEvinfyRc3NHOAGkueEcwrg4=`;
  - its `portless --version` said `0.17.0`;
  - its output was 580 KiB.

This change is the unit `portless-comes-with-the-flake` of the bolt `the-flake-brings-its-companion-tools`.

## Goals / Non-Goals

**Goals:**

- The set gains portless, built from its source at its release tag, which the lock names by its hash. The cache's workflow builds and pushes it.
- A host that turns on `services.boomdrive` gets portless on its system profile's `PATH`, where a dev server's `portless run` and crew's board find it. The host can replace it or drop it.
- The flake's shell holds it.
- The runner's code does not change, and runs no portless.

**Non-Goals:**

- A proxy of portless's run by the modules, as a launchd job or a systemd unit, and portless's certificate authority trusted by them (Decision 6).
- portless's own settings: a `portless.json`, its state directory, its TLDs, LAN mode or a tunnel.
- afterthought/swancloud's `devurl`, which crew's `crew sites` runs to match a worktree to its server.
- portless's agent skill and its MCP server.
- An install of portless for a person's profile beside the modules': `nix profile add …#portless` works as it does for the other tools, and the README names it.

## Decisions

### 1. portless's source comes in at its release tag, as a source

The flake takes `portless = { url = "github:vercel-labs/portless/v0.17.0"; flake = false; };`. The lock locks it by its revision and hash, as every input is locked.

- **Named by a release tag.** `nix/tests/tags.nix` holds portless to a release tag, as it holds Flume, sluice, crew and herdr (core.45). A tag moves only with the lock.
- **Its version.** The package's version is `packages/portless/package.json`'s at that tag. The test holds it to the tag without the `v`, as it holds Flume's.
- **No flake.** `nix/inputs.nix` gives a node whose `flake` is false as its source, so the checks read the tree the devenv shell fetched.

Alternatives:

- **The npm package's archive**, `registry.npmjs.org/portless/-/portless-0.17.0.tgz`, as afterthought/swancloud's own recipe takes it. Not taken:
  - its `dist` is a build someone else made, and core.44 rules out a release archive in place of the flake's own derivation where the flake can make one, as Context's trial build shows it can;
  - it carries no attestation the flake checks (core.37).
- **The workspace's tip, or a revision with no tag.** Not taken: core.45 moves a tool only by its tag with the lock.

### 2. A recipe of this flake's builds the package alone

`packages.<system>.portless` is `nix/packages/portless.nix`, called from `packagesOf` with the set's `pkgs` and the input as `src`.

- **The fetch.** `pnpmDeps` is `fetchPnpmDeps { inherit pname version src; pnpm = pnpm_11; pnpmWorkspaces = [ "portless" ]; fetcherVersion = 5; hash = …; }`. The derivation's own `pnpmWorkspaces` is `[ "portless" ]`, so `pnpmConfigHook` installs the same filter offline.
- **The build** is the bundle alone: `pnpm --filter=portless exec tsup`, which writes `packages/portless/dist/cli.js` and `dist/index.js`. The declarations `tsc` writes are a library's, which no program of the host reads.
- **The install** copies `packages/portless/dist` and `packages/portless/package.json` to `$out/lib/node_modules/portless`. It writes `$out/bin/portless`, a `makeWrapper` of Decision 4's Node.js running `dist/cli.js`.
- **`meta`.** Its `mainProgram` is `portless`, its license `asl20` and its home page `https://portless.sh`.
- **The derivation** is `stdenvNoCC.mkDerivation`: nothing is compiled.

core.44 holds it: a recipe of this flake's for a tool that publishes no flake, until it does, from the source the lock names by its hash, reporting its tag's version. When portless publishes a flake, the set takes its package and the recipe goes.

Alternatives:

- **The workspace's own `pnpm build`.** Not taken: turbo builds the docs site and every package, and its fetch brings the docs site's Next.js and sharp.
- **`buildNpmPackage`, or another node builder.** Not taken: the workspace's lock is pnpm's, which `fetchPnpmDeps` reads as it stands.
- **The package's own `build`, tsup then `tsc`.** Not taken: `tsc` is TypeScript 7's, a native binary of each platform, for declarations no host reads.

### 3. One fetch for every system

The recipe's fetch hash is one constant, `sha256-e9nKmcWZFM1aFxW03omeEvinfyRc3NHOAGkueEcwrg4=`, the trial build's (Context). Under pnpm 11, `fetchPnpmDeps` fetches every platform's optional packages, so the fetch is the same on every system. The test holds each system's `portless.pnpmDeps` to:

- the same `outputHash`;
- `pnpmWorkspaces` of `[ "portless" ]`;
- pnpm 11.

The hash moves with portless's tag, in the change that moves the lock: a tag whose fetch differs fails its build in the cache's workflow until the hash moves with it.

Alternatives:

- **A hash for each system.** Not taken: it is needless where the fetch ignores the platform, and a system no host here builds on could never have its hash taken.

### 4. It runs on the flake's Node.js 24, and finds the host's git

The wrapper runs `${lib.getExe nodejs_24}` with `--add-flags $out/lib/node_modules/portless/dist/cli.js`.

- **Node.js 24.** It is the flake's nixpkgs' `nodejs_24`, named by its store path, so portless runs on the Node.js its package asks for, whatever node the host or a project puts on `PATH`.
- **What it runs.** The wrapper adds `--suffix PATH : ${lib.makeBinPath runtimeInputs}`, where `runtimeInputs` is `[ openssl procps which hostname ]`. A suffix puts them after the `PATH` it is given, so the host's own come first.
- **git** is not among them: it is the host's own (records 0012 and 0023), and portless runs the host's git.
- **`security`, `sudo` and a distribution's trust command** are the host system's own, as portless expects them.
- **For the tests.** `runtimeInputs` and the Node.js are given as `passthru.runtimeInputs` and `passthru.node`, which the evaluated cases read. They are the flake's nixpkgs' own packages, by Decision 2's `pkgs`.

Alternatives:

- **`nodejs-slim_24`.** Not taken: its saving is npm, and the one Node.js named by the set is the one portless's service records (Context). Either is cheap to change.
- **git in `runtimeInputs`.** Not taken: core.44 keeps git out of the set. afterthought/swancloud's recipe gives it, as a fleet's own choice.

### 5. The modules' option, installed with the runner

`nix/modules/apps.nix` gains `services.boomdrive.portless.package`, made by its `installed "portless" "…"`. It is the set's `portless` for the host's system by default, with its `defaultText`. With `services.boomdrive.enable` it goes in `environment.systemPackages` with herdr, Ghostty and the launcher, unless it is `null`. Without the runner it does not.

The option is the Boomdrive module's own, as herdr's is, because portless gives no nix-darwin or NixOS module (the service-modules requirement *The modules hold no option that installs a tool its own module installs*). Its name says what it installs (core.12).

In the system profile, portless is on the `PATH` of the runner's service, of what Flume launches, and of a person's shell. That is where a dev server's `portless run` and crew's `portless list` find it (the service-modules requirement *The runner finds its tools on the profiles' PATH*).

Alternatives:

- **Installed whether or not the runner is enabled.** Not taken: a host that imports Boomdrive's module for one tool alone would get a tool it did not ask for (the herdr change's Decision 5).
- **A module of its own, `nix/modules/portless.nix`.** Not taken: `apps.nix` is where the Boomdrive module's own installs stand, and one more option there is one line beside herdr's.

### 6. The proxy and its trust stay the person's

The modules start no proxy of portless's:

- no launchd daemon or agent and no systemd unit of theirs runs portless;
- no activation step runs portless;
- they write none of portless's files, `~/.portless` and portless's block of `/etc/hosts`;
- they add no certificate authority to a trust store.

portless starts its own proxy:

- **At a terminal.** The first `portless run` at a terminal starts it, asking for `sudo` to bind port 443 and to trust its certificate authority.
- **As a service.** A person may instead run `portless service install`, which makes the proxy a root service of the host's own that starts at boot.
- **With no terminal.** An agent's `portless run` in a place has no terminal to ask for `sudo` in, so it starts no proxy and says how to start one. A person starts the proxy once on a host before its agents' dev servers answer at their names.

`portless trust` is the person's. The README says each.

This keeps record 0023's words, that the flake *installs* portless, and its rule for the set's tools, that each runs at the person's asking.

Alternatives:

- **A launchd daemon and a systemd service of the module's, running the set's proxy**, as afterthought/swancloud declares for its Macs (its record 0001). It would serve every agent's dev server on a fresh host with no step of the person's. Not taken:
  - it puts a root service on port 443 on every host that takes the flake, servers among them, which is a host's own choice and not record 0023's "installs";
  - core.46 keeps the modules from starting a tool's process of their own accord, sluice's, a multiplexer's server or a settle loop, and a proxy is one more such process.

  It is cheap to add later, as an option a host turns on. Record 0023's amendment of 2026-11-11 says so.
- **`portless trust` run at activation.** Not taken: it adds a certificate authority to the system's trust store at every switch, a host's security choice.
- **`PORTLESS_UNPRIVILEGED=1` for the runner's user**, so an agent's first run starts a proxy with no `sudo`. Not taken: it serves plain HTTP on port 80 by default, on Linux it needs a `sysctl` the host sets, and how a host's proxy serves is the person's choice of portless's settings.

### 7. The shell holds portless

`nix/shell.nix`'s `packages` gain `set.portless`, after `set.herdr`. Record 0023 gives the shell "its `PATH` the set's tools". crew, which the shell holds, reads portless's routes. Ghostty and the launcher stay out of it, as the herdr change's Decision 4 has it. The shell's hook does not change and starts no proxy.

Alternatives:

- **The shell as it stands.** Not taken: crew's board, run in the shell, would say "portless did not answer" on a host whose profile has none.

### 8. The tests

Every evaluated case is under `nix/tests`, run by `nix:typecheck` or `nix:test` against no store (record 0020's Decision 14), with core.41's neutral names. No case builds portless, runs Node.js or pnpm, or reaches the network, so no tool is added to the gate (core.8).

- **`typecheck.nix`.** The set gains `portless`.
- **`tags.nix`.** Its tools named by a release tag gain `portless`.
- **`packages.nix`** holds the family-packages delta's cases:
  - "portless from its source", "portless other than its tag" and "portless named by no release tag";
  - "One fetch for every system", "The flake's Node.js" and "portless runs the host's git".
- **`apps.nix`** holds the service-modules delta's cases, each on nix-darwin and on NixOS:
  - "portless with the runner", "A host's own portless", "No portless of the set's" and "No portless without the runner";
  - "No proxy of the modules'".
- **`way-in.nix`.** "Every system's shell" holds portless among the shell's tools.
- **`cache.nix`.**
  - Its host's overlay gains `portless`, so "A host's own nixpkgs" holds the flake's own.
  - "The README's words" names `services.boomdrive.portless.package`.
  - "The README's proxy" is added.
- **`workflow.nix`.** "The runner is pushed with its tools" holds as it stands: the workflow builds every package `nix eval` lists, `portless` among them.

### Rules relied on

- **core.44 and core.45.** portless comes from the source the lock names by its hash, at a release tag that moves only with the lock. It is built by a recipe of this flake's because it publishes no flake, and reports its tag's version (Decisions 1 and 2).
  - **The host's tool.** core.44 rules out "a tool a repository pins". portless is the host's tool here, by record 0023's amendment of 2026-11-08: its one proxy serves every repository's servers on the host. A repository that pins its own portless gets it in its places by sluice's equip (record 0009's amendment of 2026-11-05), and the set's does not replace that pin.
- **core.47.** portless is made with the flake's own nixpkgs, and the workflow builds and pushes it with no change of its own.
- **core.13 and core.46.** The runner runs no portless. The modules install it and start no process of it, and declare no service of it (Decision 6).
- **core.18.** The modules write no file of portless's.
- **core.12.** The option names what it installs, `portless`.
- **core.4.** No option holds a credential.
- **core.7 and core.8.** No crate, tool of the gate or tracked file type is added: `.nix` is formatted already. pnpm and Node.js are the recipe's build inputs from the flake's nixpkgs, which the gate evaluates and never runs.
- **core.41.** Neutral names in every case. afterthought/swancloud is named, with its record's id, and no text of it is copied.

### Rules amended

In the commit that writes this change, with record 0023's amendment of 2026-11-11, which holds Decisions 1 to 7 as this change's choices:

- **core.13's details** do not rule out portless's proxy started by a dev server's first `portless run`, an agent's in a place or a person's, or by a person's `portless service install`. None of these is the runner's.
- **core.44's details** name where portless comes from: by this flake's recipe from its source at its release tag, built with the flake's nixpkgs' pnpm and run on its Node.js, with git left the host's own. They name portless the host's tool, whatever a repository pins of its own.
- **core.46's details:**
  - do not rule out the Boomdrive module's own option for portless, beside herdr's, Ghostty's and the launcher's;
  - rule out a proxy of portless's started, kept or declared as a service by a module, or its certificate authority trusted by one.

### Rules departed from

None.

## Sequencing

The bolt's sibling unit `worktrunk-comes-with-the-flake` brings worktrunk the same way, and touches some of the same files:

- `flake.nix` and `flake.lock`, by an input and a package each;
- `nix/modules/apps.nix`, by an option and a line of `environment.systemPackages`;
- `nix/tests/typecheck.nix`, `tags.nix`, `packages.nix`, `apps.nix`, `cache.nix` and perhaps `way-in.nix`;
- `nix/shell.nix`, should it put `wt` on the shell's `PATH`;
- `README.md`, `CLAUDE.md`, `openspec/config.yaml`, record 0023 and core.44's and core.46's details;
- the family-packages, service-modules, binary-cache and perhaps way-in specs, by the same requirements this change modifies.

Each change adds its own lines beside the other's, and whichever lands second merges them, its `nix:test` holding both. A requirement both changes modify is whole in each delta. So the second to land first rewrites its delta of each shared requirement to hold the first's words beside its own, before its archive (Task notes 4.1).

## Risks / Trade-offs

- **No proxy on a fresh host.** Until a person starts portless's proxy once, an agent's `portless run` in a place exits 1, saying how to start it. The README says to start it once per host, by `portless service install` or a first `portless run` at a terminal.
- **The service records its paths.** `portless service install` records the Node.js and the script it was run with, by their store paths, and launchd and systemd hold no garbage-collector root on them. After the set moves portless, the service still runs the old one, and nix's garbage collector may remove it. The README says to run `portless service install` again after a switch that moves portless.
- **Linux builds unproven here.** The trial build ran on `aarch64-darwin` alone. The Linux builds use the same fetch, by Decision 3, and esbuild's Linux binary, which is static. They are first built by the cache's workflow after the landing, where a failed build pushes nothing for that system and fails its job (the binary-cache spec).
- **NixOS's trust store.** portless knows Debian's, Arch's, Fedora's and SUSE's trust commands, not NixOS's, whose trust store the host's configuration writes. On NixOS `portless trust` fails. The host adds portless's certificate authority to `security.pki` itself, or runs portless with `--no-tls`, which the README says.
- **crew reads portless's words.** crew's board parses `portless list`'s lines. A portless tag that changes them leaves crew's board with no routes until crew follows it. The lock moves portless only in a change of its own, whose author reads crew's `gather.routes` beside portless's notes.
- **Two portlesses on a host.** A repository that pins its own portless runs that one in its places, beside the host's proxy, which may be another version. portless's state directory and proxy are one for the machine, so their versions should agree. That is portless's own concern, which its README names.
- **The build's size.** The fetch holds every platform's optional packages of portless's development dependencies, esbuild's and TypeScript's among them. It is fetched once for each of the cache's jobs, and only the 580 KiB output and its Node.js reach a host.

## Migration Plan

A host takes the change by moving its Boomdrive input. With `services.boomdrive.enable`:

- **portless.** Its first switch installs the set's portless in the system profile.
  - A host with portless of its own, in a home profile, finds the profile that comes first on its `PATH`.
  - A host that keeps its own sets `services.boomdrive.portless.package` to it, or to `null`.
- **Its proxy.** A proxy already running, whoever started it, goes on.
  - A host whose proxy is a service its own configuration declares keeps it.
  - A host with none starts it once, by `portless service install` or a first `portless run` at a terminal.

afterthought/swancloud, once it takes the flake, either drops its own recipe for the set's portless and keeps its own proxy service, or passes its own package through this option (record 0023).

## Open Questions

None.

## Task notes

**1.1** In `flake.nix`:

- Add the input of Decision 1, with a comment: portless publishes no flake, so the set builds it by this flake's recipe (core.44). Lock it with `nix flake lock --update-input portless` in the devenv shell, outside the sandbox.
- Give `packagesOf`'s `tools` `portless = pkgs.callPackage ./nix/packages/portless.nix { src = inputs.portless; };`.
- The header's comment names portless, at its release tag through this flake's recipe.

`nix/packages/portless.nix` is Decisions 2 to 4:

- Its arguments are `{ lib, stdenvNoCC, fetchPnpmDeps, pnpmConfigHook, pnpm_11, nodejs_24, makeWrapper, openssl, procps, which, hostname, src }`.
- `version` is `(lib.importJSON "${src}/packages/portless/package.json").version`, which the test holds to the lock's tag.
- A header comment cites record 0023 and this design. It says why the bundle alone is built, and that the fetch's hash moves with the tag.
- With the deps' hash of Decision 3 it builds as Context's trial did. Before the check, build it once in the devenv shell, outside the sandbox: `nix build .#portless --no-link --print-out-paths`. The output's `bin/portless --version` says `0.17.0`.

`typecheck.nix`'s `set` gains `"portless"`, in name order.

**1.2** `tags.nix`'s list of tools named by a release tag gains `"portless"`. In `packages.nix`, "portless named by no release tag" is built as "herdr named by no release tag" is, with the URL `github:vercel-labs/portless/main` and the lock's node's `original.ref` `main`.

**1.3** In `packages.nix`:

- **`portlessHeld = system: version: locked: [ failures ]`** names both versions where they differ.
- **"portless from its source"**, over each system's `portless`:
  - `toString portless.src` is `inputs.portless.outPath`;
  - `portless.meta.mainProgram` is `portless`;
  - `portlessHeld` holds with `portless.version` and `t.lockedVersion "portless"`.
- **"portless other than its tag"** calls `portlessHeld "x86_64-linux" "0.17.1" "0.17.0"`, and expects one failure naming both.
- **"One fetch for every system"**, over the three systems' `portless.pnpmDeps`:
  - one `outputHash` among them;
  - each one's `pnpmWorkspaces` is `[ "portless" ]`;
  - `lib.versions.major pnpmDeps.pnpm.version` is `"11"`.
- **"The flake's Node.js"**: each system's `portless.node.drvPath` is `inputs.nixpkgs.legacyPackages.<system>.nodejs_24.drvPath`.
- **"portless runs the host's git"**: each system's `portless.runtimeInputs`, by `drvPath`, is `inputs.nixpkgs.legacyPackages.<system>`'s `openssl`, `procps`, `which` and `hostname`, in that order, and no one of them has the `lib.getName` `git`.

**2.1** In `apps.nix`, `services.boomdrive.portless.package = installed "portless" "<words>";`. Its words say: portless, which gives each worktree's dev server a `.localhost` address and whose routes crew's board reads, installed with the runner; a host's own replaces it, and `null` installs none; its proxy and its trust are the person's. `cfg.portless.package` joins the list of `environment.systemPackages`. The header names portless.

The cases, in `nix/tests/apps.nix`'s `onEach`, as its herdr cases are:

- "portless with the runner": `t.darwinSystem { }`'s and `t.nixosSystem { }`'s profiles hold the set's `portless` by `outPath`.
- "A host's own portless": with `services.boomdrive.portless.package = pkgs.runCommand "own-portless" { } "mkdir -p $out/bin";`, the profile holds it and not the set's.
- "No portless of the set's": with `null`, `t.failures` is empty and the profile holds no set's portless.
- "No portless without the runner": the platform's `without` holds no set's portless.

**2.2** "No proxy of the modules'", in `apps.nix`, on each platform, with `t.darwinSystem { }` and `t.nixosSystem { }`:

- **The text read.** It reads, as text, with `builtins.unsafeDiscardStringContext`:
  - on nix-darwin, `config.system.activationScripts.script.text`, and each of `config.launchd.daemons`, `config.launchd.agents` and `config.launchd.user.agents`' `serviceConfig.Program` and `serviceConfig.ProgramArguments`;
  - on NixOS, each `config.system.activationScripts` entry's `text`, each definition of `system.userActivationScripts`' `text`, and each of `config.systemd.services` and `config.systemd.user.services`' `script` and `serviceConfig.ExecStart`, by `toString`.
- **The check.** None holds the set's portless's `outPath`.
- **On a failure** the case names the unit or the step that holds it.

The modules do not change for it.

**2.3** `nix/shell.nix`'s `packages` gain `set.portless`, and its header names it. `nix/tests/way-in.nix`'s `shellTools` gains `"portless"`.

**2.4** `cache.nix`'s `names` gain `"portless"`, in name order.

**3.1** The README's **Installing with nix**:

- **The set's list** gains an item: portless, from its source at its release tag, built by this flake's recipe and run on the flake's Node.js 24. It gives each worktree's dev server a `.localhost` address, and crew's board reads its routes.
- **The profile line.** The sentence after it names `#portless` beside `#herdr` and `#ghostty`.
- **The modules' list** gains an item: it installs portless, `services.boomdrive.portless.package`.
- **The block of `null`s** gains `services.boomdrive.portless.package = null;`, and its sentence says "each of the four".
- **A paragraph on the proxy**, after that block:
  - portless's proxy is portless's own, and the modules start none and trust no certificate authority;
  - the first `portless run` at a terminal starts it, asking for `sudo` to serve port 443 and trust its certificate authority;
  - `portless service install` makes it a service of the host's that starts at boot. It runs the portless it was run with, so run it again after a switch that moves portless;
  - an agent's `portless run` with no terminal starts no proxy, so start it once on a host;
  - `portless trust` trusts the certificate authority. On NixOS, whose trust store its configuration writes, the host adds it to `security.pki` itself, or runs portless with `--no-tls`.
- **The shell's paragraph** names portless among the shell's tools.

In `cache.nix`, "The README's words" expects `services.boomdrive.portless.package`. "The README's proxy" expects `portless service install` and `portless trust` in `installing`.

**3.2** CLAUDE.md's paragraph on the flake says "with herdr, its plugins, Ghostty and portless", and "the modules install herdr, Ghostty, portless and the Boomdrive launcher". `openspec/config.yaml`'s stack lines say the same.

**4.1** Where `worktrunk-comes-with-the-flake` has landed on this change's base first, merge the base. Then rewrite each requirement this change's deltas modify that the base's spec now holds otherwise, so the delta holds the base's words beside portless's (Sequencing). Then archive.
