# Design

## Context

See proposal.md for why. There are no area pages, so `docs/architecture/core.md` binds every path this change touches. The change relies on these rules:

- core.1: the runner decides nothing the control plane decides. A control plane's implementation may give work because it read the control plane's own rule from the control plane's own records;
- core.2: the control plane is reached through one interface, and only an implementation names crew;
- core.10: each piece of work is carried out once, across restarts, and the runner keeps each answer until its control plane lets it go;
- core.55: the runner brings up a preview only at its control plane's asking, and puts away, on its control plane's done, only a preview it asked sluice for.

It departs from no rule and changes none. It amends record 0031, proposed (Decision 5).

The control plane is reached through `boomdrive-control`'s `ControlPlane` interface alone (core.2). What is let go is named in a report's answer, `Reported::let_go`, by the implementation that serves the control plane. Every change here is in crew's implementation, `crates/boomdrive-crew`. The hosted implementation gives no preview under version 1 (core.42), and the runner's own code is untouched.

### What stands

- **The runner drops what is let go.** `carry.rs` `reported` calls `forget` for each id of `Reported::let_go`. `forget` drops it from the report, its preview among them, and says `<id>: let go by the control plane`. Nothing is run against it. The previews spec's "A preview's lease is kept as any lease is" says so, but only a refused lease has a scenario (`tests/previews.rs` `a_previews_lease_refused`).
- **crew's implementation gives every request it holds current.** `lib.rs` `Pulling::current`: a request is given while `Main::plans` holds its work, or while the runner's report or this host's branch answers it (`reported`, `branch_answers`). A drop is the one exception. A preview request is given by that rule today.
- **`Main::plans` reads three kinds of work as leaving the plan** (`read.rs`): `unit/<u>` and `chore/<c>` with their `Unit` record, and `fix/<b>/<n>` with its `Bolt` record. Any other work, `bolt/<b>` among it, never leaves.
- **crew's implementation lets a preview go** once its work has left the plan and the preview is not up (`write.rs` `let_go`). A preview is up where it is not marked done, was answered `done` or `held`, and sluice still lists it.
- **What crew reads of its previews** (crew's design of `a-cards-preview-is-asked-of-the-runner`, Decisions 2, 3 and 7, on crew's branch `fix/crew-asks-the-runner-for-previews--takes-main-in-1`):
  - crew keys its asks by `(Host, Repo, Revision)` as written in its requests. Its ask stands while its newest preview request of those three is a bring-up the runner has not refused (`runner.py` `previews_asked`, `previews_standing`). A done ends the ask.
  - crew reads a bring-up's answer from `previews.rec` only while the bring-up is its ask. It never reads a done's answer.
  - crew writes a done only for an ask that stands, with the ask's own `Host`, `Work`, `Repo` and `Revision` (`plan.py` `preview_written`). It writes one when the user is done with a preview, and when a row or card ends: a landings row whose bolt landed, was dropped or moved its head, and a batch card's close. These often come after the bolt has left the plan.
  - A landings row's preview is asked for as `bolt/<b>`, and a batch line's as `unit/<u>`.
- **`bolt/<b>` is also a base place's work** on crew's `main`, read on 2026-11-09 (`runner.py` `WORK`; crew's `bolt-teams` spec). A bolt's base place is a launch of `bolt/<b>`, and its land an act on that launch. Once crew reads the base place landed, it closes the bolt, taking the bolt and its work out of the plan (crew's `bolt-plan` spec, "A landed bolt leaves the plan with its units"). Since `Main::plans` never reads `bolt/<b>` as leaving, a landed bolt's base place stays on the runner's branch too.
  - crew's design's *Risks* name "A landed bolt's previews stay on the runner's branch" and its *Follow-ups* ask this repository to read `bolt/<b>` as leaving with its `Bolt` record.

## Goals / Non-Goals

**Goals:**
- A preview, and its done, leave the runner's report and branch once the done is answered.
- A preview refused, or one sluice lists no more, leaves once its work has landed or been dropped, a bolt's as a unit's.
- A preview let go is never given again while crew no longer asks for it, so it is never brought up or put away twice.
- A landed or dropped bolt's base place leaves as a unit's place does, by the same reading of the plan.

**Non-Goals:**
- The runner's carrying of a preview, sluice's commands, the report's shape and `previews.rec`'s descriptor, all of which stand.
- A preview in the hosted protocol, which a later version carries (core.42).
- Previews crew asked for before this change, put away and still on a branch. They leave at the first report after this change, as Decision 1 has it.

## Decisions

### 1. A done answered is let go with the bring-ups it put away

`write.rs` `let_go` gains a first sweep of the report's previews:
- each preview marked done that the report holds, of a request `main` names, is gone, whatever the plan holds. The runner never holds a done `held`, so a done in the report is answered;
- with it goes each preview not marked done that the report holds, of a request `main` names, whose request names the same `Host`, `Repo` and `Revision` as the done's, as written, and whose number is below the done's.

The runner's done puts away every preview of its repository and revision that the report holds. So the bring-ups written before the done are what it put away, or what it found refused or already gone. Either way crew no longer asks for them, as its newest request of those three is the done.

A bring-up written after the done stays. It is crew's new ask, which a later done puts away.

Not taken:
- **Keeping the done until its work leaves the plan**, letting go only the bring-ups. The done is never asked again (Decision 2), and keeping it would leave a bolt's dones on its branch for the bolt's life.
- **The runner dropping a preview it put away of its own accord.** What leaves the report is the control plane's to name (core.1, core.10). The runner keeps each answer until then, so it answers each want once.
- **Matching a bring-up by its resolved repository**, as the runner's done does. crew keys its asks by `Repo` as written, and writes a done with its ask's own `Repo`. Matching as crew does keeps the implementation's reading crew's.

### 2. A preview request is given while crew still asks for it, or while the runner holds it

`Pulling::current` gains a preview request's own rule, for a preview request whose `read::want` reads:
- **held**: where `reported` or `branch_answers` holds it, it is given, as today;
- **not held**: it is given only while it is the newest preview request that reads, of `main`'s requests naming its `Host`, `Repo` and `Revision`. Then:
  - a bring-up is given only while `Main::plans` holds its work;
  - a done is given whatever the plan holds, but only while the runner holds a bring-up naming its `Host`, `Repo` and `Revision`, by `reported` or `branch_answers`.

A preview request that does not read is given as before, as one the runner refuses.

So:
- **A bring-up and its done, once let go, are given no more.** The bring-up is not the newest. The done is, but the runner holds no bring-up of it, since both went at one report (Decision 1).
- **A bring-up and its done that the runner has not answered bring up nothing.** This happens where crew puts a preview away before the runner's next pass. The bring-up is not the newest, and the runner holds no bring-up for the done. Neither is answered, and crew reads no ask of them.
- **A done of a landed bolt's preview is given,** though the bolt has left the plan, while the runner holds what it puts away. crew writes such a done once a row ends, and without it the preview would stay up with nothing to put it away.
- **A done written before a later bring-up is given no more.** A done puts away every preview of its repository and revision the report holds, so it would put away the later bring-up, crew's ask.

Not taken:
- **A done given whatever came after it**, for the reason just given.
- **A done given while it is the newest, whatever the runner holds.** Once both are let go, the done would be given at every pass. Each pass would refuse it, finding nothing to put away, and let it go again.
- **A bring-up given while its work is in the plan, as today.** Once let go, it would be given again and brought up again, though crew put it away.
- **Rules of the runner's own over which want to answer.** Which work is given is the control plane's to say (core.1). crew's rule, that its newest request of a host, repository and revision is its ask, is crew's own, read from crew's records.

### 3. A bolt's work leaves the plan with its `Bolt` record

`Main::plans` reads `bolt/<b>` as in the plan while `plan.rec` holds `Bolt: <b>`, as it reads `fix/<b>/<n>`. crew takes a bolt out of its plan when it closes the landed bolt, or drops it. Then:
- a bring-up of the bolt that the runner does not hold is given no more (Decision 2);
- a bring-up the runner refused, or one sluice lists no more, is let go (Decision 4);
- the bolt's base place, a launch of `bolt/<b>`, is let go with its acts once Flume no longer lists its place, as a unit's launch is. crew has read its landing by then, since it closes the bolt on that reading;
- a request of `bolt/<b>` the runner does not hold is given no more, as one of a unit gone is.

`Main::plans` is one reading of crew's plan for every request. Reading `bolt/<b>` apart for previews alone would have one work leave the plan for some requests and not others.

Not taken: **leaving the base place's records on the branch.** crew reads nothing of a bolt it has closed.

### 4. A preview still up stays until its done

`let_go`'s sweep for work gone stands, its done taken out, since Decision 1 lets a done go. A bring-up leaves once its work has left the plan and it is not up. A bring-up still up stays, whatever the plan holds, until its done.

Not taken: **letting a bring-up go once its work leaves the plan, up or not.** crew writes a done as the row ends, often after the bolt has left the plan. With the bring-up gone, the runner's done would find nothing to put away and refuse it. sluice's preview would stay up with no ask left to put it away (core.55).

### 5. The record

Record 0031 gains an amendment, proposed: *a preview leaves the runner's report once its done is answered*. It records Decisions 1 to 4 and the paths not taken. These are this stage's choices, within record 0031 and crew's record 0056, and cheap to change. The brief gives their direction. It is accepted with the record. It is written with this change.

No rule changes. core.55 says what the runner brings up and puts away, and on whose asking. It says nothing of what an implementation lets go, which core.1 and core.10 leave to the control plane.

### 6. The README

The README's section on crew's implementation says, beside `previews.rec`, when a preview leaves the runner's branch: once its done is answered, with what the done put away, or once its work has left the plan and it is no longer up, a bolt's work leaving with its `Bolt` record.

## Who owns what

| piece | owner | cited by |
|---|---|---|
| whether crew still asks for a preview | crew: its newest preview request of a host, repository and revision (crew's design, Decision 2) | crew's implementation |
| what is let go | the control plane, through its implementation (core.1, core.10) | the runner |
| when a bolt or a unit leaves the plan | crew's `plan.rec` | crew's implementation |
| bringing up and putting away | the runner (core.55) | crew |

## Risks / Trade-offs

- **A done crew writes with no bring-up the runner holds is never given.** It is never refused in the runner's words either. crew writes a done only for an ask that stands, and reads no done's answer, so nothing waits on one.
- **Two spellings of one repository are two asks.** Requests are matched by `Repo` as written, as crew keys them. crew writes each done with its ask's own `Repo`, so a done always matches its ask.
- **A bring-up refused while its work stays in the plan stays on the branch.** It goes only with its work, as today: crew shows its refusal on the card while it is the ask, and writes no done for an ask the runner refused.
- **The first report after this change lets go every done already answered**, and the bring-ups before each. The runner says one line for each. Nothing is run.

## Migration Plan

- **Runners.** A runner of a release before this one keeps put-away previews on its branch, as today. Once updated, its first report lets them go (Decision 1). `report.json` and `previews.rec` change no field or descriptor.
- **crew.** Nothing changes for crew. A bring-up and its done that the runner did not answer are left unanswered. crew reads no ask of them, since its newest request of those three is the done.
- **Rollback.** A runner rolled back gives every preview request whose work is in the plan, as before. A bring-up whose put-away this release let go is then given again and brought up again, until its work leaves. sluice answers its done again with nothing to release.

## Open Questions

None.

## Task notes

- **1.1, the test.** It follows `a_launch_let_go` in `tests/leases.rs` and `a_previews_lease_refused` in `tests/previews.rs`. Pass 1 brings `home/30` up with the stub answering `preview-starting.json`. Then `plane.let_go(id("home/30"))`, and pass 2 finds:
  - the line;
  - no `home/30` in `bench.report()`;
  - no `release` call.

  Pass 3's last report does not carry it. The fake stops giving a want it let go, so pass 2 reads no `sluice previews` for it.
- **2.1.** `Main::plans` gains the arm `["bolt", bolt] => self.bolts.contains(*bolt)`. Its documentation names `bolt/<bolt>` beside `fix/<bolt>/<name>`.
- **2.2.**
  - `read.rs` gains `Request::preview_key(&self) -> Option<(&str, &str, &str)>`, the request's `Host`, `Repo` and `Revision` where it is a preview naming all three, and `Main::newest_preview(&self, request: &Request) -> bool`. That is true where no request of a higher number, `Act: preview`, naming the same key, reads as a preview (`read::want`'s rules for `Done`).
  - `Pulling::current` takes a preview request's rule before the rest:
    - held, given;
    - else not newest, not given;
    - else a done, given where some request of a lower number with the same key, not marked done, is `reported` or `branch_answers`;
    - else, a bring-up, given where `plans` holds its work.
  - `lib.rs`'s comment over the loop cites record 0031's amendment.
- **2.2, the existing tests.**
  - `read.rs` `a_preview_marked_done` hands its pull a report holding `home/30` answered `done`, as the modified scenario has it.
  - `letgo.rs` `a_preview_whose_work_has_left_the_plan` finds `home/31` given too, by the done's rule, whatever the plan holds. Its requests hold 30 and 31.
- **2.3.**
  - `let_go`'s preview sweep runs in two passes over `report.previews`:
    - first the dones, each gone with the bring-ups before it of its key;
    - then the bring-ups not yet gone, by today's rule, a done no longer counted in it.
  - `letgo.rs`'s `a_preview_put_away_its_work_gone` keeps only its second half, the plan holding no `u`. Its first half, nothing let go while the plan holds `u`, is what Decision 1 changes. `a_preview_put_away_its_work_still_in_the_plan` proves the new rule: the plan holds `u`, and the let-go is `["home/30", "home/31"]`.
  - A helper beside `main_with_previews` takes the work and the requests, so the bolt and the later bring-up need no second copy of the requests.
