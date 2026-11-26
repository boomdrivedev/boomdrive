# Proposal

## Why

The person ruled on 2026-11-26 that the cloud leads the runner's protocol (Boom: `the-cloud-leads-the-runners-protocol`). The ruling says: "The cloud tells that runner that a newer release exists, and the host page shows the runner as out of date." No message of versions 2 to 5 can carry a newer release, and a version that a release spoke never changes (core.42). So version 6 carries it. The cloud leads, so the runner builds version 6 now and offers it only when the cloud's `main` speaks it.

## What Changes

- **Version 6 names the newest release.** `protocol/v6/` is version 5 whole, but for one new kind that the control plane sends: `newest_release`. It holds `version`, the newest release of the runner that the control plane knows.
- **The runner says the newest release.** On a connection of version 6, the runner says one line for each release that the control plane names: `<label>: this runner is <its runner_version>, and the newest release is <version>`. It says each release one time on each connection. It does nothing else with it.
- **The mark in `spoken.json` alone chooses the version.** The runner's code reads and writes version 5 and version 6. It speaks the version that `spoken.json` marks spoken: version 5 now. `spoken.json` names version 6, not spoken, so the runner's gate stays green. When the cloud's `main` speaks version 6, a change that only marks version 6 spoken moves the runner to it.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `runner-protocol`: version 6, its `newest_release`, and the version that `spoken.json` marks spoken.
- `hosted-control-plane`: the line that says the newest release.

## Impact

- `crates/boomdrive-hosted/protocol/v6/`, `spoken.json` and the README.
- `crates/boomdrive-hosted/src/`: `protocol::Version` and `protocol::spoken`. The types move to `protocol::v6`. Each check, context and message takes its version. The connection reads `newest_release`.
- `crates/boomdrive-hosted/tests/`: version 6's files, examples and connection, and each test that named version 5 now names the version spoken.
- `docs/architecture/core.md`, core.42's details: version 6, and the code that reads a version it does not speak yet.
- No record: the decision is Boom's.
- The runner has no `boomdrive status` command, so the runner says the newest release in its log alone.

## Touches

```
docs/architecture/core.md
crates/boomdrive-hosted/protocol/spoken.json
crates/boomdrive-hosted/protocol/README.md
crates/boomdrive-hosted/protocol/v6/
crates/boomdrive-hosted/src/
crates/boomdrive-hosted/tests/
```
