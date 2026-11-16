---
number: 50
title: Each open change is validated as it would archive, alone against the tree's specs
status: accepted
date: 2026-11-15
decision-makers:
- the user, by boomdrivedev/blueprints principle 16 as amended on 2026-11-15
- the code stage of chore/boomdrives-changes-gate-validates-each-open-change-as-archived
---

# Each open change is validated as it would archive, alone against the tree's specs

Status: accepted. Point 1 is the user's rule, blueprints principle 16 as amended on 2026-11-15, which the chore's brief carries to this repository. Points 2 to 5 are the code stage's own choices in the chore `boomdrives-changes-gate-validates-each-open-change-as-archived`, within the rules, each cheap to change.

Deciders: the user, whose answer of 2026-11-15 on their card `req_278df7ab2cc34988a3f5` amended principle 16: "OpenSpec's own validation stays; the rule is about checks of what pages say". The principle as amended keeps "OpenSpec's `openspec validate` of a kit's specs and its open changes, each change validated as it would archive". The code stage of `chore/boomdrives-changes-gate-validates-each-open-change-as-archived`, 2026-11-15, for points 2 to 5, from the chore's brief: "boomdrive's root:changes fails where an open change would not archive cleanly: each open change's deltas applied to the specs, and the result valid under openspec validate --strict, naming the change and what fails".

Sources: boomdrivedev/blueprints `design/principles.md` 16, at `3a5c19d` on `design/a-tools-own-validation-stays`; OpenSpec 1.14.3, the release `devenv.nix` pins, its `openspec archive` and `openspec validate`; rules core.7 and core.8.

## Context and problem statement

`root:specs` runs `openspec validate --specs --strict`, and `root:changes` ran `openspec validate --changes --strict`. A change can pass that and still fail at its archive, or archive into specs that `root:specs` then fails on `main`:

- `openspec archive` refuses a delta that does not apply, such as an ADDED requirement its spec already holds. `openspec validate --changes --strict` reports it only as information and passes.
- `openspec archive` checks the specs it rebuilds without `--strict`, so it writes a spec with a warning, which `root:specs` fails once the archive lands. For example, a new capability whose delta has no `## Purpose` gets the placeholder Purpose archive writes. A MODIFIED requirement past OpenSpec's 500 characters is another, since `openspec validate --changes` holds only an ADDED one to that length. A change of `chore/boomdrives-tasks-and-tests-read-no-markdown` had to cut its delta short for that reason (`e8497a86`).

Each is found only at the archive, after the gate has passed the branch.

## Considered options

For what a change is applied to:

1. **Each change alone, applied to a scratch copy of the tree's specs** (chosen).
2. Every open change applied in turn to one copy.
3. Each change applied to the tree's own specs, which the check then puts back.

For which specs are held to `--strict` after the archive:

1. **Each spec the archive writes, a new one or one that differs from the tree's** (chosen).
2. Every spec in the copy, as `openspec validate --specs --strict` reads them.

## Decision outcome

1. **`root:changes` holds each open change to OpenSpec's validation as it would archive (the user's).** It still runs `openspec validate --changes --strict`. Then, for each open change, it runs `openspec archive` and validates the specs that result with `--strict`. A change that fails is named with OpenSpec's words. `check-changes.sh` does it, and core.61 says so.
2. **Each change is applied alone to a scratch copy of the tree's specs (option 1).** The copy holds the tree's specs, `openspec/config.yaml` and that change. Each change archives at its own merge, onto `main` as it stands then.
   - Not taken: option 2. It judges a change by the other open changes and the order they are applied in, and neither is the change's own. Two open changes that conflict are found by the gate on the branch that lands second.
   - Not taken: option 3. A check writes nothing into the tree.
3. **The specs the archive writes pass `openspec validate --strict`, though `openspec archive` validates them without it.** `--strict` is how `root:specs` reads them once the change is archived.
4. **Only the specs the archive writes are validated (option 1).** A spec the change leaves as it was is `root:specs`' to check.
   - Not taken: option 2. A spec on `main` that fails would fail every open change, each named for what it did not write.
5. **A task left unchecked fails nothing.** The archive runs with `--yes`, and a change is open while its tasks are worked.

### Consequences

- Good: a change that would not archive, or would archive into specs `root:specs` fails, fails its own branch's gate, named, before it lands.
- Good: a delta that adds a capability has to carry its own `## Purpose`. Archive reads a delta's Purpose only when it creates the capability, and the placeholder fails `--strict`.
- Bad: `root:changes` runs `openspec` once more for each open change, and once for each spec its archive writes.
- sluice, Flume, boom and Boomdrive Cloud run the same `openspec validate --changes --strict` task today, and principle 16 as amended binds each of them. This record decides boomdrive's alone.
- core.61 says so, in the commit that writes this record.
