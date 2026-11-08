---
number: 30
title: Every launch names the runner its caller, so Flume holds each place's end until the runner has carried it and collected it
status: proposed
date: 2026-11-08
decision-makers:
- agentplot-design
---

# Every launch names the runner its caller, so Flume holds each place's end until the runner has carried it and collected it

Status: proposed on 2026-11-08 by the construct stage of the change `the-runner-carries-each-places-end`. The direction is agentplot-design's answer that day to the stage's question, quoted below. How the runner meets it is the stage's choice, made on this repository's rules and Flume's statement, and cheap to change: the caller's name, when the runner collects, what it says, and the least Flume it runs with. It waits on a Flume unit that holds a place's end for its caller, which agentplot-planner places ahead of this change.

Deciders: agentplot-design, for the direction; the construct stage, for how it is met.
Sources: the change's brief, 2026-11-08; agentplot-design's answer of 2026-11-08; Flume's statement, clauses 7, 43, 46, 62, 63, 132 and 208, and Flume's main at `81d57226` (`settle.rs`, `ends.rs`, `wait.rs`, `launch.rs`, `acts.rs`); crew records 0056 and 0062, and crew's `roles.9`; records 0002, 0010, 0021, 0027 and 0029; rules core.1, core.10, core.11, core.13, core.15 and core.38; the work-into-flume and clones specs.

## Context and problem statement

The brief: "The runner carries each place's end, landed, failed, refused or dropped, to its control plane's report before Flume tidies the place, so a control plane reads every piece's end from the runner's report and never from another tool."

Today the runner can't. One settle of a place reads its submission landed, makes its work over, tidies it and delivers its ends (Flume's 46; `settle_place`). Flume then keeps the place's document only while an end of it is pending for a recipient (62), and removes it outright otherwise. The runner reads Flume between settles, so a place that lands and is tidied inside one settle shows the runner nothing in between. Its next `flume status <place>` is refused, and the status it last carried stands, not listed (record 0027's "Not cured").

On 2026-11-08 a host's runner carried a fix's place live, its submission under test, at 04:03Z. At about 04:18Z one settle landed and tidied it. The runner's report also holds sluice's entry of that submission, landed, but crew's implementation writes only the status into crew's records. So crew read the fix as submitted, and its bolt's landing waited until the planner dropped the fix by hand. The same day, a unit of another bolt landed and was tidied with no status of its place ever carried. crew refused to close that bolt. crew record 0062 keeps a stopgap until this is mended: three readings of GitHub's landing commits, against crew record 0056, under which crew reads Flume and sluice only through the runner's report. The runner's own After reads a place landed only from Flume's status too, so an offer after a place tidied this way stays held.

Flume's statement has the pieces of a cure. Its 62 leaves "whatever else outlives a place" to "its caller's to keep, from what flume reported". Its 43 has every end, a submission's outcome with the commit it landed as among them, delivered to the place's recipients, so that no caller reads a terminal or another tool to learn it. It also has a caller collect, by `flume inbox`, "every end pending for it, of the places it launched or is recipient of". The gap is that the runner launches with no recipient of its own (the work-into-flume spec's "Flume runs with no recipient of the runner's own"), so nothing of a place is pending for the runner.

## Considered options

1. **Flume holds a launched place's end for its caller until the caller collects it.** The runner names itself the caller of every launch. It reads the record Flume holds, `<location>@<at>`, as it reads any tidied record now, carries it, and collects it once its report holding the record is saved.
2. **Flume's settle and act answers name the status of each place they tidy**, and the runner carries those from its own commands.
3. **No Flume change.** The landing is sluice's entry, which the runner already carries, written by crew's implementation beside the status, and a drop or a done is the act's own answer.

For when the runner collects: once the report that holds the record is saved; once the control plane has taken a report that holds it; or never, Flume letting the record go on a bound of time.

For what holds the end: the runner named as one of the place's recipients, by `--recipient`; or a caller named apart from its recipients.

## Decision outcome

Option 1, collected once the report is saved, and a caller named apart from the place's recipients. agentplot-design, 2026-11-08: "Option (1). What must be true: the end of every place the runner launched, landed, failed, refused or dropped, stays held by Flume for the runner until the runner has carried it into its report and collected it, so no tidy loses an end; the runner reads it from Flume as P@<at> and collects it once the report is saved." And: "Make the runner the caller its launches' ends are pending for, collected by flume inbox rather than shown in a pane, and amend that requirement in this change."

- **Every launch names the runner its caller.** Each `flume launch` the runner runs ends `--caller boomdrive`. The name is fixed and the same on every host, since one runner serves a host. The caller is not a recipient. No notice of the place goes to it, nothing is woken for it, and the launch's recipients are the ones the control plane gave, or Flume's default where it gave none (core.15).
- **Flume holds the place's record for the runner.** A place whose launch names a caller keeps its document after its tidy, as `<location>@<at>`, whatever ended its work: a landing, a drop or a done. It keeps it until that caller collects it, and its status names the caller among those the record is kept for. This is the Flume unit's part.
- **The runner carries the held record as the place's status.** It is carried as any place's status is: `flume --json status --repository <container> <place>`'s whole answer, under the launch's lease number, read again where the place's entry in Flume's listing changes (record 0027). So a control plane reads every place's end from the runner's report, whatever tidied the place. A landing, a drop and a done end the place's work, and read as the held record's `over`. A failed or a refused submission sends the place back, and is in its status while it stands. A refused launch is the launch's answer.
- **The runner collects a held record once its report holds it.** In the control pass that carries it, once the report file holds the record, the runner runs `flume --json inbox --caller boomdrive <location>@<at>`, naming no repository. It does so only for a launch the pull gives, whose clone stands, and whose carried status is listed and names `boomdrive` among those its record is kept for. It reads the answer only by Flume's exit, and carries and keeps nothing of it (core.11). Collected, the place is read at the next pass as any place: Flume gives the record kept for a recipient, or refuses it, and the last status carried stands, not listed. Where Flume refuses the collection or cannot be run, the runner says one line and collects again at the next pass.
- **Nothing is collected for a launch whose lease the control plane refused**, which the runner acts on no more and removes nothing of (core.10). Its record stays on the host as Flume holds it.
- **A drained repository's clone waits for its place's end to be collected.** For the drain, a place has not ended while its carried record is kept for the runner (record 0021).
- **The least Flume.** The runner names the least Flume it runs with: the first release that holds a place's end for its caller. Where `flume --version`, as the hello reads it, is older than that or names no version, no offer is launched. Each offer is left unanswered, or held where it was held, and one line says so until Flume's version changes. Acts and statuses go on, and such a Flume holds nothing to collect (core.38).

Option 2 misses a place tidied outside the runner's own commands: in an agent's own settle, as a role's run is declared done (132), or by a person's command. Option 3 misses a place tidied before its submission was read, a drop by hand, and the runner's own After, and it has the runner or its control plane decide a landing from sluice's entry where Flume decides it (63; core.1).

Collecting once the control plane has taken the report would hold a record across an unreachable control plane for no gain. The saved report is the one sent at every later pass until one is taken, and resent whole after a reconnect (core.21). A report that cannot be saved collects nothing, as it carries out nothing more (core.25). A bound of time would lose an end whenever the runner is down longer than the bound.

Naming the runner by `--recipient` would take the place of Flume's default recipient where a launch names none. It would also have Flume send a notice into any session named `boomdrive`. Both change who the place's ends go to, which core.15 rules out.

### Consequences

- Good: no tidy loses the end of a place launched under this record, whatever tidied it. The runner's report holds it until the runner, its report saved, collects it.
- Good: the runner's After, the keeper's landing and the drain read a landing from the record Flume holds, as they read a tidied record now.
- Good: crew's stopgap of record 0062 can go once this runs on every host crew serves. That is crew's unit `crew-reads-each-pieces-end-from-the-runners-report`.
- Bad: a pass that carries a held record runs one more Flume command for it.
- Bad: a record held for a launch whose lease was refused, or one the control plane let go before the runner collected it, stays on the host until a person drops it.
- Bad: a host whose Flume is older than the least launches nothing until its Flume is upgraded. The hello names the version it runs.
- Neutral: a place launched before this record names no caller, so Flume holds nothing of it, and it is followed as before.
- This record adds core.53 and amends the details of core.1, core.13 and core.15. The work-into-flume, clones and family-packages specs say it, in this record's change.
- Outside this repository: Flume's unit makes a launch's caller, holds a place's record for it past its tidy, names the caller in the record's status, and collects one place for it by `flume inbox`. Then it releases. agentplot-planner places it ahead of this change.
