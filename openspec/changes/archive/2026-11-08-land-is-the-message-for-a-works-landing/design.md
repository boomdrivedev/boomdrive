# Design

## Context

See proposal.md for why. There are no area pages, so `docs/architecture/core.md` binds every path this change touches. The change relies on these rules:

- core.1: the runner decides nothing the control plane or Flume decides;
- core.2: the control plane is reached through one interface, and only an implementation names crew or the hosted protocol;
- core.10: each act is carried out once, under its launch's lease;
- core.11: each answer is the tool's;
- core.15: a launch is passed as given, and no tool is asked anything for it;
- core.24: work under a lapsed lease waits;
- core.36: `report.json` keeps every field the release before reads;
- core.42: a protocol version a release has spoken is never changed;
- core.50: a pull's acts on places run after its launches, and once the next pass is due only the first runs;
- core.51: an act on a place the listing does not hold is first asked of Flume's status;
- core.52: the runner asks no tool for a branch but through Flume's commands on a place.

It draws core.53, amends core.52's text and details, and amends record 0002 with an amendment of 2026-11-08, proposed. All three are written with this change. It departs from no rule.

The control plane is reached through `boomdrive-control`'s `ControlPlane` interface alone (core.2). The land is a sort of the interface's `Act`. crew's implementation, `crates/boomdrive-crew`, gives it from crew's requests. The hosted implementation, `crates/boomdrive-hosted`, gives none under version 1 (Decision 7).

### What stands

- **The interface's acts.** `boomdrive-control`'s `ActSort` is `Retry { stage, words, submission }`, `Drop`, `Priority(i64)` and `Done` (`src/want.rs`). An `Act` names its launch and that launch's lease number. `Acted`, what the report holds of an act, names no sort, so `report.json` holds none.
- **The runner carries out an act** in `crates/boomdrive/src/carry.rs`:
  - `Work::act` answers an act the report holds from it, refuses one on a launch that is no launch of this runner's, leaves one under a lapsed lease, and sends one on a launch not launched to `act_unlaunched`;
  - `act_unlaunched` answers a priority and a drop `done` from the report, and refuses a retry or a done, naming where the launch stands;
  - `act_on` runs an act on its place in the pass's second round alone, asks Flume's status first of a place the listing does not hold, and runs none but the first once the next pass is due (core.50, core.51);
  - `flume_act` maps a sort to Flume's own command, `flume::Act`, and `sort` names it in the lines.
- **Flume's retry** is `flume::Act::Retry { stage, words, submission }`, whose arguments are `retry <place>`, then `--stage`, `--words` and `--submission` where given (`src/flume.rs`'s `act_args`).
- **A bolt's send** is a retry naming the submission (`tests/bolts.rs`'s `a_bolts_send` and `a_send_flume_refuses`).
- **crew's implementation** reads a request's `Act` in `read::want`: `launch`, `retry` (with `Stage`, `Words` and `Submission`), `drop` and `priority`. Any other `Act` is refused by the runner, naming it, for a request naming its host. For one naming no host it is given nothing, and named in the log (`src/lib.rs`, which sends a `retry`, `drop` or `priority` naming no host to the runner that holds its launch).
- **Version 1 of the hosted protocol** carries the interface's `Want` as its serde writes it (`protocol/v1/wants.schema.json`'s `act_sort`: `drop`, `done`, `retry` and `priority`). `protocol::v1` uses `boomdrive_control::Want` itself, and every message is checked against its version before it is read (core.42). Every release since v0.1.0 has spoken version 1.

## Goals / Non-Goals

**Goals:**

- A control plane asks for a work's landing with `land`, naming its launch, and names no Flume word for it.
- The runner carries a land out as Flume's retry of that launch's submission, under the launch's lease, once, as it carries out any act.
- crew's implementation gives a land from crew's `Act: land`.

**Non-Goals:**

- **A new protocol version.** Version 1 carries no land and may not change (core.42). A version that names `land` is the runner's and the cloud's together, under *Named follow-ups*.
- **crew's half.** crew posts `land` from its own unit. crew's implementation here reads it, and crew writes it.
- **What a landing reports.** A land's answer is Flume's, as every act's is. What came of the landing is in Flume's status of the place and sluice's entry of its submission, which the runner already carries (the work-into-flume and tool-records specs).
- **When work is landed, and which.** These are the control plane's (core.1).
- **A floor for Flume.** The runner runs no Flume command it did not run before (core.38).

## Decisions

### 1. A land is an act of the interface's own

`ActSort` gains `Land`, a unit variant. Its serde form is the word `"land"`, as `Drop`'s is `"drop"`, so an act is `{"id": …, "launch": …, "lease": …, "what": "land"}`. It holds nothing but its launch, named by the `Act` around it. Its documentation says it asks for the landing of its launch's work, which the runner carries out as Flume's retry of the submission (core.53). It also says version 1 of the hosted protocol names no land (Decision 7).

A land is an act, not a command. It acts on its launch's place, and that launch's lease must stop it once the runner lets the lease go (core.10). An act on a place Flume no longer stands is answered by Flume's refusal of its status (core.51).

Alternatives:

- **A retry with a field naming it a landing.** Not taken. The control plane would still send a retry, Flume's word, which is what the brief removes.
- **A land carried as a command, under a lease of its own.** Not taken, for the reasons above. Record 0002's amendment of 2026-11-07 found the same for a bolt's send.

### 2. The runner carries out a land as Flume's retry of the submission

`flume_act(ActSort::Land)` is `flume::Act::Retry { stage: None, words: None, submission: true }`. So a land of a launch at place `P` runs `flume --json retry P --submission`, and Flume's answer is the act's: `done` on exit 0, `refused` with Flume's reason on exit 1 (core.1, core.11).

Everything else about it is the act machinery's, which turns on no sort:

- it runs in the pass's second round, after every launch the pull offers;
- it is asked of Flume's status first where the pass's listing does not hold its place;
- it is left to the next pass once that pass is due, unless it is the pass's first act;
- it waits while its launch's lease has lapsed;
- it is answered from the report once it is answered.

The runner reads no kind. A bolt's base place, a fix and a unit whose submission is held all land the same way, and what each does on Flume's retry is Flume's (Flume's 203; core.15, core.52).

### 3. A land's line names it `land`

`sort(&ActSort::Land)` is `"land"`, so a land's line reads `<id>: land of <launch> at <place>: done`. A refused one reads `…: refused by Flume: <words>`, as every act's does.

### 4. A land of a launch not launched is refused

`act_unlaunched` refuses a land as it refuses a retry or a done, naming where the launch stands: held on its After, refused, or done by a drop. It runs no command. The launch, when made, is made as its offer gives it.

Alternatives:

- **The land carried onto the launch when it is made, freeing a held submission.** Not taken. It would change a launch from what its offer gives (core.15), and the answer would turn on when the launch was made. A control plane lands work that stands at a place, and one that sent a land early sees the refusal's words and sends it again.
- **The land left unanswered until the launch is made.** Not taken. A held launch may never be made, as when what it comes after is dropped, and the control plane would never learn why.

### 5. The retry naming the submission stays

`ActSort::Retry`'s `submission` stays, and the runner carries it out as it does now.

- crew's implementation gives it. crew's answer to a place stopped short in its submission is a retry naming the submission, with the user's words.
- Version 1 of the hosted protocol names it, as its serde writes it (core.42). Dropping it would make a pull holding one, which the check passes, fail to read whole.

A retry naming the submission and no words runs the same Flume command as a land. The runner carries out each as asked, and decides nothing between them (core.1).

### 6. crew's implementation reads `Act: land`

- **`read::want`** gives `Some("land")` as `act(ActSort::Land)`. A land reads its `Of` alone, and no other field of it: a `Stage`, `Words` or `Submission` on it is not read.
- **`lib.rs`**'s split of a request naming no host sends a `land` with the `retry`, `drop` and `priority`. So a land naming no host goes to the runner that holds its launch, and to no other.
- **`tests/common/mod.rs`**'s copy of crew's `requests.rec` descriptor gains `land` in its `Act` enum, as crew's unit writes it.

A runner of a release before this one reads `Act: land` as an `Act` crew's implementation does not read (Migration Plan).

Alternatives:

- **Refusing a land that holds a `Stage`, `Words` or `Submission`.** Not taken. crew's implementation reads no field a sort does not use, as with a `drop`, and crew writes none on a land.

### 7. Version 1 carries no land

Version 1 is unchanged: no file under `protocol/v1/` changes, and its `act_sort` names no `land` (core.42). A want whose act is a land fails version 1's `want`. So a land never reaches the hosted implementation's types, which read a message only once it passes its check. The runner never writes a want, and `Acted` names no sort, so nothing the runner sends changes.

`tests/interface.rs` shows it. `the_interfaces_values_pass` keeps every act sort version 1 names, and a new test, `a_land_is_no_act_of_version_1`, checks a want whose act is a land, as `boomdrive-control`'s types write it, against version 1's `want`, and finds it refused.

A hosted control plane lands work under version 1 by its retry naming the submission. The runner carries that out as it carries out a land (Decision 5).

Alternatives:

- **Adding `land` to version 1's `act_sort`.** Not taken. A release has spoken version 1, and core.42 rules out changing it.
- **Version 2 now, naming `land`.** Not taken. A version is a new directory of every file, its examples and its tests, and the cloud pins the protocol by revision. It is the runner's and the cloud's to make together, and the cloud does not yet send a bolt as a launch (*Named follow-ups*).
- **The hosted implementation reading version 1's retry naming the submission, with no words, as a land.** Not taken. The runner would rename what the control plane sent. A retry naming the submission with words is not a land, so the line would be drawn by the runner (core.1).

### 8. The rule and the record

- **core.53**, under *Work under a lease*: a control plane asks for a work's landing with `land`, an act naming the work's launch and nothing else, under that launch's lease. The runner carries it out as any act, by Flume's retry of the submission of that launch's place, naming no stage and no words, whatever the launch's kind, and Flume's answer is the land's. Its details name what it rules out and what it leaves, the retry naming the submission and version 1 among them.
- **core.52**'s text says a branch other places land on is landed on its control plane's `land` by Flume's retry of that place's submission. Its details let a bolt's send be carried as a land.
- **Record 0002's amendment of 2026-11-08** records the user's direction, quoting their words from crew's record 0056, and the stage's three choices, Decisions 4, 5 and 7. It is proposed until the user sees it as a record.

### 9. The README

The README's control pass says the runner carries out each retry, land, drop, priority or done as Flume's own command on the launch's place. It says a land is `flume retry <place> --submission`, whatever the launch's kind, and that a bolt is landed on a land, whose answer is Flume's.

## Who owns what

| piece | owner | cited by |
|---|---|---|
| when work lands, and which | the control plane (core.1) | the runner |
| the message `land`, naming its launch | the interface, `boomdrive-control` (core.2, core.53) | crew's implementation, the runner |
| `Act: land` in crew's requests | crew, which writes it (crew's record 0056) | crew's implementation, which reads it |
| how a land is carried out: Flume's retry of the submission | the runner (core.53) | crew, the cloud |
| what a retry of the submission does to a place, and its guards | Flume (203) | the runner |
| a land in the hosted protocol | a later version, the runner's and the cloud's together (core.42) | the hosted implementation |

## Named follow-ups

- **agentplot/crew: post `land`.** crew's record 0056 has crew's unit that lands each bolt through Flume's base place post `Act: land`, with `Of` naming the bolt's launch, once for each row the user sends, a red main's fix alike. That unit adds `land` to its `requests.rec` descriptor's `Act` enum. crew posts it once the runners of its partition run the release that carries this change, which `runner.rec`'s `Version` names. crew's implementation here needs nothing more.
- **agentplot/boomdrive-cloud and this repository: a version that names `land`.** The cloud lands a bolt by version 1's retry naming the submission until a later version names `land`. Its bolt is still made by a branch command, refused since `a-bolt-is-a-launch`, so it follows that change first.
- **agentplot/blueprints: Flume's statement, section 9.** crew's row has "the bolt submitted by crew's retry once the user sends it", and the runner's "a retry of a base place's submission its control plane asks (203)". crew asks Flume nothing now, and the runner's retry is asked as a land. That is the statement's owner's to amend.

Each is told to the orchestrator.

## Risks / Trade-offs

- **A hosted control plane names Flume's word until a version carries `land`** (Decision 7). The runner carries its retry as it carries a land. The cost is in the cloud's words, not in what the runner does.
- **A land sent before its launch is made is refused** (Decision 4). The control plane sees the runner's words and may send it again.
- **A retry naming the submission and a land run one command** (Decision 5). A control plane may send either, and the runner tells them apart only in its lines.

## Migration Plan

Nothing held changes shape. `report.json` keeps every field, since `Acted` names no sort (core.36). No protocol file changes (core.42).

- **crew posting `land` before its runners run this release.** A runner of a release before this one refuses a land naming its host, its words naming `Act` and `land`, and gives nothing for one naming no host, naming it in its log. crew posts `land` only once its runners run this release (*Named follow-ups*).
- **A runner rolled back** after it answered a land answers it from its report, by id, and runs nothing again (core.10). A land it never answered is read as above.

## Open Questions

None.

## Task notes

- **1.1.** `tests/json.rs`'s `an_act_of_each_sort` adds `ActSort::Land` to its sorts. It also asserts that `serde_json::to_value` of a land's `Act` gives `"what": "land"`.
- **2.1.** `tests/acts.rs`'s `a_land` acts as `acting` does, with the offer `home/12` and the act `home/13` of `ActSort::Land`. It asserts:
  - the stub's `retry` calls are exactly `--json retry <the place> --submission`;
  - no `drop`, `priority` or `done` call is made;
  - the act is `Done` with no words;
  - a line begins `home/13: land of home/12`.
- **2.2.** `tests/after.rs`'s `a_land_of_a_held_launch` follows `a_retry_or_a_done_of_a_held_launch` for the first pass, the land's words equal to the retry's there. It then reads `a` landed at a second pass, as `a_dependent_held_then_launched` does, and finds `b` launched with the arguments its offer gives, and still no act run.
- **2.3.** `tests/bolts.rs`'s `send()` becomes `act(&format!("{BOLT}/send"), BOLT, ActSort::Land)`. The module's documentation says a bolt is landed on a land, carried out as Flume's retry naming the submission, and cites core.53. `a_bolts_send` and `a_send_flume_refuses` keep their names and assertions.
- **3.1.** `tests/read.rs`'s `a_land_is_an_act_on_the_launch_it_names` writes request 12, a launch naming `host-a`, and request 17, `act(17, "land", Some("host-a"), "unit/a", 12, &[])`. It finds the pull's want `home/17` to be `Act { launch: home/12, lease: 1, what: ActSort::Land }`.
- **3.2.** `tests/claims.rs`'s `a_land_naming_no_host_goes_to_the_runner_holding_its_launch` follows `an_act_naming_no_host_goes_to_the_runner_holding_its_launch`:
  - requests 24, `act(24, "land", None, "unit/a", 21, &[])`, and 25, `act(25, "land", None, "unit/a", 20, &[])`;
  - `host-b` answers launch 20;
  - `host-a`'s pull ids are `home/21` and `home/24`, and `home/24` lands `home/21`.
- **4.1.** `tests/interface.rs`'s `a_land_is_no_act_of_version_1` builds `Want::Act(Act { id: act-1, launch: offer-1, lease: 3, what: ActSort::Land })`. It checks its JSON with `common::refused` against `validator(&definition("wants.schema.json", "want"))`. `the_interfaces_values_pass`'s module documentation says every act sort version 1 names passes, a land apart.
