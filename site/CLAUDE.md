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
- The glossary in `src/content/terms/terms.yaml` is the one vocabulary; add a
  term there, never in `styles/` (those files are generated).
- Conventional Commits; stage by explicit paths; local only, never push.
- Bare layout: `flywheel-web.git/` beside `main/`.
