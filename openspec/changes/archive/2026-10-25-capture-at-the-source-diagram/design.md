# Design

## Context

Three principles already have diagrams: `public/diagrams/one-shell.svg`,
`order-at-landing.svg` and `intents-and-bolts.svg`. Each is one standalone
SVG, 900 wide, with a `<title>`, a `<desc>`, one `<style>` holding its own
palette (dark on `:root`, light under `prefers-color-scheme: light`), CSS
keyframes on one 16-second loop, and a reduced-motion rule that pauses
every animation at a chosen moment. A principle names its diagram in its
front matter (`diagram: { src, alt }`, `src/content.config.ts`), and the
Method page draws it inline through `inlineDiagram` (`src/lib/diagram.ts`):
the stylesheet is scoped to the diagram's class, keyframes and ids are
renamed after it, the file's own palette is dropped and the family's
diagram colours are mapped onto the page's tokens. So the work here is the
drawing and two lines of front matter.

The diagram's idea is fixed by `docs/method-outline.md` (3. Capture at the
source: Diagram). Its words follow the glossary
(`src/content/terms/terms.yaml`: capture, signal, adapter, inbox) and the
capture design (`agentplot/blueprints design/capture/design.md`): an
adapter lists a source's events and keys each one; the inbox dedupes by
key; a reader, a session, reads a capture that is not its own excerpt into
signals.

## Goals / Non-Goals

**Goals:**

- One drawing that tells the principle's story in one loop, in the same
  manner as `intents-and-bolts.svg`: header, column labels, the story, a
  struck counter-example, four numbered captions.
- A reduced-motion still frame that shows the whole story.

**Non-Goals:**

- Curation, items, flume or anything after signals. The agreed idea stops
  at signals.
- Changes to the Method page, `src/lib/diagram.ts` or the principle's
  write-up, version or changelog.
- A generator script. The other diagrams were committed as hand-finished
  SVG; this one is too.

## Decisions

### The layout

viewBox `0 0 900 476`, the size of `intents-and-bolts.svg`, so the figure
column keeps one height across principles. Coordinates are a guide; the
coder may move things a few pixels for fit.

- **Header** (y 32): "Capture at the source" in `.h`, then in muted small
  type "work begins as captures, taken unattended where things happen" (the
  principle's line).
- **Column labels** (y 62, `.hd`): SOURCES (x 20), ADAPTERS (x 196), ONE
  INBOX (x 400), A READER SESSION (x 680).
- **Sources** (x 20, width 160, four rows of height 50 at y 80, 142, 204,
  266): each a `.box` with its kind in bold and an example in muted small
  type: "meeting" / "planning call", "chat" / "#ops", "log" / "export
  alerts", "pull request" / "#212 conversation". Each has a small event dot
  at its right edge, always shown.
- **Adapters** (x 196, width 110, height 30, centred on each source row): a
  `.box` reading "adapter", joined to its source by a short line. Adapters
  carry no judgement in the drawing: no colour change, no decision.
- **Connectors** from each adapter's right edge to the inbox's left edge
  (x 306 to 400), `.conn` paths drawn with `pathLength="100"`.
- **The inbox** (x 400, y 80, 250 by 236): a `.box` holding four row slots,
  one per source in the order the captures arrive. A row shows the capture's
  key in mono: `meeting/planning/2026-10-21`, `chat/ops/2026-10-21`,
  `log/export-alert/4812`, `pr/212`.
- **The reader session** (x 680, y 80, 200 by 236): a `.box` reading
  "reader session" with "a session, never unattended" in muted small type
  under it, then three signal chips stacked: "ask · retry rate-limited
  exports", "constraint · three retries at most", "question · who owns
  exports?". The chips' kind is in bold; the words carry on from
  `intents-and-bolts.svg`, whose intent is "Exports fail when the API
  rate-limits us."
- **The ticket form** (x 20 to about 560, y 340 to 400): a ghosted
  (`.ghost`, dashed `--muted`) card titled "New ticket", with two labelled
  blank lines, "Title" and "What needs doing?", each filled with a wavy
  scrawl path so it reads as hand-written. Beside it in muted small type:
  "a ticket someone remembered to write". A `--bad` line strikes it
  through, drawn like `intents-and-bolts.svg`'s `.gstrike`.
- **Captions** (y 438 to 455, badges at x 32, 250, 468, 686, as in
  `intents-and-bolts.svg`): 1 "Events happen at / their sources." 2
  "Adapters write captures, / unattended." 3 "The same event twice / is one
  capture." 4 "A reader session turns / a capture into signals."

### The loop

One 16-second linear infinite loop, every animation a CSS keyframe in the
file's own `<style>`, as the other diagrams do. Times are percentages of
the loop:

| When | What happens |
|---|---|
| 0–14% | The meeting's event pulses (a ring grows and fades from its dot); a capture token travels its adapter's connector into the inbox; row 1 fills. Caption 1 lit. |
| 12–24% | The same for the chat; row 2 fills. |
| 22–34% | The same for the log; row 3 fills. Caption 2 lights at 22%. |
| 32–44% | The same for the pull request; row 4 fills. |
| 46–58% | The log pulses again; a second token with the same key travels in and merges into row 3, which flashes `--accent` and gains a "sent twice · one capture" tag. No fifth row appears. Caption 3 lit. |
| 60–76% | Row 1, the meeting's capture, is outlined in `--accent`; a connector draws from it to the reader session, whose border lights; the three signal chips appear one after another (about 66%, 70%, 74%). Caption 4 lit. |
| 78–82% | The strike draws across the ticket form. |
| 82–96% | Everything holds. |
| 96–100% | Everything that arrived fades and resets for the next loop. |

The ticket form is drawn from the start, ghosted; only its strike moves.

### Reduced motion

`@media (prefers-reduced-motion:reduce){*{animation-play-state:paused!important;animation-delay:-14.4s!important} .cap{animation:none!important;opacity:1}}`,
the rule the other diagrams use. 14.4 seconds is 90% of the loop, inside
the hold, where every row, the sent-twice tag, the reader session's
signals and the strike are shown. Pulse rings are mid-fade or gone there,
so each source's event dot is drawn static and always shown; the source
still reads as where an event happened.

`inlineDiagram` renames keyframes and scopes selectors, so the `*` and
`.cap` rules land under the diagram's own class and touch nothing else on
the page.

### Colour

Only the ten family diagram colours. The file's `:root` block and its
`prefers-color-scheme: light` block copy `intents-and-bolts.svg`'s values
exactly, so the file reads correctly opened alone; on the Method both are
dropped and the page's tokens apply. Roles follow the other diagrams:
`--panel` and `--rule` for boxes, `--muted` for connectors and ghosts,
`--accent` for what is happening now (a capture in flight, the merging
row, the picked capture), `--ok` for what has settled (filled inbox rows,
signals), `--bad` only for the strike, `--ink` for text on `--accent`.

### Alt text

The same text is the SVG's `<desc>` and the principle's `diagram.alt`, as
`one-shell.svg` does:

> Each of four sources, a meeting, a chat, a log and a pull request, has
> an adapter beside it. Events pulse at the sources, and each adapter
> writes them, unattended, as captures that flow into one inbox. The log's
> alert is sent twice; both copies carry the same key, and the inbox keeps
> one capture. A reader session reads the meeting's capture into signals:
> an ask, a constraint and a question. Below, a hand-written ticket form,
> which the principle rules out, is struck through.

Every sentence is under 30 words and none ends on a number, which Vale
misreads (commit 02af2c8). The `<title>` is "Capture at the source: work
begins as captures, taken unattended where things happen".

### Alternatives considered

- **A dot per event rather than a keyed row.** Rows with keys are what
  makes "the same event twice becomes one capture" legible: the reader sees
  the key match. A dot merging into a dot says less.
- **The chat as the repeated event.** A monitor's webhook retried is the
  commonest real duplicate, and the log is the source the design lists as
  "log or monitor webhook", so the log carries it.
- **Showing curation after the signals.** The agreed idea ends at signals;
  adding curation would change an idea the operator signed off.

## Risks / Trade-offs

- [The key strings are long for a 250-wide inbox] → Set them in 11.5px
  mono; if one still overflows, shorten the date segment (`10-21`) rather
  than widen the inbox.
- [Four columns crowd at phone width, where the figure is about 360 wide,
  so text renders near 5px] → The other diagrams share this; the alt text
  and the write-up carry the content on a phone. Keep labels short; do not
  add a phone layout in this change.
- [The reduced-motion frame misses an element whose fade-in ends after
  90%] → Every element's arrival ends by 82%; verify the still frame by eye
  under emulated reduced motion.
- [Hand-made keyframes drift out of step] → Keep every timing in the
  table above and check one loop by eye in both themes.
