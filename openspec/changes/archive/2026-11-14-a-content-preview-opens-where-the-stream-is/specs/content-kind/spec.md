## MODIFIED Requirements

### Requirement: The repository declares one content kind

The repository SHALL declare, in `.config/flume.toml` in the family's kind
format, one kind named `content` whose places land through sluice, with two
stages in order: `change`, whose deliverable is commits, then `preview`, a
conversation that waits on the caller's word. The preview's one agent SHALL
name no pane or session of a host's terminal, so flume opens it where the
place's stream puts it (record 0045). A failed submission SHALL send the
place back to `change`.

#### Scenario: The declaration reads as the kind

- **WHEN** the declaration is read as flume reads it (its `[profiles]` and its `[kinds.content]` table)
- **THEN** it names sluice as the merge queue, and the kind `content` lands, names its places' branches `content`, and has the stage `change` delivering `commits` followed by the stage `preview`, a conversation with `wait = true`, one agent and `on_fail = "change"`

#### Scenario: Each stage's prompt is a file in the repository

- **WHEN** each stage's `prompt` path is resolved against the declaration's own folder, as flume resolves it
- **THEN** it names `site/docs/prompts/content-change.md` or `site/docs/prompts/content-preview.md`, and that file exists

#### Scenario: The declaration asks for nothing flume refuses

- **WHEN** a reader checks each stage
- **THEN** each names exactly one agent, the change stage exactly one deliverable, and the preview stage none

#### Scenario: The preview names no place in the terminal

- **WHEN** the preview stage's one agent is read
- **THEN** it gives neither `beside` nor `session`, so the declaration names no pane or session a host may lack

## ADDED Requirements

### Requirement: A launch names the pane its preview opens beside

`site/docs/content-brief.md`'s section "A task" SHALL say that a content
place's preview opens in the session its place's stream gives it, which
`flume open` shows by its name, and that a launch made from a pane puts the
preview beside that pane by naming it in the launch's own settings,
`--agent preview.beside=<pane>`.

#### Scenario: The brief says where the preview opens

- **WHEN** the brief's section "A task" is read
- **THEN** it names `--agent preview.beside=` as how a launch puts the preview beside its pane, and `flume open` as how a person is shown the session the preview opens in otherwise
