---
number: 29
title: Each pass of the runner ends with a settle of every place, which keeps Flume's watch over the host while the runner runs
status: proposed
date: 2026-11-07
decision-makers:
- Chuck Swanberg
---

# Each pass of the runner ends with a settle of every place, which keeps Flume's watch over the host while the runner runs

Status: proposed on 2026-11-07 by the construct stage of the change `each-pass-ends-with-a-settle`. The direction is the user's, in their words of 2026-11-06 quoted below. How the runner meets it, when in its loop the settle runs, the bound it states and what it says, is the stage's choice, made on this repository's rules and Flume's statement. The record is accepted when the user says yes to it.

Deciders: the user, 2026-11-06, for the direction; the construct stage, for how it is met.
Sources: Flume record 0051 and the design of Flume's change `every-place-waiting-on-the-merge-queue-is-settled-at-every-depth`, its named follow-up B1; Flume's statement, clauses 46, 119, 120 and 208 and section 9's runner; a planner's signal of 2026-11-06; Flume v0.17.0, the first release that takes `flume settle --every`; records 0002, 0003, 0010 and 0026; rules core.1, core.11, core.13, core.16, core.22 and core.50; the running spec.

## Context and problem statement

Flume settles a place when its agent goes quiet, when a caller runs a command, and, from v0.17.0, when a caller keeping watch over the host runs `flume settle --every <bound>` (Flume's 208). A caller keeps watch while its newest such settle began less than twice its bound ago. While one does, Flume runs unattended, and a place waiting on the merge queue, with no agent in it, moves within the watch's bound. While none does, Flume runs attended: every command of Flume's that acts settles every place first, and a submitted place moves only when someone acts (Flume's 119).

Until Flume v0.17.0, herdr's plugin kept that watch. The user ruled it out on 2026-11-06, reading Flume's design: "I do not like this as a solution. I do not like relying on the Herder plugin to make this work. I'd rather it be the Boomdrive process." Asked the same day whether a herdr server should still settle its own places, they ruled: "I thought we agreed that only the runner was doing the settling. I don't know what this nudge by the Herdr server is all about. I don't think we want it." Flume record 0051 holds both, and Flume's statement names the runner as the caller that keeps watch on a host that keeps one (section 9). The planner's signal of that evening put this first: "The user wants agentplot's first bolt to be Boomdrive's work."

This repository's core.13 and record 0003 had the runner run no tool's housekeeping of its own accord, and left any settle loop to the host's service manager. core.1 rules out a tool command with no ask of the control plane behind it.

The runner makes two kinds of pass on one loop: a keeping pass each `fetch_interval`, and, where it carries a control plane's work, a control pass each `poll_interval`, sooner after a pass that reported where a lease would lapse, and at once when the control plane wakes it. Both are due together often: a control pass that follows a keeping pass at start keeps step with it.

## Considered options

1. **One settle after the passes of each turn of the loop.** Once the passes due have run, keeping or control or both, and before the loop waits, the runner runs `flume settle --every <bound>` once and awaits it. A keeping pass a control pass follows at once shares that control pass's settle.
2. **One settle after every pass**, keeping and control alike, as the letter of Flume's B1 and section 9 has it.
3. **A settle on a timer of its own**, beside the two passes', each on its bound.
4. **A settle only where the last began more than half its bound ago**, so a control plane that wakes the runner often does not settle the host as often.
5. **No settle of the runner's**: a host's own timer runs `flume settle --every`, as Flume allows any caller.

For the bound the runner states: the interval within which its next turn begins, `fetch_interval`, or the shorter of it and `poll_interval` while the runner makes control passes; or the time until the turn next due.

For what it says: one line each time a settle is refused or gives no answer; or one line when that begins and one when Flume settles again.

## Decision outcome

Option 1, the interval, and one line at each change.

- **Each turn of passes ends with one settle.** `boomdrive run` runs `flume --json settle --every <bound>` once, awaited, after the passes of each turn of its loop and before it waits, and `run --once` once, after its keeping pass and its control pass. No pass leaves the runner waiting without a settle after it, and no two settles run back to back. A turn whose configuration did not read settles all the same: the watch is the host's, not the file's. A runner told to stop begins no settle, and awaits the one under way, as it awaits a git call in hand.
- **The bound is the runner's interval.** `<n>s`, whole seconds: `fetch_interval`, or the shorter of `fetch_interval` and `poll_interval` while the runner makes control passes. Each turn begins within it of the turn before, so the settles begin within it of each other, give or take the passes' own time, which Flume's doubling covers (208).
- **The settle decides nothing and goes nowhere.** It is Flume's own command, run once and awaited (core.16's shape), with no ask of a control plane behind it, and no part of a control pass: no want's answer turns on it. Every move it makes is Flume's (core.1). Its answer is carried to no control plane and kept nowhere (core.11, core.35).
- **What it says.** One line where a settle is refused, with Flume's words, or gives no answer, with why, only where the settle before was answered or it is the first; one line, `Flume settles every place again`, at the first answered after. A refused settle stops nothing, and is no failure of `run --once`.
- **No timeout of the runner's.** The runner awaits the settle as it awaits every Flume command.
- **A host whose runner stops runs attended**, once twice the last bound has passed since the last settle began. Flume offers no command that ends a watch at once, and none is needed: a lapsed watch only makes Flume settle at each command that acts (Flume's 119).

Option 2 runs two settles back to back whenever both passes are due, and holds a control pass's launches behind the keeping pass's settle, which on a host of many places took over a minute on 2026-11-06. Option 3 is the settle loop core.13 rules out, a third timer whose settles fall between passes. Option 4 cuts settles a waking control plane causes, at the price of a second clock whose miss by a moment lets the watch lapse; a settle's cost is Flume's to cut, by its own fix of a host's full status read. Option 5 is what the user ruled against. The time until the next turn would restate the bound at every settle, shorter after a pass that reported, and the watch's bound would move with the control plane's leases. A line at every refusal would repeat once a pass for a host whose Flume is older than v0.17.0, as core.23 keeps the runner from doing about its reach.

### Consequences

- Good: while the runner runs, every place on its host is settled within its interval, a submitted place with no agent in it among them, and Flume runs unattended; a host whose runner has stopped runs attended within twice its bound.
- Good: no terminal and no plugin is relied on, and one caller keeps the host's watch.
- Bad: the runner's loop waits on a settle of every place each turn, so a slow settle delays the next turn's passes by as long. Flume's fix of a host's full status read is what makes it short.
- Bad: a control plane that wakes the runner often makes it settle as often.
- Bad: the letter of Flume's statement, section 9's bullet on the runner and its row, says each pass, keeping or control, runs the settle once; under this record two passes run back to back share one. Told to the orchestrator, for Flume's statement to say each turn of passes.
- A Flume older than v0.17.0 refuses `--every` as a usage error. The runner names it once, the host runs attended as before, and the set's Flume, v0.17.1, takes it.
- core.13's rule names this settle as the one tool housekeeping the runner runs of its own accord, and its details and core.1's allow it. Record 0003's first bullet reads with this record.
- The running, configuration and family-packages specs say it, in this record's change.

## Amendment, 2026-11-08: each pass settles on its own thread

Status: proposed on 2026-11-08 by the code stage of the fix `the-runner-serves-each-partition-apart--take-main-e82cd25`, which took `main` into the bolt `the-runner-serves-each-partition-apart`, in its change `each-parts-pass-ends-with-a-settle`. It is the stage's own choice, within this record and record 0026 as they stand, and cheap to change, so it is not asked. It is accepted with this record.

### Context

This record put the settle at the end of each turn of the runner's one loop, after the keeping pass and the control pass due. Record 0026's amendment of 2026-11-07 carries each part of the control plane on a thread of its own from the end of the first keeping pass, so no part's pull, act, command or report waits on another part's or on a keeping pass (core.49). Once both stand, the loop's turn holds the keeping pass alone, and a settle on the loop could follow a part's control pass only by the part waiting on the loop.

### Decision outcome

- **Each pass ends with a settle, on its own thread.** A keeping pass ends with the settle on the loop, and each part's control pass on the part's thread, before that thread waits. The settle stays no part of the control pass.
- **The first keeping pass shares the parts' first settles.** Each part's first control pass follows it at once, so it runs no settle of its own, and no launch waits behind a settle of every place. A keeping pass after it settles on its own: where `fetch_interval` is the shorter, the parts' settles would come too seldom for the bound.
- **No two settles at once.** The loop and every part's thread share one watch. A pass that ends while another pass's settle is under way shares that settle and runs none, waiting for nothing: the watch's newest settle began within the bound, and a place the sharing pass moved after it was read moves at the next.
- **What is said is the host's.** A refusal, or no answer, is said once, and Flume settling again once, whichever thread's settle it was.
- **The bound** stays the latest `fetch_interval`, or the shorter of it and `poll_interval` while the runner carries one part or more.

Not taken:

- **The settle on the loop alone, on the shorter interval.** A settle on a timer of its own, between passes, which this record and core.13 rule out.
- **A pass waiting for the settle under way, then running its own.** Every part would wait on a settle another began, a line all parts share that core.49 rules out.
- **Settles run at once.** With many parts, as many settles of every place at once, each the cost of a host's full status read, for no watch the first does not keep.

### Consequences

- Good: no part's control pass waits on another's settle or on the loop's, and every pass still ends with a settle or shares one under way.
- Bad: a host carrying many parts settles at each part's pass where they do not overlap, as often as its parts pass.
- The letter of Flume's statement, section 9, says each pass, keeping or control, runs the settle once, which this amendment meets but for the passes that share one.
- core.13's rule and details, and core.1's and core.49's details, read with it. The running spec says it, in the change `each-parts-pass-ends-with-a-settle`.
