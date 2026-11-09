---
number: 30
title: The runner is one more recipient of each place it launches, so Flume holds its end until the runner has carried it and collected it
status: proposed
date: 2026-11-08
decision-makers:
- Chuck Swanberg
- agentplot-design
---

# The runner is one more recipient of each place it launches, so Flume holds its end until the runner has carried it and collected it

Status: proposed on 2026-11-08 by the construct stage of the change `the-runner-carries-each-places-end`, and written as it stands by that change's code stage the same day, on the user's answer on agentplot-design's card. It is proposed until the user has seen it as a record, and accepted on their yes. How the runner meets the decision is the stages' choice, made on this repository's rules and Flume's statement, and cheap to change: the runner's name, where it names itself, when it collects, what it says, and the least Flume it runs with.

Deciders: the user, whose answer on agentplot-design's card was "Runner as one more recipient now; one shared inbox in weir next."; agentplot-design, for the direction and for how the runner carries the answer: "the runner passes --recipient <its own name> beside the role's, and collects with flume inbox <place> --recipient <its own name> only after its report is saved"; the code stage, for the rest.
Sources: the change's brief, 2026-11-08; agentplot-design's answers of 2026-11-08; Flume's statement, clauses 7, 43, 46, 62, 63, 132 and 208; Flume record 0060 and Flume `v0.20.0` (`9ab0e2c`), which carries the bolt `a-tidied-places-end-stays-readable` (`72107a2`): `launch.rs`'s `recipients_of`, `wait.rs`'s `inbox`, `ends.rs` and the command line's `launch --recipient` and `inbox [<place>] --recipient`; crew records 0056 and 0062, and crew's `roles.9`; records 0002, 0010, 0021, 0027 and 0029; rules core.1, core.10, core.11, core.13, core.15 and core.38; the work-into-flume and clones specs.

## Context and problem statement

The brief: "The runner carries each place's end, landed, failed, refused or dropped, to its control plane's report before Flume tidies the place, so a control plane reads every piece's end from the runner's report and never from another tool."

Without this record the runner can't. One settle of a place reads its submission landed, makes its work over, tidies it and delivers its ends (Flume's 46; `settle_place`). Flume then keeps the place's document only while an end of it is pending for a recipient (62), and removes it outright otherwise. The runner reads Flume between settles, so a place that lands and is tidied inside one settle shows the runner nothing in between. Its next `flume status <place>` is refused, and the status it last carried stands, not listed (record 0027's "Not cured").

On 2026-11-08 a host's runner carried a fix's place live, its submission under test, at 04:03Z. At about 04:18Z one settle landed and tidied it. The runner's report also holds sluice's entry of that submission, landed, but crew's implementation writes only the status into crew's records. So crew read the fix as submitted, and its bolt's landing waited until the planner dropped the fix by hand. The same day, a unit of another bolt landed and was tidied with no status of its place ever carried. crew refused to close that bolt. crew record 0062 keeps a stopgap until this is mended: three readings of GitHub's landing commits, against crew record 0056, under which crew reads Flume and sluice only through the runner's report. The runner's own After reads a place landed only from Flume's status too, so an offer after a place tidied this way stays held.

Flume's statement has the pieces of a cure. Its 62 leaves "whatever else outlives a place" to "its caller's to keep, from what flume reported". Its 43 has every end, a submission's outcome with the commit it landed as among them, delivered to the place's recipients, so that no caller reads a terminal or another tool to learn it, and has a recipient collect by `flume inbox` every end pending for it. The gap is that the runner launches with no recipient of its own, so nothing of a place is pending for the runner. The role its launch names hears the landing by a notice into its pane, nothing stays pending, and the document goes inside the settle that read the landing.

## Considered options

1. **Flume holds a launched place's end for the runner until the runner collects it.** The runner reads the record Flume holds, `<location>@<at>`, as it reads any tidied record, carries it, and collects it once its report holding the record is saved.
2. **Flume's settle and act answers name the status of each place they tidy**, and the runner carries those from its own commands.
3. **No Flume change.** The landing is sluice's entry, which the runner already carries, written by crew's implementation beside the status, and a drop or a done is the act's own answer.

For whom Flume holds the end: the runner named one more recipient of the place, by `--recipient`, beside the recipients its launch names; or a caller named apart from the recipients, by a `--caller` of Flume's.

For when the runner collects: once the report that holds the record is saved; once the control plane has taken a report that holds it; or never, Flume letting the record go on a bound of time.

## Decision outcome

Option 1, the runner one more recipient, collected once the report is saved. agentplot-design, 2026-11-08: "Option (1). What must be true: the end of every place the runner launched, landed, failed, refused or dropped, stays held by Flume for the runner until the runner has carried it into its report and collected it, so no tidy loses an end; the runner reads it from Flume as P@<at> and collects it once the report is saved." The user, on agentplot-design's card the same day: "Runner as one more recipient now; one shared inbox in weir next."

- **The runner is one more recipient.** Each `flume launch` the runner runs for a launch that names recipients names `--recipient boomdrive` after them. The name is fixed and the same on every host, since one runner serves a host, and no pane holds it, so Flume sends it no notice and wakes nothing for it (Flume record 0060). The launch's own recipients are passed as the control plane gave them, none left out.
- **A launch that names no recipient is passed with none.** Flume's default is then its recipient: `FLUME_RECIPIENT`, which the runner unsets, else the session the command runs in, else the answerer behind the place's stream (Flume's 7). Naming the runner alone would drop that default, since naming any recipient drops it, and the stream's answerer would hear none of the place's ends. So such a place is followed as before, and Flume holds nothing of it for the runner. This is the code stage's choice, cheap to change. It keeps agentplot-design's ground for the answer, that naming the runner beside the role "drops no default recipient".
- **Flume holds the place's record for the runner.** A place's ends stand pending for the runner until it collects them: each stage's and stall's, its submission's, and, under Flume `v0.20.0`, a drop's or a done's (Flume record 0060). Flume keeps the place's document after its tidy, as `<location>@<at>`, while an end of it stands pending, and its status names the runner in the tidied phase's `pending_for` (62).
- **The runner carries the held record as the place's status.** It is carried as any place's status is: `flume --json status --repository <container> <place>`'s whole answer, under the launch's lease number, read again where the place's entry in Flume's listing changes (record 0027). So a control plane reads each such place's end from the runner's report, whatever tidied the place. A landing, a drop and a done end the place's work, and read as the held record's `over`. A failed or a refused submission sends the place back, and is in its status while it stands. A refused launch is the launch's answer.
- **The runner collects a held record once its report holds it.** In the control pass that carries it, once the report file holds the record, the runner runs `flume --json inbox <location>@<at> --recipient boomdrive`, naming no repository. It does so only for a launch the pull gives, whose clone stands, and whose carried status is listed and reads the place tidied, its `pending_for` naming `boomdrive`. It reads the answer only by Flume's exit, and carries and keeps nothing of it (core.11). Collected, the place is read at the next pass as any place: Flume gives the record kept for another recipient, or refuses it, and the last status carried stands, not listed. Where Flume refuses the collection or cannot be run, the runner says one line and collects again at the next pass.
- **Nothing is collected for a launch whose lease the control plane refused**, which the runner acts on no more and removes nothing of (core.10). Its record stays on the host as Flume holds it.
- **A drained repository's clone waits for its place's end to be collected.** For the drain, a place has not ended while its carried record is pending for the runner (record 0021).
- **The least Flume is `0.20.0`**, the first release that holds a drop's and a done's end for a recipient no pane holds, and collects one place's record by `flume inbox <place>`. Where `flume --version`, as the hello reads it, is older than that or names no version, no offer is launched. Each offer is left unanswered, or held where it was held, and one line says so until Flume's version changes. Acts and statuses go on (core.38).

A caller apart from the recipients gives Flume two lists of whom an end is owed to, with two rules for counting it delivered, where one list serves: a name no pane holds is already a recipient no notice reaches. Flume record 0060 folds the caller into the recipients for that reason, and `flume launch` takes no `--caller`.

Option 2 misses a place tidied outside the runner's own commands: in an agent's own settle, as a role's run is declared done (132), or by a person's command. Option 3 misses a place tidied before its submission was read, a drop by hand, and the runner's own After, and it has the runner or its control plane decide a landing from sluice's entry where Flume decides it (63; core.1).

Collecting once the control plane has taken the report would hold a record across an unreachable control plane for no gain. The saved report is the one sent at every later pass until one is taken, and resent whole after a reconnect (core.21). A report that cannot be saved collects nothing, as it carries out nothing more (core.25). A bound of time would lose an end whenever the runner is down longer than the bound.

### Consequences

- Good: no tidy loses the end of a place launched under this record that names a recipient, whatever tidied it. The runner's report holds it until the runner, its report saved, collects it.
- Good: the runner's After, the keeper's landing and the drain read a landing from the record Flume holds, as they read a tidied record now.
- Good: crew names a role on every launch, so crew's stopgap of record 0062 can go once this runs on every host crew serves. That is crew's unit `crew-reads-each-pieces-end-from-the-runners-report`.
- Bad: a pass that carries a held record runs one more Flume command for it.
- Bad: the runner's name is owed every end of its places, each stage's and stall's too, and collects them all at once by `flume inbox <place>` once the place is tidied (Flume record 0060).
- Bad: a session named `boomdrive` in the terminal would be sent the notices of every place the runner launches.
- Bad: a record held for a launch whose lease was refused, or one the control plane let go before the runner collected it, stays on the host until a person drops it.
- Bad: a host whose Flume is older than `0.20.0` launches nothing until its Flume is upgraded. The hello names the version it runs.
- Bad: a launch that names no recipient keeps the gap, its end held for no one where Flume's default is a role. A control plane that wants every end in the runner's report names a recipient on every launch, as crew does.
- Bad: under Flume `0.20.0` a drop's or a done's end is in the place's status as an end `{"of":"over",…}`, which version 1 of the runner's protocol does not name. The hosted implementation's copy of such a status fails version 1's check and goes up null, and says so once. Carrying it to a hosted control plane is a version of the protocol of its own (core.42).
- Neutral: a place launched before this record names no recipient of the runner's, so Flume holds nothing of it for the runner, and it is followed as before.
- Neutral: one inbox shared across the family is weir's to design next, as the user's answer says. This record leaves it open.
- This record adds core.54 and amends the details of core.1, core.13 and core.15. The work-into-flume, clones and family-packages specs say it, in this record's change.
