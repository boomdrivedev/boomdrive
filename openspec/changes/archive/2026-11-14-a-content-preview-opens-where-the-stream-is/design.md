# Design

## Context

The content kind, `[kinds.content]` in `.config/flume.toml`, runs a change stage and then a preview, a conversation with the operator that waits on the caller's word (site.5; the content-kind spec). The preview's one agent gives `beside = "operator"`.

Flume reads where an agent opens from the launch's settings and the stage's declaration, else from its place's stream (Flume's 32 and 107). At boomdrivedev/flume `2d180f0b`, `crates/flume-core/src/start.rs` opens an agent beside the pane its settings name before it reads any session, and `crates/flume/src/profiles/terminal/herdr/mod.rs`'s `named_pane` refuses a start whose pane no `pane list` entry names by its id or label: "no herdr pane is named operator". The ops role read that refusal on a host of ours on 2026-11-12. A host whose person keeps no pane labelled `operator` stops every content place at its preview.

Flume's 107: "A caller may put it beside a pane the caller names, as a conversation with the operator wants. Where the settings name no place in the terminal, the agent runs in its place's stream's own: the session the stream names for the place's kind's stage, else for its kind (199), else the one the host's configuration names for the stream, else one named by the stream's name".

Flume runs one agent to each start. `crates/flume-core/src/launch.rs` refuses a stage that names other than one agent, and `crates/flume-core/src/settings.rs` takes no session from a kind's declaration. A launch gives one stage's setting by `--agent [<stage>.]<field>=<value>`, `beside` among the fields (`crates/flume/src/commands/launch.rs`).

## Goals / Non-Goals

**Goals:**

- A content place's preview opens on any host, with or without a pane labelled `operator`.
- A person who launches from a pane can keep the preview beside it.
- The site's tests hold the kind's declaration and the brief's line to the content-kind spec.

**Non-Goals:**

- Any change to Flume, its statement or its terminals' profiles.
- Any change to what the preview shows the operator, its prompt, or the change stage.
- Any change to what the site shows (site.5).

## Decisions

### Decision 1: The preview's one agent names no place in the terminal

The preview stage's `agents` holds one empty table:

```toml
[[kinds.content.stages]]
name = "preview"
prompt = "../site/docs/prompts/content-preview.md"
conversation = true
wait = true
agents = [{}]
on_fail = "change"
```

With no `beside` and no `session` of its own, the agent opens in the session the place's stream names for `content`'s `preview`, else for `content`, else the stream's own (Flume's 107 and 199), on every host. A host that wants previews in a session of their own names one for the stage in its stream's table, which is the host's (Flume's 199). The empty table keeps the stage's one agent, which Flume requires, with no settings of its own, so its program is still the pool's account's (Flume's 173).

Not taken: `{ program = "claude" }`, which names a program the preview did not name before. Not taken: Flume opening an agent in its stream's session where the named pane is missing, which is Flume's statement and profiles to change, would open an agent somewhere its settings did not name with no refusal, and leaves a host's pane named in a repository's kind. Not taken: a session named for the stage by each host's stream, which changes nothing while the declaration gives `beside`, since `beside` is taken before any session.

The kind's comment says so. Its paragraph in `.config/flume.toml` gains, after "a no going back to change with their words.":

```toml
# The preview's one agent names no pane or session of a host's terminal,
# so it opens where the place's stream puts it (Flume's 107 and 199), and
# beside a pane only where its launch names one,
# `--agent preview.beside=<pane>` (record 0045).
```

The paragraph's other lines are rewrapped to the same width where the new sentence joins them.

### Decision 2: A launch names the pane the preview opens beside

The caller that has a pane is the one that knows it, which Flume's 107 gives: a launch made from a pane names it, `--agent preview.beside=<pane>`, a herdr pane by its id or its label, `operator` among them, and the preview opens beside it (Flume's 7 and 32, the launch's layer first). A control plane's launch that names a pane reaches Flume through the runner as given, the runner adding nothing (core.15).

`site/docs/content-brief.md`'s section "A task" says so. Its last paragraph gains, after "as a crew chore whose text is the task line and this brief's path.":

```markdown
Its preview opens in the session the place's stream gives it, which
`flume open <session>` shows; a launch made from a pane puts the preview
beside that pane instead by naming it, `--agent preview.beside=<pane>`, a
herdr pane by its id or its label.
```

The brief is the site's design under `site/docs/`, which names no host of ours (site.7), and these words name none.

### Decision 3: The site's tests hold the kind and the brief

`site/check-site.sh` already resolves each stage's prompt by awk. After that block it gains one more, which reads `../.config/flume.toml` with Bun's TOML parser, `Bun.TOML.parse`, through `bun -e` as the script reads the site theme and the quick start, so the declaration is read as TOML and its inline tables as flume reads them. It fails, naming what differs, unless:

- `profiles.merge_queue` is `sluice`;
- `kinds.content.lands` is `true` and `kinds.content.branch` is `content`;
- the stages' names are `change` then `preview`, and each stage has exactly one entry in `agents`;
- `change` delivers `["commits"]` and is no conversation;
- `preview` has `conversation = true`, `wait = true`, `on_fail = "change"` and no `deliverables`;
- `preview`'s one agent has neither `beside` nor `session`.

It then reads `docs/content-brief.md` from the line `## A task` to the next heading and fails unless that section holds `--agent preview.beside=` and `flume open`. Each pass says one line in the script's form, `check-site: …`, as the other blocks do:

- `check-site: the content kind lands through sluice, change then preview, one agent to each stage, the preview a conversation that waits and sends back to change`
- `check-site: the content kind's preview names no pane or session, opening where its place's stream puts it`
- `check-site: the content brief says a launch names the pane its preview opens beside, and flume open shows the session`

These prove the content-kind delta's scenarios "The declaration reads as the kind", "The declaration asks for nothing flume refuses", "The preview names no place in the terminal" and "The brief says where the preview opens". `site:test` already takes `/.config/flume.toml` as an input. It gains `docs/content-brief.md` among its inputs in `site/moon.yml`, so a change to the brief runs the check again.

Not taken: awk over the TOML, which reads lines and would take `agents = [{ beside = "operator" }]` and a multi-line inline table differently.

### Decision 4: The record and the rule

Record 0045 says the decision, with the options not taken. site.5's details rule out a pane or a session in the kind's declaration and leave a launch naming the pane, both citing it. Both are written with this proposal, as the change's decision, and the tasks hold them to `root:adrs`.

## Rules relied on

- **site.5**: content work runs as the `content` kind, and an OpenSpec change may change how the kind is checked; this change shows nothing new on the site.
- **site.7**: the brief names no host of ours, and neither does any file this change writes.
- **core.15**: the runner passes a launch as its control plane gave it, so a launch's `beside` reaches Flume whole.
- **core.41**: no host of ours is named in the record, the change or the check.
- **core.7**: `.config/flume.toml` stays taplo's and `site/check-site.sh` the formatter's it is now; the check is a task of the moon project `site`.

The design reaches no control plane (core.2) and relies on no section of `site/docs/site-design.md`.

## Risks / Trade-offs

- A launch that names no pane opens the preview in the stream's session, where the person must open it with `flume open` to answer. The brief says so, and the preview's first line names the place and its address.
- Bun's TOML parser reads the declaration, and Flume's own reader is weir's. Both read TOML 1.0, and the check reads only the keys the spec names.
