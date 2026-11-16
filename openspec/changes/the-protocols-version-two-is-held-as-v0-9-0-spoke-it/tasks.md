# Tasks

A task is true once the check it names passes, run as it names it.

## 1. The check

- [x] 1.1 `crates/boomdrive-hosted/tests/files.rs` holds version 2's files, schemas and examples, to their SHA-256 as `v0.9.0` holds them, read by `git show v0.9.0:<path> | shasum -a 256`, in `RELEASED_V2`. `version_2_is_as_released` and `version_1_is_as_released` both call one helper, `held_as_released`, which fails on a file missing, differing or added. True when `sluice exec main -- moon run boomdrive-hosted:test` passes, and fails where a file under `protocol/v2/` is changed or one is added.

## 2. The words

- [x] 2.1 The protocol's README, under *Versions*, says that every release from `v0.8.0` speaks version 2, so it never changes, and that the test `files` holds version 2 to `v0.9.0`'s files and version 1 to `v0.7.3`'s. `common::V2`'s doc names the releases that speak it. True when `sluice exec main -- moon run root:format` passes.

## 3. The spec

- [x] 3.1 The change's delta passes openspec's strict check. True when `sluice exec main -- moon run root:changes` passes.
- [x] 3.2 The change's delta archives into specs that pass openspec's strict check, with the requirement *Version 2 stays as a release spoke it*. True when, after `devenv shell -- openspec archive the-protocols-version-two-is-held-as-v0-9-0-spoke-it --yes`, `sluice exec main -- moon run root:specs` passes.
