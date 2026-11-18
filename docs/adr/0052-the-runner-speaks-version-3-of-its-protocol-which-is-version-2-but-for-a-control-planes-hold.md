---
number: 52
title: The runner speaks version 3 of its protocol, which is version 2 but for a control plane's hold
status: accepted
date: 2026-11-18
decision-makers:
- the construct stage of change/the-hosted-protocol-carries-a-hold
---

# The runner speaks version 3 of its protocol, which is version 2 but for a control plane's hold

Status: accepted. The direction is the change's brief, quoted below. How version 3 meets it is the construct stage's own choice, made on this repository's rules: what version 3 holds beside version 2, its files' `$id`, and how the tests hold version 3 to version 2. Each is cheap to change until a release of the runner speaks version 3. From then on version 3 never changes (core.42), and a change to it is a version 4.

The brief, 2026-11-18: "a new version of the runner's hosted protocol carries the hold act, its words and bound, so a hosted control plane can hold the move a place waits on it for as crew's can, versions 1 and 2 unchanged".

Deciders: the change's brief, for the direction; the construct stage of the change `the-hosted-protocol-carries-a-hold`, for the rest.

Sources: the brief; records 0015, 0037 and its amendment of 2026-11-09, and 0002 as amended on 2026-11-14; rules core.2, core.10, core.42, core.51 and core.59; boomdrivedev/blueprints `design/flume/requirements.md`, clause 130, and its row of the runner among Flume's callers (`7cdb007`); the change `the-runner-carries-a-hold-of-a-places-waiting-move`, whose design Decision 7 left a hold to a later version; the change `the-protocols-version-two-is-held-as-v0-9-0-spoke-it`; boomdrivedev/boomdrive-cloud records 0056 and 0069.

## Context and problem statement

A control plane asks that the move a launch's place waits on it for be held with `hold`, an act naming its launch, its words and its bound in whole seconds (core.59). crew's implementation gives one, and the runner carries it out as Flume's hold of that place's waiting move. No version of the hosted protocol names a hold. Version 1 was spoken by `v0.1.0` to `v0.7.3`, and version 2 by every release from `v0.8.0`, so neither changes (core.42). A want whose act is a hold fails version 2's check, so the whole `pulled` that holds one is refused. A hosted control plane's proven work therefore stalls after five minutes, its stall delivered as though something had failed, where crew's waits under words.

Version 2 carries every other sort the interface names. `boomdrive-control`'s `ActSort::Hold` is the one the hosted crate's test of every sort finds refused.

## Considered options

For what carries the hold:

1. **Version 3, version 2 whole but for the hold (chosen).**
2. **Version 3, carrying the hold and the other things version 2 lacks**: a forge token, health, the runner's own commands, a terminal's certificate, an observing connection (record 0048). None is in the brief, and none is built. Each would hold this version back for a design not yet made.
3. **Version 2 changed to carry it.** A release has spoken version 2, so core.42 rules it out.

For the hold's form in the schema:

- a. **As the interface writes it, `{"hold":{"words":…,"bound_interval":…}}`, its words any string and its bound an integer from 0 that JavaScript reads exactly (chosen).**
- b. Words of one character or more, and a bound of 1 or more.

For each file's `$id`:

- i. **Under `boomdrivedev/boomdrive`, where the files are now (chosen).**
- ii. Under `agentplot/boomdrive`, as versions 1 and 2 name them.

For how a test holds version 3 to version 2:

- **Each schema file of version 3 equals version 2's but for the differences this record names, read with every `description` set aside, and each example of version 2 has its like in version 3 (chosen).**
- Each requirement of version 2's content tested again against version 3, with no test of what differs.

## Decision outcome

Option 1, with a, i and the test of what differs.

- **Version 3 is version 2 whole, at `crates/boomdrive-hosted/protocol/v3/`, but for these:**
  - every file's `$id` is under `https://raw.githubusercontent.com/boomdrivedev/boomdrive/main/crates/boomdrive-hosted/protocol/v3/`, and every message's `protocol_version` is 3;
  - `wants.schema.json`'s `act_sort` gains a hold, `{"hold":{"words":…,"bound_interval":…}}`, closed, both fields required, `words` a string and `bound_interval` `shared.schema.json`'s `unsigned_integer`;
  - a description that names version 2 names version 3, and `act_sort`'s names the hold.
  Nothing else changes. Its examples are version 2's, each naming `protocol_version` 3, `enroll.json` signed over version 3's context, and one more, a `pulled` holding a hold.
- **The hold's words and bound are bounded no further.** The runner refuses a hold with no words, running nothing (core.59). A bound of 0 is passed to Flume as given, and Flume lets that hold lapse at once. A bound in the schema would refuse a whole `pulled` for one act. Unbounded, the hold reaches the runner as an act and is answered as one, as a confirm of no key is (record 0037).
- **Version 3's `$id`s name `boomdrivedev/boomdrive`.** The runner-protocol spec has each file name its own address as its `$id`, and the repository is now `boomdrivedev/boomdrive`. Versions 1 and 2 keep the `agentplot` address their releases spoke. Every `$ref` is relative and no check fetches a `$id`, so each version resolves within its own files whatever its address.
- **The runner speaks version 3 alone**, as record 0037 decided for version 2. Its upgrade asks for `boomdrive.v3`, its `enroll` is a message of version 3, and the contexts its keys sign start `boomdrive.v3`. A control plane accepts the current version and the one before (the protocol's README; record 0015). One that speaks version 2 alone refuses the upgrade, which the runner says once and tries again at its backoff's cap (core.22, core.23). A key enrolled under version 2 stays the partition's, and its next connection proves it under version 3. The runner's message types move to version 3's module, and version 2's keeps its files alone, as version 1's does.
- **A hosted control plane's hold is carried as crew's.** A `pulled` whose act is a hold passes version 3 and is given to the runner as any act, under the partition's prefix. The runner carries it out as core.59 says, whatever the implementation (core.2). Nothing of the hosted implementation reads it or refuses it.
- **Version 2 stays as `v0.9.0` spoke it**, and version 1 as `v0.7.3` did. Each file of each stays held to its digest, and each example of each still passes its own version.
- **A test holds version 3 to version 2.** Each schema file of version 3 is version 2's with every `description` set aside, its `$id` read under version 2's address, its `protocol_version` 2 and the hold taken out of `act_sort`, and is then equal to version 2's. Each example of version 2 has an example of version 3 by the same name, equal to it but for its `protocol_version` and, in `enroll.json`, its signature. So version 3 carries every end, confirm, land and preview version 2 carries, and differs from it by exactly the hold.

Not taken:

- **Option 2.** It holds a hosted control plane's hold back for designs no brief asks for yet. Each of those adds itself in a version of its own. A version may change until a release speaks it (core.42), so one built on the same bolt before it lands may still join version 3.
- **Option 3.** core.42.
- **b.** It would refuse a whole `pulled`, every want in it, for one act the runner answers itself.
- **ii.** The address would name an organisation the repository has left, against the runner-protocol spec's own address.

### Consequences

- Good: a hosted control plane holds the move a place waits on it for, with its words and bound, as crew's does, so its proven work waits under words in place of stalling.
- Good: a test shows exactly what version 3 adds, so a reader of version 2's requirements reads version 3's.
- Bad: a control plane that speaks version 2 alone cannot be reached by a release that carries this record until it takes version 3. boomdrive-cloud pins version 2 today, so it takes version 3 by a change of its own, and its local preview's observer waits on the version that carries an observing connection (its record 0069).
- Bad: the tree holds a third copy of every schema file.
- Neutral: the interface, crew's implementation and the runner's own carrying of a hold do not change.
- This record amends the details of core.42 and core.59, and adds to record 0002's amendment of 2026-11-14 that version 3 is the later version it named.
