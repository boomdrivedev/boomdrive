# Verify: boomdrive-keeps-its-site

Verified on 2026-11-09 against `change/boomdrive-keeps-its-site` at
`58e9929b`.

**Verdict: not archived.** The repository's whole checks pass. All 20
tasks are ticked and hold in the tree, and every requirement and scenario
of the change's six specs is met. But archiving the change writes a
`releases` spec that `root:specs` refuses: its two modified requirements
are too long for `openspec validate --strict`. See the finding below.

## The whole checks

`sluice exec main`, run once on `58e9929b`, exited 0: "passed: 33 tasks,
recorded 33 results under their keys for 58e9929".

| project | tasks passed |
|---|---|
| `boomdrive`, `boomdrive-control`, `boomdrive-crew`, `boomdrive-hosted` | format, lint, typecheck, test, each |
| `nix` | format, lint, typecheck, test |
| `root` | format, toolchain, adrs, specs, changes, commits, releases, workflow |
| `site` | install, vocab, check, prose, build, test, workflow |

The site's lines, as the gate printed them:

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

## Tasks

All 20 tasks are ticked. Each was checked against the tree:

- **1.1**: The merge `64a576df` takes boomdrive-web at `4ed5fbe`. It changes nothing outside `site/`. The history of `site/src/content/notes/what-our-fleet-re-proves.md` reaches boomdrive-web's first commit, of 2026-10-05. `root:commits` passes.
- **1.2, 6.4**: boomdrive-web's records are 0039 to 0041, and this change's is 0038. Each file's name and frontmatter `number` give it. Each of 0039 to 0041 says "Recorded in agentplot/boomdrive-web as its record 000N.", and 0039 and 0040 link each other by their new files. Every citation in `core.md`, `site.md`, the root's files, `site/` and the change's design, proposal, tasks and specs names them by these numbers. No tracked file remains under `site/docs/adr/`. `main` holds records to 0036, so none of these numbers clashes with it.
- **1.3**: The six archived changes are under `openspec/changes/archive/`, and `site/openspec/` is gone. The content kind's proposal cites "the operator's signal of 2026-10-26" with no id. `openspec/config.yaml` holds the site's context paragraph, and its rules and `operations.apply` guidance, each scoped "For the site's specs and changes".
- **1.4**: Every path Decision 2 drops is gone. `.gitignore` names `site/node_modules/`, `site/dist/` and `site/.astro/`. `treefmt.nix` runs Biome `format` over the site's `.ts`, `.mjs` and `.css`, and excludes each other site file Decision 6 names, with its reason.
- **2.1**: `devenv.nix` pins and asserts bun 1.4.2, Node.js 24.20.0 and Biome 2.5.13, and builds Vale 3.24.0 from its release with four hashes. `check-toolchain.sh` checks all four by `--version`, and its comment names them.
- **2.2 to 2.6**: These match Decision 3's table:
  - `BOOMDRIVE_SITE_PACKAGES` comes from flume's `site/packages.nix` at the flake's `flume` input, called with `site/bun.lock` and `site/package.json`.
  - `site/scripts/install.sh` installs `--offline` from a writable copy, and fails naming the variable where it is unset.
  - `.moon/workspace.yml` lists `site`, and its comment names the project.
  - `site/moon.yml` declares `language: 'typescript'` and each task with its layer, deps and inputs, `/devenv.nix`, `/devenv.lock` and `/.prototools` among them. `check` and `build` share the mutex `astro` and run with `ASTRO_TELEMETRY_DISABLED`.
  - The package name is `boomdrive-site`, in `package.json` and `bun.lock`.
  - The base is `/boomdrive`, `url.ts` names `/boomdrive/`, and `Base.astro` links `https://github.com/agentplot/boomdrive`, labelled "boomdrive on GitHub".
- **3.1, 3.2**: `.github/workflows/pages.yml` is Decision 11's. Each action is pinned by commit, with its version in a comment. `site:workflow` lints it, and `site:test` reads its trigger, concurrency, build and deploy.
- **4.1**: `.config/flume.toml` declares `[kinds.content]` with its two stages, their prompts under `../site/docs/prompts/`, and its comment says it declares `content` beside flume's built-in kinds. The brief's first lines say its paths are read from `site/`, but the repository's own, which are read from its root.
- **4.2**: `devenv.nix` holds the scripts `vocab`, `prose`, `site-check`, `site-dev`, `site-serve` and `site-stop`, and the process `site`. `site-serve` prints the address under `/boomdrive/`. `.config/sluice.toml` holds `release = "site-stop"` and `[checkout] ports = ["web"]`, each with a comment.
- **4.3**: `astro-docs`, `https://mcp.docs.astro.build/mcp`, is declared beside devenv's server and approved by name through `enabledMcpjsonServers`.
- **5.1**: `cliff.toml`'s `[git]` has `exclude_paths = ["site/**", ".github/workflows/pages.yml"]`, its comment citing core.48 and record 0038. `check-releases.sh` covers "A change to the site alone", "A change to the site and the runner" and "The site's commit in no notes", and `root:releases` passes.
- **6.1**: The root `CLAUDE.md` says what the site is, where its rules are, and names the project `site` and the site's checks in the `check-fast` row. `README.md`'s "Rules and checks" names `site`. `site/CLAUDE.md` cites `site.1` to `site.7`.
- **6.2**: `measurement-roadmap.md` uses `host: host-a` and `drive: home`, and `site-design.md` uses `bolts open, home`. Outside `site/src/content/{measurements,contexts,notes}/`, no file under `site/` names a drive or machine of ours. MadSwan stands only as the company, in `site-design.md` and `audience-plan.md`, as Decision 15 keeps it.
- **6.3**: The `tools-hub` spec's scenarios "The cards as built" and "Reading timber's card" give timber's card Never prove twice, linking that principle's place on the Method page. That is what the site shows: `site/src/pages/tools.astro` maps `timber: "never-prove-twice"`, and that principle's strip names timber. The site is unchanged. `root:changes` passes.

## Specs

- **`site`**: every requirement and scenario is met, each proved by the site's task that the gate ran.
- **`releases`**: the two modified requirements and their three new scenarios are met (`root:releases`). Archived, though, the spec fails `root:specs`; see the finding below.
- **`content-kind`**: met.
  - The declaration names sluice as the merge queue and the kind as the spec has it, and each prompt resolves under `site/docs/prompts/`.
  - The brief's paths exist, read from `site/` or, where it says so, from the root.
  - The brief names its proof: `site-check`, `sluice exec`, Vale, and a look in light and dark, at phone width, with scripts off and under reduced motion.
  - `site-serve` prints the address under `/boomdrive/`, and `site-stop` exits zero where nothing was served.
  - `site/CLAUDE.md` sends content work to the brief and the `content` kind.
- **`method-tools`**: met. Each principle's strip in `site/src/content/principles/` names the tools the scenario "The tools each principle names" lists. `site:check` and `site:build` hold the strips' schemas.
- **`method-diagrams`**: carried as boomdrive-web held it, its paths under `site/`. No task of this change changes a diagram, and the site builds.
- **`tools-hub`**: met. Timber's card names Never prove twice, as the site shows it.

## Finding: the archived releases spec fails root:specs

`openspec archive boomdrive-keeps-its-site` applies cleanly, and
`openspec validate boomdrive-keeps-its-site --strict` passes. The spec it
writes, `openspec/specs/releases/spec.md`, then fails `root:specs`
(`sluice exec main -- moon run root:specs`): "Totals: 21 passed, 1 failed
(22 items)", `✗ spec/releases`. `openspec validate releases --type spec
--strict` gives the reason:

> requirements[0]: Requirement text is very long (>500 characters).
> requirements[1]: Requirement text is very long (>500 characters).

Each modified requirement adds a sentence for the site's paths to text
that was already near the limit:

| requirement | on `main` | with this change |
|---|---|---|
| The version is git-cliff's, from the commits since the newest release | 474 | 605 |
| The notes are git-cliff's, for the same commits | 470 | 591 |

The change itself validates under `--strict`, and `root:changes` passes.
Only the spec the archive writes fails the length check, so the gate
cannot see this before the archive.

The archive commit was moved back out of the archive, so the change stands
unarchived on this branch.

**What must change:** keep each of the `releases` delta's requirements
within 500 characters. One way is to state the site's paths as a
requirement of their own, "A change to the site alone releases nothing",
holding the scenarios "A change to the site alone", "A change to the site
and the runner" and "The site's commit in no notes". Then the two MODIFIED
requirements can keep their `main` text, or be dropped from the delta. A
narrow proof is to archive in a scratch checkout and run
`sluice exec main -- moon run root:specs`.

## Not blocking

- The comment in `.moon/workspace.yml` lists the projects, `site` among them, but its `check-fast` paragraph names only the crates', the flake's and the root's checks, not the site's. The root `CLAUDE.md`'s `check-fast` row names them. No task asks for the comment.
