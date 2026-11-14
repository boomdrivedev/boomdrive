# Verify: a-content-preview-opens-where-the-stream-is

The work holds. The gate passes, every task is ticked, and each scenario of the delta has a check that proves it.

The code stage archived the change under task 4.2, which needs the archive before `root:specs` can prove it. This report sits beside the archive, on `change/a-content-preview-opens-where-the-stream-is` at `2129a75`.

## The whole checks

`sluice exec main` ran once on `2129a75`. It ran the checks the commit affects against `main` (`6808fbf`) and exited 0. All 34 passed, and sluice recorded their results under their keys for `2129a75`:

- `boomdrive`, `boomdrive-control`, `boomdrive-crew` and `boomdrive-hosted`: `format`, `lint`, `typecheck` and `test`;
- `nix`: `format`, `lint`, `typecheck` and `test`;
- `root`: `format`, `toolchain`, `adrs`, `changes`, `specs`, `commits`, `versions`, `releases` and `workflow`;
- `site`: `install`, `check`, `build`, `prose`, `vocab`, `test` and `workflow`.

`root:specs` passed 27 items, `content-kind` among them, and 0 failed. `root:adrs` read 45 records, each with a number of its own. `root:changes` found no open change to validate, since this one is archived. moon also warned that it could not write its output archives under `.moon/cache/outputs`. Those warnings come from the sandbox, and no check failed because of them.

`site:test` printed the change's three new lines:

- `check-site: the content kind lands through sluice, change then preview, one agent to each stage, the preview a conversation that waits and sends back to change`
- `check-site: the content kind's preview names no pane or session, opening where its place's stream puts it`
- `check-site: the content brief says a launch names the pane its preview opens beside, and flume open shows the session`

## Tasks

All seven tasks are ticked, 1.1 to 4.2, and none is open.

- 1.1 and 1.2 rely on `root:format`. `.config/flume.toml`'s preview stage gives `agents = [{}]`, and the kind's comment names Flume's 107 and 199, `--agent preview.beside=<pane>` and record 0045. `site/docs/content-brief.md`'s section "A task" gains the paragraph that design Decision 2 gives, word for word, and it names no host of ours (site.7).
- 2.1 and 2.2 rely on `site:test`. `site/check-site.sh` reads the declaration with `Bun.TOML.parse` through `bun -e` and checks each point that design Decision 3 lists. It reads the brief's section from `## A task` to the next heading, joined into one line. `site/moon.yml`'s `test` task takes `docs/content-brief.md` among its inputs.
- 3.1 relies on `root:adrs`. Record 0045 and site.5's details cite each other. They were written in the proposing commit, `d07245a`.
- 4.1 was proven by `root:changes` before the archive, in the commit that ticked it, `8b0fe40`. Since then, the archive's strict merge and `root:specs` hold the delta.
- 4.2 relies on `root:specs`. After the archive, `openspec/specs/content-kind/spec.md` holds the scenario "The preview names no place in the terminal" under "The repository declares one content kind", and the requirement "A launch names the pane its preview opens beside" with its scenario.

The diff stays inside proposal.md's Impact: the kind, the brief, the site's tests and their inputs, record 0045, site.5 and the spec. It adds no crate, gate tool or tracked file type, and it changes nothing the site shows (site.5). No file outside the archive still names `beside = "operator"` or a preview beside the operator. The preview's prompt is unchanged.

## The delta, scenario by scenario

### content-kind

| Scenario | Proven by |
|---|---|
| The declaration reads as the kind | `check-site.sh`'s TOML check: `merge_queue = "sluice"`, `lands = true`, `branch = "content"`, the stages `change` then `preview`, `change` delivering `["commits"]`, `preview` with `conversation = true`, `wait = true` and `on_fail = "change"`, one agent to each stage |
| Each stage's prompt is a file in the repository | `check-site.sh`'s prompt block, unchanged, which resolved both prompts in this run |
| The declaration asks for nothing flume refuses | the same TOML check: one agent to each stage, `change` delivering one deliverable, `preview` none |
| The preview names no place in the terminal | the TOML check of the preview's one agent, which fails on a `beside` or a `session` key and names its value |
| The brief says where the preview opens | `check-site.sh`'s brief block, which fails unless section "A task" holds `--agent preview.beside=` and `flume open` |

The other requirements and scenarios of `content-kind` are unchanged, and `root:specs` passes them.

## Rules

- site.5: content work stays the `content` kind. This change is to how the kind is declared and checked, which an OpenSpec change may make. site.5's details now rule out a pane or a session in the kind's declaration.
- site.7 and core.41: the record, the brief, the change and the check name no host, drive or machine of ours.
- core.15: a launch's `--agent preview.beside=<pane>` reaches Flume through the runner as given. The runner's code is unchanged.
- core.7: `.config/flume.toml` stays taplo's, and the new check is part of the `site` project's `test` task.

Nothing here departs from a rule.
