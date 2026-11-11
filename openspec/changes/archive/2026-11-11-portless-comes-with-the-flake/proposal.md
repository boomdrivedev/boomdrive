# Proposal

## Why

Record 0023, as amended on 2026-11-08, puts portless in the set. The user chose it on their card "Boomdrive flake: beyond Pending You and Plannotator, which of your tools should it also install?", answering "worktrunk (wt), portless (the .localhost dev addresses)". In the record's words:

- **What it is for.** The flake installs portless, "which gives each worktree's dev server a `.localhost` address and whose routes crew's board reads".
- **As herdr is.** It is "the set's by default and overridable as herdr is: a host replaces either through the modules' options, or drops it for its own".

Today no host gets portless from the flake:

- A host that has portless writes its own recipe and installs it itself, as afterthought/swancloud does for its fleet.
- crew's board reads a host's dev servers from `portless list` (crew's `gather.routes`). On a host without portless it says "portless did not answer".
- sluice's previews guide routes a preview's names with `portless alias`.

The pieces are ready:

- **portless** is vercel-labs/portless, under Apache-2.0. It is a pnpm workspace whose package `packages/portless` is the command line. tsup bundles it into one file with no dependency, which runs on Node.js 24 or later.
- **Its release.** Its latest release is `v0.17.0`, tagged on 2026-11-08. It publishes no flake, and nixpkgs carries no portless.
- **The tools to build it.** The flake's locked nixpkgs carries `pnpm_11`, `fetchPnpmDeps`, `pnpmConfigHook` and `nodejs_24`. A recipe made with them built `v0.17.0` from its source on `aarch64-darwin` on 2026-11-11, and the build's `portless --version` said `0.17.0` (design Context).

This change is the unit `portless-comes-with-the-flake` of the bolt `the-flake-brings-its-companion-tools`, beside the unit `worktrunk-comes-with-the-flake`.

## What Changes

- **portless in the set.**
  - portless's source joins the inputs at its release tag `v0.17.0`, as a source and not a flake.
  - `packages.<system>.portless` is a recipe of this flake's, `nix/packages/portless.nix`. It builds the workspace's `packages/portless` alone from that source, with the flake's nixpkgs' pnpm 11, and runs it on that nixpkgs' Node.js 24.
  - Its version is its tagged `package.json`'s, held to the tag. Its dependencies are one fetch, the same on every system.
  - git stays the host's own: portless finds the host's git on its `PATH`.
- **The modules.** With `services.boomdrive.enable` they install `services.boomdrive.portless.package`. It is the set's by default, a host's own replaces it, and `null` installs none. Without the runner they install none.
- **The proxy stays the person's.** The modules start no proxy of portless's, declare no service of it, write none of its files and trust no certificate authority. portless's proxy is portless's own: its first `portless run` at a terminal starts it, or the person's `portless service install` does. Trusting its certificate authority is the person's `portless trust`.
- **The shell.** The flake's shell's `PATH` gains the set's portless, beside crew, which reads its routes.
- **The cache.** The workflow builds and pushes portless with the rest, and the modules install the flake's own.
- **Words.**
  - The README's install with nix names portless in the set and its option. It says the proxy and the trust are the person's.
  - CLAUDE.md and `openspec/config.yaml` name portless in the flake.
  - Record 0023 gains an amendment holding this change's choices. core.13's, core.44's and core.46's details are amended. Both are in the commit that writes this change (design, Rules amended).

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `family-packages`: the set gains `portless`, built by a recipe of this flake's from its source at its release tag. portless is named by a release tag. Its dependencies are one fetch for every system, it runs on the flake's Node.js 24, and git stays the host's own.
- `service-modules`: the modules install portless, replaceable or dropped, and start no proxy of it. The README names its option, and the proxy and the trust as the person's.
- `binary-cache`: portless is built and pushed with the rest, and the modules install the flake's own.
- `way-in`: the shell's `PATH` holds the set's portless.

## Impact

- `flake.nix` and `flake.lock`: portless's input, the package `portless` and the header's words.
- `nix/packages/portless.nix`, added.
- `nix/shell.nix`.
- `nix/modules/apps.nix`: the option `services.boomdrive.portless.package`, installed with the runner.
- `nix/tests/`: `typecheck.nix`, `tags.nix`, `packages.nix`, `apps.nix`, `way-in.nix` and `cache.nix`.
- `README.md`, `CLAUDE.md` and `openspec/config.yaml`.
- Record 0023 and `docs/architecture/core.md`, in the commit that writes this change.
- The runner's code does not change, and runs no portless (core.1, core.13).
- No crate, tool of the gate or tracked file type is added (core.7, core.8). The gate evaluates the recipe and never builds it. pnpm and Node.js are its build's, from the flake's nixpkgs.
- crew does not change. Its board reads `portless list` from the host's `PATH`, which the system profile now gives. Its `crew sites` names a worktree's server with afterthought/swancloud's `devurl`, which this change does not bring.
- Outside this repository: afterthought/swancloud, once it takes the flake, may drop its own recipe for the set's portless, or keep its own through `services.boomdrive.portless.package` (record 0023).

## Touches

```
flake.nix
flake.lock
nix/**
README.md
CLAUDE.md
openspec/config.yaml
docs/adr/0023-the-boomdrive-flake-configures-the-whole-host-each-tool-by-its-own-module-and-each-repository-named-once-by-its-owner.md
docs/architecture/core.md
```
