# Proposal

## Why

The workflow `cache` builds every package of every system at each push to `main` and again at each tag `v*`, and the workflow `release` cuts a tag from that same push and runs `cache` at it. So every release is built twice, and the tag's build, the one a host takes, queues behind the push's on GitHub's runners: on 2026-11-18 v0.18.1's `aarch64-darwin` build waited 21 minutes behind the duplicate, and the cache was green 31 minutes after the release (signal `2026-11-19-boomdrive-ops-b31633c8/01-a-flake-change-reaches-a-host-in-minutes`; ops' reading of 2026-11-18, cause 4). The user wants a flake change to reach a host as fast as doing it by hand.

## What Changes

- **Built once for each release, from its tag.** `cache` runs on a push of a tag `v*` and by hand, and on no push of a branch; `release` runs it at each tag it cuts, as it does.
- **The boot runs at the tag with it,** and a box pins a tag whose boot passed.
- **Record 0020** is amended on 2026-11-19; the README, `CLAUDE.md` and the workflow's header say it.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `binary-cache`: the workflow runs once for each release, at its tag, and on no push of a branch.

## Impact

- `.github/workflows/cache.yml`: its push names tags `v*` alone.
- `nix/tests/workflow.nix`: "A branch's push" holds the push to tags alone.
- `docs/adr/0020-…`, `README.md`, `CLAUDE.md`.
- A host that pinned a commit of `main` whose `cache` run passed pins a tag instead, as the README says: afterthought/swancloud's boxes, whose pin reads the boot's check, move to tags.

## Touches

```
.github/workflows/cache.yml
nix/tests/workflow.nix
docs/adr/0020-boomdrive-publishes-a-flake-of-the-familys-host-tools-their-binaries-in-the-familys-cache-and-modules-for-the-runners-service.md
README.md
CLAUDE.md
```
