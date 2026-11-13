# CLAUDE.md

Boomdrive's site, `site/` of boomdrivedev/boomdrive: the Method's principles,
field notes, the fleet's measurements and the tools, an Astro site on bun
served at `https://boomdrivedev.github.io/boomdrive/`. `docs/` holds its design
and plans; read `docs/site-design.md` first. The paths here are read from
`site/`.

The site's rules are the area page `docs/architecture/site.md` at the
repository's root, beside `core.md`, which binds wherever a change is made
(record 0038):

- Nothing commercial goes here: pricing, revenue and product plans live in the
  private strategy repository. Products appear on the site only as a waitlist
  (site.1).
- No client is named, and no measurement or example comes from client work.
  Measurements read our own repositories only (site.2).
- A field note is never rewritten after it is published; a later note
  supersedes it. The Method's pages are revised in place, with a version and a
  changelog entry (site.3).
- The glossary in `src/content/terms/terms.yaml` is the one vocabulary; add a
  term there, never in `styles/` (those files are generated) (site.4).
- A change to what the site shows is content work, below (site.5).
- Every internal link goes through `url()` in `src/lib/url.ts`, never a bare
  root path: the site is served under `/boomdrive/`, its repository's name
  (site.6; record 0040).
- Our own fleet's drives and machines are named only on the measurement and
  field-note pages and the content they are built from, as ours. The guides,
  reference, glossary, quick start, the site's design and its archived changes
  name none of ours, and no page presents a name of ours as the reader's own
  (site.7).
- Each step of the quick start, `/start/`, shows its text as files under
  `src/start/`, each on the page as it stands, and names the one test of the
  gate that runs them: a reworded file runs as reworded at the gate, and a
  step with no test fails `site:test` (site.8; record 0043).

Working here:

- Every tool comes from the repository's `devenv.nix`; never a global install.
  Run `site-check` in the devenv shell before committing content: the type
  check, Vale over the words, the build and the site's tests, each a task of
  the moon project `site`. Vale's errors and a schema failure both fail it,
  and a Vale warning is fixed, not ignored.
- After a change to `src/content.config.ts`, restart the dev server
  (`astro dev stop`, then `bun run dev`) rather than debug the page: Astro
  7.3.5's dev server stops finishing responses after one.
- Conventional Commits; stage by explicit paths. `main` moves only through
  sluice (`.config/sluice.toml` at the repository's root), whose gate runs the
  repository's checks, the site's among them, as moon tasks tagged
  `check-fast`; nothing is pushed to it by hand. The workflow `pages`
  (`.github/workflows/pages.yml`) publishes `main`'s site at each push that
  changes it. A commit that changes only the site is in no release's notes
  and bumps no version (core.48).

A change to what the site shows (a diagram, a page's words, a figure, a tools
strip) is content work: it reads `docs/content-brief.md` whole and a one-line
task naming the pages or diagrams, and runs as the `content` kind in the
repository's `.config/flume.toml`. It is not written up as an OpenSpec change.

**Specs.** The site's specs are the repository's, under `openspec/specs/`, and
change only through an OpenSpec change: a proposal, a delta spec, and tasks,
validated with `openspec validate --strict` and archived with
`openspec archive`, so the spec reconciles, under the site's rules in
`openspec/config.yaml`. This holds for a fix or a chore too. Where the work
changes a requirement, it carries that change in the same branch. Never edit a
spec file directly.
