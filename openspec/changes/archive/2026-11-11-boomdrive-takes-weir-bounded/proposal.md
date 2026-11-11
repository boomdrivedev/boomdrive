# Proposal

## Why

The brief: "Boomdrive pins weir at 9cba691 or later and its runner runs its calls to a remote through weir-bounded in place of its own boomdrive-bounded crate, so its fetches and pushes of a state repository through weir-branch end at the limit too."

Record 0042 bounded the runner's fetch and crew's listing at thirty seconds a try, through `boomdrive-bounded`, a copy of sluice's `bounded.rs` kept because the family had no shared owner. It left weir-branch's fetch and push of crew's state repository with no limit, weir's to bound. agentplot-design ruled on 2026-11-10 that weir owns the code of a call's limit. weir record 0008 made `weir-bounded` that owner, and at weir's `9cba691` weir-branch runs its fetch, its listing and its push each as one try to it. A silent state repository still holds its partition's part at the read of `<label>/main`, and its report at the write, with no limit, until the runner takes that revision.

## What Changes

- **weir at `9cba691`.** weir-branch, weir-capture and weir-signals move to `9cba691` with weir-bounded beside them, in `Cargo.toml`, `Cargo.lock` and the flake's `nix/packages/git-crates.nix`.
- **The runner's calls run through weir-bounded.** `git::bounded` and `Store::hosts` call `weir_bounded::output` and `weir_bounded::stopped`, and the limit is `weir_bounded::TRY_FOR`. The words, the limit and the grace are as before.
- **`crates/boomdrive-bounded` goes**, with its moon project, its inputs in the other projects' tasks and its name in README's and CLAUDE.md's lists.
- **crew's read and write of a state repository end at the limit.** crew's implementation reads `<label>/main` and each runner's branch, and writes its own, by weir-branch at `9cba691`, so each try of its fetch, listing and push is stopped at thirty seconds with everything it started. A stopped read leaves its partition unread, and a stopped write its report unsent, each with weir-branch's words.
- **Record 0042 is amended and core.58 with it**: weir's crate holds the limit, and weir-branch's calls are bounded too.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `crew-control-plane`: no try of weir-branch's fetch, listing or push of a state repository outlasts its limit.

## Impact

- `Cargo.toml`, `Cargo.lock`, `nix/packages/git-crates.nix`: weir at `9cba691`, and weir-bounded.
- `crates/boomdrive`: `Cargo.toml` and `moon.yml` take weir-bounded in place of boomdrive-bounded; `src/git.rs` calls it; `tests/rules.rs` has the crates depend as the amendment has them.
- `crates/boomdrive-crew`: `Cargo.toml` and `moon.yml` likewise; `src/read.rs` and `src/lib.rs` call it and say so; `tests/read.rs` gains a state repository that never answers the read, and one that never answers a write.
- `crates/boomdrive-bounded`: removed. `.moon/workspace.yml` lists it no more.
- `README.md`, `CLAUDE.md`: the projects, and crew's read and write stopped at the limit.
- `docs/adr/0042-…` and `docs/architecture/core.md`: record 0042's amendment of 2026-11-11 and core.58.
- No tool, tracked file type, configuration key, field of `report.json`, file of crew's or protocol message changes. The clones spec's words stand.
