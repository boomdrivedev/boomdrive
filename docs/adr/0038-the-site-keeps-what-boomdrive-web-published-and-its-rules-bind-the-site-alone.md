---
number: 38
title: The site keeps what boomdrive-web published, and its rules bind the site alone
status: proposed
date: 2026-11-09
decision-makers:
- Chuck Swanberg
---

# The site keeps what boomdrive-web published, and its rules bind the site alone

Status: proposed on 2026-11-09 by the construct stage of the change `boomdrive-keeps-its-site`, which carries out record 0025. What moves, and that the site's history and rules come with it, is record 0025's, which the user accepted on 2026-11-07. How three of the site's things meet this repository's rules is the stage's choice, made on those rules and on the family's other sites. The line our own fleet's names keep is the planner's, in its answer to the stage of 2026-11-09, on the user's ruling of 2026-10-26. The record is accepted when the user says yes to it.

Deciders: the user, 2026-11-07, for the move (record 0025), and 2026-10-26, that the site never presents a name of ours as the reader's own; the planner, 2026-11-09, for where our own names may stand; the construct stage, for how the site meets core.6, core.41 and core.48.
Sources: record 0025, and its order as agentplot-design amends it on 2026-11-09, which moves the site into this repository while it is private, before the opening; the planner's answer to the construct stage of `boomdrive-keeps-its-site`, 2026-11-09; agentplot/blueprints `design/principles.md` 12; agentplot/boomdrive-web at `4ed5fbe`, its `CLAUDE.md`, `openspec/config.yaml`, `src/content/contexts/contexts.yaml`, `src/content/measurements/`, `src/content/rates/rates.yaml`, `docs/site-design.md` and `docs/measurement-roadmap.md`, and its history, each public; rules core.6, core.7, core.41 and core.48; git-cliff 2.14.2's `exclude_paths`, run on a scratch repository on 2026-11-09; convco 0.7.2 under `.versionrc`, run on boomdrive-web's 166 commits that are not merges, which all pass.

## Context and problem statement

Record 0025 moves boomdrive-web into this repository's `site/`, while the repository is still private, before it opens. Its files come with their history, which is already public. Its working rules come with it, scoped to `site/`, and its commits stay out of the runner's release notes and move no version. Three things of the site meet this repository's rules.

- **Our own fleet's names.** The site's measurements count the fleet by drive. `contexts.yaml` names our two drives and the machine they run on, and the measurements' series name the drives too. So do examples in the site's design, `docs/measurement-roadmap.md` and `docs/site-design.md`, and one archived change cites a signal by an id holding a drive's name. Many of the history's messages name the drives, the machine and the agents named after a drive. core.41 rules out a real host's or drive's name in the tree's code, tests, fixtures or docs. The site's own rule names our own drives, machines and repositories, and only those: "drives, machines and repositories that carry client work are neither named nor counted."
- **The site's rules.** boomdrive-web's working rules bind the site's files: nothing commercial, no client named, a field note never rewritten, the glossary as the one vocabulary, content work run as its kind, and every link under the site's address. `core.md` binds wherever a change is made, and has no area page yet.
- **The release notes.** git-cliff makes the next version and its notes from every commit since the newest tag. The site's history and every later change to the site would land in the runner's notes and bump its version.

core.6 also meets the site: its `rates.yaml` holds a model vendor's list prices, which its measurements are costed with, and its products appear as a waitlist.

## Considered options

For our own fleet's names:

1. **Our own names stand where the site shows our fleet as a case study.** The site's measurement and field-note pages, and the content they are built from, may name our own drives and machines as ours, and its history comes as boomdrive-web published it. Everything else of the site, its guides, reference, glossary and quick start, its design and its archived changes among them, names none of ours, and no page presents a name of ours as the reader's own.
2. The site's pages and design may name our own fleet anywhere, as the site's own rule alone would have it.
3. The site's content takes neutral names, and the history's messages are rewritten.
4. Our own names are left out of the site's measurements and stay in boomdrive-web.

For the site's rules:

1. **An area page, `docs/architecture/site.md`, binding `site/` and its workflow.**
2. The rules in `core.md`, binding the runner's tree too.
3. The rules in `site/CLAUDE.md` alone, with no id a design or a commit can cite.

For the release notes:

1. **By path.** `cliff.toml` leaves out a commit that changes only the site's paths, `site/` and `.github/workflows/pages.yml`.
2. By scope. A commit parser skips a commit scoped `site`.

## Decision outcome

**Option 1 for the names.** core.41's reason is that nothing private enters the repository (record 0014), and that no deployment's details sit in its code and tests. The site's measurements and field notes are a case study of our own fleet, which boomdrive-web already publishes, and they name it as ours. Elsewhere a name of ours would read as an example the reader takes for their own, which the user ruled out on 2026-10-26, so the rest of the site keeps core.41 whole. Option 2 loosens core.41 past that case study. Neutral names everywhere (option 3) would change what a public measurement page shows, which is content work of its own, and rewriting the history's messages would break record 0025's promise that the history comes as published. Option 4 splits the site's content across two repositories, which record 0025 ends.

**Option 1 for the rules.** The site's rules are the area page's, ids `site.1` to `site.7`, beside `core.md`. A rule in `core.md` (option 2) would bind a crate's change to the site's glossary. A rule only in `CLAUDE.md` (option 3) has no id, so a review cannot point at it.

**Option 1 for the notes.** git-cliff's `exclude_paths` leaves out a commit only when every path it changes matches. A commit that changes the site and the runner together is still in the notes. A commit's scope (option 2) is the writer's choice, and the site's 166 commits carry many scopes. The paths are what a commit changed.

### Consequences

- `site.7` holds the line: our own drives and machines are named only on the site's measurement and field-note pages and the content they are built from, as ours, and nowhere else of the site. core.41 lets those pages name them, and lets the site's history stand as boomdrive-web published it.
- The site's design takes neutral names in its examples, and the archived change cites its signal without the drive in its id.
- core.6's details let the site cost its measurements with a vendor's published list prices, and show a product as a waitlist, with no price or tier of ours.
- core.7 holds the site as a moon project of its own, beside the crates and the flake.
- core.48 leaves out of every release's notes and version a commit that changes only `site/` and its workflow. The gate still holds every commit to the convention, the site's among them.
- `core.md` names its one area page.
