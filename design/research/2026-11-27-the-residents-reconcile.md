# The residents, reconciled with the code (2026-11-27)

This research reads the narrative `the-residents@3ee0a3e27a97` against
the code of each repository that its claims bind. The narrative includes
31 claims: 18 decisions and 13 invariants. For each claim and each bound
repository, the table gives one verdict. A bolt plan follows the breaks.
The plan is a proposal for the person, who decides it.

The person chose this work on 2026-11-27. The card asked: "Start the rest
of the process model now: one GitHub client per host, workers through
process-wrap, a worker limit?".

## What this research read

This research read each repository at its `origin/main` in the runner's
clone on deneb, by `git show` and `git grep`:

| Repository | Commit | Subject |
|---|---|---|
| boomdrivedev/sluice | `fa12b1cd5` | #201 Two pushes to main never reckon one tag |
| boomdrivedev/flume | `429eafd7` | #176 fix/releases-never-share-a-tag |
| boomdrivedev/boomdrive | `ef16c842` | #51 the set takes sluice 0.110.21 |
| boomdrivedev/weir | `ce5f2fa` | #8 format hashes only tracked files |
| afterthought/swancloud | `0714339b` | #406 spotlight stays out of the repositories |

The owed fixes are the section "Owed fixes" of
`boomdrivedev/blueprints/plans/boom-roadmap.md`, read on 2026-11-27.

In this file, a path names a file at the commit above, and the number
after the colon is its line.

## The verdicts

The marks are:

- **H**: the code holds the claim.
- **B**: the code breaks the claim.
- **P**: the code holds the claim in one part and breaks it in another.
  A P counts as a break.
- **U**: the evidence does not show whether the code holds the claim.
- **–**: the claim does not bind the repository, or the repository has
  nothing that the claim speaks of.

The columns are sluice (S), flume (F), the runner in boomdrive (R), weir
(W) and swancloud (C).

| # | Claim | S | F | R | W | C |
|---|---|---|---|---|---|---|
| 1 | `one-resident-per-tool` | H | P | H | – | – |
| 2 | `flume-has-a-resident` | H | H | H | – | – |
| 3 | `the-runner-syncs-the-drives-log` | – | – | B | – | – |
| 4 | `residents-do-the-network-and-git-work` | H | B | P | – | – |
| 5 | `clients-never-fetch-or-start-a-tool` | P | B | B | – | – |
| 6 | `reads-go-through-the-resident` | H | H | B | – | – |
| 7 | `residents-ask-each-other-by-socket` | B | P | B | – | – |
| 8 | `the-runner-serves-a-socket` | B | – | B | – | – |
| 9 | `one-github-client-per-host` | P | H | P | – | – |
| 10 | `sluice-host-reads-github-through-octocrab-with-tower-layers` | B | – | – | – | – |
| 11 | `residents-speak-json-lines-over-a-unix-socket` | H | H | B | – | – |
| 12 | `one-read-in-flight` | H | H | H | – | – |
| 13 | `one-forge-read-per-repository-per-interval` | P | H | B | – | – |
| 14 | `residents-learn-of-change-from-events-and-poll-once-a-minute-as-a-backstop` | H | H | B | – | – |
| 15 | `workers-end-with-their-owner` | H | B | P | – | – |
| 16 | `nothing-detached` | P | B | H | – | – |
| 17 | `sluices-runs-end-with-its-host-process` | H | – | – | – | – |
| 18 | `each-worker-starts-through-process-wrap` | B | B | B | – | – |
| 19 | `a-resident-ends-its-leftover-worker-groups-when-it-starts` | H | B | B | – | – |
| 20 | `one-worker-limit-per-tool` | P | B | B | – | – |
| 21 | `workers-within-their-tools-limit` | P | B | B | – | – |
| 22 | `a-session-in-a-herdr-pane-is-herdrs-child-and-flume-owns-its-place` | – | P | – | – | B |
| 23 | `a-leaked-child-fails-its-test` | H | B | B | – | – |
| 24 | `each-tools-tests-run-under-cargo-nextest` | H | B | B | – | – |
| 25 | `a-residents-core-stays-synchronous-behind-a-tokio-edge` | P | H | B | – | – |
| 26 | `tokio-streams-before-rxrust` | H | H | H | – | – |
| 27 | `one-weir-crate-holds-the-residents-skeleton-and-its-child-ownership` | B | B | B | B | – |
| 28 | `a-resident-tracks-its-tasks-with-tokio-util-and-uses-no-actor-crate` | H | H | B | – | – |
| 29 | `the-runtime-flavour-is-chosen-in-main-and-tests-pause-tokios-clock` | H | B | B | – | – |
| 30 | `the-service-manager-starts-each-resident` | P | P | H | – | H |
| 31 | `one-subscription-per-plugin` | P | B | H | – | – |

The counts, over 87 verdicts: 33 hold, 54 break (16 of them in part),
and 0 are unknown. The section "What stays unknown" names two open
questions that change no verdict.

| Repository | Holds | Breaks | In part | Unknown |
|---|---|---|---|---|
| sluice | 15 | 14 | 9 | 0 |
| flume | 10 | 17 | 4 | 0 |
| runner | 7 | 21 | 3 | 0 |
| weir | 0 | 1 | 0 | 0 |
| swancloud | 1 | 1 | 0 | 0 |

Breaks counts each B and each P. "In part" counts the P marks among
them.

## The breaks, with their evidence

### The runner serves no socket, and its rules forbid one

The accepted decision `the-runner-serves-a-socket` says that `boomdrive
run` serves a Unix socket for status and tokens. The runner's own rules
forbid that socket:

- core.3 says that the runner opens no listening socket.
- core.20 says that a command reaches a running runner only through a
  file in its data directory, and no command connects to the runner.
- The test `crates/boomdrive/tests/no_listener.rs:1` fails each source
  that names `UnixListener` or a `bind`.

Thus the decision and the rules of the repository disagree. A change
that builds the socket must amend core.3 and core.20 by a record of
boomdrive in the same branch.

This break explains these rows: 6 (`boomdrive status` reads the data
directory, as core.20 allows), 8, 11 and part of 5, 9 and 13. It also
explains part of the owed fault "The GitHub App runs out of hourly
requests". `boomdrive forge-token get` buys a new installation token
from GitHub in its own process at each ask
(`crates/boomdrive/src/forge_token.rs:11`). sluice runs that command as
its credential command (swancloud
`docs/architecture/herdr-readings.md:34`), so each new `sluice host`
process and each refused token costs two calls to GitHub's API.

### sluice reads the forge for each branch, through ureq, with no global limit

- sluice reads GitHub through ureq and backon (`Cargo.toml:65`,
  `Cargo.toml:69`, `crates/sluice/src/profiles/github/app.rs:279`). No
  repository uses octocrab or tower. No layer holds one limit on the
  requests at one time across the host. This breaks row 10.
- sluice reads the open pull requests of each branch every 60 seconds
  while nothing moves (`crates/sluice/src/profiles/github/pulls.rs:449`,
  `crates/sluice/src/instance.rs:283`). While work is in play, a pass
  asks the remote at each reading, each 1 to 3 seconds
  (`crates/sluice/src/host_process/tasks.rs:28`). The rule asks for one
  read for each repository in each interval. This breaks row 13 in part.
- sluice reads the budget from GitHub's headers and keeps a pause for the
  host (`crates/sluice/src/profiles/github/app.rs:31`,
  `crates/sluice/src/profiles/github/app.rs:1339`). That part holds.
- `sluice forge check` reads GitHub in the client process
  (`crates/sluice/src/main.rs:205`). This breaks rows 5 and 9 in part.

These breaks explain two owed faults. One is "The GitHub App runs out of
hourly requests". The other is "A `sluice submit --no-wait` exits 3 when
the GitHub App's hourly requests are spent".

The owed fault "Agents' watchers ran this account's GitHub API limit to
0 of 5000" comes from agents that run `gh` in loops. No claim of the
narrative binds an agent. This research proposes the claim
`agents-read-work-through-the-tools` for it (see "Claims proposed").

### No repository starts its workers through process-wrap

No `Cargo.toml` of sluice, flume, the runner or weir names process-wrap.
This breaks row 18 in each tool.

- sluice owns its workers with code of its own: a process group for
  each worker, a record in `host/workers/`, a stop and a sweep at start
  (`crates/sluice/src/workers.rs:1`). Rows 15, 17 and 19 hold for
  sluice.
- flume runs git with `Command::output` and no process group
  (`crates/flume/src/git.rs:79`). It runs sluice the same way from its
  merge queue profile
  (`crates/flume/src/profiles/merge_queue/sluice/mod.rs:228`). flume
  keeps no record of a worker, and sweeps nothing at its start. Rows 15
  and 19 break for flume.
- The runner runs each remote git call in a process group through
  weir-bounded (`crates/boomdrive/src/git.rs:336`), as core.58 asks. It
  runs flume, sluice and millpond with `Command::output` and no group
  (`crates/boomdrive/src/flume.rs:219`). It keeps no record and sweeps
  nothing. Row 15 breaks in part, and row 19 breaks.

### No tool has a worker limit from the host's settings

- sluice bounds its blocking pool at 256 threads
  (`crates/sluice/src/host_process/mod.rs:148`). Its git workers have no
  other limit. `[runs] max` counts only checks. Rows 20 and 21 break in
  part. The owed fault "sluice's git workers have no limit from the
  host's settings" is this break.
- flume and the runner have no limit on their workers. No repository
  uses a tokio `Semaphore` for its workers.

These breaks explain in part the owed fault "deneb's own load fails the
merge queues' gates on timing". The fault also names the agents' own
test runs, which no limit of a tool counts.

### Processes run detached

- flume's Stop hook starts `flume settle <place> --on-quiet` with
  `setsid`, in a session of its own that outlives the hook
  (`crates/flume/src/profiles/agent/claude/hook.rs:600`). The decision
  in row 22 says that a settle that a hook asks for is a request to
  `flume host`. This breaks rows 1, 16 and 22 for flume.
- A sluice client starts `sluice host` with `setsid` where no host
  answers (`crates/sluice/src/host_process/client.rs:342`). It does so
  also where the service manager keeps `sluice daemon`. Row 30 says that
  a client refuses there. The client reaps its child on a thread of its
  own (`crates/sluice/src/host_process/client.rs:390`). When the client
  ends first, nothing reaps the child. This explains the owed fault
  "`sluice overview --json --watch --stdin` starts `sluice host` as its
  own child when no host runs, and leaves one finished child unreaped".
- sluice's herdr plugin starts its watcher with `nohup`
  (`plugins/herdr/bin/watch.sh:105`). Each source starts a subshell that
  sleeps one second and records no pid
  (`plugins/herdr/bin/watch.sh:463`). This is the owed fault
  "watch.sh can leave a forked subshell for up to a second when
  stopped".

### A herdr server ends with its supervisor job

The swancloud job `swancloud.herdr.<name>` runs `herdr-supervise`
(`modules/herdr-host.nix:270`). The supervisor starts herdr's own daemon
in a process session of its own (`lib/herdr-supervise.nix:1`). On
2026-11-23 at 13:01:50, the service manager restarted the job, and each
herdr server ended with it. Each session in its panes ended too. The
decision in row 22 says that a session in a pane is a child of herdr. It
also says that a new `flume host` ends no session of a person. A server that ends with an
unrelated job breaks that promise on the host. The trigger of the
restart is not found.

### Tests do not find a leaked child

- sluice runs each Rust test task under cargo-nextest, inside
  `tests/leaks.sh`, which fails a task that leaves a process
  (`crates/sluice/moon.yml:22`, `tests/leaks.sh:1`). Rows 23 and 24
  hold for sluice.
- flume runs `cargo test` (`crates/flume/moon.yml:36`), and the runner
  runs `cargo test` (`crates/boomdrive/moon.yml:39`). Neither finds a
  leaked child. Rows 23 and 24 break for both.

The load-only failures of sluice's gate are a different fault. Each
failing test asserts a bound on real time. For example,
`a_connection_is_answered_with_no_poll` fails when the median `hello`
takes more than 4 ms. No claim of the narrative states that a
gate test holds no such bound. This research proposes the claim
`a-gate-test-asserts-no-real-time-bound` for it (see "Claims proposed").

### The runner has no tokio edge

The runner carries its parts on threads of its own, with `Condvar` and
stop flags (`crates/boomdrive/src/run.rs:120`,
`crates/boomdrive/src/flume_host.rs:38`). Its hosted implementation
builds a tokio runtime of its own (`crates/boomdrive-hosted/src/plane.rs:595`).
`main` builds no runtime, and no `TaskTracker` tracks its tasks.
This breaks rows 25, 28 and 29 for the runner.

flume host runs on a current-thread runtime in production
(`crates/flume/src/resident.rs:211`). Its blocking pool has tokio's
default bound. The decision in row 29 asks for a multi-thread runtime
with a bound in production. Row 29 breaks for flume.

sluice holds rows 25, 28 and 29 at its edge
(`crates/sluice/src/host_process/mod.rs:597`). One wait of a command
still sleeps on a blocking thread
(`crates/sluice/src/host_process/commands.rs:453`), and the client
starts threads to reap (`crates/sluice/src/host_process/client.rs:390`).
Row 25 breaks in part.

### Residents start clients of other tools

- The runner asks flume by `flume --json launch`, `act`, `status
  <place>` and `confirm` (`crates/boomdrive/src/flume.rs:219`). It asks
  sluice by its command line (`crates/boomdrive/src/sluice.rs:1`). Only
  status comes from `flume host` by one subscription
  (`crates/boomdrive/src/flume_host.rs:1`). flume host serves status
  alone on its socket (`crates/flume/src/served.rs:1`), so the runner
  has no socket for `flume launch` or an act.
- flume host reads sluice by its socket
  (`crates/flume/src/profiles/merge_queue/sluice/ask.rs:109`). It still
  runs `sluice submit`, `sluice inbox`, `sluice confirm` and `sluice
  submit --check` as clients
  (`crates/flume/src/profiles/merge_queue/sluice/mod.rs:850`,
  `crates/flume/src/profiles/merge_queue/sluice/mod.rs:934`,
  `crates/flume/src/profiles/merge_queue/sluice/mod.rs:996`).
- `sluice host` gets its tokens by running `boomdrive forge-token` as a
  credential command
  (`crates/sluice/src/profiles/github/app.rs:14`).

These break row 7 in each tool.

### flume's clients do git work

`flume launch`, `flume settle` and most other commands build flume's
engine in the client process and do the work there
(`crates/flume/src/commands/mod.rs:740`). `flume launch` runs `git fetch`
for its branch
(`crates/flume/src/git.rs:200`). Only `flume status` asks `flume host`
(`crates/flume/src/commands/mod.rs:755`). This breaks rows 4 and 5 for
flume.

### Plugins poll

- flume's Claude Code plugin runs `flume hook deliver` every 10 seconds
  in each session (`plugins/claude/hooks/flume.ts:38`,
  `plugins/claude/hooks/flume.ts:93`). Row 31 breaks for flume.
- sluice's herdr plugin holds one subscription for each folder
  (`plugins/herdr/lib/sources.sh:11`). Its forecasts run `sluice submit
  <branch> --check --json` for each worktree at its ticks
  (`plugins/herdr/bin/forecast.sh:84`). Row 31 breaks in part. This is
  part of the owed fault "Polling left on deneb", whose flume part
  landed.

### No resident starts by socket activation

The service manager starts each resident: the runner's agent, flume's
`flume-host` and sluice's `sluice daemon`, which swancloud checks
(`flake.nix:1159`). No module declares a socket of the service manager,
so no resident starts by socket activation. sluice and flume break row 30 in
part. The runner serves no socket, so the row holds for the runner.

### Parts that no repository has

- No crate of the runner keeps or syncs the drive's SQLite log. Row 3
  breaks.
- The runner passes no forge webhook to `sluice host`. Row 14 breaks
  for the runner. sluice and flume hold row 14 with file events and a
  backstop of one minute.
- weir has no crate for the skeleton of a resident. Its crates are
  adapters, bounded, branch, cadence, capture, delivery, engine, kinds,
  pack, policy, signals and weir. sluice and flume each keep a skeleton
  of their own (`crates/sluice/src/host_process/mod.rs:1`,
  `crates/flume/src/served.rs:1`). Row 27 breaks in each.

## What stays unknown

Two questions stay open. Neither changes a verdict.

- How much of the App's budget each read of sluice spends. sluice sends
  an ETag with each `GET` (`crates/sluice/src/profiles/github/app.rs:36`).
  GitHub does not count a REST answer of 304 against the budget, but it
  counts each GraphQL query. This research did not measure the reads.
  Row 13 is P for sluice from the read of each branch alone.
- What made the service manager restart the herdr jobs on 2026-11-23. The system log
  of deneb holds the cause behind sudo. Row 22 is B for swancloud from
  the observed end of the servers.

## Claims proposed

The reading found two rules that no claim states. An agent proposed each
in Boom, with 0 hard findings of ste-lint and no plan words. The person accepted
neither.

| Claim | Version | Binds | Rests on |
|---|---|---|---|
| `agents-read-work-through-the-tools`: An agent reads how its work stands through the commands of the tools, which ask the residents, and never reads the forge in a loop. | `9eae7b9d5b05`, entry 1283 | the whole system | `one-github-client-per-host`, `reads-go-through-the-resident` |
| `a-gate-test-asserts-no-real-time-bound`: A test in the gate asserts no bound on real time that the load of the host can break. | `9698ec61dc28`, entry 1284 | boomdrive, flume, sluice | `the-runtime-flavour-is-chosen-in-main-and-tests-pause-tokios-clock` |

## The bolt plan (a proposal)

This plan is a proposal for the person's approval. Each unit is one
OpenSpec change in one repository. Its name says what becomes true. The
size is in flume's points. The order puts first the faults that cost the
most on 2026-11-27. The GitHub budget that ran out comes first. The
processes that leak come next, and the tests that fail only under load
after them. The units that
change the structure come last.

### The GitHub budget

| # | Repository | Unit | Points | Needs | Closes |
|---|---|---|---|---|---|
| 1 | sluice | `sluice-host-reads-the-forge-once-per-repository-per-interval`: one read of the open pull requests of each repository in each interval, and one limit on GitHub requests at one time across the host | 5 | – | rows 9 and 13 (sluice), and the faults "The GitHub App runs out of hourly requests" and "`sluice submit --no-wait` exits 3" |
| 2 | sluice | `an-agent-waits-on-sluice-and-not-on-the-forge`: sluice's Claude Code plugin refuses `gh run watch` and a loop of `gh pr view` or `gh pr checks` in a repository that sluice queues, and names `sluice wait` | 3 | the proposed claim `agents-read-work-through-the-tools` accepted | fault "Agents' watchers ran this account's GitHub API limit to 0" |
| 3 | weir | `weir-resident-holds-the-skeleton-of-a-resident`: one crate from sluice's host process. It holds the socket, its start lock and `hello`, and the hand-over. It starts the workers through process-wrap, with their records, the sweep and one `Semaphore`. It tracks tasks with one `TaskTracker` | 13 | – | rows 18, 19, 20, 27 (weir) |
| 4 | boomdrive | `the-runner-serves-a-socket-for-status-and-tokens`: `boomdrive run` serves JSON lines on a Unix socket. `boomdrive status` and `boomdrive forge-token get` ask it. The runner keeps each token until five minutes before it ends. A record amends core.3 and core.20 | 8 | 3 | rows 5, 6, 8, 9, 11, 13 (runner), and the fault "The GitHub App runs out of hourly requests" (the mint at each ask) |
| 5 | sluice | `sluice-host-gets-its-tokens-from-the-runners-socket` | 3 | 4 | rows 7 and 8 (sluice) |

### Leaked processes

| # | Repository | Unit | Points | Needs | Closes |
|---|---|---|---|---|---|
| 6 | flume | `a-hook-asks-flume-host-to-settle`: the Stop hook sends the settle to `flume host` and starts no process | 5 | – | rows 1, 16, 22 (flume) |
| 7 | sluice | `a-sluice-client-starts-no-host-under-a-service-manager`: a client refuses where the service manager keeps `sluice daemon`, and a client that starts a host on a bare install reaps it before it ends | 3 | – | rows 5, 16, 30 (sluice), and the fault "`sluice overview --json --watch --stdin` … leaves one finished child unreaped" |
| 8 | sluice | `sluices-herdr-plugin-leaves-no-process-and-forecasts-from-its-subscription` | 5 | – | rows 16 and 31 (sluice), and the faults "watch.sh can leave a forked subshell" and the forecast part of "Polling left on deneb" |
| 9 | swancloud | `a-herdr-server-outlives-a-restart-of-its-supervisor-job` | 3 | – | row 22 (swancloud), and the fault of 2026-11-23, "A herdr server must outlive its supervisor" |
| 10 | sluice | `sluice-host-owns-its-workers-through-weir-resident`: git workers take a slot from a limit in the host's settings | 8 | 3 | rows 18, 20, 21, 27 (sluice), and the fault "sluice's git workers have no limit from the host's settings" |
| 11 | flume | `flume-host-owns-its-workers`: git and sluice run as workers through weir-resident, with a limit, on a multi-thread runtime with a bounded pool | 8 | 3 | rows 15, 18, 19, 20, 21, 27, 29 (flume) |

### Tests that fail only under load

| # | Repository | Unit | Points | Needs | Closes |
|---|---|---|---|---|---|
| 12 | sluice | `sluices-gate-asserts-no-real-time-bound`: each test of a time bound moves to a paused clock or to the speed task | 5 | the proposed claim `a-gate-test-asserts-no-real-time-bound` accepted | faults `a_connection_is_answered_with_no_poll`, the four tests of U6a, `run_again_marks_a_draft_ready`, `ten_statuses_share_one_read`, `twenty_statuses_during_a_reading_make_one_more` |
| 13 | flume | `flumes-tests-run-under-nextest-and-fail-a-leak` | 3 | – | rows 23 and 24 (flume) |
| 14 | boomdrive | `the-runners-tests-run-under-nextest-and-fail-a-leak` | 3 | – | rows 23 and 24 (runner) |

### The structure

| # | Repository | Unit | Points | Needs | Closes |
|---|---|---|---|---|---|
| 15 | flume | `flume-host-takes-launches-acts-and-confirms-on-its-socket` | 8 | 11 | row 7 (runner, with 18) |
| 16 | flume | `flume-host-asks-sluice-host-by-its-socket-for-each-submit`: each submit, inbox, confirmation and forecast goes to the socket of `sluice host` | 5 | – | row 7 (flume) |
| 17 | flume | `flumes-claude-plugin-holds-one-subscription` | 5 | – | row 31 (flume) |
| 18 | boomdrive | `the-runner-runs-on-a-tokio-edge`: a runtime built in `main`, its parts as tasks under one `TaskTracker`, its workers through weir-resident | 13 | 3 | rows 15, 18, 19, 20, 21, 25, 27, 28, 29 (runner) |
| 19 | boomdrive | `the-runner-asks-flume-host-and-sluice-host-by-their-sockets` | 5 | 15, 18 | row 7 (runner) |
| 20 | flume | `flumes-clients-ask-flume-host`: `flume launch`, `flume settle` and each command that does git work run in `flume host` | 13 | 11 | rows 4 and 5 (flume) |
| 21 | boomdrive | `the-runner-passes-the-forges-changes-to-sluice-host` | 8 | 4, and a protocol version that boomdrive-cloud speaks | row 14 (runner) |
| 22 | boomdrive | `the-set-starts-each-resident-by-socket-activation` | 5 | 3 | row 30 (sluice, flume) |
| 23 | sluice | `sluice-host-reads-github-through-octocrab-with-tower-layers` | 13 | 1 | row 10 |
| 24 | boomdrive | `the-runner-syncs-the-drives-log` | 13 | 18, and a design of the log with millpond and boom | row 3 |

The plan holds 24 units and 163 points. Units 1, 2, 6, 7, 8, 9, 12, 13,
14, 16 and 17 need no other unit, and can run side by side.
