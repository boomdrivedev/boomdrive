# Design

## Context

See proposal.md for why. This change has each turn of the runner's passes end with Flume's settle of every place, `flume settle --every <bound>`, so the runner keeps Flume's watch over its host while it runs. The decision is record 0029, in this change's commit, with core.13's rule and details and core.1's details amended beside it.

What stands:

- **This repository**, at `3f428cf`.
  - `Runner::keep` (`crates/boomdrive/src/run.rs`) is the runner's one loop. Each turn runs the keeping pass where it is due, reading the configuration again, and keeps the last `fetch_interval` read. It then breaks where the runner was stopped, runs the control pass where it is due, and waits for the earlier of the two timers, a landing, a wake or a stop. A landing keeps one repository between passes. A turn always runs at least one pass: the wait ends only at a timer due or a wake with control passes made.
  - `Runner::once` runs one keeping pass, then one control pass where the control plane was opened, and gives their outcome. `run::main` exits by it.
  - `Programs`, the paths of `flume`, `sluice` and `herdr`, lives in `Control`, the opened control plane. `Programs::on_path()` names each by its name. `Runner::carry_for(open, programs)` lets a test put a control plane and stub programs of its own in place. A runner with no control plane holds no programs, and runs no tool.
  - `flume::Cli::run` (`crates/boomdrive/src/flume.rs`) puts `--json` first, unsets `FLUME_RECIPIENT`, gives no standard input, and `read` takes the answer by Flume's exit: exit 0 and JSON is `Reply::Given` as printed, exit 1 is `Reply::Refused` with Flume's `reason`, anything else `Reply::Refused` with Flume's standard error. A program that cannot be run, or a signal, is `Unrun`.
  - `Reach` (`crates/boomdrive/src/carry.rs`) says the control plane unread once, and read again once (core.23): the model for what the settle says.
  - The tests. `tests/control.rs` runs the binary with stub `flume`, `sluice`, `herdr` and `systemd-creds` first on its `PATH` (`Tools`), and the loop in-process through `carry_for`. `tests/running.rs` and `tests/dropped.rs` run the binary on the test's own `PATH`, and `tests/loop.rs` runs `Runner::start` and `keep` with `Programs::on_path()`; none of these runs Flume today, since no control plane is named. `tests/fakes/mod.rs`'s `Stub::new()` answers `status` with Flume 0.4.1's `status.json`, and any other command it has no answer for with nothing and exit 0. `tests/family.rs` runs the set's Flume and sluice by the paths the dev shell gives, `BOOMDRIVE_SET_FLUME` and `BOOMDRIVE_SET_SLUICE`, on a scratch host.
  - The flake pins Flume at `v0.10.0`, which has no `--every`. The chore `the-sets-flume-and-sluice-are-their-latest-releases`, unlanded, moves it to `v0.17.1` and sluice to `v0.74.3` at `9946cc2`, with the lock, Flume's git crates' hashes and README's override examples.
- **Flume**, from v0.17.0 (2026-11-07), and its statement:
  - `flume settle --every <bound>` takes `<n>`, `<n>s`, `<n>m` or `<n>h`. It writes the host's watch, when it began and its bound, by compare-and-set, then settles every place as `flume settle` naming none does. It answers `{"settled":{"places":[…],"tidy":{…}}}`. On a scratch host holding no place it prints `{"settled":{"places":[],"tidy":{"removed":[],"kept":[],"branches_kept":[],"not_over":[],"terminal":[]}}}`. A bound of zero is refused, exit 1. A place named beside `--every` is refused.
  - A caller keeps watch while its newest such settle began less than twice its bound ago (208). Flume runs unattended only then, and attended otherwise: every command that acts settles every place first (119). No terminal settles (122), and herdr's plugin keeps no process (Flume record 0051).
  - `flume --json status` gives the host's watch as `"watch": {"bound": {"seconds": <n>}}` while one is kept, with `watch_settled`; and `"watch": "none"` with `watch_lapsed` once it lapses.
  - Flume's state is under `$XDG_STATE_HOME/flume`, else `~/.local/state/flume`, so a Flume run on a scratch `HOME` touches nothing of the host's.
  - A Flume older than v0.17.0 refuses `--every` as a usage error, exit 2, its words on standard error.
  - Section 9 names the runner as the caller that keeps watch on a host that keeps one: "each of its passes runs `flume settle --every <bound>` once and awaits it", and in the callers' table "each pass, keeping or control".

There are no area pages, so `docs/architecture/core.md` binds every path this change touches.

## Goals / Non-Goals

**Goals:**

- While the runner runs, every Flume place on its host is settled within the runner's own interval, and Flume runs unattended; once it stops, its host runs attended within twice that interval.
- The settle decides nothing, reaches no control plane, keeps nothing, and stops nothing.
- No test runs a Flume but a stub, or the set's on a scratch host.

**Non-Goals:**

- **A bolt as a launch of a base place, and a retry of its submission.** The other half of Flume's B1, the sibling change `a-bolt-is-a-launch`.
- **The watch in the hello or a report.** Flume's status carries it, as Flume printed it to crew's state repository, and to a hosted control plane where the protocol's copy names it (core.27). This change names no new field.
- **A floor for Flume.** The runner gates no work on Flume's version today, and a Flume that refuses `--every` leaves its host attended, as before. A floor is core.38's, for its own change.
- **A settle's cost.** How long a settle of every place takes is Flume's, by its fix of a host's full status read.

## Decisions

### 1. Each turn of passes ends with one settle

`Runner::keep` notes whether its turn ran a pass. After the control pass, where the runner has not been stopped, it runs one settle, then waits as now. So:

- a keeping pass alone ends with the settle;
- a control pass alone, on its timer or woken, ends with the settle;
- a keeping pass and a control pass run back to back share one settle, after the control pass;
- a landing kept between passes is followed by none: it is no pass.

`Runner::once` runs one settle after its keeping pass and its control pass, whether the control plane was opened or not, and the settle leaves its outcome as it was.

A turn whose reading of the configuration failed still settles. The watch is the host's, not the file's, and a file mid-edit must not let Flume's places stop. The control pass after a broken reading acts on nothing, as now. The settle is no act of it.

The settle comes after the passes, never before or between. A settle between a keeping pass and a control pass would hold the control pass's launches behind a settle of every place, which took over a minute on a host of many places on 2026-11-06, and settle twice where once covers both (record 0029, option 2).

### 2. The bound is the runner's interval

`settle::bound(fetch_interval, poll_interval)` is `fetch_interval` where `poll_interval` is `None`, and the shorter of the two otherwise. `Runner` passes `Some(poll_interval)` only while it makes control passes (`carries()`: a control plane named and opened), and the `fetch_interval` the latest reading of the configuration gave: the keeping pass's in `keep`, the configuration started with in `once`.

`settle::every(bound)` writes it `<n>s`, in whole seconds rounded up, and never less than `1s`. Both intervals are whole seconds above zero already (the configuration spec), so the rounding never moves a configured interval; it keeps Flume's zero refusal out of reach.

Each turn begins within the bound of the turn before: the keeping timer is at most `fetch_interval` after its pass's start, and the control timer at most `poll_interval` after its pass's start, sooner after a lease or a wake. So the settles begin within the bound of each other, give or take the passes' own time, and Flume's doubling covers a settle that runs as long as its bound (208). The bound is not the time until the next turn: that would change at every settle, shorter after a pass that reported, and the host's watch would move with the control plane's leases.

### 3. What the runner says, and where the settle goes

`settle::Settles` holds whether the last settle was answered, none before the first. `Settles::heard(answer, say)` takes the settle's `Result<Reply, Unrun>`:

| the answer | the settle before | said |
|---|---|---|
| `Ok(Reply::Given(_))` | answered, or none | nothing |
| `Ok(Reply::Given(_))` | not answered | `boomdrive: Flume settles every place again` |
| `Ok(Reply::Refused(words))` | answered, or none | `boomdrive: Flume refused the settle of every place: <words>` |
| `Err(Unrun(why))` | answered, or none | `boomdrive: Flume gave no answer to the settle of every place: <why>` |
| a refusal or no answer | not answered | nothing |

Flume's words and why are said as given (core.1). Once per change, as the control plane's reach is said (core.23), so a host whose Flume is older than v0.17.0 says its refusal once, at the runner's start, and not at every pass.

Nothing else of the answer is read. The settle's places and tidy are Flume's, read through Flume's status at the next control pass. The answer is carried to no control plane, written to no report and kept in no file (core.11, core.35). A settle is no part of a control pass: no want is answered from it, so the work-into-flume spec's scenarios that say a want runs no `flume` command hold of the control pass as before.

No settle stops the loop, and none is a failure of `run --once`: the running spec counts a tool the control pass could not run or read as left and named, and the settle is left and named the same way.

### 4. Stop and timeout

A stop is read before the settle: a runner told to stop during its passes breaks before settling, as it breaks before a control pass now. A settle under way when the runner is told to stop is awaited, as a git call in hand is. The service modules end the runner alone at a stop, so a settle left running when the service manager kills the runner runs to its end: a settle holds its state first and is safe to repeat (Flume's core.16 and core.17).

The runner sets no timeout of its own on the settle, as it sets none on any Flume command. A Flume that hangs holds the loop, as a hung `flume launch` would.

Flume offers no command that ends a watch at once. A runner that stops leaves its last watch standing until twice its bound has passed since the settle began, then its host runs attended. That is safe: a lapsed watch only makes Flume settle at each command that acts (Flume's 119).

### 5. The set's Flume takes `--every`

The set's tests run the set's Flume (core.45), so the set's Flume must be one that keeps a watch: v0.17.1, the release the chore `the-sets-flume-and-sluice-are-their-latest-releases` moves the set to at `9946cc2`, with sluice v0.74.3, the lock, Flume's git crates' hashes and README's override examples. Task 1.1 takes that commit where the branch's `flake.nix` still names a Flume below v0.17.0, so this change does not wait on the chore's landing, and the two land as one patch whichever lands first. The tag moves with the lock (core.45).

### 6. The code

- **`src/flume.rs`**: `Cli::settle_every(&self, bound: Duration) -> Result<Reply, Unrun>`, running `flume --json settle --every <settle::every(bound)>` through `Cli::run`, so it unsets `FLUME_RECIPIENT`, gives no standard input, and is read by Flume's exit as every Flume answer is. It is `Cli`'s own and not on the `Flume` trait: only the loop runs it, and the loop always runs `Cli`; the pass's fakes keep their shape.
- **`src/settle.rs`**, new, `pub mod settle` in `src/lib.rs`: `bound`, `every` and `Settles`, as Decisions 2 and 3 have them, with their unit tests. The module's name is Flume's word for Flume's own thing (core.12).
- **`src/run.rs`**:
  - `Programs` moves from `Control` to `Runner`, set by `Programs::on_path()` at `Runner::start`, so a runner with no control plane runs Flume's settle too. `Control` keeps its relays, built from the runner's programs. `carry_for(open, programs)` sets the runner's programs, as it set the control plane's.
  - `Runner::tools(&mut self, programs: Programs)` runs `programs` in place of those on the host's `PATH`: how a test runs the loop with a stub `flume` and no control plane.
  - `Runner` holds a `Settles`, kept across turns, and `fn settle(&mut self, fetch_interval: Duration, say)` runs `flume::Cli::new(&self.programs.flume).settle_every(settle::bound(…))` and hands the answer to `Settles::heard`.
  - `keep` and `once` call it as Decision 1 has it.
- **`README.md`**, "Running on a host": each turn of passes ends with `flume settle --every <bound>`, its bound, what it says, and that a host whose runner stops runs attended once the watch lapses; the control pass's section says a control pass that follows a keeping pass at once shares its settle.

### 7. The tests

No test runs a real Flume. On a host whose `PATH` holds one, a runner in-process would settle the host's own places and write the host's watch with a test's bound, so a stub or the set's on a scratch host runs every settle:

- **`tests/common/mod.rs`**: `Scratch` writes a stub `flume` of its own into its temporary directory, which answers `--json settle` with `tests/fixtures/flume-0.17.1/settled.json` and exit 0 and anything else with nothing and exit 1, and `Scratch::boomdrive()` runs the binary with that directory first on `PATH`. So `tests/running.rs` and `tests/dropped.rs` run unchanged, and say nothing more: an answered settle is quiet. A test that sets `PATH` itself, as `tests/control.rs` does, puts its own stubs first.
- **`tests/fixtures/flume-0.17.1/settled.json`**: the set's Flume v0.17.1's answer to `--json settle --every 60s` on a scratch host holding no place, byte for byte with its trailing newline, and a `README.md` beside it saying how and where it was read, as `flume-0.4.1/README.md` does. treefmt already excludes `crates/*/tests/fixtures/**` (core.7).
- **`tests/fakes/mod.rs`**: `Stub::new()` answers `settle` with that fixture and exit 0, beside `status`.
- **`tests/flume.rs`**: `a_settle_as_the_hosts_watch`: `settle_every(Duration::from_secs(120))`'s argv is `--json settle --every 120s`, run with no `FLUME_RECIPIENT`, its answer given as printed, and a refusal on exit 1 given with Flume's reason.
- **`src/settle.rs`**, unit tests: `the_bound_is_the_fetch_interval_with_no_control_pass`, `the_bound_is_the_shorter_interval`, `every_is_whole_seconds`, `a_refusal_is_said_once_until_flume_settles_again` and `a_settle_with_no_answer_is_said_once`.
- **`tests/control.rs`**:
  - `one_control_pass`: the last `flume` call is `--json settle --every 60s`, and it is the only settle;
  - `a_control_plane_that_cannot_be_read`: `settle` joins `status` and `--version` as the commands run, once;
  - `no_control_plane` and `a_word_not_carried`: the stub `flume`'s calls are exactly one, `--json settle --every 120s`; sluice and herdr are asked nothing, as now;
  - `a_settle_flume_refuses`, new: no control plane, the stub refusing `settle` with `{"refused":{"refusal":{"reason":"no store"}}}` and exit 1; `--once` says the refusal line and exits 0;
  - `passes_back_to_back_share_one_settle`, new, in-process: `carrying("1h", "1h")`; once the first pull is made and a settle has run, the stub's calls hold one settle, `--every 3600s`, its last;
  - `each_control_pass_ends_with_a_settle`, new, in-process: `carrying("1h", "1s")`; after three pulls and a stop, the stub's calls, split at each control pass's first `status`, end each whole pass's group with one settle, `--every 1s`.
- **`tests/loop.rs`**: `mod fakes`, and `Running::start` hands the runner a `fakes::Stub` as its `flume` through `Runner::tools`, kept with the running loop so a test reads its calls; sluice and herdr are named by stubs that are not there, since a runner with no control plane runs neither. `each_keeping_pass_ends_with_a_settle`, new: no control plane, `fetch_interval` one second; after three keeping passes the stub's calls are settles alone, each `--every 1s`, at least one after each pass. `a_file_broken_while_running` also holds that the stub's settles go on, `--every 1s`, while the file is broken.
- **`tests/family.rs`**: `the_sets_flume_keeps_the_runners_watch`, new: `flume::Cli::new(family.set("flume")).settle_every(Duration::from_secs(60))` on the family's scratch host is `Reply::Given` holding `settled`, and the set's `--json status` then gives the host's `watch` as `{"bound":{"seconds":60}}`; `family.no_host_tool_ran()`.

### Rules relied on

- **core.13**, as amended in this change's commit: the settle that ends each turn of passes is the one tool housekeeping the runner runs of its own accord; no settle loop and no timer of its own.
- **core.1**, its details as amended: the settle decides nothing, and every move it makes is Flume's.
- **core.11** and **core.35**: the settle's answer is carried nowhere and kept nowhere.
- **core.16**: the settle has the shape of a tool's command the runner carries, one command line run once and awaited, though no control plane asks it.
- **core.22** and **core.23**: the bound never exceeds the runner's own intervals, and the settle's refusal is said once until it is answered again, as the control plane's reach is.
- **core.29**: no multiplexer is named; the settle is Flume's command line.
- **core.41**: the fixture is read on a scratch host, and the tests name no host, account or person's path.
- **core.45**: the set's tests run the set's Flume, moved with its lock.

### Rules departed from

- **core.13** as it stood, and record 0003's first bullet: the runner now runs one tool's housekeeping of its own accord. Both are amended in this change's commit, on the user's word in record 0029.
- **Flume's statement, section 9**, in letter: its bullet on the runner and its row in the callers' table say each pass, keeping or control, runs the settle once. Under record 0029 a keeping pass and a control pass run back to back share one settle, so the settles still come within the bound, as 208 needs, with no settle back to back and no launch held behind one. That statement is Flume's, in agentplot/blueprints: told to the orchestrator, for section 9 to say each turn of the runner's passes ends with one.

## Risks / Trade-offs

- **A slow settle delays the next turn.** The loop awaits each settle, so a settle of every place that takes a minute holds the next keeping and control pass that long; where it takes longer than the bound, the runner settles in nearly every moment between passes. Flume's fix of a host's full status read is what keeps it short.
- **A waking control plane makes the runner settle as often.** Each woken control pass ends with a settle. A settle due by time was the other way (record 0029, option 4).
- **A test that forgets the stub runs the host's Flume.** `Scratch::boomdrive()` puts the stub first for every binary run; an in-process runner needs `Runner::tools` or `carry_for`. Every such test is in this change's tasks.
- **A runner stopped leaves its watch standing for up to twice its bound.** Flume reads it as kept until then, so a submitted place waits for a command or the lapse. Flume offers no end of a watch.
- **An older Flume on a host refuses `--every`.** Said once; the host runs attended, as it does today.

## Migration Plan

Nothing held changes shape: no configuration key, no file of the runner's, no message. At the first turn of this release the runner settles every place and Flume keeps its watch; `flume status` on the host says `a caller keeps watch: every place settles every <bound>`. Where the host's herdr plugin is an older Flume's, its own watch keeps settling beside the runner's until the plugin is upgraded, which is harmless: each settle is safe to repeat.

## After deploy

Nothing. No step of this change is outside the gate.

## Task notes

- **1.1** `git cherry-pick 9946cc2` where `flake.nix` names Flume below `v0.17.0`; where it already names `v0.17.0` or later, the task is true as it stands. Its check rebuilds the set's Flume in the dev shell from the lock.
- **2.1** `settle.rs` as Decisions 2 and 3; its unit tests as Decision 7 names them.
- **2.2** `Cli::settle_every` as Decision 6; the fixture captured as Decision 7 has it, from the set's Flume, under an environment holding only a temporary `HOME`, a `PATH` and nothing of the host's, as `flume-0.4.1/README.md` describes.
- **3.1** The scratch's stub and `Stub::new()`'s settle answer land before the runner settles, so no test runs a Flume of the host's at any commit.
- **3.2** `Programs` moved to `Runner` and `Runner::tools`, per Decision 6; nothing settles yet.
- **3.3** `Runner::settle` and `Settles` held by the runner; `once` settles after its passes, per Decisions 1 to 3. `tests/control.rs` as Decision 7 names it for `--once`.
- **3.4** `keep` settles at the end of each turn that ran a pass, after the stop check that follows the control pass, per Decisions 1 and 4. `tests/control.rs`'s two in-process tests as Decision 7 names them.
