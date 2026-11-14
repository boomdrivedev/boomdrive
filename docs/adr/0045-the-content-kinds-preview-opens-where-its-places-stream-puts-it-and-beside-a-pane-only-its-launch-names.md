---
number: 45
title: The content kind's preview opens where its place's stream puts it, and beside a pane only its launch names
status: accepted
date: 2026-11-14
decision-makers:
- the construct stage of change/a-content-preview-opens-where-the-stream-is
---

# The content kind's preview opens where its place's stream puts it, and beside a pane only its launch names

Status: accepted. The direction is the change's brief. How it is carried out is the construct stage's own choice, within Flume's statement and the site's rules, cheap to change, and none of it turns on a word of the user's.

Deciders: the construct stage of `change/a-content-preview-opens-where-the-stream-is`, from its brief: "the content kind's preview opens beside the pane named operator where the host has one, and in the place's stream's own session where it has none, so a content place passes preview on any host".

Sources: the brief, queued by the design role on 2026-11-12 from the ops role's reading of that day on a host of ours, Flume's refusal "no herdr pane is named operator"; `.config/flume.toml`'s `[kinds.content]`; record 0038 and site.5; boomdrivedev/blueprints `design/flume/requirements.md` at `41bd0a2`, clauses 7, 23, 32, 107, 173 and 199 and the kind format's example; boomdrivedev/flume at `2d180f0b`: `crates/flume-core/src/start.rs`, which opens an agent beside the pane its settings name before any session, `crates/flume/src/profiles/terminal/herdr/mod.rs`, whose `named_pane` refuses a pane no `pane list` entry names by its id or label, `crates/flume-core/src/settings.rs`, which takes no session from a kind's declaration, `crates/flume-core/src/launch.rs`, which refuses a stage naming other than one agent, and `crates/flume/src/commands/launch.rs`, which takes `--agent [<stage>.]beside=<pane>`; rules core.15, core.41 and site.5; the OpenSpec change `a-content-preview-opens-where-the-stream-is`, whose design gives each point below.

## Context and problem statement

The content kind's preview stage gives its one agent `beside = "operator"`. Flume opens an agent beside the pane its settings name before it reads any session its place's stream names, and under herdr it refuses the start where no pane is labelled `operator`. A host whose person never labels a pane so refuses every content place's preview, and the place waits on a stage that cannot open. Which pane a person keeps is the host's terminal state, which a repository's declaration cannot know, as Flume's 32 keeps a host's stream, pool and account out of a repository's kinds.

Flume's 107 says where an agent opens: beside a pane its caller names, and where its settings name no place in the terminal, in the session its place's stream names for the kind's stage, else for the kind, else the stream's own.

## Considered options

1. **The preview's one agent names no pane and no session, `agents = [{}]`.** The preview opens where the place's stream puts it on every host. A launch made from a pane puts it beside that pane by naming it, `--agent preview.beside=<pane>`, and the content brief says so.
2. The preview's agent names `program = "claude"` in place of the pane, as the change stage's does.
3. Flume opens an agent in its stream's session where the pane its settings name is not there.
4. Each host labels a pane `operator`.
5. Each host's stream names a session for the content kind's preview (Flume's 199).

## Decision outcome

**Option 1.** It follows Flume's 107 as written: the caller that has a pane names it, and an agent whose settings name no place opens in its stream's session, on any host. Flume runs exactly one agent to a stage and refuses a stage that names none, so the stage keeps one agent with no settings of its own; its program is the pool's account's, as it was (Flume's 173). A control plane's launch that names a pane reaches Flume through the runner as given (core.15). Not taken: option 2, which names a program the preview did not name before, a change the brief does not ask. Not taken: option 3, Flume's statement and its terminals' profiles, which would open an agent somewhere its settings did not name without the refusal Flume gives now, and leave a host's pane named in a repository's kind. Not taken: option 4, a step by hand on every host, which the brief's "any host" rules out. Not taken: option 5, since a `beside` the declaration gives comes before any session a stream names, so it changes nothing while the declaration names a pane.

### Consequences

- Good: a content place's preview opens on every host, in a session a person opens by its name with `flume open`.
- Good: a person who launches from a pane keeps the preview beside it, by one setting at the launch.
- Bad: a launch that forgets the setting opens the preview in the stream's session, and the person opens that session to answer it.
- site.5's details name a pane or a session in the kind's declaration as ruled out, and a launch naming the pane as not ruled out.
