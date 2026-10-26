# Proposal

## Why

On the Method page, "Never prove twice" still shows only its faint numeral
where its diagram belongs. The site design gives every principle an animated
diagram of its idea, agreed with the operator before it is drawn, and this
principle's idea was agreed on 2026-10-22 (`docs/method-outline.md`, 7. Never
prove twice, Diagram; `docs/site-design.md`, Every principle gets a diagram).
This change draws it.

The user asked, on 2026-10-26, that this diagram be grounded in tailrace and
sluice, since they are now the tools that do what the principle describes.
So the diagram draws what those tools really do: the cache in the middle is
tailrace's, filled with the archives a landing's verdicts name and replayed
into every checkout, and beside it is sluice's own record of every run, a
fail as faithfully as a pass (`agentplot/blueprints
design/tailrace/models/the-method-names-its-tools.md`, The two diagrams,
grounded; the tailrace book's cache page; the sluice statement, clauses 15,
16 and 19).

## What Changes

- A new diagram, `public/diagrams/never-prove-twice.svg`, drawn in the manner
  of the Method's other diagrams (one shell's is the nearest model). It shows:
  - tailrace's cache in the middle, holding one entry for each check under
    the hash of the inputs it read;
  - four places running the same three checks, the site's own gate (`check`,
    `prose` and `build`): sluice's merge queue, a worktree, a second worktree
    and CI;
  - the merge queue testing a candidate first, each check taking its time
    and passing, and each run going into sluice's record;
  - the candidate landing, and the landing's verdicts filling the cache, one
    entry for each check;
  - the worktree, the second worktree and CI then replaying those checks
    from the cache in a flash, each marked replayed rather than run;
  - a next candidate in the queue that changes one input, the Vale
    configuration, so only the prose check runs again while the other two
    replay, and that run fails;
  - sluice's record keeping that fail, while the cache, which only a landing
    fills, never takes it;
  - a row of four numbered captions that light in turn with the story;
  - a line naming the tools it shows, moon declaring each check's inputs,
    tailrace keeping the cache and sluice testing, landing and recording,
    with links to tailrace's page and sluice's page.
- The diagram uses only the family's diagram colours, so it follows the
  reader's light or dark setting once the page draws it inline.
- It moves on CSS alone, so it plays with scripts off. Under reduced motion it
  holds one still moment that shows the whole idea at once: the landing's
  entries in the cache, the replays, the one check run again, its fail in
  sluice's record and not in the cache, all four captions and the tools line.
- The principle's record names the diagram and carries alt text that says
  what it shows, tailrace and sluice included; the diagram's own description
  carries the same words.

## Capabilities

### New Capabilities

- `method-diagrams`: each principle's diagram on the Method page, what it must
  show, and how it behaves in the reader's theme, with scripts off and under
  reduced motion. This change adds the requirements for "Never prove twice";
  other principles' diagrams add theirs to the same capability.

### Modified Capabilities

None. The project has no specs yet.

## Impact

- One new file in `public/diagrams/`, and a `diagram` entry added to one
  principle's front matter. No code, schema, layout or dependency changes:
  the Method page already draws any diagram a principle names
  (`src/lib/diagram.ts`), and a link inside the SVG survives that drawing
  unchanged.
- The principle's numeral placeholder on the Method page gives way to the
  diagram.
- The alt text is prose under Vale's house style, so `site-check` reads it.
- The principle's write-up, its "today" and its claim are not changed here;
  linking each principle to its tools' pages in the page's text is a
  separate unit (`the-method-names-its-tools`).

## Touches

```
public/diagrams/never-prove-twice.svg
src/content/principles/never-prove-twice.md
```
