# Proposal

## Why

Boomdrive's site is the Method, its field notes and the fleet's measurements. It lives in a repository of its own, agentplot/boomdrive-web, public, published to GitHub Pages at each landing on its `main`. Every other tool of the family keeps its site in its own repository's `site/`. On 2026-11-07 the user decided that boomdrive-web "just becomes the website under BoomDrive", and that "We can make Boomdrive public and merge the website." Record 0025, accepted that day, holds the decision.

The brief: "Boomdrive's site is the public repository's site/, its own project with its own checks, served under /boomdrive/, carrying boomdrive-web's history, working rules, records, specs and content kind, its commits in no release notes and moving no version." Its sources are records 0025 and 0024 and boomdrive-web's `CLAUDE.md`.

## What Changes

- **The site's history comes with it.** boomdrive-web's `main` is rewritten so that every file is under `site/`, and merged once. A field note's date and its first words stay readable from this repository.
- **The site is a moon project of its own, `site`.** Its checks are Astro's type check, Vale over its words, its build, its tests and its workflow's lint, each one task (core.7).
  - They run in the gate's sandbox, which reaches no registry. The devenv shell fetches the site's packages outside it, by flume's `site/packages.nix` at the set's flume tag (flume's record 0059).
  - The shell gains the site's tools, each pinned and checked by `root:toolchain` (core.8): bun, Node.js, Vale, and Biome, which formats the site's TypeScript, JavaScript and CSS.
- **The site is served under `/boomdrive/`**, the public repository's name, and links to `agentplot/boomdrive`. The workflow `pages` builds it in the devenv shell at each push to `main` that changes it, and publishes it.
- **The site's working rules come with it, scoped to `site/`.** They are an area page, `docs/architecture/site.md`, rules `site.1` to `site.6`, and `site/CLAUDE.md`. Its OpenSpec context and rules join `openspec/config.yaml` for the site's specs and changes.
- **Its records and specs come into this repository's.** Its records 0001 to 0003 become records 0038 to 0040. Its archived changes join `openspec/changes/archive/`. Its four specs, `content-kind`, `method-diagrams`, `method-tools` and `tools-hub`, come through this change.
- **Its content kind is the repository's.** `.config/flume.toml` declares `content`, its prompts under `site/docs/prompts/`. A place serves its own site on sluice's `web` port, and `site-stop` is sluice's release command.
- **Its commits are in no release notes and move no version.** `cliff.toml` leaves out a commit that changes only `site/` and the workflow `pages` (core.48). The gate still holds every commit to the convention.
- **Its devenv's MCP server joins this one's**, Astro's documentation beside devenv's (records 0022 and 0025).
- **Record 0037**, proposed, says how the site meets core.6, core.41 and core.48. The site's own fleet names stay as boomdrive-web published them. Its rules are an area page. Its commits are left out of the notes by path. core.6's details, core.7, core.41, core.48 and `core.md`'s opening name the site. They are written with this change.

## Capabilities

### New Capabilities

- `site`: the site as a moon project of its own, its packages installed with no network, its words held to its glossary, its address under `/boomdrive/`, the workflow that publishes it, and the content kind and a place's site as the repository's.
- `content-kind`: boomdrive-web's spec, its paths under `site/`, its address under `/boomdrive/`, and `site/CLAUDE.md` sending content work to the `content` kind.
- `method-diagrams`, `method-tools` and `tools-hub`: boomdrive-web's specs, their paths under `site/`.

### Modified Capabilities

- `releases`: a commit that changes only the site's paths bumps nothing and is in no notes.

## Impact

- The history: boomdrive-web's 201 commits at `4ed5fbe`, each under `site/`, and one merge commit.
- `site/`: boomdrive-web's site, as below.
  - Kept as they are: `public/`, `styles/`, `scripts/vocab.ts`, `.vale.ini`, `bun.lock`, `tsconfig.json`, and `src/` and `docs/` but for what follows.
  - Kept, each with its address or paths changed: `astro.config.mjs`, `package.json`, `src/lib/url.ts`, `src/layouts/Base.astro`, `docs/content-brief.md`, `docs/prompts/`, `docs/site-design.md`, `docs/measurement-roadmap.md`, `README.md` and `CLAUDE.md`.
  - Rewritten: `moon.yml`.
  - New: `scripts/install.sh` and `check-site.sh`.
  - Gone, each replaced by the repository's own:
    - `devenv.nix`, `devenv.yaml`, `devenv.lock`, `.prototools` and `.moon/`;
    - `.claude/` and `scripts/openspec-skills.ts`;
    - `.config/`, `.github/` and `.gitignore`;
    - `openspec/`, once its archive has moved.
- `docs/adr/0038-…` to `0040-…`: boomdrive-web's records 0001 to 0003.
- `openspec/changes/archive/2026-10-25-…` and `2026-10-27-…`: boomdrive-web's six archived changes.
- `openspec/config.yaml`: the site's context and rules.
- `.moon/workspace.yml`: the project `site`.
- `devenv.nix`:
  - bun, Node.js, Vale and Biome;
  - `BOOMDRIVE_SITE_PACKAGES`;
  - the scripts `vocab`, `prose`, `site-dev`, `site-check`, `site-serve` and `site-stop`, and the process `site`;
  - the MCP server `astro-docs`.
- `check-toolchain.sh`: bun, Node.js, Vale and Biome.
- `treefmt.nix`: Biome for the site's TypeScript, JavaScript and CSS, and the site's excludes.
- `cliff.toml`: `exclude_paths`.
- `check-releases.sh`: the releases spec's new scenarios.
- `.config/flume.toml`: the `content` kind.
- `.config/sluice.toml`: `release = "site-stop"` and the `web` port.
- `.github/workflows/pages.yml`: new.
- `.gitignore`: the site's build outputs.
- `CLAUDE.md` and `README.md`: the site and its project.
- Written with this change:
  - `docs/adr/0037-…`, the record, proposed;
  - `docs/architecture/site.md`, the area page;
  - `docs/architecture/core.md`: the opening, core.7, core.41 and core.48, and the details of core.6, core.7, core.41 and core.48.
- No crate, flake output or module changes. No runner behavior changes.
- Out of scope:
  - the opening itself: the public repository, its fresh history and the rename to `agentplot/boomdrive-archive` (records 0014 and 0025);
  - the repository's Pages source, set to GitHub Actions once by hand;
  - boomdrive-web's address pointing readers to `/boomdrive/`, and its archiving once the site publishes from here, which are boomdrive-web's.
