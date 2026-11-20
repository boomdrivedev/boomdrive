# Verify: boomdrives-tools-install-on-a-machine-without-the-runner

The work holds, and the change is archived. The gate passes on `b23b052`. All twelve tasks are ticked. Every scenario of the delta spec is a case of `nix/tests/apps.nix` or `nix/tests/tools.nix`, or of `nix/tests/activate.sh`, and each held in the gate.

This report is on `change/boomdrives-tools-install-on-a-machine-without-the-runner` at `b23b052`. The branch was cut from `bolt/every-machine-takes-boomdrives-tools` at `2ea1a481`. Nix is 2.34.8.

## The whole checks

`devenv shell -- sluice exec main` ran once on `b23b052`. It ran the checks that commit affects against `main` (`59ed1e9`) and exited 0. All 8 passed, and sluice recorded their results under their keys for `b23b052`:

- `nix`: `format`, `lint`, `typecheck` ("nix: typecheck: every case holds") and `test` ("nix: test: every case holds");
- `root`: `format` ("formatted 427 files (0 changed)"), `specs` ("27 passed, 0 failed") and `changes` ("1 open, each valid under openspec validate --strict and archiving to specs that are");
- `boomdrive`: `test`, 727 tests, each `ok`, none failed or ignored.

moon gave two kinds of warning, and no check failed because of either. It could not write its output archives under `.moon/cache/outputs`, eight times. It also skipped the Rust dependency install for want of a network. Both come from the sandbox.

`root:changes` archived the open change into a scratch copy of the specs and held them to `openspec validate --strict` (core.61). That is task 7.1's check. `root:specs` is task 6.2's.

The branch is behind `main`'s tip, `59ed1e9`, by the landings #273 to #279. `git merge-tree` merges the two with no conflict.

## Tasks

All twelve tasks are ticked, 1.1 to 7.1, and none is open:

- Tasks 1.1 to 5.1 each name cases of `nix:test`, which held: "every case holds".
- Task 6.1's three greps of `README.md`: `services.boomdrive.pendingyou.enable` prints one line, `without the runner` four, and `runner or without crew, none of them is installed` none. The README also names each of the eight switches, and `services.boomdrive.user` for NixOS.
- Task 6.2's grep of `a switch of its own` prints one line in `CLAUDE.md` and one in `openspec/config.yaml`. `root:specs` passed.
- Task 7.1's check is `root:changes`, above.

The diff from `2ea1a481` stays inside the paths proposal.md's Touches lists, and the change's own directory. It adds no crate, gate tool or tracked file type, and changes no crate, package, lock, workflow, boot check or shell. Record 0023's and record 0044's amendments of 2026-11-18, core.63, and core.18's and core.46's details are in the change's first commit, `bb835cf7`. The code and tests cite them and change none of them.

## The delta, requirement by requirement

Each case runs on nix-darwin and on NixOS, as `<scenario> on nix-darwin` and `<scenario> on NixOS`, unless the table names one platform.

### Without the runner, the modules install only the tools the host turns on

| Scenario | Proven by |
|---|---|
| Nothing turned on | `Nothing turned on`: with `system.primaryUser` forced to `null` on nix-darwin and no `services.boomdrive.user` on NixOS, the system fails no assertion and its profile holds none of the set's thirteen packages and no `pendingyou-signin`. The nix-darwin activation runs no `sudo --user`, and NixOS declares no user activation script of the set's |
| Only the tools turned on | `Only the tools turned on`: the profile holds the set's `flume` and `herdr` alone, and the plugin step links `flume-herdr-plugin` alone |
| Every tool turned on without the runner | `Every tool turned on without the runner`: the profile holds each of the twelve tools by the set's `outPath`, and no `boomdrive` |

### herdr's plugins by default are those of the tools the set installs

| Scenario | Proven by |
|---|---|
| The runner's plugins | `The runner's plugins` |
| A host's own Flume | `A host's own Flume` |
| Flume's module off | `Flume's module off` |
| The plugins of the tools turned on without the runner | `The plugins of the tools turned on without the runner`: `sluice-herdr-plugin`, then `plannotator-herdr-plugin` |
| A host's own list | `A host's own list`: the one step links the set's `timber-herdr-plugin` with the set's herdr, as `alice` on nix-darwin and in the user activation on NixOS |

`apps.nix` reads "the set's" as the tool's module or switch on and its package `==` the set's derivation, which Nix decides by `outPath` (design Task notes 3.1). The `defaultText` gives the same five conditions.

### The set's user links the plugins and writes Pending You's folder, with the runner or without

| Scenario | Proven by |
|---|---|
| The plugins linked without the runner on nix-darwin | `The plugins linked without the runner on nix-darwin`: the one step runs `sudo --user=alice -- /usr/bin/env HOME=/Users/alice`, the activation names no `boomdrive-config.toml`, and no agent `boomdrive` is declared |
| The plugins linked without the runner on NixOS | `The plugins linked without the runner on NixOS`: the step is inside the test of `alice`, no `keep` and no runner's file is in the script, no other user's script links plugins, no unit `boomdrive` is declared, and `alice` does not linger |
| No primary user on nix-darwin | `No primary user on nix-darwin`: the failures name `services.boomdrive.herdr.enable`, through `system.requiresPrimaryUser` |
| No user on NixOS | `No user on NixOS`: the failures name `services.boomdrive.user`. The assertion reads `options.services.boomdrive.user.isDefined` before `users.users`, so the message is the module's |

### The Boomdrive module turns on Flume's, sluice's and timber's modules and leaves crew's to the host

| Scenario | Proven by |
|---|---|
| A host with the runner alone | `A host with the runner alone` (tools.nix) |
| A host that keeps its own Flume file | `A host that keeps its own Flume file` (tools.nix) |
| The tools' user on NixOS | `The tools' user on NixOS` (tools.nix) |
| The tools' user without the runner | `The tools' user without the runner` (tools.nix): Flume's and sluice's `user` are `alice`, and the system fails no assertion |

### The modules install herdr, Ghostty and the launcher, each replaceable or dropped

| Scenario | Proven by |
|---|---|
| The way in with the runner | `The way in with the runner` |
| A host's own herdr | `A host's own herdr` |
| No Ghostty of the set's | `No Ghostty of the set's` |
| A host's own launcher | `A host's own launcher` |
| Not without the runner | `Not without the runner` |
| The way in without the runner | `The way in without the runner` |
| Ghostty turned off with the runner | `Ghostty turned off with the runner` |

### The modules link the set's herdr plugins with herdr's own command

| Scenario | Proven by |
|---|---|
| The plugins linked on nix-darwin | `The plugins linked on nix-darwin` (nix-darwin only) |
| The plugins linked on NixOS | `The plugins linked on NixOS` (NixOS only) |
| The companions' plugins with crew | `The companions' plugins with crew`, one case reading both platforms |
| Each plugin linked | `Each plugin linked`, in `nix/tests/activate.sh`, which this change leaves as it was |
| No herdr of the set's | `No herdr of the set's` |
| herdr turned off | `herdr turned off` |

Both platforms' `linking` now reads `services.boomdrive.herdr.enable`, beside a herdr and a plugin given.

### The modules name herdr and Ghostty Flume's terminal and window

| Scenario | Proven by |
|---|---|
| Flume's terminal and window | `Flume's terminal and window` |
| A host's own window | `A host's own window` |
| The word of before | `The word of before` |
| Flume's terminal without the runner | `Flume's terminal without the runner` |

### The modules install portless, replaceable or dropped

| Scenario | Proven by |
|---|---|
| portless with the runner | `portless with the runner` |
| A host's own portless | `A host's own portless` |
| No portless of the set's | `No portless of the set's` |
| No portless without the runner | `No portless without the runner` |
| portless without the runner | `portless without the runner` |

### The modules install worktrunk, replaceable or dropped

| Scenario | Proven by |
|---|---|
| worktrunk with the runner | `worktrunk with the runner` |
| A host's own worktrunk | `A host's own worktrunk` |
| No worktrunk of the set's | `No worktrunk of the set's` |
| No worktrunk without the runner | `No worktrunk without the runner` |
| worktrunk without the runner | `worktrunk without the runner` |

### The launcher has an entry for each drive of this host

| Scenario | Proven by |
|---|---|
| Each drive of this host in the launcher | `Each drive of this host in the launcher` |
| A drive of another machine has no entry | `A drive of another machine has no entry` |
| No drive, the set's launcher | `No drive, the set's launcher` |
| No entries without the runner | `No entries without the runner` |

The default override passes `entries` only where `services.boomdrive.enable` is on and `[ ]` otherwise, and its `defaultText` says so.

### The modules install crew's companions, each turned on by itself, replaceable or dropped

| Scenario | Proven by |
|---|---|
| The companions with the runner and crew | `The companions with the runner and crew` |
| A host's own Plannotator | `A host's own Plannotator` |
| No Pending You of the set's | `No Pending You of the set's` |
| No companions without crew | `No companions without crew` |
| No companions without the runner | `No companions without the runner` |
| A companion turned on without the runner | `A companion turned on without the runner` |

The RENAMED header matches the requirement the spec held, and `root:changes` archived it cleanly.

### Each activation writes Pending You's folder before it links the plugins

| Scenario | Proven by |
|---|---|
| The folder written on nix-darwin | `The folder written on nix-darwin` (nix-darwin only) |
| The folder written on NixOS | `The folder written on NixOS` (NixOS only) |
| The folder written without the runner | `The folder written without the runner on nix-darwin` and `The folder written without the runner on NixOS`: the one folder step runs as `alice` into `<home>/.config/pendingyou`, and on NixOS no other user's script writes it |

On nix-darwin, the folder's and the links' steps are appended to `postActivation` after the runner's link, so the order stays the runner's link, then the folder, then the links. On NixOS, `steps` lists them in that order, then `keep` with the runner.

### The modules install timber, replaceable or dropped

| Scenario | Proven by |
|---|---|
| timber with the runner | `timber with the runner` |
| A host's own timber | `A host's own timber` |
| No timber of the set's | `No timber of the set's` |
| A host that keeps its own timber file | `A host that keeps its own timber file` |
| No timber without the runner | `No timber without the runner` |
| timber turned on without the runner | `timber turned on without the runner` |

## What the requirements hold that no scenario names

- **"Without the runner, no other step of the set's runs as that user."** On nix-darwin, the agent, the signing, the runner's `requiresPrimaryUser` entry and the runner's link stay under `services.boomdrive.enable`. On NixOS, the linger, the unit and the runner's assertions stay under it, and the script gains the runner's link and `keep` only with the runner.
- **The primary user's home.** On nix-darwin, the assertion that the primary user has a home holds wherever the runner, the folder or the links run, not only with the runner.
- **The tools' user where none is named.** On NixOS, Flume's, sluice's, crew's and timber's `user` are set only where `services.boomdrive.user` is defined, so each tool's module asks for its own where it is not.

## Outcome

The checks pass and the work holds, so the change is archived with this report. Its delta is merged into `openspec/specs/service-modules/spec.md`.
