## 1. Each principle names its tools

- [x] 1.1 Add `tools` to the principle schema in `src/content.config.ts` as design.md gives it (a list of `name` and an optional full-address `href`, default empty), with its comment; prove it with `devenv shell -- moon run :check`, which reports no error, and by giving one record a bare `/sluice/` `href` for a moment, which makes `moon run :build` fail naming that file, then putting it back
- [x] 1.2 Give each of the nine principles (not `the-statement.md`) its `tools`, directly after `line`, in the order and with the links of design.md's table, Crew and Flywheel Next with no `href`; prove it by `grep -c '^tools:' src/content/principles/*.md` reporting 1 for each of the nine and 0 for the statement, and by `git diff` showing `lastReviewed` unchanged in every record
- [x] 1.3 Fetch every `href` on the strips and look for its anchor: for each, `curl -s <page>` answers and, where the link has a `#<id>`, the page holds `id="<id>"`; record the list and its result in your summary
- [x] 1.4 Draw the strip in `src/pages/method/index.astro` after the idea and before the fold, styled as design.md says (the Tools card's "In the Method" manner, page tokens only, `order: 6` on a phone); prove it in a browser on `sluice exec main -- bun run dev`: on the Method page every principle shows "In the tools" with its names, linked names open their pages, Crew and Flywheel Next are plain, the statement shows none, and it reads in light and dark, at 375px wide with no sideways scroll, with scripts off, and under reduced motion with no transition

## 2. No clause number on the site

- [x] 2.1 Remove the clause citations from `capture-at-the-source.md`, `one-agent-one-worktree.md`, `two-loops-one-gate.md`, `order-at-landing.md` and `never-prove-twice.md` as design.md lists them, closing up each sentence; prove it with `grep -rniE 'clauses? [0-9]' src/` reporting nothing, and with `devenv shell -- vale src/content/principles` reporting no error or warning
- [x] 2.2 Add `styles/Flywheel/Clauses.yml` as design.md gives it; prove it by writing "clause 4" into a principle's text for a moment and seeing `devenv shell -- vale src/content/principles` report the `Flywheel.Clauses` warning on it, then removing it and seeing Vale report nothing
- [x] 2.3 Prove the built site carries no clause number: after `devenv shell -- moon run :build`, `grep -rliE 'clauses? [0-9]' dist` reports nothing

## 3. The Tools page

- [x] 3.1 In `src/content/pages/tools.md`, add tailrace's entry after sluice's and the sentence after "…and the work lands through sluice.", both worded as design.md gives them; prove it with `devenv shell -- vale src/content/pages` reporting no error or warning, and by the Tools page in a browser showing both, in light and dark and at 375px wide
- [x] 3.2 Add the `tailrace` term to `src/content/terms/terms.yaml` after `sluice`, as design.md gives it; prove it with `devenv shell -- moon run :vocab` regenerating `styles/config/vocabularies/Flywheel/accept.txt` with `tailrace` in it, and with `moon run :prose` reporting no error or warning
- [x] 3.3 In `src/pages/tools.astro`, make `principle()` take the member's name and throw, with the message design.md gives, when the principle's `tools` does not name it, and map sounding to `measure-the-fleet` with a comment saying why; prove it with `devenv shell -- moon run :build` passing, the Tools page showing sounding's card under "In the Method" as Measure the fleet and the other cards as before, and by mapping flume to `one-shell` for a moment, which makes the build fail naming flume and `one-shell`, then putting it back

## 4. The docs

- [x] 4.1 In `docs/method-outline.md`, add "The tools each principle names" before the changelog with design.md's table, change the opening sentence on cited clauses to say the site shows none and the outline keeps them, and add the changelog entry design.md gives; prove it by checking each row's tools and links against the principles' `tools` (every name and address matches) and by the changelog entry reading in the shape of the ones before it
- [ ] 4.2 In `docs/site-design.md`, add `tools (name, href)` to the Principle row of "The types", the strip and the no-clause-number rule to "The Method reads one principle at a time", the card's agreement with the strips to "The Tools hub", and the `Clauses` row to the house style table; prove it by `git diff docs/site-design.md` touching only those four places

## 5. Integration

- [ ] 5.1 Run `site-check` in the devenv shell and `devenv shell -- moon run ':#check-fast'`; both pass, with Vale reporting no error or warning
