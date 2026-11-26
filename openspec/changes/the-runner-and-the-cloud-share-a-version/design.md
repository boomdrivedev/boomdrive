# Design

## Context

The person ruled on 2026-11-26 that the cloud leads the runner's protocol (Boom: `the-cloud-leads-the-runners-protocol`). The cloud speaks a new protocol version before any release of the runner offers it, and keeps each older version until no enrolled runner speaks it. The runner's gate refuses a release that offers a version the cloud's main does not speak; this change is that gate. The decision supersedes record 0056, whose shared version moved one step at a time; core.42 is amended to it. Each side still names the versions it speaks in one file of data, and the runner's gate reads the cloud's file from `main` as the forge holds it, never from a cache. The cloud's file is `workers/control-plane/runner-protocol/pin.json`, each version marked `spoken`, beside a repository, a revision and each version's tree. Rules relied on: core.42 (as amended here), core.7 (one check a task, its inputs declared), core.8 (every tool the tree pins), core.41 (neutral names in tests) and principle 16 (no check reads Markdown).

## Decisions

### Decision 1: `spoken.json` is shaped as the cloud's pin is

`crates/boomdrive-hosted/protocol/spoken.json` is `{"versions": [{"version": N, "spoken": bool}, …]}`, one entry for each `v<N>` directory, lowest first. It reads as the cloud's pin reads, so each gate reads the other's file the same way, and it names the versions the runner no longer speaks, as the cloud's pin does.

### Decision 2: the crate's test holds the file to the code

The runner's code speaks one version, version 5: every message type, check, statement's context and the upgrade's subprotocol are version 5's (`protocol::v5`). The crate's test `spoken` reads the file and holds it to the version directories, to the versions the crate carries, and its one spoken version to the one the code speaks, whose subprotocol the upgrade offers. So the file and the code cannot differ at any commit the gate passes, and the code carries no second reading of the version it speaks.

### Decision 3: the check is a script on git and Node.js

The check passes where each version `spoken.json` marks spoken is one the pin marks spoken, and `spoken.json` marks at least one. A version the cloud speaks and the runner does not is the cloud's lead, and passes.

`check-cloud-protocol.sh` fetches `main` of boomdrive-cloud, depth 1 with no blobs, into a scratch bare repository, by the host's own git credentials, and reads the pin's blob alone. A git fetch spends none of the forge's API allowance. A fetch that moves no byte for 20 seconds fails, inside sluice's own bound on the task. The comparison runs on the Node.js `devenv.nix` pins (core.8), so no tool is added. It takes the repository and the runner's file as arguments, so `test-cloud-protocol.sh` runs it against scratch repositories by `file://` with no network. The task has `cache: false`, as `root:toolchain` and `root:commits` do for what they read outside the tree.

### Decision 4: the gate's executor allows the network

sluice's sandbox holds a run to the network its policy names. The executor's `network` is `true`, which landed on its own first, since sluice reads the gate from the branch's current revision (sluice clause 56).

## Risks

- The check fails whenever the forge cannot be reached, holding every landing until it can. A check that cannot read the cloud's file passes nothing.
- The check fails while the runner offers a version the cloud's main does not speak, whatever the change. A runner's new version lands only once the cloud's main speaks it.
