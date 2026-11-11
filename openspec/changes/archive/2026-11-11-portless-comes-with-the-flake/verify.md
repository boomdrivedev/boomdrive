# Verify: portless-comes-with-the-flake

Verified on `change/portless-comes-with-the-flake` at `db0313a`, cut from
`bolt/the-flake-brings-its-companion-tools` at `57b1d1e`, on 2026-11-11.

**Verdict:** the work holds. The whole checks pass, every requirement
and scenario of the four deltas is met, and every task is ticked.

The code stage archived the change already, at `db0313a`, as task 4.1
asks. So this report is written into the archived change's directory,
and the archive needs no second move.

## The whole checks

`sluice exec main`, run once on `db0313a`, exited 0:

- **What it ran.** sluice ran the checks `db0313a` affects against
  `main` (`57b1d1e`). 16 tasks completed, and 14 passed and were
  recorded under their keys.
- **The nix project.** `nix:format`, `nix:lint`, `nix:typecheck` ("every
  case holds") and `nix:test` ("every case holds").
- **The root.** `root:format`, `root:specs` (23 passed, 0 failed),
  `root:changes`, `root:adrs` and `root:releases`.
- **The crate and the site.** `boomdrive:test`, and the site's
  `install`, `vocab`, `build`, `check`, `prose` and `test`.

The run also printed two kinds of warning, neither of which failed a
task. moon could not write the shared cache's blobs or archive a
task's outputs from inside the sandbox, and devenv named
`.mcp.json` and `.claude/settings.json` as conflicting files.

## Tasks

All ten tasks, 1.1 to 4.1, are ticked `[x]`. Each names a check that
passed in the whole run above:

- **1.1 to 3.1.** `nix:typecheck` and `nix:test`.
- **3.2.** `root:format`.
- **4.1.** `root:specs`. The bolt's base has not moved since the change
  was cut. `worktrunk-comes-with-the-flake` has not landed on it, so the
  merge that 4.1 names had nothing to take, and the deltas archived as
  written.

The trial build of task 1.1 is on this host:
`/nix/store/ykfqn8mwv4y2q2knayx8av4mfllgblil-portless-0.17.0`, whose
`bin/portless --version` says `0.17.0`.

## Requirements and scenarios

### family-packages

- **The flake gives the family's host tools as packages.**
  - `nix/tests/typecheck.nix`'s set gains `portless`, so "Every system's
    packages" holds the ten.
  - "No tool of the host's own" holds, since git is not among them.
- **The flake and its lock name one set.**
  - `tags.nix` lists `portless` among the tools held to a release tag.
  - `packages.nix`'s "portless named by no release tag" builds a
    `main` URL and ref, and expects one failure naming portless and
    `main`.
  - `flake.nix` names `github:vercel-labs/portless/v0.17.0` with
    `flake = false`, and `flake.lock` locks that ref at `1d2dbbc`.
- **Each package is made from this tree, its tool's own flake or source
  at its release tag, or the flake's nixpkgs.** The requirement is
  renamed, and archived under its new name.
  - "portless from its source" holds each system's `src` to
    `inputs.portless`, its `mainProgram` to `portless`, and its version
    to the lock's tag.
  - "portless other than its tag" expects one failure naming `0.17.1`
    and `0.17.0`.
  - `nix/packages/portless.nix` reads the version from the tagged
    tree's `packages/portless/package.json`.
- **portless is built from its package alone and runs on the flake's
  Node.js.**
  - "One fetch for every system" checks one `outputHash` across the
    three systems, `pnpmWorkspaces == [ "portless" ]`, and pnpm 11.
  - "The flake's Node.js" checks `passthru.node` against nixpkgs'
    `nodejs_24` by `drvPath`.
  - "portless runs the host's git" checks `runtimeInputs` against
    openssl, procps, which and hostname, in that order, with no git.
  - The recipe builds with `pnpm --filter=portless exec tsup`. Its
    wrapper puts those inputs after the host's `PATH`, with
    `--suffix PATH`.

### service-modules

- **The modules hold no option that installs a tool its own module
  installs.**
  - `apps.nix` declares `services.boomdrive.portless.package` beside
    herdr's, Ghostty's and the launcher's.
  - `cache.nix`'s "The README's words" expects
    `services.boomdrive.portless.package`.
- **The modules install portless, replaceable or dropped.** Each of
  these cases runs on nix-darwin and on NixOS through `onEach`:
  - "portless with the runner";
  - "A host's own portless";
  - "No portless of the set's";
  - "No portless without the runner".

  `cfg.portless.package` joins `environment.systemPackages` under
  `mkIf cfg.enable`.
- **portless's proxy and its trust are the person's.**
  - "No proxy of the modules'" reads, on each platform, the activation
    script, the launchd daemons and agents, the user agents, the NixOS
    activation and user activation steps, and the systemd services and
    user services. It finds the set's portless in none of them.
  - "The README's proxy" expects `portless service install` and
    `portless trust`.
  - The modules write no file of portless's.

### binary-cache

- **The workflow builds and pushes every package of every system.**
  - The workflow builds every attribute of `packages.$SYSTEM`, which
    `workflow.nix`'s "The runner is pushed with its tools" holds. So
    `portless`, now among each system's packages, is built and pushed
    with the rest.
- **A host substitutes the set, or builds the same paths.**
  - `cache.nix`'s `names` gain `portless`.
  - "A host's own nixpkgs" holds the flake's own portless in the profile
    on both platforms, and not the host's overlay's.

### way-in

- **The flake gives a shell that starts herdr.**
  - `nix/shell.nix` adds `set.portless`, and `way-in.nix`'s tools gain
    `portless`. So "Every system's shell" holds the six.
  - The hook is unchanged.

## Words and records

- **The README.** Its install with nix names portless in the set's list,
  `#portless` on the profile line, the option with its `null`, the
  proxy paragraph and the shell's tools, as Task notes 3.1 lay out.
- **CLAUDE.md and `openspec/config.yaml`.** Both name portless in the
  flake's set and among what the modules install.
- **Record 0023 and core.md.** Record 0023's amendment of 2026-11-11,
  and core.13's, core.44's and core.46's details, are in `cc99d12`, the
  commit that wrote the change, as the proposal says.
- **Touches.** No path outside the proposal's Touches changed but the
  change's own `openspec/` files. The runner's code is untouched.

## Observations

Neither of these blocks the change. The formatters pass both, since
neither nixfmt nor treefmt rewraps comments or YAML block text.

- **`flake.nix`, line 4.** The header comment's rewrap left one line of
  131 characters.
- **`openspec/config.yaml`, line 26.** The context's line runs to 87
  characters, past its paragraph's width.

The design's risk "Linux builds unproven here" stands as it says. The
gate evaluates the recipe and builds nothing. The Linux builds are first
made by the cache's workflow after landing.
