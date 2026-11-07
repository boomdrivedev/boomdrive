# Design

## Context

See proposal.md for why. There are no area pages, so `docs/architecture/core.md` binds every path this change touches. It relies on core.1 (the runner decides nothing Flume decides), core.10 (each act is carried out once), core.11 (each answer is the tool's), core.22 (no pass back to back while the control plane cannot be reached) and core.25 (no act after a report that cannot be saved), and amends what this bolt's core.49 held, as core.50.

What stands, in `crates/boomdrive/src/carry.rs`: `Carrier::pass` reads the pass's one `flume --json status`, pulls, and walks the pull's wants in two rounds, each in the control plane's order. The second carries out every act on a place as Flume's command on it. In `crates/boomdrive/src/run.rs`, the loop makes a control pass when its timer or the control plane's wake makes one due, and a pass runs to its end whatever it holds.

## Goals / Non-Goals

**Goals:**

- An act on a place Flume does not stand costs the pass the reading of Flume's refusal.
- No launch a pull offers waits on more than one act of the pass before it.

**Non-Goals:**

- **Each partition's pass apart.** That is record 0026's change. Here the partitions still share one pass.
- **Flume's settle.** What an act Flume carries out costs is Flume's, and its settle before refusing a place that does not stand is told to the orchestrator for Flume.

## Decisions

### 1. A place the listing does not hold is asked of Flume's status first

`Work::act_on` asks `Work::listed` whether the pass's listing holds the place, standing or as the record Flume keeps of it tidied, as `facts::entry` reads it. Where it does not, or the pass read no listing, it runs `Flume::status_of(None, place)`, as the act's own command names no repository:

- refused: the act is answered `refused` with Flume's words, as `acted` answers any refusal of Flume's, and no act command runs;
- given: the act's command runs as before;
- not run: the act is named and left, for the next pass.

Alternatives:

- **Flume's act command on every place.** Not taken: it costs Flume's settle for the answer Flume's status gives in milliseconds (record 0010's amendment).
- **The listing as the answer.** Not taken: an act's answer is Flume's (core.1, core.11).

### 2. The pass ends its acts on places once the next is due

`Carrier` gains `due`, which says whether the next control pass is due. The second round counts the acts whose command it ran. Before each after the first, it asks `due`, and where it says so, leaves the act, which the next pull gives again, and says at the round's end how many it left. The status read of decision 1 is taken whatever `due` says, since it settles nothing.

The loop's `due` reads the control timer, `poll_interval` after the pass began, and the control plane's wake since the pass began. `Notices` gains a wake flag its clones share: `wake` sets it as it sends `Event::Wake`, and the loop clears it as a pass begins. The runner keeps one `Notices`, from which the implementation's waker, the loop and the pass are each given a clone. `run --once` hands its pass a `due` that never says so.

A pass that leaves acts ends past its timer, so the next is due at once (`next_due`). That pass pulls, takes its launches, then the acts. A pass that cannot reach its control plane leaves no act, since it carries out none, so core.22's backoff stands.

Alternatives:

- **Every act of a pull in its pass.** Not taken: a backlog of acts holds every partition's next pull.
- **Acts apart from the pass, on a thread of their own.** Not taken: two lines of work would share the report and the pass's room, and core.10 would ask an act's being under way be kept.

## Risks / Trade-offs

- **Acts on places Flume carries out get through more slowly**, each pass paying its pull and listing before its acts. Their answers reach the control plane at each pass's end, sooner than at the end of a pass of every act.
- **A launch still waits on the act in hand**, up to Flume's settle, while the partitions share one pass.
