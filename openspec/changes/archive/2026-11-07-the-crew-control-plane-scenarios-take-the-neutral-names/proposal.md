# Proposal

## Why

The bolt a-dropped-launch-is-never-offered is to land with nothing private in what it carries (core.41). Its tests, its archived change and the five crew-control-plane requirements whose scenarios it wrote already take the neutral names (the change a-dropped-launchs-tests-take-the-neutral-names). The scrub fix on boomdrive-is-ready-to-go-public found more that the bolt carries: the other scenarios of the crew-control-plane spec it archives into, among them those of three requirements it reworded, record 0002, which it amends, and the README's examples, beside which it adds a paragraph, still name the operator's hosts, partitions and repositories.

The planner, 2026-11-07: "Scrub all of them to the neutral vocabulary the scrub uses (partitions home and work, hosts host-a and host-b, owner example, pool pool-a), so this bolt carries nothing private when it lands."

## What Changes

- **The crew-control-plane spec names no host, partition or account of anyone's.** Every scenario still naming one takes the words the scrub fix on boomdrive-is-ready-to-go-public gives it, its `WHEN` and `THEN` lines word for word; each requirement's own text stays as this bolt has it (design Decision 1).
- **Record 0002** names no host: its amendment for the chore a-launch-waits-on-no-other-act says "On a host" where it named one, as the scrub fix says it (design Decision 2).
- **The README's examples** of a configuration name the host `host-a`, the partition `home`, the state repository `example/home-state` and the repository `example/crew`, line for line as the scrub fix gives them (design Decision 3).
- No requirement is added, and no requirement's text, scenario's name or test changes.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `crew-control-plane`: the scenarios of sixteen requirements name the partitions `home` and `work`, the hosts `host-a` and `host-b`, and the state repositories and example repositories under `example`.

## Impact

- `openspec/specs/crew-control-plane/spec.md`, through this change's delta.
- `docs/adr/0002-…`: one sentence of its amendment of 2026-11-07 for a-launch-waits-on-no-other-act.
- `README.md`: the five lines of its examples that name a private value.
- No code, test, fixture, crate, tool, tracked file type, task or rule changes. crew's tests already name what the scenarios name (a-dropped-launchs-tests-take-the-neutral-names).
- Out of scope: what boomdrive-is-ready-to-go-public scrubs that this bolt does not carry changed.

## Touches

```
openspec/specs/crew-control-plane/spec.md
README.md
docs/adr/0002-the-runner-takes-leased-work-while-its-flume-stream-has-room-and-carries-each-tools-record-back-as-given.md
```
