# Proposal

## Why

On the Method page, "Never prove twice" still shows only its faint numeral
where its diagram belongs. The site design gives every principle an animated
diagram of its idea, agreed with the operator before it is drawn, and this
principle's idea was agreed on 2026-10-22 (`docs/method-outline.md`, 7. Never
prove twice, Diagram; `docs/site-design.md`, Every principle gets a diagram).
This change draws it.

## What Changes

- A new diagram, `public/diagrams/never-prove-twice.svg`, drawn in the manner
  of the Method's other diagrams (one shell's is the nearest model). It shows:
  - the cache in the middle, holding a pass for each check under the key of
    the inputs it read;
  - four places around it running the same three checks, the site's own gate
    (`check`, `prose` and `build`): a worktree, a second worktree, CI and the
    merge queue;
  - the first worktree's run taking its time, each check filling slowly, and
    each pass filling the cache;
  - the second worktree, CI and the merge queue then replaying those passes in
    a flash, each check marked replayed rather than run;
  - a next candidate in the merge queue that changes one input, the Vale
    configuration, so only the prose check runs again while the other two
    replay, and that run fails;
  - beside the cache, the queue's own record keeping that fail, while the cache,
    which holds passes only, never takes it;
  - a row of four numbered captions that light in turn with the story.
- The diagram uses only the family's diagram colours, so it follows the
  reader's light or dark setting once the page draws it inline.
- It moves on CSS alone, so it plays with scripts off. Under reduced motion it
  holds one still moment that shows the whole idea at once: the first run's
  passes in the cache, the replays, the one check run again, its fail in the
  queue's record and not in the cache, and all four captions.
- The principle's record names the diagram and carries alt text that says what
  it shows; the diagram's own description carries the same words.

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
  principle's front matter. No code, schema, layout or dependency changes: the
  Method page already draws any diagram a principle names
  (`src/lib/diagram.ts`).
- The principle's numeral placeholder on the Method page gives way to the
  diagram.
- The alt text is prose under Vale's house style, so `site-check` reads it.

## Touches

```
public/diagrams/never-prove-twice.svg
src/content/principles/never-prove-twice.md
```
