# Design

## Context

See proposal.md for why. This change adds a flake to the runner's repository: the family's host tools pinned together and distributed as binaries through the family's cache, and the runner's service as nix-darwin and NixOS modules. The runner itself does not change.

What stands:

- **This repository**, at `f6d9092`.
  - No nix but `devenv.nix` and `treefmt.nix`. `treefmt.nix` already formats `.nix` files with deadnix and nixfmt, and YAML with yamlfmt.
  - The runner reads one TOML file: `--config <file>`, else `$XDG_CONFIG_HOME/boomdrive/config.toml`, else `~/.config/boomdrive/config.toml`. It refuses a key it does not read. It reads the file again at every pass, but `[control_plane]` only at start (the configuration spec). `config::read(file, home)` is its reader.
  - The command line is `run [--config] [--once]` and `enroll [--config] [--json] <label>`. `enroll` reads the configuration for the control plane's `url` and the partition, and the token from standard input.
  - The runner runs `git`, `flume` and `sluice` by name from `PATH`, and refuses to start below git 2.48. Its tests reach Flume and sluice through stubs, over fixtures of Flume 0.4.1's and sluice 0.61.1's answers. `flume::Cli` and `sluice::Cli` take the program's path.
  - Its version is `Cargo.toml`'s, `0.0.0`. It has no release, no release workflow and no workflow on GitHub at all.
  - The gate is sluice's sandbox with `network = "localhost"`. sluice's `[tests] toolchain`, `devenv print-dev-env`, runs outside the sandbox, with the network and the nix daemon. sluice lands each change on `main` on GitHub.
- **The family's flakes**, read 2026-11-05.
  - sluice (`v0.68.0`) and timber (`v0.5.3`) publish `packages.default`, built from source with rust-overlay at the Rust `.prototools` names, for `aarch64-darwin`, `x86_64-darwin`, `aarch64-linux` and `x86_64-linux`. Their tagged tree says `0.0.0`. A release build is given its version as `SLUICE_VERSION` or `TIMBER_VERSION` under `--impure`, so a pure build of the tag says `0.0.0`.
  - Flume (`v0.10.0`) publishes no flake. Its tag names a release commit with the version written into `Cargo.toml` (`set-version.sh`), so its tagged tree says `0.10.0`. Its release archives are built by cargo, for three systems: no `x86_64-darwin`.
  - Each tool's herdr plugin is `plugins/herdr` in its tree: Flume's manifest takes Flume's version, and sluice's keeps its own.
  - No release of the family carries an attestation. No tool pushes its binaries to a cache: their workflows only read devenv's.
- **nixpkgs and `x86_64-darwin`.** nixpkgs 26.11, which `nixos-unstable` is, has dropped `x86_64-darwin`: importing it for that system fails, unless its configuration sets `allowDeprecatedx86_64Darwin = "force"`, which nixpkgs says "is not expected to function", with no binaries built for it. Its 26.05 branch keeps the platform until the end of 2026. So sluice's and timber's `x86_64-darwin` packages evaluate only while their own locked nixpkgs predates 26.11.
- **The family's cache.** No file of the family names a cache of its own. Cachix's public API, read 2026-11-05, gives a public cache `boomdrive`, at `https://boomdrive.cachix.org`, signed by `boomdrive.cachix.org-1:vHLgmasPg2jKqkvPXWOOrYzspoGL2xxsW4fAuHBtiag=`, Cachix's own key for it. A token of the cache's own, with write access, is made on the cache's settings page, `https://app.cachix.org/cache/boomdrive/settings?binaryCacheSettingsTab=authtokens`, which has no command line.
- **GitHub's hosted runners**, read 2026-11-05: `ubuntu-24.04`, `ubuntu-24.04-arm` (arm64, for public repositories), and `macos-15` (arm64). `cachix/cachix-action` (v17) takes the cache's `name`, an `authToken`, a `signingKey` for a cache signed by a key of its owner's, and `skipPush`. `1password/load-secrets-action` (v5.0.1) reads each `op://<vault>/<item>/<field>` its step's `env` names with the service account token `OP_SERVICE_ACCOUNT_TOKEN`, and gives it as the step's output, masked in the log.
- **devenv's own Cachix push**, in the devenv `devenv.lock` pins, read 2026-11-05. `cachix.push = "<cache>"` starts a push daemon at every entry to the shell, `devenv print-dev-env` among them, and queues every path the shell realises, on whichever host enters it with a write token in `CACHIX_AUTH_TOKEN`, SecretSpec or `cachix authtoken`'s file. It also adds the cache to `cachix.pull`. devenv's documentation suggests a push be turned on where wanted, in the untracked `devenv.local.nix`, since "you'll likely not want every user to push to the cache".
- **A host's configuration**, read 2026-11-05. Its runner's module, its check and its package are what a working module needs:
  - a launchd agent of the user's login through home-manager, `boomdrive run`, `RunAtLoad`, `KeepAlive`, `ThrottleInterval` 30, `ProcessType` `Interactive` and `AbandonProcessGroup`, so a `flume launch` in flight outlives a restart, with `HOME` and the profiles' `PATH` alone in its environment, and a log file;
  - the file rendered from the host's lists, written at activation as a real file at 0600, and the agent restarted only when the write changed it, since the runner reads its control plane only at start;
  - Flume and sluice installed in the user's profile from their release archives, sluice's relinked and re-signed on each platform, and their herdr plugins from the same tags;
  - the runner built from source at a pinned commit, on each host;
  - a check that the agent's keys, its environment, the file's keys and its writer hold.
- **The platforms.** nix-darwin's `launchd.user.agents` are installed in `system.primaryUser`'s `~/Library/LaunchAgents`, and reloaded at activation when the plist changed. NixOS's switch re-executes each user's manager and runs `system.userActivationScripts`, but restarts no changed user unit.
- **nix's trust.** nix substitutes a path only from a substituter it is allowed to use, and only with a signature by a key in `trusted-public-keys`. A flake's `nixConfig` applies only to a command run on that flake, once the person accepts it, and its substituters only for a trusted user. No input's `nixConfig` applies when a host's system is built.
- **Evaluation in the gate.** Under a sandbox that denies the network and the nix daemon's socket, `nix eval --offline --read-only` against a dummy store, `dummy://`, whose store directory is not `/nix/store`, reads the inputs' sources under `/nix/store` as plain files, and computes every store path under its own directory without writing one. The host's store opened read-only, `local?read-only=true`, opens its database immutable, and misses each path still in the database's write-ahead log, as the inputs a shell fetched minutes before are on a busy host. A flake cannot be evaluated either way, since its source is copied into the store first.

There are no area pages, so `docs/architecture/core.md` binds every path this change touches.

## The user's rulings

The user approved this design on 2026-11-06 ("Approve: start its code"), and ruled on its questions:

1. **The cache is `boomdrive`, signed by Cachix's own key.** "The token is all the workflow needs." Against the cache's name they wrote `{ cachix.push = "boomdrive"; }` (Decision 4, and The user's to rule on).
2. **The token comes from 1Password.** "Configure the GitHub Actions to use a service account token from 1Password to read the variable" (Decision 5).
3. **The user's steps are commands.** "Send as CLI commands" (After deploy).
4. **The set gives three systems.** Asked on 2026-11-05 whether the set leaves out `x86_64-darwin`, which nixpkgs 26.11 refuses, they answered "Leave it out: three systems" (Decision 2).

The first set is Flume `v0.10.0` and sluice `v0.68.0`, each the newest release on 2026-11-05, held there only if the runner's tests pass with them (Decision 6).

## The user's to rule on

1. **devenv's own push.** The user's `{ cachix.push = "boomdrive"; }` is devenv's push. Set in `devenv.nix`, it pushes every path a devenv shell realises, the shell's own tools among them, from every host that enters the shell holding a write token, the landing host's `devenv print-dev-env` before the gate's checks among them. core.47, which the user approved with this design, has this repository's workflow alone push, only the flake's own outputs, and only from a commit the gate passed. Recommended: `devenv.nix` pulls from the cache (`cachix.pull = [ "boomdrive" ]`) and pushes nothing, so a shell substitutes what the workflow pushed. Where the user wants a host's builds pushed, core.47 changes with it, and a host turns it on in its untracked `devenv.local.nix`, as devenv's documentation suggests. Until the user rules, `devenv.nix` pulls only (Decision 6).

## Goals / Non-Goals

**Goals:**

- `nix profile install github:agentplot/boomdrive` gives the runner, and a host's configuration gets the runner, Flume and sluice at versions the runner's tests passed with.
- A host whose nix trusts the family's cache substitutes every package of the set and compiles nothing. A host that does not trust it builds the same packages from source.
- A host's configuration that imports a module and sets `services.boomdrive` gets the runner's file, its service, its restarts and its tools, and carries none of its own.
- Every option is the runner's file in Nix, under the file's own names.
- Nothing secret is ever in the store, the tree or a log.

**Non-Goals:**

- **herdr and git.** They are the host's own (record 0012). The modules neither install nor check them. The runner refuses to start below its git floor, and says so in its log.
- **Linking the herdr plugins into herdr.** The set packages them. Linking one is `herdr plugin link`, herdr's own command, which the host's configuration or a person runs.
- **sluice's driver, herdr's server, Flume's settle loop and Flume's configuration.** They are the host's service manager's and Flume's (core.13). The modules start and configure only the runner.
- **Enrollment and the managed level.** A partition is enrolled by hand on the host. The managed level of record 0009 is unbuilt, and stays the control plane's to write.
- **Record 0012's releases.** Their archives, the installer and their attestations are record 0012's release workflow, its own change. The cache holds the flake's store paths, which are no release.
- **Cutting a tag.** A tag of this repository is the user's to cut. The workflow builds what one names.
- **home-manager.** A home-manager module is a later addition, should a host want one.

## Decisions

### 1. Where it lives, and its project

`flake.nix` and `flake.lock` are at the root, so `github:agentplot/boomdrive` is the flake. The workflow is `.github/workflows/cache.yml`. Everything else is under `nix/`:

```
nix/moon.yml
nix/inputs.nix              the lock's inputs, located or fetched, for the shell and the tests
nix/packages/boomdrive.nix  the runner
nix/packages/git-crates.nix the runner's git crates: each one's revision and hash
nix/packages/flume.nix      Flume, until Flume publishes a flake
nix/packages/flume-git-crates.nix Flume's git crates at its tag: each one's revision and hash
nix/packages/sluice.nix     sluice's flake's package, given its tag's version
nix/packages/herdr-plugin.nix
nix/packages/tag.nix        the tag a flake URL names
nix/modules/options.nix     services.boomdrive, shared by both modules
nix/modules/toml.nix        the file's text
nix/modules/darwin.nix
nix/modules/nixos.nix
nix/modules/activate.sh     the link, and NixOS's start and restart
nix/check.sh                the project's type check and test
nix/tests/                  the evaluated tests, the rendered files and the activation's tests
```

The moon project is `nix`, at `nix/`, `language: 'nix'`, listed in `.moon/workspace.yml`. It owns the flake and the workflow that builds it. Its four tasks run from the workspace root, in the fast layer: `format`, `lint`, `typecheck` and `test` (Decision 14).

Alternatives:

- **The flake under `nix/`**, as `github:agentplot/boomdrive?dir=nix`. Not taken: every reference to the flake would carry the query.
- **The flake's checks in the root project.** Not taken: core.7 and CLAUDE.md give each part with its own tooling a project of its own, one task per check.

### 2. What the set holds

`packages.<system>`, for `aarch64-darwin`, `aarch64-linux` and `x86_64-linux`, the systems sluice's flake builds that the flake's nixpkgs builds for:

| package | what | from |
|---|---|---|
| `boomdrive`, `default` | the runner | this tree |
| `flume` | `flume` | Flume's tag, by this flake's recipe |
| `sluice` | `sluice` and `wt-submit` | sluice's tag, by sluice's flake |
| `flume-herdr-plugin` | Flume's `plugins/herdr` | the `flume` package's source |
| `sluice-herdr-plugin` | sluice's `plugins/herdr` | the `sluice` package's source |

These are the host's tools of record 0009's amendment: the runner, Flume, sluice and herdr's plugins of theirs. herdr and git are the host's own (record 0012). timber, weir and boom are not in it: the runner and Flume run none of them on a host. weir is in the runner's `Cargo.lock` as a library.

Alternatives:

- **`x86_64-darwin` too, from nixpkgs 26.05.** Not taken: sluice's `nixpkgs` follows this flake's one `nixpkgs`, so its package would need its flake called a second time, and 26.05 keeps the platform only until the end of 2026.
- **`x86_64-darwin` from `nixos-unstable` with `allowDeprecatedx86_64Darwin = "force"`.** Not taken: nixpkgs builds no binary for it and says it "is not expected to function", so the workflow's job would build every toolchain from source, unsupported.

### 3. How each package is made

Each package is a derivation of the source the flake's lock names by its hash: the runner at this tree, Flume and sluice each at a release tag. The cache holds those derivations' outputs and nothing else (Decision 4), so a binary a host substitutes and one it builds are one store path (core.44).

- **The runner** is built from this tree, as sluice's flake builds sluice. That means `rustPlatform` from rust-overlay at the Rust `.prototools` names, `cargoBuildFlags = [ "-p" "boomdrive" ]`, and the check phase off, since the gate runs its tests. Its source is `Cargo.toml`, `Cargo.lock` and `crates/` alone, by `lib.fileset`. Its crates are fetched by `Cargo.lock`'s own checksums. Its git crates, weir's, are fetched by their locked revision under `outputHashes`. Each hash is kept in `nix/packages/git-crates.nix` beside the revision it was taken at, and a test holds that revision to `Cargo.lock`'s (Task notes 2.1). Its version is `Cargo.toml`'s.
- **sluice** is sluice's flake's `packages.default`, with `nixpkgs` and `rust-overlay` following this flake's. Its tagged tree says `0.0.0`, so the package is given its tag's version as sluice's release build is: `version` and `env.SLUICE_VERSION`. Where sluice's tree carries a version other than `0.0.0`, that is read instead, and nothing is given. The tag is read from `flake.nix`'s own input URL, so one string names it.
- **Flume** publishes no flake. A recipe here builds its tag the way sluice's flake builds sluice: rust-overlay at the Rust its own `.prototools` names, `-p flume --bin flume`, `Cargo.lock`'s checksums. Its git crates, weir's, are fetched as the runner's are, each hash kept in `nix/packages/flume-git-crates.nix` beside its revision, and the same test holds them to the tag's `Cargo.lock`, so Flume's tag moves with them. Its version is its tagged tree's `Cargo.toml`. The recipe is a bridge, and goes when Flume publishes a flake (What this change asks of others).
- **The herdr plugins** are `plugins/herdr` of each tool's package's source, copied whole, with the tool's version, so a plugin and its tool are one release. Nothing in them is rewritten. A plugin's scripts find the tool on the user's `PATH`, where the module puts it (Decision 11).

Alternatives:

- **Each tool's release archive, by the digest GitHub publishes.** Not taken: it re-packages each tool here, sluice's binaries relinked and re-signed on each platform because they name the Nix store of sluice's build. It has no `x86_64-darwin`, and no release carries an attestation. The cache serves binaries already, and they are the flake's own outputs.
- **Each tool's flake taken whole.** Not taken: sluice's says `0.0.0`, below any floor the runner names (core.38), and Flume has none.
- **One recipe here for every tool.** Not taken: each tool's flake is the one owner of its build, which the family cites rather than copies.

### 4. The family's cache

The set reaches a host as binaries from `boomdrive` on Cachix, served at `https://boomdrive.cachix.org` and signed by `boomdrive.cachix.org-1:vHLgmasPg2jKqkvPXWOOrYzspoGL2xxsW4fAuHBtiag=`, Cachix's own key for it, so the push needs the cache's token alone and no signing key (The user's rulings, 1; core.47).

- **What it holds.** Each package of each system the flake gives, with its runtime closure, as this repository's workflow builds them (Decision 5). This repository pushes nothing else to it.
- **The flake names it.** `flake.nix` holds:

  ```nix
  nixConfig = {
    extra-substituters = [ "https://boomdrive.cachix.org" ];
    extra-trusted-public-keys = [ "boomdrive.cachix.org-1:vHLgmasPg2jKqkvPXWOOrYzspoGL2xxsW4fAuHBtiag=" ];
  };
  ```

  nix reads it for a command run on the flake itself, `nix profile install github:agentplot/boomdrive` or `nix run`. It applies once the person accepts it at nix's prompt or has `accept-flake-config`, and its substituter only for a trusted user, or where the daemon's `trusted-substituters` names it.
- **A host's configuration names it itself.** nix reads no input's `nixConfig` when it builds a host's system. So a host that imports the modules and wants the binaries adds the same two lines to its own nix settings: `nix.settings.extra-substituters` and `nix.settings.extra-trusted-public-keys`, or its nix's own configuration where nix-darwin does not manage nix. The README gives the lines, and a gate test holds them to `nixConfig`'s. The modules add neither, since which caches a host trusts is the host's (Decision 13).
- **The modules' packages are the flake's own.** Each module's `package` and `tools` default to the flake's own `packages.<system>`, made with the flake's locked nixpkgs and never with the host's `pkgs`. So they are the paths the cache holds, whatever nixpkgs the host runs. A host that makes the flake's `nixpkgs` follow its own changes every derivation, and the cache holds none of them. The README says to leave it.
- **A host that does not trust the cache builds the set.** nix substitutes nothing whose signature it cannot check against a key the host trusts. A host whose nix does not trust the cache, or whose person declines the prompt, builds the same derivations from the locked source and gets the same store paths. So does a host whose path the cache no longer holds. Nothing is lost by distrust but time.

Alternatives:

- **A cache of our own, served by attic or from a bucket.** Not taken: the user has the Cachix cache, and a public cache needs no account to read.
- **The modules adding the cache to `nix.settings`.** Not taken: trusting a cache's key is the host's call. nix-darwin with `nix.enable = false` has no `nix.settings` to add it to.
- **The modules' packages made with the host's `pkgs`.** Not taken: each host's nixpkgs makes other paths, which the cache never holds.

### 5. The workflow that fills it

`.github/workflows/cache.yml`, the workflow `cache`:

- **When.** On a push to `main`, on a push of a tag `v*`, and by hand (`workflow_dispatch`). `main` holds only what the gate passed, as sluice lands it. A tag of this repository is cut from `main`, and names a set: the flake at that tag. A push of any other branch runs nothing.
- **Where.** One job for each system the flake gives, on GitHub's hosted runner of that system. `fail-fast` is off, so one system's failure leaves the others' pushes. A gate test holds the jobs' systems to the flake's.

  | system | runner |
  |---|---|
  | `x86_64-linux` | `ubuntu-24.04` |
  | `aarch64-linux` | `ubuntu-24.04-arm` |
  | `aarch64-darwin` | `macos-15` |

- **What each job does.**
  1. Checks out the commit.
  2. Installs nix, by `cachix/install-nix-action`.
  3. Fails, naming `OP_SERVICE_ACCOUNT_TOKEN`, where the environment gives none.
  4. Loads the cache's token from 1Password, by `1password/load-secrets-action`, as the step's output `CACHIX_AUTH_TOKEN`, masked in the log.
  5. Fails, naming `CACHIX_AUTH_TOKEN`, where the token it loaded is empty, before it builds anything.
  6. Sets up `cachix` for `boomdrive` with the token, by `cachix/cachix-action` with `skipPush: true`, so it pushes nothing of its own accord, and a path the cache holds is substituted, not built again.
  7. Builds every package of its system, each named by `nix eval --json .#packages.<system> --apply builtins.attrNames`, with `nix build --no-link --print-out-paths`, into a list of store paths.
  8. Pushes that list with `cachix push boomdrive`, each path with its runtime closure.

  A package that fails to build fails the job at step 7, and nothing of that system is pushed.
- **The runner's package is built and pushed with the rest.** It is `packages.<system>.boomdrive` and `default`, built in step 7 beside Flume's and sluice's, on every system, at the push of `main` that lands each change. A host substitutes the runner as it does the other two.
- **The tokens.** The workflow holds no Cachix token as a GitHub secret (The user's rulings, 2).
  - **The cache's token** is a token of the cache's own with write access to `boomdrive`. It is kept in 1Password, in the item `CACHIX_AUTH_TOKEN` of the vault `GitHub-Actions`, its field `credential`, the vault and item the family's workflows read their Cachix token from. Step 4 names it once, as `op://GitHub-Actions/CACHIX_AUTH_TOKEN/credential`, and only steps 5, 6 and 8 are given it.
  - **The service account's token** is a 1Password service account's, which may read that vault alone. It is the one secret of the repository's environment `cachix`, `OP_SERVICE_ACCOUNT_TOKEN`, given to steps 3 and 4 alone. The environment's deployment rule admits only `main` and tags `v*`, so a run on any other ref is refused it. The job names `environment: cachix`.
  - The user makes the token, the service account, the secret and the environment (After deploy). Neither token is ever in the tree, an option, a log or the store (core.4).
- **Hardening.** `permissions` is `contents: read`. Each action is pinned by its commit, with its tag in a comment. `concurrency` is one run for each ref at a time, never cancelling one in flight, so every commit of `main` is pushed.

Alternatives:

- **The cache's token as the environment's secret, `CACHIX_AUTH_TOKEN`.** Not taken: the user ruled the workflow reads it from 1Password through a service account (The user's rulings, 2), so the token is kept and turned over in one place.
- **A signing key of the user's own beside the token, `CACHIX_SIGNING_KEY`.** Not taken: Cachix signs what is pushed with the cache's own key (The user's rulings, 1).
- **Pushing every path a job builds**, cachix-action's default. Not taken: it pushes the toolchains and build inputs too, which no host substitutes.
- **Building in the gate, on the host that lands.** Not taken: the gate runs on one system, in a sandbox with no nix daemon (Decision 14).
- **Runners of the user's own hosts.** Not taken: GitHub's hosted runners give every system the flake gives to a public repository, and no host of a person's stands in the path of a public build.
- **Each pushed path also attested by the workflow**, `actions/attest-build-provenance` on its NAR. Not taken: nix checks a substitute by the cache's signature alone, so no install would ever check the attestation. Record 0012's releases carry theirs.

### 6. How a set is proven

A set is the one the runner's tests pass with, running the set's own Flume and sluice (core.45).

- **The shell gives them.** `devenv.nix` reads `flake.lock` through `nix/inputs.nix`, evaluates the flake's outputs with those inputs, and gives the runner's tests the set's programs by path: `BOOMDRIVE_SET_FLUME` and `BOOMDRIVE_SET_SLUICE`. They are never put on `PATH`, so a shell's `sluice exec` still runs the host's sluice. `devenv.nix` pulls from the family's cache (`cachix.pull = [ "boomdrive" ]`), so `devenv print-dev-env`, outside the sandbox, substitutes the very binaries a host substitutes. Where the host's nix does not trust the cache, it builds them, once per host for each set. It pushes nothing to the cache, since the workflow alone pushes (core.47; The user's to rule on, 1).
- **The tests run them.** `crates/boomdrive/tests/family.rs`, in `boomdrive:test`, on the tests' scratch host:
  - each program's `--version` is its tag's, the tag read from `flake.lock`;
  - the set's Flume's `--json status` of a scratch host with no place is read by `flume::Cli` and the runner's status readers, as the work-into-flume spec reads it;
  - the set's sluice's `status <id> --json` for an id it has no record of, in a scratch repository, is read by `sluice::Cli` as a missing entry.
  - Where either variable is unset, the test fails, saying the devenv shell gives it. It never skips, and never looks on `PATH`.
- **A set moves by one change.** It moves a tag in `flake.nix` and the lock together, and lands through the gate. `nix:typecheck` fails where a tag and the lock differ. The workflow then pushes the new set at that landing.
- **The fixtures stay.** Flume 0.4.1's and sluice 0.61.1's answers stay the unit fixtures of the runner's readers. They are what those versions printed. The family test reads the set's own.

The first set is Flume `v0.10.0` and sluice `v0.68.0`, the newest releases on 2026-11-05. Where the family test finds an answer of theirs the runner cannot read, the set takes the newest release of that tool the test passes with, and the build names what the newer one printed. The runner's readers do not change in this change.

Alternatives:

- **Fixtures of each set's answers alone.** Not taken: they prove what a version printed when someone captured it, by hand, and nothing makes a bump re-capture them.
- **Flume and sluice on the shell's `PATH`.** Not taken: an agent's `sluice exec` in the devenv shell would run the set's sluice, not the host's driver's.

### 7. How a host overrides a tool

- **A tool's version** moves by its input: `--override-input flume github:agentplot/flume/v0.10.1`, or `inputs.boomdrive.inputs.sluice.follows`. Flume's version follows its tree. sluice's follows its tree too once that carries its version. Until then the package's version must be given with the input (README).
- **A package** is replaced by the modules' `package` and `tools` (Decision 11).

An override is the host's. Its paths are not the cache's, so the host builds them. It is not proven here, and the runner's own floors judge it (core.38).

### 8. Two modules, one set of options

`darwinModules.default` and `nixosModules.default`, also named `boomdrive`, each import `nix/modules/options.nix`:

| option | type | default | |
|---|---|---|---|
| `services.boomdrive.enable` | bool | false | |
| `services.boomdrive.package` | package | the flake's `boomdrive` | the runner |
| `services.boomdrive.tools` | list of packages | the flake's `flume` and `sluice` | installed beside it |
| `services.boomdrive.settings` | the file | `{ }` | Decision 9 |
| `services.boomdrive.user` | string | none | NixOS: the service's user |
| `services.boomdrive.logFile` | path | `<home>/Library/Logs/boomdrive.log` | nix-darwin: the agent's log |

`package` and `tools` default to the flake's own packages for the host's system (Decision 4).

`settings` is RFC 42's: free-form TOML, with the README's keys declared for their types and documentation.

| key | type | where unset |
|---|---|---|
| `repositories` | list of `owner/name` | `[ ]`, always written |
| `repository_root` | string | left out: the runner's `~/Code` |
| `fetch_interval` | `<n>s`, `<n>m` or `<n>h` | left out: the runner's `2m` |
| `code_workspaces` | bool | left out: the runner's false |
| `control_plane` | table, `type` one of `crew` and `hosted`, its `poll_interval`, and `crew` or `hosted` free-form | left out |

A key the module does not declare passes to the file, and the runner judges it. Nothing the runner defaults is defaulted again here, so the two never drift.

On nix-darwin the user is `system.primaryUser`, since nix-darwin installs user agents for that user alone. On NixOS it is `services.boomdrive.user`.

A host's configuration names its runner as:

```nix
services.boomdrive = {
  enable = true;
  settings = {
    repository_root = "~/Code";
    code_workspaces = true;
    repositories = [ "agentplot/flume" "agentplot/sluice" ];
    control_plane = {
      type = "crew";
      crew = { host = "host-a"; partitions = [ { label = "home"; state_repository = "example/state"; } ]; };
    };
  };
};
```

Alternatives:

- **One option per key at the top**, `services.boomdrive.repositories`. Not taken: two names for each key, the option's and the file's, where core.12 and core.46 want one.
- **Only declared keys.** Not taken: a key a later runner reads would need the module first.
- **Each runner default repeated as the option's default.** Not taken: two defaults for each key, which drift.

### 9. The file

- **Rendered in Nix.** `nix/modules/toml.nix` renders `settings` at evaluation: strings, booleans, integers, lists of strings, tables and lists of tables. It refuses any other value, naming its path. Top-level keys come before tables, each sorted by name. The first line is `# Written by the boomdrive flake's module from services.boomdrive.settings; a change goes there.` So the gate compares the text without a build, and one `settings` gives one text on both platforms.
- **In the store.** The text is `pkgs.writeText "boomdrive-config.toml"`. The service runs `boomdrive run --config <that file>`, so the service reads exactly what the module wrote, and a changed file changes the service (Decision 10).
- **Linked at the runner's own path.** The activation links the user's `~/.config/boomdrive/config.toml` to the same file, so `boomdrive enroll <label>` and `boomdrive run --once` run by hand read it. A link there is replaced. A file or folder that is no link is moved once to `config.toml.before-nix`, and the activation names both paths. Where `config.toml.before-nix` is there already, it changes neither, names both, and fails the link alone, so the rest of the activation goes on: the service reads its own `--config` either way. A shell with another `XDG_CONFIG_HOME` passes `--config`.
- **The host's level.** It is the level nix writes under record 0009. Once a control plane writes a managed level, that level is its own file.

Alternatives:

- **A real file written at activation, at 0600**, as a host's configuration writes it today. Not taken: it needs a writer of its own on each platform, and a restart that compares what it wrote. The file holds no secret (Decision 12), so a link into the store, readable by every user, hides nothing.
- **A second file moved over the first one aside, or to a name of its own.** Not taken: the first would be lost, or the names would pile up unseen.
- **No link, and `--config` on every command run by hand.** Not taken: `boomdrive enroll` would read another file, or none.
- **`pkgs.formats.toml`.** Not taken: its text is a build's output, which the gate cannot read without building.

### 10. The service and its restarts

**nix-darwin.** `launchd.user.agents.boomdrive`, whose label is nix-darwin's `org.nixos.boomdrive`:

- `ProgramArguments` `<package>/bin/boomdrive run --config <file>`;
- `RunAtLoad`, `KeepAlive`, `ThrottleInterval` 30, so an exit 2 is tried again no more than twice a minute;
- `ProcessType` `Interactive`, so neither the runner nor what Flume starts under it runs throttled;
- `AbandonProcessGroup`, so a stop ends the runner alone and a `flume launch` in flight runs to its end;
- `EnvironmentVariables` `HOME` and `PATH` alone, so the runner and what it runs read what a pass by hand reads;
- `StandardOutPath` and `StandardErrorPath` `logFile`.

nix-darwin reloads the agent when its plist changes, which is when the package or the file changes, and leaves it running when neither does. The link is made by `system.activationScripts.postActivation`, as the primary user (`sudo --user`), with `activate.sh link`. Evaluation fails where `system.primaryUser` is unset, or `users.users.<primary user>.home` is.

**NixOS.** `systemd.user.services.boomdrive`:

- `ExecStart` `<package>/bin/boomdrive run --config <file>`, `wantedBy` `default.target`;
- `unitConfig.ConditionUser` the user, so no other user's manager runs it;
- `Restart=always`, `RestartSec=30s` and `StartLimitIntervalSec=0`, as launchd's `KeepAlive` and throttle;
- `KillMode=process`, as `AbandonProcessGroup`;
- `Environment` `PATH` alone of the module's own; the user's manager gives `HOME` and `XDG_RUNTIME_DIR`, and the system's `systemd.globalEnvironment` what it gives every unit;
- `users.users.<user>.linger = true`, so the runner runs from boot on a host no one logs into.

NixOS restarts no changed user unit. So `system.userActivationScripts.boomdrive` runs `activate.sh keep` for the user, after linking. That compares the command line of the unit's main process, by `systemctl show` and `ps`, with the unit's, and starts the unit where it has none, restarts it where they differ, and does nothing where they are one, so a new package restarts it as a new file does. It is `systemctl --user --no-block`, since it runs inside the user's `nixos-activation.service`. Any other user's activation does nothing. Evaluation fails where the user is not in `users.users`, or where `settings` names a hosted control plane on a systemd older than 256 (record 0018).

The runner reads its file again at every pass, so most changes would take effect without a restart. A restart on every change is still taken, since `[control_plane]` is read only at start and the restart is what a host would do by hand.

Alternatives:

- **home-manager's `launchd.agents` and `systemd.user.services`**, as a host's configuration does it today. Not taken: every host that imports the module would need home-manager. A home-manager module is a later addition.
- **A NixOS system service with `User=`**, which the switch restarts on its own. Not taken: records 0012 and 0018 have a user unit, which has the user's runtime directory, and `systemd-creds --user` is the user's.
- **A restart only when `[control_plane]` changes.** Not taken: it needs the module to know which keys the runner reads at start.

### 11. Its tools on `PATH`

Each module puts `package` and `tools` in `environment.systemPackages`. `tools` defaults to the flake's `flume` and `sluice`. A host that installs its own sets `tools = [ ]`, or its own packages.

The service's `PATH` is the profiles, never a package's own folder, so the runner, the agents Flume launches and a person's shell run one copy of each tool:

- nix-darwin: `/etc/profiles/per-user/<user>/bin`, `<home>/.nix-profile/bin`, `<home>/.local/state/nix/profile/bin`, `/run/current-system/sw/bin`, `/nix/var/nix/profiles/default/bin`, `/usr/local/bin`, `/usr/bin`, `/bin`, `/usr/sbin`, `/sbin`;
- NixOS: `/etc/profiles/per-user/<user>/bin`, `<home>/.nix-profile/bin`, `<home>/.local/state/nix/profile/bin`, `/run/wrappers/bin`, `/run/current-system/sw/bin`, `/nix/var/nix/profiles/default/bin`.

git and herdr are found there, from wherever the host installs them. A Mac whose only git is the system's, older than 2.48, gets the runner's refusal in its log. The README says so.

Alternatives:

- **The set's tools first on the service's `PATH`, from their store paths.** Not taken: the runner's sluice and the host's sluice driver could be two versions on one clone.
- **The user's profile, `users.users.<user>.packages`.** Not taken: nix-darwin and NixOS both have the system profile, and a host with home-manager may manage the user's.

### 12. Credentials and enrollment

- **No option holds a secret.** There is no token, key or enrollment option.
- **A key named for one fails evaluation.** Any key of `settings`, at any depth, named `token`, `key`, `secret` or `password`, or ending in `_token`, `_key`, `_secret` or `_password`, fails evaluation. The failure names its path, and says a partition's key is made on the host by `boomdrive enroll`. The runner would refuse such a file too, but only once the store holds it, readable by every user (core.4).
- **Enrollment is by hand.** After a switch, a person runs `boomdrive enroll <label>` on the host, the token on standard input. The key goes to the login keychain, or to `systemd-creds` of the user (core.32, record 0018). The agent runs in the user's login, where the keychain is. The unit runs under the user's manager, where `systemd-creds --user` decrypts.
- **The forge.** The host's git credential helper answers the runner's fetches, as it does today. The modules name none.
- **The cache's token is the workflow's.** It is kept in 1Password, and loaded by the workflow's job in the repository's `cachix` environment, through a service account whose token is that environment's secret, for the push alone (Decision 5). A host reads the public cache with no token.

### 13. What stays the host's

The host's configuration decides which hosts set `enable`, which user, which repositories, which control plane and partitions, whether its nix trusts the family's cache, its git and herdr and its plugins' linking, its secrets and enrollments, its Flume configuration, its sluice driver, and its doctor or monitoring. The modules decide only how the runner is installed, configured from `settings`, kept running and restarted.

### 14. The gate

The `nix` project's tasks run in the gate's sandbox, with no network and no nix daemon. They evaluate and never build.

- **The inputs are there first.** `devenv.nix` reads `flake.lock` through `nix/inputs.nix`, which takes each locked input by `builtins.fetchTree`. That fetches them during `devenv print-dev-env`, outside the sandbox. It gives the tasks their paths as `BOOMDRIVE_FLAKE_INPUTS`, a JSON object of input name and store path.
- **The tasks read no store.** `nix/check.sh` runs the pinned nix as `nix eval --offline --read-only --impure --store 'dummy://?store=/boomdrive-check/store' --extra-experimental-features nix-command`, its configuration, caches and state under the task's scratch folder. It evaluates `nix/tests`, which reads each input's source as plain files at the path `BOOMDRIVE_FLAKE_INPUTS` gives it, and calls `flake.nix`'s `outputs` with them and `self` as this tree, as flake-compat does. Every path is computed under the dummy store's own directory and never written, so a test compares paths with each other, never with `/nix/store`'s.
- **`nix:format`**: `nixfmt --check` on `flake.nix` and `nix/`.
- **`nix:lint`**: `statix check` and `deadnix --fail` on the same files, and `actionlint` on `.github/workflows/cache.yml`.
- **`nix:typecheck`**: every package of every system the flake gives evaluates to a derivation, each module evaluates in a minimal system of its platform, and each input's tag in `flake.nix` is the lock's.
- **`nix:test`**: the evaluated tests of the three specs, the rendered files compared byte for byte with `nix/tests/files`, `activate.sh`'s cases run by `sh` against stub `systemctl` and `ps` in a scratch folder, and the workflow's and README's text held to the flake's systems and `nixConfig`.
- **`boomdrive:test`** gains `family.rs` (Decision 6) and `module_files.rs`, which reads each file under `nix/tests/files` with `config::read`.
- **The runner's package.** The gate evaluates it on every system the flake gives, and its crates are the ones `boomdrive:test` compiles. The one thing its nix build adds is the git crates' hash, and `nix:test` holds each to the revision `Cargo.lock` names, so a weir bump without its hash fails before it lands. The workflow builds the package on every system at the push of `main` that lands the change, before anything of a system is pushed (Decision 5). A host that pins a tag, or a commit of `main` whose workflow passed, takes a runner built already.
- **Pins.** nix, nixfmt, statix, deadnix and actionlint are `devenv.nix`'s, asserted to their versions as git is (core.8), and `check-toolchain.sh` holds them. statix answers no `--version`, so the check reads its version from the name of the store path its program is in. `treefmt.nix` excludes `nix/tests/files/**`, which the tests compare byte for byte, as it excludes the tools' fixtures. yamlfmt already owns the workflow.

Alternatives:

- **`nix flake check` in the gate.** Not taken: the sandbox has no daemon, and a flake's evaluation copies its source into the store.
- **The host's store, opened read-only**, `local?read-only=true`. Not taken: it opens the store's database immutable, which misses each path still in its write-ahead log, as the inputs the shell has just fetched are (Context).
- **The executor opened for nix.** Not taken: it would open the network to every check. sluice's executor has no allowance for the daemon's socket alone.
- **The runner's package built by the shell, as Flume's and sluice's are.** Not taken: the shell would build it again at every entry after an edit to a crate, compiling the runner twice, and on the landing host's system alone. The workflow builds it on every system.

### Rules relied on

- **core.4, core.32**: no credential in an option, the file, the store, the tree or a log; the key stays in the host's credential store; the cache's token stays in 1Password, loaded by the workflow's job alone (Decisions 5 and 12).
- **core.5**: the module writes the repositories the host names, and no stream, kind or drive.
- **core.7, core.8**: the flake is a project of its own, one task per check, and owns its workflow; nix, nixfmt, statix and actionlint are pinned and checked; the rendered files are excluded by name with their reason.
- **core.12**: each option under `settings` is its key's name in the file; `logFile` names a file.
- **core.13**: the modules start the runner alone.
- **core.17**: the file is the host's level, written by nix, which the runner reads as it reads one written by hand.
- **core.18**: the modules write only the runner's own file.
- **core.38**: an override is judged by the runner's floors.
- **core.41**: tests and examples use neutral names: `alice`, `bob`, `host-a`, `home`, `work`, `example/state`, `control.example`. Beyond what core.41 asks, the change names no host, fleet or repository of a person's, as its brief asks of a public repository.
- **core.44, core.45, core.46 and core.47**, which this change adds, and **core.37** as it amends it.

### Rules departed from

None. core.37's details are amended by record 0020: a store path a host's nix substitutes from the family's cache, checked against the key the host trusts, or builds from the locked source, is no release, so core.37's check of a release's provenance does not bind it. That narrows record 0012's sentence that both install routes install the same attested releases, as the user ruled (record 0020).

## What this change asks of others

Each is told to the orchestrator, not done here.

- **Flume**: publish a flake whose `packages.default` builds `flume` at its tag, as sluice's and timber's do. This flake then takes it from there, and `nix/packages/flume.nix` and `nix/packages/flume-git-crates.nix` go.
- **sluice and timber**: carry each release's version in the tagged tree, as Flume's release commit does, so a pure build of a tag reports it. This flake then stops giving sluice its version.
- **sluice and timber, on `x86_64-darwin`**: drop it from their flakes' systems, as this flake gives none, since nixpkgs 26.11 refuses it and their packages for it stop evaluating once their locks reach it (Decision 2).
- **The family, on pushing binaries from each tool's own repository.** Recommended: yes, to the one family cache, `boomdrive`, once a tool carries its version in its tagged tree and publishes a flake. Each tool's release workflow pushes its own flake's `packages` at each tag. This flake then takes each tool's package as the tool's flake gives it, without making its `nixpkgs` follow this flake's, so a host substitutes the very paths the tool's own release pushed. This workflow then builds and pushes only the runner and what a tool did not push. One cache keeps one key for a host to trust, and each tool's build keeps one owner. Until then this workflow builds and pushes all three: a tool's own push now would be paths this flake does not name.
- **A host's configuration, and its change that retires its views once the runner's workspaces stand:**
  - import `darwinModules.default` and set `services.boomdrive` with the options of Decision 8, `settings.code_workspaces` and `settings.repositories` among them, in place of its runner's module and package;
  - trust the family's cache in its own nix settings, and leave the flake's `nixpkgs` its own, so its hosts substitute the set;
  - set `tools = [ ]` while it installs its own Flume and sluice;
  - point its check at the module's options, and its doctor at the agent `org.nixos.boomdrive` and at `logFile`;
  - read its own rule on the file's mode against a file linked into the store, which holds no secret.
- **Record 0012's release workflow**, when built: its archives and their attestations serve the installer and `upgrade`; the flake's route stays the cache.

## Risks / Trade-offs

- **The token can push to every host that trusts the cache.** Anyone holding it can push paths the cache signs. It is a write token of the cache, kept in one 1Password item, read through a service account that reads one vault, whose token one environment holds that admits `main` and tags `v*` alone, by a job whose actions are pinned by commit.
- **The service account reads its whole vault.** Its token reads every item of `GitHub-Actions`, not only the cache's. The environment gives it to two steps of one job, and to no run of another ref.
- **The runner's package is first built after it lands.** A change that breaks only its nix build on one system fails that system's job at the landing push, which pushes nothing of it. A host pinned to that commit builds it and fails, keeping its running generation. A host that pins a tag, or a commit whose workflow passed, never meets it.
- **The cache's storage is bounded.** Cachix drops the paths least recently fetched. A host pinned to an old commit builds what was dropped.
- **An Intel Mac gets no package of the set.** nixpkgs 26.11 builds nothing for `x86_64-darwin`, so the flake gives it none. A host there installs the tools as it does today, and nix-darwin's module does not evaluate for it.
- **A host that makes the flake's `nixpkgs` follow its own builds everything.** Its paths are not the cache's. The README says to leave it.
- **sluice's version is given here** until its tree carries it. A host that overrides sluice's input must give its version too.
- **The first set may stop short of the newest release** where the runner cannot read a newer answer. The build names what the newer one printed, for a change to the runner's readers.
- **A person's own file is moved aside.** A host that edited `~/.config/boomdrive/config.toml` by hand finds it at `config.toml.before-nix`.
- **nix-darwin installs user agents for the primary user alone.** A Mac whose runner is another user's needs home-manager, or a later module.
- **The checks' paths are a dummy store's.** The gate proves that every package evaluates and what the modules write, never that a path is the one the cache holds. The workflow's build is where a path is made, and a host's nix checks each substitute by the cache's key.

## Migration Plan

Nothing changes on a host until its configuration imports a module. A host moving from its own module:

1. adds the family's cache to its nix settings, in the switch before or the one that enables `services.boomdrive`;
2. disables its own runner's agent and file writer in the same switch that enables `services.boomdrive`;
3. finds its file moved to `config.toml.before-nix`, where it was a real file;
4. finds the runner under nix-darwin's label `org.nixos.boomdrive`, its data directory, clones and views as they were, since the data directory follows `HOME` and not the service.

Two runners never run at once: the runner's own lock on its data directory refuses the second with exit 2.

## After deploy

Each step is the user's, run in this order on a Mac signed in to `gh` as an admin of the repository and to 1Password's `op` as a person who may make service accounts (The user's rulings, 3). Steps 1 to 5 come before the change lands, so the landing push finds the token. No step puts a token on a command line or on the screen: each goes from the clipboard or from one command's output straight into the next.

1. Make the environment `cachix`, its deployments limited to `main` and the tags `v*`:

   ```sh
   gh api --method PUT repos/agentplot/boomdrive/environments/cachix \
     -F 'deployment_branch_policy[protected_branches]=false' \
     -F 'deployment_branch_policy[custom_branch_policies]=true'
   gh api --method POST repos/agentplot/boomdrive/environments/cachix/deployment-branch-policies \
     -f name=main -f type=branch
   gh api --method POST repos/agentplot/boomdrive/environments/cachix/deployment-branch-policies \
     -f name='v*' -f type=tag
   ```

2. See whether the vault's item already holds a token that writes the cache. It prints `Write` or `Admin` where it does, `Read` where its token writes other caches only, and `no item` where the vault has none, and nothing of the token:

   ```sh
   if op read 'op://GitHub-Actions/CACHIX_AUTH_TOKEN/credential' >/dev/null 2>&1; then
     op read 'op://GitHub-Actions/CACHIX_AUTH_TOKEN/credential' |
       sed 's/^/Authorization: Bearer /' |
       curl -fsS -H @- https://app.cachix.org/api/v1/cache/boomdrive | jq -r .permission
   else
     echo 'no item'
   fi
   ```

   `Write` or `Admin`: go to step 5. `no item`: steps 3 and 4. `Read`: leave the item to what reads it, and say so at the change's review, since the workflow then names an item of its own.
3. Make a token of the cache with write access, on its settings page, which has no command line: <https://app.cachix.org/cache/boomdrive/settings?binaryCacheSettingsTab=authtokens>. Copy it to the clipboard; the page shows it once.
4. Store it from the clipboard as the item `CACHIX_AUTH_TOKEN` of the vault `GitHub-Actions`, its field `credential`, then empty the clipboard:

   ```sh
   pbpaste |
     jq -Rs '{title: "CACHIX_AUTH_TOKEN", category: "API_CREDENTIAL",
       fields: [{id: "credential", label: "credential", type: "CONCEALED", value: rtrimstr("\n")}]}' |
     op item create --vault GitHub-Actions - >/dev/null
   pbcopy </dev/null
   ```

5. Make a service account that may read that vault alone, its token going straight into the environment's secret `OP_SERVICE_ACCOUNT_TOKEN`:

   ```sh
   op service-account create boomdrive-cache --vault GitHub-Actions:read_items --raw |
     gh secret set OP_SERVICE_ACCOUNT_TOKEN --env cachix --repo agentplot/boomdrive
   ```

6. After the landing push, the run of `cache` on `main` passes on every system:

   ```sh
   gh run watch --repo agentplot/boomdrive --exit-status \
     "$(gh run list --repo agentplot/boomdrive --workflow cache.yml --branch main --limit 1 --json databaseId --jq '.[0].databaseId')"
   ```

   Where a job failed naming `OP_SERVICE_ACCOUNT_TOKEN` or `CACHIX_AUTH_TOKEN`, the step that makes it is done again, then `gh workflow run cache.yml --repo agentplot/boomdrive --ref main` pushes the set.
7. The cache holds the set: `nix path-info --store https://boomdrive.cachix.org "$(nix eval --raw github:agentplot/boomdrive#packages.aarch64-darwin.boomdrive.outPath)"` answers with the path.

## Task notes

**1.1** `flake.nix`:

```nix
nixConfig = {
  extra-substituters = [ "https://boomdrive.cachix.org" ];
  extra-trusted-public-keys = [ "boomdrive.cachix.org-1:vHLgmasPg2jKqkvPXWOOrYzspoGL2xxsW4fAuHBtiag=" ];
};
inputs = {
  nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  rust-overlay = { url = "github:oxalica/rust-overlay"; inputs.nixpkgs.follows = "nixpkgs"; };
  sluice = {
    url = "github:agentplot/sluice/v0.68.0";
    inputs.nixpkgs.follows = "nixpkgs";
    inputs.rust-overlay.follows = "rust-overlay";
  };
  flume = { url = "github:agentplot/flume/v0.10.0"; flake = false; };
  nix-darwin = { url = "github:nix-darwin/nix-darwin"; inputs.nixpkgs.follows = "nixpkgs"; };
};
```

`nix-darwin` is the tests'. A host's configuration follows its own, which changes none of the packages. The outputs are `packages`, `darwinModules` and `nixosModules`. `flake.lock` is made by `nix flake lock`, outside the sandbox.

**1.2** `nix/inputs.nix` takes `flake.lock` and an optional `paths`, and gives `inputs`, the root's inputs, and `paths`, each node's source by its store path. A node's source is `builtins.fetchTree` of its `locked` attributes, or the path `paths` names for it, read as plain files. A flake's node is its `outputs` called with its own inputs, `follows` followed from the root. In `devenv.nix` it fetches them, and `env.BOOMDRIVE_FLAKE_INPUTS` is the JSON of its `paths`. `nix/tests` reads them from those paths.

**1.3** `devenv.nix` gains `pkgs.nix`, `pkgs.nixfmt`, `pkgs.statix`, `pkgs.deadnix` and `pkgs.actionlint`, each asserted to the version it pins as git is. `check-toolchain.sh` checks each on `PATH` is that version. `nix:format` runs `nixfmt --check` on `flake.nix` and each `.nix` file under `nix/`, by name, since nixfmt no longer takes a folder. `nix:lint` runs `statix check` on each and `deadnix --fail` on both. `nix:format`, `nix:lint`, `nix:typecheck` and `nix:test` declare as inputs `/flake.nix`, `/flake.lock`, `/nix/**/*`, `/devenv.nix` and `/devenv.lock`. `nix:lint` and `nix:test` also declare `/.github/workflows/**/*`, and `nix:test` `/README.md`. `nix:typecheck` and `nix:test` also declare `/Cargo.toml`, `/Cargo.lock`, `/.prototools` and `/crates/**/*`, since the runner's package is computed from them.

**1.4** `nix/check.sh typecheck` and `nix/check.sh test`. Each runs the pinned nix as Decision 14 says, on the attribute of `nix/tests` its argument names, with `NIX_CONF_DIR`, `NIX_USER_CONF_FILES` and the four `XDG_` folders under the task's scratch folder and `NIX_CONFIG` unset, so nix reads no configuration of the host's and writes nothing in the home. It fails, naming the variable, where `BOOMDRIVE_FLAKE_INPUTS` is unset. It reads `flake.nix`'s own `inputs` and `nixConfig` by importing the file, as an attribute set. It prints each failure the evaluation returns as one line, `nix: <test>: <what>`, and exits 1 where there is one. A test returns a list of failures, empty when it holds, never an assertion that stops the rest.

**2.1** `nix/packages/git-crates.nix` is an attribute set of each of `Cargo.lock`'s git packages, by `<name>-<version>`, to `{ rev; hash; }`. `nix/packages/boomdrive.nix` gives `cargoLock = { lockFile = ../../Cargo.lock; outputHashes = mapAttrs (_: c: c.hash) gitCrates; }`. A test reads `Cargo.lock` with `builtins.fromTOML`, and fails where a git package has no entry, an entry has no package, or an entry's `rev` is not the `#<rev>` of the package's `source`, naming the package and both revisions. `meta.mainProgram = "boomdrive"`. Platforms are the flake's three.

**2.2** `nix/packages/flume.nix` takes the source and builds as sluice's flake does, reading Rust's version from the source's `.prototools` and the package's from its `Cargo.toml`. `cargoBuildFlags = [ "-p" "flume" "--bin" "flume" ]`, `doCheck = false`. Its `outputHashes` are `nix/packages/flume-git-crates.nix`'s, of the shape of 2.1's, held by 2.1's test to the tag's `Cargo.lock`. The overridden input is `nix/tests/tree-9.9.9`, a tree holding only a `Cargo.toml`, and the case reads the package's source and version, never its derivation, since the tree has no crates.

**2.3** `nix/packages/sluice.nix` takes sluice's package and its source. Where the source's `Cargo.toml` workspace version is `0.0.0`, it gives `overrideAttrs` `version` and `env.SLUICE_VERSION`, from the tag in `flake.nix`'s sluice URL without its `v`. Otherwise it gives the package unchanged.

**2.4** `nix/packages/herdr-plugin.nix` takes a tool's package and copies `${source}/plugins/herdr` to `$out`, unchanged, `version` the tool's.

**2.5** The tag check reads each input's URL from `flake.nix`'s `inputs` and its `original.ref` from `flake.lock`, and fails naming both where they differ.

**3.1** `devenv.nix` sets `env.BOOMDRIVE_SET_FLUME = "${flume}/bin/flume"` and `env.BOOMDRIVE_SET_SLUICE = "${sluice}/bin/sluice"`, from the flake's own `packages` for the shell's system, evaluated with `nix/inputs.nix`'s inputs. It puts neither on `PATH`. It sets `cachix.pull = [ "boomdrive" ]`, and no `cachix.push` (The user's to rule on, 1).

**3.2** `crates/boomdrive/tests/family.rs` uses `tests/common`'s scratch host. It reads `flake.lock` with `serde_json` for `nodes.flume.original.ref` and `nodes.sluice.original.ref`. Flume's scratch host is as the 0.4.1 fixtures' README describes, with no place. sluice's scratch repository has a policy and an `origin` in the same scratch folder. Each answer is read through `flume::Cli` and `sluice::Cli`. `crates/boomdrive/moon.yml`'s `test` gains `/flake.lock` and `/nix/tests/files/**/*` as inputs.

**4.1** `nix/modules/toml.nix`: `render settings -> string`. Strings are TOML basic strings, with `"`, `\` and control characters escaped. Booleans, integers, lists of strings, tables as `[a.b]`, and lists of tables as `[[a.b]]`. A key that is not bare is quoted. Any other value fails, naming its path. `nix/tests/files/empty.toml`, `crew.toml` and `hosted.toml` are the spec's three settings, written once by the renderer and read here as the expected text.

**4.2** `nix/modules/options.nix` declares the options of Decision 8, `package` and `tools` defaulting to the flake's own `packages.${pkgs.stdenv.hostPlatform.system}`, and holds the credential check of Decision 12 as an assertion whose message names every path found. `crates/boomdrive/tests/module_files.rs` reads each file under `nix/tests/files` with `config::read`, against a scratch home, and checks each repository and the control plane's type.

**5.1** `nix/modules/darwin.nix` evaluated by the tests in a nix-darwin system of `aarch64-darwin` with `system.primaryUser = "alice"`, `users.users.alice.home = "/Users/alice"` and `system.stateVersion` set. The tests read `launchd.user.agents.boomdrive.serviceConfig` key by key, and `environment.systemPackages`.

**5.2** `nix/modules/activate.sh link <file>` makes the link of Decision 9 under `$HOME`, saying a line only where it moved or linked something. `nix/modules/activate.sh keep <user> <program> <argument>...` does Decision 10's NixOS step. Each is shellcheck'd and laid out by treefmt. Its tests run each case in a scratch `HOME`, with `systemctl` and `ps` stubs first on `PATH` that record their calls.

**6.1** `nix/modules/nixos.nix` evaluated by the tests in a NixOS system of `x86_64-linux` with `users.users.alice` and `users.users.bob` normal users, `services.boomdrive.user = "alice"`, and `boot.loader.grub.enable = false`, `fileSystems."/"` and `system.stateVersion` set. The tests read `systemd.user.services.boomdrive` and `systemd.user.units."boomdrive.service".text`, `users.users.alice.linger`, `system.userActivationScripts.boomdrive` and `environment.systemPackages`. The old systemd is `systemd.package` overridden to report 255.

**7.1** The cache's tests, in `nix:test`:
- `nixConfig` is read from `flake.nix` imported as an attribute set, and holds exactly the substituter and key of Decision 4.
- The README's **Installing with nix** holds that substituter and that key, each as a string of its nix block, read with `builtins.readFile`.
- Each module is evaluated in its test system with `nixpkgs.overlays` holding one overlay that adds a package, so the host's `pkgs` differs from the flake's. `environment.systemPackages` holds the `outPath`s of the flake's own `packages.<system>` `boomdrive`, `flume` and `sluice`, and `nix.settings.substituters` and `nix.settings.trusted-public-keys` name nothing of the cache's.

**7.2** `.github/workflows/cache.yml`, laid out by yamlfmt and linted by actionlint. `on.push.branches: [main]`, `on.push.tags: ['v*']` and `on.workflow_dispatch`. `permissions: contents: read`. `concurrency: { group: cache-${{ github.ref }}, cancel-in-progress: false }`. One job, `build`, `environment: cachix`, `strategy.fail-fast: false`, its `matrix.include` the three `{ system, runner }` of Decision 5, `runs-on: ${{ matrix.runner }}`. Its steps are Decision 5's eight, the actions `actions/checkout`, `cachix/install-nix-action`, `1password/load-secrets-action` (v5.0.1, `id: cachix-token`) and `cachix/cachix-action` (v17, `name: boomdrive`, `authToken: ${{ steps.cachix-token.outputs.CACHIX_AUTH_TOKEN }}`, `skipPush: true`), each pinned by its commit with its tag in a comment. Step 3 is given `OP_SERVICE_ACCOUNT_TOKEN: ${{ secrets.OP_SERVICE_ACCOUNT_TOKEN }}` through `env`, and fails with `::error::OP_SERVICE_ACCOUNT_TOKEN is not set in the cachix environment`. Step 4's `env` is the same `OP_SERVICE_ACCOUNT_TOKEN` and `CACHIX_AUTH_TOKEN: op://GitHub-Actions/CACHIX_AUTH_TOKEN/credential`, the only place the workflow names the vault and the item. Step 5 is given the output through `env` and fails with `::error::CACHIX_AUTH_TOKEN loaded from 1Password is empty`. Step 8 is given it as `CACHIX_AUTH_TOKEN`. No step echoes either token, and no other step is given one. A `nix:test` case reads the file's text and fails where a system the flake's `packages` gives has no `system: <system>` line, or the file names a system the flake does not give, or it names a cache other than `nixConfig`'s.

**8.1** README gains **Installing with nix**, after **Running on a host**:
- the profile install, and nix's prompt to accept the flake's `nixConfig`;
- trusting the family's cache in a host's own nix settings, the two lines of Decision 4, and leaving the flake's `nixpkgs` its own;
- what a host that trusts no cache gets: the same packages, built from source;
- pinning a tag, or a commit of `main` whose `cache` run passed;
- a host's configuration importing a module, with Decision 8's example;
- `tools = [ ]`;
- enrolling after the switch;
- git 2.48 on a Mac;
- linking a herdr plugin with herdr's command;
- overriding a tool's input, with sluice's version given beside it, which the host builds.

`tests/readme.rs` still reads the README's TOML blocks.

**8.2** `CLAUDE.md` lists the `nix` project beside the crates, and says it owns the flake and `.github/workflows/cache.yml`. `openspec/config.yaml`'s context names the flake and the family's cache in its stack.
