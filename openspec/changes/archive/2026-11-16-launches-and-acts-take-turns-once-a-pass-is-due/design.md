# Design

## Context

Record 0002's amendment of 2026-11-14 on taking turns decides that once the next control pass is due, a pass's launches and its acts on places take turns, neither waiting behind more than one of the other, the pass leaving to the next what it would have run after its first of each. It leaves to this stage how the runner holds a launch the pass took and left, within core.10 and core.50. This stage's choices are recorded with that amendment, in its consequences, and core.50's and core.55's details carry them.

`Carrier::pass` in `crates/boomdrive/src/carry.rs` answers a pull's wants in two rounds, each in the control plane's order. The first takes every offer, running `flume launch` for each it launches, and answers every want that runs no command. A launch whose clone the keeper holds at its turn has its room counted then, and is made between the rounds once the keep is done. The second carries out each act on a place, as Flume's command, and each preview, as sluice's, and once the next pass is due runs no act's command after the pass's first, and no preview's after its first, leaving the rest to the next pass. `Carrier::due` says whether the next pass is due; `boomdrive run --once` gives one that never is. Once due, `Work::answered` sends the share after each want whose answer changed it.

## Goals / Non-Goals

**Goals:**

- Once the next pass is due, no act waits behind more than one launch, no launch behind more than one act, and the next pull behind more than one of each.
- No launch the pass took is left unanswered, and none is made twice (core.10).
- No answer waits on a command once the pass is due, and a pass that holds many launches does not hold its act behind a send for each.

**Non-Goals:**

- No change before the pass is due: its launches still go first, every one of them.
- No change to what an offer, an act, a command, an inbox or a preview is answered, but as Decisions 2 and 4 say.
- No change to the protocol, the interface or a control plane's implementation: `held` is an answer every one of them carries.

## Decisions

### Decision 1: A pass's first is its first of each kind

Once the next pass is due, a pass runs no `flume launch` after the first it ran, and no act's command on a place after the first it ran, whether that first ran before the pass fell due or after. This is core.50's reading of "none of either after its first past due", as it already held for acts: the work keeps a count of the launches it ran, as it keeps one of the acts it ran, and checks it beside `Carrier::due` where it checks the acts'.

The turns follow from the rounds. Before due, every launch goes first, as now. A pass that falls due among its launches makes none more, so its first act, run next in the second round, waits behind the one launch in hand. A pass due from its start makes its first launch, then its first act. A pass that falls due among its acts runs none more, so the next pull's first launch waits behind the one act in hand. Each kind keeps the control plane's order among its own, since each round walks the pull in that order.

A launch counts where its `flume launch` runs, whatever Flume answers, as an act counts where its command runs: a launch Flume refuses, or one Flume could not be run for, counts. A launch found by its mark runs no command and counts none. A launch made between the rounds, once its clone's keep is done, counts as any.

Not taken: one more launch and one more act once due, whatever ran before. A pass that falls due among its launches would run an act and then one more launch, so the next pull's act would wait behind the launch in hand and that one.

### Decision 2: A launch the pass took and left is held

Where a pass past due has run a launch and comes to an offer it would launch, every check of `Work::offer` passed and its clone standing, it answers it `held`, with the words `waits for the next control pass, which is due`, in place of running `flume launch`. Its room is counted at its turn, as a launch that waits on its clone's keep has it counted, so every offer after it gets the answer the control plane's order gives: one that finds no room left is left unanswered, if new, as it would be had the launch been made. A launch that waited on its clone's keep is held the same way between the rounds, without waiting on the keep.

The held answer is said in one line where it changes the report, as every held answer is, `<id>: held: waits for the next control pass, which is due`. The next pass takes it as any held offer, in the control plane's order: it reads the mark, the launches it comes after, the room and its turn again, and makes it where all allow. The mark keeps it from being made twice (core.10).

Its acts are answered as a held launch's are (record 0002's amendment of 2026-11-07): a priority is done, and carried when the launch is made; a drop is done, and the launch is answered done and never made; a retry, a land, a confirm or a done is refused, naming where the launch stands. A control plane learns a launch's place from the runner's report, so it has none to retry, land, confirm or end while the report holds the launch `held`.

Not taken: leaving it unanswered, which the record rules out, and which crew reads as a launch its runner holds with no word why; making it after the pass's acts, which holds the next pull behind it; and an outcome of its own, which the protocol's version 1 does not name and every control plane would read apart (core.42).

### Decision 3: Once due, a send goes before each command

Once the next pass is due, the pass sends its share before each tool's command it runs for a want, and once it has answered its last want, wherever an answer changed the share since its last send. The commands are a `flume launch`, inline or between the rounds; Flume's status of a place for an act, and Flume's command on it; and sluice's preview or release. Its end sends as before. A pass not yet due sends once, at its end, as before.

The send after each want whose answer changed the share is replaced. A pass that falls due among its launches answers each offer after with no command, and under crew each send is a commit and a push to the runner's branch, up to thirty seconds a try (core.58), so a send after each would hold its first act behind them all. A send before each command carries every answer made since the last together, and no answer waits on a command: the answer to a launch or an act goes up before the next command runs, or once the last want is answered.

The work reaches the pass's control plane where it runs a command, so it can send there. The share is still saved after every want, and a want whose save fails ends the pass's answers (core.25). A send that fails leaves the pass's answers to the send at its end, as before (core.22).

Not taken: a send after each want whose answer changed the share, held launches apart. A held answer made after a launch would then wait behind the act that follows it.

### Decision 4: What a send lets go is taken no more, its acts with it

The answer to a send before a command is taken as the end's is: what the control plane let go or refused leaves the report, said in its one line. The want about to run is then taken anew: where it, or the launch it acts on, was let go or refused, no command runs for it. No want the control plane let go or refused at a send of the pass, and no act on a launch it let go or refused, is taken more that pass, offered or not, so the runner neither acts on that work nor reports it again (core.10). An act on a launch let go was refused before, naming it no launch of the runner's, where its offer was not in the pull; it now has no answer that pass, and the next pull settles it.

Not taken: running the command the send's answer let go, which acts on work no longer the runner's.

### Decision 5: A preview waits on no launch held for its turn

A preview's command still runs in the second round, after every offer of its pull is answered, a launch held for its turn among them, and once due none after its first, counted apart from the acts (core.55). The previews spec says so where it said "after every launch the pull offers", which a held launch would otherwise leave waiting for good behind a pull that keeps offering launches.

## Risks / Trade-offs

- Where a launch takes longer than `poll_interval`, a pass makes one launch and carries out one act on a place, so a pull of many launches is made over as many passes, each adding its pull, its listing, its sends, its carrying and its settle (core.13). The record accepts it: the next pull is never held behind more than one of each.
- A priority set on a held launch is carried when it is made only while the pull still gives the priority's act, as for any held launch.
- A pass due from its start runs its launch first, so where a pull's acts and launches are both waiting, an act waits behind one launch, never none.
