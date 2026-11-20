# Proposal

## Why

The runner and Boomdrive Cloud have each moved the protocol on their own, and neither gate looked at the other. The runner's releases v0.21.0 to v0.21.3 speak version 4 alone while the cloud's pin named versions 1 and 2, so no released runner could reach the cloud. Record 0056 says the two always share a version, each gate holding it. The runner's half is a file of data naming the versions it speaks, which its own code reads and the cloud's gate reads at the runner's newest release, and a check of the runner's gate that reads the cloud's pin from boomdrive-cloud's `main` as GitHub holds it.

## What Changes

- **The runner names the versions it speaks in one file of data.** `crates/boomdrive-hosted/protocol/spoken.json` names each version directory once, lowest first, and whether the runner speaks it. Today it marks version 4 alone spoken. The crate reads it: a connection's upgrade offers the newest version it marks spoken.
- **The gate refuses a tree that shares no version with the cloud's main.** A new check, `boomdrive-hosted:cloud-protocol`, fetches boomdrive-cloud's `main` with the host's git credentials, reads `workers/control-plane/runner-protocol/pin.json`, and fails where it cannot read it, naming what it could not read, or where the versions the pin marks spoken share none with those `spoken.json` marks. It never replays from a cache. `boomdrive-hosted:cloud-protocol-checks` runs it on scratch repositories.
- **The gate's checks may reach the network.** `.config/sluice.toml`'s executor allows the network, since sluice reads a branch's gate from its current revision (sluice clause 56), so this lands before the check.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `runner-protocol`: the file naming the versions the runner speaks, and the gate's check against the cloud's main.

## Impact

- `.config/sluice.toml`: the executor's network.
- `crates/boomdrive-hosted/protocol/spoken.json`, `src/protocol.rs`, `src/connection.rs`: the file, its reader and the upgrade's offer.
- `crates/boomdrive-hosted/check-cloud-protocol.sh`, `test-cloud-protocol.sh`, `moon.yml`: the check and its cases.
- `crates/boomdrive-hosted/tests/spoken.rs`; `crates/boomdrive-hosted/protocol/README.md`, *Versions*.
- No record or rule changes: record 0056 and core.42 as amended with it already say this.

## Touches

```
.config/sluice.toml
crates/boomdrive-hosted/protocol/spoken.json
crates/boomdrive-hosted/protocol/README.md
crates/boomdrive-hosted/src/protocol.rs
crates/boomdrive-hosted/src/connection.rs
crates/boomdrive-hosted/check-cloud-protocol.sh
crates/boomdrive-hosted/test-cloud-protocol.sh
crates/boomdrive-hosted/moon.yml
crates/boomdrive-hosted/tests/spoken.rs
```
