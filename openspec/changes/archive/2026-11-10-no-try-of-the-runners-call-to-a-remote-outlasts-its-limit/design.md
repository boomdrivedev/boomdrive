# Design

## Context

See proposal.md for why. There is no area page for `crates/`, so `docs/architecture/core.md` binds every path this change touches. The change relies on these rules:

- core.2: the control plane is reached through one interface, and no code outside an implementation names crew. The new crate names no implementation, and crew's implementation depends on it as the runner's crate does;
- core.7: every crate is a moon project of its own, with one task for each check;
- core.13 and core.49: each pass ends with Flume's settle, and no drive's slow answer delays another's work. A fetch with no limit holds both;
- core.14 and core.34: the runner's clone is the host's one clone, and the runner changes it only through the host's own `git` command. A bounded fetch is still that command;
- core.17: the runner is configured by its configuration alone. The limit is no key of it (Decision 1);
- core.22: a control plane that cannot be reached is tried again on a bounded backoff. A stopped listing is one such failure;
- core.40: each line the running runner writes starts with its time, then its subject and its words.

It departs from no rule. It adds core.58 and record 0042 (Decision 7).

**The family's design.** agentplot/sluice record 0109, accepted 2026-11-10, on sluice's main at `e6809f9` and released in v0.87.0, is the family's one design for a git call's limit:

- a try of a git call to a remote runs in a process group of its own;
- the try is stopped at `retry::TRY_FOR`, thirty seconds: its group is asked to end, given two seconds (`bounded::GRACE`), then killed, so the remote's helper ends with it;
- each pipe is read on a thread of its own as the process runs;
- a stopped try's words are sluice's own, `git <fetch|push|ls-remote> with <remote> was still going after 30s, the most a try may take, and was stopped`, after anything git printed.

Record 0109's last consequence names the runner's fetch in `crates/boomdrive/src/clone.rs` as one with no limit. It says the family has no shared owner of a git call's limit to cite. This change takes the design by citing record 0109, and holds the code in a crate of the runner's own (Decision 2).

**The runner's calls to a remote today.**

- `clone::fetch` (`crates/boomdrive/src/clone.rs`) runs `git --git-dir <bare> fetch --prune --quiet origin` by `Git::run`, which waits by `Command::output`. `pass::keep` calls it for a clone it has, and `clone::make` for a new clone's first fetch. A landing told to the runner is kept by the same `keep`. Before it, `not_by_ssh` runs `git ls-remote --get-url origin`, which reads the clone's configuration and reaches no remote.
- `Store::hosts` (`crates/boomdrive-crew/src/read.rs`) runs `git --git-dir <cache> ls-remote -- <url> refs/heads/<label>/runner/*` by `Command::output`. `Crew::others` calls it at every pull.
- weir-branch's `read` and `write` run their own `git fetch` and `git push` of crew's state repository. They are weir's calls (Risk 1).
- The hosted implementation makes no git call.

**Where the waits sit.** The keeping pass (`pass.rs`) keeps one repository after another, on the runner's loop. Each keep holds the clone's lock alone (`locks.rs`, record 0026), so every launch into that repository waits for it. The loop reads its stop between repositories. crew's part pulls on its own thread (record 0026), so its listing holds only that part.

## Goals / Non-Goals

**Goals:**

- No try of a clone's fetch, or of a state repository's listing, runs past thirty seconds. Nothing it started outlives it.
- A stopped call fails naming its command, its remote and the limit. The pass keeps every other repository, and the next pass or pull tries again.
- The limit and how a call is ended have one owner in the tree, which both calls use. That owner is sluice's design, cited.

**Non-Goals:**

- weir-branch's fetch and push of crew's state repository, which are weir's (Risk 1).
- Flume's own fetch, which record 0109 already names to the orchestrator.
- A limit on the runner's git calls that reach no remote. These end on their own (Decision 4).
- Trying a stopped call again within its pass or pull. A failed fetch has never been tried again within its pass (the clones spec's "One fetch fails").

## Decisions

### 1. The limit is sluice's thirty seconds, and no key of the configuration

`boomdrive-bounded` holds `TRY_FOR`, thirty seconds, and `GRACE`, two seconds, documented as agentplot/sluice record 0109's. Every call to a remote the runner makes takes `TRY_FOR`. A test may give a shorter limit through the type that makes the call (Decision 6). No host sets it.

Not taken:

- **A configuration key such as `fetch_limit_interval`.** One host would then bound its calls differently from its sluice, which reads the same remotes. Record 0109 makes `TRY_FOR` one value for every call. "A host whose fetches need longer raises `TRY_FOR`, one value for every call." Raising it here is a change to the constant, as it is in sluice.
- **git's own stall limit, `http.lowSpeedLimit` and `http.lowSpeedTime`.** This is sluice's option 2. It covers HTTP's transfer alone. It does not cover a connection stalled in its TLS handshake, which curl allows five minutes, or a helper waiting in `discover_refs`.
- **A limit on the whole keeping pass.** It would stop the repository in flight and leave every later repository unkept. That is the stall this change ends.

### 2. One crate of the runner's own holds the limit and the ending

A new crate, `crates/boomdrive-bounded`, a library with no dependency on the tree's other crates and `libc` alone outside std, holds:

- `TRY_FOR` and `GRACE` (Decision 1);
- `output(command: &mut Command, limit: Duration) -> io::Result<Ended>`. It spawns `command` with its standard output and error piped and `process_group(0)`, and reads each pipe on a thread of its own. It waits for the process and both pipes to `limit`. At the limit it asks the group to end with `SIGTERM` (`libc::killpg`), waits up to `GRACE` for it, then sends `SIGKILL`. Once the group is gone it waits no more than 200 ms for the pipes, since a process outside the group may hold one open. The caller sets standard input, which both callers set to null;
- `Ended { status, stopped, stdout, stderr }`: the exit, by itself or by the signal that ended its group; whether the limit came first; and what each pipe gave until then;
- `stopped(call: &str, remote: &str, limit: Duration) -> String`, the words `<call> with <remote> was still going after <span>, the most a try may take, and was stopped`, where `<span>` is `30s`, or `300ms` under a second, as sluice's `bounded::span` gives it.

The crate's documentation cites record 0109 and says the crate mirrors sluice's `crates/sluice/src/bounded.rs`. The runner's crate and crew's implementation each depend on it by path. It is a moon project, `boomdrive-bounded`, with `format`, `lint`, `typecheck` and `test`, each tagged `check-fast` (core.7). The `boomdrive` and `boomdrive-crew` projects' tasks take its sources as inputs, as `rules.rs`'s `each_task_takes_the_sources_of_every_crate_it_builds` requires.

Not taken:

- **A copy in each crate**, `git.rs` and `read.rs` each ending its own group. One behaviour would have two owners, which drift apart. That is record 0109's reason for one owner in sluice.
- **`boomdrive-control`.** The interface "assumes no git, files or a socket" (its crate documentation). A process runner there would make every implementation's interface crate carry one.
- **The runner's crate handing crew's implementation a function to run its listing**, through `planes.rs`. That function would need a type for a stopped call that neither std nor the interface names, which needs a shared crate anyway.
- **A crate of the family's, under weir**, that sluice, Flume, weir-branch and the runner all call. None exists, and making one is weir's and the family's design. Record 0042 tells it to the orchestrator. Since this crate mirrors sluice's module, moving to such a crate later is a dependency swap.
- **A maintained crate**, such as `process-wrap` or `wait-timeout`. Either spawns in a group or waits to a limit. The runner would still write the grace, the kill and the pipe threads around it, which is the whole of the crate.

### 3. A call stopped at its limit fails with words naming its command, its remote and the limit

- **A clone's fetch.** `Git` gains `call_remote(at, remote, args)`. It runs `args` at `at` as `run` does, but through `boomdrive_bounded::output` to the git's limit. Where the call ended by itself, its answer is read as `run`'s is: its standard output, or git's last line on standard error. Where it was stopped, it fails with `Words` of `boomdrive_bounded::stopped("git <args[0]>", remote, limit)`. Where git wrote a line on standard error before it was stopped, its last such line comes first, then `; `, then those words. A stopped call's standard output is never read as an answer. `clone::fetch` runs `["fetch", "--prune", "--quiet", "origin"]` by `git.call_remote(At::GitDir(bare), "origin", …)`. `pass::keep` then says `<address>: not fetched: git fetch with origin was still going after 30s, the most a try may take, and was stopped`, as any failed fetch, and the pass goes on to the next repository with `Outcome::Failed`. `clone::make` fails the same way, its partial folder removed as any failed clone's is. The line names the remote as `origin`, since it already begins with the repository's address.
- **A state repository's listing.** `Store::hosts` runs its `git ls-remote` through `boomdrive_bounded::output` to the store's limit. Where it was stopped, it fails with `stopped("git ls-remote", &self.url, limit)`, after git's last line where git wrote one, as above. Where it ended by itself it is read as today. The words name the URL, since the partition's line names no repository. The URL holds no credential (core.4). `Crew::others` passes the failure up, and the part is unread with its words, as for any listing that fails. The next pull tries again under core.22's backoff.

Not taken: **git's words alone**, as today's failure. A stopped call has no words of git's that say why, and a silent remote's git writes none.

### 4. Only a call that reaches a remote is bounded

`Git::run`, `bytes` and `ask` stay as they are. `git ls-remote --get-url origin` reads the clone's configuration and reaches no remote, so it stays a `run`. The same holds for every other call of the runner's: `init`, `config`, `rev-parse`, `symbolic-ref`, `update-ref`, `worktree`, `status` and the rest.

Not taken: **every git call bounded.** A local call ends on its own. A limit on `worktree add` or `update-ref` would add a way to fail, killed partway through a write, with nothing to bound. Record 0109 bounds calls to a remote alone.

### 5. A stopped call is one failed try, tried again at the next pass or pull

The runner tries each fetch once a pass, and each listing once a pull. A stopped call is such a try failed:

- a fetch's keep fails, and the next pass on `fetch_interval` fetches again;
- a listing's part is unread, and the next pull comes under core.22's backoff.

A keep that fails holds the clone's lock no longer than the limit and the grace, so a launch into the repository waits at most that long (record 0026). The loop reads its stop between repositories, so a SIGTERM or SIGINT now stops the runner within one try's limit and grace. Before, it waited for as long as the remote kept the connection.

git, in a group of its own, no longer takes the terminal's Ctrl-C with the runner. The runner's own handler sets its stop, and the try in flight ends at its end or its limit. This is record 0109's Ctrl-C for any command but a wait.

Not taken: **ending the group of the try in flight when the runner is asked to stop.** That would need a registry of tries in flight across the loop and every part's thread. The stop is already bounded by one try.

### 6. The limit is a field of the type that makes the call, for tests

`Git` gains a private `try_for: Duration`. `Git::new` and `Git::default` set it to `TRY_FOR`, and `Git::trying_for(self, limit) -> Git` gives a git with another, for tests. `Store` gains `pub try_for: Duration` and `Store::new(cache, url)`, which sets it to `TRY_FOR`. `Crew::store` makes its store by `Store::new`. No configuration key, environment variable or command-line flag sets either (Decision 1).

The tests' silent remote is a `TcpListener` on `127.0.0.1:0`. It takes every connection on a thread of its own and holds it open without a byte. That is how record 0109 measured sluice. git over HTTP connects, sends its first request and waits in `git-remote-http`, a process of git's, so a test that ends shows the helper ended with the group:

- the runner's scratch host gains `Scratch::silent(address)`. It starts such a listener and adds to the scratch git configuration `[url "http://127.0.0.1:<port>/<address>"] insteadOf = https://github.com/<address>`, which git prefers over the forge's shorter rewrite, and `[protocol "http"] allow = always`. It also gains `Scratch::pass_trying_for(config, limit)`, a pass whose git is `runner_git().trying_for(limit)`;
- crew's test makes a bare cache with `git init --bare` and a `Store` whose `url` is the listener's, `http://127.0.0.1:<port>/state.git`, with a short `try_for`.

Each such test gives a limit of 500 ms. It asserts the call ends within 5 seconds, the limit, `GRACE` and a margin, and that its words say `500ms`.

### 7. Record 0042 and core.58

Record 0042, `docs/adr/0042-no-try-of-the-runners-call-to-a-remote-outlasts-its-limit.md`, is accepted as the construct stage's own decision under the brief. It is cheap to change. It cites agentplot/sluice record 0109 as the design it takes, and records Decisions 1 to 6 with the paths not taken. It says that weir-branch's fetch and push, and Flume's fetch, remain with no limit, and that the family has no shared owner yet.

`docs/architecture/core.md` gains core.58 after core.34, with its details. `rules.rs`'s `the_crates_depend_as_decision_1_has_them` takes the new crate: `boomdrive` depends on `boomdrive-bounded` beside the interface and the implementations, `boomdrive-crew` on `boomdrive-control` and `boomdrive-bounded`, and `boomdrive-bounded` on none of the tree's crates. Its documentation cites record 0042.

## Risks / Trade-offs

1. **[A silent state repository still holds its partition at weir-branch's fetch]** crew's implementation reads `<label>/main` by weir-branch's `read` before it lists the runner branches. That read runs weir's own `git fetch` with no limit (`keeper.rs`), so a state repository that never answers holds the part there, and the bounded listing is never reached. Writing the runner's branch by weir-branch's `write` runs weir's `git push`, also with no limit. Only that part's thread waits (core.49). The keeping pass and every other part go on. → Bounding weir-branch's calls is weir's change. Record 0042 tells it to the orchestrator, beside record 0109's note of Flume's fetch. The family's shared owner of a git call's limit is for the family to design.
2. **[A fetch that needs more than thirty seconds never succeeds]** A first clone of a large repository on a slow link may take longer than thirty seconds and be stopped at every pass. → Record 0109 found the largest repository on the host it measured was 21 MiB, and a keep's fetch is incremental. A host that needs longer raises `TRY_FOR`, one value for the family, in sluice and here alike (Decision 1).
3. **[A pass still pays each silent remote's limit]** Each silent origin costs its limit and grace, about 32 seconds, before the next repository is kept. Five silent remotes cost under three minutes. → The pass is bounded, not instant. Keeping repositories side by side is not this change.
4. **[A runner killed outright leaves its try in flight running]** The service modules end the runner alone (`AbandonProcessGroup`, `KillMode=process`), and launchd kills a job 20 seconds after its SIGTERM. A try in flight then runs on, in its own group, with no limit of git's own. → This is as it stands today, when every fetch in flight outlives a killed runner. The runner holds no lock in git's files across a try, and its own locks are in its process.
