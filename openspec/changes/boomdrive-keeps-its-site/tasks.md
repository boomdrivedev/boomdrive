# Tasks

Build the tasks in order. A task is true once the check it names passes, run as it names it. The groups are headings, for reading.

- Record 0037, the area page `docs/architecture/site.md`, and the amendments to `core.md` are already written with the change. A task that relies on a choice the design names cites its Decision.
- Each tool a gate task runs is pinned in `devenv.nix` and checked by `root:toolchain` (core.8). Each file type the tree gains is claimed by a formatter or excluded with its reason in `treefmt.nix` (core.7); task 1.4 does that for the site.
- From task 1.1 until task 1.4 is committed, nothing is run through `devenv shell`, whose own treefmt run formats the tree at each entry before the site's files are claimed. Each check runs through `sluice exec`, as its task names it, which sources the shell and runs none of its tasks (design Decision 6).
- What boomdrive-web held and the site drops in task 1.4, its devenv, its sluice and flume policies and its workflow among them, is read from the merge of task 1.1, `git show <merge>:site/<path>`, by the tasks that carry it over.
- A field note's words, figures and date are not changed (site.3). No task changes what a page shows.

## 1. The site's history and its place in the tree

- [x] 1.1 boomdrive-web's history is merged under `site/`, as design Decision 1 has it:
  - In a scratch clone of `https://github.com/agentplot/boomdrive-web`, its `main` is rewritten by `nix run nixpkgs#git-filter-repo -- --to-subdirectory-filter site`.
  - The rewritten `main` is merged into this branch with `git merge --allow-unrelated-histories --no-ff`, as one commit, `chore(site): take boomdrive-web's history at <commit> under site/`, naming the commit taken, `4ed5fbe` at this writing.
  - The merge changes nothing outside `site/`. `git log --format=%ad -- site/src/content/notes/what-our-fleet-re-proves.md` reaches the commit that first published the note.

  True when `sluice exec main -- moon run root:commits` passes, holding the imported commits to the convention.
- [x] 1.2 `site/docs/adr/0001` to `0003` are moved by `git mv` to `docs/adr/0038` to `0040`, as design Decision 8 has them:
  - each gains this repository's frontmatter, and its line naming its number in boomdrive-web;
  - each one's links to the others name their new files;
  - `site/CLAUDE.md`, `site/docs/` and `site/src/` cite them by their new numbers.

  True when `sluice exec main -- moon run root:adrs` passes.
- [x] 1.3 boomdrive-web's six archived changes are moved by `git mv` from `site/openspec/changes/archive/` to `openspec/changes/archive/`, and `site/openspec/` is removed. `2026-10-25-the-content-kind-and-its-brief/proposal.md` cites its signal as the operator's signal of 2026-10-26, with no id (design Decision 15). `openspec/config.yaml` gains the site's part, as design Decision 9 has it: its context's paragraph, and its rules and `operations.apply` guidance, each scoped to the site's specs and changes. True when `sluice exec main -- moon run root:changes` passes.
- [x] 1.4 What the repository replaces leaves `site/`, and the site's files are claimed (design Decisions 2 and 6):
  - These are removed: `site/devenv.nix`, `site/devenv.yaml`, `site/devenv.lock`, `site/.prototools`, `site/.moon/`, `site/.claude/`, `site/scripts/openspec-skills.ts`, `site/.config/`, `site/.github/` and `site/.gitignore`.
  - The root `.gitignore` gains `site/node_modules/`, `site/dist/` and `site/.astro/`.
  - `devenv.nix` pins Biome `2.5.13`, asserted as git is, in `packages`.
  - `treefmt.nix` runs the same Biome's `format` over `site/**/*.ts`, `site/**/*.mjs` and `site/**/*.css`, and excludes the rest of Decision 6, each exclude with its comment.

  The changes treefmt then makes to the site's files are one commit, `style(site)`. True when `sluice exec main -- moon run root:format` passes.

## 2. The site's tools and its project

- [x] 2.1 `devenv.nix` pins and adds to `packages`:
  - bun `1.4.2` and Node.js `24.20.0` (`nodejs_24`), each asserted as git is;
  - Vale `3.24.0` from Vale's release build, with boomdrive-web's four hashes.

  `check-toolchain.sh` checks bun, Node.js, Vale and Biome each by `--version`, and its comment names them (design Decision 5). True when `sluice exec main -- moon run root:toolchain` passes.
- [x] 2.2 The site's packages install with no network (design Decision 4):
  - `devenv.nix` gives `BOOMDRIVE_SITE_PACKAGES` from flume's `site/packages.nix` at the flake's `flume` input, called with `site/bun.lock` and `site/package.json`.
  - `site/scripts/install.sh` installs from a writable copy of it, `--offline`, and fails naming the variable where it is unset.
  - `.moon/workspace.yml` lists `site: 'site'`, and its comment names the project. `site/moon.yml` is rewritten with `language: 'typescript'` and the task `install`, as Decision 3's table has it.
  - `site/package.json`'s name is `boomdrive-site`, and `site/bun.lock`'s name follows.

  True when `sluice exec main -- moon run site:install` passes in the gate's sandbox.
- [x] 2.3 `site/moon.yml` gains `check`, as Decision 3's table has it, with `ASTRO_TELEMETRY_DISABLED=1` and the mutex `astro`. True when `sluice exec main -- moon run site:check` passes, proving the site spec's "The site's type check".
- [x] 2.4 `site/moon.yml` gains `vocab` and `prose`, as Decision 3's table has it. True when `sluice exec main -- moon run site:prose` passes, proving the site spec's "The prose of the site".
- [ ] 2.5 The site's address is `/boomdrive/` (design Decision 12):
  - `site/astro.config.mjs`'s `base` is `/boomdrive`, and its comment and `site/src/lib/url.ts`'s name `/boomdrive/`.
  - `site/src/layouts/Base.astro` links `https://github.com/agentplot/boomdrive`, its label naming boomdrive.
  - `site/moon.yml` gains `build`, as Decision 3's table has it.

  True when `sluice exec main -- moon run site:build` passes, proving the site spec's "The site's build".
- [ ] 2.6 `site/check-site.sh` reads every `.html` of `site/dist`. It fails, naming the page and the link, on an `href` or `src` that starts with `/` but not `/boomdrive/`. It fails on a page holding `/boomdrive-web/` or `agentplot/boomdrive-web`, or linking a repository but `https://github.com/agentplot/boomdrive`. `site/moon.yml` gains `test`, as Decision 3's table has it. True when `sluice exec main -- moon run site:test` passes, proving the site spec's "Every link under the base" and "boomdrive-web's name is gone".

## 3. Publishing

- [ ] 3.1 `.github/workflows/pages.yml` is design Decision 11's, each action pinned by its commit with its version in a comment. `site/moon.yml` gains `workflow`, as Decision 3's table has it. True when `sluice exec main -- moon run site:workflow` passes, proving the site spec's "The workflow's lint".
- [ ] 3.2 `site/check-site.sh` reads `.github/workflows/pages.yml` line by line, as `check-releases.sh` reads the workflow `release`. It checks the trigger, the push to `main` limited to Decision 11's paths and `workflow_dispatch`, and the concurrency group `pages` with `cancel-in-progress: false`. It checks that the build runs `devenv shell -- moon run site:build` and uploads `site/dist`, and that the deploy is to `github-pages`. True when `sluice exec main -- moon run site:test` passes, proving the site spec's "The workflow's trigger" and "The workflow's build".

## 4. Content work and a place's site

- [ ] 4.1 `.config/flume.toml` declares `[kinds.content]` and its two stages as boomdrive-web's did, their prompts `../site/docs/prompts/content-change.md` and `../site/docs/prompts/content-preview.md`, and its comment says it declares `content` beside flume's built-in kinds. The prompts name `site/docs/content-brief.md`. The brief says at its top that its paths are read from `site/`, and writes the repository's own from its root (design Decision 10). `site/check-site.sh` resolves each `prompt` of `[kinds.content]` against `.config/` and fails where it names no file under `site/docs/prompts/`. True when `sluice exec main -- moon run site:test` passes, proving the site spec's "The kind's prompts".
- [ ] 4.2 `devenv.nix` gains the scripts `vocab`, `prose`, `site-check`, `site-dev`, `site-serve` and `site-stop`, and the process `site`, as design Decision 10's table has them. `site-serve` prints the address under `/boomdrive/`.

  `.config/sluice.toml` gains `release = "site-stop"` under `[tests]` and `[checkout] ports = ["web"]`, each with its comment.

  `site/check-site.sh` runs `site-stop` with `DEVENV_STATE` an empty scratch folder and fails unless it exits zero.

  True when `sluice exec main -- moon run site:test` passes, proving the site spec's "A release where nothing was served".
- [ ] 4.3 `devenv.nix`'s `claude.code.mcpServers` gains `astro-docs`, `https://mcp.docs.astro.build/mcp`, approved by name with the others, and its comment names what it serves (design Decision 14). True when `sluice exec main -- moon run root:toolchain` passes, the repository's OpenSpec skills still its pinned OpenSpec's.

## 5. Releases

- [ ] 5.1 `cliff.toml`'s `[git]` gains `exclude_paths = ["site/**", ".github/workflows/pages.yml"]`, with a comment citing core.48 and record 0037. `check-releases.sh` makes a commit that changes a file under `site/` alone, and one that changes a file under `site/` and one outside it. It gains the releases spec's three new scenarios (design Decision 7). True when `sluice exec main -- moon run root:releases` passes, proving the releases spec's "A change to the site alone", "A change to the site and the runner" and "The site's commit in no notes".

## 6. What the repository says of the site

- [ ] 6.1 `site/CLAUDE.md` is the site's working rules, as design Decision 13 has it, citing `site.1` to `site.7`. `site/README.md` and `site/docs/` name the site's repository and address as boomdrive's (design Decision 12). The root `CLAUDE.md` says what the site is and where its rules are, names the project `site`, and names the site's checks in the `check-fast` row. `README.md`'s "Rules and checks" names `site`. True when `sluice exec main -- moon run root:format` passes.
- [ ] 6.2 Our own fleet's names stand only where design Decision 15 keeps them (`site.7`). `site/docs/measurement-roadmap.md`'s examples name `host: host-a` and `drive: home`, and `site/docs/site-design.md`'s example split is `bolts open, home`. No file under `site/` but `site/src/content/measurements/`, `site/src/content/contexts/` and `site/src/content/notes/` names a drive or a machine of ours. True when `sluice exec main -- moon run root:format` passes.
