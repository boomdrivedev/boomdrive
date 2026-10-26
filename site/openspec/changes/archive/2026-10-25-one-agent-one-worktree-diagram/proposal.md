# Proposal

## Why

On the Method page, "One agent, one worktree" still shows only its faint
numeral where its diagram belongs. The site design gives every principle an
animated diagram of its idea, agreed with the operator before it is drawn,
and this principle's idea was agreed on 2026-10-22 (`docs/method-outline.md`,
2. One agent, one worktree, Diagram; `docs/site-design.md`, Every principle
gets a diagram). This change draws it.

The user asked, on 2026-10-26, that a diagram showing ports, emulators and
dev servers be grounded in sluice, the family's merge queue, since sluice is
now the tool that does what the principle describes. So the diagram draws
what sluice really does: it equips each agent's worktree as a place with a
block of ports of its own, on which the place's emulator and dev server run
(sluice ADR 0018 and 0016; `agentplot/blueprints
design/tailrace/models/the-method-names-its-tools.md`, The two diagrams,
grounded), and every head goes to sluice's queue, which performs every merge
and landing (sluice statement, clauses 1 and 2).

## What Changes

- A new diagram, `public/diagrams/one-agent-one-worktree.svg`, drawn in the
  manner of the Method's other diagrams (one shell's is the nearest model).
  It shows:
  - three worktrees side by side, each a place sluice equips: a block of
    ports of its own from sluice, with its emulator and its dev server
    running on those ports, and its own agent;
  - the three agents working at their own pace, so each finishes at a
    different moment;
  - each agent, once finished, submitting its head to sluice's queue on the
    line below, where sluice merges and lands it, while no worktree ever
    merges: each worktree's merge is shown struck out;
  - partway through, one agent reaching across into another worktree's dev
    server and being refused at that worktree's edge;
  - a row of four numbered captions that light in turn with the story;
  - a line naming the tools it shows, Worktrunk making each worktree and
    sluice equipping it and landing its head, with a link to sluice's page.
- The diagram uses only the family's diagram colours, so it follows the
  reader's light or dark setting once the page draws it inline.
- It moves on CSS alone, so it plays with scripts off. Under reduced motion it
  holds one still moment that shows every part of the idea at once: heads
  already on the line, an agent still working, the refused reach, the struck
  merges and all four captions.
- The principle's record names the diagram and carries alt text that says
  what the diagram shows, sluice included; the diagram's own description
  carries the same words.

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
  page already draws any diagram a principle names (`src/lib/diagram.ts`),
  and a link inside the SVG survives that drawing unchanged.
- The principle's numeral placeholder on the Method page gives way to the
  diagram.
- The alt text is prose under Vale's house style, so `site-check` reads it.
- The principle's write-up, its "today" and its claim are not changed here;
  linking each principle to its tools' pages in the page's text is a
  separate unit (`the-method-names-its-tools`).

## Touches

```
public/diagrams/one-agent-one-worktree.svg
src/content/principles/one-agent-one-worktree.md
```
