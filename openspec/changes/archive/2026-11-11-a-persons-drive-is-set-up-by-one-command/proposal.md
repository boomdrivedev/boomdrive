# Proposal

## Why

The brief: "A person who has Boomdrive's programs from Nix, on a Mac Nix does not manage, runs one setup command, which an agent can run for them, that takes their account, drive, repositories and crew state once and leaves the runner running for their login and crew's agents ready to talk to, setting each role through crew drive set's --runs <role> standing|on-demand and the batches through --review <after_changes> <after_hours>, never --flume; adding or removing a repository later is setup's too."

Record 0009's amendment of 2026-11-09 decides it. The route and its timing are the user's, on their card: "No: Nix installs the programs, a setup command does the rest", "With Boomdrive going public". Before it, of colleagues on their own Macs: "They'd have to set up their own crew state." and "How do we assist them in that setup process?"

Today a person with the set's programs in their Nix profile has no way to a working drive but writing four tools' files by hand. The runner reads `~/.config/boomdrive/config.toml`, which only Nix's module writes. crew reads `~/.config/crew/teams.json`, which crew's `crew drive set` now writes on a host Nix does not configure (crew record 0065), once a state repository exists on GitHub. The runner needs a launchd agent of the login, which only nix-darwin declares. herdr needs Flume's and sluice's plugins linked, which only the modules' activation does. core.16 has named `setup` among the runner's own commands since record 0008, and nothing builds it.

## What Changes

- **`boomdrive setup`, one command.** It sets up one drive of one account: the drive's repositories, by their addresses, and its account's crew state repository. Each question is a flag, and a terminal is asked what no flag gives. It answers in JSON with `--json` (core.39). Run again with the same answers, it changes nothing.
- **What it leaves on the host.**
  - The account's state repository on GitHub, made private by `gh` where none exists.
  - crew's file naming the drive and its state, by `crew drive set`. setup passes each role's `--runs` and the drive's `--review`, and never `--flume`. It also passes the Mac's own values: the session named for the drive, the repository root as its folder, the account for each role's Claude account, this host, and the runner's clone and view of each repository (core.57).
  - The runner's managed layer, `config.toml`, written by the keys Nix's module writes for the same account. A person's own file there is moved aside once and named.
  - Flume's and sluice's herdr plugins, linked by herdr's own `plugin link` from the person's Nix profile.
  - The runner as a launchd agent of the login, as nix-darwin's agent runs it.
- **Later changes are setup's too.** `--add-repository` and `--remove-repository` change the drive's repositories. setup takes every answer not given from what the host holds, and changes only what they change.
- **What setup checks and leaves to the person.** It checks Claude Code's, GitHub's and Pending You's sign-ins. It refuses before writing anything where GitHub's is missing, since the state repository needs it, and names any other that is missing. It names `crew main up <drive>` as how the person starts crew's agents, and starts none (core.13).
- **On a host Nix manages**, where the runner's file is a link, setup writes nothing and says so, naming `services.boomdrive.accounts`.
- **The set gives its herdr plugins as one package a profile holds.** The two plugin packages clash in a profile, since each has `bin/open.sh` at its top. A new package, `herdr-plugins`, holds each at `share/herdr/plugins/<tool>`. README's install line adds it and herdr.
- **The control-plane interface gains a drive's setup.** It is reached through `planes`, as `enroll` is. crew's implementation carries it: the state repository, `crew drive set`, its table in the runner's file, and the command that starts its agents (core.2).
- **Record 0009 gains its amendment of 2026-11-11**, the writing agent's choices for how setup is built. core.5's, core.16's, core.18's, core.28's and core.29's details are amended with it, and core.29's text gains setup's link of the set's herdr plugins.

## Capabilities

### New Capabilities

- `setup`: `boomdrive setup`, which sets up a person's drive on a Mac Nix does not manage, and changes its repositories later.

### Modified Capabilities

- `family-packages`: the flake gives `herdr-plugins`, the set's herdr plugins in one package a profile holds.

## Impact

- `crates/boomdrive`:
  - `src/setup.rs`, new: the answers and their checks, the host's checks and sign-ins, the runner's file, the plugins, the agent, and the answer;
  - `src/main.rs`: the `setup` command and its flags;
  - `src/planes.rs`: an implementation's setup of a drive, beside its enrollment;
  - `src/lib.rs`: the module;
  - `tests/setup.rs`, new: setup through the binary, over stand-in programs;
  - `tests/module_files.rs`: the rendered file setup's file is held to;
  - `tests/readme.rs`: README names setup.
- `crates/boomdrive-crew`:
  - `src/setup.rs`, new: the state repository, `crew drive set`'s arguments and answer, the drive's answers as crew's file holds them, and crew's table;
  - `src/lib.rs`: the module;
  - `tests/setup.rs`, new.
- `flake.nix` and `nix/packages/herdr-plugins.nix`, new: the `herdr-plugins` package.
- `nix/tests/settings.nix`, `nix/tests/file.nix` and `nix/tests/files/setup.toml`, new: the file Nix's module renders for one account's drive under crew, which setup's file is held to. `nix/tests/typecheck.nix` and `nix/tests/packages.nix`: the new package in the set, and what it holds.
- `README.md`: setting up a drive, and the install line.
- `docs/adr/0009-…` and `docs/architecture/core.md`: record 0009's amendment of 2026-11-11, and core.5, core.16, core.18, core.28 and core.29, written with this change.
- No configuration key, field of `report.json`, protocol message, tool, tracked file type or crate is added. The runner's `run` and `enroll` are unchanged.
- Outside this repository:
  - setup runs `crew drive set` with crew v0.59.0's flags. The set pins crew v0.50.2, which has no `drive set`. The bolt `the-sets-crew-pin-takes-its-new-names` moves the set to v0.59.0 (design Risk 1).
  - Flume with no file of its own runs no multiplexer, and setup writes none of Flume's files (core.18). Design Risk 2 says what that leaves, and record 0009's amendment names what Flume is asked for.

## Touches

```
crates/boomdrive/src/setup.rs
crates/boomdrive/src/main.rs
crates/boomdrive/src/planes.rs
crates/boomdrive/src/lib.rs
crates/boomdrive/tests/setup.rs
crates/boomdrive/tests/module_files.rs
crates/boomdrive/tests/readme.rs
crates/boomdrive-crew/src/setup.rs
crates/boomdrive-crew/src/lib.rs
crates/boomdrive-crew/tests/setup.rs
flake.nix
nix/packages/herdr-plugins.nix
nix/tests/settings.nix
nix/tests/file.nix
nix/tests/files/setup.toml
nix/tests/packages.nix
nix/tests/typecheck.nix
README.md
docs/adr/0009-a-host-is-set-up-through-each-tools-own-commands-and-its-repositories-follow-its-drives.md
docs/architecture/core.md
openspec/specs/setup/spec.md
openspec/specs/family-packages/spec.md
```
