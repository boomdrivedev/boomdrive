# Proposal

## Why

The brief: "the content kind's preview opens beside the pane named operator where the host has one, and in the place's stream's own session where it has none, so a content place passes preview on any host".

The content kind's preview stage, in `.config/flume.toml`, gives its one agent `beside = "operator"`. Flume opens an agent beside the pane its settings name before it reads any session its place's stream names. Under herdr it refuses the start where no pane is labelled `operator`, in the words the ops role read on a host of ours on 2026-11-12: "no herdr pane is named operator". On such a host every content place stops at its preview. Flume's 107 already gives the way that holds on any host: the caller that has a pane names it, and an agent whose settings name no place in the terminal opens in its place's stream's session.

## What Changes

- The preview stage's one agent names no pane and no session: `agents = [{}]`. Flume opens it in the session the place's stream names for the content kind's preview, else for the kind, else the stream's own (Flume's 107 and 199), on every host. Its program stays the pool's account's, as before (Flume's 173).
- A launch made from a pane puts the preview beside that pane by naming it, `--agent preview.beside=<pane>`, a herdr pane by its id or its label, `operator` among them. The kind's comment and `site/docs/content-brief.md`'s section "A task" say so, and that the session the preview opens in is shown by `flume open`.
- `site/check-site.sh`, the site's tests, reads `.config/flume.toml` with the site's bun and holds the kind to the content-kind spec: its stages, the change stage's deliverable, the preview's conversation, wait and send-back, one agent to each stage, and the preview's agent naming neither `beside` nor `session`. It reads the brief's section "A task" for the launch's setting and `flume open`.
- Record 0045 and site.5's details: a pane or a session named in the kind's declaration ruled out, the launch naming the pane not ruled out.

### What stays

- The preview's prompt, `site/docs/prompts/content-preview.md`, and what the preview shows the operator: the address first, then each page, then one question.
- The change stage, the kind's branch, its landing through sluice, and `on_fail = "change"`.

### Not taken

- **Flume opening an agent in its stream's session where the pane its settings name is not there.** It is Flume's statement and its terminals' profiles. It would open an agent somewhere its settings did not name without the refusal Flume gives now, and leave a host's pane named in this repository's kind.
- **`program = "claude"` in place of the pane.** It names a program the preview did not name before, which the brief does not ask.
- **Each host labelling a pane `operator`.** A step by hand on every host, which "any host" rules out.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `content-kind`: the preview's agent names no pane or session, so it opens where its place's stream puts it; a launch names the pane it opens beside; the content brief says how.

## Impact

- `.config/flume.toml`: the preview stage's agent and the kind's comment.
- `site/docs/content-brief.md`: the section "A task".
- `site/check-site.sh`: the kind's declaration and the brief's line, read by `site:test`.
- `openspec/specs/content-kind/spec.md`, by this change's delta once archived.
- `docs/adr/0045-*.md` and `docs/architecture/site.md`'s site.5 details, written with this proposal.
- The change carries out no part of `site/docs/site-design.md` or `site/docs/method-outline.md`: the content kind is the content-kind spec's and site.5's. It changes nothing the site shows (site.5).
