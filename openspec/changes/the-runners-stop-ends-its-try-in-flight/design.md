# Design

## Context

See proposal.md for why. There is no area page for `crates/`, so `docs/architecture/core.md` binds every path this change touches. The change relies on these rules:

- core.2: the control plane is reached through one interface. The halt reaches crew's implementation through `planes.rs`'s opener, the one place outside an implementation that names one, and `boomdrive-control` is untouched (Decision 4);
- core.13: the runner starts, keeps and stops no tool's long-running process. A try is the runner's own git call, not a tool's process, and a `flume launch` or a sluice ask in hand still runs to its end (Risk 1);
- core.20: SIGTERM and SIGINT are the signals that carry a meaning, the runner's stop. No other signal gains one;
- core.39: `run --once` exits 1 for a pass that failed. A stopped `run --once` is one (Decision 6);
- core.46: the service modules run the runner as the user's service. Their options are unchanged (Decision 7);
- core.49: each part of the control plane runs on a thread of its own. One halt reaches every thread, and no thread waits on another's try;
- core.58: no try of the runner's git call to a remote runs past thirty seconds, and `boomdrive-bounded` alone holds its ending. The halt is one more way that ending comes, in the same crate.

It departs from record 0042's last decision point, "Ctrl-C and a stop", which this change amends (Decision 8). It departs from no rule.

**The family's design.** agentplot/sluice record 0109 is the family's one design for a git call's limit, and `boomdrive-bounded` mirrors sluice's `crates/sluice/src/bounded.rs` at `e6809f9`. Record 0109's Ctrl-C point lets a wait's try in flight run to its end or its limit, and ends any other sluice command at once, its git then ending by itself. It did not take "passing Ctrl-C on to the group of the try in flight", since every command would need a signal handler. The runner already has one handler, for its stop, and runs as a service that a service manager stops. This change keeps sluice's ending, its group asked, its grace and its kill, and changes only what may trigger it in the runner. Whether sluice ends its own try in flight when a command is ended is sluice's to decide (Risk 3).

**Where a try waits today.** `boomdrive_bounded::output` polls its call every 10 ms, reading the process's exit, its two pipes and the time against the limit. Two calls use it:

- `Git::call_remote`, which `clone::fetch` runs for a pass's keep, a landing's keep and a new clone's first fetch. The keeping pass runs on the runner's loop, which reads its stop between repositories;
- `Store::hosts`, which crew's implementation runs at every pull on its part's thread. That thread reads its bell's stop between passes.

**How the runner stops today.** `Runner::stop_on_signals` spawns a thread that waits for SIGTERM or SIGINT and calls `Stopper::stop`. That sets the runner's stop flag, wakes the loop, and stops every part's bell. The loop then breaks, every part ends its pass in hand, and the runner says `boomdrive: stopped` and exits 0. `boomdrive run --once` installs no handler.

## Goals / Non-Goals

**Goals:**

- A try of the runner's git call to a remote in flight when the runner stops ends as its limit would end it: its group asked to end, given two seconds, then killed.
- No such try begins once the runner has stopped.
- The runner stops within a few seconds of SIGTERM or SIGINT where all it has in hand is such a try, and nothing a try started outlives it.
- `boomdrive run --once` stops the same way.

**Non-Goals:**

- A tool's command in hand when the runner stops: a `flume launch`, sluice's preview ask within its patience, weir-branch's read or write of crew's state repository, and a hosted part's socket. Each still runs to its end as before (Risk 1).
- A runner killed outright, by SIGKILL or a service manager's timeout (Risk 2).
- A service manager's stop timeout, which agentplot-design's ruling leaves as it is (Decision 7).
- A landing told between two repositories of a pass after the runner has stopped. It is kept as before. Its fetch begins after the halt, so it runs no process and is named with the halted words (Decision 3).

## Decisions

### 1. A halt is a value one stop sets, and every try holding it reads

`boomdrive-bounded` gains:

- `Halt`, a handle to one shared flag, `Clone`, `Debug` and `Default`. Clones share the flag. `Halt::default()` gives one never set. `halt(&self)` sets it, and setting it again changes nothing. `halted(&self) -> bool` reads it.
- `output(command: &mut Command, limit: Duration, halt: &Halt) -> io::Result<Ended>`. It runs `command` as today, and at each poll it reads `halt` beside the limit. Whichever comes first ends the group as the limit does today (Decision 2).
- `Stopped`, an enum: `AtLimit` or `Halted`.
- `Ended { status: Option<ExitStatus>, stopped: Option<Stopped>, stdout, stderr }`. `stopped` is none where the process ended by itself. `status` is none only where no process ran (Decision 3).
- `halted(call: &str, remote: &str) -> String`, the words of a call ended by its halt (Decision 5).

The runner makes one halt and gives it to whatever runs a try for it (Decision 4).

Not taken:

- **A halt kept in a static of the crate**, set by the stop and read by every call in the process. Tests run many runners side by side in one process, each with its own `Stopper`, and the halt of one would end every other's tries. A static would also make the crate's behaviour turn on state that no caller passed.
- **A registry of groups in flight, which the stop signals itself.** This is record 0042's not-taken registry. The wait loop already polls each call every 10 ms, so a flag read there ends the group through the one ending the limit uses. A registry would need its own locking. It could also signal a group whose leader had been reaped and its id reused.
- **SIGTERM passed on to the runner's own process group**, as a terminal's Ctrl-C reached git before record 0042. Each try is in a group of its own, so this would reach none of them, and it would reach a `flume launch` in flight.

### 2. A halted try is ended as its limit ends it

At the halt, `output` asks the group to end with `SIGTERM`, gives it `GRACE`, two seconds, then sends `SIGKILL`, and waits no more than 200 ms for the pipes once the group is gone. This is the same `end_group` the limit runs. A halt that comes while the group is already being ended at its limit changes nothing. `stopped` names whichever came first.

Not taken:

- **`SIGKILL` at once.** git, asked, removes the lock files it holds in the clone (record 0109's grace). A fetch killed outright can leave a lock that fails the next pass's fetch.
- **A shorter grace for a stop.** The brief asks for the limit's ending, asked to end and killed two seconds later. Two seconds is well inside launchd's twenty.

### 3. A try begun after its halt runs no process

`output` reads `halt` before it spawns. Where the halt is set, it spawns nothing and returns at once, with `status` none, `stopped` `Halted` and both pipes empty. A halt set between that read and the spawn is read at the first poll, 10 ms later, and ends the group as in Decision 2.

A try can begin after the stop:

- in a keeping pass, by a landing kept between two repositories, since `between` keeps each landing told before the pass reads the stop;
- in a part's pass in hand, whose pull lists runner branches after reading `<label>/main`.

Each such try fails at once with the halted words, rather than running to its limit while the runner waits to stop.

Not taken: **spawning the try and ending it at the first poll.** That forks a git only to end it, and asks the remote for nothing it will read.

### 4. The runner makes one halt and gives it to every try it runs

- `Runner::start` makes the runner's halt, `Halt::default()`. It gives it to the git it was given, by `Git::halted_by(halt)`, and keeps it for its `Stopper`.
- `Git` gains a private `halt: Halt`. `Git::new` and `Git::default` give one never set, and `Git::halted_by(self, halt) -> Git` gives a git with another, as `trying_for` gives one with another limit. `call_remote` runs its call to its limit and its halt.
- `planes.rs`'s `Opener` gains `halt: &Halt`, and `planes::open` passes it. `Control::open` gives the runner's halt. The `crew` entry passes it to `boomdrive_crew::open`. The `hosted` entry ignores it, since the hosted implementation makes no git call.
- crew's `open(table, file, data_dir, say_for, halt)` and `parts(config, forge, data_dir, say_for, halt)` give each part's `Crew` a clone of it, by `Crew::halted_by(self, halt) -> Crew`. `Crew::new` keeps its signature and gives a halt never set. `Store` gains `pub halt: Halt`, which `Store::new` sets never set, as it sets `try_for`. `Crew::store` gives the store its crew's halt. `Store::hosts` runs its listing to `try_for` and `halt`.
- `Stopper` gains the runner's halt. `Stopper::stop` sets the stop flag first, then the halt, then wakes the loop and every part's bell. With the flag set first, a keeping pass whose fetch the halt ended reads the stop before it begins the next repository.

`Runner::carry_for`, a test's opener, opens no implementation through `planes.rs` and takes no halt. A test that carries crew's parts makes them by `parts`, with a halt of its own.

Not taken:

- **The halt carried by `Git` and taken from it by the runner.** `Git` is the runner's tool for its git calls, not its owner of a stop. Crew's implementation runs its own git and takes nothing of `Git`'s (core.2).
- **The halt in `boomdrive-control`'s interface**, handed to every part beside its wake. The interface assumes no git, files or a socket, and the hosted implementation would carry a value it never reads. `planes.rs` already hands each implementation what the runner gives it.
- **A new parameter of `Crew::new`.** Its callers are crew's tests and the runner's `tests/pass.rs`, none of which stops a runner. `halted_by` leaves them as they are.

### 5. A halted try fails with words naming its command, its remote and the stop

`halted(call, remote)` gives `<call> with <remote> was stopped as the runner stopped`. A halted call fails as a call stopped at its limit does: git's last line on standard error first where git wrote one, then `; `, then the words. Its standard output is never read as an answer. A call begun after its halt wrote nothing, so its words stand alone.

- `Git::call_remote` fails with `Words` of `halted("git <args[0]>", remote)`. `pass::keep` then says `<address>: not fetched: git fetch with origin was stopped as the runner stopped`, and `clone::make` fails a new clone's first fetch the same way, its folder removed, `not cloned`.
- `Store::hosts` fails with `halted("git ls-remote", &self.url)`, and the part is unread with those words.

Each line is said once, as any failed fetch or unread part is. It is the last thing the runner says of that repository or part before `boomdrive: stopped`.

Not taken:

- **The limit's words.** A halted try was not still going after thirty seconds. The words would mislead a reader of the log about why it ended.
- **Saying nothing of a halted try.** The line says which repository the runner left unfetched at its stop, which the next start fetches.

### 6. `boomdrive run --once` stops on SIGTERM and SIGINT

`main` calls `stop_on_signals` for `run --once` as for `run`. `Runner::once` reads the stop:

- the keeping pass reads it between repositories, as today, and its try in flight is halted;
- once the keeping pass ends with the runner stopped, `once` begins no part's pass and no settle;
- where the stop comes while the parts' passes run, each part ends the pass in hand, its listing halted, and no settle follows;
- stopped, `once` says `boomdrive: stopped` and its outcome is `Failed`, so `run --once` exits 1.

Not taken:

- **`run --once` left to the signals' default.** A Ctrl-C would end the runner at once and leave its try's git running in its own group, which nothing ends. This is the defect the brief names, reached by a terminal instead of a service manager.
- **Exit 0 for a stopped `run --once`.** Its pass did not keep every repository or carry every part, and core.39 gives `run --once` 1 for a pass that failed.
- **An exit of its own, such as 130.** core.39 gives no exit code another meaning.

### 7. No service manager's stop timeout changes

The darwin module keeps launchd's default `ExitTimeOut`, twenty seconds, and `AbandonProcessGroup = true`. The NixOS module keeps systemd's default `TimeoutStopSec` and `KillMode = "process"`. Each still ends the runner alone, so a `flume launch` in flight runs to its end. The runner ends its own tries. This is agentplot-design's ruling of 2026-11-10: "a read stopped is harmless, so the stop ends it; no service manager's stop timeout is raised for it."

Not taken:

- **`ExitTimeOut` raised past a try's limit and grace.** The ruling rules it out. A host's stop would also wait up to 32 seconds on a remote that never answers.
- **`AbandonProcessGroup = false`.** launchd would then end the runner's process group, which holds a `flume launch` in flight, and still not reach a try in a group of its own.

### 8. Record 0042 is amended, and core.58 with it

Record 0042 gains `## Amendment, 2026-11-11: the runner's stop ends its try in flight`. It quotes the brief and agentplot-design's ruling, says which of the record's decision points and consequences it replaces, and records Decisions 1 to 7 with the paths not taken. It is accepted as record 0042 is. The direction is the brief's and the ruling's. Its form is the construct stage's own choice, and cheap to change.

core.58's rule gains: "a stop of the runner ends each such try in flight as its limit would, and begins none". Its details rule out a stop that waits on a try in flight, a try begun once the runner is stopped, a halt kept in a static, and a service manager's stop timeout raised for a try. They leave a tool's command in hand at the stop to run to its end. Its source names this change's brief and the ruling. Both are written with this change.

### 9. The tests see the try's helper end through its connection

A try's git and its helper are in no process the test started, so a test cannot name their pids. A test sees them end through the connection the helper held.

`Scratch::silent(address)` returns a `Silent`. It is the same listener and rewrite as record 0042's design Decision 6. For each connection it takes, the listener notes it taken, then reads it on a thread of its own until the far end closes it, and notes it closed. `Silent::taken(within: Duration) -> bool` waits for a connection to be taken. `Silent::closed(within: Duration) -> bool` waits for every connection taken to be closed. A test asserts that the connection is closed within a second of the runner's exit, so nothing the try started outlives the runner. The calls that ignore what `silent` returns stand as they are.

The tests stop the runner well before a try's limit, with the git each test runs on a host: thirty seconds. Each asserts the runner ends within five seconds of its signal, `GRACE` and a margin. crew's test halts its store from a thread of its own 300 ms into the listing, with `try_for` thirty seconds. The unit tests of `boomdrive-bounded` and `git.rs` halt a stand-in `sh` the same way.

## Risks / Trade-offs

1. **[A stop still waits on a tool's command in hand]** A part's pass in hand may be running `flume launch`, sluice's preview ask within its patience, weir-branch's read or write of crew's state repository, or a hosted part's wait on its socket. The stop still lets each end. weir-branch's fetch and push have no limit (record 0042's Risk 1), so a silent state repository can still hold a part's thread past launchd's twenty seconds. → Each is its tool's own call. weir-branch's limit is already told to the orchestrator under record 0042. A `flume launch` is meant to outlive a stop, which is why the modules end the runner alone.
2. **[A runner killed outright still leaves its try running]** A runner killed by SIGKILL, or by a service manager past its timeout while a tool's command held it (Risk 1), runs no `end_group`. Its try's git then runs on in its own group. → Linux can bind a child's end to its parent's (`PR_SET_PDEATHSIG`), and macOS has no such call. A Linux-only cure would leave launchd's case, the one the brief names. With this change, a runner holding only a try stops within its grace, well before either manager kills it.
3. **[sluice's commands still leave a try running when ended]** Record 0109 ends any sluice command but a wait at once on Ctrl-C, and its try's git then ends by itself. Against a remote that never answers, it does not end. The runner now ends its own tries at its stop, which sluice does not. → The ending is the family's one design, sluice's, and only its trigger differs here, on agentplot-design's ruling for the runner. Whether sluice takes the same trigger is sluice's to decide, and the construct stage tells it to the orchestrator.
4. **[`run --once` stops more slowly on Ctrl-C than before]** It used to end at once. It now lets each part end the pass in hand, which can include a `flume launch`. → This matches `run`. A second Ctrl-C does not end it sooner, since the stop is already set. A person who must end it at once can send SIGKILL, which leaves its try running (Risk 2).
