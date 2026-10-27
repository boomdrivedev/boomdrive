# CLAUDE.md

flywheel-web is the content site for the Method: principles, field notes, the
fleet's measurements and the tools. `docs/` holds its design and plans; read
`docs/site-design.md` first.

- Nothing commercial goes here: pricing, revenue and product plans live in the
  private strategy repository. Products appear on the site only as a waitlist.
- No client is named, and no measurement or example comes from client work.
  Measurements read our own repositories only.
- A field note is never rewritten after it is published; a later note
  supersedes it. The Method's pages are revised in place, with a version and a
  changelog entry.
- Every tool comes from `devenv.nix`; never a global install. Run
  `site-check` before committing content: Vale's errors and a schema failure
  both fail it, and a Vale warning is fixed, not ignored.
- After a change to `src/content.config.ts`, restart the dev server
  (`astro dev stop`, then `bun run dev`) rather than debug the page: Astro
  7.3.5's dev server stops finishing responses after one.
- The glossary in `src/content/terms/terms.yaml` is the one vocabulary; add a
  term there, never in `styles/` (those files are generated).

A change to what the site shows (a diagram, a page's words, a figure, a tools
strip) is content work: it reads `docs/content-brief.md` whole and a one-line
task naming the pages or diagrams, and runs as the `content` kind in
`.config/flume.toml` or, until flume runs here, as a crew chore whose text is
the task and the brief's path. It is not written up as an OpenSpec change.

- Conventional Commits; stage by explicit paths. The repository is public and
  `main` is the published site: every landing on `main` deploys it to GitHub
  Pages (`.github/workflows/pages.yml`) after `site-check` passes there.
  `main` and every `bolt/*` branch move only through sluice
  (`.config/sluice.toml`), whose gate is the same checks as moon tasks,
  `devenv shell -- moon run ':#check-fast'`; nothing is pushed to them by
  hand.
- Every internal link goes through `url()` in `src/lib/url.ts`, never a bare
  root path: the site is served under `/flywheel-web/`.
- Bare layout: `flywheel-web.git/` beside `main/`.
