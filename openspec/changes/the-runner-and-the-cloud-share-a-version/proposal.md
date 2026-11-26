# Proposal

## Why

The runner and boomdrive-cloud each move the protocol on their own, and neither gate looks at the other. The runner's release v0.29.0 speaks version 5 alone, while the cloud's main marks versions 2 to 4 spoken, so no released runner can reach the cloud, and nothing refused the release. The person ruled on 2026-11-26 that the cloud leads the runner's protocol (Boom: `the-cloud-leads-the-runners-protocol`): the cloud speaks a new version before any release of the runner offers it, and the runner's gate refuses a release that offers a version the cloud's main does not speak. The runner's half is a file of data naming the versions it speaks, and a check of the runner's gate that reads the cloud's pin from boomdrive-cloud's `main` as the forge holds it.

## What Changes

- **The runner names the versions it speaks in one file of data.** `crates/boomdrive-hosted/protocol/spoken.json` names each version directory once, lowest first, and whether the runner speaks it: version 5 alone. The crate's test holds it to the directories and to the version the crate's code checks, sends and offers.
- **The gate refuses a runner that offers a version the cloud's main does not speak.** A new check, `boomdrive-hosted:cloud-protocol`, fetches boomdrive-cloud's `main` with the host's git credentials, reads `workers/control-plane/runner-protocol/pin.json`, and fails where it cannot read it, naming what it could not read, or where `spoken.json` marks spoken a version the pin does not, or marks none. It never replays from a cache. `boomdrive-hosted:cloud-protocol-checks` runs it on scratch repositories.
- **The gate's checks reach the network.** `.config/sluice.toml`'s executor allows the network, which landed on its own first, since sluice reads a branch's gate from its current revision (sluice clause 56).

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `runner-protocol`: the file naming the versions the runner speaks, and the gate's check against the cloud's main.

## Impact

- `crates/boomdrive-hosted/protocol/spoken.json`, and its test `crates/boomdrive-hosted/tests/spoken.rs`.
- `crates/boomdrive-hosted/check-cloud-protocol.sh`, `test-cloud-protocol.sh`, `moon.yml`: the check and its cases.
- `crates/boomdrive-hosted/protocol/README.md`, *Versions*.
- `docs/architecture/core.md`, core.42: the cloud leads the protocol, in place of record 0056's shared version moved one step at a time.
- `docs/adr/0056-…`: superseded by the Boom decision `the-cloud-leads-the-runners-protocol`. No new record: the decision is Boom's.

## Touches

```
docs/architecture/core.md
docs/adr/0056-the-runner-and-the-cloud-always-share-a-protocol-version-and-move-one-version-at-a-time.md
crates/boomdrive-hosted/protocol/spoken.json
crates/boomdrive-hosted/protocol/README.md
crates/boomdrive-hosted/check-cloud-protocol.sh
crates/boomdrive-hosted/test-cloud-protocol.sh
crates/boomdrive-hosted/moon.yml
crates/boomdrive-hosted/tests/spoken.rs
```
