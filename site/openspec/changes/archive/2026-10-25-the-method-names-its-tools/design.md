## Context

See proposal.md for why. The design rests on these parts of `docs/site-design.md`: "How Home and the Method read" (a principle is a place on the Method page, its write-up open and its practice folded beneath), "The Tools hub" (a card per member of `@agentplot/site-theme`'s family, each naming the principle it carries, and the two columns from `src/content/pages/tools.md`), "The types" (the principle type) and "The house style" (Vale's rules). What the strips name, and the rule that clause numbers stay in the outline, come from the design note `agentplot/blueprints design/tailrace/models/the-method-names-its-tools.md` (fa05ff3), "Each principle names the tools that realise it" and "What this does not do".

As the code stands:

- A principle's record (`src/content/principles/<id>.md`) has `order`, `title`, `summary`, `line`, `status`, `introducedIn`, `lastReviewed`, `related` and an optional `diagram` (`src/content.config.ts`). The Method page (`src/pages/method/index.astro`) draws each as a scene: count, title, claim, the idea (the body before its first `##`), then a folded `<details class="more">` with the practice, the notes that cite it and its changes.
- The Tools page (`src/pages/tools.astro`) keeps a `serves` map from family member to principle and draws it on each card as "In the Method". sounding maps to `two-loops-one-gate`; the others are headwater, eddy and millpond to `capture-at-the-source`, flume to `one-agent-one-worktree`, sluice to `order-at-landing`.
- tailrace is not in the family list (site-theme v0.1.1, nor its main), so it has no card, status or accent, and `named()` gives its bold name no dot.
- Clause numbers appear only in five principles' text: `capture-at-the-source.md` (Flywheel Next clauses 111, 112, 115 and 215, 116 and 118, 217g), `one-agent-one-worktree.md` (1 and 2), `two-loops-one-gate.md` (4), `order-at-landing.md` (3 and 6, 7, 10 to 12, 17, 18 and 20, 8) and `never-prove-twice.md` (15, 16 and 19). No note, term, page, component, measurement or drawn diagram carries one. The two diagrams in flight on the bolt (`one-agent-one-worktree-diagram`, `never-prove-twice-diagram`) carry none either.

## Goals / Non-Goals

**Goals:**

- One place holds a principle's tools: its own record. The Method draws it; the Tools page is checked against it.
- Every link lands on a section a reader can read today, on the tool's own site.

**Non-Goals:**

- A card, status or accent for tailrace on the Tools page. Those come from `@agentplot/site-theme`'s family, and the site keeps no list of its own (`site-design.md`, The family is one list, and it is not ours).
- Restating a tool's design on the site: a link lands on the tool's own page.
- The two diagrams the design note also describes; they are their own units.
- Clause numbers in `docs/`: the outline keeps them, and Vale reads only `src/content/`.

## Decisions

### The strip is a `tools` list in the principle's frontmatter

```ts
// The tools that do what the principle says, in the order its strip names
// them. Each links the section of its product page that does it, or of its
// design book where the page has none; a tool with neither is named alone.
tools: z.array(z.object({ name: z.string().min(1), href: z.url().optional() })).default([]),
```

`href` is a full address because every target is another site, so the url() rule for internal links does not apply, and a bare path fails the schema. Rejected: a list keyed by family id, with the address built from the family's `site`. It cannot name tailrace, Crew or Flywheel Next, and each link lands on a section, not the site's front page, so the address has to be written anyway.

In each record `tools` goes directly after `line`, before `status`. The diagram units add `diagram` after `lastReviewed`, three unchanged lines below, so git merges the changes to `one-agent-one-worktree.md` and `never-prove-twice.md` cleanly in either order. For the same reason `lastReviewed` is left as it is: the changelog entry records the revision, and editing that line would collide with the diagram units.

### What each principle names and where each link lands

From the design note's table. Every tool with a site has a product page section that does what the principle says, so no link needs a design book today; the rule stays for a later tool. Each address was checked live on 2026-10-25: every page answered and every `#` names a heading on it.

| principle | tool | link | what it lands on | clauses behind it (outline only) |
|---|---|---|---|---|
| One shell | sluice | `https://agentplot.github.io/sluice/reference/commands/#sluice-exec` | `sluice exec`: a check runs in the place's own shell, the one the repository declares | — |
| One agent, one worktree | sluice | `https://agentplot.github.io/sluice/decisions/0016/` | sluice gives a place its ports on the host and releases what its checks hand off | sluice 1, 2, 30, 31, I5; record 0016 |
| | flume | `https://agentplot.github.io/flume/` | one agent launched in one worktree | — |
| Capture at the source | millpond | `https://agentplot.github.io/millpond/` | the inbox records each event once | Flywheel Next 111, 112, 115, 116, 118, 215, 217g |
| | headwater | `https://agentplot.github.io/headwater/` | an adapter that writes captures once | |
| | eddy | `https://agentplot.github.io/eddy/` | an adapter that writes captures once | |
| Intents and bolts | Crew | no link | bolts and units | — |
| | Flywheel Next | no link | a bolt lands on the shared line as one | |
| Two loops, one gate | tailrace | `https://agentplot.github.io/tailrace/how-it-works/#channels-moved-only-by-proof` | the word is a promotion | tailrace 97, C.16 |
| | sluice | `https://agentplot.github.io/sluice/#why` | landing is automatic on green | sluice 4 |
| Order at landing, never at start | sluice | `https://agentplot.github.io/sluice/#why` | submit, combine, blame one, land | sluice 3, 6, 7, 8, 10 to 12, 17, 18, 20 |
| Never prove twice | tailrace | `https://agentplot.github.io/tailrace/#one-store-three-jobs` | the build cache in the one store | tailrace 52 to 55, 100, C.11 |
| | sluice | `https://agentplot.github.io/sluice/reference/state/#the-log` | every run logged, a failure exactly like a pass | sluice 15, 16, 19, 104, 108; core.7 |
| Secrets by declaration | sluice | `https://agentplot.github.io/sluice/reference/configuration/#executor-what-the-tests-may-touch` | a run gets the host's shared caches and nothing else outside its place | sluice 35 |
| Measure the fleet | Crew | no link | the records the collectors read | — |
| | sounding | `https://agentplot.github.io/sounding/` | the records the collectors read | |

Crew and Flywheel Next are named without a link: Crew has no product site or design book, and Flywheel Next's design is private, as Flywheel's own is (`site-design.md`, Flywheel's card links the Method instead: its design book is private). This holds them to the rule the site already keeps, where Crew is named on the Tools page and in the components with no address. The statement (`the-statement.md`) gets no `tools`.

### The strip sits after the write-up, before the fold

In `method/index.astro`, between the idea and `<details class="more">`, a principle with tools gets:

```astro
<p class="tools"><span>In the tools</span> …each tool: <a href={href}>{name}</a>, or <span>{name}</span> with no href…</p>
```

It is drawn in the manner of the Tools card's "In the Method" (`.serves` in `tools.astro`), so the two pages read as one pair: the label in the small mono capitals in `--ap-fg-3`, each name in the display face, links in `--accent-text` with " ↗" after them, as the Tools page marks a link to another site, and an unlinked name in `--ap-fg-2`. Names wrap in a row with a gap. Every colour is a page token, so the strip follows light and dark. As a direct child of `.text` it takes the scene's arrival transition and loses it under reduced motion, as every other line of the write-up does. On a phone, where `.text` is `display: contents` and its children are ordered, it takes `order: 6`, after `.body`. Rejected: inside the fold, where a reader would have to open it; in the figure column, which a diagram fills.

### The Tools page checks its cards against the strips

`principle(id)` in `tools.astro` takes the member's name too, and throws `the Tools hub has <tool> carry "<id>", whose strip does not name <tool>` when the principle's `tools` has no entry with that name. The `serves` map stays the one place a card's principle is chosen, since a tool such as sluice is named by six principles and the card shows one. sounding's entry becomes `measure-the-fleet`, the one principle whose strip names it, and its comment says so. Rejected: deriving each card's principle from the strips, which would have to pick one of sluice's six by a rule nobody has made.

The glossary's `sounding` term keeps `principle: two-loops-one-gate`. The design note does not speak to it, and the check binds only the Tools page.

### tailrace's words on the Tools page

The design note's line is one 34-word sentence, over Vale's 30-word warning, so it is split where its clauses already part, keeping every word:

> - **tailrace** carries what the merge queue lands into a store you own and names each piece by its content. It keeps channels and the environments that follow them, and serves consumers in their ecosystem's own form.

It goes after sluice's entry. In "The Method joins them", after "…and the work lands through sluice.", goes:

> tailrace carries the landing to a channel and to every environment that follows it.

The glossary gains, after `sluice`:

```yaml
- id: tailrace
  term: tailrace
  definition: The tool that carries what the merge queue lands into a store you own, keeps its channels and the environments that follow them, and serves consumers.
  lowercase: true
```

It has no `principle`: each sibling term's principle is the one its card names, and tailrace has no card. `scripts/vocab.ts` regenerates `accept.txt` (and `Terms.yml`, which may come out unchanged) when `site-check` runs.

### Clause citations come out of the five principles

Each parenthetical citation goes, with the link to the statement it carried; the sentence is otherwise unchanged. The strip now names the tool and links its page. Where the citation sat mid-sentence, the sentence closes up:

- `order-at-landing.md`: "The queue never lands it first ([sluice statement](…), clauses 3 and 6), and waiting on a dependency holds back nobody else (clause 7)." becomes "The queue never lands it first, and waiting on a dependency holds back nobody else."; "(clauses 10 to 12)", "(clauses 17, 18 and 20)" and "(clause 8)" go.
- `capture-at-the-source.md`: "(Flywheel Next statement, clause 111)", "(clause 112)", "(clauses 115 and 215)", "(clauses 116 and 118)" and "(clause 217g)" go.
- `one-agent-one-worktree.md`, `two-loops-one-gate.md`, `never-prove-twice.md`: the one "([sluice statement](…), clauses …)" each ends a sentence with goes.

### A Vale rule keeps clause numbers off

`styles/Flywheel/Clauses.yml`, hand-written beside `NotXButY.yml` and in its shape: an `existence` rule at `warning`, ignoring case, on the raw pattern `\bclauses? \d+[a-z]?\b`, with a message saying the outline keeps clause numbers and the page names the tool. It starts at warning, as the house style's ratchet asks, and a warning is fixed, never ignored. Vale reads `src/content/` and not the frontmatter's alt text or the SVGs, so the built site is also searched once (see tasks). The house style table in `site-design.md` gains its row.

### The docs

- `docs/method-outline.md`: a section "The tools each principle names", after Measure the fleet and before the changelog, holding the table above (principle, tools, link, what it lands on, clauses). The opening paragraph's sentence on cited clauses says the site shows none and this outline keeps them. A changelog entry: **2026-10-26 · the Method · revised.** Each principle names the tools that do what it says, in a strip on its place that links each tool's own page; the principles' text drops the statements' clause numbers, which this outline keeps. Prompted by the operator on 2026-10-26: "cross-reference these sections to the tools pages as they're being worked on".
- `docs/site-design.md`: the Principle row of "The types" gains `tools (name, href)`, with its reason; "The Method reads one principle at a time" gains the strip, after the write-up and outside the fold, and that no clause number appears on the site; "The Tools hub" says a card's principle is one whose strip names the tool; the house style table gains `Clauses`.

## Risks / Trade-offs

- [A tool's site renames a heading and a link lands at the top of the page] → the build does not fetch other sites, so the check is a task: every address is fetched and its anchor looked for. A later unit can make it a moon task if it recurs.
- [tailrace is named in the list while the headline counts six tools and no card shows it] → tailrace joins `@agentplot/site-theme`'s family in its own repository, and a bump of the theme here then adds its card, status and accent with no change to this page's words.
- [sounding's card moves from "Two loops, one gate" to "Measure the fleet"] → it follows the design note's table, which is the source; if the operator wants sounding on Two loops, one gate, it is one line in that principle's `tools`, and the card can return.
- [The diagram units and this one edit the same two principle files] → `tools` goes after `line` and `lastReviewed` is untouched, so the hunks never meet.
- [Crew has a public repository a reader could follow] → the site gives Crew no address anywhere today, so the strip names it alone; linking it later is one `href` in two records.
