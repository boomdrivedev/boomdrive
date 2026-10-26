# Design

## Context

The Method page draws every diagram a principle names into the page
(`src/pages/method/index.astro`, through `inlineDiagram` in
`src/lib/diagram.ts`). A principle with no `diagram` in its front matter shows
its numeral in faint outline instead; "Never prove twice"
(`src/content/principles/never-prove-twice.md`) is one of those today.

Three diagrams are drawn: `one-shell.svg`, `intents-and-bolts.svg` and
`order-at-landing.svg` in `public/diagrams/`. One shell's is the closest model
for this one: four places in a column feeding one shared thing in the middle,
dots travelling along wires, boxes whose edge turns accent then ok, a
struck-out chip in the fail colour, and four numbered captions that light in
turn, all on one 16-second CSS cycle. Each was added as one SVG plus a
`diagram` entry in the principle's front matter (commits `caad3cd`,
`d8fcbd1`), and the diagram's `<desc>` carries the same words as the
principle's alt text.

The idea to draw is the one agreed with the operator on 2026-10-22
(`docs/method-outline.md`, 7. Never prove twice, Diagram): a cache in the
middle; four places run the same check, a worktree, a second worktree, CI and
the merge queue; the first run takes its time and fills the cache, the next
three replay in a flash; one input changes and only that check runs again;
beside the cache, the queue's own record keeps a fail the cache never
remembers. The site design sets the rules every diagram keeps
(`docs/site-design.md`, Every principle gets a diagram; Motion never hides
content). The record rests on the sluice statement (`agentplot/blueprints
design/sluice/requirements.md`): a required check whose key already holds a
pass is read from that pass and not run again (clause 15); every check run on
a candidate is recorded with its key and outcome, pass and fail alike, and a
result a tool answered from an earlier run is not a run and is never recorded
as one (clause 16); a key holding both a pass and a fail reads as the policy
says (clause 19). The principle's own page says the cache remembers only
passes, which is why the queue keeps the record beside it.

## Goals / Non-Goals

**Goals:**

- Draw the agreed idea, and nothing beyond it, as one standalone SVG that the
  existing inline drawing takes without any change to the loader or the page.
- Keep the Method's diagrams one family: same canvas size, type, palette
  classes, caption row and cycle length as one shell's.
- Use the site's real checks and a real input, so a reader who opens
  `moon.yml` finds the story true of this repository.

**Non-Goals:**

- No change to `src/lib/diagram.ts`, the Method page, the principle's claim or
  write-up, `docs/method-outline.md` or the Method's changelog.
- No drawing of who may write the cache (the remote cache, the confined
  executor and its credential): the agreed idea does not include it, and the
  write-up carries it.
- No pause button or controls: the Method's diagrams have none, and the site
  design asks for one only on Home's attention figure.
- No generator script in the repository: as with the three drawn diagrams,
  only the SVG is committed.

## Decisions

### The checks and the changed input

The three checks are the site's own gate, the moon tasks tagged `check-fast`
in `moon.yml`: `check` (the types), `prose` (Vale) and `build`. The changed
input is `.vale.ini`, which `prose` declares as an input and neither `check`
nor `build` does, so only `prose` runs again. The keys are short illustrative
hashes, one per check (for example `3f9a`, `a17c`, `5e02`), with `prose`'s new
key distinct (for example `c4d8`); no two are the same.

The diagram shows no durations. "Takes its time" and "in a flash" are said by
how fast a check fills and by the words `ran` and `replayed`; a time in
seconds would read as a measurement, and the site never passes off a number as
measured.

### The layout

A 900 × 446 canvas, as one shell's, so the diagrams sit alike on the page.

```
 Never prove twice   a check that passed on these inputs is not run again
 ┌ worktree a ───────┐   ┌ the cache ────────────────┐   ┌ CI ───────────────┐
 │ check  ▓▓▓▓▓ ran  │──▶│ check  3f9a  pass         │◀──│ check  ▪ replayed │
 │ prose  ▓▓▓▓▓ ran  │   │ prose  a17c  pass         │   │ prose  ▪ replayed │
 │ build  ▓▓▓▓▓ ran  │   │ build  5e02  pass         │   │ build  ▪ replayed │
 └───────────────────┘   │ prose  c4d8  — passes only│   └───────────────────┘
 ┌ worktree b ───────┐   └───────────────────────────┘   ┌ merge queue ──────┐
 │ check  ▪ replayed │◀──┌ the queue's record ───────┐──▶│ candidate 2       │
 │ prose  ▪ replayed │   │ prose  c4d8  fail         │◀──│ .vale.ini changed │
 │ build  ▪ replayed │   └───────────────────────────┘   │ check ▪ replayed  │
 └───────────────────┘                                   │ prose ▓▓▓ ran fail│
                                                         │ build ▪ replayed  │
 (1) The first run…   (2) The next three replay…   (3) One input changes…   (4) The queue's record…
```

- **The four places** are panel boxes (`.box`), two in a column on the left
  (worktree a above worktree b) and two on the right (CI above the merge
  queue), each listing the three checks with a status bar and a word, `ran` or
  `replayed`, and the outcome. A run's bar fills over seconds; a replay's bar
  fills in a blink. The merge queue's box carries a header line naming its
  candidate, `candidate 1`, then `candidate 2 · .vale.ini changed`.
- **The cache** sits in the middle on a `--soft` band, titled as the cache,
  with one row per check: its name, its key and `pass` in `--ok`. Each row is
  empty until worktree a's pass for it arrives. Under the three rows a fourth
  row stands empty for `prose`'s new key, labelled `passes only` in
  `--muted`, so it reads as the place where the fail is never kept.
- **The queue's record** is a second box directly under the cache, between it
  and the merge queue, titled as the queue's record, with its wire running to
  the merge queue only. It is empty until the fail arrives, then shows one row:
  `prose`, its new key and `fail` in `--bad`. Its title line says it records
  every run, pass and fail.
- **The wires** run from each place to the cache, as one shell's wires run to
  its shell. A dot in `--accent` travels from worktree a into the cache for
  each pass it fills, and from the cache out to a place for each replay. The
  fail's dot, in `--bad`, travels from the merge queue into the record, and
  none travels into the cache.
- **Captions**, numbered 1 to 4 on accent badges along the bottom, as one
  shell's: (1) the first run takes its time and fills the cache; (2) the next
  three replay in a flash; (3) one input changes, and only its check runs
  again; (4) the queue's record keeps the fail the cache never remembers.

### The cycle

One 16-second linear cycle, the same as one shell's, with every keyframe on
it.

| Share of the cycle | What happens |
|---|---|
| 0–2% | Every place idle, the cache and the record empty |
| 2–26% | Worktree a's three checks fill slowly, staggered, each marked `ran` and `pass` |
| 10–30% | As each of a's checks passes, its dot travels into the cache and that row fills |
| 32–37% | Worktree b replays: three dots out of the cache, three checks `replayed` at once |
| 39–44% | CI replays the same way |
| 46–51% | The merge queue's candidate 1 replays the same way |
| 56–60% | Candidate 2 arrives, `.vale.ini changed`; `check` and `build` replay, `prose` shows its new key and misses |
| 60–76% | `prose` runs again, filling slowly, and ends in `fail` |
| 77–83% | The fail's dot travels into the queue's record and its row appears; the cache's `passes only` row flashes and stays empty |
| 83–96% | Everything holds |
| 96–100% | Everything fades back to the start |

The captions light in turn: 1 from 0 to 31%, 2 from 31 to 55%, 3 from 55 to
77%, 4 from 77 to 96%, each dimmed the rest of the cycle.

The coder may move these marks by a few points to make the drawing read well,
as long as the four parts of the story stay in that order and the
reduced-motion moment below still shows everything the spec asks.

### Reduced motion

The same rule one shell uses: under `prefers-reduced-motion: reduce` every
animation is paused at one negative delay, and the captions drop their
animation and show at full opacity. The held moment is 90% of the cycle
(`animation-delay:-14.4s`): worktree a's three checks `ran` and passed, the
cache's three passes, worktree b's and CI's checks `replayed`, the merge queue
on candidate 2 with `.vale.ini changed`, `check` and `build` replayed and
`prose` ran and failed, the fail's row in the queue's record, the cache's
`passes only` row empty, and all four captions lit. That is the whole idea in
one frame.

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
- no script, no external reference, no `@import`, no font file: the text uses
  the same system font stacks as one shell's;
- the `<svg>` carries `role="img"` and `aria-labelledby="ttl dsc"` for the
  file on its own; the loader replaces them with the alt text as the label and
  the description as `aria-describedby`.

### Alt text

Written once, set both as the principle's `diagram.alt` and as the SVG's
`<desc>`. A draft the coder may tighten:

> A cache sits in the middle, and four places run the same three checks
> around it: a worktree, a second worktree, CI and the merge queue. In the
> first worktree the checks take their time and pass, and each pass fills the
> cache under the key of the inputs it read. The second worktree, CI and the
> merge queue then replay those passes in a flash, running nothing. A next
> candidate in the merge queue changes the Vale configuration, so only the
> prose check runs again while the other two replay. That run fails. The
> queue's own record, beside the cache, keeps the fail; the cache keeps passes
> only, so it never remembers it.

Every sentence stays under Vale's 30 words, and none ends on a number: Vale
does not read a full stop after a number as a sentence end (commit `02af2c8`).

### Alternatives considered

- **The changed input in a worktree rather than the queue.** It would show the
  rerun, but the fail would then be a worktree's, which the queue's record
  never holds. Putting the change in the queue's next candidate lets the one
  check run again and its fail land in the record in one move, as the
  principle's write-up has it: testing the combination reruns only what it
  changed.
- **Durations on the runs and replays.** Rejected above: an invented time
  would read as a measurement.
- **SMIL animation instead of CSS keyframes.** SMIL ignores
  `prefers-reduced-motion`, and the loader scopes and renames CSS only. CSS
  keyframes, as the other diagrams use.

## Risks / Trade-offs

- [Five boxes, three rows each, crowd a 900-wide canvas and shrink at phone
  width] → keep each place's rows to a check name, a bar and one word, in the
  11.5px mono size one shell uses at its smallest, and check the page at
  phone width.
- [The cache and the record, stacked in the middle, read as one box] → give
  them separate titles and boxes with a gap between, the cache on the `--soft`
  band and the record on `--panel`, and run the record's wire only to the
  merge queue.
- [A colour fixed outside the palette breaks the reader's theme] → every fill
  and stroke goes through a class on the palette variables; the tasks check
  the file for any hex outside the two palette blocks.
