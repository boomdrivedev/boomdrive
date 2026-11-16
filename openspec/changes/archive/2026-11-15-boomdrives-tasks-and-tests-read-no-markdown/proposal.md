# Proposal

## Why

The brief: "boomdrive's tasks and tests read no Markdown file of its own: root:adrs goes, root:toolchain no longer greps the OpenSpec skills, site:prose goes, site:test no longer reads the start pages, nix:test no longer reads README.md's install section, and the cargo tests readme.rs, facts.rs, sluice.rs and boomdrive-hosted's signatures.rs no longer read a README. Where a test checks an example a page shows, the example lives in a non-Markdown file the test reads, and the page keeps its own copy. OpenSpec's validation, the site's build and check, and tests that write a README.md into a scratch repository as data stay."

It carries boomdrivedev/blueprints principle 16, the user's rule of 2026-11-11 as amended on 2026-11-15: no check, test or gate task in any kit reads a Markdown file, and a rule about what a page must say stays a rule its writers keep. Record 0049 decides how, and core.7, site.4 and site.8 say so.

## What Changes

- `root:adrs` and `check-adrs.sh` go; `root:toolchain` reads no skill of OpenSpec's.
- `site:prose` goes. Vale stays the writers' own check, the shell's `prose`, which `site-check` runs after the site's tasks, and no task of the gate runs it.
- `site:test` reads the built quick start, the built stylesheet and the files under `site/src/start/`, and no step's record or page's words: each block of a step's is a file there as it stands, every file there is on the page, and none names ours. A step's places and the test it names are its writers' to keep.
- README's examples that a test checks are files of their own: `crates/boomdrive/tests/examples/*.toml`, read by `tests/examples.rs` in place of `readme.rs`, and `nix/tests/examples/*.nix`, read by `cache.nix`, `tools.nix` and `claude-code.nix`. The protocol's worked bytes are `crates/boomdrive-hosted/tests/worked-bytes.json`, read by `signatures.rs`. Each page keeps its own copy.
- The tests that checked only a page's words go: README's in `readme.rs` and in `nix/tests`, the protocol README's in `signatures.rs`, and the fixtures' READMEs' in `facts.rs` and `sluice.rs`. `nix/tests/quick-start.nix` names the flake's address.

### What stays

- `openspec validate` of the specs and the open changes; the site's build and type check, and the schema of a step's record; `site:test`'s reading of the built pages; every test that writes a `README.md` into a scratch repository; README's, the protocol README's and the specs' words, which their writers keep.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `site`: no check of the site's words; the requirement "The site's words are held to its glossary" leaves the spec.
- `quick-start`: `site:test` reads the built page and the files, no step's record; a step's places and its test are its writers'.
- `runner-protocol`: the worked bytes are a file of the tests, which the README shows a copy of.

## Impact

- `moon.yml`, `check-adrs.sh`, `check-toolchain.sh`, `.moon/workspace.yml`, `CLAUDE.md`.
- `site/moon.yml`, `site/check-site.sh`, `devenv.nix`'s `prose` and `site-check`, `site/CLAUDE.md`, `site/README.md`, `site/docs/site-design.md`, `site/src/content.config.ts`'s comment, `openspec/config.yaml`.
- `nix/moon.yml`, `nix/tests/lib.nix`, `cache.nix`, `tools.nix`, `claude-code.nix`, `quick-start.nix`, `sources.nix`, `clone-dirs.nix`, `data-dir.nix`, `kept.nix`, and `nix/tests/examples/`.
- `crates/boomdrive/moon.yml`, `tests/examples.rs` and `tests/examples/`, `facts.rs`, `sluice.rs`, `rules.rs`; `crates/boomdrive-hosted/tests/signatures.rs` and `tests/worked-bytes.json`.
- Record 0049, and records 0022 and 0043 amended by it; core.7, site.4 and site.8.
