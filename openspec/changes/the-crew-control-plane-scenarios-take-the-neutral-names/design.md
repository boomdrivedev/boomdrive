# Design

## Context

See proposal.md for why. It changes no behaviour of the runner and no test.

What stands, read on 2026-11-07:

- **The spec.** `openspec/specs/crew-control-plane/spec.md` on this bolt is main's, with what a-dropped-launch-is-never-offered changed: four requirements reworded, five added, and one scenario added. The scenarios of sixteen requirements name private values: thirteen requirements the bolt left as main has them, and three it reworded but whose scenarios it left as main had them. The fourth it reworded, "A claim stands until the runner answers its launch", took the neutral names with a-dropped-launchs-tests-take-the-neutral-names.
- **The scrub fix.** fix/boomdrive-is-ready-to-go-public--takes-main-in-and-scrubs-it, which took main at `9b21d2c` into the public bolt, holds main's crew-control-plane spec scrubbed. Against main it differs only in scenarios' `WHEN` and `THEN` lines, and it names nothing private.
- **Record 0002.** The bolt's own amendment of it names nothing private. Its amendment for the chore a-launch-waits-on-no-other-act, which main holds and the bolt carries, names the host it was seen on; the scrub fix says "On a host" there.
- **crew's tests** on this bolt name what the scrub's scenarios name: they took boomdrives-tree-holds-nothing-private's `ed66819` (a-dropped-launchs-tests-take-the-neutral-names, its Decision 1).

There are no area pages, so `docs/architecture/core.md` binds every path this change touches. It relies on core.41 and no other rule.

## Goals / Non-Goals

**Goals:**

- Nothing private in the crew-control-plane spec or record 0002 as this bolt carries them.
- Where this bolt and the public bolt meet, the same lines, so each region either side changed alone merges as it is.

**Non-Goals:**

- **What the public bolt scrubs that this bolt carries unchanged from main**: the README's examples, the other records, `design/`, the other specs and the tests of the other crates. The scrub fix takes main in and scrubs it.
- **Record 0002's other text** that the public bolt rewrites, its citations of a private repository among it, which this bolt does not change.

## Decisions

### 1. Each scenario takes the scrub fix's lines; each requirement keeps this bolt's text

For each scenario that names a private value and that this bolt left as main had it, the delta takes the scrub fix's `WHEN` and `THEN` lines word for word. The requirement's own text stays as this bolt has it, so the three requirements here that a-dropped-launch-is-never-offered reworded keep its words. The scenarios that change was the first to write already took the same vocabulary (a-dropped-launchs-tests-take-the-neutral-names).

The scenarios were matched by requirement, scenario and line, and every line naming a private value found its counterpart there; none was written by hand. The spec as this delta leaves it differs from the scrub fix's only where a-dropped-launch-is-never-offered changed it.

Alternatives:

- **Rename by a mapping of each private value to its neutral name.** Not taken: the scrub chose its words per role (a second partition, a state repository, a blueprints repository), and a mapping would have to re-derive each choice and could differ from it.
- **Restate only the three requirements the bolt reworded, leaving the thirteen it did not touch to the public bolt.** Not taken: the bolt would land carrying them, which the planner rules out, and the thirteen merge as they are either way, since both bolts would then hold the same lines.

### 2. Record 0002's sentence takes the scrub fix's words

The amendment for a-launch-waits-on-no-other-act says "On a host on 2026-11-07" where it named the host, the sentence otherwise unchanged, word for word as the scrub fix has it. The record's decision does not change.

## Risks / Trade-offs

- **The four requirements a-dropped-launch-is-never-offered reworded still meet the public bolt's delta.** boomdrives-tree-holds-nothing-private's delta restates them in main's earlier words. Whoever merges the two bolts keeps this bolt's requirement text; their scenarios are already the same on both sides.
