# Verify: boomdrive-keeps-its-site

Verified on 2026-11-09 against `change/boomdrive-keeps-its-site` at
`486f9f75`.

**Verdict: not archived.** The repository's whole checks pass, and every
task is ticked and holds. One spec this change adds, `tools-hub`, gives
timber's card a principle the site does not show. Archiving it would write
into `openspec/specs/` a requirement the site does not meet.

## The whole checks

`sluice exec main`, run once on `486f9f75`, exited 0: "passed: 33 tasks,
recorded 33 results under their keys for 486f9f7".

| project | tasks passed |
|---|---|
| `boomdrive`, `boomdrive-control`, `boomdrive-crew`, `boomdrive-hosted` | format, lint, typecheck, test, each |
| `nix` | format, lint, typecheck, test |
| `root` | format, toolchain, adrs, specs, changes, commits, releases, workflow |
| `site` | install, vocab, check, prose, build, test, workflow |

The site's lines, as the gate printed them:

- `site:check`: Astro's check, 31 files, 0 errors, 0 warnings, 0 hints.
- `site:prose`: Vale, 0 errors, 0 warnings and 0 suggestions in 12 files.
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

## Tasks

All 18 tasks are ticked. Each was checked against the tree:

- **1.1**: The merge `64a576df` takes boomdrive-web at `4ed5fbe`. It changes nothing outside `site/`. `root:commits` passes.
- **1.2**: Records 0038 to 0040 carry this repository's frontmatter and the line "Recorded in agentplot/boomdrive-web as its record 000N.". Their links name the new files. `site/CLAUDE.md` and `site/docs/` cite them by their new numbers. No tracked file remains under `site/docs/adr/`.
- **1.3**: The six archived changes are under `openspec/changes/archive/`, and `site/openspec/` is gone. The content kind's proposal cites "the operator's signal of 2026-10-26" with no id. `openspec/config.yaml` has the site's context, rules and `operations.apply` guidance, each scoped "For the site's specs and changes".
- **1.4**: Every path Decision 2 drops is gone. `.gitignore` names the site's three build outputs. `treefmt.nix` has Biome `format` over the site's `.ts`, `.mjs` and `.css`, and excludes each of the other site files Decision 6 names, with its reason.
- **2.1**: `devenv.nix` pins and asserts these, and `check-toolchain.sh` checks all four:
  - bun 1.4.2;
  - Node.js 24.20.0;
  - Biome 2.5.13;
  - Vale 3.24.0, from its release with four hashes.
- **2.2 to 2.6**: These match Decision 3's table:
  - `BOOMDRIVE_SITE_PACKAGES` comes from flume's `site/packages.nix`. `site/scripts/install.sh` installs `--offline` from a writable copy, and fails naming the variable.
  - `.moon/workspace.yml` lists `site`.
  - `site/moon.yml` declares the tasks with their layers, deps, inputs, the mutex `astro` and `ASTRO_TELEMETRY_DISABLED`.
  - The package name is `boomdrive-site`.
  - The base is `/boomdrive`, and `Base.astro` links `https://github.com/agentplot/boomdrive`.
- **3.1, 3.2**: `.github/workflows/pages.yml` is Decision 11's. Each action is pinned by commit, with its version in a comment. `site:test` reads its trigger, concurrency, build and deploy.
- **4.1**: `.config/flume.toml` declares `[kinds.content]`, with its prompts under `../site/docs/prompts/`. The brief's first lines say its paths are read from `site/`, but the repository's own, which are read from its root. Every path the brief names exists.
- **4.2**: The scripts and the process `site` are Decision 10's. `.config/sluice.toml` holds `release = "site-stop"` and `[checkout] ports = ["web"]`, each with a comment.
- **4.3**: `astro-docs` is declared and approved by name through `enabledMcpjsonServers`.
- **5.1**: `cliff.toml` has `exclude_paths`. `check-releases.sh` covers "A change to the site alone", "A change to the site and the runner" and "The site's commit in no notes", and `root:releases` passes.
- **6.1**: The root `CLAUDE.md`, `README.md` and `site/CLAUDE.md` say what the site is and cite `site.1` to `site.7`.
- **6.2**: `measurement-roadmap.md` uses `host: host-a` and `drive: home`, and `site-design.md` uses `bolts open, home`. Outside `site/src/content/{measurements,contexts,notes}/`, no file under `site/` names a drive or machine of ours. MadSwan stands only as the company, in `site-design.md` and `audience-plan.md`, as Decision 15 keeps it.

## Specs

- **`site`**: every requirement and scenario is met, each proved by the site's task that the gate ran.
- **`releases`**: the modified requirements and their three new scenarios are met (`root:releases`).
- **`content-kind`**: met.
  - The declaration matches, each prompt resolves under `site/docs/prompts/`, and the brief's paths exist.
  - The brief names its proof: `site-check`, `sluice exec`, Vale, and a look in light and dark, at phone width, with scripts off and under reduced motion.
  - `site-serve` prints the address under `/boomdrive/`, and `site-stop` exits zero where nothing was served.
  - `site/CLAUDE.md` sends content work to the brief and the `content` kind.
- **`method-tools`**: met. Each principle's strip in `site/src/content/principles/` names the tools the scenario "The tools each principle names" lists. `site:check` and `site:build` hold the strips' schemas.
- **`method-diagrams`**: carried as boomdrive-web held it, its paths under `site/`. No task of this change changes a diagram, and the site builds.
- **`tools-hub`**: **not met.** See the finding below.

## Finding: the tools-hub spec gives timber's card the wrong principle

`openspec/changes/boomdrive-keeps-its-site/specs/tools-hub/spec.md` has
timber's card carry Two loops, one gate:

- line 36, scenario "The cards as built": "and timber names Two loops, one gate";
- line 50, scenario "Reading timber's card": "\"In the Method\" naming Two loops, one gate and linking that principle's place on the Method page".

The site gives timber's card **Never prove twice**:

- `site/src/pages/tools.astro` maps `timber: "never-prove-twice"`.
- The built `site/dist/tools/index.html` links timber's card to `/boomdrive/method/#never-prove-twice`.
- The glossary's `timber` term has `principle: never-prove-twice`.

This is a ruling, not a slip in the site. boomdrive-web's commit `b0c23a52`, "chore: tailrace's card and term carry Never prove twice", cites the design note the-method-names-its-tools (agentplot/blueprints `1aeabbf`, ruled 2026-10-27). That note gives the card Never prove twice, the principle its own line states and whose strip it heads. Two loops, one gate keeps timber in its strip.

That chore changed the site and not boomdrive-web's spec, so the spec was already stale at `4ed5fbe`. This change carried it over as it stood. Decision 9 lists the four specs' changes and does not mention this one.

**What must change:** update the delta spec's two scenarios to say timber's card names Never prove twice and links that principle's place. Leave the site unchanged, since no task of this change changes what a page shows. Then this report can pass and the change can be archived.

## Not blocking

- The comment in `.moon/workspace.yml` lists the projects, `site` among them, but its `check-fast` paragraph lists only the crates', the flake's and the root's checks, not the site's. The root `CLAUDE.md`'s `check-fast` row does name them. No task asks for the comment.
