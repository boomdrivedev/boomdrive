# Design

## Context

See proposal.md for why. This change adds a flake to the runner's repository: the family's host tools pinned together, and the runner's service as nix-darwin and NixOS modules. The runner itself does not change.

What stands:

- **This repository**, at `f6d9092`.
  - No nix but `devenv.nix` and `treefmt.nix`. `treefmt.nix` already formats `.nix` files with deadnix and nixfmt.
  - The runner reads one TOML file: `--config <file>`, else `$XDG_CONFIG_HOME/boomdrive/config.toml`, else `~/.config/boomdrive/config.toml`. It refuses a key it does not read. It reads the file again at every pass, but `[control_plane]` only at start (the configuration spec). `config::read(file, home)` is its reader.
  - The command line is `run [--config] [--once]` and `enroll [--config] [--json] <label>`. `enroll` reads the configuration for the control plane's `url` and the partition, and the token from standard input.
  - The runner runs `git`, `flume` and `sluice` by name from `PATH`, and refuses to start below git 2.48. Its tests reach Flume and sluice through stubs, over fixtures of Flume 0.4.1's and sluice 0.61.1's answers. `flume::Cli` and `sluice::Cli` take the program's path.
  - Its version is `Cargo.toml`'s, `0.0.0`. It has no release, no release workflow and no CI.
  - The gate is sluice's sandbox with `network = "localhost"`. sluice's `[tests] toolchain`, `devenv print-dev-env`, runs outside the sandbox, with the network and the nix daemon.
- **The family's flakes**, read 2026-11-05.
  - sluice (`v0.67.0`) and timber (`v0.5.3`) publish `packages.default`, built from source with rust-overlay at the Rust `.prototools` names, for `aarch64-darwin`, `x86_64-darwin`, `aarch64-linux` and `x86_64-linux`. Their tagged tree says `0.0.0`. A release build is given its version as `SLUICE_VERSION` or `TIMBER_VERSION` under `--impure`, so a pure build of the tag says `0.0.0`.
  - Flume (`v0.10.0`) publishes no flake. Its tag names a release commit with the version written into `Cargo.toml` (`set-version.sh`), so its tagged tree says `0.10.0`. Its release archives are built by cargo, for three systems: no `x86_64-darwin`.
  - Each tool's herdr plugin is `plugins/herdr` in its tree: Flume's manifest takes Flume's version, and sluice's keeps its own.
  - No release of the family carries an attestation.
- **A host's configuration**, afterthought/swancloud, read 2026-11-05 and cited by id (core.41). Its runner's module, `boomdrive-config` check and package are what a working module needs:
  - a launchd agent of the user's login through home-manager, `boomdrive run`, `RunAtLoad`, `KeepAlive`, `ThrottleInterval` 30, `ProcessType` `Interactive` and `AbandonProcessGroup`, so a `flume launch` in flight outlives a restart, with `HOME` and the profiles' `PATH` alone in its environment, and a log file;
  - the file rendered from the host's lists, written at activation as a real file at 0600 (its files.1), and the agent restarted only when the write changed it, since the runner reads its control plane only at start;
  - Flume and sluice installed in the user's profile from their release archives, sluice's relinked and re-signed on each platform, and their herdr plugins from the same tags (packages.6);
  - the runner built from source at a pinned commit (packages.4's last step, chosen by the user under packages.5);
  - a check that the agent's keys, its environment, the file's keys and its writer hold.
- **The platforms.** nix-darwin's `launchd.user.agents` are installed in `system.primaryUser`'s `~/Library/LaunchAgents`, and reloaded at activation when the plist changed. NixOS's switch re-executes each user's manager and runs `system.userActivationScripts`, but restarts no changed user unit.
- **Evaluation in the gate.** Under a sandbox that denies the network and the nix daemon's socket, `nix eval --offline --read-only --store 'local?read-only=true'` evaluates a NixOS system's user unit, computing store paths without writing them. A flake cannot be evaluated so, since its source is copied into the store first.

There are no area pages, so `docs/architecture/core.md` binds every path this change touches.

## The user's to rule on

Three things this design proposes are the user's, at its review:

1. **The set is built from source at release tags** (Decision 3), not taken from release archives as a host's configuration takes them today. It amends core.37's details. A host that takes the set compiles the runner, Flume and sluice at each bump, unless a cache it trusts serves them.
2. **The gate does not build the runner's package** (Decision 12). It builds Flume's and sluice's, and evaluates everything, but the runner's package is first built with nix by the first host after a landing.
3. **The first set is Flume `v0.10.0` and sluice `v0.67.0`**, the newest releases on 2026-11-05, held there only if the runner's tests pass with them (Decision 4).

## Goals / Non-Goals

**Goals:**

- `nix profile install github:agentplot/boomdrive` gives the runner, and a host's configuration gets the runner, Flume and sluice at versions the runner's tests passed with.
- A host's configuration that imports a module and sets `services.boomdrive` gets the runner's file, its service, its restarts and its tools, and carries none of its own.
- Every option is the runner's file in Nix, under the file's own names.
- Nothing secret is ever in the store.

**Non-Goals:**

- **herdr and git.** They are the host's own (record 0012). The modules neither install nor check them. The runner refuses to start below its git floor, and says so in its log.
- **Linking the herdr plugins into herdr.** The set packages them. Linking one is `herdr plugin link`, herdr's own command, which the host's configuration or a person runs.
- **sluice's driver, herdr's server, Flume's settle loop and Flume's configuration.** They are the host's service manager's and Flume's (core.13). The modules start and configure only the runner.
- **Enrollment and the managed level.** A partition is enrolled by hand on the host. The managed level of record 0009 is unbuilt, and stays the control plane's to write.
- **A binary cache, a release workflow and attestations.** Record 0012's release workflow is its own change.
- **home-manager.** A home-manager module is a later addition, should a host want one.

## Decisions

### 1. Where it lives, and its project

`flake.nix` and `flake.lock` are at the root, so `github:agentplot/boomdrive` is the flake. Everything else is under `nix/`:

```
nix/moon.yml
nix/inputs.nix              the lock's inputs, located or fetched, for the shell and the tests
nix/packages/boomdrive.nix  the runner
nix/packages/flume.nix      Flume, until Flume publishes a flake
nix/packages/sluice.nix     sluice's flake's package, given its tag's version
nix/packages/herdr-plugin.nix
nix/modules/options.nix     services.boomdrive, shared by both modules
nix/modules/toml.nix        the file's text
nix/modules/darwin.nix
nix/modules/nixos.nix
nix/modules/activate.sh     the link, and NixOS's start and restart
nix/check.sh                the project's type check and test
nix/tests/                  the evaluated tests, the rendered files and the activation's tests
```

The moon project is `nix`, at `nix/`, `language: 'nix'`, listed in `.moon/workspace.yml`. Its four tasks run from the workspace root, in the fast layer: `format`, `lint`, `typecheck` and `test` (Decision 12).

Alternatives:

- **The flake under `nix/`**, as `github:agentplot/boomdrive?dir=nix`. Not taken: every reference to the flake would carry the query.
- **The flake's checks in the root project.** Not taken: core.7 and CLAUDE.md give each part with its own tooling a project of its own, one task per check.

### 2. What the set holds

`packages.<system>`, for the four systems sluice's flake builds:

| package | what | from |
|---|---|---|
| `boomdrive`, `default` | the runner | this tree |
| `flume` | `flume` | Flume's tag, by this flake's recipe |
| `sluice` | `sluice` and `wt-submit` | sluice's tag, by sluice's flake |
| `flume-herdr-plugin` | Flume's `plugins/herdr` | the `flume` package's source |
| `sluice-herdr-plugin` | sluice's `plugins/herdr` | the `sluice` package's source |

These are the host's tools of record 0009's amendment: the runner, Flume, sluice and herdr's plugins of theirs. herdr and git are the host's own (record 0012). timber, weir and boom are not in it: the runner and Flume run none of them on a host. weir is in the runner's `Cargo.lock` as a library.

### 3. How each tool is built

Each tool is built on the host from source at a release tag, the tag locked by its hash in `flake.lock`. Nothing prebuilt runs (core.44).

- **The runner** is built from this tree, as sluice's flake builds sluice. That means `rustPlatform` from rust-overlay at the Rust `.prototools` names, `cargoBuildFlags = [ "-p" "boomdrive" ]`, and the check phase off, since the gate runs its tests. Its source is `Cargo.toml`, `Cargo.lock` and `crates/` alone, by `lib.fileset`. Its crates are fetched by `Cargo.lock`'s own checksums. Its git crates, weir's, are fetched by their locked revision under `outputHashes`, the one hash kept here (Task notes 2.1). Its version is `Cargo.toml`'s.
- **sluice** is sluice's flake's `packages.default`, with `nixpkgs` and `rust-overlay` following this flake's. Its tagged tree says `0.0.0`, so the package is given its tag's version as sluice's release build is: `version` and `env.SLUICE_VERSION`. Where sluice's tree carries a version other than `0.0.0`, that is read instead, and nothing is given. The tag is read from `flake.nix`'s own input URL, so one string names it.
- **Flume** publishes no flake. A recipe here builds its tag the way sluice's flake builds sluice: rust-overlay at the Rust its own `.prototools` names, `-p flume --bin flume`, `Cargo.lock`'s checksums. Its version is its tagged tree's `Cargo.toml`. The recipe is a bridge, and goes when Flume publishes a flake (What this change asks of others).
- **The herdr plugins** are `plugins/herdr` of each tool's package's source, copied whole, with the tool's version, so a plugin and its tool are one release (as a host's packages.6 has it). Nothing in them is rewritten. A plugin's scripts find the tool on the user's `PATH`, where the module puts it (Decision 9).

Alternatives:

- **Each tool's release archive, by the digest GitHub publishes**, as a host's configuration takes Flume and sluice. Nothing compiles on a host but the runner, which has no release. Not taken: it re-packages each tool here, sluice's binaries relinked and re-signed on each platform because they name the Nix store of sluice's build. It has no `x86_64-darwin`. And no release carries an attestation, so core.37 is met no better than by a source build. Once the release workflow attests releases, a host that wants no compiler can be offered them beside this.
- **Each tool's flake taken whole.** Not taken: sluice's says `0.0.0`, below any floor the runner names (core.38), and Flume has none.
- **One recipe here for every tool.** Not taken: each tool's flake is the one owner of its build, which the family cites rather than copies.

### 4. How a set is proven

A set is the one the runner's tests pass with, running the set's own Flume and sluice (core.45).

- **The shell builds them.** `devenv.nix` reads `flake.lock` through `nix/inputs.nix`, evaluates the flake's outputs with those inputs, and gives the runner's tests the set's programs by path: `BOOMDRIVE_SET_FLUME` and `BOOMDRIVE_SET_SLUICE`. They are never put on `PATH`, so a shell's `sluice exec` still runs the host's sluice. `devenv print-dev-env` builds them, outside the sandbox, once per host for each set.
- **The tests run them.** `crates/boomdrive/tests/family.rs`, in `boomdrive:test`, on the tests' scratch host:
  - each program's `--version` is its tag's, the tag read from `flake.lock`;
  - the set's Flume's `--json status` of a scratch host with no place is read by `flume::Cli` and the runner's status readers, as the work-into-flume spec reads it;
  - the set's sluice's `status <id> --json` for an id it has no record of, in a scratch repository, is read by `sluice::Cli` as a missing entry.
  - Where either variable is unset, the test fails, saying the devenv shell gives it. It never skips, and never looks on `PATH`.
- **A set moves by one change.** It moves a tag in `flake.nix` and the lock together, and lands through the gate. `nix:typecheck` fails where a tag and the lock differ.
- **The fixtures stay.** Flume 0.4.1's and sluice 0.61.1's answers stay the unit fixtures of the runner's readers. They are what those versions printed. The family test reads the set's own.

The first set is Flume `v0.10.0` and sluice `v0.67.0`, the newest releases on 2026-11-05. Where the family test finds an answer of theirs the runner cannot read, the set takes the newest release of that tool the test passes with, and the build names what the newer one printed. The runner's readers do not change in this change.

Alternatives:

- **Fixtures of each set's answers alone.** Not taken: they prove what a version printed when someone captured it, by hand, and nothing makes a bump re-capture them.
- **Flume and sluice on the shell's `PATH`.** Not taken: an agent's `sluice exec` in the devenv shell would run the set's sluice, not the host's driver's.

### 5. How a host overrides a tool

- **A tool's version** moves by its input: `--override-input flume github:agentplot/flume/v0.10.1`, or `inputs.boomdrive.inputs.sluice.follows`. Flume's version follows its tree. sluice's follows its tree too once that carries its version. Until then the package's version must be given with the input (README).
- **A package** is replaced by the modules' `package` and `tools` (Decision 9).

An override is the host's. It is not proven here, and the runner's own floors judge it (core.38).

### 6. Two modules, one set of options

`darwinModules.default` and `nixosModules.default`, also named `boomdrive`, each import `nix/modules/options.nix`:

| option | type | default | |
|---|---|---|---|
| `services.boomdrive.enable` | bool | false | |
| `services.boomdrive.package` | package | the set's `boomdrive` | the runner |
| `services.boomdrive.tools` | list of packages | the set's `flume` and `sluice` | installed beside it |
| `services.boomdrive.settings` | the file | `{ }` | Decision 7 |
| `services.boomdrive.user` | string | none | NixOS: the service's user |
| `services.boomdrive.logFile` | path | `<home>/Library/Logs/boomdrive.log` | nix-darwin: the agent's log |

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

### 7. The file

- **Rendered in Nix.** `nix/modules/toml.nix` renders `settings` at evaluation: strings, booleans, integers, lists of strings, tables and lists of tables. It refuses any other value, naming its path. Top-level keys come before tables, each sorted by name. The first line is `# Written by the boomdrive flake's module from services.boomdrive.settings; a change goes there.` So the gate compares the text without a build, and one `settings` gives one text on both platforms.
- **In the store.** The text is `pkgs.writeText "boomdrive-config.toml"`. The service runs `boomdrive run --config <that file>`, so the service reads exactly what the module wrote, and a changed file changes the service (Decision 8).
- **Linked at the runner's own path.** The activation links the user's `~/.config/boomdrive/config.toml` to the same file, so `boomdrive enroll <label>` and `boomdrive run --once` run by hand read it. A link there is replaced. A file or folder that is no link is moved once to `config.toml.before-nix`, and the activation names both paths. A shell with another `XDG_CONFIG_HOME` passes `--config`.
- **The host's level.** It is the level nix writes under record 0009. Once a control plane writes a managed level, that level is its own file.

Alternatives:

- **A real file written at activation, at 0600**, as a host's configuration writes it today (its files.1). Not taken: it needs a writer of its own on each platform, and a restart that compares what it wrote. The file holds no secret (Decision 10), so a link into the store, readable by every user, hides nothing.
- **No link, and `--config` on every command run by hand.** Not taken: `boomdrive enroll` would read another file, or none.
- **`pkgs.formats.toml`.** Not taken: its text is a build's output, which the gate cannot read without building.

### 8. The service and its restarts

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
- `Environment` `PATH` alone; the user's manager gives `HOME` and `XDG_RUNTIME_DIR`;
- `users.users.<user>.linger = true`, so the runner runs from boot on a host no one logs into.

NixOS restarts no changed user unit. So `system.userActivationScripts.boomdrive` runs `activate.sh keep` for the user, after linking. That compares the running runner's command line, by `ps`, with the unit's, and starts the unit where it is not running, restarts it where they differ, and does nothing where they are one. It is `systemctl --user --no-block`, since it runs inside the user's `nixos-activation.service`. Any other user's activation does nothing. Evaluation fails where the user is not in `users.users`, or where `settings` names a hosted control plane on a systemd older than 256 (record 0018).

The runner reads its file again at every pass, so most changes would take effect without a restart. A restart on every change is still taken, since `[control_plane]` is read only at start and the restart is what a host would do by hand.

Alternatives:

- **home-manager's `launchd.agents` and `systemd.user.services`**, as a host's configuration does it today. Not taken: every host that imports the module would need home-manager. A home-manager module is a later addition.
- **A NixOS system service with `User=`**, which the switch restarts on its own. Not taken: records 0012 and 0018 have a user unit, which has the user's runtime directory, and `systemd-creds --user` is the user's.
- **A restart only when `[control_plane]` changes.** Not taken: it needs the module to know which keys the runner reads at start.

### 9. Its tools on `PATH`

Each module puts `package` and `tools` in `environment.systemPackages`. `tools` defaults to the set's `flume` and `sluice`. A host that installs its own sets `tools = [ ]`, or its own packages.

The service's `PATH` is the profiles, never a package's own folder, so the runner, the agents Flume launches and a person's shell run one copy of each tool:

- nix-darwin: `/etc/profiles/per-user/<user>/bin`, `<home>/.nix-profile/bin`, `<home>/.local/state/nix/profile/bin`, `/run/current-system/sw/bin`, `/nix/var/nix/profiles/default/bin`, `/usr/local/bin`, `/usr/bin`, `/bin`, `/usr/sbin`, `/sbin`;
- NixOS: `/etc/profiles/per-user/<user>/bin`, `<home>/.nix-profile/bin`, `<home>/.local/state/nix/profile/bin`, `/run/wrappers/bin`, `/run/current-system/sw/bin`, `/nix/var/nix/profiles/default/bin`.

git and herdr are found there, from wherever the host installs them. A Mac whose only git is the system's, older than 2.48, gets the runner's refusal in its log. The README says so.

Alternatives:

- **The set's tools first on the service's `PATH`, from their store paths.** Not taken: the runner's sluice and the host's sluice driver could be two versions on one clone.
- **The user's profile, `users.users.<user>.packages`.** Not taken: nix-darwin and NixOS both have the system profile, and a host with home-manager may manage the user's.

### 10. Credentials and enrollment

- **No option holds a secret.** There is no token, key or enrollment option.
- **A key named for one fails evaluation.** Any key of `settings`, at any depth, named `token`, `key`, `secret` or `password`, or ending in `_token`, `_key`, `_secret` or `_password`, fails evaluation. The failure names its path, and says a partition's key is made on the host by `boomdrive enroll`. The runner would refuse such a file too, but only once the store holds it, readable by every user (core.4).
- **Enrollment is by hand.** After a switch, a person runs `boomdrive enroll <label>` on the host, the token on standard input. The key goes to the login keychain, or to `systemd-creds` of the user (core.32, record 0018). The agent runs in the user's login, where the keychain is. The unit runs under the user's manager, where `systemd-creds --user` decrypts.
- **The forge.** The host's git credential helper answers the runner's fetches, as it does today. The modules name none.

### 11. What stays the host's

The host's configuration decides which hosts set `enable`, which user, which repositories, which control plane and partitions, its git and herdr and its plugins' linking, its secrets and enrollments, its Flume configuration, its sluice driver, and its doctor or monitoring. The modules decide only how the runner is installed, configured from `settings`, kept running and restarted.

### 12. The gate

The `nix` project's tasks run in the gate's sandbox, with no network and no nix daemon. They evaluate and never build.

- **The inputs are there first.** `devenv.nix` reads `flake.lock` through `nix/inputs.nix`, which takes each locked input by `builtins.fetchTree`. That fetches them during `devenv print-dev-env`, outside the sandbox. It gives the tasks their paths as `BOOMDRIVE_FLAKE_INPUTS`, a JSON object of input name and store path.
- **The tasks read the store, read-only.** `nix/check.sh` runs the pinned nix as `nix eval --offline --read-only --impure --store 'local?read-only=true' --extra-experimental-features 'nix-command read-only-local-store'`, with nix's caches under the task's scratch folder. It evaluates `nix/tests`, which calls `flake.nix`'s `outputs` with those inputs and `self` as this tree, as flake-compat does. Every path is computed, never written.
- **`nix:format`**: `nixfmt --check` on `flake.nix` and `nix/`.
- **`nix:lint`**: `statix check` and `deadnix --fail`, on the same files.
- **`nix:typecheck`**: every package of the four systems evaluates to a derivation, each module evaluates in a minimal system of its platform, and each input's tag in `flake.nix` is the lock's.
- **`nix:test`**: the evaluated tests of the two specs, the rendered files compared byte for byte with `nix/tests/files`, and `activate.sh`'s cases run by `sh` against stub `systemctl` and `ps` in a scratch folder.
- **`boomdrive:test`** gains `family.rs` (Decision 4) and `module_files.rs`, which reads each file under `nix/tests/files` with `config::read`.
- **What is built.** The set's Flume and sluice are built by the shell, and run by `family.rs`. The runner's package is evaluated, and its crates are the ones `boomdrive:test` compiles, but no gate task builds it with nix. The first host to build it after a landing is the first to do so. The release workflow of record 0012 builds the flake on each system before it publishes.
- **Pins.** nix, nixfmt and statix are `devenv.nix`'s, asserted to their versions as git is (core.8), and `check-toolchain.sh` holds them. `treefmt.nix` excludes `nix/tests/files/**`, which the tests compare byte for byte, as it excludes the tools' fixtures.

Alternatives:

- **`nix flake check` in the gate.** Not taken: the sandbox has no daemon, and a flake's evaluation copies its source into the store.
- **The executor opened for nix.** Not taken: it would open the network to every check. sluice's executor has no allowance for the daemon's socket alone.
- **The runner's package built by the toolchain step at every gate run.** Not taken here: every run would compile the runner twice, once by cargo and once by nix. It is the user's to weigh (The user's to rule on, 2).

### Rules relied on

- **core.4, core.32**: no credential in an option, the file or the store; the key stays in the host's credential store (Decision 10).
- **core.5**: the module writes the repositories the host names, and no stream, kind or drive.
- **core.7, core.8**: the flake is a project of its own, one task per check; nix, nixfmt and statix are pinned and checked; the rendered files are excluded by name with their reason.
- **core.12**: each option under `settings` is its key's name in the file; `logFile` names a file.
- **core.13**: the modules start the runner alone.
- **core.17**: the file is the host's level, written by nix, which the runner reads as it reads one written by hand.
- **core.18**: the modules write only the runner's own file.
- **core.38**: an override is judged by the runner's floors.
- **core.41**: tests and examples use neutral names: `alice`, `bob`, `host-a`, `home`, `work`, `example/state`, `control.example`.
- **core.44, core.45, core.46**, which this change adds, and **core.37** as it amends it.

### Rules departed from

None. core.37's detail on nix flakes is amended by record 0020: what it rules out is a release's binaries that the release workflow did not attest, and a package built on the host from a locked source is none.

## What this change asks of others

Each is told to the orchestrator, not done here.

- **Flume**: publish a flake whose `packages.default` builds `flume` at its tag, as sluice's and timber's do. This flake then takes it from there, and `nix/packages/flume.nix` goes.
- **sluice and timber**: carry each release's version in the tagged tree, as Flume's release commit does, so a pure build of a tag reports it. This flake then stops giving sluice its version.
- **A host's configuration**, afterthought/swancloud, and its change `views-are-retired-once-the-runners-workspaces-stand`:
  - import `darwinModules.default` and set `services.boomdrive` with the options of Decision 6, `settings.code_workspaces` and `settings.repositories` among them, in place of its runner's module and package;
  - set `tools = [ ]` while it installs its own Flume and sluice;
  - point its check at the module's options, its doctor at the agent `org.nixos.boomdrive` and at `logFile`;
  - and read its files.1 against a file linked into the store, which holds no secret.
- **Record 0012's release workflow**, when built: build the flake on each system before it publishes.

## Risks / Trade-offs

- **Hosts compile.** A host that takes the set compiles three Rust tools at each bump. A cache is the host's, or a later public one.
- **The runner's package is not built in the gate.** A change that breaks only its nix build, a stale weir hash say, is found by the first host that builds it, whose switch fails and keeps its running generation.
- **sluice's version is given here** until its tree carries it. A host that overrides sluice's input must give its version too.
- **The first set may stop short of the newest release** where the runner cannot read a newer answer. The build names what the newer one printed, for a change to the runner's readers.
- **A person's own file is moved aside.** A host that edited `~/.config/boomdrive/config.toml` by hand finds it at `config.toml.before-nix`.
- **nix-darwin installs user agents for the primary user alone.** A Mac whose runner is another user's needs home-manager, or a later module.
- **The pinned nix reads the host's store.** Its store format must be one the host's nix daemon wrote. The type check fails where it is not.

## Migration Plan

Nothing changes on a host until its configuration imports a module. A host moving from its own module:

1. disables its own runner's agent and file writer in the same switch that enables `services.boomdrive`;
2. finds its file moved to `config.toml.before-nix`, where it was a real file;
3. finds the runner under nix-darwin's label `org.nixos.boomdrive`, its data directory, clones and views as they were, since the data directory follows `HOME` and not the service.

Two runners never run at once: the runner's own lock on its data directory refuses the second with exit 2.

## After deploy

Nothing. No step of this change is outside the gate.

## Task notes

**1.1** `flake.nix`:

```nix
inputs = {
  nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  rust-overlay = { url = "github:oxalica/rust-overlay"; inputs.nixpkgs.follows = "nixpkgs"; };
  sluice = {
    url = "github:agentplot/sluice/v0.67.0";
    inputs.nixpkgs.follows = "nixpkgs";
    inputs.rust-overlay.follows = "rust-overlay";
  };
  flume = { url = "github:agentplot/flume/v0.10.0"; flake = false; };
  nix-darwin = { url = "github:nix-darwin/nix-darwin"; inputs.nixpkgs.follows = "nixpkgs"; };
};
```

`nix-darwin` is the tests'. A host's configuration follows its own. The outputs are `packages`, `darwinModules` and `nixosModules`. `flake.lock` is made by `nix flake lock`, outside the sandbox.

**1.2** `nix/inputs.nix` takes `flake.lock` and, for each node the root names, gives `builtins.fetchTree` of its `locked` attributes, following `follows`. In `devenv.nix` that fetches them. In `nix/check.sh` they are in the store already, and the read-only store finds them. `devenv.nix` sets `env.BOOMDRIVE_FLAKE_INPUTS` from it.

**1.3** `devenv.nix` gains `pkgs.nix`, `pkgs.nixfmt` and `pkgs.statix`, each asserted to the version it pins as git is. `check-toolchain.sh` checks each on `PATH` is that version. `nix:format`, `nix:lint`, `nix:typecheck` and `nix:test` declare as inputs `/flake.nix`, `/flake.lock`, `/nix/**/*`, `/devenv.nix` and `/devenv.lock`. `nix:typecheck` and `nix:test` also declare `/Cargo.toml`, `/Cargo.lock`, `/.prototools` and `/crates/**/*`, since the runner's package is computed from them.

**1.4** `nix/check.sh typecheck` and `nix/check.sh test`. Each runs the pinned nix as Decision 12 says, with `XDG_CACHE_HOME` under the task's scratch folder, so nix writes nothing in the home. It reads `flake.nix`'s own `inputs` by importing the file, as an attribute set. It prints each failure the evaluation returns as one line, `nix: <test>: <what>`, and exits 1 where there is one. A test returns a list of failures, empty when it holds, never an assertion that stops the rest.

**2.1** `nix/packages/boomdrive.nix`: `cargoLock = { lockFile = ../../Cargo.lock; outputHashes = { ... }; }`, one entry for weir's git crates. A test holds the entries' names to `Cargo.lock`'s git packages, so a weir bump that drops or renames one fails `nix:test`. `meta.mainProgram = "boomdrive"`. Platforms are the four.

**2.2** `nix/packages/flume.nix` takes the source and builds as sluice's flake does, reading Rust's version from the source's `.prototools` and the package's from its `Cargo.toml`. `cargoBuildFlags = [ "-p" "flume" "--bin" "flume" ]`, `doCheck = false`.

**2.3** `nix/packages/sluice.nix` takes sluice's package and its source. Where the source's `Cargo.toml` workspace version is `0.0.0`, it gives `overrideAttrs` `version` and `env.SLUICE_VERSION`, from the tag in `flake.nix`'s sluice URL without its `v`. Otherwise it gives the package unchanged.

**2.4** `nix/packages/herdr-plugin.nix` takes a tool's package and copies `${source}/plugins/herdr` to `$out`, unchanged, `version` the tool's.

**2.5** The tag check reads each input's URL from `flake.nix`'s `inputs` and its `original.ref` from `flake.lock`, and fails naming both where they differ.

**3.1** `devenv.nix` sets `env.BOOMDRIVE_SET_FLUME = "${flume}/bin/flume"` and `env.BOOMDRIVE_SET_SLUICE = "${sluice}/bin/sluice"`, from the flake's own `packages` for the shell's system, evaluated with `nix/inputs.nix`'s inputs. It puts neither on `PATH`.

**3.2** `crates/boomdrive/tests/family.rs` uses `tests/common`'s scratch host. It reads `flake.lock` with `serde_json` for `nodes.flume.original.ref` and `nodes.sluice.original.ref`. Flume's scratch host is as the 0.4.1 fixtures' README describes, with no place. sluice's scratch repository has a policy and an `origin` in the same scratch folder. Each answer is read through `flume::Cli` and `sluice::Cli`. `crates/boomdrive/moon.yml`'s `test` gains `/flake.lock` and `/nix/tests/files/**/*` as inputs.

**4.1** `nix/modules/toml.nix`: `render settings -> string`. Strings are TOML basic strings, with `"`, `\` and control characters escaped. Booleans, integers, lists of strings, tables as `[a.b]`, and lists of tables as `[[a.b]]`. A key that is not bare is quoted. Any other value fails, naming its path. `nix/tests/files/empty.toml`, `crew.toml` and `hosted.toml` are the spec's three settings, written once by the renderer and read here as the expected text.

**4.2** `nix/modules/options.nix` declares the options of Decision 6, and holds the credential check of Decision 10 as an assertion whose message names every path found. `crates/boomdrive/tests/module_files.rs` reads each file under `nix/tests/files` with `config::read`, against a scratch home, and checks each repository and the control plane's type.

**5.1** `nix/modules/darwin.nix` evaluated by the tests in a nix-darwin system of `aarch64-darwin` with `system.primaryUser = "alice"`, `users.users.alice.home = "/Users/alice"` and `system.stateVersion` set. The tests read `launchd.user.agents.boomdrive.serviceConfig` key by key, and `environment.systemPackages`.

**5.2** `nix/modules/activate.sh link <file>` makes the link of Decision 7 under `$HOME`, saying a line only where it moved or linked something. `nix/modules/activate.sh keep <user> <program> <argument>...` does Decision 8's NixOS step. Each is shellcheck'd and laid out by treefmt. Its tests run each case in a scratch `HOME`, with `systemctl` and `ps` stubs first on `PATH` that record their calls.

**6.1** `nix/modules/nixos.nix` evaluated by the tests in a NixOS system of `x86_64-linux` with `users.users.alice` and `users.users.bob` normal users, `services.boomdrive.user = "alice"`, and `boot.loader.grub.enable = false`, `fileSystems."/"` and `system.stateVersion` set. The tests read `systemd.user.services.boomdrive` and `systemd.user.units."boomdrive.service".text`, `users.users.alice.linger`, `system.userActivationScripts.boomdrive` and `environment.systemPackages`. The old systemd is `systemd.package` overridden to report 255.

**7.1** README gains **Installing with nix**, after **Running on a host**: the profile install; a host's configuration importing a module, with Decision 6's example; `tools = [ ]`; enrolling after the switch; git 2.48 on a Mac; linking a herdr plugin with herdr's command; and overriding a tool's input, with sluice's version given beside it. `tests/readme.rs` still reads the README's TOML blocks.

**7.2** `CLAUDE.md` lists the `nix` project beside the crates. `openspec/config.yaml`'s context names the flake in its stack.
