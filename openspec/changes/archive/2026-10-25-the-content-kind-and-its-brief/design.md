# Design

## Context

Six diagrams are drawn (`public/diagrams/`): one shell, intents and bolts and
order at landing came as an SVG and a `diagram` entry in front matter; capture
at the source, one agent one worktree and never prove twice each came as a full
change (`openspec/changes/archive/2026-10-25-*-diagram/`). In each of those,
the useful part was the design's Context section: the drawn diagrams'
conventions, the outline's agreed idea, the glossary, the design notes behind
the tools. That context is the same for every diagram, and most of it holds
for any change to what the site shows. The proof each relied on was the gate:
`site-check` and the moon tasks tagged `check-fast`.

flume reads a repository's kinds from `.config/flume.toml` at the base's
revision (`agentplot/blueprints design/flume/requirements.md` 19, 20). A
kind's declaration holds its stages in order, its branch prefix and whether it
lands; a stage holds its name, its prompt, one deliverable or that it is a
conversation, and may say it waits on its caller's word and where its agent
runs (23). C.2 gives the TOML: prompt paths are relative to the declaration's
own file; a kind that lands needs `merge_queue = "sluice"` under `[profiles]`
or it is refused; `on_fail` is read on a kind's last stage only; a prompt's
parts are `{{brief}}`, `{{name}}`, `{{kind}}`, `{{stage}}`, `{{branch}}`,
`{{base}}`, `{{place}}`, `{{change}}`, `{{report}}`, `{{words}}`,
`{{unready}}` and `{{needs}}`, with `{{#words}}…{{/words}}` rendered only when
there are words. In flume's words the *brief* is the task as the caller wrote
it; this repository's *content brief* is a different thing, a standing file,
and the prompts keep the two apart by calling the first "the task".

flume is not installed on this host and does not yet run this repository.
Until it does, the same shape runs as a crew chore whose text is the task line
and the brief's path (`a-repositorys-content-kind.md`, The kind: content).

sluice gives a worktree ports and `SLUICE_PLACE` under `sluice exec`, from
`[checkout] ports`; a server kept up is handed by the command to a tool that
keeps it, such as `devenv up -d`, and stopped by `[tests] release` on `sluice
release`, which also runs after every candidate's checks (sluice's
`using-sluice` references: servers, checkout). flywheel-web's policy holds no
ports and no release command today. devenv 2.4 is the shell; its `devenv up
-d` starts declared processes in the background and `devenv processes down`
stops them.

Design sections this rests on: `docs/site-design.md` How Home and the Method
read (Every principle gets a diagram; Motion never hides content; the "In the
tools" strip), The Tools hub, and The house style; `docs/method-outline.md`
each principle's Diagram line and The tools each principle names.

## Goals / Non-Goals

**Goals:**

- A content task is one line plus the brief, and nothing is proposed,
  specified or verified as a change: the commits and the gate are the record.
- The kind and its prompts are generic; only the brief is flywheel-web's.
- The preview shows the operator the place's own site, on the place's own
  port, through sluice.

**Non-Goals:**

- No page, diagram, component, schema or glossary term changes. The remaining
  diagrams are the first content task after this lands.
- No `[preview]` table for `sluice preview` of a landed revision; the preview
  stage serves the place before landing.
- No change to crew, flume, sluice or the family's other sites. The sluice and
  tailrace sites and the books take the kind by writing their own briefs.
- No change to `openspec/config.yaml`: it still binds any change this
  repository does write, which is now work other than content.

## Decisions

### The declaration

```toml
# .config/flume.toml
[profiles]
merge_queue = "sluice"

[kinds.content]
branch = "content"
lands = true

[[kinds.content.stages]]
name = "change"
prompt = "../docs/prompts/content-change.md"
deliverables = ["commits"]
agents = [{ program = "claude" }]

[[kinds.content.stages]]
name = "preview"
prompt = "../docs/prompts/content-preview.md"
conversation = true
wait = true
agents = [{ beside = "operator" }]
on_fail = "change"
```

The file carries a short comment saying what the kind is for and that the
brief tailors it. `place` is left out: the repository has no worktrunk
configuration, so flume's default, plain `git worktree`, is what it would use.
`change` is left out: the kind reads no change. No `version`, so the kind is
`content@1`. No `quiet` limit: the change stage takes flume's default.

*Alternative:* a chore kind with one stage and no preview. Rejected: the
operator asked for a preview, and the preview is where the "is this what I
asked for" question belongs, not in a proposal read before the work.

### The content brief

`docs/content-brief.md`, in the repository's plain-prose doc style, holds:

1. **Read first.** `docs/site-design.md`, `docs/method-outline.md`,
   `src/content/terms/terms.yaml` and `.vale.ini` with `styles/Flywheel/`,
   and `CLAUDE.md`'s rules, each with one line on what it settles.
2. **The agreed ideas.** A principle's Diagram line in the outline is the
   brief for its diagram; a line reading "drawn, `<path>`" means drawn. The
   outline's table of the tools each principle names gives the tools, the
   section of each tool's page a link lands on, and the clauses behind it,
   which never appear on the site. A diagram whose idea is not in the outline
   is not drawn: the agent stops and asks. The brief points to these and does
   not copy them, since the outline is revised in place and a copy would
   drift.
3. **Each sort, its conventions, examples and landing place:**
   - *A diagram:* one standalone SVG, 900 wide, with `<title>`, `<desc>` and
     one `<style>` carrying the dark palette on `:root` and the light under
     `prefers-color-scheme: light`, the ten family colours by token and their
     roles (accent for what is happening now, ok for what has settled, bad
     only for a refusal or a strike, muted for wires and ghosts); CSS
     keyframes on one 16-second loop and never SMIL; the reduced-motion rule
     that pauses every animation at a moment in the hold where the whole idea
     shows, with captions lit; four numbered captions where the story has
     parts; a tools line with each tool's page as an SVG link with a visible
     focus outline; alt text identical to `<desc>`, every sentence under 30
     words and none ending on a number. What `inlineDiagram`
     (`src/lib/diagram.ts`) does to it: scopes the styles, renames keyframes
     and ids, maps the colours onto the page's tokens, replaces the labelling.
     Lands as `public/diagrams/<principle>.svg` plus `diagram: { src, alt }`
     in `src/content/principles/<principle>.md`. Examples:
     `capture-at-the-source.svg` and `never-prove-twice.svg`, with the
     archived designs of their changes as the worked reasoning.
   - *A page's words:* Markdown or YAML under `src/content/`, checked by its
     collection's schema (`src/content.config.ts`) and by Vale. A principle is
     a place on the Method, never a page; a revised claim or idea gets an
     entry in the outline's changelog; a field note is never rewritten.
     Internal links through `url()`. Examples:
     `src/content/principles/never-prove-twice.md` and
     `src/content/pages/tools.md`.
   - *A figure:* an Astro component under `src/components/`, inline SVG with
     CSS keyframes, a still for reduced motion, the site's tokens. Examples:
     `Attention.astro` and `Flywheel.astro`.
   - *A tools strip:* a principle's `tools` (name, href) in its front matter
     and the matching row of the outline's table. Examples:
     `one-agent-one-worktree.md` and `never-prove-twice.md`.
4. **What proves a change:** `site-check` in the devenv shell; `sluice exec
   <base>` for the gate's own verdict once committed; no Vale error or warning
   on words written; and, in the preview, the pages in light and dark, at
   phone width, with scripts off and under reduced motion, wherever the look
   is the point.
5. **A task:** one line naming the pages or diagrams, several at once if the
   operator wants to see them together.

The coder writes it from the archived designs' Context and Decisions sections
and the SVGs as they are, checking every convention it states against at least
one drawn diagram.

### The prompts

Both are plain Markdown with flume's parts, and name no page of
flywheel-web's: each sends the agent to `docs/content-brief.md`.

`content-change.md`: the task is `{{brief}}`, in place `{{place}}` on
`{{branch}}`, cut from `{{base}}`. Read the content brief whole, then the
pages the task names as they stand; make the change; run `site-check` and
`sluice exec {{base}}` until both pass; commit by explicit paths with a
Conventional Commit. Write no OpenSpec change, spec or report. Never submit,
merge or push. `{{#words}}`The operator, or a failed landing, sent this back:
`{{words}}`. Change what they name.`{{/words}}` Where only the operator can
settle something, such as a diagram with no agreed idea, stop short with
`{{needs}}`, naming what is needed.

`content-preview.md`: the task was `{{brief}}`. Serve the site with `sluice
exec {{base}} -- site-serve` (once; it stays up through a send-back). The
first line the agent shows the operator, before anything else, is the
preview's address with the place's name and port, in the form `Preview of
{{name}} on port <port>: <address>`, the port and address being what
`site-serve` printed. Then it gives the address of each page the task names
and asks one thing: does each now say what the task asked? On yes, run `sluice release` and tell the
operator the stage can be finished. On no, write the operator's words down as
what must change and stop short with them through `{{needs}}`, leaving the
server up, so the caller starts the change stage again with them. Make no
edits yourself.

The address comes first because the operator reaches the preview from the
pane that opened beside them, and the line names the place because several
places may be in preview at once (`a-repositorys-content-kind.md`, The kind:
content). The note also puts the address in the place's status. flume's
status (`requirements.md` 56) gives no field a stage's agent fills, so this
repository makes the address follow from the place's port alone: sluice's
view of the place (`sluice --worktree <place>`) lists its `web` port while
it is held, and that port with `/flywheel-web/` is the address. A field in
flume's status that carries it is flume's to add.

*Alternative:* the preview agent edits on the spot. Rejected: the preview is
a conversation with no deliverable, and an edit there would land without the
change stage's checks.

### Serving the site in a place

- `.config/sluice.toml` gains `[checkout] ports = ["web"]`, so each worktree
  and checkout holds `SLUICE_PORT_WEB`, and `[tests] release = "site-stop"`.
- `devenv.nix` gains a process `site` that runs the site's dev server bound to
  `127.0.0.1` on the port read from `$DEVENV_STATE/site-port`, and two
  scripts. `site-serve` writes `$SLUICE_PORT_WEB` to that file (failing with a
  message when the variable is unset, outside `sluice exec`), runs `devenv up
  -d site`, waits until the port answers, prints
  `http://127.0.0.1:<port>/flywheel-web/` and exits. `site-stop` does nothing
  and exits zero when the port file is absent; otherwise it runs `devenv
  processes down` and removes the file.
- The dev server rather than a build: it reloads as the change stage works
  again after a no, so a second look needs no restart.
- The port goes through a file rather than the environment because whether
  `devenv up -d` passes its caller's environment to the process is not
  something the repository should rest on.

*Alternative:* `sluice preview` at the landed revision. It shows the site only
after landing, which is the wrong side of the operator's yes; it stays
available as a later addition.

### method-diagrams as the generic spec

The 32 per-diagram requirements are removed and eight generic ones added (see
the delta): the diagram takes the numeral's place, draws its agreed idea,
takes the family colours, moves with scripts off, is whole under reduced
motion, has alt text that says what it shows, lights its captions in turn, and
links the tools it names. What a particular diagram draws lives in the
outline's Diagram line and in the SVG itself. Removed requirements carry a
reason and a migration naming where their content now lives.

*Alternative:* leave the per-diagram requirements and add no more. Rejected:
the spec would keep reading as one diagram's contract, and the next diagram's
agent would take it as the pattern to follow.

### CLAUDE.md

One paragraph after the bullet on the glossary: a change to what the site
shows (a diagram, a page's words, a figure, a tools strip) is content work,
read from `docs/content-brief.md` and a one-line task, run as the `content`
kind in `.config/flume.toml` or, until flume runs here, as a crew chore whose
text is the task and the brief's path; it is not written up as an OpenSpec
change.

## Risks / Trade-offs

- [`devenv up -d` started under `sluice exec` may not outlive the command's
  process group] → sluice's own servers reference names `devenv up -d` as the
  hand-off; the task proves it by serving, letting `sluice exec` exit, and
  loading the address afterwards. If it does not survive, stop and report
  rather than background the server some other way.
- [`[tests] release` now runs after every candidate's checks on every bolt and
  on main] → `site-stop` is a no-op without the port file, which no candidate
  has; the task runs it in a fresh checkout and checks it exits zero.
- [Running devenv inside sluice's devenv toolchain] → the task proves the
  serve and the stop under `sluice exec` itself, not in a bare shell.
- [flume is not installed, so the declaration cannot be read by flume yet] →
  the task parses it as TOML and checks every field against C.2; once flume
  runs here, `flume prompt <place> <stage>` renders each prompt and is the
  real check.
- [flume's status does not yet carry the preview's address] → the address is
  made from the place's port alone, which sluice's view of the place lists;
  the task proves the two agree while the site is served.
- [The brief drifts from the drawn diagrams] → it points to the outline and
  the SVGs rather than copying them, and is revised in place when a
  convention changes.

## Migration Plan

Nothing to migrate: no published page changes. The method-diagrams spec is
rewritten at archive. Rollback is reverting the commit.
