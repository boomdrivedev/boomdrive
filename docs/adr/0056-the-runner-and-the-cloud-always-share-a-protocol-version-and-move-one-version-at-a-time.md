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

Status: accepted by the user on 2026-11-19, on their card. Point 4 amended on 2026-11-19, below: a runner that speaks several versions tries one version at a time.

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
4. **To speak a version is to accept and send it** (boomdrive-design's). A side speaks a version when its code checks the messages it is sent against that version's schema, sends messages of that version, and, on the runner's side, offers that version's subprotocol at its upgrade. When the runner speaks several versions, it tries them one at a time, newest first, as the amendment of 2026-11-19 below says. Each side names the versions it speaks in one file of data that its own code and the other side's gate both read. On the cloud's side that file is `workers/control-plane/runner-protocol/pin.json`, its versions marked spoken. On the runner's side it is a file beside the version directories in `crates/boomdrive-hosted/protocol/`. Neither is ever a Markdown page (principle 16).
5. **Where each gate reads the other side** (boomdrive-design's). The runner's gate reads the cloud's file from boomdrive-cloud's `main` as GitHub holds it. The cloud's gate reads the runner's file at the runner's newest release tag. Each check reads something outside its own tree, so it is never replayed from a cache, as a task that reads the host's tools is not. A check that cannot reach the other side fails, naming what it could not read, and passes nothing.

### Not changed

A version's files never change once a release has spoken it, and its directory is never removed (core.42). A change to the protocol is still a new version, and speaking a version still means checking every message against it.

## Consequences

- Good: no runner release can leave the cloud unreachable, and no cloud change can shut out the runners in the field. A missing step is refused at the merge, naming the version, rather than found by a host that cannot connect.
- Bad: each side's gate now reads the other repository's state, so a change may wait on the other side's step before it can land.
- Neutral: core.42 is amended with this record. Record 0052's "the runner speaks version 3 alone" and record 0054's point 3 are amended in place. The README's *Versions* now says the runner speaks the versions this record allows. boomdrive-cloud amends its core.36 and core.106 to this record.

## Amendment, 2026-11-19: a runner that speaks several versions tries one version at a time

Status: boomdrive-design's, on the question boomdrive-unstick asked before version 5 lands, relayed by the user on 2026-11-19. Each choice in it is cheap to change.

### Context

Every statement a key signs is signed over a context that starts with the subprotocol of its version (records 0053 and 0054). The runner signs its upgrade's statement before the control plane picks a version, so a statement signed for one version is refused under another. Enrollment, `POST runner/enroll`, and a forge token's ask, `POST runner/forge-token`, are plain requests with no upgrade at which to agree a version. And `boomdrive forge-token` runs as a process of its own that reaches no running runner (core.20), so it cannot learn the version a socket agreed.

boomdrive-cloud's control plane, at main 05fb3b6 (`workers/control-plane/src/seam/runner.ts`, `runner-protocol/index.ts`):

- At `runner/connect` it takes the newest offered version it speaks, answers 400 with no body where it speaks none of them, and checks the statement over that version's context, answering 401 where the check fails.
- At `runner/enroll` and `runner/forge-token` it reads the version from the body's `protocol_version`. It answers 400 with no body where it does not speak that version. Otherwise it checks the message and its statement against that version and answers in that version, whatever the status.

### Considered options

1. **One version for each attempt, newest first (chosen).** Each upgrade offers one version, and each request is written in one version. The statement is signed over that version's context. Where the control plane answers that it speaks none of what it was given, the runner tries again in the next older version it speaks.
2. **The upgrade offers every version, and the statement signs over the whole offer.** The control plane would check the statement over the offer as it received it, and then pick a version. A control plane that speaks version 4 alone checks over version 4's context, so a runner that adds version 5 beside version 4 could not reach it. That breaks point 1 at the step point 3 orders first.
3. **The runner keeps the version its last connection agreed, and its plain requests use it.** Nothing outside the running runner may read what it keeps (core.20, core.35), and enrollment comes before any connection.

### Decision outcome

Option 1.

- **One version for each attempt.** An upgrade offers the subprotocol of one version, and a plain request's body names one version. The statement of either is signed over a context that starts with that version's subprotocol. So the context always names the version agreed, and the control plane checks it over the version it took.
- **Newest first, stepping down only on a version not taken.** The runner tries the versions it speaks newest first. It tries the next older one only where the control plane answers 400 with no subprotocol at an upgrade, or 400 with no message to a request. Any other answer is final:
  - a message in the version the request was written in, whatever its status;
  - a 401, a statement not taken;
  - no answer within the time allowed.
  A 400 with no message for the oldest version the runner speaks refuses, naming the path, the status and each version tried.
- **Nothing is kept between attempts.** Each connection, enrollment and ask finds its version again, so no process reads what another wrote. While the control plane is a version behind, each costs one more round trip. Point 3 keeps that to one version and one step.
- **The time allowed is the whole.** A forge token's ask, every version tried and every partition asked, stops at twenty seconds together (record 0053).

The control plane changes nothing. It already answers a version it does not take with 400 and no body, checks a statement over the version it took, and answers a request in the version the request names.

### Consequences

- Good: a runner that speaks versions 4 and 5 reaches a control plane that speaks version 4 alone, on every path, which point 1 requires.
- Good: a statement is never signed for a version other than the one it is checked under, and a statement for one version is never taken under another.
- Bad: while the control plane is a version behind the runner, each upgrade, enrollment and ask makes one request that is refused before the one taken.
- Neutral: record 0053's rule that an answer which is no message of version 4 refuses the ask now reads as this amendment says. A 400 with no message steps down to the next version the runner speaks, and refuses only for the oldest.
