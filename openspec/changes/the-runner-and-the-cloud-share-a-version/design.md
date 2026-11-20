# Design

## Context

Record 0056 holds the runner and the cloud to a shared protocol version, each side's gate refusing a change that would leave none (point 1), each side naming the versions it speaks in one file of data that its own code and the other side's gate read (point 4), and the runner's gate reading the cloud's file from `main` as GitHub holds it, never from a cache (point 5). The cloud's file is `workers/control-plane/runner-protocol/pin.json`, each version marked `spoken`. Rules relied on: core.42 (as amended with record 0056), core.7 (one check a task, its inputs declared), core.8 (every tool the tree pins) and principle 16 (no check reads Markdown).

## Decisions

### Decision 1: `spoken.json` is shaped as the cloud's pin is

`crates/boomdrive-hosted/protocol/spoken.json` is `{"versions": [{"version": N, "spoken": bool}, …]}`, one entry for each `v<N>` directory, lowest first. It reads as the cloud's pin reads, so each gate reads the other's file the same way, and it names the versions the runner no longer speaks, as the cloud's pin does. The crate's test `spoken` holds it to the directories, and to the versions the crate's code carries.

### Decision 2: the crate reads it

`boomdrive_hosted::protocol` includes the file and gives `spoken()`, the versions marked spoken newest first, and `newest()`, whose subprotocol `connection.rs` offers at each upgrade, one version at an attempt (record 0056, point 4 as amended on 2026-11-19). With version 4 alone spoken, the upgrade asks for `boomdrive.v4` alone, as the hosted-control-plane spec says.

### Decision 3: the check is a script on git and Node.js

`check-cloud-protocol.sh` fetches `refs/heads/main` of boomdrive-cloud, depth 1 with no blobs, into a scratch bare repository, by the host's own git credentials, and reads the pin's blob alone. A git fetch spends none of GitHub's API allowance. The comparison runs on the Node.js `devenv.nix` pins (core.8), so no tool is added. It takes the repository and the file as arguments, so `test-cloud-protocol.sh` runs it against a scratch repository by `file://` with no network. The task has `cache: false`, as `root:toolchain` and `root:commits` do for what they read outside the tree.

### Decision 4: the gate's executor allows the network

sluice's sandbox holds a run to the network its policy names, and `localhost` would refuse the fetch. The executor's `network` becomes `true`. sluice reads the gate from the branch's current revision (sluice clause 56), so the policy lands on its own first and the check after it.

## Risks

- The check fails whenever GitHub cannot be reached, holding every landing until it can. Record 0056 accepts that a check that cannot reach the other side passes nothing.
