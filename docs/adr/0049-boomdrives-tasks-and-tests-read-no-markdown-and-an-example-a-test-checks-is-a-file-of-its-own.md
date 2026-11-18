---
number: 49
title: Boomdrive's tasks and tests read no Markdown, and an example a test checks is a file of its own
status: accepted
date: 2026-11-15
decision-makers:
- the user, by boomdrivedev/blueprints principle 16
- the code stage of chore/boomdrives-tasks-and-tests-read-no-markdown
---

# Boomdrive's tasks and tests read no Markdown, and an example a test checks is a file of its own

Status: accepted. Point 1 is the user's rule, blueprints principle 16, and the chore's brief carries it to this repository. Points 2 to 7 are the code stage's own choices in the chore `boomdrives-tasks-and-tests-read-no-markdown`, within the rules, each cheap to change.

Deciders: the user, whose words of 2026-11-11 principle 16 quotes: "I don't want fucking checks on instructions. That has got to fucking stop. That is my biggest pet peeve. We do not fucking write code tests against Markdown files." The principle as amended on 2026-11-15, on their answer on their card: "OpenSpec's own validation stays; the rule is about checks of what pages say". The code stage of `chore/boomdrives-tasks-and-tests-read-no-markdown`, 2026-11-15, for points 2 to 7, from the chore's brief: "boomdrive's tasks and tests read no Markdown file of its own [...] Where a test checks an example a page shows, the example lives in a non-Markdown file the test reads, and the page keeps its own copy. OpenSpec's validation, the site's build and check, and tests that write a README.md into a scratch repository as data stay."

Sources: boomdrivedev/blueprints `design/principles.md` 16, at `3a5c19d`; boomdrivedev/sluice's core.21 as amended for principle 16 (`4295ca8`); boomdrivedev/crew's test change `26f1c4d`, which dropped its tests' pins on README, the skill and the briefs; records 0022, 0025, 0038 and 0043; rules core.7, core.8, site.4 and site.8.

## Context and problem statement

Principle 16 says no check, test or gate task in any kit reads a Markdown file: a doc, a README, a skill, a site page, an instruction, a spec or a record. What a page says is kept by the people and agents who write and review it. A rule about what a page must say stays a rule its writers keep, and no task enforces it. A tool's own validation of the files it works on is no check of what a page says, and stays.

On 2026-11-15 boomdrive's gate read Markdown in these places:

- `root:adrs`, `check-adrs.sh`, read each record's name and frontmatter for its number.
- `root:toolchain` read each of OpenSpec's Claude Code skills for the release that wrote it (record 0022).
- `site:prose` ran Vale over the site's content (site.4).
- `site:test` read the quick start's step records, `site/src/content/start/*.md`, and its page, `site/src/content/pages/start.md`, for each step's place, files and test, and for our names (site.7, site.8).
- `nix:test` read README's **Installing with nix** for its words and its nix blocks.
- The runner's tests read README for its TOML blocks and its words (`readme.rs`), and the fixtures' READMEs for each fixture's name (`facts.rs`, `sluice.rs`); the hosted implementation's read the protocol's README for its worked bytes and its words (`signatures.rs`).

## Considered options

For a test that checks an example a page shows:

1. **The example is a file of its own, in its own language, which the test reads; the page shows its own copy, which its writers keep to the file** (chosen).
2. The test checks that the page's copy equals the file.
3. The page is built from the file.

For the site's words and Vale:

1. **Vale stays in the shell as its writers' check, `prose`, which `site-check` runs, and no task of the gate runs it** (chosen).
2. Vale leaves the shell.

For the quick start's steps:

1. **`site:test` reads the built page, the built stylesheet and the files under `site/src/start/`, and no step's record** (chosen).
2. Each step's place, files and test move from its record's frontmatter into a data file `site:test` reads.
3. `site:test` stops checking the quick start.

## Decision outcome

1. **No task, test or formatter the gate runs reads a Markdown file of boomdrive's own (the user's).** What stays: OpenSpec's `openspec validate` of the specs and the open changes, the site's build and type check, which are Astro's own reading of its content, and a test that writes a `README.md` into a scratch repository as data. A task may list a folder that holds a README and leave it unread, as `facts.rs` and `sluice.rs` list their fixtures. core.7 says so.
2. **`root:adrs` goes.** A record's number is its writer's to keep: the next free number, read from `docs/adr/` and from the open branches that add records. `check-adrs.sh` goes with it.
3. **`root:toolchain` reads no skill.** It still checks that `openspec` is the release `devenv.nix` pins. OpenSpec's Claude Code skills are still the ones the pinned `openspec` writes, written again when the pin moves (record 0022); their writers keep that.
4. **`site:prose` goes; Vale stays the writers' (option 1).** The shell's `prose` writes the vocabulary from the glossary and runs Vale over `site/src/content`, and `site-check` runs the site's tasks and then `prose`. The content kind's brief already names Vale for the words a change writes, and the house style still has a warning fixed. No task of the gate runs Vale. site.4 says so.
   - Not taken: Vale out of the shell. The writers lose the house style's check of their own words, which principle 16 leaves to them, not away from them.
5. **`site:test` reads the built quick start, the built stylesheet and `site/src/start/` (option 1).** Each block of a step's on the built page is a file of `site/src/start/` as it stands, and every file there but `.shellcheckrc` is on the page. The page is its lede, then `On a Mac`, then `On NixOS`, each one numbered list, with no script. Our names are read from the built page and the files. A step's place on each platform and the test it names are its writers' to keep (site.8): the build still refuses a step with no place, no file or no test, by the collection's schema.
   - Not taken: option 2, a step's frontmatter moved into a data file. It changes how the site holds a step for the sake of a check, and a step's place and test are what its writers keep under the principle.
   - Not taken: option 3. The built page is HTML, which the site's build writes from the files and the records. The files shown as they stand are what the gate runs (record 0043).
6. **An example a test checks is a file of its own (option 1).** README's TOML examples are `crates/boomdrive/tests/examples/`, `configuration.toml`, `local-layer.toml`, `crew.toml` and `hosted.toml`, which `tests/examples.rs` reads as `readme.rs` read README's blocks. README's nix examples that a test checks are `nix/tests/examples/`: `trust.nix`, held to the flake's `nixConfig`; `host-flake.nix`, the Mac's flake, whose outputs `tools.nix` calls to read crew's table and settings; and `telemetry.nix` and `transcripts.nix`, which `claude-code.nix` evaluates on each platform. The protocol's worked bytes are `crates/boomdrive-hosted/tests/worked-bytes.json`, which `signatures.rs` reads. Each page keeps its own copy. A nix example is in nixfmt's layout, and README's copy may keep a shorter one. A test that checked only a page's words goes, with no file in its place.
   - Not taken: option 2. It reads the page.
   - Not taken: option 3. A step that writes a page from a file is a tool of its own, and a check that the page was written again would read it.
7. **The flake's address the quick start's install adds is named in its test.** `nix/tests/quick-start.nix` names `github:boomdrivedev/boomdrive` in place of reading it from README.

### Consequences

- Good: no page's wording fails the gate, and no check's words need changing for a page's.
- Bad: a page's copy of an example can drift from the file the test reads, a record can take a number another holds, and a skill can lag its pin, until a writer or a reviewer sees it.
- Bad: a step's place and test go unchecked past the schema, and a step that names a test its project lacks passes the gate.
- core.7's rule and details, site.4 and site.8 say so, in the commit that writes this record. Records 0022 and 0043 are amended by it.
