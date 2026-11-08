# Verify

The verify stage's report on `change/herdr-and-ghostty-come-with-the-flake-and-an-app-opens-the-agents` at `e363319`, over its base `bolt/the-boomdrive-flake-configures-the-whole-host` at `4c1996b`, which the branch holds merged (task 3.5) and merges into with no conflict.

**Verdict: the change passes.** The whole checks pass. Every task is ticked and true, and every requirement and scenario of the four spec deltas has a case that passes.

The code stage archived the change in task 4.3, so this report sits in the archived directory, which is the change's directory. The change stands archived, with this report in it.

## The whole checks

Run once, as the repository's instructions name them: `sluice exec main`, exit 0.

```
Tasks: 21 completed
passed: 21 tasks
recorded 21 results under their keys for e363319
```

| project | format | lint | typecheck | test |
|---|---|---|---|---|
| `boomdrive` | pass | pass | pass | pass |
| `boomdrive-crew` | pass | pass | pass | pass |
| `boomdrive-hosted` | pass | pass | pass | pass |
| `nix` | pass | pass | pass (`nix: typecheck: every case holds`) | pass (`nix: test: every case holds`) |

`root:adrs`, `root:changes`, `root:format`, `root:releases` and `root:specs` pass. `root:specs` reports 17 passed and 0 failed, `way-in` among them.

`nix:lint` runs statix, deadnix and actionlint in turn, and each passes.

sluice reports that `e363319` would conflict with `main` (`5e9c753`) in `README.md`, `flake.nix`, `flake.lock`, `docs/architecture/core.md`, records 0023 and 0028, `openspec/specs/views/spec.md` and several of the runner's files, so it ran the checks on the branch as it stands. Those are the bolt's differences from `main`, a matter for landing the bolt, and no fault of this change: against the bolt at `4c1996b` the branch merges cleanly.

## Tasks

All 16 tasks, 1.1 to 4.3, are ticked `[x]`. Each task's "True when" names `nix:test`, `nix:typecheck`, `nix:lint`, `root:changes`, `root:format` or `root:specs`, and each of those passes in the gate above. Task 4.3's archive is in place, and `root:specs` passes after it.

Task 3.5 holds the base merged:

- `nix/modules/darwin.nix` and `nix/modules/nixos.nix` import `way-in.nix` beside the base's `drives.nix`, render the runner's file from `accounts` and `settings` through `accounts.nix`, and link herdr's plugins. Their headers name both records 0023 and 0028.
- `nix/tests/default.nix` runs `drives.nix`'s cases beside `way-in.nix`'s, and no two files give a case of one name.
- `services.flume.settings` takes `host.multiplexer` and `host.terminal_app` from `way-in.nix` and the drives' `streams` and `repos` from `drives.nix`, side by side.
- CLAUDE.md names herdr, its plugins, Ghostty, the launcher and the shell beside the base's `accounts` and each drive's stream given to Flume's module.
- core.46's details keep this change's options, Flume's two keys and herdr's `plugin link` beside the base's `accounts`.
- `openspec/specs/service-modules/spec.md` holds the base's requirements, its drives' and crew table's among them, with this change's deltas applied over them.

## Requirements and scenarios

Each scenario below has a case under its name, either in `nix/tests/` (evaluated by `nix/check.sh`) or in a sh case that `nix/check.sh test` runs. Each one passes.

### way-in (added)

| Requirement | Scenario | Proven by |
|---|---|---|
| The launcher opens the host's agents at a person's asking | Opened; The profiles first | `nix/tests/way-in.sh`: `flume open` called once with nothing shown; `/etc/profiles/per-user/alice/bin` and `$HOME/.nix-profile/bin` before `/usr/bin` |
| The launcher shows Flume's refusal | Flume refuses on macOS; Flume refuses elsewhere | `nix/tests/way-in.sh`: on macOS, Flume's words reach `osascript` under `display alert "Boomdrive"`; on Linux, they go to standard error with no alert; exit 1 |
| The launcher is an app on macOS and a desktop entry on Linux | The app on macOS; The desktop entry on Linux | `nix/tests/way-in.nix`, from `launcher.nix`'s `passthru.parts`: the plist's `CFBundleName` and `CFBundleExecutable`, the entry's `Name`, `Exec` and `Terminal` |
| The flake gives a shell that starts herdr | Every system's shell | `nix/tests/way-in.nix` (the five tools and the hook, on three systems) and `nix/tests/typecheck.nix` (each shell evaluates) |
| | Entering the shell; A command through the shell; Inside herdr already | `nix/tests/way-in.sh`: bash with `-i`, bash without it, and bash with `HERDR_ENV=1` |

### family-packages (renamed, modified, added)

| Requirement | Scenario | Proven by |
|---|---|---|
| The flake gives the family's host tools as packages | Every system's packages; No tool of the host's own | `nix/tests/typecheck.nix`: nine packages and `default`, and git alone the host's |
| The flake and its lock name one set | A tag moved without the lock; A tool named by no release tag; herdr named by no release tag | `nix/tests/tags.nix` and `nix/tests/packages.nix` |
| Each package is made from this tree, its tool's own flake at its release tag, or the flake's nixpkgs | Flume, sluice, crew through their own flakes; The runner from this tree; herdr through its own flake | `nix/tests/packages.nix`: herdr's package is its flake's, at the version its lock tag gives |
| herdr is the release Flume's terminal profile was checked against | herdr is Flume's; herdr other than Flume's; A plugin newer than herdr | `nix/tests/packages.nix`: `herdrVersion` read from Flume's `devenv.nix` at the locked tag, and each plugin's `min_herdr_version` |
| Ghostty is the release Flume's window profile was checked against | Ghostty on each system; Ghostty moved with nixpkgs | `nix/tests/packages.nix`: `ghostty-bin` on darwin, `ghostty` on Linux, at `1.4.0` |

### service-modules (renamed, modified, added)

| Requirement | Scenario | Proven by |
|---|---|---|
| The runner finds its tools on the profiles' PATH | The set's tools; A host's own tools; The user's profile first | `nix/tests/tools.nix` |
| The Boomdrive module writes no file of the other tools' | The tools' activation is their own | `nix/tests/tools.nix`: nix-darwin's comparison leaves out the runner's link and the plugins' links |
| The modules hold no option that installs a tool its own module installs | The tools option is gone; The README's words | `nix/tests/tools.nix`; `nix/tests/cache.nix` (the four new options and `nix develop github:agentplot/boomdrive`) |
| The modules install herdr, Ghostty and the launcher, each replaceable or dropped | The way in with the runner; A host's own herdr; No Ghostty of the set's; Not without the runner | `nix/tests/way-in.nix`, on nix-darwin and on NixOS |
| The modules link the set's herdr plugins with herdr's own command | The plugins linked on nix-darwin; The plugins linked on NixOS; No herdr of the set's | `nix/tests/way-in.nix`: the step as `alice` on nix-darwin, inside `alice`'s test on NixOS and in no other user snippet, and none with no herdr or no plugin |
| | Each plugin linked | `nix/tests/activate.sh`: the plugins linked in order, with no `HERDR_*` variable set, exit 0 |
| A plugin herdr refuses to link stops nothing else | A link herdr refuses | `nix/tests/activate.sh`: one line, the second plugin still linked, exit 0 |
| The modules name herdr and Ghostty Flume's terminal and window | Flume's terminal and window; A host's own window; The word of before | `nix/tests/way-in.nix`, on nix-darwin and on NixOS; `way-in.nix` sets both keys with `mkDefault`, and no `multiplexer` where `host.terminal` is given |

### binary-cache (modified)

| Requirement | Scenario | Proven by |
|---|---|---|
| The workflow builds and pushes every package of every system | Each system has its job; The runner is pushed with its tools; A package that does not build; A branch's push | `nix/tests/workflow.nix`: each job builds every name `nix eval` lists for `packages.$SYSTEM`, `herdr`, `ghostty` and `boomdrive-launcher` among them |
| A host substitutes the set, or builds the same paths | A host's own nixpkgs; The host's trust | `nix/tests/cache.nix`: the overlay shadows all seven names, on nix-darwin and on NixOS |

## The rest of the change

- **Touches.** Against the base `4c1996b`, the branch changes only the paths `proposal.md` lists, and `openspec/` for the archive. The runner's code does not change. No crate, gate tool or tracked file type is added (core.7, core.8).
- **Rules.** core.13, core.29, core.44 and core.46 carry record 0023's amendments, as the design's "Rules amended" says, and core.46 keeps the base's `accounts` beside them.
- **Record.** Record 0023 is accepted.
- **Specs.** The archived specs pass `root:specs`. `openspec/specs/way-in/spec.md` is new, and `family-packages`, `service-modules` and `binary-cache` hold the deltas over the base's.
- **Rule relied on.** These start no herdr server and set no `HERDR_*` variable for the runner (core.13, core.29):
  - the launcher, which runs `flume open` once, at a person's asking;
  - the shell's hook, which starts herdr only in an interactive shell outside herdr;
  - `activate.sh plugins`, which runs herdr's own `plugin link` with every `HERDR_*` variable unset.

  The runner's code names no multiplexer.
