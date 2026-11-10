# Verify: boomdrive-keeps-its-site

Verified on 2026-11-09 against `change/boomdrive-keeps-its-site` at
`cc855358`.

**Verdict: archived.** The repository's whole checks pass. All 21 tasks
are ticked and hold in the tree. Every requirement and scenario of the
change's six specs is met. Archived, the change writes specs that
`root:specs` passes.

## The whole checks

`sluice exec main`, run once on `cc855358`, exited 0: "passed: 33 tasks,
recorded 33 results under their keys for cc85535".

| project | tasks passed |
|---|---|
| `boomdrive`, `boomdrive-control`, `boomdrive-crew`, `boomdrive-hosted` | format, lint, typecheck, test, each |
| `nix` | format, lint, typecheck, test |
| `root` | format, toolchain, adrs, specs, changes, commits, releases, workflow |
| `site` | install, vocab, check, prose, build, test, workflow |

The gate printed these lines for the change's checks:

- `root:changes`: 1 passed, 0 failed.
- `root:specs`: 17 passed, 0 failed, on the specs as they stand before the archive.
- `root:commits`: no errors in 200 commits.
- `root:format`: 244 files, 0 changed.
- `root:releases`, the releases spec's new scenarios:
  - A feature that changes only `site/` after `v0.1.0` releases nothing.
  - A fix that changes `site/` and a file outside it after `v0.1.0` releases `v0.1.1`.
  - The notes hold the fix under Fixes and nothing of the site's page.
- `site:check`: Astro's check, 31 files, 0 errors, 0 warnings.
- `site:prose`: Vale, 0 errors, 0 warnings and 0 suggestions in 12 files.
- `site:build`: 11 pages built.
- `site:test`:
  - 11 pages, every link under `/boomdrive/`, and none of boomdrive-web's but the theme's.
  - The workflow `pages` runs on a push to `main` that changes the site, its shell or `flake.lock`, and when dispatched.
  - It runs one at a time, none cancelled.
  - It builds the site in the devenv shell, uploads `site/dist` and deploys to `github-pages`.
  - It runs no check.
  - Each content kind prompt is under `site/docs/prompts/`.
  - `site-stop` exits zero where nothing was served.

moon warned that it could not write its output archives and shared cache
from inside the sandbox. No task failed on it.

## The archive's specs

Task 5.2's proof was run in a scratch worktree of `cc855358`:

1. `openspec archive boomdrive-keeps-its-site --yes` applied 33 additions,
   with nothing modified, removed or renamed.
2. `openspec validate --specs --strict --no-interactive`, the command of
   `root:specs`, then gave "Totals: 22 passed, 0 failed (22 items)".
3. `treefmt --ci` changed nothing.

The `releases` spec it writes keeps its four requirements as `main` holds
them and adds a fifth, "A commit that changes only the site releases
nothing". That requirement's text is 349 characters, within openspec's
500. The change validates under `--strict`.

## Tasks

All 21 tasks are ticked. Each was checked against the tree:

- **1.1**: The merge `64a576df` takes boomdrive-web at `4ed5fbe`. It changes nothing outside `site/`. The history of `site/src/content/notes/what-our-fleet-re-proves.md` reaches boomdrive-web's first commit, of 2026-10-05. `root:commits` passes.
- **1.2, 6.4**: This change's record is 0038, and boomdrive-web's records are 0039 to 0041. Each file's name and frontmatter `number` give it. Each of 0039 to 0041 says "Recorded in agentplot/boomdrive-web as its record 000N." No tracked file remains under `site/docs/adr/`. `root:adrs` passes.
- **1.3**: The six archived changes are under `openspec/changes/archive/`, and no tracked file is under `site/openspec/`. The content kind's proposal cites "The operator's signal of 2026-10-26" with no id. `openspec/config.yaml` holds the site's rules and `operations.apply` guidance, each scoped "For the site's specs and changes".
- **1.4**: Every path Decision 2 drops is gone. `.gitignore` names `site/node_modules/`, `site/dist/` and `site/.astro/`. `treefmt.nix` runs Biome `format` over the site's TypeScript, JavaScript and CSS. `root:format` passes.
- **2.1**: `devenv.nix` pins bun 1.4.2, Node.js 24.20.0 (`nodejs_24`, asserted), Biome 2.5.13 and Vale 3.24.0. `check-toolchain.sh` checks all four by `--version`, and its comment names them. `root:toolchain` passes.
- **2.2 to 2.6**: These match Decision 3's table:
  - `BOOMDRIVE_SITE_PACKAGES` comes from flume's `site/packages.nix` at the flake's `flume` input.
  - `site/scripts/install.sh` installs `--offline` from a writable copy, and fails naming the variable where it is unset.
  - `.moon/workspace.yml` lists `site: 'site'`, and its comment names the project.
  - `site/moon.yml` declares `language: 'typescript'` and the tasks `install`, `check`, `vocab`, `prose`, `build`, `test` and `workflow`. `check` and `build` share the mutex `astro` and run with `ASTRO_TELEMETRY_DISABLED`.
  - The package name is `boomdrive-site`, in `package.json` and `bun.lock`.
  - The base is `/boomdrive`, `url.ts` names `/boomdrive/`, and `Base.astro` links `https://github.com/agentplot/boomdrive`.
  - `site:install`, `site:check`, `site:prose`, `site:build` and `site:test` pass in the gate.
- **3.1, 3.2**: `.github/workflows/pages.yml` runs on a push to `main` and on `workflow_dispatch`, in the concurrency group `pages` with `cancel-in-progress: false`. It builds with `devenv shell -- moon run site:build`, uploads `site/dist` and deploys to `github-pages`. Each action is pinned by commit, with its version in a comment. `site:workflow` and `site:test` pass.
- **4.1**: `.config/flume.toml` declares `[kinds.content]` with the stages `change` and `preview`, their prompts `../site/docs/prompts/content-change.md` and `../site/docs/prompts/content-preview.md`. The brief's first lines say its paths are read from `site/`, but the repository's own, which are read from its root. Every path it names exists.
- **4.2**: `devenv.nix` holds the site's scripts and the process `site`. `.config/sluice.toml` holds `release = "site-stop"` and `[checkout] ports = ["web"]`, each with its comment. `site:test` runs `site-stop` where nothing was served.
- **4.3**: `devenv.nix` declares `astro-docs`, `https://mcp.docs.astro.build/mcp`, with a comment naming what it serves. `root:toolchain` passes.
- **5.1**: `cliff.toml`'s `[git]` has `exclude_paths = ["site/**", ".github/workflows/pages.yml"]`, its comment citing core.48 and record 0038. `check-releases.sh` covers the three new scenarios, and `root:releases` passes.
- **5.2**: The `releases` delta adds one requirement, "A commit that changes only the site releases nothing", holding the three scenarios, and modifies none. `root:changes` passes here, and `root:specs`'s command passes once the change is archived (see "The archive's specs").
- **6.1**: The root `CLAUDE.md` says what the site is and where its rules are, names the project `site`, and names the site's checks in the `check-fast` row. `README.md`'s "Rules and checks" names `site`. `site/CLAUDE.md` cites `site.1` to `site.7`.
- **6.2**: `measurement-roadmap.md` uses `host: host-a` and `drive: home`. `site-design.md`'s example split is `bolts open, home`. MadSwan stands only as the company, as Decision 15 keeps it.
- **6.3**: The `tools-hub` spec's scenarios "The cards as built" and "Reading timber's card" give timber's card Never prove twice, linking that principle's place on the Method page. That is what the site shows: `site/src/pages/tools.astro` maps `timber: "never-prove-twice"`.

## Specs

- **`site`**: every requirement and scenario is met, each proved by the site's task the gate ran.
- **`releases`**: the added requirement and its three scenarios are met (`root:releases`). Archived, the spec passes `openspec validate --strict`.
- **`content-kind`**: met.
  - The declaration and its prompts are as the spec has them.
  - Every path the brief names exists.
  - The brief names its proof.
  - `site-stop` exits zero where nothing was served.
  - `site/CLAUDE.md` sends content work to the brief and the `content` kind.
- **`method-tools`**: met. `site:check` and `site:build` hold the principle strips' schemas.
- **`method-diagrams`**: carried as boomdrive-web held it, its paths under `site/`. No task of this change changes a diagram, and the site builds.
- **`tools-hub`**: met. Timber's card names Never prove twice, as the site shows it.

## Not blocking

- The comment in `.moon/workspace.yml` lists the projects, `site` among them, but its `check-fast` paragraph names only the crates', the flake's and the root's checks. The root `CLAUDE.md`'s `check-fast` row names the site's. No task asks for the comment.
