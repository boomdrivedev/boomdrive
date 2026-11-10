## Why

The Method's principles describe what the family's tools now do, but a reader of a principle has no way from it to the tool that does it: nothing on the Method links sluice, tailrace or the others, and the Tools page does not list tailrace at all. The user asked, on 2026-10-26, that the Method's sections be cross-referenced to the tools' pages as they are worked on, and the design note for it (`agentplot/blueprints design/tailrace/models/the-method-names-its-tools.md`) says what each principle names and where each link lands. The same note keeps clause numbers off the site: today five principles cite statement clauses by number in their text, and those numbers belong in the outline, not on a page.

## What Changes

- Each principle's place on the Method page shows a short strip, "In the tools", naming the tools that do what the principle says. Each name links to the tool's own product page, at the section that does it, or to the tool's design book where the product page has no such section. A tool with neither a public page nor a public book (Crew, Flywheel Next) is named without a link.
- The strip is data in each principle's frontmatter, a new `tools` field the schema checks, so the page draws it and nothing else lists it.
- The Tools page and the strips agree: the principle each tool's card names ("In the Method") must be one whose strip names that tool, and the build fails otherwise. sounding's card therefore names "Measure the fleet", the principle whose strip names it, where it names "Two loops, one gate" today.
- The Tools page's "Each stands alone" list gains tailrace, in the voice of the others, and its "The Method joins them" paragraph gains one sentence on how tailrace carries a landing to a channel and to every environment that follows it. The glossary gains tailrace as a lowercase term, as its siblings are.
- No clause number of any statement appears on a page of the site. The five principles that cite clauses by number (capture at the source, one agent one worktree, two loops one gate, order at landing, never prove twice) drop those citations, and a new Vale rule warns on a clause number anywhere under `src/content/`. The outline keeps the numbers.
- The outline (`docs/method-outline.md`) gains a table saying which tools each principle names, what each link lands on and the clauses behind it, and a changelog entry for the revision. The site design (`docs/site-design.md`) records the strip, the principle type's new field and the new Vale rule.

This carries out the site design's "How Home and the Method read" (a principle's place on the Method page) and "The Tools hub" (the family's cards and the two columns), and the design note's "Each principle names the tools that realise it" and "What this does not do". It leaves the two diagrams the note also describes to their own units (`one-agent-one-worktree-diagram`, `never-prove-twice-diagram`).

## Capabilities

### New Capabilities

- `method-tools`: what a reader of a principle sees of the tools that do what it says, the strip, where each of its links lands, and the rule that no clause number reaches a page of the site.
- `tools-hub`: the Tools page's words and cards as they bear on the Method: tailrace in the list of tools, how the Method joins them, and each card's principle agreeing with the principles' strips.

### Modified Capabilities

None. No spec under `openspec/specs/` covers the Method's text or the Tools page; `method-diagrams` (on the bolt) covers only the diagrams.

## Impact

- `src/content.config.ts`: the principle schema gains `tools`.
- `src/content/principles/*.md`: every principle but the statement gains its `tools`; five lose their clause citations. `one-agent-one-worktree.md` and `never-prove-twice.md` are also touched by the two diagram units, which add `diagram` after `lastReviewed`; `tools` goes after `line`, so the three changes merge apart.
- `src/pages/method/index.astro`: draws the strip.
- `src/pages/tools.astro`: checks each card's principle against the strips; sounding's card names Measure the fleet.
- `src/content/pages/tools.md`: tailrace's line and the sentence that joins it.
- `src/content/terms/terms.yaml` and the files `scripts/vocab.ts` generates from it in `styles/`; a new hand-written Vale rule, `styles/Flywheel/Clauses.yml`.
- `docs/method-outline.md`, `docs/site-design.md`.
- tailrace gets no card, no status and no accent on the Tools page: those come from `@agentplot/site-theme`'s family, which does not list tailrace, and adding it there is outside this repository.

## Touches

```
src/content.config.ts
src/content/principles/*.md
src/pages/method/index.astro
src/pages/tools.astro
src/content/pages/tools.md
src/content/terms/terms.yaml
styles/config/vocabularies/Flywheel/accept.txt
styles/Flywheel/Terms.yml
styles/Flywheel/Clauses.yml
docs/method-outline.md
docs/site-design.md
```
