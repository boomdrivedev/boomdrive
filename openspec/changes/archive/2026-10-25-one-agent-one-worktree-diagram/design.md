# Design

## Context

The Method page draws every diagram a principle names into the page
(`src/pages/method/index.astro`, through `inlineDiagram` in
`src/lib/diagram.ts`). A principle with no `diagram` in its front matter shows
its numeral in faint outline instead; "One agent, one worktree"
(`src/content/principles/one-agent-one-worktree.md`) is one of those today.

Three diagrams are drawn: `one-shell.svg`, `intents-and-bolts.svg` and
`order-at-landing.svg` in `public/diagrams/`. One shell's is the closest model
for this one: boxes in columns, a dashed lane that is stopped at a wall, a
struck-out chip, and four numbered captions that light in turn, all on one
16-second CSS cycle. Each was added as one SVG plus two lines of front matter
(commits `caad3cd`, `d8fcbd1`), and the diagram's `<desc>` carries the same
words as the principle's alt text.

The idea to draw is the one agreed with the operator on 2026-10-22
(`docs/method-outline.md`, 2. One agent, one worktree, Diagram): three
worktrees side by side, each with its own agent, ports, emulator and dev
server, working at their own pace; each submits its head to the line, none
merges; one agent reaches into another's server and is refused. The site
design sets the rules every diagram keeps (`docs/site-design.md`, Every
principle gets a diagram; Motion never hides content).

The user asked on 2026-10-26 that the diagram be grounded in sluice where it
shows ports, emulators and dev servers. The design agent's note for that
(`agentplot/blueprints design/tailrace/models/the-method-names-its-tools.md`,
The two diagrams, grounded) and sluice's own records say what sluice does:

- **An agent's worktree is a place sluice equips and never removes**
  (sluice ADR 0018). sluice holds a block of ports for it from its first run
  until its release, the same ports on every run and never another
  worktree's, and `sluice release` ends what its runs left.
- **The block comes from one rule** (sluice ADR 0016): the policy names the
  ports a stack uses, and each name takes a fixed position in the place's
  block, `ports_from + block × names + position`, with `ports_from` 20000 by
  default, every port checked free.
- **sluice starts no service and chooses no port of its own invention**
  (sluice statement, clause 31 and I5). The emulator and the dev server are
  brought up by the repository's own tools, on the ports sluice gave the
  place, and end with the place.
- **A session submits a head and does nothing more**; the queue performs
  every merge and landing, and an attempt to merge is refused (sluice
  statement, clauses 1 and 2).

sluice's page is `https://agentplot.github.io/sluice/`, as the family's list
in `@agentplot/site-theme` gives it (`src/family.ts`, sluice's `site`).

## Goals / Non-Goals

**Goals:**

- Draw the agreed idea, as sluice really does it, as one standalone SVG that
  the existing inline drawing takes without any change to the loader or the
  page.
- Name the tools the diagram shows, and link sluice's page from it.
- Keep the Method's diagrams one family: same width, type, palette classes,
  caption row and cycle length as one shell's.

**Non-Goals:**

- No change to `src/lib/diagram.ts`, the Method page, the principle's claim or
  write-up, `docs/method-outline.md` or the Method's changelog.
- No strip of tool links beside the principle, and no clause numbers on the
  page: the note keeps the numbers in the outline, and the strip is the
  separate unit `the-method-names-its-tools`.
- No release shown: the idea agreed is the places, the pace, the submit and
  the refusal, and a fifth beat would crowd the cycle.
- No pause button or controls: the Method's diagrams have none, and the site
  design asks for one only on Home's attention figure.
- No generator script in the repository: as with the three drawn diagrams,
  only the SVG is committed.

## Decisions

### The layout

900 wide, as one shell's, so the diagrams sit alike on the page, and about
470 high, one shell's 446 with room for the tools line.

```
 One agent, one worktree   each agent works alone, and submits rather than merges
 ┌ worktree a · sluice place ┐ ┌ worktree b · sluice place ┐ ┌ worktree c · sluice place ┐
 │ agent      ▓▓▓▓▓▓░░░░     │ │ agent      ▓▓▓░░░░░░░     │ │ agent      ▓▓░░░░░░░░     │
 │ ports      20000 20001    │ │ ports      20002 20003    │ │ ports      20004 20005    │
 │ dev server :20000         │◀╳┤╌╌ reach ╌╌              │ │ dev server :20004         │
 │ emulator   :20001         │ │ emulator   :20003         │ │ emulator   :20005         │
 │ merge̶      [submit head]  │ │ merge̶      [submit head]  │ │ merge̶      [submit head]  │
 └────────────┬──────────────┘ └────────────┬──────────────┘ └────────────┬──────────────┘
              ▼ head                        ▼                             ▼
 THE LINE ════●═════════════════════════════●═════════════════════════════●════▶
              sluice's queue merges and lands each head
 (1) Own place, own ports…  (2) Each at its own pace  (3) Another's server…  (4) Each submits…
 Worktrunk makes each worktree; sluice gives it its ports and lands its head.  sluice's page ↗
```

- **Each worktree** is a panel box (`.box`) headed with its letter and
  "sluice place", holding four rows: the agent with a work bar that fills,
  the ports sluice gave the place, its dev server and its emulator, each of
  those two on one of the place's ports.
- **The ports are sluice's.** The policy is taken to name two ports, the dev
  server's and the emulator's, so the three places hold the blocks
  20000–20001, 20002–20003 and 20004–20005, as ADR 0016's rule gives them
  with `ports_from` at its default. The numbers are illustrative only in
  which block falls to which worktree; no two worktrees share one.
- **"None merges"** is drawn twice: each worktree's footer carries a `merge`
  label struck out in `--bad`, static so it reads in the still as well, beside
  the `submit head` chip that lights when that worktree submits; and nothing
  but a head ever travels from a worktree to the line.
- **The line** runs across the bottom on a `--soft` band, labelled as the
  line, with "sluice's queue merges and lands each head" under it. A
  submitted head drops from its worktree onto the band and becomes a landed
  dot in `--ok`.
- **The refusal** is one shell's lane-and-wall: a dashed lane (`.lanel`) runs
  from worktree b's agent toward worktree a's dev server row and stops at a
  `--bad` wall on worktree a's edge, with a short label in `--bad`
  ("refused: not its place"). Worktree a's box edge flashes `--bad` as the
  wall appears.
- **Captions**, numbered 1 to 4 on accent badges under the line, as one
  shell's: (1) each agent has its own place, with its own ports, emulator
  and server; (2) each works at its own pace; (3) another's server is out of
  reach; (4) each submits its head, and sluice merges and lands it.
- **The tools line** sits under the captions, in the muted caption type:
  "Worktrunk makes each worktree; sluice gives it its ports and lands its
  head", then "sluice's page ↗" as the one link (below). It is static, so it
  reads in the still and with motion.

### The link

The link is an SVG `<a href="https://agentplot.github.io/sluice/">` around
the "sluice's page ↗" text, in `--accent`, underlined, with a visible focus
outline in `--accent` under `a:focus-visible`. It opens in the same tab, as
the site's other links to family sites do.

It stays inside the SVG because the intent puts it in the diagram, and the
loader leaves it as it is: it renames only `href="#…"` references to the
diagram's own ids, and the page draws the SVG inline, so the link is a real
link on the page. It is the file's one external reference; every other
`href` is none.

### The cycle

One 16-second linear cycle, the same as one shell's, with every keyframe on
it, so the agents' different paces come from different spans on one clock
rather than different durations (different durations would drift apart and
leave no single moment for reduced motion to hold).

| Share of the cycle | What happens |
|---|---|
| 0–2% | All three worktrees idle, bars empty |
| 2–28% | Worktree a's bar fills |
| 2–56% | Worktree b's bar fills |
| 2–78% | Worktree c's bar fills |
| 28–38% | a's `submit head` lights; its head drops to the line and lands |
| 40–96% | b's reach runs out and is stopped at a's wall; the refusal stays shown |
| 56–66% | b submits; its head lands |
| 78–88% | c submits; its head lands |
| 96–100% | Everything fades back to the start |

The captions light in turn: 1 from 0 to 16%, 2 from 16 to 40%, 3 from 40 to
56%, 4 from 56 to 92%, each dimmed the rest of the cycle. The ports, the
servers, the struck merges and the tools line never move.

The coder may move these marks by a few points to make the drawing read
well, as long as the three finishes stay apart and the reduced-motion moment
below still shows everything the spec asks.

### Reduced motion

The same rule one shell uses: under `prefers-reduced-motion: reduce` every
animation is paused at one negative delay, and the captions drop their
animation and show at full opacity. The held moment is 70% of the cycle
(`animation-delay:-11.2s`): heads a and b on the line, c still working with
its bar most of the way full, the refused reach and the wall shown, every
struck `merge` visible, all four captions lit, and the ports, servers and
tools line as always. That is the whole idea in one frame.

### Working inside the loader

`inlineDiagram` rewrites the SVG as it draws it, so the file keeps to what it
handles:

- one `<style>` block; the palette on `:root`, with a light palette under
  `@media (prefers-color-scheme:light)`, both of which the loader drops in
  favour of the page's tokens;
- every other rule a plain selector, which the loader prefixes with the
  diagram's class, so no rule may target the `<svg>` element itself;
- keyframes named `k-…`, which the loader renames after the principle, and ids
  only on `<title>` and `<desc>` (`ttl`, `dsc`), which it also renames;
- no script, no `@import`, no font file and no external reference but the one
  link to sluice's page: the text uses the same system font stacks as one
  shell's;
- the `<svg>` carries `role="img"` and `aria-labelledby="ttl dsc"` for the
  file on its own; the loader replaces them with the alt text as the label and
  the description as `aria-describedby`.

### Alt text

Written once, set both as the principle's `diagram.alt` and as the SVG's
`<desc>`. A draft the coder may tighten:

> Three worktrees side by side, each a place sluice equips with a block of
> ports of its own, on which its emulator and its dev server run. Each
> worktree has its own agent, and the three agents work at their own pace,
> so each finishes at a different time. As each agent finishes, it submits
> its head to sluice's queue, which merges and lands it on the line; no
> worktree merges its own work. Partway through, the agent in the second
> worktree reaches into the first worktree's dev server and is refused,
> because that server is not its own. Worktrunk makes each worktree, and
> sluice equips it and lands its head.

Every sentence stays under Vale's 30 words, and none ends on a number: Vale
does not read a full stop after a version or port number as a sentence end
(commit `02af2c8`). The port numbers stay out of the alt text: they are
illustrative, and Vale's unsourced-number rule would read them.

### Alternatives considered

- **SMIL animation instead of CSS keyframes.** SMIL ignores
  `prefers-reduced-motion`, and the loader scopes and renames CSS only. CSS
  keyframes, as the other diagrams use.
- **Each agent on its own cycle length** to show "own pace" literally. The
  cycles would drift out of step, the captions could not follow them, and
  reduced motion would have no single moment to hold. Spans on one cycle
  instead.
- **Showing a merge attempted and refused**, as one shell shows a tool
  refused. The idea agreed with the operator gives the refusal to the reach
  into another's server and says only that none merges; a second refusal
  would compete with it. The struck `merge` in each footer says it without
  motion.
- **sluice drawn starting the emulator and dev server.** sluice starts no
  service (clause 31, I5); drawing it so would teach the reader something
  false. sluice gives the ports, and the servers stand on them.
- **The refusal drawn as sluice's sandbox stopping the reach.** The design
  note grounds it in clause 35's confined run, but ADR 0018 records an
  agent's own run in its worktree as unconfined. The wall stays at the
  place's edge, said as "not its place", with no claim about which part of
  sluice draws it.
- **The link to sluice's page outside the diagram**, in the principle's text
  or a strip beside it. That is the separate cross-reference unit; this
  diagram carries its own link as the intent asks.

## Risks / Trade-offs

- [The loader sets `role="img"` on the drawn SVG, so a screen reader treats
  its children, the link among them, as one picture and does not announce
  the link] → the alt text names sluice, the link is still reached by
  keyboard and pointer, and the principle's write-up already links sluice's
  statement; a change to the loader is out of this unit.
- [The narrow gap between worktree a and b crowds the reach and its label] →
  place the label above the boxes or inside worktree b's panel near the lane's
  start, and check it at the Method page's width and at phone width.
- [Dense numbers (ports) shrink to illegibility at phone width] → keep the
  rows to two short numbers each in the 11.5px mono size one shell uses at its
  smallest, and check the page at phone width.
- [The tools line and its link are small at phone width] → keep the link's
  text short ("sluice's page ↗") and its hit area the full line height, and
  check it at phone width.
- [A colour fixed outside the palette breaks the reader's theme] → every
  fill and stroke goes through a class on the palette variables; the tasks
  check the file for any hex outside the two palette blocks.
