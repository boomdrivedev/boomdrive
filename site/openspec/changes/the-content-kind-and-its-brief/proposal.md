# Proposal

## Why

Each of the Method's diagrams went through a full change, proposed, applied
and verified, and the operator doesn't want that: a diagram is a few lines
of SVG and front matter, and the proposal, spec and verify report around it
were ceremony. What was worth keeping was the context a good diagram needs,
and that context is the same every time. So this repository writes it down
once, as a brief, and declares one kind of work for changing what the site
shows: change it, then show it to the operator in the place's own site.
(Signal `2026-10-26-madswan-operator-f16c114b/01`; `agentplot/blueprints
design/flume/models/a-repositorys-content-kind.md`.)

## What Changes

- **One content kind, declared.** `.config/flume.toml` declares the kind
  `content` in the family's kind format: places on `content/<name>`, landed
  through sluice, with two stages. **change** delivers commits: what the
  pages show, changed, and the checks green. **preview** is a conversation
  opened beside the operator, who looks at the place's own site served in
  the place and says whether it now says what the task asked; a no goes
  back to change with the operator's words. A failed submission also goes
  back to change.
- **The content brief, written once.** `docs/content-brief.md` holds what a
  change to the site reads first and everything a good page or diagram
  needs: where the agreed ideas are (the outline's Diagram lines, read as
  the briefs they are), the site design's rules, the clauses and the tools'
  pages each principle rests on, the conventions of each sort of thing the
  site shows (a diagram, a page's words, a figure, a tools strip), two
  finished examples of each, where each lands, and what proves a change.
- **Two prompts.** `docs/prompts/content-change.md` and
  `docs/prompts/content-preview.md`, the stages' prompts, which name no page
  of flywheel-web's own: what tailors them is the brief and the task line.
- **A task may name several pages or diagrams**, and one preview covers the
  set: the remaining diagrams can be one task with one look.
- **The site served in a place, on the place's port.** `site-serve` starts
  the site's dev server through `sluice exec`, on a port sluice holds for
  the worktree, and hands it to devenv's process manager; `sluice release`
  stops it. This is what the preview stage opens.
- **The repository says so.** `CLAUDE.md` says a change to what the site
  shows is content work, read from the brief, and is not written up as an
  OpenSpec change; until flume runs this repository, the same shape is a
  crew chore whose text is the task line and the brief's path.
- **`method-diagrams` becomes the generic spec.** Its per-diagram
  requirements (capture at the source, one agent one worktree, never prove
  twice) are replaced by requirements every principle's diagram meets; what
  a particular diagram draws is the idea the outline records as agreed. No
  later diagram adds requirements of its own.

## Capabilities

### New Capabilities

- `content-kind`: how a change to what the site shows is done here: the
  declared kind with its two stages, the brief it reads, the prompts, one
  task naming several things with one preview, and the site served in the
  place for that preview.

### Modified Capabilities

- `method-diagrams`: per-diagram requirements are removed and replaced by
  generic ones that hold for every principle's diagram, the drawn ones and
  those still to come.

## Impact

- New files: `.config/flume.toml`, `docs/content-brief.md`,
  `docs/prompts/content-change.md`, `docs/prompts/content-preview.md`.
- `devenv.nix`: two scripts, `site-serve` and `site-stop`, and one process,
  `site`.
- `.config/sluice.toml`, the gate's policy: a `web` port held for each
  checkout and worktree, and `[tests] release` running `site-stop`, which
  is a no-op where nothing was served (every candidate).
- `CLAUDE.md`: one paragraph on content work.
- `openspec/specs/method-diagrams/spec.md` through this change's delta, at
  archive.
- No page, diagram, component or schema of the site changes; what readers
  see is unchanged.
- Rests on: `docs/site-design.md` (Every principle gets a diagram; Motion
  never hides content; The house style), `docs/method-outline.md` (each
  principle's Diagram line and The tools each principle names), and
  `agentplot/blueprints design/flume/requirements.md` 19, 20, 23 and C.2 for
  the declaration's form.

## Touches

```
.config/flume.toml
.config/sluice.toml
devenv.nix
docs/content-brief.md
docs/prompts/content-change.md
docs/prompts/content-preview.md
CLAUDE.md
openspec/specs/method-diagrams/spec.md
```
