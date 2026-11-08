# Design

## Context

See proposal.md for why. There are no area pages, so `docs/architecture/core.md` binds every path this change touches. It relies on core.1 (the runner decides nothing the control plane or Flume decides), core.9 (a launch is taken only while its stream has room, as Flume's status gives it), core.10 (each act is carried out once), core.11 (each answer is the tool's) and core.25 (no act after a report that cannot be saved).

What stands, in `crates/boomdrive/src/carry.rs`: `Carrier::pass` reads the pass's one `flume --json status`, pulls, and walks the pull's wants once, in the control plane's order, saving the report after each and stopping where it cannot. An offer is launched, held, refused or left. An act is answered from the report where it runs no command, and otherwise run as Flume's command on its launch's place. Room and holds are read from the pass's listing, and room is counted down only by the pass's own launches.

## Goals / Non-Goals

**Goals:**

- No launch of a pull waits behind an act on a place of the same pull.
- Every want gets the answer the control plane's order gives it.

**Non-Goals:**

- **Making Flume's refusal cheaper.** What an act's answer takes is Flume's (record 0010's amendment of 2026-11-07).
- **Commands and inboxes.** They keep their place in the control plane's order, so a launch after a command still finds the branch the command makes.

## Decisions

### 1. Two rounds over the pull, each in the control plane's order

`Work` holds the round under way. In the first, every want is answered as before, except that an act which would run Flume's command on its launch's place is left. In the second, only acts are taken, through the same `act`, which now runs Flume's command, and the rest of the wants are passed over. The report is saved after each want in both rounds, and a save that fails ends the pass's carrying, as core.25 asks.

An act in the second round is judged again from the report:

- one the first round answered is held there, and is left;
- one whose launch the first round took is carried out on its place, or answered from the report where the launch was held or refused;
- one whose launch the pull offers and no round took still waits.

So an act given twice in one pull runs once (core.10).

Alternatives:

- **Every act after every other want.** Not taken: a drop of a held launch, answered from the report, would come after the launch where the pull gives it ahead. Where the launch's After lands that pass, the launch would be made and then dropped, where the control plane's order never makes it (core.1).
- **An act on a place the listing does not hold answered from the listing.** Not taken: an act's answer is Flume's (core.1, core.11).
- **The control plane orders its wants.** Not taken: each control plane would order them for a cost the runner pays.

### 2. No answer turns on the round

Room and holds are read from the pass's one listing (core.9), and an act on a place moves neither within the pass. So the first round makes the launches the control plane's order makes, and each act on a place gets its answer from Flume as it did. The one change is to an act ahead of its own launch, which is now carried out on that launch's place in the same pass, where before it waited for the next. core.49 holds it, and core.1's details allow it.

## Risks / Trade-offs

- **A pass's acts on places run later in it.** Their answers reach the control plane in the same report as before.
- **A pass is no shorter.** Each refused act still costs Flume's answer. The launches now go first, so the cost no longer delays them.
