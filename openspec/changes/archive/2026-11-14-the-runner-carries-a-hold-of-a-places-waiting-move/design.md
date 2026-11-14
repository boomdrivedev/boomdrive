# Design

## Context

See proposal.md for why. No area page binds the paths this change touches, so `docs/architecture/core.md` binds every one. The change relies on these rules:

- core.1: the runner decides nothing the control plane or Flume decides;
- core.2: the control plane is reached through one interface, and only an implementation names crew or the hosted protocol;
- core.10: each act is carried out once, under its launch's lease, and a lease the control plane refused stops it;
- core.11: each answer is the tool's;
- core.12: every key and type names what it holds, a duration's key ending in `_interval`;
- core.24: work under a lapsed lease waits;
- core.26: what goes over one account's connection names only that account's own places;
- core.36: `report.json` keeps every field the release before reads;
- core.38: below a tool's floor the runner takes no work that needs the tool;
- core.42: a protocol version a release has spoken is never changed;
- core.50: a pull's acts on places run after its launches, and once the next pass is due only the first runs;
- core.51: an act on a place the listing does not hold is first asked of Flume's status;
- core.53: a land is Flume's retry of the submission of its launch's place.

It draws core.59 and amends record 0002 with an amendment of 2026-11-14. Both are written with this change. It departs from no rule.

The control plane is reached through `boomdrive-control`'s `ControlPlane` interface alone (core.2). The hold is a sort of the interface's `Act`. crew's implementation, `crates/boomdrive-crew`, gives it from crew's requests. The hosted implementation, `crates/boomdrive-hosted`, gives none, since no version of its protocol names one (Decision 7).

### What stands

- **Flume's side**, its clause 130 and its rule core.24, at Flume's `v0.17.0` and every release since:
  - a place whose next move is its caller's waits on that move, and status says which and since when;
  - a caller may hold that move with words saying who makes it and how, and a bound after which the hold lapses. A later hold on that move replaces it. A retry naming the stage or the submission ends it as the move made;
  - a place waiting on its caller under no hold's words for five minutes, or the launch's bound, is stalled, and its stall is delivered once to each recipient, to the role a stop-short named, and to the answerer behind its stream;
  - Flume watches no event for a hold and ends none itself, but lets a hold on the waiting move lapse at its bound;
  - the command is `flume hold <place> --waiting [--for <time>] <words>`. `--for` takes `<n>`, `<n>s`, `<n>m` or `<n>h`, and with none the hold holds until the move is made. `<words>` is a positional argument. Flume refuses a hold of a place that waits on no move of its caller's, naming it, and a place whose work is over. It answers `acted` on exit 0.
- **crew's side**, crew's record 0067 as amended on 2026-11-14 by boomdrive-design:
  - while a proven bolt waits on the user, its base place waits under words naming what it waits on and where the user answers it;
  - crew posts `hold`, naming the bolt's launch and the words, and names neither Flume nor its command (crew's record 0056, point 1);
  - the runner carries it as Flume's hold of the base place's waiting move, with the words and bound given. This unit is built first, and crew's unit `a-bolt-waiting-on-the-user-waits-under-words` builds on it.
- **The interface's acts.** `ActSort` is `Retry { stage, words, submission }`, `Land`, `Confirm { recipient, deliveries }`, `Drop`, `Priority(i64)` and `Done` (`src/want.rs`). An `Act` names its launch and that launch's lease number. `Acted`, what the report holds of an act, names no sort, so `report.json` holds none. The interface also has a type `Hold { stage, words }`, a stage a launch puts on hold, which is a field of `Launch` and no act.
- **The runner carries out an act** in `crates/boomdrive/src/carry.rs`:
  - `Work::act` answers an act the report holds from it, refuses one on a launch that is no launch of this runner's, leaves one under a lapsed lease, refuses a confirm the runner carries for no one (`refused_confirm`), and sends one on a launch not launched to `act_unlaunched`;
  - `act_unlaunched` refuses a retry, a land, a confirm or a done, naming where the launch stands;
  - `act_on` runs an act on its place in the pass's second round alone. It first asks Flume's status of a place the listing does not hold, whose refusal is the act's answer. Once the next pass is due, it runs none but the first (core.50, core.51). An act it runs sets `ran`, so the statuses are carried against Flume's listing read again;
  - `flume_act` maps a sort to Flume's own command, `flume::Act`, and `sort` names it in the lines.
- **Flume's command line** is `flume::Cli`. Every call puts `--json` first and runs with `FLUME_RECIPIENT` unset. `act_args` gives each act's arguments after `--json`. `flume::LEAST` is `0.21.0`, and the set's Flume is `v0.21.2`.
- **crew's implementation** reads a request's `Act` in `read::want`: `launch`, `retry`, `preview`, `land`, `confirm`, `drop` and `priority`. Any other `Act` is refused by the runner, naming it, for a request naming its host. One naming no host is given nothing, and named in the log, on every pass (`src/lib.rs`). crew's requests already name a field `Hold`, a launch's `Hold: submission`, and a field `Words`, a retry's.
- **The hosted protocol.** Version 1, `protocol/v1/`, was spoken by every release from `v0.1.0` to `v0.7.3`. Version 2, `protocol/v2/`, was spoken by `v0.9.0`, and is unchanged since. Its `act_sort` names `retry`, `land`, `confirm`, `drop`, `priority` and `done`. The hosted crate's `tests/interface.rs` holds `every_sort_the_interface_names`, which names every sort of `Want`, `ActSort` and `CommandSort` by a match with no `_` arm, and checks a value of each against version 2's `want`.

## Goals / Non-Goals

**Goals:**

- A control plane asks that the move a launch's place waits on it for be held with `hold`, naming the launch, the words and a bound, and names no Flume command for it.
- The runner that holds the launch carries it out as Flume's hold of that place's waiting move, with exactly those words and that bound, under the launch's lease, once, as it carries out any act.
- crew's implementation gives a hold from crew's `Act: hold`.

**Non-Goals:**

- **What a place waits on, and which words say so.** These are the control plane's (core.1). crew decides them for its proven bolts, in its own unit.
- **Renewing or ending a hold.** A hold lasts until its bound, a later hold, or the move made. The control plane posts a new hold where it wants one, and a land ends it. The runner keeps no hold of its own.
- **A hold of a stage or of the submission after the launch.** Flume takes one, and nothing asks for it. A launch's own holds stay its `holds` and `hold_submission`.
- **A new protocol version.** Neither spoken version may change (core.42). A version that names a hold is the runner's and the cloud's to make together (*Named follow-ups*).

## Decisions

### 1. A hold is an act of the interface's own

`ActSort` gains `Hold { words: String, bound_interval: u64 }`. Its serde form is `{"hold": {"words": "…", "bound_interval": 28800}}`, as `Confirm`'s is a map under its name.

- `words` is the control plane's words, saying who makes the move and how. They are the caller's to make good (Flume's 130).
- `bound_interval` is how long the hold holds, in whole seconds, from when Flume takes it. Flume lets it lapse then, and the place stalls again. It ends `_interval`, as core.12 has a duration's key end.

Its documentation says it asks that the move the launch's place waits on its caller for be held under its words for its bound. It says the runner carries it out as Flume's hold of that waiting move (core.59). It also says no version of the hosted protocol names a hold (Decision 7).

A hold is an act, not a command. It acts on its launch's place, and that launch's lease must stop it once the control plane refuses the lease (core.10). A place Flume no longer stands must answer it with Flume's refusal (core.51). And it goes to the runner that holds the launch, on whichever host that is.

The variant `ActSort::Hold` and the struct `Hold` are each a hold as Flume means it, one of a stage put at the launch and the other of the waiting move put after it (Flume's 130). The variant lives in `ActSort`'s namespace, so the two names do not collide.

Alternatives:

- **An optional bound, none holding until the move is made.** Not taken. The brief and crew's record 0067 as amended name a bound. A hold with none hides the place's stall for good if the control plane never makes the move and never says so. The amendment's "a stall still says something" holds only where every hold lapses.
- **A time the hold lapses at, in place of a span.** Not taken. Flume takes a span. The runner would compute it from two hosts' clocks and judge a time already passed. An act waits at most a pass or two before it runs (core.50), which a bound of hours does not notice.
- **One act holding a stage, the submission or the waiting move, naming which.** Not taken. Nothing asks for a stage's or the submission's hold after the launch, and Flume gives a bound to the waiting move's alone. A field naming which can be added when something asks.

### 2. The runner carries out a hold as Flume's hold of the waiting move

`flume::Act` gains `Hold { words: String, bound_interval: u64 }`. Its arguments are `hold <place> --waiting --for=<n>s -- <words>`, so the call is `flume --json hold <place> --waiting --for=<n>s -- <words>`.

- **The words are never read as a flag.** They come from the control plane and follow `--`. Words such as `--stage code` are then words Flume holds, never a flag. The place is the runner's own, from its report, and comes before `--`.
- **The bound is the act's.** It is `--for=<n>s`, `<n>` being `bound_interval` in decimal digits, joined to its flag with `=`. A bound of 0 is passed as given. Flume then lets the hold lapse at once, which is Flume's to answer.
- **Exactly the words given.** The runner adds nothing to them and changes nothing of them (core.1, core.15).

`flume_act(ActSort::Hold { words, bound_interval })` is `flume::Act::Hold { words, bound_interval }`. Flume's answer is the act's (core.1, core.11):

- exit 0 is `done`;
- exit 1 is `refused`, with Flume's reason, such as `the place P waits on no move of its caller's`;
- any other exit is `refused`, with Flume's standard error.

Everything else is the act machinery's, which turns on no sort:

- it runs in the pass's second round, after every launch the pull offers;
- it is asked of Flume's status first where the pass's listing does not hold its place;
- it is left to the next pass once that pass is due, unless it is the pass's first act;
- it waits while its launch's lease has lapsed;
- it is answered from the report once it is answered;
- a hold Flume ran sets `ran`, so the statuses are carried against Flume's listing read again, which shows the hold.

`sort(&ActSort::Hold { bound_interval, .. })` is `hold for <n>s`. A hold's line reads `<id>: hold for <n>s of <launch> at <place>: done`. The words stay out of the line, as a retry's do. The report holds no sort and no words.

### 3. The runner judges nothing of the wait

The runner does not read whether the place waits on its caller, on which move, or since when, and does not hold a hold's words to anything. Flume refuses a hold of a place that waits on no move of its caller's, and of one whose work is over, and that refusal is the act's answer (core.1, core.11).

A hold is carried out once (core.10). A later `hold` act, under its own id, is carried out as any act, and Flume replaces the earlier hold with it. A land is Flume's retry of the submission (core.53), which Flume takes as the move made, ending the hold. The runner watches no event, renews no hold and ends none (Flume's 130 and 72; Flume's core.24).

Alternatives:

- **Holding only a place whose status reads waiting on its caller.** Not taken. Flume answers that, and the runner would read Flume's meaning of a wait.
- **Renewing a hold before its bound, while the control plane still names it.** Not taken. The control plane decides how long its words hold (core.1), and the runner would keep a timer of its own.

### 4. A hold with no words is refused

`Work::act`, once the act's launch is the runner's and its lease is live, refuses a hold whose `words` is empty, with the words `a hold with no words says nothing of who makes the move`. It runs no command, so its answer turns on no round. This sits beside `refused_confirm`, the runner's refusals of an act it carries for no one.

Why: a hold's words are what Flume shows in its stall's place (Flume's 130). A hold with none would take the place out of stalling with nothing said, which is what the hold is meant to prevent. Flume takes any string, the empty one among them, so the runner says it before Flume takes it.

Alternatives:

- **Passing it to Flume.** Not taken. Flume would hold the move under no words, and the place would read neither stalled nor waiting on anyone.
- **Refusing words of spaces alone.** Not taken. The runner judges no words but their absence.

### 5. A hold of a launch not launched is refused

`act_unlaunched` refuses a hold as it refuses a retry, a land, a confirm or a done, naming where the launch stands: held on its After, refused, or done by a drop. It runs no command. A launch not launched has no place, so nothing waits on its caller. A hold so refused changes nothing of the launch when it is made.

### 6. crew's implementation reads `Act: hold`

- **`read::want`** gives `Some("hold")` as `act(ActSort::Hold { words, bound_interval })`:
  - `words` is its `Words`, as a retry's are read, `request.get("Words")`. None is `it names no Words`;
  - `bound_interval` is its `Bound`, a whole number of seconds. None is `it names no Bound`. One that is not decimal digits fitting a `u64` is `its Bound <text>, which is no whole number of seconds`;
  - it reads no other field. The words are passed as crew wrote them, a `Words` over `+` lines joined by newlines, as records read them.
- **`lib.rs`**'s split of a request naming no host sends a `hold` with the `retry`, `land`, `confirm`, `drop` and `priority`. So a hold naming no host goes to the runner that holds its launch, and to no other. One that cannot be read is given nothing and named in the log, as any act naming no host is.
- **`tests/common/mod.rs`**'s copy of crew's `requests.rec` descriptor gains `hold` in its `Act` enum, `%type: Bound int`, and `Bound` in its `%allowed` after `Delivery`. Its `FIELDS` gains `Bound` after `Delivery`. Its documentation says so, naming crew's unit `a-bolt-waiting-on-the-user-waits-under-words`, which writes them.
- `Request::of`'s documentation names a hold among the acts whose `Of` it reads.

The field names are crew's words for its requests. `Words` is the one a retry already carries. `Bound` is clause 130's word for what the hold lapses after. crew's unit takes them from here, since this unit is built first.

Alternatives:

- **`Bound` in Flume's span form, `8h`.** Not taken. crew's implementation would hold a copy of Flume's reading of a span, and a person reading crew's requests reads a number of seconds as readily. recutils checks an `int`.
- **A hold whose `Words` is missing given as one of no words, for the runner to refuse.** Not taken. A field crew did not write is a request crew's implementation cannot read, as a confirm's missing `Recipient` is.

### 7. No version of the hosted protocol carries a hold

No file under `protocol/v1/` or `protocol/v2/` changes. Version 1 was spoken by `v0.1.0` to `v0.7.3`, and version 2 by `v0.9.0`. Neither names a hold, and core.42 rules out changing either. A want whose act is a hold fails each version's `want`, so a hold never reaches the hosted implementation's types. The runner never writes a want, and `Acted` names no sort, so nothing the runner sends changes.

`tests/interface.rs` shows it:

- **A new test, `a_hold_is_no_act_of_version_1_or_2`,** checks a want whose act is a hold against version 1's `want` and against version 2's, and finds each refused.
- **`every_sort_the_interface_names`** keeps its intent, that a sort the interface adds fails to build until it is named. `act_sort` names the hold, `"hold"`, and `ACT_SORTS` holds it. `every_sort` gives a hold among its acts. The test checks every value against version 2's `want`: each passes but the hold, which is refused, and the sorts named are still every sort the interface has. Its documentation says a sort added after `v0.9.0` is refused by version 2 until a later version names it.

`protocol/README.md`'s sentence "Version 2 has no message for a forge token, for health, for the runner's own commands, or for a terminal's certificate" gains "or for a hold of a place's waiting move". That file describes the versions, and is no version's file.

Alternatives:

- **Adding `hold` to version 2.** Not taken. `v0.9.0` has spoken it, and core.42 rules out changing it.
- **Version 3 now, naming a hold.** Not taken. A version is the runner's and the cloud's to make together, with its subprotocol, contexts, copies, examples and tests, and the cloud pins the protocol by revision. The README's list of what version 2 lacks says those changes add them in a version of their own (*Named follow-ups*).

### 8. No floor moves

`flume hold --waiting --for` is Flume's from `v0.17.0`. `flume::LEAST`, `0.21.0`, and the set's Flume, `v0.21.2`, both have it, so neither moves (core.38). A host whose Flume lacks it is below the floor, where the runner takes no work that needs Flume.

### 9. The rule and the record

- **core.59**, under *Work under a lease*, after core.56: a control plane asks that the move a launch's place waits on it for be held with `hold`, an act naming its launch, its words and its bound, under that launch's lease. The runner carries it out as any act, by Flume's hold of that place's waiting move, with exactly those words, after `--`, and that bound. It refuses a hold with no words. It judges nothing of the wait, and renews and ends no hold of its own accord. Flume's answer is the hold's. Its details name what it rules out and what it leaves.
- **Record 0002's amendment of 2026-11-14** records the direction, boomdrive-design's in crew's record 0067 as amended, and the stage's choices, Decisions 1, 4, 6 and 7. Nothing in it is the user's alone, and each choice is cheap to change.

### 10. The README

The README's control pass says the runner carries out each retry, land, confirm, hold, drop, priority or done as Flume's own command on the launch's place. It says a hold is `flume hold <place> --waiting --for=<n>s -- <words>`, with exactly the words and bound its control plane names. It says the runner judges nothing of the wait, and renews and ends no hold of its own accord. Its hosted section says no version carries a hold yet, so a hosted control plane holds nothing through the runner.

## Who owns what

| piece | owner | cited by |
|---|---|---|
| what a place waits on, the words, and the bound | the control plane (core.1); crew, for its proven bolts (crew's record 0067 as amended) | the runner |
| the message `hold`, naming its launch, its words and its bound | the interface, `boomdrive-control` (core.2, core.59) | crew's implementation, the runner |
| `Act: hold` in crew's requests, with `Words` and `Bound` | crew, which writes it (its unit `a-bolt-waiting-on-the-user-waits-under-words`) | crew's implementation, which reads it |
| how a hold is carried out | the runner (core.59) | crew, the cloud |
| a hold of the waiting move, its lapse, and the stall | Flume (its 130 and core.24) | the runner |
| a hold in the hosted protocol | a later version, the runner's and the cloud's together (core.42) | the hosted implementation |

## Named follow-ups

- **boomdrivedev/crew, its unit `a-bolt-waiting-on-the-user-waits-under-words`:** posts `Act: hold` with `Of`, `Words` and `Bound`, naming no `Host`, and adds `hold` to its `Act` enum, `Bound` as an `int` and `Bound` to `%allowed` and `FIELDS` after `Delivery`, as Decision 6 names them. Its holds written before a host runs this release are carried once it does.
- **agentplot/boomdrive-cloud and this repository: a version that names `hold`.** Until then a hosted control plane's proven work stalls as it does today.
- **boomdrivedev/blueprints: Flume's statement, the runner's row of its callers.** It names a retry of a base place's submission as a land (203), and a confirm of ends its control plane asks (43). A hold of a place's waiting move its control plane asks, with its words and bound, joins them (130). That is the statement's owner's to amend.
- **This repository: version 2 held as `v0.9.0` spoke it.** The runner-protocol spec's "Version 1 stays as a release spoke it" holds version 1's files to their digests. Version 2 has no such check since `v0.9.0` spoke it. A chore adds one.

Each is told to the orchestrator.

## Risks / Trade-offs

- **A hold that reaches Flume after the place stalled.** The stall is delivered once already, and the hold then takes the place out of stalling (Flume's 130). The runner carries a hold in its next control pass, so a hold crew posts at a bolt's proof reaches Flume within a pass or two, and before the five minutes where crew posts it at once.
- **A hold whose bound passes before the control plane renews it.** The place stalls again, and its stall says so. That is the control plane's to avoid, by a bound as long as the wait or a hold posted again.
- **A hold of a place that waits on nothing yet.** Flume refuses it, and the control plane sees Flume's words. crew's unit posts a hold once the bolt waits on the user.
- **A hosted control plane cannot hold** until a later version (Decision 7).

## Migration Plan

Nothing held changes shape. `report.json` keeps every field, since `Acted` names no sort (core.36). No protocol file of a version changes (core.42).

- **crew's holds written before this release.** A runner of a release before this one refuses a hold naming its host, naming its `Act`, and gives one naming no host nothing, naming it in its log. Once the runner runs this release, it carries each one it has not answered.
- **A runner rolled back** after it answered a hold answers it from its report, by id, and runs nothing again (core.10). A hold it never answered is read as above.

## Open Questions

None.

## Task notes

- **1.1.** `tests/json.rs`'s `an_act_of_each_sort`:
  - adds `ActSort::Hold { words: "the user sends it on the landings card, row 3", bound_interval: 28800 }` to its sorts;
  - asserts that `serde_json::to_value` of that hold's `Act` gives `"what": {"hold": {"words": "the user sends it on the landings card, row 3", "bound_interval": 28800}}`.
- **2.1.**
  - `src/flume.rs`'s `Act` gains `Hold { words, bound_interval }`, documented: Flume's hold of the move the place waits on its caller for, under the words, for the bound in whole seconds (Flume's 130).
  - `act_args(place, Act::Hold { .. })` is `["hold", place, "--waiting", "--for=<n>s", "--", words]`. Its documentation says the words follow `--`.
  - In `src/carry.rs`, `flume_act` maps `ActSort::Hold` to it, and `sort` names it `hold for <n>s`. `act_unlaunched` refuses it with the retry's words, which is Decision 5 and lets the crate build.
  - `tests/flume.rs` gains `a_holds_argv`. Over the stub, it runs `cli.act("/places/P", &Act::Hold { words: "the user sends it", bound_interval: 28800 })` and finds the call `--json hold /places/P --waiting --for=28800s -- the user sends it`, the words one argument. It also runs a hold whose words are `--stage code` and whose bound is 0, and finds `--json hold /places/P --waiting --for=0s -- --stage code`.
  - The module's documentation names a hold among the acts.
- **2.2.**
  - `tests/fakes/mod.rs`'s `FakeFlume::act` answers a hold as any act, `{"acted": {"act": "hold", "place": <place>}}`, or from `act_answers` by its place, and changes no place.
  - **2.2 to 2.3** live in a new `tests/holds.rs`. It opens with a module comment naming the work-into-flume spec's hold requirements and core.59, and holds `mod fakes;`. Each test makes `a` with a first pass over `Fake::holding(vec![Want::Offer(offer("a", "a"))])`, then pulls `a` and its act, as `confirms.rs`'s `launched` and `pull` do. Its words are neutral: `the user sends it on the landings card, row 3`.
  - **2.2 `a_hold_is_flumes_hold_of_the_waiting_move`:** `a/hold` holds `a` under the words for 28800 seconds. `bench.flume.acts()` is exactly `[(P, Act::Hold { words, bound_interval: 28800 })]`. The act is `Done` with no words, and the lines hold `a/hold: hold for 28800s of a at P: done`, and no line holds the words.
  - **2.2 `flumes_refusal_is_the_holds_answer`:** `bench.flume.act_answers(&P, Ok(Reply::Refused("the place P waits on no move of its caller's")))`, `P` being `place_of("a")`. The act is `Refused` with those words.
  - **2.3 `a_hold_with_no_words_is_refused`:** `a/hold` holds `a` under the words `""` for 28800 seconds. The act is `Refused` with words saying it names no words, and `bench.flume.acts()` is empty.
- **2.4.** `tests/after.rs`'s `a_hold_of_a_held_launch` follows `a_confirm_of_a_held_launch`. It pulls `a`, `b` after `a`, `b/retry` and `b/hold`, a hold under the words for 28800 seconds. The hold's words equal the retry's, and `bench.flume.acts()` is empty.
- **3.1.** `tests/read.rs`'s `a_hold_is_an_act_on_the_launch_it_names` writes request 12, a launch naming `host-a`, and request 17, `act(17, "hold", Some("host-a"), "unit/a", 12, &[("Words", "the user sends it on the landings card, row 3"), ("Bound", "28800")])`. It finds the pull's want `home/17` to be `Act { launch: home/12, lease: 1, what: ActSort::Hold { words: "the user sends it on the landings card, row 3", bound_interval: 28800 } }`.
- **3.2.** `tests/read.rs`'s `a_hold_naming_no_words_no_bound_or_a_bound_of_no_whole_seconds_is_refused` writes requests 18, with a `Bound` of `60` and no `Words`, 19, with `Words` and no `Bound`, and 20, with `Words` and a `Bound` of `8h`, each naming `host-a` and `Of: 12`. Each is given as `read::refused` gives it, as `a_confirm_naming_no_recipient_two_or_no_delivery_is_refused` reads its requests. 18's launch id names `it names no Words`, 19's `it names no Bound`, and 20's `its Bound 8h, which is no whole number of seconds`.
- **3.3.** `tests/claims.rs`'s `a_hold_naming_no_host_goes_to_the_runner_holding_its_launch` follows `a_confirm_naming_no_host_goes_to_the_runner_holding_its_launch`:
  - requests 26, `act(26, "hold", None, "unit/a", 21, &[("Words", "the user sends it"), ("Bound", "28800")])`, and 27, the same of `Of: 20`;
  - `host-b` answers launch 20;
  - `host-a`'s pull ids are `home/21` and `home/26`, and `home/26` holds `home/21` under `the user sends it` for 28800 seconds.
- **4.1.** In `tests/interface.rs`:
  - `act_sort` gains `ActSort::Hold { .. } => "hold"`, and `ACT_SORTS` becomes `["retry", "land", "confirm", "hold", "drop", "priority", "done"]`;
  - `every_sort` gives `ActSort::Hold { words: "the user sends it", bound_interval: 28800 }` among its acts;
  - `every_sort_the_interface_names` checks each value with `common::refused` where it is an act whose sort is a hold, and with `passes` otherwise. It still asserts the three sets of sorts named;
  - its documentation, and the module's, say a hold is the one sort version 2 does not carry, since `v0.9.0` spoke it, and that a sort added after it fails version 2 until a later version names it.
- **4.2.** `tests/interface.rs`'s `a_hold_is_no_act_of_version_1_or_2` builds `Want::Act(Act { id: act-1, launch: offer-1, lease: 3, what: ActSort::Hold { words: "the user sends it", bound_interval: 28800 } })`. It checks its JSON with `common::refused` against `V1.validator(&V1.definition("wants.schema.json", "want"))` and against `validator(&definition("wants.schema.json", "want"))`.
