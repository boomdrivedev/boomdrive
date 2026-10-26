# Spec Delta

## Purpose

How a change to what the site shows is done in this repository: one
declared kind of work with a change stage and a preview stage, read from a
brief the repository writes once, instead of a planned change for each page
or diagram.

## ADDED Requirements

### Requirement: The repository declares one content kind

The repository SHALL declare, in `.config/flume.toml` in the family's kind
format, one kind named `content` whose places land through sluice, with two
stages in order: `change`, whose deliverable is commits, then `preview`, a
conversation opened beside the operator that waits on the caller's word. A
failed submission SHALL send the place back to `change`.

#### Scenario: The declaration reads as the kind

- **WHEN** the declaration is read as flume reads it (its `[profiles]` and its `[kinds.content]` table)
- **THEN** it names sluice as the merge queue, and the kind `content` lands, names its places' branches `content`, and has the stage `change` delivering `commits` followed by the stage `preview`, a conversation with `wait = true`, an agent beside `operator` and `on_fail = "change"`

#### Scenario: Each stage's prompt is a file in the repository

- **WHEN** each stage's `prompt` path is resolved against the declaration's own folder, as flume resolves it
- **THEN** it names `docs/prompts/content-change.md` or `docs/prompts/content-preview.md`, and that file exists

#### Scenario: The declaration asks for nothing flume refuses

- **WHEN** a reader checks each stage
- **THEN** each names exactly one agent, the change stage exactly one deliverable, and the preview stage none

### Requirement: The content brief holds what a good change needs

The repository SHALL hold `docs/content-brief.md`, written once and revised
in place, saying what a change to what the site shows reads first, the
conventions of each sort of thing the site shows (a diagram, a page's words,
a figure, a tools strip), at least two finished examples of each sort,
where each sort lands in the repository, and what proves a change.

#### Scenario: Every path the brief names is there

- **WHEN** every file path the brief names is looked up in the repository
- **THEN** each exists

#### Scenario: The brief points to the agreed ideas rather than copying them

- **WHEN** an agent reads the brief for a diagram
- **THEN** the brief sends it to that principle's Diagram line in `docs/method-outline.md` as the agreed idea, and to the outline's table of the tools each principle names for the tools, their pages and the clauses behind them, and says a diagram with no agreed idea there is not drawn

#### Scenario: The brief names its proof

- **WHEN** an agent reads what proves a change
- **THEN** the brief names `site-check` in the devenv shell, the gate's checks through `sluice exec`, Vale with no error or warning on the words written, and the preview's look in light and dark, at phone width, with scripts off and under reduced motion

### Requirement: The change stage works from the brief and a task line

The change prompt SHALL hand its agent the task line and the brief's path,
to be read whole before any edit, and SHALL ask for the task's pages or
diagrams changed, the checks green and the work committed. It SHALL ask for
no OpenSpec change, spec or verify report, and SHALL NOT ask the agent to
submit, merge or push.

#### Scenario: A one-line task is enough

- **WHEN** the change stage is launched with the task line "draw the diagram for two-loops-one-gate"
- **THEN** its prompt carries that line and sends the agent to `docs/content-brief.md`, and the stage ends on the agent's commits, with no file written under `openspec/changes/`

#### Scenario: Sent back with words

- **WHEN** the change stage starts again with the operator's words from a preview, or a failed submission's
- **THEN** its prompt carries those words as what must change

#### Scenario: Something only the operator can give

- **WHEN** the task names a diagram whose idea the outline does not record as agreed
- **THEN** the prompt has the agent stop short with the way flume gives it, naming the idea it needs, rather than draw one

### Requirement: A task may name several pages or diagrams

A content task SHALL be able to name several pages or diagrams in one task
line, and the change stage SHALL change all of them in the one place before
the preview.

#### Scenario: Three diagrams in one task

- **WHEN** a task line names three principles' diagrams
- **THEN** the change stage draws all three in one place and one preview follows

### Requirement: One preview covers the set

The preview prompt SHALL have its agent bring up the place's own site,
served in the place, give the operator the address of every page the task
named, and ask one thing: whether what those pages show now says what the
task asked.

#### Scenario: The operator says yes

- **WHEN** the operator says every page says what the task asked
- **THEN** the agent stops the served site and tells the operator to finish the stage, after which the place lands

#### Scenario: The operator says no

- **WHEN** the operator says a page does not yet say what was asked
- **THEN** the agent writes the operator's words down as what must change, stops short with them, and the place goes back to the change stage carrying them

### Requirement: The site is served in a place on its own port

A worktree SHALL be able to bring up the site's dev server on a port sluice
holds for that worktree, so the site a preview shows is the place's own and
no two places' servers share a port. Releasing the worktree SHALL stop it,
and releasing a place where nothing was served SHALL do nothing and succeed.

#### Scenario: Serving in a place

- **WHEN** an agent runs `sluice exec <base> -- site-serve` in a place
- **THEN** the command prints the site's address on the place's own port, under the site's base path `/flywheel-web/`, exits, and the address keeps serving the place's working tree

#### Scenario: Two places at once

- **WHEN** two places each serve the site
- **THEN** each answers on its own port and shows its own tree

#### Scenario: Release stops it

- **WHEN** `sluice release` runs in the place
- **THEN** the dev server stops and the port no longer answers

#### Scenario: A candidate's release

- **WHEN** sluice runs the release command after a candidate's checks, where nothing was served
- **THEN** it exits zero and the candidate's verdict is unchanged

### Requirement: The repository says content work reads the brief

`CLAUDE.md` SHALL say that a change to what the site shows is content work:
it is read from `docs/content-brief.md` and a task line, and it is not
written up as an OpenSpec change. Until flume runs this repository, a crew
chore whose text is the task line and the brief's path is the same work.

#### Scenario: An agent asked to change a page

- **WHEN** an agent in this repository is asked to change what a page or diagram shows
- **THEN** `CLAUDE.md` sends it to the brief, and nothing tells it to propose an OpenSpec change for it
