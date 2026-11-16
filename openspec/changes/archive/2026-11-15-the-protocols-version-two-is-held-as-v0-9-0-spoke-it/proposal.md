# Proposal

## Why

The brief: "a check of boomdrive's gate fails where protocol/ version 2's schema files differ from those boomdrive v0.9.0 shipped, so a spoken version never changes (core.42)".

core.42 holds that a version a release of the runner has spoken is never changed or removed, a change to it being a new version. Every release from `v0.8.0` speaks version 2: its subprotocol is `boomdrive.v2` and every message names `protocol_version` 2. Each release from `v0.8.0` to `v0.12.0` holds the same bytes under `protocol/v2/`. The gate holds version 1 to the files `v0.7.3` holds, by `boomdrive-hosted`'s test `files`, but holds version 2 to nothing. So a change could edit version 2 in place, and the gate would pass it. A hosted control plane that pinned version 2's files would then refuse a message the runner sends, or take one the runner refuses.

## What Changes

- **The test.** `boomdrive-hosted`'s `tests/files.rs` gains `version_2_is_as_released`. It holds every file under `protocol/v2/`, schemas and examples, to its SHA-256 as `v0.9.0` holds it, with none missing and none added, as `version_1_is_as_released` holds version 1 to `v0.7.3`. The two tests share one helper, `held_as_released`, and the digests are written in the test, so the gate reads no tag. The test runs in `boomdrive-hosted:test`, in the fast layer.
- **The spec.** The runner-protocol spec gains *Version 2 stays as a release spoke it*, beside *Version 1 stays as a release spoke it*.
- **The words.** The protocol's README says, under *Versions*, that every release from `v0.8.0` speaks version 2, so it never changes, and which test holds each version to which release. The tests' `common::V2` names the releases that speak it.

### Choices of this stage's

- **The whole directory, examples too.** The brief names version 2's schema files. The test also holds its examples, and refuses a file added there, as version 1's does. core.42's detail rules out "a version's file edited … once a release has spoken it", and the README says a spoken version's files and examples stay as its releases hold them. A new example, like a new message, goes to a new version.
- **`v0.9.0`, as the brief names.** `v0.8.0` was the first release to speak version 2. Every release from it to `v0.12.0` holds the same digests, so holding to either tag holds the same bytes. The test names the brief's tag, and its doc says that `v0.8.0` was first.
- **Digests in the test, not `git show` in the gate.** A checkout with no tags, such as a shallow clone, still runs the check, as version 1's already does.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `runner-protocol`: version 2 stays as the releases that speak it hold it.

## Impact

- `crates/boomdrive-hosted/tests/files.rs` and `crates/boomdrive-hosted/tests/common/mod.rs`.
- `crates/boomdrive-hosted/protocol/README.md`.
- `openspec/specs/runner-protocol/spec.md`, by this change's delta once archived.
- No file under `protocol/` changes. No record or rule changes: core.42 already rules out changing a version a release has spoken, and record 0037 already says a release fixes version 2. No crate, tool of the gate or tracked file type is added (core.7, core.8).
- **Outside this repository.** A change that would alter version 2, a hold among them, goes to version 3, in `protocol/v3/` beside it.

## Touches

```
crates/boomdrive-hosted/tests/**
crates/boomdrive-hosted/protocol/README.md
openspec/changes/the-protocols-version-two-is-held-as-v0-9-0-spoke-it/**
openspec/specs/runner-protocol/spec.md
```
