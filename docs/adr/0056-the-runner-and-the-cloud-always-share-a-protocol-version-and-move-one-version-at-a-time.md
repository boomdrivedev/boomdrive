---
number: 56
title: The runner and the cloud always share a protocol version, and move one version at a time
status: accepted
date: 2026-11-19
decision-makers:
- Chuck Swanberg
- boomdrive-design
---

# The runner and the cloud always share a protocol version, and move one version at a time

Status: accepted by the user on 2026-11-19, on their card.

The runner and Boomdrive Cloud always have at least one protocol version they both speak. Each one's merge checks refuse a change that would leave none in common. A side stops speaking a version only once the other has moved past it, and a new version opens only once the other side speaks the one before it. So the two move a version at a time, and a released runner can always reach the cloud.

Deciders: the user, for points 1 to 3; boomdrive-design for points 4 and 5, each cheap to change.

Sources: records 0015, 0037, 0052, 0053 and 0054; rule core.42; the protocol's README (`crates/boomdrive-hosted/protocol/README.md`, *Versions*); boomdrivedev/boomdrive-cloud's rules core.36 and core.106 and its pin, `workers/control-plane/runner-protocol/pin.json`; design principle 16.

## Context and problem statement

Each side has moved its protocol on its own. The runner speaks its current version alone (records 0037 and 0052; the README's *Versions*), and a control plane accepts the current version and the one before (record 0015; the cloud's core.36). Nothing in either side's checks looks at the other. The runner's releases v0.21.0 to v0.21.3 speak version 4 alone, while the cloud's pin names versions 1 and 2. So no released runner can connect to the cloud, and neither gate noticed.

## Considered options

1. **Each side speaks its newest version alone, and the other catches up.** This is the rule today, and it is how the runner reached version 4 while the cloud still speaks versions 1 and 2.
2. **The runner speaks every version it ever spoke, for good.** It never strands a cloud, but the runner carries every old version's code forever, and nothing says when one may go.
3. **A version always shared, held by both sides' gates, moved one step at a time** (chosen).

## Decision outcome

1. **Always a version in common** (the user's). The runner's gate refuses a change after which the versions the runner speaks share none with the versions boomdrive-cloud's main pins as spoken. The cloud's gate refuses a change after which the versions it speaks share none with those the runner's newest release speaks.
2. **A version goes only once the other side has moved past it** (the user's). The runner stops speaking a version only once the cloud's main speaks a newer one. The cloud stops speaking a version only once the runner's newest release speaks a newer one.
3. **A new version opens only once the other side speaks the one before it** (the user's). The runner opens version N+1 only once the cloud's main speaks version N. The cloud takes version N+1 only once the runner's main holds that version's schema and the runner speaks version N. From today the order is:
   - the cloud speaks version 4;
   - the runner adds version 5 beside version 4;
   - the cloud adds version 5;
   - the runner may then drop version 4.
4. **To speak a version is to accept and send it** (boomdrive-design's). A side speaks a version when its code checks the messages it is sent against that version's schema, sends messages of that version, and, on the runner's side, offers that version's subprotocol at its upgrade. When the runner speaks several versions, its upgrade offers each of them, newest first, and the control plane picks the newest it speaks. Each side names the versions it speaks in one file of data that its own code and the other side's gate both read. On the cloud's side that file is `workers/control-plane/runner-protocol/pin.json`, its versions marked spoken. On the runner's side it is a file beside the version directories in `crates/boomdrive-hosted/protocol/`. Neither is ever a Markdown page (principle 16).
5. **Where each gate reads the other side** (boomdrive-design's). The runner's gate reads the cloud's file from boomdrive-cloud's `main` as GitHub holds it. The cloud's gate reads the runner's file at the runner's newest release tag. Each check reads something outside its own tree, so it is never replayed from a cache, as a task that reads the host's tools is not. A check that cannot reach the other side fails, naming what it could not read, and passes nothing.

### Not changed

A version's files never change once a release has spoken it, and its directory is never removed (core.42). A change to the protocol is still a new version, and speaking a version still means checking every message against it.

## Consequences

- Good: no runner release can leave the cloud unreachable, and no cloud change can shut out the runners in the field. A missing step is refused at the merge, naming the version, rather than found by a host that cannot connect.
- Bad: each side's gate now reads the other repository's state, so a change may wait on the other side's step before it can land.
- Neutral: core.42 is amended with this record. Record 0052's "the runner speaks version 3 alone" and record 0054's point 3 are amended in place. The README's *Versions* now says the runner speaks the versions this record allows. boomdrive-cloud amends its core.36 and core.106 to this record.
