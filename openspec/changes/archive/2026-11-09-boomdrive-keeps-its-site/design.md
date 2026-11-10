# Design

## Context

See proposal.md for why. `docs/architecture/core.md` binds every path this change touches. The new area page, `docs/architecture/site.md`, binds `site/` and `.github/workflows/pages.yml` beside it. The change relies on these rules:

- core.4: no token or credential is in the repository. The workflow `pages` deploys by GitHub's own OIDC token and holds no secret.
- core.6: nothing commercial, as its details now read for the site.
- core.7: every tracked file is owned by a formatter or excluded with its reason, and the site is a moon project of its own, one task per check.
- core.8: every tool a gate task runs is the version the tree pins.
- core.41: nothing private, as it now reads for the site's measurement and field-note pages.
- core.47: `main` holds only what sluice's gate passed, so a workflow at a push to `main` checks nothing again.
- core.48: the site's commits are in no release's notes and bump no version.
- site.1 to site.7: the site's own rules.

Record 0038 and the rules it changes are written with this change, beside this design:

- the area page, rules `site.1` to `site.7`;
- `core.md`'s opening, naming its one area page;
- core.7, core.41 and core.48, and the details of core.6, core.7, core.41 and core.48.

The change departs from no rule. It reaches no control plane, and changes no crate, flake output or module.

It is built in this repository while it is private, before the opening. Record 0025's order, as agentplot-design amends it on 2026-11-09, moves the site here first. At the opening, an agent makes the public repository's fresh root from the scrubbed tree without `site/`, and merges `site/`'s history back on, filtered to that path, by hand (agentplot/blueprints `design/principles.md` 12). That step is the opening's, and no code of this change carries it.

### What stands

- **boomdrive-web at `4ed5fbe`**, its `main` on 2026-11-09:
  - 201 commits, of which 166 are not merges, each a Conventional Commit under this repository's `.versionrc` (convco 0.7.2, run that day);
  - an Astro 7.3.5 site on bun, its content in `src/content/` under schemas in `src/content.config.ts`, served under `/boomdrive-web/` (`astro.config.mjs`, `src/lib/url.ts`);
  - its own devenv, with bun, Node.js 24, Vale 3.24.0 from Vale's release build, OpenSpec 1.14.3 and proto 0.63.1, on the same devenv-nixpkgs revision as this repository's, `c2f38fe`;
  - its own moon workspace, one project `site` at its root, whose check-fast tasks are `check` (`astro check`), `prose` (Vale, after `vocab` writes the vocabulary from the glossary), `build` and `openspec-skills`, with `install` running `bun install --frozen-lockfile` over the network;
  - its own sluice policy, with no sandbox, a `web` port for every checkout and `site-stop` as the release command;
  - its own `.config/flume.toml`, declaring the `content` kind;
  - three records, `docs/adr/0001` to `0003`, in MADR with no frontmatter;
  - four specs and six archived changes;
  - `CLAUDE.md`, its working rules, and `openspec/config.yaml`, its OpenSpec context and rules;
  - `.github/workflows/pages.yml`, which runs `devenv shell -- site-check` and deploys at each push to `main`.
- **This repository's gate** runs in sluice's sandbox, reaching nothing past the host (`.config/sluice.toml`). The devenv shell fetches what the checks need outside it: the flake's inputs, by `nix/inputs.nix`, as `BOOMDRIVE_FLAKE_INPUTS`.
- **flume's site** runs in the same kind of sandbox. Its `site/packages.nix`, a function of `pkgs`, `lib`, `lock` and `manifest`, builds bun's install cache for a lock outside the sandbox, each package fetched by the lock's integrity, GitHub packages among them. Its `site/scripts/install.sh` installs from a writable copy of it with `--offline` (flume's record 0059). The set's flume, the flake's input `flume` at `v0.20.0`, carries both.
- **The versions** devenv-nixpkgs `c2f38fe` gives: bun 1.4.2, Node.js 24.20.0 (`nodejs_24`) and Biome 2.5.13.
- **git-cliff 2.14.2** reads `[git] exclude_paths` from `cliff.toml`. A commit is left out of the notes and the bump only when every path it changes matches, as a run on a scratch repository showed on 2026-11-09.
- **yamlfmt**, which `treefmt.nix` runs on every `*.yaml` and `*.yml`, unwraps a folded scalar onto one line, and drops the spaces inside a flow mapping. The site's measurements are written with both.

## Goals / Non-Goals

**Goals:**

- The site is `site/`, with boomdrive-web's history, its rules, records, specs and content kind, built, checked and published from this repository under `/boomdrive/`.
- The site's checks run in the gate's sandbox, from tools the tree pins.
- The runner's releases see none of the site's commits.

**Non-Goals:**

- The opening: the public repository, its fresh root with `site/`'s history merged back on, the rename to `agentplot/boomdrive-archive`, and enabling the repository's Pages.
- boomdrive-web's redirect to `/boomdrive/` and its archiving, which are boomdrive-web's.
- Any change to what a page shows. Content work stays the `content` kind's (site.5).
- Biome's lint over the site's code.

## Decisions

### 1. The history is rewritten under `site/` and merged once

In a scratch clone of `https://github.com/agentplot/boomdrive-web`, `git filter-repo --to-subdirectory-filter site` rewrites every commit so that each path is under `site/`. The rewritten `main` is fetched into the branch and merged with `git merge --allow-unrelated-histories --no-ff`, as one commit:

    chore(site): take boomdrive-web's history at <commit> under site/

The merge changes nothing outside `site/`. Each commit keeps its message, author and dates; only its id changes. So `git log -- site/src/content/notes/<note>.md` reaches the commit that first published the note. The commit taken is boomdrive-web's `main` when the task runs, `4ed5fbe` at this writing, named in the merge's message.

git-filter-repo runs once, from nixpkgs (`nix run nixpkgs#git-filter-repo`). It is no gate tool, so devenv does not pin it (core.8 binds what a gate task runs).

Not taken:

- `git subtree add`, whose old commits keep their paths at the root, so a file's log under `site/` stops at the merge;
- a squash, which drops the history record 0025 keeps.

### 2. What stays in `site/`, what joins the repository's own files, and what goes

| boomdrive-web's | Here |
|---|---|
| `src/`, `public/`, `styles/`, `scripts/vocab.ts`, `.vale.ini`, `astro.config.mjs`, `package.json`, `bun.lock`, `tsconfig.json`, `README.md`, `CLAUDE.md`, `docs/` but `docs/adr/` | stay in `site/` |
| `moon.yml` | stays in `site/`, rewritten (Decision 3) |
| `docs/adr/0001` to `0003` | `docs/adr/0039` to `0041` (Decision 8) |
| `openspec/changes/archive/*` | `openspec/changes/archive/` |
| `openspec/specs/*` | this change's specs, made at its archive (Decision 9) |
| `openspec/config.yaml` | joins `openspec/config.yaml` (Decision 9) |
| `.config/flume.toml` | joins `.config/flume.toml` (Decision 10) |
| `.config/sluice.toml` | joins `.config/sluice.toml` (Decision 10) |
| `.github/workflows/pages.yml` | `.github/workflows/pages.yml`, rewritten (Decision 11) |
| `.gitignore` | joins `.gitignore` |
| `devenv.nix`, `devenv.yaml`, `devenv.lock`, `.prototools`, `.moon/` | gone: the repository's shell and moon (Decisions 4, 5 and 10) |
| `.claude/`, `scripts/openspec-skills.ts` | gone: the repository's OpenSpec skills, which `root:toolchain` checks (record 0022) |
| `openspec/changes/archive/.gitkeep`, `openspec/specs/.gitkeep` | gone |

What goes leaves the tree in task 1.4, straight after the merge (Decision 6). A later task that carries one of its parts over, Vale's hashes, a script, the content kind, the sluice policy's port and release, or the workflow, reads it from the merge of task 1.1, `git show <merge>:site/<path>`.

A commit that brings the site's files out of `site/` is `chore(site)`. It is in no notes, since record 0025 keeps the site's commits out of the runner's.

### 3. The site's project and its tasks

`.moon/workspace.yml` lists `site: 'site'`. `site/moon.yml` declares `language: 'typescript'` and these tasks. Each task runs in `site/`. Each declares `/devenv.nix`, `/devenv.lock` and `/.prototools` among its inputs, with the files it reads.

| task | command | layer | deps | notes |
|---|---|---|---|---|
| `install` | `sh scripts/install.sh` | none | | `cache: false`; inputs `package.json`, `bun.lock`, `scripts/install.sh`, `/flake.lock`, `/nix/inputs.nix` (Decision 4) |
| `vocab` | `bun run scripts/vocab.ts` | none | `install` | outputs `styles/config/vocabularies/Boomdrive/accept.txt` and `styles/Boomdrive/Terms.yml` |
| `check` | `bun run check` | check-fast | `install` | mutex `astro` |
| `prose` | `vale src/content` | check-fast | `vocab` | |
| `build` | `bun run build` | check-fast | `install` | outputs `dist`; mutex `astro` |
| `test` | `sh check-site.sh` | check-fast | `build` | inputs `dist/**/*` and the files it reads, `/.config/flume.toml`, `/.config/sluice.toml` and `/.github/workflows/pages.yml` among them |
| `workflow` | `actionlint ../.github/workflows/pages.yml` | check-fast | | |

These are boomdrive-web's checks, as its `moon.yml` has them. The changes:

- `install` reaches no registry (Decision 4).
- `test` and `workflow` are new, so that the `site` spec's scenarios are each a task's (the repository's rule for specs).
- `openspec-skills` goes, since the skills are the repository's.

`check` and `build` share the mutex `astro`, since both write `.astro/`. Each sets `ASTRO_TELEMETRY_DISABLED=1`, so Astro reads and writes nothing in the user's home, which the sandbox may refuse.

Node.js is on the shell's `PATH`, so `bun run` runs Astro's bin with that Node.js. It writes no stand-in `node` under `/tmp`, which flume's site `moon.yml` names as what bun writes where no Node.js is found.

`site/check-site.sh` is the site's tests, each scenario of the `site` spec that names `site:test`. It reads every `.html` of `site/dist`. It reads `.github/workflows/pages.yml` line by line, as `check-releases.sh` reads the workflow `release`. It resolves `[kinds.content]`'s prompts against `.config/`. It runs `site-stop` with `DEVENV_STATE` set to an empty scratch folder.

### 4. The site's packages are fetched outside the sandbox by flume's `packages.nix`

`devenv.nix` gives the shell this:

    sitePackages = import "${flakeInputs.inputs.flume}/site/packages.nix" {
      inherit pkgs lib;
      lock = ./site/bun.lock;
      manifest = ./site/package.json;
    };
    env.BOOMDRIVE_SITE_PACKAGES = "${sitePackages}";

`site/scripts/install.sh` is flume's in shape, reading `BOOMDRIVE_SITE_PACKAGES`. It fails, naming the variable, where it is unset. It copies the store's cache to a scratch folder and makes it writable, runs `bun install --frozen-lockfile --offline --cache-dir` over the copy, and removes the copy.

flume owns the way a site's packages reach a sandboxed gate (flume's record 0059). This repository calls flume's function at the set's tag and keeps no copy of it. A tag of flume that changes the function's arguments fails the shell's evaluation in the change that moves the tag, which the gate passes before the tag moves (core.45).

Not taken:

- a copy of `packages.nix` here, which would make two owners of one thing;
- `bun install` with the network, which the gate's sandbox refuses.

### 5. The shell's tools for the site

`devenv.nix` pins each of the site's tools and adds it to `packages`. A version from nixpkgs is asserted, as git's is: `bunVersion = "1.4.2"`, `nodeVersion = "24.20.0"` (`nodejs_24`), `biomeVersion = "2.5.13"`. Vale comes from Vale's release build, `valeVersion = "3.24.0"`, with boomdrive-web's four hashes, since nixpkgs trails it.

`check-toolchain.sh` checks each by its own answer: `bun --version`, `node --version`, `vale --version` and `biome --version`. `treefmt.nix` takes the same Biome package, as it takes the shell's nixfmt and deadnix.

### 6. The site's files are formatted, or excluded with their reasons

`treefmt.nix` claims the site's files as follows (core.7).

- **Biome** formats `site/**/*.ts`, `site/**/*.mjs` and `site/**/*.css`, as flume's does: spaces, two wide, lines of 100. It runs `biome format`, not flume's `biome check`. A lint of the site's code, whose recommended rules the code has never passed, would be a check of its own, not a formatter's side effect.
- **jsonfmt** formats the site's JSON, `package.json` and `tsconfig.json`, as it formats the tree's.
- **yamlfmt** formats `site/moon.yml`, as it formats the tree's `moon.yml` files.

The site's other files are excluded, each with its reason:

- `site/**/*.astro`: Biome formats only an `.astro` file's frontmatter, and only experimentally, as flume's says.
- `*.svg`: drawn diagrams, which no formatter keeps stable.
- `site/.vale.ini` and `site/styles/**`: Vale's configuration and rules, and the vocabulary and terms `site/scripts/vocab.ts` writes.
- `site/src/content/**/*.yaml`: the site's content, written by hand. yamlfmt would unwrap its folded prose and respace its points.
- `site/node_modules/**`, `site/dist/**` and `site/.astro/**`: build outputs.
- `*.md` and `*.lock`: excluded already.

The first run's changes are one commit, `style(site)`.

The devenv shell runs treefmt over the tree at each entry, `devenv:treefmt:run`, writing what it changes. Until the site's files are claimed, that run would format them under the tree's formatters: yamlfmt over the content's YAML, nixfmt and deadnix over boomdrive-web's `devenv.nix`. So task 1.4 removes what goes and claims the rest straight after the merge, and nothing enters the shell between them. Each check of tasks 1.1 to 1.4 runs through `sluice exec`, which sources the shell by `devenv print-dev-env` and runs none of its tasks.

### 7. The site's commits are left out of the notes by path

`cliff.toml`'s `[git]` gains this:

    exclude_paths = ["site/**", ".github/workflows/pages.yml"]

A commit that changes only those paths is in no notes and bumps nothing. A commit that changes the site and anything else is read as any commit. `release.sh` and `release-preview` read `cliff.toml`, so the workflow, the gate's test and the preview agree.

`check-commits.sh` is unchanged. Every commit is held to the convention, and boomdrive-web's 166 commits pass. `check-releases.sh` gains the releases spec's three new scenarios. Its scratch repositories commit a file under `site/` beside its root files.

Record 0038 records the choice, and the path not taken, a scope `site` skipped by a commit parser.

### 8. boomdrive-web's records become records 0039 to 0041

This change's record is 0038, and boomdrive-web's three take the numbers after it, in their order. `main` holds records to 0036, 0033 is another branch's, and 0037 is the hosted protocol bolt's, which lands first, as agentplot-design ruled on 2026-11-09. `root:adrs` fails where two records share a number.

| boomdrive-web's | Here | Status |
|---|---|---|
| `0001-the-site-is-flywheel.md` | `0039-the-site-is-flywheel.md` | superseded by 0040 |
| `0002-flywheel-is-renamed-boomdrive.md` | `0040-flywheel-is-renamed-boomdrive.md` | proposed |
| `0003-the-site-declares-its-mcp-servers-in-its-devenv-and-approves-them.md` | `0041-the-site-declares-its-mcp-servers-in-its-devenv-and-approves-them.md` | as its own status line says |

Each record changes in these ways, and no other:

- It gains this repository's frontmatter: `number`, `title` (its heading), `status`, `date` (its own) and `decision-makers` (the user, as each says).
- It gains one line under its heading: "Recorded in agentplot/boomdrive-web as its record 000N."
- Its links to the others name their new files.

What cites them by path or number is changed to the new numbers: `site/CLAUDE.md`, `site/docs/`, `site/src/` and `openspec/config.yaml`. The archived changes keep their text.

### 9. The four specs come through this change, and OpenSpec's config gains the site's part

boomdrive-web's `content-kind`, `method-diagrams`, `method-tools` and `tools-hub` are this change's ADDED specs. These are their changes:

- Their paths are under `site/`.
- Their address is `/boomdrive/`.
- `content-kind`'s brief is read from `site/`.
- `content-kind`'s last requirement names `site/CLAUDE.md` and the `content` kind, and no crew chore, since flume runs this repository.
- `tools-hub`'s timber card names Never prove twice and links its place, as the site has since boomdrive-web's chore `b0c23a52`, which carried the ruling of 2026-10-27 in the design note the-method-names-its-tools (agentplot/blueprints `1aeabbf`) and left boomdrive-web's spec naming Two loops, one gate.

`openspec/specs/` gains them when this change is archived. They are never written there by hand.

`openspec/config.yaml` gains the site's part, scoped to the site:

- **Context.** A paragraph saying the site is `site/`, an Astro site on bun whose content is under `site/src/content/`, served under `/boomdrive/`, its design `site/docs/site-design.md`, and its rules the area page's.
- **Rules,** each opening "For the site's specs and changes". They are boomdrive-web's `rules` and `operations.apply` guidance, with their paths under `site/`.
- **The `site` spec** names a moon task for each scenario, as the repository's rule asks of every spec.
- **The other four specs** say what a reader sees. Their scenarios are proved by a task of the site's or by what a browser shows, as boomdrive-web's rules have them.

### 10. The content kind and a place's site are the repository's

`.config/flume.toml` gains boomdrive-web's `[kinds.content]` and its two stages. Their prompts are `../site/docs/prompts/content-change.md` and `../site/docs/prompts/content-preview.md`. Its comment says the repository declares `content` beside flume's built-in kinds. Its `[profiles]` are unchanged.

The prompts change in their paths only: the brief is `site/docs/content-brief.md`. The brief says at its top that the paths it names are read from `site/`, and writes the repository's own, `.config/flume.toml` and the archived changes, from the repository's root.

`devenv.nix` gains boomdrive-web's scripts. Each runs in the repository's `site/`, and each check is the gate's own task.

| script | runs |
|---|---|
| `vocab` | `moon run site:vocab` |
| `prose` | `moon run site:prose` |
| `site-check` | `moon run site:check site:prose site:build site:test` |
| `site-dev` | `moon run site:install`, then `bun run dev` |
| `site-serve` | boomdrive-web's, printing `http://127.0.0.1:$SLUICE_PORT_WEB/boomdrive/` |
| `site-stop` | boomdrive-web's |

The process `site` runs `bun run dev` in `site/` on the port `site-serve` wrote.

`.config/sluice.toml` gains `release = "site-stop"` under `[tests]` and `[checkout] ports = ["web"]`. Its comment says what each is for.

### 11. The workflow `pages` builds in the devenv shell

`.github/workflows/pages.yml` runs as follows.

- **When.** On a push to `main` that changes `site/**`, the workflow itself, `devenv.nix`, `devenv.yaml`, `devenv.lock` or `flake.lock`, and on `workflow_dispatch`. `flake.lock` names flume's tag, whose `packages.nix` fetches the site's packages.
- **Permissions.** `contents: read`, `pages: write` and `id-token: write`.
- **One run at a time.** The concurrency group `pages`, with `cancel-in-progress: false`. A deploy cut short can leave the site half published, as boomdrive-web's says.
- **`build`** runs the following on `ubuntu-24.04`:
  - checkout;
  - Nix, with the `devenv` cache, as release's;
  - `nix profile add nixpkgs#devenv`;
  - `devenv shell -- moon run site:build`;
  - `actions/upload-pages-artifact` of `site/dist`.
- **`deploy`** runs `actions/deploy-pages` in the environment `github-pages`.

Every action is pinned by its commit, its version in a comment, as `release.yml` and `cache.yml` pin theirs.

It runs no check: `main` holds only what the gate passed (core.47). `site:workflow` lints it. `site:test` reads its trigger, its concurrency, its build and its deploy.

The shell also substitutes the set's flume and sluice from the family's cache, which a Pages run does not use.

Not taken: `oven-sh/setup-bun` with its own `bun-version`, as flume's and sluice's workflows do. That is a second pin of bun with no check that the two agree, which core.8 rules out.

### 12. The site's address and links

These name boomdrive rather than boomdrive-web:

- `site/astro.config.mjs`'s `base` is `/boomdrive`.
- `site/src/lib/url.ts`'s comment names `/boomdrive/`.
- `site/src/layouts/Base.astro`'s repository link is `https://github.com/agentplot/boomdrive`, and its label names boomdrive.
- `site/package.json`'s name is `boomdrive-site`, as sluice's site is `sluice-site`, and `site/bun.lock`'s name follows.
- Every mention of boomdrive-web as the site's repository or address in `site/docs/` and `site/README.md` becomes boomdrive's.

The records, the archived changes and the field notes keep their text (site.3).

The family's entries `@agentplot/site-theme` draws on every page, its Boomdrive member's `site` and its `hub`, stay as the theme gives them, `/boomdrive-web/` at v0.1.3, and the site does not override them (record 0040). They move at the opening, by a site-theme release naming the new address, which the opening carries (agentplot-design, 2026-11-09). `site:test` reads them from the theme, so what it holds the site to is everything else the site writes.

### 13. The site's working rules

These are written with this change:

- `docs/architecture/site.md` holds rules `site.1` to `site.6`, from boomdrive-web's `CLAUDE.md` and `openspec/config.yaml`, and `site.7`, where our own fleet's names may stand, each with its details.
- core.6's details, core.7, core.41, core.48 and `core.md`'s opening name the site.
- Record 0038, proposed, gives the reasons.

`site/CLAUDE.md` is boomdrive-web's, for the site:

- It cites `site.1` to `site.7`.
- It names the repository's devenv and gate, and `site-check`.
- It says the workflow `pages` publishes `main`'s site at each push that changes it.
- It drops its bare layout's line, its own devenv's MCP and OpenSpec lines, which are the repository's (record 0022), and the crew chore.

Claude Code reads it when it reads a file under `site/`.

The root `CLAUDE.md` says what the site is, where it is served, and that its rules are the area page's and `site/CLAUDE.md`'s. It names the project `site` beside the others, and the site's checks in the `check-fast` row. `README.md`'s "Rules and checks" names `site` beside the other projects.

### 14. The site's MCP server joins the repository's

`devenv.nix`'s `claude.code.mcpServers` gains `astro-docs`, `https://mcp.docs.astro.build/mcp`, beside `mcp.devenv.sh`. It is approved by name as every server is, and its comment says what it serves: Astro's documentation for the release `site/package.json` pins (records 0022 and 0025).

### 15. Our own fleet's names stand on the case study alone

`site.7` holds the line. Our own drives and machines stay named, as ours, where the site shows our fleet as a case study, and nowhere else of the site:

| Where | Names of ours |
|---|---|
| `site/src/content/measurements/`, `site/src/content/contexts/`, `site/src/content/notes/` and the pages built from them | stay, as boomdrive-web published them |
| The history task 1.1 merges, the agents named after a drive in its messages among them | stays, as published (record 0025) |
| `site/docs/measurement-roadmap.md`'s examples: `host: mac-studio` and `drive: madswan` | take this repository's neutral names, `host: host-a` and `drive: home` |
| `site/docs/site-design.md`'s example of a split's name, `bolts open, madswan` | takes `bolts open, home` |
| `openspec/changes/archive/2026-10-25-the-content-kind-and-its-brief/proposal.md`'s signal, cited by an id holding a drive's name | is cited as the operator's signal of 2026-10-26, with no id, as record 0022's deciders cite theirs |
| MadSwan named as the company, in records 0039 and 0040, `site/docs/site-design.md` and `site/docs/audience-plan.md` | stays: a company is no drive or machine |

The guides, reference, glossary and quick start hold none of ours today, and `site.7` keeps them so. No check lists our names, since a check would have to hold them. The rule is a review's.

## Risks / Trade-offs

- **The site's history at the opening.** Task 1.1's history is part of this repository's history, which the public repository does not take. The opening keeps it by its by-hand step, its fresh root from the scrubbed tree without `site/` with `site/`'s history merged back on. A fresh root made of the whole tree would drop it.
- **Pages from a private repository.** The organization's plan publishes Pages only from a public repository. Until the opening, the workflow's deploy fails at each push that changes the site, which is expected; enabling Pages is part of the opening. boomdrive-web keeps publishing until the site publishes from here, and is archived then.
- **boomdrive-web moving after the merge.** Task 1.1 takes boomdrive-web's `main` as it stands. boomdrive-web takes no new work from then on, which the orchestrator holds.
- **The shell grows.** It now holds bun, Node.js, Vale, Biome and the site's packages, each fetched once per lock and substituted from caches after.
- **flume's `packages.nix` at the set's tag.** A flume release that changes the function changes the site's install with it. The gate catches a break in the change that moves the tag (core.45).
- **The gate's time.** `site:install` runs at each gate, `cache: false`, copying the cache and installing offline in seconds. The site's checks replay wherever their inputs are unchanged.

