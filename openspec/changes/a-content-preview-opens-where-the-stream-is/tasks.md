# Tasks

A task is true once the check it names passes, run as it names it.

## 1. The kind and the brief

- [x] 1.1 `.config/flume.toml`'s preview stage gives its one agent no settings, `agents = [{}]`, and the kind's comment says the preview opens where the place's stream puts it, and beside a pane only where its launch names one, `--agent preview.beside=<pane>` (design Decision 1). True when `sluice exec main -- moon run root:format` passes, taplo reading the file.
- [x] 1.2 `site/docs/content-brief.md`'s section "A task" says the preview opens in the session the place's stream gives it, which `flume open <session>` shows, and that a launch made from a pane puts the preview beside that pane by naming it, `--agent preview.beside=<pane>`, a herdr pane by its id or its label, naming no host of ours (design Decision 2; site.7). True when `sluice exec main -- moon run root:format` passes.

## 2. The site's tests

- [ ] 2.1 `site/check-site.sh` reads `../.config/flume.toml` with `Bun.TOML.parse` through `bun -e` and fails, naming what differs, unless the kind lands through sluice on the branch `content` with the stages `change` then `preview`, one agent to each, `change` delivering `["commits"]`, and `preview` a conversation with `wait = true`, `on_fail = "change"`, no deliverables and one agent giving neither `beside` nor `session`; each pass says its line as design Decision 3 gives it. True when `sluice exec main -- moon run site:test` passes, proving the content-kind delta's "The declaration reads as the kind", "The declaration asks for nothing flume refuses" and "The preview names no place in the terminal".
- [ ] 2.2 `site/check-site.sh` reads `docs/content-brief.md`'s section from `## A task` to the next heading and fails unless it holds `--agent preview.beside=` and `flume open`, and `site/moon.yml`'s `test` task takes `docs/content-brief.md` among its inputs (design Decision 3). True when `sluice exec main -- moon run site:test` passes, proving the content-kind delta's "The brief says where the preview opens".

## 3. The record and the rule

- [ ] 3.1 Record 0045 and site.5's details, written with the proposal, rule out a pane or a session in the kind's declaration and leave a launch naming the pane, each citing the other (design Decision 4). True when `sluice exec main -- moon run root:adrs` passes.

## 4. The spec

- [ ] 4.1 The change's delta passes openspec's strict check. True when `sluice exec main -- moon run root:changes` passes.
- [ ] 4.2 The change's delta archives into specs that pass openspec's strict check, the requirement "The repository declares one content kind" holding the scenario "The preview names no place in the terminal" and the requirement "A launch names the pane its preview opens beside" its scenario. True when, after `devenv shell -- openspec archive a-content-preview-opens-where-the-stream-is --yes`, `sluice exec main -- moon run root:specs` passes.
