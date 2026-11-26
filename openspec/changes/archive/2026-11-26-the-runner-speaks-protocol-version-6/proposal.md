# Proposal

## Why

Under the decision that the cloud leads the runner's protocol (Boom: `the-cloud-leads-the-runners-protocol`), a release of the runner offers a version only once boomdrive-cloud's `main` speaks it. boomdrive-cloud's `main` now speaks versions 2 to 6, and its pin marks version 6 spoken. The runner's code already reads and writes version 6, behind `spoken.json`'s mark. So the runner now speaks version 6.

## What Changes

- **`spoken.json` marks version 6 spoken.** Version 5 is no longer marked. The runner's upgrade asks for `boomdrive.v6` alone, and every message it sends or reads, on a socket or by a request, is of version 6.
- **The runner says the newest release.** On its connection of version 6, the runner says one line for each release that the control plane names: `<label>: this runner is <its runner_version>, and the newest release is <version>`. It judges nothing of it.
- **The specs name version 6 as the version spoken.** Each requirement of the hosted control plane that named version 5 as the version the runner sends, reads, checks or signs over now names version 6.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `hosted-control-plane`: the version that each message, check, context and upgrade takes is version 6.
- `runner-protocol`: the subprotocol that the upgrade asks for while version 6 is marked spoken.

## Impact

- `crates/boomdrive-hosted/protocol/spoken.json` and the protocol's README.
- No code changes: `protocol::spoken()` reads the mark, and the crate's tests pass with either mark.
- `docs/architecture/core.md`, core.42's details: version 6 was built while version 5 was spoken.
- A runner of an earlier release keeps version 5, which the cloud keeps while an enrolled runner speaks it.

## Touches

```
docs/architecture/core.md
crates/boomdrive-hosted/protocol/spoken.json
crates/boomdrive-hosted/protocol/README.md
```
