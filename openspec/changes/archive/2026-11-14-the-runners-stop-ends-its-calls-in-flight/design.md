# Design

## Context

See proposal.md for why. There is no area page for `crates/`, `nix/packages/` or the root's manifests, so `docs/architecture/core.md` binds every path this change touches. The change relies on these rules:

- core.2: the control plane is reached through one interface. The halt reaches crew's implementation through `planes.rs`'s opener, the one place outside an implementation that names one, and `boomdrive-control` is untouched (Decision 4);
- core.13: the runner starts, keeps and stops no tool's long-running process. A try is the runner's own git call or weir-branch's, made for the runner, not a tool's process; a `flume launch` or a sluice ask in hand still runs to its end (Risk 1);
- core.20: SIGTERM and SIGINT are the signals that carry a meaning, the runner's stop. No other signal gains one;
- core.54: an end is confirmed to Flume only once its control plane has taken a report holding it. A halted write of the runner's branch is a report unsent, as any write that fails, so no end is confirmed for it (Decision 5, Risk 4);
- core.39: `run --once` exits 1 for a pass that failed. A stopped `run --once` is one (Decision 6);
- core.46: the service modules run the runner as the user's service. Their options are unchanged (Decision 7);
- core.49: each part of the control plane runs on a thread of its own. One halt reaches every thread, and no thread waits on another's try;
- core.58: no try of a git call to a remote, the runner's own or weir-branch's for crew's implementation, runs past thirty seconds; `weir-bounded` alone holds the limit, the halt and the ending; a stop of the runner ends each such try in flight as its limit would, through `weir-bounded`'s halt, and begins none.

The change builds record 0042's amendment of 2026-11-12, which decides its direction, and departs from no rule. It amends core.58's naming of weir's revision (Decision 8).

**The family's design.** weir record 0008, as amended on 2026-11-12, is the family's one owner of a call's limit and of its caller's halt. At weir's main `5e3f128`, `weir-bounded` gives:

- `Halt`, a handle to one flag its clones share, `Clone`, `Debug`, `Default`, `PartialEq` and `Eq`; `halt(&self)` sets it for every clone and it is never cleared; `halted(&self)` reads it; two handles are equal when they share one flag. The crate holds no halt in a static.
- `output(command, limit, halt: &Halt) -> io::Result<Ended>`, which runs a command in a process group of its own to the first of its limit and the halt. At either it asks the group to end, gives it `GRACE`, two seconds, then kills it. A command begun with its halt set runs no process.
- `Ended { status: Option<ExitStatus>, stopped: Option<Stopped>, stdout, stderr, stdout_cut, stderr_cut }`: `stopped` is `Some(Stopped::AtLimit)` or `Some(Stopped::Halted)` where one of them ended the group, and none where the command ended by itself; `status` is none only where no process ran.
- `stopped(call, remote, limit)`, as before, and `halted(call, remote)`: `<call> with <remote> was stopped as its caller asked`.

weir-branch there re-exports `Halt`, gives `Branch::with_halt(self, halt) -> Branch`, and runs each try of a branch's fetch, listing and push to the first of its limit and that halt. A halted call fails its read or write with `halted`'s words after git's last line, and a write halted while it reads lets its lock go. A branch with no remote makes no try and runs to its end whatever its halt says.

**Where a try waits today.** Each call to a remote is one of:

- `Git::call_remote`, which `clone::fetch` runs for a pass's keep, a landing's keep and a new clone's first fetch, through `git::bounded`. The keeping pass runs on the runner's loop, which reads its stop between repositories;
- `Store::hosts`, crew's listing of `<label>/runner/*`, at every pull on its part's thread;
- weir-branch's `read` and `write`, through `Store::branch`, for `Store::main`, `Store::runner_files` and `write::write`, at every pull and report on its part's thread.

Each part's thread reads its bell's stop between passes.

**How the runner stops today.** `Runner::stop_on_signals` spawns a thread that waits for SIGTERM or SIGINT and calls `Stopper::stop`. That sets the runner's stop flag, wakes the loop, and stops every part's bell. The loop breaks, every part ends its pass in hand, and the runner says `boomdrive: stopped` and exits 0. `boomdrive run --once` installs no handler.

## Goals / Non-Goals

**Goals:**

- A try of any call to a remote the runner makes, or weir-branch makes for crew's implementation, in flight when the runner stops, ends as its limit would end it: its group asked to end, given two seconds, then killed.
- No such try begins once the runner has stopped.
- The runner stops within a few seconds of SIGTERM or SIGINT where all it has in hand is such tries, and nothing a try started outlives it.
- `boomdrive run --once` stops the same way.

**Non-Goals:**

- A tool's command in hand when the runner stops: a `flume launch`, sluice's preview ask within its patience, Flume's settle in hand, a mint's calls to GitHub's API, each within its own ten seconds, and a hosted part's wait on its socket. Each still runs to its end as before (Risk 1).
- A runner killed outright, by SIGKILL or a service manager's timeout (Risk 2).
- A service manager's stop timeout, which agentplot-design's ruling leaves as it is (Decision 7).
- A landing told between two repositories of a pass after the runner has stopped. It is kept as before; its fetch begins after the halt, so it runs no git and is named with the halted words (Decision 3).

## Decisions

### 1. weir at `5e3f128`, its four crates together

`Cargo.toml` takes weir-signals, weir-capture, weir-branch and weir-bounded at `5e3f1286d7572a4c70b5e61600f2d50609793820`, weir's main once its `a-bounded-call-ends-at-its-callers-halt` landed, and `Cargo.lock` moves the four alone. `nix/packages/git-crates.nix` names each at that revision with fetchgit's hash there, taken as its own comment says, and `nix:test` holds the file to `Cargo.lock`. Between `9cba691` and `5e3f128`, weir-signals and weir-capture change only their comments' addresses, weir-branch gains the halt, and `weir-bounded`'s `output` takes a halt and its `Ended` the fields above.

The two callers of `weir_bounded::output`, `git::bounded` and `Store::hosts`, read `Ended` as it now stands: a call `Some(Stopped::AtLimit)` fails with the limit's words as before, a call `Some(Stopped::Halted)` with the halted words (Decision 5), and `status` is read only where `stopped` is none. Until Decisions 2 and 4 give each its halt, each passes one never set, so nothing a host sees changes with the pin alone.

Not taken:

- **`8fca6f3`**, the commit that landed weir's change on its bolt. `5e3f128` is the commit on weir's main that carries it, which weir record 0008's amendment and core.58 read as weir's.
- **weir-bounded alone at the new revision.** weir-branch must take the same `Halt` its caller makes, and the four move together, as record 0042's amendment of 2026-11-11 has them.

### 2. The runner's git carries the runner's halt

`Git` gains a private `halt: Halt`. `Git::new` and `Git::default` give one never set, and `Git::halted_by(self, halt) -> Git` gives a git with another, as `trying_for` gives one with another limit. `call_remote` passes its halt to `bounded`, which passes it to `weir_bounded::output` beside the limit. Every other call of `Git`'s reaches no remote and runs to its end, whatever the halt says (core.58).

Not taken: **the halt in a static of the runner's**, set by the stop and read by every call in the process. Tests run many runners side by side in one process, each with its own `Stopper`, and one's halt would end every other's tries; `weir-bounded` holds none in a static for the same reason (weir's core.1).

### 3. A try begun after the halt runs no git

`weir-bounded` reads the halt before it spawns, and a call begun with its halt set fails at once with the halted words, running no process. A try can begin after the stop:

- in a keeping pass, by a landing kept between two repositories, since `between` keeps each landing told before the pass reads the stop;
- in a part's pass in hand, whose pull reads `<label>/main`, lists the runner branches and reads each, and whose report writes the runner's branch, one after another.

Each such try fails at once, rather than running to its limit while the runner waits to stop.

### 4. The runner makes one halt and gives it to every try made for it

- `Runner::start` makes the runner's halt, `Halt::default()`. It gives it to the git it was given, by `git.halted_by(halt)`, and keeps it for its `Stopper` and its control plane.
- `planes.rs`'s `Opener` gains `halt: &Halt`, after `wake_for`, and `planes::open` passes it. `Control::open` gives the runner's halt. The `crew` entry passes it to `boomdrive_crew::open`. The `hosted` entry ignores it, since the hosted implementation makes no git call.
- crew's `open(table, file, data_dir, say_for, halt)` and `parts(config, forge, data_dir, say_for, halt)` give each part's `Crew` a clone of it, by `Crew::halted_by(self, halt) -> Crew`. `Crew::new` keeps its signature and gives a halt never set.
- `Store` gains `pub halt: Halt`, which `Store::new` sets never set, as it sets `try_for`. `Crew::store` gives the store its crew's halt. `Store::hosts` runs its listing to `try_for` and `halt`. `Store::branch` gives `Branch::remote(name, cache, url).with_halt(self.halt.clone())`, so every read and write of crew's implementation through weir-branch carries it, `write::write`'s among them, which takes its branch from the store.
- `Stopper` gains the runner's halt. `Stopper::stop` sets the stop flag first, then the halt, then sends `Event::Stop` and stops every part's bell. With the flag set first, a keeping pass whose fetch the halt ended reads the stop before it begins the next repository, as record 0042's amendment has it.

`Runner::carry_for`, a test's opener, opens no implementation through `planes.rs` and takes no halt. A test that carries crew's parts makes them by `parts`, with a halt of its own.

Not taken:

- **The halt in `boomdrive-control`'s interface**, handed to every part beside its wake. The interface assumes no git, files or a socket, and the hosted implementation would carry a value it never reads (record 0042's amendment of 2026-11-12).
- **A new parameter of `Crew::new` or `Store::new`.** Their callers are crew's tests and the runner's `tests/pass.rs`, none of which stops a runner. `halted_by` and the store's public field leave them as they are.
- **The halt given to weir-branch per call.** weir-branch takes it on the branch, by `with_halt`, and `Store::branch` is the one place crew's implementation makes a branch.

### 5. A halted try fails with `weir-bounded`'s halted words

`halted(call, remote)` gives `<call> with <remote> was stopped as its caller asked`. A halted call fails as a call stopped at its limit does: git's last line on standard error first where git wrote one, then `; `, then the words. Its standard output is never read as an answer. A call begun after its halt wrote nothing, so its words stand alone.

- `git::bounded` fails with `Words` of `halted(call, remote)`. `pass::keep` then says `<address>: not fetched: git fetch with origin was stopped as its caller asked`, and `clone::make` fails a new clone's first fetch the same way, its folder removed, `not cloned`.
- `Store::hosts` fails with `halted("git ls-remote", &self.url)`, and the part is unread with those words.
- weir-branch's read fails with its own words ending `git fetch with <url> was stopped as its caller asked`, or `git ls-remote …` for a branch not fetched, and the part is unread with them; its write fails the same way, and the report is unsent.

Each line is said once, as any failed fetch or unread part is. It is the last thing the runner says of that repository or part before `boomdrive: stopped`.

Not taken:

- **The limit's words.** A halted try was not still going after thirty seconds.
- **Words of the runner's own**, such as `stopped as the runner stopped`. core.58 rules a stopped call's words out of every crate of the runner's own, and `weir-bounded`'s are the family's.
- **Saying nothing of a halted try.** The line says which repository or partition the runner left at its stop, which the next start fetches or pulls.

### 6. `boomdrive run --once` stops on SIGTERM and SIGINT

`run::main` calls `stop_on_signals` for `run --once` as for `run`. `Runner::once` reads the stop:

- the keeping pass reads it between repositories, as today, and its try in flight is halted;
- once the keeping pass ends with the runner stopped, `once` begins no part's pass and no settle;
- where the stop comes while the parts' passes run, each part ends the pass in hand, its tries halted, and no settle follows;
- stopped, `once` says `boomdrive: stopped` and its outcome is `Failed`, so `run --once` exits 1.

Not taken:

- **`run --once` left to the signals' default.** A Ctrl-C would end the runner at once and leave its try's git running in its own group, which nothing ends.
- **Exit 0 for a stopped `run --once`.** Its pass did not keep every repository or carry every part, and core.39 gives `run --once` 1 for a pass that failed.
- **An exit of its own, such as 130.** core.39 gives no exit code another meaning.

### 7. No service manager's stop timeout changes

The darwin module keeps launchd's default `ExitTimeOut`, twenty seconds, and `AbandonProcessGroup = true`. The NixOS module keeps systemd's default `TimeoutStopSec` and `KillMode = "process"`. Each still ends the runner alone, so a `flume launch` in flight runs to its end. The runner ends its own tries. This is agentplot-design's ruling of 2026-11-10.

Not taken:

- **`ExitTimeOut` raised past a try's limit and grace.** The ruling rules it out.
- **`AbandonProcessGroup = false`.** launchd would then end the runner's process group, which holds a `flume launch` in flight, and still not reach a try in a group of its own.

### 8. core.58 names weir at `5e3f128`

core.58's rule says `weir-bounded`, "at weir's `9cba691` or later, holds the limit, the halt and the ending". `9cba691` holds no halt. The rule names `5e3f128` in its place, the first revision on weir's main that holds it, and its details rule out a pin whose `weir-bounded` has no halt beside a pin whose weir-branch runs its calls with no limit. Its source names weir's change `a-bounded-call-ends-at-its-callers-halt` on weir's main at `5e3f128` and this change's brief. Record 0042 is not amended: its amendment of 2026-11-12 leaves the revision to the build ("with weir pinned at that revision"), and naming it is this design's own choice, cheap to change.

### 9. The tests see a try's helper end through its connection

A try's git and its helper are in no process the test started, so a test cannot name their pids. A test sees them end through the connection the helper held.

`Scratch::silent(address)` returns a `Silent`. It is the same listener and rewrite as today. For each connection it takes, the listener notes it taken, then reads it on a thread of its own until the far end closes it, and notes it closed; it writes nothing. `Silent::taken(within: Duration) -> bool` waits for a connection to be taken. `Silent::closed(within: Duration) -> bool` waits for every connection taken to be closed. The calls that ignore what `silent` returns stand as they are.

The runner's tests stop the runner well before a try's limit, with the git each runs on a host, thirty seconds. Each waits for the silent remote to take the connection, sends its signal, asserts the runner ends within five seconds, `GRACE` and a margin, and asserts the connection closed within a second of the runner's exit, so nothing the try started outlives it. crew's tests halt a store from a thread of their own 300 ms into its call, with `try_for` and weir-branch's limit thirty seconds, and assert the call fails within five seconds. `git.rs`'s unit tests halt a stand-in `sh` the same way. `weir-bounded`'s and weir-branch's own tests prove the group's ending and a halted call's processes gone, and this change's tests do not repeat them.

## Risks / Trade-offs

1. **[A stop still waits on a tool's command in hand]** A part's pass in hand may be running `flume launch`, sluice's preview ask within its patience, or a mint's calls to GitHub's API, each up to ten seconds; a hosted part may be waiting on its socket; and Flume's settle in hand runs to its end. The stop still lets each end. → Each is its tool's own call, or bounded by its own limit. A `flume launch` is meant to outlive a stop, which is why the modules end the runner alone.
2. **[A runner killed outright still leaves its try running]** A runner killed by SIGKILL, or by a service manager past its timeout while a tool's command held it (Risk 1), sets no halt. Its try's git then runs on in its own group to its limit, which only the runner enforced. → Linux can bind a child's end to its parent's (`PR_SET_PDEATHSIG`), and macOS has no such call. A Linux-only cure would leave launchd's case, the one the brief names. With this change, a runner holding only tries stops within their grace, well before either manager kills it.
3. **[`run --once` stops more slowly on Ctrl-C than before]** It used to end at once. It now lets each part end the pass in hand, which can include a `flume launch`. → This matches `run`. A second Ctrl-C does not end it sooner, since the stop is already set. A person who must end it at once can send SIGKILL, which leaves its try running (Risk 2).
4. **[A report halted mid-push]** A push weir-branch halts may have reached the remote before git was asked to end. → weir-branch never takes a stopped or halted push as a refusal and never replays it; the next start reads the branch at its tip and writes from there (weir record 0008). An end is confirmed to Flume only once a report holding it is taken, so a halted write loses no end (core.54).
