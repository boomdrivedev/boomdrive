# Design

## Context

Record 0002's amendment of 2026-11-14 decides that a pass past due sends its report after each launch, act, command or preview it carries out. This change builds it in `Carrier::pass`, whose pass saves its share after every want and sends it once, at its end.

## Decisions

### Decision 1: A send follows each want whose answer changed the share

After each want is answered and the share saved, and after each launch that waited on its clone's keep, the pass sends the share where the next pass is due and a save since the last send wrote an answer. A want answered from the report, or left, writes nothing, and so sends nothing. Every answer counts, an inbox carried or an offer held among them, since none should wait on the rest of the pass. Not taken: sending before each tool's command instead, which batches answers made with no command but leaves the rule's "after each" to a reading of what is slow.

### Decision 2: The answer to a send is taken at once

The answer to a send made before the pass's end is taken as the end's is: what the control plane let go or refused leaves the report, said in its one line. The pass then takes no want of those ids more, though its pull, read before, gives them: an offer refused and its place tidied would otherwise be launched again, its report no longer holding it and Flume's listing no mark of it (core.10). An act on a launch let go waits with its launch's offer, as an act on any launch the pull offers and the report does not hold, and the next pull settles it. Not taken: holding the answer until the pass's end, which would report again what the control plane refused (core.10).

### Decision 3: A send that fails leaves the pass's answers to its end

A send before the pass's end that fails is said in one line naming the end's send, and the pass tries none more before its end, whose send is tried as it always is. Each try of crew's write may take thirty seconds where its state repository does not answer (core.58), and a pass past due may make hundreds of answers. Not taken: trying again after each answer, which the record's words also allow ("a send that fails is retried by the next send") but which holds every launch behind an unreachable control plane.

## Risks

- Under crew's control plane each send is a commit to the runner's branch, so a pass past due writes one for each answer after the due time: on deneb on 2026-11-14 that would have been about seventy commits in place of one. The record accepts it.
