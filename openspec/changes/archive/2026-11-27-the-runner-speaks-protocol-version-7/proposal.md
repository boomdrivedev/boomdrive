# Proposal

## Why

Under the decision that the cloud leads the runner's protocol (Boom: `the-cloud-leads-the-runners-protocol`), a release of the runner offers a version only once boomdrive-cloud's `main` speaks it. boomdrive-cloud's `main` now speaks versions 2 to 7, and its pin marks version 7 spoken. The runner's code already reads and writes version 7, its hello naming each kept repository's status, behind `spoken.json`'s mark. So the runner now speaks version 7.

## What Changes

- **`spoken.json` marks version 7 spoken.** Version 6 is no longer marked. The runner's upgrade asks for `boomdrive.v7` alone, and every message it sends or reads, on a socket or by a request, is of version 7.
- **The hello names each repository's status.** Each partition's hello names `repository_statuses`, Flume's status of each of its own repositories as its copy, as version 7 requires.
- **The specs name version 7 as the version spoken.** Each requirement of the hosted control plane that named version 6 as the version the runner sends, reads, checks or signs over now names version 7.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `hosted-control-plane`: the version that each message, check, context and upgrade takes is version 7.
- `runner-protocol`: the version `spoken.json` marks spoken, and the subprotocol that the upgrade asks for while version 7 is marked spoken.

## Impact

- `crates/boomdrive-hosted/protocol/spoken.json` and the protocol's README.
- No code changes: `protocol::spoken()` reads the mark, and the crate's tests pass with either mark.
- A runner of an earlier release keeps version 6, which the cloud keeps while an enrolled runner speaks it.

## Touches

```
crates/boomdrive-hosted/protocol/spoken.json
crates/boomdrive-hosted/protocol/README.md
```
