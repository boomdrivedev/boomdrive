# Proposal

## Why

The brief: "Every agentplot/ address in boomdrive's tree of a tool that has moved names boomdrivedev/ instead: its flake inputs (sluice, flume, crew, weir, timber), its site's address, its README, policies and skills."

Record 0025's amendment of 2026-11-09 says every address moves once: after its move, each repository's own change rewrites its `agentplot/` addresses to `boomdrivedev/`. Its amendments of 2026-11-12 record what moved, in the user's words to agentplot-ops that day. timber, weir, flume, crew, blueprints and sluice moved. So did boomdrive-cloud, strategy, boom, site-theme, sounding, millpond, headwater and eddy. "Boomdrive itself too": `agentplot/boomdrive` moved whole, still private, as `boomdrivedev/boomdrive`. agentplot/agentplot stays in agentplot. GitHub redirects a moved repository's git and web addresses, but not its Pages site. The family's sites under `agentplot.github.io` answer 404, and they answer at `boomdrivedev.github.io`.

agentplot-planner's answer of 2026-11-12, to this fix's code stage, settles the brief's two open points. Boomdrive's own addresses move, and so does its site's address, to `boomdrivedev.github.io/boomdrive`. The site serves there once Boomdrive opens, which is the user's call later. The launchd label `io.github.agentplot.boomdrive` stays: it names the runner's service on each host, and is no address. Renaming it would leave deneb's running service behind. The site's pages and diagrams are the content kind's (site.5), handed on separately.

## What Changes

- **The set's inputs.** The flake's sluice, Flume and crew inputs and the lock's owners name `boomdrivedev`, at the same tags and revisions. The runner's weir crates do too, in `Cargo.toml` and `Cargo.lock`. Each source's hash is unchanged, so the set's Flume, sluice and crew paths are too, and the runner builds from the same trees (core.44, core.45).
- **The site's theme.** `site/package.json` takes `github:boomdrivedev/site-theme#v0.1.3`. `site/bun.lock` is bun's own install of it: the same commit, under the root GitHub now serves, with that tarball's integrity.
- **Boomdrive's own addresses.**
  - setup names `github:boomdrivedev/boomdrive#crew` and `github:boomdrivedev/boomdrive#herdr-plugins`.
  - README's installs, flake and shell name `github:boomdrivedev/boomdrive`, and its quick start's address is `boomdrivedev.github.io/boomdrive`.
  - `cliff.toml`'s links and the packages' homepages name `boomdrivedev/boomdrive`.
- **The site's address.** The site is built for `https://boomdrivedev.github.io` under `/boomdrive/`. Its repository link is `https://github.com/boomdrivedev/boomdrive`, as `site:test` holds it. The quick start's two files take the new flake address, since `nix:test` holds each to README's (site.8). That departs from site.5's letter, under which a step's words are the content kind's to change. Only the address a test ties to README changes, and no step is reworded.
- **The policies.**
  - README's examples and citations name the moved tools at `boomdrivedev`, and so do CLAUDE.md and `site/CLAUDE.md`.
  - The area page's address and core.48's, core.57's and core.58's citations do too. No rule's words change.
  - `site/README.md`, the site's design and the modules' option examples follow.

### What stays

- **Identifiers that are no address.**
  - The launchd label `io.github.agentplot.boomdrive`, and the launcher's bundle identifier of the same name (agentplot-planner, 2026-11-12).
  - The modules' key `github:agentplot/boomdrive#<platform>Modules.boomdrive`, which only tells one import of the module from another.
  - The protocol schemas' `$id`s and `ADDRESS`. Version 1's files never change (core.42), and version 2 keeps the namespace version 1 has, which boomdrive-cloud's copy reads.
- **What the site shows** (site.5): the principles' links, the six diagrams' links, `method-outline.md`'s table of them, and the URL the method-tools spec quotes. They are the content kind's, handed on by agentplot-planner.
- **History.** Records, archived changes and the design notes of past changes keep the addresses they cited. That includes every mention of `agentplot/boomdrive-cloud`, all of which are in records and archived changes.
- **Neutral examples.** Tests, fixtures and specs that name `agentplot/flume` and its neighbours as an example repository (core.41).
- **`@agentplot/site-theme`**: the theme's package name, its own until a release of the theme changes it.
- **The site spec's Purpose**, which names the old address in words no delta reaches. Its requirements name the new one.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `setup`: a missing program or plugin is named with the set's address at `boomdrivedev/boomdrive`.
- `service-modules`: README's words name `nix develop github:boomdrivedev/boomdrive`.
- `site`: the site's repository link is `https://github.com/boomdrivedev/boomdrive`.

## Impact

- `flake.nix`, `flake.lock`, `Cargo.toml`, `Cargo.lock`, `nix/tests/packages.nix`: the set's inputs and the weir crates.
- `site/package.json`, `site/bun.lock`: the theme's source.
- `crates/boomdrive/src/setup.rs`, `crates/boomdrive-crew/src/setup.rs` and their `tests/setup.rs`: setup's words.
- `README.md`, `crates/boomdrive/tests/readme.rs`, `nix/tests/cache.nix`: README and the tests that read it.
- `cliff.toml`, `nix/packages/boomdrive.nix`, `nix/packages/launcher.nix`: release links and homepages.
- `site/astro.config.mjs`, `site/src/layouts/Base.astro`, `site/check-site.sh`, `site/src/start/mac/install.sh`, `site/src/start/nixos/flake.nix`: the site's address, its repository link and the quick start's flake address.
- `CLAUDE.md`, `site/CLAUDE.md`, `site/README.md`, `docs/architecture/core.md`, `docs/architecture/site.md`, `site/docs/site-design.md`, `site/docs/method-outline.md`, `nix/modules/options.nix`: addresses and citations.
- No configuration key, field of `report.json`, protocol message, tool, tracked file type or launchd label changes.
