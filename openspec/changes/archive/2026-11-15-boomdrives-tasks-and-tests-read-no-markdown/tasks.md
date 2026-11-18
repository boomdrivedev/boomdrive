# Tasks

A task is true once the check it names passes, run as it names it.

## 1. The root

- [x] 1.1 `root:adrs` and `check-adrs.sh` are gone, and `root:toolchain` reads no skill. True when `sluice exec main -- moon run root:toolchain` passes.

## 2. The site

- [x] 2.1 `site:prose` is gone, and the shell's `prose` writes the vocabulary and runs Vale, which `site-check` runs after the site's tasks. True when `sluice exec main -- moon run site:check site:build` passes.
- [x] 2.2 `site:test` reads the built quick start, the built stylesheet and `site/src/start/`, and no step's record or page's words. True when `sluice exec main -- moon run site:test` passes.

## 3. The flake

- [x] 3.1 No case of `nix/tests` reads README; the examples a case checks are `nix/tests/examples/`. True when `sluice exec main -- moon run nix:test nix:lint nix:format` passes.

## 4. The crates

- [x] 4.1 `tests/examples.rs` reads README's TOML examples from `tests/examples/`, and `facts.rs` and `sluice.rs` read no fixtures' README. True when `devenv shell -- cargo test -p boomdrive --features stand-in --test examples --test facts --test sluice --test rules` passes.
- [x] 4.2 `signatures.rs` reads the worked bytes from `tests/worked-bytes.json`. True when `devenv shell -- cargo test -p boomdrive-hosted --test signatures` passes.

## 5. The spec

- [x] 5.1 The change's deltas pass openspec's strict check. True when `sluice exec main -- moon run root:changes` passes.
- [x] 5.2 The deltas archive into specs that pass openspec's strict check. True when, after `devenv shell -- openspec archive boomdrives-tasks-and-tests-read-no-markdown --yes`, `sluice exec main -- moon run root:specs` passes.
