# Content brief

What a change to what the site shows reads first, and everything a good page
or diagram needs. A content task is one line naming the pages or diagrams to
change; this brief is the rest of its context, written once and revised in
place when a convention changes. Read it whole before editing anything.

## Read first

- `docs/site-design.md`: the site's decisions, its map, the content model,
  how Home and the Method read (Every principle gets a diagram; Motion never
  hides content; the "In the tools" strip), the Tools hub and the house
  style. It settles what a page may say and how it reads.
- `docs/method-outline.md`: each principle as its place on the Method reads,
  its Diagram line, the table of the tools each principle names, and the
  Method's changelog. It settles what a principle says and what its diagram
  draws.
- `src/content/terms/terms.yaml`: the one vocabulary. Words on the site use
  its terms in its forms; a new term is added there, never in `styles/`,
  whose files `scripts/vocab.ts` generates from it.
- `.vale.ini` and `styles/Flywheel/`: the house style Vale holds
  `src/content/` to. An error fails the checks, and a warning on words you
  wrote is fixed, never left.
- `CLAUDE.md`: the rules every change holds to. Nothing commercial; no client
  named and no example from client work; a field note never rewritten; the
  Method revised in place with a version and a changelog entry; every tool
  from `devenv.nix`; every internal link through `url()`.

## The agreed ideas

A principle's Diagram line in `docs/method-outline.md` is the brief for its
diagram: the idea agreed with the operator before it is drawn. Draw every
part the line names and nothing that changes it. A line that says "drawn"
and gives a path means the diagram exists there; its idea is then what the
SVG and its alt text show, and a change that alters it is agreed with the
operator first.

The outline's table of the tools each principle names gives, for each
principle, its tools, the address each link lands on (the section of the
tool's own product page that does what the principle says, or of its design
book where the page has none, and no link where neither is public), and the
clauses behind it. The clauses never appear on the site: Vale's `Clauses`
rule flags one.

A diagram whose idea the outline does not record is not drawn. Stop and ask
the operator for the idea; the outline records it, and then it is drawn.

This brief points to the outline and does not copy it: the outline is
revised in place, and a copy would drift.

## Each sort of thing the site shows

### A diagram

One standalone SVG per principle, drawn by hand and committed as it is,
with no generator script.

- **Size.** 900 wide, with a height near the drawn ones' (446 to 476), so
  the Method's figure column keeps one height across principles.
- **Labelling.** The `<svg>` carries `role="img"` and
  `aria-labelledby="ttl dsc"`. Its `<title id="ttl">` reads "the principle's
  title: its line", and its `<desc id="dsc">` holds the alt text.
- **Header.** The principle's title in bold at the top left, its line beside
  it in muted small type.
- **One style block.** The dark palette on `:root`, the light one under
  `@media (prefers-color-scheme:light)`, with the drawn diagrams' values
  copied exactly, so the file reads correctly opened on its own. Every
  colour is one of the family's ten diagram colours, by token: `--bg`,
  `--panel`, `--fg`, `--muted`, `--rule`, `--accent`, `--soft`, `--ok`,
  `--bad`, `--ink`.
- **Colour roles.** `--panel` and `--rule` for boxes; `--muted` for wires,
  ghosts and secondary text; `--soft` for bands and tracks; `--accent` for
  what is happening now; `--ok` for what has settled; `--bad` only for a
  refusal, a fail or a strike; `--ink` for text on an accent badge.
- **Type.** The system font stacks the drawn diagrams use, mono for keys,
  commands and names. No script, no `@import`, no font file, and no external
  reference but a tools line's links.
- **Motion.** CSS keyframes, never SMIL: SMIL ignores reduced motion, and the
  page renames only CSS. Every animation runs on one 16-second linear
  infinite loop, keyframes named `k-` and a short name. The story plays in
  order, everything holds near the end of the loop (about 83 to 96 per cent),
  then fades and starts again.
- **Reduced motion.** The rule every drawn diagram carries pauses each
  animation at 14.4 seconds, 90 per cent of the loop, inside the hold, where
  the whole idea shows at once, and lights every caption:

  ```css
  @media (prefers-reduced-motion:reduce){*{animation-play-state:paused!important;animation-delay:-14.4s!important}
  .cap{animation:none!important;opacity:1}}
  ```

  Anything that is gone at that moment, such as a pulse ring, has a static
  mark drawn beside it so the still frame still says it.
- **Captions.** Where the story has parts, four numbered captions on accent
  badges along the bottom, each with the class `cap`, lit while its part
  plays and dimmed to about a third the rest of the loop.
- **Tools line.** Where the principle names tools, a line under the captions
  in muted caption type says what each tool does in the drawing, then links
  each tool that has a page: an SVG `<a href>` around "the tool's page ↗",
  in `--accent` and underlined, with a visible focus outline,
  `a:focus-visible{outline:2px solid var(--accent);outline-offset:2px}`. The
  line never moves, and its links are the file's only references outside
  itself.
- **Alt text.** Written once: the same words are the principle's
  `diagram.alt` and the SVG's `<desc>`. It says every part of the idea and
  every tool the diagram names, in the glossary's terms. Every sentence stays
  under 30 words and none ends on a number, since Vale does not read a full
  stop after a number as a sentence's end. A tool's name stays lowercase at a
  sentence's start where the glossary marks it so. Vale does not read front
  matter, so `site-check` never sees the alt text: hold it to the house style
  yourself, for instance by running `vale` on a scratch Markdown file whose
  body is the alt text.
- **No number passed off as measured.** No durations, no counts that read as
  a measurement; keys and hashes are short and plainly illustrative.

What the page does to it: the Method draws each diagram inline through
`inlineDiagram` in `src/lib/diagram.ts`. It scopes every rule in the style
block under the diagram's own class, so no rule may target the `<svg>`
element itself; renames the keyframes and ids after the principle, so keep
ids to the `<title>` and `<desc>`; drops the file's palette and maps the ten
colours onto the page's tokens, so the diagram follows the reader's light or
dark setting; and replaces the labelling with the principle's alt text as
the label and the `<desc>` as its description.

**Lands as** an SVG in `public/diagrams/`, named for the principle's id, and
`diagram: { src, alt }` in the principle's file in `src/content/principles/`,
with `src` the file's path under `public/`. The Method page then shows it in
place of the principle's outlined numeral, beside the write-up on a wide
screen and after the claim on a phone.

**Examples.** `public/diagrams/capture-at-the-source.svg` with
`src/content/principles/capture-at-the-source.md`, and
`public/diagrams/never-prove-twice.svg` with
`src/content/principles/never-prove-twice.md`, whose tools line is the model
for one. The worked reasoning behind each, the layout, the loop's timeline,
the held moment and the alt text, is in
`openspec/changes/archive/2026-10-25-capture-at-the-source-diagram/design.md`
and `openspec/changes/archive/2026-10-25-never-prove-twice-diagram/design.md`.

### A page's words

Markdown or YAML under `src/content/`, each type a collection whose schema is
in `src/content.config.ts`: a missing field or a broken reference fails the
build. Vale reads the body of every Markdown file there.

- A principle is a place on the Method page, at its id's anchor, never a page
  of its own, and every link to a principle goes there.
- A principle's claim or idea is revised in place, with an entry in the
  outline's changelog in the shape its entries take: the date, the
  principle, the kind, the summary and what prompted it.
- A field note is never rewritten once published; a later note supersedes it
  through `supersededBy`.
- Internal links go through `url()` in `src/lib/url.ts`, never a bare root
  path. In content, another entry is named in front matter (a note's
  `principles`, a term's `principle`), where the build checks it.
- A sample point is flagged wherever it appears, Home never reads one, and a
  tool's status is never overstated. The family's members, lines, statuses
  and sites come from the site theme's family list, never a list of the
  site's own.

**Lands as** the file it changes, or a new file in its collection's folder.

**Examples.** `src/content/principles/never-prove-twice.md` and
`src/content/pages/tools.md`.

### A figure

An Astro component under `src/components/`: inline SVG drawn in the site's
own tokens (the theme's `--ap-` colours), moving by CSS keyframes so it plays
with scripts off, with a still under reduced motion that holds one moment
where the whole picture shows. A control, such as the attention figure's
pause button, only where the site design asks for one.

**Lands as** the component and the page under `src/pages/` that sets it.

**Examples.** `src/components/Attention.astro` and
`src/components/Flywheel.astro`.

### A tools strip

A principle's `tools` in its front matter, each a `name` and an `href`, the
`href` left out for a tool with no public page, in the order the strip shows
them. The outline's table holds the same names and addresses, with the
clauses behind each, so a strip and its row change together. A link lands on
the section of the tool's own page that does what the principle says; check
each address live before it goes in. The Tools hub
(`src/pages/tools.astro`) names the principle each family member carries,
and the build fails if that principle's strip does not name the tool.

**Lands as** the principle's front matter and the outline's row.

**Examples.** `src/content/principles/one-agent-one-worktree.md` and
`src/content/principles/never-prove-twice.md`.

## What proves a change

- `site-check` in the devenv shell passes: the types and schemas, the prose,
  then the build.
- Vale reports no error and no warning on the words you wrote, the alt text
  included, checked as the diagram's conventions say.
- `sluice exec <base>` on the committed work passes: the gate's own verdict,
  the moon tasks tagged `check-fast`.
- In the preview, wherever the look is the point, each page the task names
  is looked at in light and dark, at phone width, with scripts off and under
  reduced motion.

## A task

One line naming the pages or diagrams, several at once when the operator
wants to see them together; all of them change in one place, and one preview
covers the set. It runs as the `content` kind declared in
`.config/flume.toml`, or, until flume runs this repository, as a crew chore
whose text is the task line and this brief's path.
