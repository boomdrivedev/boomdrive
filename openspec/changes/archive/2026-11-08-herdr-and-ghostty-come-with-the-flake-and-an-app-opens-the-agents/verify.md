# Verify

The verify stage's report on `change/herdr-and-ghostty-come-with-the-flake-and-an-app-opens-the-agents` at `0a09003`, cut from `bolt/the-boomdrive-flake-configures-the-whole-host` at `4f39b90`.

**Verdict: the whole checks fail, so the change does not pass.** `nix:lint` fails on one statix warning in a test file. The other 20 tasks pass. Every task is ticked, and every requirement and scenario of the four spec deltas has a case that passes in `nix:test` or `nix:typecheck`. The code stage must fix the lint before this change can pass.

The code stage archived the change in task 4.3, so this report sits in the archived directory, which is now the change's directory. The verify stage archived nothing and undid nothing.

## The whole checks

Run once, as the repository's instructions name them: `devenv shell -- sluice exec main`, exit 1.

```
Tasks: 20 completed, 1 failed
failed: nix:lint
```

| project | format | lint | typecheck | test |
|---|---|---|---|---|
| `boomdrive` | pass | pass | pass | pass |
| `boomdrive-crew` | pass | pass | pass | pass |
| `boomdrive-hosted` | pass | pass | pass | pass |
| `nix` | pass | **fail** | pass | pass (`nix: test: every case holds`) |

`root:adrs`, `root:changes`, `root:format`, `root:releases` and `root:specs` pass.

### What fails

`nix:lint` runs `statix check flake.nix && statix check nix && deadnix --fail flake.nix nix && actionlint .github/workflows/cache.yml`. `statix check nix` fails:

```
[08] Warning: These parentheses can be omitted
  ╭─[ nix/tests/way-in.nix:28:17 ]
28 │     ++ t.expect (parts.script.executable) "the aarch64-darwin launcher's script is not executable"
   │                 ─────────────┬──────────
   │                              ╰──────────── Useless parentheses around primitive expression
```

The line came in with `5a1178e` (task 2.2). The `&&` chain stops at statix, so `deadnix` and `actionlint` did not run over the change's files in this gate. They need to pass once statix passes.

## Tasks

All 14 tasks, 1.1 to 4.3, are ticked `[x]`. Each task's "True when" names `nix:test`, `nix:typecheck`, `root:format` or `root:specs`, and each of those passes. No task names `nix:lint`, but the gate requires it (`.config/sluice.toml`, check-fast on `main`, core.7). Task 2.2 is done in substance, but its commit leaves the gate failing.

## Requirements and scenarios

Each scenario below has a case under that name in `nix/tests/` (evaluated by `nix/check.sh`) or in a sh case it runs, and each passes.

### way-in (added)

| Requirement | Scenario | Proven by |
|---|---|---|
| The launcher opens the host's agents at a person's asking | Opened; The profiles first | `nix/tests/way-in.sh` |
| The launcher shows Flume's refusal | Flume refuses on macOS; Flume refuses elsewhere | `nix/tests/way-in.sh`: the words go to `osascript` as an argument, and on Linux to standard error with no alert |
| The launcher is an app on macOS and a desktop entry on Linux | The app on macOS; The desktop entry on Linux | `nix/tests/way-in.nix`, from `launcher.nix`'s `passthru.parts` |
| The flake gives a shell that starts herdr | Every system's shell | `nix/tests/way-in.nix` (the five tools and the hook, three systems) and `nix/tests/typecheck.nix` (each `drvPath` evaluates) |
| | Entering the shell; A command through the shell; Inside herdr already | `nix/tests/way-in.sh`, with bash `-i` and without, and with `HERDR_ENV=1` |

### family-packages (renamed, modified, added)

| Requirement | Scenario | Proven by |
|---|---|---|
| The flake gives the family's host tools as packages | Every system's packages; No tool of the host's own | `nix/tests/typecheck.nix`: nine packages and `default`, and `hosts = [ "git" ]` |
| The flake and its lock name one set | A tag moved without the lock; A tool named by no release tag; herdr named by no release tag | `nix/tests/tags.nix` (herdr added) and `nix/tests/packages.nix` |
| Each package is made from this tree, its tool's own flake at its release tag, or the flake's nixpkgs | Flume, sluice, crew through their own flakes; The runner from this tree | `nix/tests/packages.nix`, unchanged |
| | herdr through its own flake | `nix/tests/packages.nix`: `drvPath` is herdr's flake's, at the version its lock tag gives |
| herdr is the release Flume's terminal profile was checked against | herdr is Flume's | `nix/tests/packages.nix`: reads `herdrVersion` from Flume's `devenv.nix` at the locked tag, and each plugin's `min_herdr_version` |
| | herdr other than Flume's; A plugin newer than herdr | `nix/tests/packages.nix`, on the given versions |
| Ghostty is the release Flume's window profile was checked against | Ghostty on each system; Ghostty moved with nixpkgs | `nix/tests/packages.nix`: `ghostty-bin` on darwin, `ghostty` on Linux, `1.4.0` |

### service-modules (renamed, modified, added)

| Requirement | Scenario | Proven by |
|---|---|---|
| The runner finds its tools on the profiles' PATH | The set's tools; A host's own tools; The user's profile first | `nix/tests/tools.nix`, unchanged |
| The Boomdrive module writes no file of the other tools' | The tools' activation is their own | `nix/tests/tools.nix`: nix-darwin leaves out the runner's link and the plugins' links, and `nix/tests/lib.nix`'s tools-only system sets Flume's two keys |
| The modules hold no option that installs a tool its own module installs | The tools option is gone | `nix/tests/tools.nix`, unchanged |
| | The README's words | `nix/tests/cache.nix`: the four new options and `nix develop github:agentplot/boomdrive` |
| The modules install herdr, Ghostty and the launcher, each replaceable or dropped | The way in with the runner; A host's own herdr; No Ghostty of the set's; Not without the runner | `nix/tests/way-in.nix`, on nix-darwin and on NixOS |
| The modules link the set's herdr plugins with herdr's own command | The plugins linked on nix-darwin; The plugins linked on NixOS; No herdr of the set's | `nix/tests/way-in.nix` (NixOS: inside alice's guard, and in no other user's snippet) and `nix/tests/nixos.nix` |
| | Each plugin linked | `nix/tests/activate.sh`: in order, with neither `HERDR_*` variable set, exit 0 |
| A plugin herdr refuses to link stops nothing else | A link herdr refuses | `nix/tests/activate.sh`: one line, the second plugin still linked, exit 0 |
| The modules name herdr and Ghostty Flume's terminal and window | Flume's terminal and window; A host's own window; The word of before | `nix/tests/way-in.nix`, on nix-darwin and on NixOS |

### binary-cache (modified)

| Requirement | Scenario | Proven by |
|---|---|---|
| The workflow builds and pushes every package of every system | Each system has its job; The runner is pushed with its tools; A package that does not build; A branch's push | `nix/tests/workflow.nix`: the job builds every name `nix eval` lists for `packages.$SYSTEM`, which now include `herdr`, `ghostty` and `boomdrive-launcher` |
| A host substitutes the set, or builds the same paths | A host's own nixpkgs | `nix/tests/cache.nix`: the overlay shadows all seven names, on nix-darwin and on NixOS |
| | The host's trust | `nix/tests/cache.nix`, unchanged |

## The rest of the change

- **Touches.** The branch changes only the paths `proposal.md` lists, plus `openspec/` from the archive. The runner's code is unchanged, and no crate, gate tool or tracked file type is added.
- **Rules.** The commit that wrote the change (`f0a0e95`) amends core.13, core.29, core.44 and core.46, as the design's "Rules amended" says.
- **Record.** Record 0023 is accepted.
- **Specs.** The archived specs pass `root:specs`. `openspec/specs/way-in/spec.md` is new, and `family-packages`, `service-modules` and `binary-cache` hold the deltas.
- **Rule relied on.** The launcher, the shell's hook and `activate.sh plugins` start no herdr server and set no `HERDR_*` for the runner (core.13, core.29). The runner's code names no multiplexer.

## What goes back to the code stage

- `nix/tests/way-in.nix:28` reads `t.expect (parts.script.executable) "…"`. Write it `t.expect parts.script.executable "…"`, or otherwise clear statix's [08] warning.
- Then run `sluice exec main -- moon run nix:lint` to check that statix, deadnix and actionlint all pass, and run the gate again.
