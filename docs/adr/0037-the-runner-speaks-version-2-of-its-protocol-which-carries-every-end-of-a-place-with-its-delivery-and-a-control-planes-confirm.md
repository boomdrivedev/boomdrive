---
number: 37
title: The runner speaks version 2 of its protocol, which carries every end of a place with its delivery and a control plane's confirm
status: proposed
date: 2026-11-09
decision-makers:
- Chuck Swanberg
---

# The runner speaks version 2 of its protocol, which carries every end of a place with its delivery and a control plane's confirm

Status: proposed on 2026-11-09 by the construct stage of the change `the-hosted-protocol-carries-every-end-of-a-place`, on the bolt `the-hosted-protocol-carries-every-end`. The direction is the change's brief. How version 2 meets it is the stage's own choice, made on this repository's rules. It is proposed until the user has seen it as a record, and accepted on their yes. Once a release of the runner speaks version 2, version 2 never changes (core.42), so its choices are cheap to change only until then. Amended on 2026-11-09, below: version 2 carries a land and a preview, with what the runner answered of each preview, so it carries everything crew's control plane can ask.

Deciders: the change's brief, for the direction; the construct stage, for the rest.
Sources: the change's brief, 2026-11-09: "A runner under the hosted control plane reports every end Flume gives a place, a drop and a done among them, each with its delivery's key, and carries the hosted control plane's confirm of a reader's ends as it carries crew's, in one protocol version that names them both, and a place whose end a version does not name never empties the rest of the host's status"; record 0015; agentplot/weir record 0006, accepted by the user on 2026-11-08; records 0002 and 0030 as amended on 2026-11-09, each accepted by the user that day; Flume `v0.21.0` (`77d5b93`), its `crates/flume-core/src/answer.rs` (`EndStatus`) and `crates/flume-seams/src/held.rs` (`EndOf`, `Over`, `Delivery`, `DeliveredBy`, `OwedTo`, `ConfirmedBy`); rules core.2, core.11, core.12, core.27, core.42, core.54 and core.56.

## Context and problem statement

Version 1 of the runner's protocol copies a place's status as Flume 0.4.1 printed it, and every release of the runner from `v0.1.0` to `v0.7.3` has spoken it. So it never changes (core.42).

Under Flume `v0.21.0`, the least the runner runs with, a place's status gives three things version 1 does not name:

- a drop's or a done's end, `{"of":"over","over":"dropped"|"done",…}`;
- an end handed over at a session's start, a hand-over `by` `session_start`;
- each end's delivery: its `key`, its delivery's key `delivery`, `expires_at`, the readers it is `owed` to and those that `confirmed` it.

Every object of version 1 is closed, so the third is cut away. The first two fail the copy's check, and the place's whole status goes up `null`. The control plane loses that place's phase, stages, submission and agent along with the one end. The runner confirms an end owed to it only once its control plane has taken the status holding it (record 0030 as amended), and a status that went up `null` is not taken. So every end of such a place stays owed in Flume, and its tidied record stays on the host.

Version 1 also names no confirm. A hosted control plane cannot ask a reader's ends confirmed, as crew's can (core.56), and it has no delivery key to name.

weir's record 0006 has a reader confirm what it acted on, and has an owed message that expires shown with its reader wherever the tool shows what is owed. The control plane can do neither without each end's delivery.

## Considered options

For what carries them:

1. **Version 2, beside version 1 (chosen).**
2. **Version 1 changed to carry them.** A release has spoken it, so core.42 rules it out.
3. **Each end carried whole as Flume printed it.** It would carry a field because Flume printed it, against core.27, and an agent's stop-short words and a hold's words with it.

For an end of a form a version does not name:

- a. the place's whole status goes up `null`, as under version 1;
- b. the end is left out, and the rest of the status goes up;
- c. **the end goes up as its delivery alone, or where even that fails is left out, and the rest of the status goes up (chosen).**

For which statuses the answer to a report names taken:

- **each that went up whole, every end of it in a form the version names (chosen);**
- each that went up, an end carried as its delivery alone among them.

For which versions the runner speaks:

- **version 2 alone (chosen);**
- version 2 and version 1, both offered at the upgrade, the runner speaking the one the control plane picks.

For the signatures' contexts: **named by the version spoken, `boomdrive.v2` (chosen)**; or `boomdrive.v1` kept in every version.

## Decision outcome

Option 1, with c, the statuses that went up whole, version 2 alone, and contexts named by the version spoken.

- **Version 2 is version 1 whole, at `crates/boomdrive-hosted/protocol/v2/`, but for these:**
  - every file's `$id` is under `v2/`, and every message's `protocol_version` is 2;
  - a place's status copy names an end of the place's work, `{"of":"over","over":…}`, beside a stage's, a submission's and a stall's;
  - every end names its `key`, its `delivery` where Flume gives one, its `expires_at` where Flume gives one, each reader it is `owed` to with whether it has `expired` for that reader and whether that reader is `known`, and each reader that `confirmed` it with when;
  - a hand-over is `by` `notice`, `wait`, `inbox` or `session_start`, and its definition is named `handover`, since `delivery` now names the delivery's key (core.12);
  - an end's delivery alone, `end_delivery`, is a form of an end: its `of` as Flume printed it, `key`, `read_at`, `delivery`, `expires_at`, `owed`, `confirmed` and `pending_for`, and nothing of what ended or how it was handed over;
  - an act may be a confirm, `{"confirm":{"recipient":…,"deliveries":[…]}}`, as the interface writes it (core.56).
  Nothing else changes. Version 2 names no land and no preview, which wait for a version that names them (core.53, core.55).
- **An end is copied on its own.** Each end of a place's status is cut by the form version 2 names for it and checked. One that fails is cut to its delivery alone and checked again. One that fails that too is left out. Either is said in one line while it lasts, naming the launch and the end's path, never a value. The rest of the status goes up as its copy. A status still goes up `null` where something other than an end fails, such as a stage's state version 2 does not have.
- **A status is taken only where it went up whole.** The answer to a report names taken each launch whose status went up as its copy with every end in a form version 2 names. An end carried as its delivery alone, or left out, holds the status back, so each end of it owed to the runner stays owed in Flume (record 0030 as amended). The control plane then has an end's delivery but not what ended, and the runner does not let Flume's copy of it go.
- **The runner speaks version 2 alone.** Its upgrade asks for `boomdrive.v2`, and its enrollment's `enroll` is a message of version 2. A control plane accepts the current version and the one before (the protocol's README; record 0015), so the compatibility is the control plane's end. A control plane that speaks version 1 alone refuses the upgrade, which the runner says once and tries again at its backoff's cap, as any subprotocol refused (core.22, core.23). Keeping version 1's types beside version 2's would double the runner's message types for a window the control plane's pin already covers.
- **Each context is named by the version spoken.** The contexts a key signs start `boomdrive.v2`, as version 1's start `boomdrive.v1`: each version's name is its subprotocol and the start of its contexts, as the code already makes them. A key enrolled under version 1 stays the partition's, and nothing is enrolled again.
- **Version 1 stays as a release spoke it.** Its directory and examples stay, each example still passing version 1, and a test holds each of its files to its digest at `v0.7.3`. The runner's message types move to version 2's module, and version 1's keeps its files alone.
- **A control plane's confirm is carried as crew's.** A `pulled` whose act is a confirm passes version 2 and is given to the runner as any act, under the partition's prefix. The runner carries it out as core.56 says, whatever the implementation (core.2). Nothing of the hosted implementation reads it.

### Consequences

- Good: a hosted control plane reads every end Flume 0.21.0 gives a place from the runner's report, a drop's and a done's among them, each with its delivery, and can show each expired end with its reader (weir 0006).
- Good: a hosted control plane confirms a reader's ends by their keys, as crew does.
- Good: a place whose end version 2 does not name keeps the rest of its status in the report.
- Good: a tidied record held for the runner is taken and let go under a hosted control plane, which version 1 could not do for a drop or a done.
- Bad: a control plane that speaks version 1 alone cannot be reached by a release that carries this record until it takes version 2. boomdrive-cloud pins version 1 today, so it takes version 2 by a change of its own.
- Bad: a status with an end carried as its delivery alone is never taken while that end stands, so every end of that place stays owed, as under version 1, until a version names the end or a person confirms it.
- Bad: version 2 is fixed from its first release, so a land, a preview, or any field the cloud wants next is a version 3.
- Bad: the tree holds a second copy of every schema file.
- Neutral: the interface, crew's implementation and the runner's own passes do not change.
- This record amends the details of core.42 and core.27.

## Amendment, 2026-11-09: version 2 carries a land and a preview

Status: proposed on 2026-11-09 by the construct stage of the change `the-hosted-protocol-carries-land-and-preview`, on a branch of the bolt `the-hosted-protocol-carries-every-end`, until the user sees it as a record. The direction is the change's brief: "Version 2 of the runner's protocol with a hosted control plane carries, before it is first released, everything crew's control plane can ask of the runner: launches, acts, a work's land, a preview with its done, and a confirm, beside every end with its delivery key, so a hosted control plane asks what crew's can and no later version is needed for them." How version 2 carries them is the stage's own choice, made on this repository's rules, and cheap to change until a release speaks version 2: the preview's revision left unbounded, `previews` required in every report, the copies of sluice's answers in a file of their own, and a preview's copy given back as its text.

Sources: the change's brief; record 0015; records 0002 as amended on 2026-11-08 and 0031; rules core.2, core.11, core.12, core.26, core.27, core.42, core.53 and core.55; the control-plane-interface spec; sluice 0.74.3's answers of a preview, the binary's fixtures.

### Context

The interface names five wants, an offer, an act, a command, an inbox and a preview, and six acts, a retry, a land, a confirm, a drop, a priority and a done. crew's implementation gives every one. Version 2, as decided above, names every one but a land and a preview, and its report has no previews, so a hosted control plane lands a work only by a retry naming the submission, and brings up no preview.

No release has spoken version 2: the newest tag, `v0.7.3`, holds `protocol/v1/` alone. So version 2 may still change (core.42). Once its bolt lands on `main`, a release follows (record 0024) and fixes it, and a land or a preview would wait for a version 3.

### Considered options

1. **Version 2 carries them (chosen).**
2. **A version 3 carries them**, after version 2's release. The runner and the cloud would keep three directories for one release's worth of difference.

For a preview's revision: **any string (chosen)**; or a full commit, as `shared.schema.json`'s `commit`.

For `previews` in a report: **required, `{}` where the runner answered none (chosen)**; or left out where empty, as the interface's file writes it.

For what of sluice's answers goes up: **their copies, in `preview.schema.json` (chosen)**; or the runner's text carried whole as a string, as a command's `text` is.

For a preview in a report the control plane hands back: **its copy's JSON as its text, and no status (chosen)**; or no text, as a launch's status and entry are given back none.

### Decision outcome

Option 1, with each first choice.

- **A land is an act of version 2**, written `"land"`, as the interface writes `ActSort::Land`, holding nothing but its act's launch and lease. The retry naming the submission stays.
- **A preview is a want of version 2**, `{"preview":{"id","lease","repository","branch","revision","done"}}`, as the interface writes `Want::Preview`. Its branch and revision are any string, so a revision sluice cannot bring up reaches the runner and is answered with sluice's refusal, where a bound would refuse the whole `pulled`.
- **Every report of version 2 holds `previews`**, by id, each with its lease, repository, revision, done, outcome (`held`, `done` or `refused`), `copy`, words, `status` and time. `copy` is the copy of sluice's answer, a preview's or a release's, or `null`; it is the interface's `text`, named for what it holds (core.12). `status` is `{copy, at, listed}`, the copy of sluice's listing of the preview, or `null`. A report is whole, so the part is there where it is empty.
- **The copies of sluice's answers are in `preview.schema.json`**: a preview's `name`, `branch`, `revision`, `state`, `made` and `addresses`, and a release's `preview` and `released`, each as sluice printed it, which are the fields the runner already carries (core.55). No copy holds a preview's `checkout`, `names`, `ports`, `exit`, `output` or `release_failed` (core.11, core.27). A copy that fails goes up `null`, said once.
- **The hosted implementation gives a land and a preview as any want**, under the partition's prefix, withholding a preview of a repository the partition is not served (core.26), and reads nothing else of either. The runner carries each out as it carries crew's (core.2, core.53, core.55).
- **A report handed back gives each preview its copy's JSON as its text**, and no status, so a done after the runner lost its report file puts away the preview by the name sluice gave it.
- **A test holds every sort the interface names to version 2**, by a match with no wildcard, so a sort added to the interface fails it until a version names it.
- **Version 1 is not changed**, and carries no land, no preview and no confirm (core.42).

This replaces the sentence above, "Version 2 names no land and no preview, which wait for a version that names them (core.53, core.55)", and the consequence "a land, a preview, or any field the cloud wants next is a version 3", which holds now of a field the cloud wants next alone.

### Consequences

- Good: a hosted control plane asks of the runner everything crew's control plane can, from version 2's first release.
- Good: a hosted control plane reads a preview's addresses and state from the runner's report, as crew reads them (record 0031).
- Bad: boomdrive-cloud takes three more shapes with version 2, which its own change carries.
- Bad: every report of version 2 carries `"previews": {}` where the runner answered no preview.
- Neutral: the runner's own carrying of a land and a preview, the interface and crew's implementation do not change.
- This amendment amends the details of core.27, core.42, core.53 and core.55.
