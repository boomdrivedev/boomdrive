# Proposal

## Why

The brief: "No try of the runner's git call to a remote, a clone's fetch or a remote's ls-remote, runs past thirty seconds: it ends with everything it started and fails naming the remote and the limit, so one silent remote can't stall the runner's pass over every other repository."

On a host on 2026-11-10, sluice's reads of one repository hung for 10 to 17 minutes, again and again (signal `2026-11-10-agentplot-ops-e19d134d/01`). The cause was a git call to a remote with no limit. Its helper, `git-remote-http`, waited in `select` on a socket the remote had taken and never answered. sluice's record 0109 fixed sluice's calls. It notes that the Boomdrive runner runs its own fetch with no limit. It names `crates/boomdrive/src/clone.rs`.

The runner has two git calls that reach a remote, and both wait with no limit:

- **A clone's fetch.** `clone::fetch` runs `git fetch --prune --quiet origin` by `Command::output`. Every keep of a repository runs it: a pass's keep, a landing's keep, and a new clone's first fetch. The keeping pass keeps one repository after another, so one silent origin holds back every repository after it. It also holds the settle that ends the pass (core.13). The keep holds the clone's lock, so every launch into that repository waits too (record 0026). A SIGTERM waits for the pass to stop, so the runner cannot be stopped while the fetch hangs.
- **A state repository's listing.** crew's implementation lists `<label>/runner/*` by `git ls-remote` in `Store::hosts`, also by `Command::output`. One silent state repository holds its partition's part at every pull.

sluice's record 0109 is the family's one design for a git call's limit. A try runs in a process group of its own and ends at thirty seconds, `retry::TRY_FOR`. Its group is asked to end, given two seconds, then killed. It fails naming the call, the remote and the limit. The runner takes that design by citing it.

## What Changes

- **One crate holds the limit and how a call is ended.** A new crate, `crates/boomdrive-bounded`, runs a process to a limit in a process group of its own. It reads each pipe on a thread of its own as the process runs. At the limit it asks the group to end, gives it two seconds, then kills what is left. It holds the limit, `TRY_FOR`, thirty seconds, which is sluice's. It also holds the words of a call it stopped. The runner's crate and crew's implementation both depend on it. It is a moon project of its own (core.7).
- **A clone's fetch is bounded.** `git::Git` gains a call to a remote, which runs through `boomdrive-bounded` to the git's limit. `clone::fetch` runs its fetch through it. A stopped fetch fails with the words `git fetch with origin was still going after 30s, the most a try may take, and was stopped`, after git's last line where git wrote one. The repository is then said `not fetched` or `not cloned`, as for any failed fetch. A new clone leaves nothing. Every other repository is still kept in that pass, and the next pass tries the fetch again.
- **A state repository's listing is bounded.** `Store::hosts` runs its `git ls-remote` through `boomdrive-bounded`. A stopped listing leaves its partition unread, with the words `git ls-remote with <url> was still going after 30s, the most a try may take, and was stopped`. The next pull tries again under core.22's backoff.
- **The runner's git calls that reach no remote are unchanged.** These are `ls-remote --get-url`, `config`, `rev-parse`, `symbolic-ref`, `update-ref`, `worktree` and the rest. They still run to their end.
- **Record 0042 and rule core.58** record the decision, citing sluice's record 0109. Both are written with this change.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `clones`: no try of a clone's fetch outlasts its limit. A stopped fetch is named, nothing it started outlives it, and the pass keeps every other repository.
- `crew-control-plane`: no listing of a state repository's runner branches outlasts its limit. A stopped listing leaves its partition unread, named.

## Impact

- `crates/boomdrive-bounded`: new. A process run to a limit in a group of its own, `TRY_FOR`, `GRACE` and a stopped call's words. It has its own unit tests.
- `crates/boomdrive`:
  - `src/git.rs`: `Git` carries its limit for a call to a remote, `TRY_FOR` by default, and gains the call that runs to it;
  - `src/clone.rs`: `fetch` runs `git fetch` as a call to a remote;
  - `tests/common/mod.rs`: a silent origin, and a pass whose git has a short limit;
  - `tests/clones.rs`: a fetch, and a clone, whose origin never answers;
  - `tests/rules.rs`: the crates depend as record 0042 has them.
- `crates/boomdrive-crew`:
  - `src/read.rs`: `Store` carries its limit, and `hosts` runs `git ls-remote` as a call to a remote;
  - `src/lib.rs`: `Crew::store` gives the store `TRY_FOR`;
  - `tests/read.rs`: a state repository that never answers the listing.
- `Cargo.toml`, `Cargo.lock`, `.moon/workspace.yml`, and the `moon.yml` of `boomdrive` and `boomdrive-crew`: the new crate and its inputs.
- `README.md` and `CLAUDE.md`: the new crate in the list of projects, and README's pass says a fetch's limit.
- `docs/adr/0042-…` and `docs/architecture/core.md`: record 0042 and core.58, written with this change.
- Nothing else changes. No tool, tracked file type, configuration key, field of `report.json`, file of crew's, or protocol message changes. The hosted implementation makes no git call and is untouched.
- Outside this repository: crew's implementation reads `<label>/main` and each runner's branch, and writes its own branch, by weir-branch's `read` and `write`. Those run weir's own `git fetch` and `git push` with no limit (`weir-branch` `keeper.rs` at `0b6eb5a`). They are weir's calls, so this change leaves them as they are. Design Risk 1 says what that leaves.

## Touches

```
crates/boomdrive-bounded/Cargo.toml
crates/boomdrive-bounded/moon.yml
crates/boomdrive-bounded/src/lib.rs
crates/boomdrive/Cargo.toml
crates/boomdrive/moon.yml
crates/boomdrive/src/git.rs
crates/boomdrive/src/clone.rs
crates/boomdrive/tests/common/mod.rs
crates/boomdrive/tests/clones.rs
crates/boomdrive/tests/rules.rs
crates/boomdrive-crew/Cargo.toml
crates/boomdrive-crew/moon.yml
crates/boomdrive-crew/src/read.rs
crates/boomdrive-crew/src/lib.rs
crates/boomdrive-crew/tests/read.rs
Cargo.toml
Cargo.lock
.moon/workspace.yml
README.md
CLAUDE.md
docs/adr/0042-no-try-of-the-runners-call-to-a-remote-outlasts-its-limit.md
docs/architecture/core.md
openspec/specs/clones/spec.md
openspec/specs/crew-control-plane/spec.md
```
