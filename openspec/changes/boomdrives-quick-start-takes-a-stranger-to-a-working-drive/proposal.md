# Proposal

## Why

The brief: "A person with a Mac or a NixOS machine and nothing of the family installed follows Boomdrive's quick start on its site from nothing to a drive whose agents build and land a change, each step of it one a test runs." Its sources are record 0025, under which Boomdrive opens and its site publishes from this repository; agentplot/blueprints `design/principles.md` 3, "Design for every operator"; and the context pack's "Plugins, sites and the record of a release", under which each tool's site is in its own repository.

The site has no quick start. A stranger's way to a working drive is spread over README's sections: the install with nix, the modules and `boomdrive setup`. Tests read some of their commands, and none reads them as one way through. Two gaps sit on that way. On a Mac Nix does not manage, Flume has no file of its own, so it runs no multiplexer, and the runner's launches print their commands rather than open an agent that builds (record 0009, its amendment of 2026-11-11). And no step of README's is held to run as a reader would run it.

The site's design gains its part for this change, "The quick start" in `site/docs/site-design.md`, with a row of its site map and the type Step. Record 0043 and rule site.8 are written with this change's plan.

## What Changes

- **A quick start page, `/start/`.** It is reached from the masthead as "Start". It opens on what the reader brings: Nix with flakes on an Apple silicon Mac, or a NixOS configuration that is a flake; a GitHub account; Claude Code. Each is linked to its own installer. Then come the steps "On a Mac" and "On NixOS", each in order. A step both share is one record shown in both. The page has no script and no motion.
- **A step is a record, words and files.** A new collection, `start`, holds one Markdown record per step. Each record has its title, its place on each platform, the files it shows and the test that runs it. Each file is under `site/src/start/`, a shell line, a TOML file or a Nix file. The page shows it as it stands, with the path the reader writes it to where they write one (record 0043; site.8).
- **The steps.**
  - On a Mac: install the set with git, gh and Node.js by `nix profile add`; sign in; write Flume's host keys, `multiplexer = "herdr"` and `terminal_app = "terminal"`; run `boomdrive setup`; commit sluice's policy, and Flume's binding of sluice as the merge queue, to the drive's repository; start the drive's agents; ask for a change and watch it land.
  - On NixOS: a host flake that takes the set's module; the drive's module; the switch; the sign-ins; the state repository; then the same last three steps.
- **Each step's test runs its files as they stand.** The tests are in two places.
  - `crates/boomdrive/tests/quick_start.rs` runs the shell steps through `sh`, with the runner built from the tree, the set's Flume and sluice by their paths, and stand-ins for gh, Claude Code, Pending You, herdr and `launchctl` alone.
  - `nix/tests/quick-start.nix` evaluates the Nix steps and the Mac's install, as the flake's checks evaluate a module.

  Where a line's run would reach only a stand-in, the test holds it to what setup names for it: the sign-ins, the state repository and `start`. The last step's test runs a change from crew's records through the runner, the set's Flume and the set's sluice to the repository's origin, with a stand-in agent.
- **The site's tests hold the steps to their tests.** `site:test` fails where:
  - a step names a test its project does not hold;
  - a file under `site/src/start/` is no step's;
  - a step's file is not on the page as it stands;
  - a platform's steps are out of order;
  - the page names a drive or a machine of ours (site.7).
- **README names the quick start** where it says how a host is set up.
- **Record 0043 and rule site.8**, written with the plan, and `site/CLAUDE.md`'s line for them.

## Capabilities

### New Capabilities

- `quick-start`: the site's quick start, its steps and the tests that run each.

### Modified Capabilities

None. No requirement of the setup, service-modules, family-packages or site specs changes. The quick start runs what they already require.

## Impact

- `site/`:
  - `src/content.config.ts`: the collection `start`;
  - `src/content/start/*.md`, new: one record per step;
  - `src/content/pages/start.md`, new: the page's lede and what the reader brings;
  - `src/start/`, new: the steps' files, under `mac/`, `nixos/` and `both/`;
  - `src/pages/start.astro`, new: the page;
  - `src/layouts/Base.astro`: the masthead's "Start";
  - `src/styles/site.css`: a block of a step's file, in the page's tokens;
  - `src/content/terms/terms.yaml`: a term the steps' words take, where Vale asks for one;
  - `check-site.sh` and `moon.yml`: the quick start's checks and their inputs;
  - `docs/site-design.md` and `CLAUDE.md`: written with the plan.
- `crates/boomdrive/tests/quick_start.rs`, new; `crates/boomdrive/tests/stand-in/claude.rs`, new, the stand-in agent, a binary `crates/boomdrive/Cargo.toml` builds only under its feature `stand-in`; `crates/boomdrive/moon.yml`: the test's inputs, the steps' files among them, and the feature its test names.
- `nix/tests/quick-start.nix`, new; `nix/tests/default.nix`; `nix/moon.yml`: the steps' files as inputs.
- `treefmt.nix`: taplo claims the steps' TOML files, and the steps' shell files need no shebang (core.7).
- `README.md`: the quick start's address.
- `docs/adr/0043-…`, `docs/architecture/site.md`: written with the plan.
- No configuration key, field of `report.json`, protocol message, crate or tool is added, and no command of the runner's changes. The site gains a page type, its schema and its checks (site.5).
- Outside this repository:
  - Flume has no command that sets its host keys. The Mac's Flume step stands until it does and setup runs it. That change removes the step, its file and its test (principle 11; record 0009's amendment of 2026-11-11).
  - The bolt `the-sets-crew-pin-takes-its-new-names` renames crew's module settings. The NixOS drive's file is written by the names this branch holds, and `nix:test` fails it until it follows (design Risk 2).

## Touches

```
site/src/content.config.ts
site/src/content/start/
site/src/content/pages/start.md
site/src/start/
site/src/pages/start.astro
site/src/layouts/Base.astro
site/src/styles/site.css
site/src/content/terms/terms.yaml
site/check-site.sh
site/moon.yml
site/docs/site-design.md
site/CLAUDE.md
crates/boomdrive/tests/quick_start.rs
crates/boomdrive/tests/stand-in/claude.rs
crates/boomdrive/Cargo.toml
crates/boomdrive/moon.yml
nix/tests/quick-start.nix
nix/tests/default.nix
nix/moon.yml
treefmt.nix
README.md
docs/adr/0043-the-quick-start-is-steps-the-gate-runs-each-a-file-its-page-shows-as-it-stands.md
docs/architecture/site.md
openspec/specs/quick-start/spec.md
```
