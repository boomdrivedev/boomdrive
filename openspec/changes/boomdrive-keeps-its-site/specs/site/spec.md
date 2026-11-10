## Purpose

Boomdrive's site, `site/`: the Method, its field notes and the fleet's measurements, an Astro site that is a moon project of its own, checked in the gate's sandbox from packages the devenv shell fetches outside it, served at `https://agentplot.github.io/boomdrive/`, and published from `main` by the workflow `pages` (records 0025 and 0037; site.1 to site.7; core.7, core.8).

## ADDED Requirements

### Requirement: The site is a moon project of its own

`.moon/workspace.yml` SHALL list the project `site` at `site/`, its language TypeScript. Each of its tasks SHALL declare its inputs, `/devenv.nix`, `/devenv.lock` and `/.prototools` among them, so a candidate that changes none of the site's inputs replays the site's checks (core.7).

#### Scenario: The project's tasks run

- **WHEN** `moon run site:check` runs from the repository's root
- **THEN** moon finds the project `site` at `site/` and runs its task there

### Requirement: Each of the site's checks is one task

The site's check-fast tasks SHALL be one check each: `check`, Astro's type check; `prose`, Vale over the site's content; `build`, the site built into `site/dist`; `test`, the site's tests; and `workflow`, actionlint over `.github/workflows/pages.yml`. Its `install` and `vocab` tasks SHALL be in no layer, run as the tasks that need them ask (core.7).

#### Scenario: The site's type check

- **WHEN** `moon run site:check` runs
- **THEN** Astro's check reports no error in the site's pages, components or content, every content reference resolving

#### Scenario: The site's build

- **WHEN** `moon run site:build` runs
- **THEN** the site is built into `site/dist`, its content's schemas and references holding

### Requirement: The site's packages are installed with no network

`site:install` SHALL install every package `site/bun.lock` locks from the packages the devenv shell fetched outside the gate's sandbox, given by `BOOMDRIVE_SITE_PACKAGES`, and SHALL reach no registry. The shell SHALL build those packages with flume's `site/packages.nix`, at the tag of flume the flake's lock names, from `site/bun.lock` and `site/package.json` (flume's record 0059).

#### Scenario: Installed in the gate's sandbox

- **WHEN** `site:install` runs in the gate's sandbox, which reaches nothing past the host
- **THEN** it installs every package `site/bun.lock` locks, and `site:check` and `site:build`, which follow it, find each

### Requirement: The site's words are held to its glossary

`site:prose` SHALL run once `site:vocab` has written Vale's vocabulary and terms from the glossary, `site/src/content/terms/terms.yaml`, and SHALL fail on any error Vale reports over `site/src/content` under `site/.vale.ini` (site.4).

#### Scenario: The prose of the site

- **WHEN** `moon run site:prose` runs
- **THEN** the vocabulary is written from the glossary first, and Vale reports no error over the site's content

### Requirement: The site is served under /boomdrive/

The site SHALL be built for the base `/boomdrive/`, its repository's name, and every link and asset of its own pages SHALL be under that base. No built page SHALL name boomdrive-web's address or repository. The site's link to its repository SHALL be `https://github.com/agentplot/boomdrive` (site.6).

#### Scenario: Every link under the base

- **WHEN** `site:test` reads every page of `site/dist`
- **THEN** each `href` and `src` that starts with `/` starts with `/boomdrive/`

#### Scenario: boomdrive-web's name is gone

- **WHEN** `site:test` reads every page of `site/dist`
- **THEN** no page holds `/boomdrive-web/` or `agentplot/boomdrive-web`, and every page that links the site's repository links `https://github.com/agentplot/boomdrive`

### Requirement: The workflow pages publishes main's site

The workflow `pages` SHALL run on a push to `main` that changes `site/`, the workflow itself, `devenv.nix`, `devenv.yaml`, `devenv.lock` or `flake.lock`, and when it is dispatched, one run at a time, none cancelled. It SHALL build the site in the devenv shell with `moon run site:build`, upload `site/dist` as the Pages artifact, and deploy it to the `github-pages` environment. It SHALL run no other check, since `main` holds only what sluice's gate passed.

#### Scenario: The workflow's trigger

- **WHEN** `site:test` reads the workflow `pages`
- **THEN** it runs on a push to `main` limited to those paths and on `workflow_dispatch`, in the concurrency group `pages` with no run cancelled

#### Scenario: The workflow's build

- **WHEN** `site:test` reads the workflow `pages`
- **THEN** it runs `devenv shell -- moon run site:build`, uploads `site/dist`, and deploys to `github-pages`

#### Scenario: The workflow's lint

- **WHEN** `moon run site:workflow` runs
- **THEN** actionlint reports nothing in `.github/workflows/pages.yml`

### Requirement: The content kind and a place's site are the repository's

The repository's `.config/flume.toml` SHALL declare the `content` kind, each of its stages' prompts a file under `site/docs/prompts/`, resolved against the declaration's own folder. `.config/sluice.toml` SHALL hold a `web` port for every checkout and name `site-stop` as the command sluice runs at a release, which SHALL exit zero where nothing was served (site.5).

#### Scenario: The kind's prompts

- **WHEN** `site:test` resolves each `prompt` of `[kinds.content]` against `.config/`
- **THEN** each names `site/docs/prompts/content-change.md` or `site/docs/prompts/content-preview.md`, and that file exists

#### Scenario: A release where nothing was served

- **WHEN** `site:test` runs `site-stop` with a state directory that holds no port
- **THEN** it exits zero
