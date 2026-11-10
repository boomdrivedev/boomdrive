# Design

## Context

See proposal.md for why. There are no area pages, so `docs/architecture/core.md` binds every path this change touches. The change relies on these rules:

- core.1: the runner decides nothing the control plane or Flume decides;
- core.2: the control plane is reached through one interface, and only an implementation names crew or the hosted protocol;
- core.10: each act is carried out once, under its launch's lease, and a lease the control plane refused stops it;
- core.11: each answer is the tool's;
- core.15: a launch is passed as given, and no tool is asked anything for it;
- core.24: work under a lapsed lease waits;
- core.26: what goes over one account's connection names only that account's own places;
- core.36: `report.json` keeps every field the release before reads;
- core.38: below a tool's floor the runner takes no work that needs the tool;
- core.42: a protocol version a release has spoken is never changed;
- core.50: a pull's acts on places run after its launches, and once the next pass is due only the first runs;
- core.51: an act on a place the listing does not hold is first asked of Flume's status;
- core.54: the runner is one more recipient, `boomdrive`, of each launch that names recipients, and carries and collects that end itself.

It draws core.56 and amends record 0002 with an amendment of 2026-11-09, proposed. Both are written with this change. It departs from no rule.

The control plane is reached through `boomdrive-control`'s `ControlPlane` interface alone (core.2). The confirm is a sort of the interface's `Act`. crew's implementation, `crates/boomdrive-crew`, gives it from crew's requests. The hosted implementation, `crates/boomdrive-hosted`, gives none under version 1 (Decision 7).

### What stands

- **Flume's side**, Flume record 0061, accepted on 2026-11-09, on Flume's main at c010ad3f and in its release `v0.21.0` (`77d5b93`):
  - each end of a place is a delivery under the key `<kind>--<name>/<token>/<end key>`, its readers the end's recipients;
  - `flume confirm <key>… [--recipient <name>]` confirms each key as the recipient. A key confirmed before answers when it first was. A key no end is under, or one whose end is another recipient's, is refused, naming the key, and the keys after it are not confirmed. It exits 0 when every key is confirmed and 1 on a refusal;
  - Flume keeps the host's deliveries in one document, so `flume confirm` names no place and takes any key on the host;
  - status gives each end its `delivery`, the key `flume confirm` takes, with `owed`, `confirmed` and `pending_for`;
  - a tidied place's record stays while an end of it is owed (62). Flume "takes a recipient's confirmation from the caller naming it ... and never confirms for one itself".
- **crew's side**, crew's change `crews-role-runs-confirm-what-they-collect`, landed on crew's main at 271e2c4:
  - a role's run takes, at its start, each end Flume owes its address, read from Flume's status as the runner carries it in `places.rec`;
  - at its turn's end it appends to `requests.rec` one request per launch: `Act: confirm`, `Work`, `Of` the launch, `Recipient` the role's address `<label>/<role>`, and one `Delivery` for each key, naming no `Host`;
  - crew's rules refuse a confirm with no `Of` launch, no `Delivery`, or a `Recipient` that is no role's address;
  - an end any confirm names is never taken again, whatever the runner answered (that change's Decision 5);
  - crew's `runner.REQUESTS` at crew's main fa8501c1 has `confirm` in its `Act` enum and `Delivery` in `%allowed`, and `runner.FIELDS` puts `Delivery` after `Words`.
- **The interface's acts.** `boomdrive-control`'s `ActSort` is `Retry { stage, words, submission }`, `Land`, `Drop`, `Priority(i64)` and `Done` (`src/want.rs`). An `Act` names its launch and that launch's lease number. `Acted`, what the report holds of an act, names no sort, so `report.json` holds none.
- **The runner carries out an act** in `crates/boomdrive/src/carry.rs`:
  - `Work::act` answers an act the report holds from it, refuses one on a launch that is no launch of this runner's, leaves one under a lapsed lease, and sends one on a launch not launched to `act_unlaunched`;
  - `act_unlaunched` refuses a retry, a land or a done, naming where the launch stands;
  - `act_on` runs an act on its place in the pass's second round alone. It first asks Flume's status of a place the listing does not hold, whose refusal is the act's answer. Once the next pass is due, it runs none but the first (core.50, core.51). Today it discards the status Flume gives;
  - `flume_act` maps a sort to Flume's own command, `flume::Act`, and `sort` names it in the lines;
  - `facts::entry` gives the place's entry in a listing or a status answer: the place standing at its location, else the latest record Flume keeps of it tidied, `<location>@<at>`.
- **Flume's command line** is `flume::Cli`. Every call puts `--json` first and runs with `FLUME_RECIPIENT` unset. `act_args` gives each act's arguments after the place.
- **crew's implementation** reads a request's `Act` in `read::want`: `launch`, `retry`, `land`, `drop`, `priority` and `preview`. Any other `Act` is refused by the runner, naming it, for a request naming its host. One naming no host is given nothing, and named in the log, on every pass (`src/lib.rs`). That is what becomes of each of crew's confirms today.
- **crew's mirror of a role's wants** (`src/roles.rs`'s `wanted`) reads a turn's `Took` only for `ask/…` and wait keys. A turn's `end/…` keys, which crew's change adds, answer nothing there, so it needs no change.
- **Version 1 of the hosted protocol** names the act sorts `drop`, `done`, `retry` and `priority` (`protocol/v1/wants.schema.json`'s `act_sort`). Its copy of an end, in `place-status.schema.json`, names `deliveries` and `pending_for`, and no `delivery`, `owed` or `confirmed`. Every release since v0.1.0 has spoken version 1.
- **Flume's floor.** `flume::LEAST` is `0.20.0`, and the set's Flume is `v0.20.0`, neither of which has `flume confirm`.

## Goals / Non-Goals

**Goals:**

- A control plane asks that a reader's ends be confirmed with `confirm`, naming the launch, the reader and each delivery's key, and names no Flume command for it.
- The runner that holds the launch carries it out as Flume's confirm of exactly those keys as that reader, under the launch's lease, once, as it carries out any act.
- A confirm under one launch's lease confirms no end of any other place.
- An end a control plane never named stays owed.
- crew's implementation gives a confirm from crew's `Act: confirm`.

**Non-Goals:**

- **The runner's own confirmation, as `boomdrive`.** It is this bolt's other unit, `the-runner-confirms-an-end-once-its-control-plane-has-it`. This change refuses a control plane's confirm as the runner (Decision 4) and changes nothing of the runner's own collection.
- **Which ends a reader acted on, and when it confirms them.** These are the control plane's (core.1). crew decides them for its role runs.
- **A new protocol version.** Version 1 carries no confirm and may not change (core.42). A version that names one is the runner's and the cloud's to make together (*Named follow-ups*).
- **The least Flume, and the set's Flume.** Decision 8.
- **sluice's outcomes**, which move onto weir's form in sluice's own bolt.

## Decisions

### 1. A confirm is an act of the interface's own

`ActSort` gains `Confirm { recipient: String, deliveries: Vec<String> }`. Its serde form is `{"confirm": {"recipient": "…", "deliveries": ["…", …]}}`, as `Retry`'s is a map under its name. `recipient` is the one reader the confirm is made as, as the control plane names it. `deliveries` is each delivery's key, as Flume gave it, in the control plane's order. Its documentation says so, and says that the runner carries it out as Flume's confirm of exactly those keys, each held to the launch's place (core.56). It also says version 1 of the hosted protocol names no confirm (Decision 7).

The names are Flume's and weir's own words for their own things: a recipient, and a delivery and its key (core.12).

A confirm is an act, not a command. It acts on its launch's place, and that launch's lease must stop it once the control plane refuses the lease (core.10). A place Flume no longer stands must answer it with Flume's refusal (core.51). And it goes to the runner that holds the launch, on whichever host that is, since only that runner's Flume holds the place's ends.

Alternatives:

- **A command, under a lease of its own, naming keys alone.** Not taken. It names no launch, so no implementation could send it to the host that holds the place. Every runner that took it would confirm any key on its host, under no launch's lease.
- **The runner confirming what crew's records say a reader took.** Not taken. Only crew's implementation reads crew's records (core.2). The runner would read crew's meaning of a turn, a confirm that no record says was asked. crew's change found the same, its Decision 4.
- **One act confirming every end its recipient is owed on the place.** Not taken. The brief has an end the control plane never named stay owed. A reader that took some ends of a place and not others confirms only those it took.

### 2. The runner carries out a confirm as Flume's confirm

`flume::Act` gains `Confirm { recipient: String, keys: Vec<String> }`. Its arguments are `confirm --recipient=<recipient> -- <key>…`, so the call is `flume --json confirm --recipient=<recipient> -- <key>…`.

- **No place is named.** Flume's key names its own place, and `flume confirm` takes none. `act_args` leaves the place out for a confirm. Decision 3 holds each key to the place.
- **Neither a key nor the recipient is read as a flag.** Both come from the control plane. The recipient is joined to its flag with `=`, and the keys follow `--`. A key such as `--recipient=boomdrive` is then a key Flume refuses, never a flag.
- **Exactly the keys named, as the recipient named.** The runner adds no key, drops none, and names no other recipient (core.1, core.15). An end the control plane never named is confirmed for no one, and stays owed until a confirm names it or it expires and Flume shows it (Flume record 0061).

`flume_act(ActSort::Confirm { recipient, deliveries })` is `flume::Act::Confirm { recipient, keys: deliveries }`. Flume's answer is the act's (core.1, core.11):

- exit 0 is `done`;
- exit 1 is `refused`, with Flume's reason. A key Flume refuses leaves the keys before it confirmed, which Flume's words name;
- any other exit is `refused`, with Flume's standard error. One is a Flume with no `confirm` (Decision 8).

Everything else is the act machinery's, which turns on no sort:

- it runs in the pass's second round, after every launch the pull offers;
- it is asked of Flume's status first where the pass's listing does not hold its place;
- it is left to the next pass once that pass is due, unless it is the pass's first act;
- it waits while its launch's lease has lapsed;
- it is answered from the report once it is answered;
- a confirm Flume ran sets `ran`, so the statuses are carried against Flume's listing read again, which shows the end confirmed.

This bolt's other unit gives the runner its own confirmation, as `boomdrive`, by a `Flume::confirm` of its own in place of `collect`. Both run `flume confirm`, so both name one form of its arguments, `confirm --recipient=<name> -- <key>…`. Whichever of the two the bolt takes second builds its call from the other's, so `flume.rs` holds that form once.

`sort(&ActSort::Confirm { recipient, .. })` is `confirm as <recipient>`. A confirm's line reads `<id>: confirm as <recipient> of <launch> at <place>: done`. The keys stay out of the line. The report holds no sort and no key.

### 3. Each key is held to its launch's place

Flume confirms any key on the host. So a key of another place, another launch's, another drive's or another account's, would be confirmed under this launch's lease, and as a reader of that other place (core.10, core.26). The runner holds each key to the launch's place before it runs anything:

- In `act_on`, the place's entry is the pass's listing's, where the listing holds the place. Else it is `facts::entry` of the answer Flume gives to `flume --json status <place>`, which `act_on` already asks for (core.51) and now keeps.
- `facts::deliveries(entry)` gives each `ends[].delivery` that is a string.
- Where a key of the confirm is none of them, the confirm is answered `refused` with the runner's words, `<key> is the delivery of no end of <place> as Flume gives it`, naming the first such key. No `flume confirm` is run.
- This comes before the check of whether the next pass is due, as Flume's refusal of the place's status does, so a confirm refused here counts no act run.

The runner reads Flume's own field and judges nothing of the end: not whether it is owed, expired or confirmed, nor whom it is owed to. Those are Flume's to answer (Flume's 43). The entry is the one the runner reads as its launch's place everywhere else, `facts::entry`, for its statuses and its landings.

Alternatives:

- **Reading the key's `<kind>--<name>/<token>/` against the place.** Not taken. The key's form is Flume's, and the status names each end's delivery outright.
- **Leaving it to Flume.** Flume cannot judge it. The key is under an end, and the recipient may be owed it.
- **No check.** A control plane's confirm under one launch's lease could confirm another account's end, as that account's reader, on a host they share.

### 4. A confirm as the runner, or of no key, is refused

`Work::act`, once the act's launch is the runner's and its lease is live, refuses a confirm whose recipient is `flume::RECIPIENT`, `boomdrive`. It also refuses one with no key. Each is answered at once, running no command, so its answer turns on no round:

- as the runner: `boomdrive is the runner's own name, whose ends the runner confirms itself`;
- of no key: `a confirm naming no delivery key confirms nothing`.

Why:

- The runner's ends are its own to carry and collect once its report holds them (core.54), and this bolt's other unit confirms them once its control plane has them. A control plane confirming as the runner could have Flume let go of a tidied place's record before the runner carried its end.
- `flume confirm` needs a key. Refusing an act that names none gives the control plane the runner's words in place of Flume's usage text.

Alternatives:

- **Only the recipients the launch named.** Not taken. Flume may owe an end to a reader the launch did not name, such as a stall's to its stream's answerer. Whom an end is owed to is Flume's to answer (Flume's 43).

### 5. A confirm of a launch not launched is refused

`act_unlaunched` refuses a confirm as it refuses a retry, a land or a done, naming where the launch stands: held on its After, refused, or done by a drop. It runs no command. A launch not launched has no place, so no end of it is owed to anyone.

### 6. crew's implementation reads `Act: confirm`

- **`read::want`** gives `Some("confirm")` as `act(ActSort::Confirm { recipient, deliveries })`:
  - `recipient` is its one `Recipient`. None is `it names no Recipient`, and two is `it names Recipient twice`, since a confirm is made as one reader;
  - `deliveries` is each `Delivery` in the order written. None is `it names no Delivery`;
  - it reads no other field. The `Recipient` is passed as crew wrote it: crew's rules hold it to a role's address, and the runner judges no recipient but its own name (Decision 4).
- **`lib.rs`**'s split of a request naming no host sends a `confirm` with the `retry`, `land`, `drop` and `priority`. So a confirm naming no host goes to the runner that holds its launch, and to no other. One that cannot be read is given nothing and named in the log, as any act naming no host is.
- **`tests/common/mod.rs`**'s copy of crew's `requests.rec` descriptor gains `confirm` in its `Act` enum and `Delivery` in its `%allowed`, and its `FIELDS` gains `Delivery` after `Words`, as crew's `runner.REQUESTS` and `runner.FIELDS` hold them at crew's main fa8501c1.
- `Request::of`'s documentation names a confirm among the acts whose `Of` it reads.

Alternatives:

- **Reading every `Recipient` and confirming as each.** Not taken. crew writes one, and a confirm made as two readers would be two confirms. A second is refused, never dropped.

### 7. Version 1 carries no confirm

Version 1 is unchanged: no file under `protocol/v1/` changes. Its `act_sort` names no `confirm` (core.42), and its copy of an end names no delivery key, so no hosted control plane could name one. A want whose act is a confirm fails version 1's `want`, so a confirm never reaches the hosted implementation's types. The runner never writes a want, and `Acted` names no sort, so nothing the runner sends changes.

`tests/interface.rs` shows it. `the_interfaces_values_pass` keeps every act sort version 1 names. A new test, `a_confirm_is_no_act_of_version_1`, checks a want whose act is a confirm against version 1's `want` and finds it refused.

Alternatives:

- **Adding `confirm` to version 1.** Not taken. A release has spoken version 1, and core.42 rules out changing it.
- **Version 2 now, naming `confirm` and an end's delivery.** Not taken. A version is the runner's and the cloud's to make together, with its copies, examples and tests, and the cloud pins the protocol by revision (*Named follow-ups*).

### 8. No floor moves here

`flume confirm` is Flume's from `v0.21.0`. This change moves neither `flume::LEAST` nor the set's Flume:

- This bolt's other unit, `the-runner-confirms-an-end-once-its-control-plane-has-it`, has the runner confirm its own ends, which needs the same release. Its proposal moves `flume::LEAST` to `0.21.0` and the set's Flume to `v0.21.0` (`77d5b93`), with the lock, the git crates' hashes and README's override example. That is one piece of work for the bolt. Two units doing it would conflict on every one of those files.
- A control plane names a key only from a status Flume gave, and a Flume older than `v0.21.0` gives none. So no confirm reaches such a Flume but on a host whose Flume was rolled back after it gave keys. There, Flume's refusal of the command it lacks is the act's answer, as for any act Flume refuses (core.1, core.11). The end stays owed, and Flume shows it once it expires.
- The release that carries the bolt names the least Flume it runs with, as core.38 asks.

Alternatives:

- **Moving `LEAST` and the set's Flume here too.** Not taken, for the conflict above. If the bolt's other unit does not move them, the bolt's verify finds the set's Flume without `confirm`, and that is told to the orchestrator.
- **A floor of the confirm's own, leaving a confirm unanswered below `0.21.0`.** Not taken. It is a second floor beside `LEAST` that the bolt's other unit makes redundant, and it guards a host that cannot be reached with a key.

### 9. The rule and the record

- **core.56**, under *Work under a lease*: a control plane asks a reader's ends confirmed with `confirm`, an act naming its launch, the one recipient it confirms as and each delivery key, under that launch's lease. The runner carries it out as any act, on the host that holds the launch, by Flume's confirm of exactly those keys as that recipient, once each key is the delivery of an end of the launch's place as Flume's status gives it, and never as its own name. It confirms no end of its own accord for a reader its control plane did not name, and Flume's answer is the confirm's. Its details name what it rules out and what it leaves.
- **Record 0002's amendment of 2026-11-09** records the direction, quoting the user's words from crew's record 0056 and weir's record 0006, and the stage's choices, Decisions 3, 4, 7 and 8. It is proposed until the user sees it as a record.

### 10. The README

The README's control pass says the runner carries out each retry, land, confirm, drop, priority or done as Flume's own command on the launch's place. It says a confirm is `flume confirm --recipient=<recipient> -- <key>…`, of exactly the keys its control plane names, each the delivery of an end of the launch's place, as the reader it names and never as `boomdrive`. It says the runner confirms no end for another reader of its own accord.

## Who owns what

| piece | owner | cited by |
|---|---|---|
| which ends a reader acted on, and when it confirms them | the control plane (core.1); crew, for its role runs | the runner |
| the message `confirm`, naming its launch, its recipient and its keys | the interface, `boomdrive-control` (core.2, core.56) | crew's implementation, the runner |
| `Act: confirm` in crew's requests | crew, which writes it (crew's change `crews-role-runs-confirm-what-they-collect`) | crew's implementation, which reads it |
| how a confirm is carried out, and each key held to its place | the runner (core.56) | crew, the cloud |
| what a delivery is, who may confirm it, expiry | weir (record 0006) | Flume |
| a place's deliveries and `flume confirm` | Flume (record 0061; 43) | the runner |
| the runner's own confirmation, as `boomdrive` | the runner, this bolt's other unit (core.54) | the control plane |
| the least Flume and the set's Flume | the runner (core.38, core.44); this bolt's other unit moves them | this change |
| a confirm in the hosted protocol | a later version, the runner's and the cloud's together (core.42) | the hosted implementation |

## Named follow-ups

- **This bolt's other unit, `the-runner-confirms-an-end-once-its-control-plane-has-it`:** moves `flume::LEAST` and the set's Flume to `v0.21.0`, the first release with `flume confirm`, once for the bolt (Decision 8).
- **agentplot/boomdrive-cloud and this repository: a version that names `confirm`.** It names the act, and an end's `delivery`, `owed` and `confirmed` in the copy of a place's status, so a hosted control plane's readers can name the keys they confirm. Until then a hosted control plane's readers confirm nothing through the runner, and their ends stay owed and expire visibly.
- **agentplot/blueprints: Flume's statement, the runner's row of its callers.** It names "a retry of a base place's submission its control plane asks (203)". It gains a confirm its control plane asks, as the reader it names, of ends of a place the runner launched (43). That is the statement's owner's to amend.
- **agentplot/crew:** nothing. crew posts `confirm` since its main at 271e2c4, and its change's *Dependencies* name this runner half. Its confirms written before a host runs this release are carried once it does, and a key Flume let go of by then is refused, which crew's Decision 5 leaves harmless.

Each is told to the orchestrator.

## Risks / Trade-offs

- **A confirm Flume refuses part of.** Flume confirms the keys before the one it refuses, and the act is answered `refused` with Flume's words, which name them. crew takes none of the confirm's ends again, so a key after the refused one stays owed until it expires. Flume then shows it, and the operator confirms it as that recipient (Flume record 0061).
- **The backlog on upgrade.** A runner that starts carrying confirms finds every confirm crew wrote meanwhile. Each is an act on a place, so once the next pass is due a pass runs one and leaves the rest to the passes after (core.50).
- **A confirm on a place relaunched at the same location.** `facts::entry` gives the place standing there before a record kept tidied. A confirm of a key of the tidied record is then refused while a newer place stands at that location. A control plane sees the runner's words, and the end expires visibly.
- **A hosted control plane cannot confirm** until a later version (Decision 7).

## Migration Plan

Nothing held changes shape. `report.json` keeps every field, since `Acted` names no sort (core.36). No protocol file changes (core.42).

- **crew's confirms written before this release.** A runner of a release before this one gives a confirm naming no host nothing and names it in its log. crew's confirms name no host. Once the runner runs this release, it carries each one (Risks).
- **A runner rolled back** after it answered a confirm answers it from its report, by id, and runs nothing again (core.10). A confirm it never answered is read as above.

## Open Questions

None.

## Task notes

- **1.1.** `tests/json.rs`'s `an_act_of_each_sort`:
  - adds `ActSort::Confirm { recipient: "scratch/planner", deliveries: ["change--a/0a1b2c3d/stage/code/1/delivered", "change--a/0a1b2c3d/submission/1/landed"] }` to its sorts;
  - asserts that `serde_json::to_value` of that confirm's `Act` gives `"what": {"confirm": {"recipient": "scratch/planner", "deliveries": [the two keys, in that order]}}`.
- **2.1.**
  - `src/flume.rs`'s `Act` gains `Confirm { recipient, keys }`, documented: Flume's confirm of each key as the recipient, naming no place, since Flume's key names its own.
  - `act_args(place, Act::Confirm { .. })` is `["confirm", "--recipient=<recipient>", "--", <key>…]`. Its documentation says the place is not passed for a confirm.
  - In `src/carry.rs`, `flume_act` maps `ActSort::Confirm` to it, and `sort` names it `confirm as <recipient>`. `act_unlaunched` refuses it with the retry's words, which is Decision 5 and lets the crate build.
  - `tests/flume.rs` gains `a_confirms_argv`. Over the stub, it runs `cli.act("/places/P", &Act::Confirm { recipient: "scratch/planner", keys: ["K1", "K2"] })` and finds the call `--json confirm --recipient=scratch/planner -- K1 K2`. It also runs a confirm whose recipient is `-x` and whose keys are `--recipient=boomdrive` and `K1`, and finds `--json confirm --recipient=-x -- --recipient=boomdrive K1`.
  - The module's documentation names a confirm among the acts.
- **2.2.**
  - `tests/fakes/mod.rs`'s `FakeFlume::act` records a confirm's call as any act's, answers it from `act_answers` by its place, else `{"confirmed": {"recipient": <recipient>, "confirmed": [{"key": <key>, "at": 1794150000, "before": false}…]}}`, and changes no place.
  - A function `fakes::ends(deliveries: &[&str]) -> Value` gives one end per key: `{"of": "stage", "key": "stage/code/<i>/delivered", "delivery": <key>, "owed": [{"recipient": "scratch/planner", "expired": false, "known": true}], "confirmed": [], "pending_for": ["scratch/planner"]}`.
  - A test sets a place's ends with `bench.flume.set(&place_of("a"), "ends", fakes::ends(&[K1, K2]))`.
- **2.2 to 2.5** live in a new `tests/confirms.rs`. It opens with a module comment naming the work-into-flume spec's three confirm requirements and core.56, and holds `mod fakes;`. Its keys are neutral: `K1` is `change--a/0a1b2c3d/stage/code/1/delivered`, `K2` is `change--a/0a1b2c3d/submission/1/landed`, and `K3` is `change--b/4e5f6a7b/stage/code/1/delivered`. Each test makes `a` with a first pass over `Fake::holding(vec![Want::Offer(offer("a", "a"))])`, sets its ends, then pulls `a` and its act, as `acts.rs`'s `an_act_on_a_place_the_listing_does_not_hold_which_flume_stands` does.
  - **2.2 `a_confirm_names_only_the_keys_it_was_given`:** `a`'s ends are `K1` and `K2`, and `a/confirm` confirms `K1` as `scratch/planner`. `bench.flume.acts()` is exactly `[(P, Act::Confirm { recipient: "scratch/planner", keys: [K1] })]`. The act is `Done` with no words, and the lines hold `a/confirm: confirm as scratch/planner of a at P: done`.
  - **2.2 `flumes_refusal_is_the_confirms_answer`:** `bench.flume.act_answers(&P, Ok(Reply::Refused(words)))`, the words the spec's. The act is `Refused` with those words.
  - **2.3 `a_key_of_no_end_of_the_launchs_place_is_refused`:**
    - `a` and `b` are made, `a`'s ends `K1` and `b`'s `K3`;
    - `a/confirm` confirms `K1` and `K3`;
    - the act is `Refused` with words naming `K3` and `P`;
    - `bench.flume.acts()` holds no confirm.
  - **2.4 `a_tidied_places_record_is_confirmed`:** `a`'s ends are `K1`, then `bench.flume.tidy_held(&P, 1794150000, &["scratch/planner"])`. The listing holds `P@1794150000`, and `a/confirm` of `K1` runs Flume's confirm at `P` and is `Done`.
  - **2.4 `a_place_the_listing_does_not_hold_is_held_to_its_status`:**
    - the listing is scripted with no places;
    - `status_answers(&P, …)` gives `{"status": {"places": [{"place": P, "ends": fakes::ends(&[K1])}]}}`;
    - `a/confirm` of `K1` reads `status P`, then runs the confirm, and is `Done`;
    - a second confirm, `a/confirm-2` of `K3`, is `Refused` naming `K3` and runs no confirm.
  - **2.5 `a_confirm_as_the_runner_is_refused`:** `a/confirm` of `K1` as `boomdrive` is `Refused` with words naming `boomdrive`, and no confirm is run.
  - **2.5 `a_confirm_of_no_key_is_refused`:** `a/confirm` of no key as `scratch/planner` is `Refused` with words saying it names no delivery key, and no confirm is run.
- **2.6.** `tests/after.rs`'s `a_confirm_of_a_held_launch` follows `a_retry_or_a_done_of_a_held_launch`. It pulls `a`, `b` after `a`, `b/retry` and `b/confirm`, a confirm of `K1` as `scratch/planner`. The confirm's words equal the retry's, and `bench.flume.acts()` is empty.
- **3.1.** `tests/read.rs`'s `a_confirm_is_an_act_on_the_launch_it_names` writes request 12, a launch naming `host-a`, and request 17, `act(17, "confirm", Some("host-a"), "unit/a", 12, &[("Recipient", "scratch/planner"), ("Delivery", K1), ("Delivery", K2)])`. It finds the pull's want `home/17` to be `Act { launch: home/12, lease: 1, what: ActSort::Confirm { recipient: "scratch/planner", deliveries: [K1, K2] } }`.
- **3.2.** `tests/read.rs`'s `a_confirm_naming_no_recipient_two_or_no_delivery_is_refused` writes requests 18, with a `Delivery` and no `Recipient`, 19, with a `Recipient` and no `Delivery`, and 20, with two `Recipient` lines and a `Delivery`, each naming `host-a` and `Of: 12`. Each is given as `read::refused` gives it, as `a_preview_with_no_revision` reads `home/32`. 18's launch id names `it names no Recipient`, 19's `it names no Delivery`, and 20's `it names Recipient twice`.
- **3.3.** `tests/claims.rs`'s `a_confirm_naming_no_host_goes_to_the_runner_holding_its_launch` follows `a_land_naming_no_host_goes_to_the_runner_holding_its_launch`:
  - requests 26, `act(26, "confirm", None, "unit/a", 21, &[("Recipient", "scratch/planner"), ("Delivery", K1)])`, and 27, the same of `Of: 20`;
  - `host-b` answers launch 20;
  - `host-a`'s pull ids are `home/21` and `home/26`, and `home/26` confirms `K1` of `home/21` as `scratch/planner`.
- **4.1.** `tests/interface.rs`'s `a_confirm_is_no_act_of_version_1` builds `Want::Act(Act { id: act-1, launch: offer-1, lease: 3, what: ActSort::Confirm { recipient: "scratch/planner", deliveries: [K1] } })`. It checks its JSON with `common::refused` against `validator(&definition("wants.schema.json", "want"))`. The documentation of `the_interfaces_values_pass` says every act sort version 1 names passes, a land and a confirm apart.
