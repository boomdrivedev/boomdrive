# Proposal

## Why

On the Method page, "One agent, one worktree" still shows only its faint
numeral where its diagram belongs. The site design gives every principle an
animated diagram of its idea, agreed with the operator before it is drawn,
and this principle's idea was agreed on 2026-10-22 (`docs/method-outline.md`,
2. One agent, one worktree, Diagram; `docs/site-design.md`, Every principle
gets a diagram). This change draws it.

## What Changes

- A new diagram, `public/diagrams/one-agent-one-worktree.svg`, drawn in the
  manner of the Method's other diagrams (one shell's is the nearest model).
  It shows:
  - three worktrees side by side, each with its own agent, its own ports,
    its own emulator and its own dev server;
  - the three agents working at their own pace, so each finishes at a
    different moment;
  - each agent, once finished, submitting its head to the line below, where
    the merge queue merges and lands it, while no worktree ever merges: each
    worktree's merge is shown struck out;
  - partway through, one agent reaching across into another worktree's dev
    server and being refused at that worktree's edge;
  - a row of four numbered captions that light in turn with the story.
- The diagram uses only the family's diagram colours, so it follows the
  reader's light or dark setting once the page draws it inline.
- It moves on CSS alone, so it plays with scripts off. Under reduced motion it
  holds one still moment that shows every part of the idea at once: one head
  already on the line, two agents still working, the refused reach, the
  struck merges and all four captions.
- The principle's record names the diagram and carries alt text that says
  what the diagram shows; the diagram's own description carries the same
  words.

## Capabilities

### New Capabilities

- `method-diagrams`: each principle's diagram on the Method page, what it
  must show, and how it behaves in the reader's theme, with scripts off and
  under reduced motion. This change adds the requirements for "One agent,
  one worktree"; other principles' diagrams add theirs to the same
  capability.

### Modified Capabilities

None. The project has no specs yet.

## Impact

- One new file in `public/diagrams/`, and two lines added to one principle's
  front matter. No code, schema, layout or dependency changes: the Method
  page already draws any diagram a principle names (`src/lib/diagram.ts`).
- The principle's numeral placeholder on the Method page gives way to the
  diagram.
- The alt text is prose under Vale's house style, so `site-check` reads it.

## Touches

```
public/diagrams/one-agent-one-worktree.svg
src/content/principles/one-agent-one-worktree.md
```
