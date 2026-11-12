---
number: 42
title: No try of the runner's call to a remote outlasts its limit
status: accepted
date: 2026-11-10
decision-makers:
- the construct stage of change/no-try-of-the-runners-call-to-a-remote-outlasts-its-limit
---

# No try of the runner's call to a remote outlasts its limit

Status: accepted, with its amendment of 2026-11-11. The direction is the change's brief, which takes agentplot/sluice record 0109 as the family's one design for a git call's limit. Its form here is the construct stage's own choice, cheap to change.

Deciders: the construct stage of `change/no-try-of-the-runners-call-to-a-remote-outlasts-its-limit`, from its brief: "No try of the runner's git call to a remote, a clone's fetch or a remote's ls-remote, runs past thirty seconds: it ends with everything it started and fails naming the remote and the limit, so one silent remote can't stall the runner's pass over every other repository."

Sources: agentplot/sluice record 0109, on sluice's main at `e6809f9` and released in v0.87.0, and its `crates/sluice/src/bounded.rs` and `runs::end_group` there; agentplot-ops's signal `2026-11-10-agentplot-ops-e19d134d/01`; `crates/boomdrive/src/clone.rs` `fetch` and `crates/boomdrive-crew/src/read.rs` `Store::hosts`; weir-branch's `keeper.rs` at `0b6eb5a`; record 0026; rules core.2, core.7, core.13, core.17, core.22, core.34 and core.49; the OpenSpec change `no-try-of-the-runners-call-to-a-remote-outlasts-its-limit`, whose design gives each point below.

## Context and problem statement

On a host on 2026-11-10, sluice's reads of one repository each hung for 10 to 17 minutes. The cause was a git call to a remote with no limit, whose helper waited on a socket the remote had taken and never answered. sluice's record 0109 bounds every try of sluice's calls to a remote at thirty seconds. Its last consequence names the runner's fetch as one with no limit.

The runner has two git calls that reach a remote. Both wait by `Command::output`, with no limit:

- `clone::fetch` runs every keep's fetch: a pass's, a landing's and a new clone's first. The keeping pass keeps one repository after another. So one silent origin holds back every repository after it, and the settle that ends the pass (core.13). It also holds every launch into its repository, which waits on the clone's lock (record 0026). A SIGTERM waits for the pass to stop, so the runner cannot be stopped while the fetch hangs.
- `Store::hosts` lists a state repository's runner branches by `git ls-remote`, at every pull of crew's implementation. One silent state repository holds its partition's part.

## Considered options

1. **sluice's design, cited, in one crate of the runner's own that both calls use** (chosen).
2. **A copy of the group's ending in each crate**, `git.rs` and crew's `read.rs`.
3. **git's own stall limit**, `http.lowSpeedLimit` and `http.lowSpeedTime`, given on each call.
4. **A limit set in the runner's configuration.**

## Decision outcome

Option 1.

- **The limit is sluice's.** A try of a git call to a remote runs in a process group of its own, and at thirty seconds, `TRY_FOR`, its group is asked to end, given two seconds, `GRACE`, then killed, so the remote's helper ends with it. Each pipe is read on a thread of its own as the call runs. No key of the configuration sets the limit (core.17). A host whose calls need longer raises `TRY_FOR`, one value for the family, as record 0109 has it. Not taken: option 4, since a host would then bound its calls differently from its sluice, which reads the same remotes. Not taken: option 3, sluice's option 2, which covers HTTP's transfer alone and leaves a stalled TLS handshake or a helper waiting for refs.
- **One owner.** A crate of the runner's own, `crates/boomdrive-bounded`, holds the limit, the grace, the call run to a limit and a stopped call's words. It mirrors sluice's `bounded.rs`. The runner's crate and crew's implementation each depend on it, and it depends on none of the tree's crates (core.2), each a moon project of its own (core.7). Not taken: option 2, since one behaviour would have two owners that drift apart, record 0109's reason for one owner in sluice. Not taken: the interface crate, which assumes no git, files or a socket. Not taken: a family crate, under weir, that sluice, Flume, weir-branch and the runner all call. None exists, and it is the family's to design. Since this crate mirrors sluice's module, moving to such a crate is a dependency swap.
- **Only a call that reaches a remote.** The bounded calls are `clone::fetch`'s `git fetch` and `Store::hosts`'s `git ls-remote`. Every other git call of the runner's reaches no remote and runs to its end as before, `ls-remote --get-url` among them. Not taken: every git call bounded, which would add a way to fail, killed partway through a local write, with nothing to bound.
- **The words.** A stopped call fails with `git <fetch|ls-remote> with <remote> was still going after 30s, the most a try may take, and was stopped`, after git's last line on standard error and `; ` where git wrote one. The remote is `origin` for a fetch, whose line names the repository's address, and the state repository's URL for a listing. A stopped call's standard output is never read as an answer.
- **One try.** A stopped fetch is a failed fetch: its repository is named not fetched, or not cloned with nothing left, and the next pass fetches it again. A stopped listing leaves its partition unread, pulled again under core.22's backoff. Neither is tried again within its pass or pull.
- **Ctrl-C and a stop.** git, in a group of its own, no longer takes the terminal's Ctrl-C with the runner. The runner's own stop is read between repositories, so the try in flight ends at its end or its limit. Not taken: ending the group in flight when the runner is asked to stop, which would need a registry of tries across the loop and every part's thread, while the stop is already bounded by one try.

### Consequences

- No try of the runner's fetch, or of crew's listing, takes longer than thirty seconds and its grace, and nothing it started outlives it. A keep holds the clone's lock no longer than that, and a SIGTERM stops the runner within one try's limit and grace.
- A silent origin costs its pass about 32 seconds, and every other repository is kept in the same pass.
- A fetch that needs longer than thirty seconds, a first clone of a large repository on a slow link, is stopped at every pass until `TRY_FOR` is raised.
- weir-branch's own `git fetch` and `git push` of crew's state repository, which crew's implementation runs by `read` and `write`, still have no limit. A silent state repository still holds its partition's part at the read of `<label>/main`, before the listing, on that part's thread alone (core.49). Flume's `fetch_branch` has none either, as record 0109 says. Each is its own tool's to bound. The family has no shared owner of a git call's limit, so this is told to the orchestrator.
- This record adds core.58. The change `no-try-of-the-runners-call-to-a-remote-outlasts-its-limit` builds it.

## Amendment, 2026-11-11: weir-bounded holds the limit

Status: accepted. Made on 2026-11-11 by the code stage of the chore `boomdrive-takes-weir-bounded`, from its brief: "Boomdrive pins weir at 9cba691 or later and its runner runs its calls to a remote through weir-bounded in place of its own boomdrive-bounded crate, so its fetches and pushes of a state repository through weir-branch end at the limit too." The owner is agentplot-design's ruling of 2026-11-10, that weir owns the code of a call's limit, which weir record 0008 records. The swap is the one this record's decision foresaw, and cheap to change, so it is not asked.

Sources: weir record 0008, on weir's main at `9cba691`, with its crate `weir-bounded` and weir-branch's `Git::reach` there; the statement in agentplot/blueprints, `design/weir/requirements.md`, 69, C.3 and section 9 as amended at blueprints `ad3a1ae`; this record's "One owner" and its fourth consequence.

### Context

This record put the limit in a crate of the runner's own, `boomdrive-bounded`, a copy of sluice's `bounded.rs`, because the family had no shared owner. It left weir-branch's fetch and push of crew's state repository with no limit, each weir's to bound. weir record 0008 since made `crates/weir-bounded` that owner: `TRY_FOR`, `GRACE`, a command run to its limit in a process group of its own, the group's ending and a stopped call's words. At `9cba691` weir-branch runs its fetch, its listing and its push of a branch each as one try to that limit. Its public surface holds `boomdrive-bounded`'s: `output`, `Ended`, `stopped` and `TRY_FOR`.

### Decision outcome

- **weir's crate holds the limit.** The runner's fetch and crew's listing run through `weir-bounded`, taken at weir's revision with the other weir crates. `crates/boomdrive-bounded` goes, with its moon project. Not taken: keeping the copy beside weir's, which leaves one behaviour with two owners that drift apart, record 0109's reason for one owner.
- **weir at `9cba691` or later.** weir-branch, weir-capture and weir-signals move with weir-bounded to one revision, so crew's implementation reads `<label>/main` and each runner's branch, and writes its own, through a weir-branch whose fetch, listing and push each end at the same limit. A stopped read leaves its partition unread, and a stopped write its report unsent, each with weir-branch's words, as any read or write that fails. weir-branch never replays a stopped push (weir record 0008).
- **What a try prints is read from files.** `weir-bounded` writes a call's output to unnamed temporary files, read once the call has ended, and starts no thread, where `boomdrive-bounded` read each pipe on a thread of its own. The call's exit, its words and its limit are as before. A process of the call's group that outlives git is ended with it at the limit.
- **The words are unchanged.** `stopped(call, remote, limit)` gives the same words, so the clones and crew-control-plane specs' words stand.
- **No caller shortens weir-branch's limit.** A test of crew's implementation against a state repository that never answers waits the whole thirty seconds, its read's and its write's tests side by side in one binary. Not taken: a limit set on a branch by its caller, which weir record 0008 rules out.

### Consequences

- This record's fourth consequence is closed for weir-branch: a silent state repository holds its partition's part, or its report, no longer than one try's limit and grace. Flume's `fetch_branch` is still Flume's to bound.
- A host's runner and its weir-branch end a try at one limit, from one crate. Raising `TRY_FOR` is a change to weir.
- crew's implementation's tests run about thirty seconds longer.
- core.58 is amended with this amendment. The change `boomdrive-takes-weir-bounded` builds it, and the crew-control-plane spec says it.
