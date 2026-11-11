---
number: 42
title: No try of the runner's call to a remote outlasts its limit
status: accepted
date: 2026-11-10
decision-makers:
- the construct stage of change/no-try-of-the-runners-call-to-a-remote-outlasts-its-limit
---

# No try of the runner's call to a remote outlasts its limit

Status: accepted. The direction is the change's brief, which takes agentplot/sluice record 0109 as the family's one design for a git call's limit. Its form here is the construct stage's own choice, cheap to change.

Deciders: the construct stage of `change/no-try-of-the-runners-call-to-a-remote-outlasts-its-limit`, from its brief: "No try of the runner's git call to a remote, a clone's fetch or a remote's ls-remote, runs past thirty seconds: it ends with everything it started and fails naming the remote and the limit, so one silent remote can't stall the runner's pass over every other repository."

Sources: agentplot/sluice record 0109, on sluice's main at `e6809f9` and released in v0.87.0, and its `crates/sluice/src/bounded.rs` and `runs::end_group` there; agentplot-ops's signal `2026-11-10-agentplot-ops-e19d134d/01`; `crates/boomdrive/src/clone.rs` `fetch` and `crates/boomdrive-crew/src/read.rs` `Store::hosts`; weir-branch's `keeper.rs` at `0b6eb5a`; record 0026; rules core.2, core.7, core.13, core.17, core.22, core.34 and core.49; the OpenSpec change `no-try-of-the-runners-call-to-a-remote-outlasts-its-limit`, whose design gives each point below.

## Context and problem statement

On deneb on 2026-11-10, sluice's reads of one repository each hung for 10 to 17 minutes. The cause was a git call to a remote with no limit, whose helper waited on a socket the remote had taken and never answered. sluice's record 0109 bounds every try of sluice's calls to a remote at thirty seconds. Its last consequence names the runner's fetch as one with no limit.

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

## Amendment, 2026-11-11: the runner's stop ends its try in flight

Status: accepted. The direction is the brief of the change `the-runners-stop-ends-its-try-in-flight` and agentplot-design's ruling of 2026-11-10, both quoted below. Its form here is the construct stage's own choice, cheap to change, and is accepted with the record.

Deciders: agentplot-design, ruling on 2026-11-10 ("a read stopped is harmless, so the stop ends it; no service manager's stop timeout is raised for it"); the construct stage of `change/the-runners-stop-ends-its-try-in-flight`, from its brief: "A stop of the runner ends a call to a remote it has in flight as that call's limit would, asked to end and killed two seconds later, so the runner stops within a few seconds under any service manager and leaves no git running after it."

Sources: the brief; agentplot-design's ruling of 2026-11-10; agentplot/sluice record 0109, its Ctrl-C point; `crates/boomdrive/src/run.rs` `Stopper` and `Runner::once`; `nix/modules/darwin.nix`'s `AbandonProcessGroup` and `nix/modules/nixos.nix`'s `KillMode`; rules core.2, core.13, core.20, core.39, core.46, core.49 and core.58; the OpenSpec change `the-runners-stop-ends-its-try-in-flight`, whose design gives each point below.

### Context

This record's point "Ctrl-C and a stop" read the runner's stop between repositories, so a try in flight ran to its end or its limit, up to thirty seconds and two of grace. It did not take ending the group in flight, since "the stop is already bounded by one try". launchd kills a job twenty seconds after it asks the job to stop. The darwin module keeps that default, and, like the NixOS module, ends the runner alone, so a `flume launch` in flight runs to its end. A runner killed with a try in flight leaves that try's git running in its own group, and nothing then ends it, since its limit was the runner's to enforce. `boomdrive run --once` took no signal of its own, so a Ctrl-C ended it at once and left its try's git running the same way.

### Decision outcome

This amendment replaces the point "Ctrl-C and a stop" and the consequence that a SIGTERM stops the runner within one try's limit and grace.

- **A halt ends a try as its limit does.** `boomdrive-bounded` holds a `Halt`, one shared flag that a stop sets. A try runs to its limit and its halt. At whichever comes first its group is asked to end, given `GRACE`, then killed, by the one ending the limit uses. A try begun once its halt is set runs no process. Not taken: a halt in a static of the crate, since tests run many runners in one process; a registry of groups in flight that the stop signals, since the wait already polls each try and a registry could signal a reused group id; `SIGKILL` at once, which can leave git's lock files in the clone.
- **The runner makes one halt, and its stop sets it.** `Runner::start` gives it to the runner's git and, through `planes.rs`'s opener, to crew's implementation, which gives it to each part's store. The stop sets the runner's stop flag, then its halt. Not taken: the halt in `boomdrive-control`'s interface, which assumes no git and whose hosted implementation makes no git call.
- **The words.** A halted try fails with `git <fetch|ls-remote> with <remote> was stopped as the runner stopped`, after git's last line on standard error and `; ` where git wrote one. Its repository is named not fetched, or not cloned with nothing left, or its partition unread, as for any failed try. Not taken: the limit's words, which would say a try was still going after thirty seconds when it was not.
- **`boomdrive run --once` stops on SIGTERM and SIGINT as `boomdrive run` does.** Its try in flight is halted, it begins no part's pass and no settle once stopped, says `boomdrive: stopped` and exits 1 (core.39). Not taken: the signals' default, which leaves its try's git running.
- **No service manager's stop timeout changes,** as the ruling has it. `AbandonProcessGroup` and `KillMode=process` stay, so a `flume launch` in flight still runs to its end. Not taken: `ExitTimeOut` raised past a try's limit and grace.

### Consequences

- A runner sent SIGTERM or SIGINT with only a try in hand stops within the try's grace and a margin, well inside launchd's twenty seconds, and nothing the try started outlives it.
- A stop still lets a tool's command in hand end: a `flume launch`, sluice's preview ask within its patience, weir-branch's read or write of crew's state repository. weir-branch's calls have no limit, so a silent state repository can still hold a part past launchd's twenty seconds, as this record's consequences already say.
- A runner killed outright still leaves its try running. macOS has no call that ends a child with its parent.
- sluice still ends any command but a wait at once on Ctrl-C and leaves its try's git to end by itself. The ending is the family's one design, sluice's. Only its trigger differs in the runner. Whether sluice takes the same trigger is sluice's to decide, and the construct stage tells it to the orchestrator.
- core.58's rule and details read with this amendment. The clones, crew-control-plane and running specs say it, in the change `the-runners-stop-ends-its-try-in-flight`.
