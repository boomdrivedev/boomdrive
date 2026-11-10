# Design

## Context

The Method page draws every diagram a principle names into the page
(`src/pages/method/index.astro`, through `inlineDiagram` in
`src/lib/diagram.ts`). A principle with no `diagram` in its front matter shows
its numeral in faint outline instead; "Never prove twice"
(`src/content/principles/never-prove-twice.md`) is one of those today.

Three diagrams are drawn: `one-shell.svg`, `intents-and-bolts.svg` and
`order-at-landing.svg` in `public/diagrams/`. One shell's is the closest model
for this one: places feeding one shared thing in the middle, dots travelling
along wires, boxes whose edge turns accent then ok, a struck-out chip in the
fail colour, and four numbered captions that light in turn, all on one
16-second CSS cycle. Each was added as one SVG plus a `diagram` entry in the
principle's front matter (commits `caad3cd`, `d8fcbd1`), and the diagram's
`<desc>` carries the same words as the principle's alt text.

The idea to draw is the one agreed with the operator on 2026-10-22
(`docs/method-outline.md`, 7. Never prove twice, Diagram): a cache in the
middle; four places run the same check, a worktree, a second worktree, CI and
the merge queue; the first run takes its time and fills the cache, the next
three replay in a flash; one input changes and only that check runs again;
beside the cache, the queue's own record keeps a fail the cache never
remembers. The site design sets the rules every diagram keeps
(`docs/site-design.md`, Every principle gets a diagram; Motion never hides
content).

The user asked, on 2026-10-26, that the diagram be grounded in tailrace and
sluice now. The design note for that (`agentplot/blueprints
design/tailrace/models/the-method-names-its-tools.md`, The two diagrams,
grounded) says what each part of the drawing is in those tools:

- **The cache is tailrace's.** moon's output archives are tailrace's first
  cache entries, each named by the task's hash. After a landing, the landing's
  host pushes the archives that landing's build left, and `tailrace sync`
  renders into each checkout's moon outputs directory, shared by every
  checkout on the host, the entries whose hashes the landing's verdicts name;
  a task whose inputs are unchanged hashes the same and replays (the tailrace
  book's cache page; tailrace statement, clauses 52 to 55 and 100, and C.3;
  tailrace's Bolt 3, the push after a landing). A cache entry decides nothing
  and is never an artifact.
- **The record is sluice's.** sluice records every check run on a candidate
  with its key, its outcome, pass and fail alike, and its executor, before
  the verdict is read; a result a tool answered from an earlier run is not a
  run and is never recorded as one (sluice statement, clause 16). A required
  check whose key already holds a trusted pass is read from that pass and
  not run again (15), and a replayed result counts only where a trusted
  signer has signed a pass for the same key; the tool's cache supplies
  outputs and decides nothing (108). A key holding both a pass and a fail
  reads as the policy says (19).
- **One input changes and only that check runs again** because moon hashes
  each task over the inputs it declares.

The note asks that the caption name moon, sluice and tailrace, and that the
site's pages carry no clause numbers: the drawing names tools and links
their pages, and the numbers stay in the outline.

## Goals / Non-Goals

**Goals:**

- Draw the agreed idea as tailrace and sluice do it, and nothing beyond it,
  as one standalone SVG that the existing inline drawing takes without any
  change to the loader or the page.
- Keep the Method's diagrams one family: same width, type, palette classes,
  caption row, tools line and cycle length as the others.
- Use the site's real checks and a real input, so a reader who opens
  `moon.yml` finds the story true of this repository.

**Non-Goals:**

- No change to `src/lib/diagram.ts`, the Method page, the principle's claim or
  write-up, `docs/method-outline.md` or the Method's changelog.
- No "In the tools" strip, Tools page entry or glossary term for tailrace:
  those are the separate unit `the-method-names-its-tools`.
- No drawing of who may sign a run or which signer counts (sluice 78, 108):
  the agreed idea does not include it, and the write-up carries it.
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
task hashes, one per check (for example `3f9a`, `a17c`, `5e02`), with
`prose`'s new key distinct (for example `c4d8`); no two are the same.

The diagram shows no durations. "Takes its time" and "in a flash" are said by
how fast a check fills and by the words `ran` and `replayed`; a time in
seconds would read as a measurement, and the site never passes off a number as
measured.

### The merge queue runs first

Only a landing fills tailrace's cache, so the first run is the merge queue's:
sluice tests a candidate, its checks run and pass, and the candidate lands.
The worktree, the second worktree and CI are the next three, replaying what
the landing left. The changed input comes in the queue's next candidate too,
because only a run on a candidate goes into sluice's record; a worktree's own
run is recorded nowhere.

### The layout

900 wide, as the other diagrams, and about 470 high, one shell's 446 with
room for the tools line, the same as one agent, one worktree's.

```
 Never prove twice   a check that passed on these inputs is not run again
 ┌ sluice's merge queue ──┐   ┌ tailrace's cache ──────────┐   ┌ worktree a ──────────┐
 │ candidate 1 · landed   │──▶│ filled by landings         │──▶│ check prose build    │
 │ candidate 2 · .vale.ini│◀──│ check  3f9a  pass          │   │ ▪ ▪ ▪   replayed     │
 │ check ▪   replayed pass│   │ prose  a17c  pass          │   └──────────────────────┘
 │ prose ▓▓▓▓▓ ran   fail │   │ build  5e02  pass          │   ┌ worktree b ──────────┐
 │ build ▪   replayed pass│   │ prose  c4d8  a fail never  │──▶│ ▪ ▪ ▪   replayed     │
 └────────────┬───────────┘   │              lands here    │   └──────────────────────┘
 ┌ sluice's record ───────┐   └────────────────────────────┘   ┌ CI ──────────────────┐
 │ every run, pass & fail │                                 ──▶│ ▪ ▪ ▪   replayed     │
 │ check  3f9a  pass      │                                    └──────────────────────┘
 │ prose  a17c  pass      │
 │ build  5e02  pass      │
 │ prose  c4d8  fail      │
 └────────────────────────┘
 (1) The queue's run…   (2) The next three replay…   (3) One input changes…   (4) sluice's record…
 moon declares what each check reads; tailrace keeps the cache; sluice tests, lands and records.
                                                         tailrace's page ↗   sluice's page ↗
```

- **sluice's merge queue** is a panel box (`.box`) at the top left, with a
  header line naming its candidate, `candidate 1`, which turns to
  `candidate 1 · landed` and stays, dimmed, when `candidate 2 · .vale.ini
  changed` arrives beneath it. Its three rows list the checks, each with a
  status bar, a word, `ran` or `replayed`, and the outcome. A run's bar fills
  over seconds; a replay's bar fills in a blink. When candidate 2 fails it is
  marked `not landed`.
- **sluice's record** is a second box directly under the queue, joined to it
  by a short wire, titled as sluice's record and saying it keeps every run,
  pass and fail. A row appears for each run as it ends: candidate 1's three
  passes, then candidate 2's `prose` fail in `--bad`. A replay adds no row.
- **tailrace's cache** sits in the middle on a `--soft` band, beside the
  record, titled as tailrace's cache and saying it is filled by landings. It
  has one row per check: its name, its key and `pass` in `--ok`, each empty
  until the landing fills it. Under the three rows a fourth row stands empty
  for `prose`'s new key, labelled `a fail never lands here` in `--muted`.
- **The worktrees and CI** are three compact panel boxes stacked on the
  right, each a title and one row: the three check names, three small bars
  and the word `replayed`.
- **The wires** run from the queue into the cache, and from the cache out to
  each place on the right and back to the queue. Dots in `--accent` travel
  from the queue into the cache for the landing's three entries, and from the
  cache out to a place for each replay, candidate 2's two replays included.
  The fail's dot, in `--bad`, travels from the queue into the record, and
  none travels into the cache.
- **Captions**, numbered 1 to 4 on accent badges under the boxes, as one
  shell's: (1) the queue's run takes its time, and its landing fills the
  cache; (2) the next three replay in a flash; (3) one input changes, and
  only its check runs again; (4) sluice's record keeps the fail the cache
  never remembers.
- **The tools line** sits under the captions, in the muted caption type:
  "moon declares what each check reads; tailrace keeps the cache; sluice
  tests, lands and records", then "tailrace's page ↗" and "sluice's page ↗"
  as the two links (below). It is static, so it reads in the still and with
  motion.

The coder may move boxes and wires to make the drawing read well, as long as
the cache stays in the middle, the record stays beside it and joined to the
queue alone, and each part above is there.

### The links

Each link is an SVG `<a href="…">` around its text, in `--accent`,
underlined, with a visible focus outline in `--accent` under
`a:focus-visible`: `https://agentplot.github.io/tailrace/` around
"tailrace's page ↗" and `https://agentplot.github.io/sluice/` around
"sluice's page ↗". Both are the tools' own sites, the product page the
design note asks for first (tailrace's README names its site; sluice's is
in the family's list in `@agentplot/site-theme`). They open in the same
tab, as the site's other links to family sites do.

They stay inside the SVG because the diagram names the tools, and the
loader leaves them as they are: it renames only `href="#…"` references to
the diagram's own ids, and the page draws the SVG inline, so each is a real
link on the page. They are the file's only external references; every other
rule below stands.

### The cycle

One 16-second linear cycle, the same as one shell's, with every keyframe on
it.

| Share of the cycle | What happens |
|---|---|
| 0–2% | Every place idle, the cache and the record empty |
| 2–22% | The queue's candidate 1: its three checks fill slowly, staggered, each marked `ran` and `pass`; as each ends, its row appears in sluice's record |
| 23–30% | Candidate 1 is marked `landed`; three dots travel from the queue into the cache, and its three rows fill |
| 32–37% | Worktree a replays: three dots out of the cache, its checks `replayed` at once |
| 39–44% | Worktree b replays the same way |
| 46–51% | CI replays the same way |
| 55–60% | Candidate 2 arrives, `.vale.ini changed`; `check` and `build` replay, two dots out of the cache; `prose` shows its new key and misses |
| 60–76% | `prose` runs again, filling slowly, and ends in `fail` |
| 77–83% | The fail's dot travels into sluice's record and its row appears; candidate 2 is marked `not landed`; the cache's empty row flashes and stays empty |
| 83–96% | Everything holds |
| 96–100% | Everything fades back to the start |

The captions light in turn: 1 from 0 to 31%, 2 from 31 to 54%, 3 from 54 to
77%, 4 from 77 to 96%, each dimmed the rest of the cycle. The tools line
never moves.

The coder may move these marks by a few points to make the drawing read well,
as long as the four parts of the story stay in that order and the
reduced-motion moment below still shows everything the spec asks.

### Reduced motion

The same rule one shell uses: under `prefers-reduced-motion: reduce` every
animation is paused at one negative delay, and the captions drop their
animation and show at full opacity. The held moment is 90% of the cycle
(`animation-delay:-14.4s`): the queue showing `candidate 1 · landed` above
candidate 2 with `.vale.ini changed`, `check` and `build` replayed and
`prose` ran and failed, `not landed`; sluice's record with candidate 1's
three passes and the fail; the cache's three entries and its empty row;
worktree a's, worktree b's and CI's checks `replayed`; all four captions lit;
and the tools line with its two links. That is the whole idea in one frame.

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
- no script, no `@import`, no font file and no external reference but the two
  links: the text uses the same system font stacks as one shell's;
- the `<svg>` carries `role="img"` and `aria-labelledby="ttl dsc"` for the
  file on its own; the loader replaces them with the alt text as the label and
  the description as `aria-describedby`.

### Alt text

Written once, set both as the principle's `diagram.alt` and as the SVG's
`<desc>`. A draft the coder may tighten:

> tailrace's cache sits in the middle, and four places run the same three
> checks around it: sluice's merge queue, a worktree, a second worktree and
> CI. The queue tests a candidate first, and its checks take their time and
> pass. sluice records each run, and when the candidate lands, its verdicts
> fill the cache with one entry for each check. The worktree, the second
> worktree and CI then replay those checks from the cache in a flash,
> running nothing. A next candidate changes the Vale configuration, so only
> the prose check runs again while the other two replay. That run fails.
> sluice's record keeps the fail; the cache fills only from landings, so it
> never remembers it. moon declares what each check reads.

Every sentence stays under Vale's 30 words, and none ends on a number: Vale
does not read a full stop after a number as a sentence end (commit `02af2c8`).
The tools keep their lowercase names at a sentence's start, as the glossary
has sluice.

### Alternatives considered

- **The first run in a worktree, as the outline lists the places.** A
  worktree's run fills no shared cache in tailrace: only the archives a
  landing's verdicts name are pushed and synced. Drawing the worktree first
  would show a cache tailrace does not keep.
- **The changed input in a worktree rather than the queue.** It would show the
  rerun, but the fail would then be a worktree's, which sluice's record never
  holds. Putting the change in the queue's next candidate lets the one check
  run again and its fail land in the record in one move.
- **Durations on the runs and replays.** Rejected above: an invented time
  would read as a measurement.
- **The links in the page beside the diagram.** That changes the Method page,
  which this unit does not touch; the page's own links to each principle's
  tools are the separate unit's strip.
- **SMIL animation instead of CSS keyframes.** SMIL ignores
  `prefers-reduced-motion`, and the loader scopes and renames CSS only. CSS
  keyframes, as the other diagrams use.

## Risks / Trade-offs

- [The loader gives the drawn SVG `role="img"`, whose children assistive
  technology treats as one picture, so a screen reader may not reach the two
  links] → the alt text names tailrace and sluice, the links stay reachable by
  keyboard and pointer, and the separate unit's strip links the same pages in
  the page's text; a change to the loader is out of this unit.
- [Six boxes crowd a 900-wide canvas and shrink at phone width] → keep the
  three replaying places to one row each, keep every row to a check name, a
  bar and one word, in the 11.5px mono size one shell uses at its smallest,
  and check the page at phone width.
- [The cache and the record, side by side, read as one box] → give them
  separate titles and boxes with a gap between, the cache on the `--soft`
  band and the record on `--panel`, and run the record's wire only to the
  queue.
- [The tools line and its links are small at phone width] → keep the links'
  text short and their hit areas the full line height, and check them at
  phone width.
- [A colour fixed outside the palette breaks the reader's theme] → every fill
  and stroke goes through a class on the palette variables; the tasks check
  the file for any hex outside the two palette blocks.
