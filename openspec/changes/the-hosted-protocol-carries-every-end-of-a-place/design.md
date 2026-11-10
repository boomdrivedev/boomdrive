# Design

## Context

See proposal.md for why. The decision is record 0037, with core.42's and core.27's details, written with this change.

What stands:

- **This branch** was cut from `main` at `ecbe5fa`. That commit holds none of the confirm this change carries.
- **`bolt/the-runner-confirms-the-ends-it-carries`**, at `f3f693b`, holds the changes `the-runner-carries-a-control-planes-confirm` (#67) and `the-runner-confirms-an-end-once-its-control-plane-has-it` (#69), and records 0002's and 0030's amendments of 2026-11-09, accepted by the user (#75):
  - `boomdrive-control`'s `ActSort::Confirm { recipient, deliveries }`, written `{"confirm":{"recipient":…,"deliveries":[…]}}`, which the runner carries out by `flume --json confirm --recipient=<name> -- <key>…`, refusing one that names no key, names `boomdrive`, names a key of no end of its launch's place, or acts on a launch held unlaunched (core.56);
  - `Reported::statuses_taken`, which the runner confirms its own ends by (record 0030 as amended, core.54);
  - the hosted implementation's `Partition::upward`, which puts `null` for each status or entry whose copy fails version 1 and keeps those in `failing`, and `statuses_up`, which names taken each launch with a status but those in `failing`;
  - `tests/interface.rs`'s `a_confirm_is_no_act_of_version_1`, and the runner-protocol spec's "Version 1 carries no confirm";
  - the set's Flume at `v0.21.0`, and `flume::LEAST` `0.21.0`;
  - core.54 as amended and core.56.
  Its core.54 conflicts with `main`'s line, which `main` changed to name record 0030 accepted. Its records 0002 and 0030 conflict with this branch's, and equal `main`'s at `cee700c`.
- **Version 1**, `crates/boomdrive-hosted/protocol/v1/`, ten schema files and 28 examples, spoken by every release from `v0.1.0`, the first that dials a hosted control plane, to `v0.7.3`, the newest, and unchanged across them. `src/protocol/v1.rs` holds `VERSION` 1, `SUBPROTOCOL` `boomdrive.v1`, `ADDRESS`, `FILES` and the runner's message types. `key.rs` makes each context from `SUBPROTOCOL`. `check.rs`, `copy.rs`, `connection.rs`, `enrollment.rs` and `plane.rs` use version 1's module. The tests' `common` module builds its validators from version 1's `FILES`, and `common::fake` speaks `boomdrive.v1`. `tests/signatures.rs` reads the worked bytes from `protocol/README.md`.
- **The copy's cut**, `copy::cut`, walks a definition: a closed object keeps what it names, a `oneOf` cuts by the first branch the value fits, by a constant or by its keys, and a value that fits no branch is kept whole so the check fails. `copy::place_status` cuts and checks the whole place, and fails whole.
- **Flume `v0.21.0`** (`77d5b93`) gives each end of a place in status as its `EndStatus`: the end's `of` and its fields (`stage`, `start`, `state`, `end`; `submit`, `verdict`; a stall's `since`, `waiting` and the rest; or `over`), `read_at`, `deliveries` (each hand-over, `{recipient, at, by}`, `by` one of `notice`, `wait`, `inbox`, `session_start`), `key`, `pending_for`, `owed` (`{recipient, expired, known}`), `confirmed` (`{recipient, at}`), and, where Flume has them, `delivery`, `expires_at` and a stop-short's `open_until`. A drop's or a done's end is `{"of":"over","over":"dropped"|"done",…}`, keyed `over/dropped` or `over/done`. Every top-level field version 1's copy requires is still printed.
- **boomdrive-cloud** keeps its own copy of version 1, `workers/control-plane/runner-protocol/v1/`, and checks a runner's messages against it.

## Goals / Non-Goals

**Goals:**

- A hosted control plane reads every end Flume 0.21.0 gives a place from the runner's report, each with its delivery's key, its expiry and its readers.
- A hosted control plane confirms a reader's ends as crew's does, through the same act.
- An end a version does not name loses only itself, never the rest of its place's status or any other place's.
- Version 1 stays exactly as releases spoke it.

**Non-Goals:**

- **A land or a preview in the protocol.** Neither is in the brief. Each stays out of version 2, and waits for a version that names it (core.53, core.55; Open Questions).
- **The cloud's side.** boomdrive-cloud takes version 2 by a change of its own.
- **Any change to the interface, crew's implementation or the runner's passes.** The confirm is carried as the bolt carries it.
- **Speaking version 1.** The runner speaks version 2 alone (Decision 4).

## Decisions

### 1. Version 2 is version 1 whole, but for its ends and its confirm

`crates/boomdrive-hosted/protocol/v2/` holds version 1's ten files and its examples, each file's `$id` under `v2/`, and every envelope's `protocol_version` `const` 2. Its differences from version 1 are these, and no others.

`place-status.schema.json`:

- `end_record` is a `oneOf` of five forms: `stage`, `submission`, `stall`, `over` and `end_delivery`, in that order.
- Each of the four named forms keeps version 1's fields and requires, beside them, `key` (a string), `owed` (an array of `owed`) and `confirmed` (an array of `confirmation`), and takes `delivery` (a string) and `expires_at` (`seconds`) where Flume prints them.
- `over` is `{"of":{"const":"over"},"over":{"$ref":"#/$defs/over"},…}`, with `read_at`, `deliveries`, `pending_for` and the delivery's fields as above.
- `owed` is `{"recipient": string, "expired": boolean, "known": boolean}`, closed and each required. `confirmation` is `{"recipient": string, "at": seconds}`, closed and each required.
- Version 1's `delivery` definition, a hand-over, is named `handover`, since the field `delivery` now holds a delivery's key (core.12). Its `by` is one of `notice`, `wait`, `inbox` and `session_start`.
- `end_delivery` is an end's delivery alone: `of` (any string, as Flume printed it), `key`, `read_at`, `owed`, `confirmed` and `pending_for`, each required, and `delivery` and `expires_at` where Flume prints them. It is closed and has no `deliveries`. Every named form requires `deliveries`, so no end passes two forms, and `oneOf` holds.
- Nothing else of an end goes up: no stall's `since`, `waiting`, `answerer`, `stop_short` or `lapsed`, no stage end's `answered`, `quiet_from` or `open_until`, and none of `end`'s `addressee` or `report` (core.27).

`wants.schema.json`:

- `act_sort` gains `{"confirm":{"recipient": string, "deliveries": [string]}}`, closed, both required, as the interface writes `ActSort::Confirm`. The schema bounds neither, so a confirm the runner refuses (core.56) reaches the runner and is refused as an act, not the whole `pulled` as a message.

Version 2 names no land and no preview: `act_sort` and `want` take neither, and a report has no `previews`.

Not taken: changing version 1, which a release has spoken (core.42); carrying each end whole as Flume printed it, which carries fields because Flume printed them (core.27).

### 2. Each end is copied on its own

`copy::place_status` copies each end of the place on its own, in order:

1. where its `of` is a named form's, `stage`, `submission`, `stall` or `over`, the end cut by that form and checked against `end_record`;
2. where its `of` is none of those, or that check fails, the end cut by `end_delivery` and checked against `end_record`;
3. where that fails too, the end left out of `ends`.

It cuts the rest of the place by `place_status`, puts the ends' copies in `ends` in their order, and checks the whole. It gives the copy with the index of each end it carried by its delivery alone, by step 2, or left out, by step 3. Where the place fails its check with its ends so copied, the failure lies outside `ends`, and the copy fails whole, as version 1's does: the status goes up `null`.

An end Flume 0.21.0 prints in a form version 2 names passes step 1. Step 2 serves an end of a later Flume, or one whose fields hold a value version 2 does not have, such as a stage's state. Step 3 serves an end with no `key`, as Flume 0.4.1 printed every end, so the tree's Flume 0.4.1 fixtures go up under version 2 with their ends left out, which the runner, running Flume 0.21.0 or later, never meets on a host.

`Partition::upward` says one line for each launch whose copy carried an end by its delivery alone or left one out, once while it lasts, as it says a failing copy now: `<id>: its status goes up with ends version 2 does not name: /ends/<n> by its delivery alone, /ends/<m> left out`. The line names paths and never a value (core.11).

Not taken: the status `null` whole, as under version 1, which loses the place for one end; the end left out at once, which loses its delivery, so the control plane could neither show it owed nor confirm it for its reader.

### 3. A status is taken only where it went up whole

`Partition::upward` keeps, beside `failing`, the launches whose copy carried an end by its delivery alone or left one out, `partial`. `statuses_up` names each launch with a status, but those in `failing` or `partial`. The answer to the last report sent names those taken, as now.

So an end owed to the runner that went up only as its delivery, or not at all, holds back its place's whole status, and every end of that place owed to the runner stays owed in Flume (record 0030 as amended). The control plane holds its delivery, which is not what ended, so the runner does not let Flume's copy go.

Not taken: naming taken every status that went up. Then a confirm would let go an end whose content the control plane never held.

### 4. The runner speaks version 2 alone

`protocol::v2` holds `VERSION` 2, `SUBPROTOCOL` `boomdrive.v2`, `ADDRESS` under `v2/`, `FILES` and every message type, moved from `protocol::v1`. Every module of the crate uses version 2: `check.rs`'s validators, `copy.rs`'s files, `connection.rs`'s upgrade and envelope, `enrollment.rs`'s `enroll`, `key.rs`'s contexts and `plane.rs`'s copies. Every word the crate says of version 1, in documentation or in a line, names version 2.

A control plane that does not take `boomdrive.v2` refuses the upgrade. The runner says so once and tries again at the backoff's cap, as it does any subprotocol refused (core.22, core.23). Its words name `boomdrive.v2`.

Not taken: offering `boomdrive.v2` and `boomdrive.v1` and speaking the one picked. A control plane keeps the current version and the one before (the README; record 0015), so the overlap is the control plane's to keep, and two versions' types in the runner would double what every change to it touches.

### 5. Each context is named by its version

`key.rs` makes each context from `SUBPROTOCOL`, so version 2's contexts start `boomdrive.v2`: `boomdrive.v2 enroll`, `boomdrive.v2 connect` and `boomdrive.v2 terminal share`. A key enrolled under version 1 stays the partition's, and is never enrolled again. Its next connection proves it under version 2's context.

Not taken: version 1's prefix kept in every version, which ties the contexts to a name the version no longer has.

### 6. Version 1 stays as a release spoke it

`protocol::v1` keeps `VERSION`, `SUBPROTOCOL`, `ADDRESS` and `FILES`, and no type. Its directory and its examples stay.

- `tests/files.rs` checks each version's files as it checks version 1's now: each a schema of its draft whose `$id` is its own address, every `$ref` resolved within the version, every file listed in its `FILES`.
- `tests/examples.rs` checks that each example of version 1 passes its direction's envelope of version 1. Version 2's examples are read into version 2's types and written back.
- `version_1_is_as_released`, in `tests/files.rs`, holds each file under `protocol/v1/`, examples among them, to its SHA-256 as release `v0.7.3` holds it, listed in the test. Any edit fails, naming the file.
- `tests/interface.rs`'s tests that version 1 carries no land, preview or confirm stay, checked against version 1's validators.

### 7. The README describes version 2

`protocol/README.md` describes version 2 as it describes version 1 now. Every message names `protocol_version` 2. The subprotocol is `boomdrive.v2`, and each context and worked example is version 2's, computed by the crate's own signing and checked by `tests/signatures.rs`. Its copies section names each end's delivery, the end of a place's work, the hand-over at a session's start and an end's delivery alone. Its messages table names a confirm among the acts a `pulled` holds. A section **Version 1** says which releases spoke it, that a control plane may still take it, and how it differs: its subprotocol and contexts, no confirm, and ends with no delivery, no `over` form and no `session_start`. The README keeps saying a control plane accepts the current version and the one before.

### 8. A control plane's confirm is carried as crew's

A `pulled` holding an act whose `what` is a confirm passes `to-runner.schema.json` of version 2 and is read into `Want::Act` with `ActSort::Confirm`. `own_wants` gives it to the runner as any act, its id and launch under the partition's prefix, its recipient and keys as given. The runner carries it out as core.56 says, the same code for every implementation (core.2). The hosted implementation reads nothing of it, and refuses nothing of it.

`boomdrive-control`'s `ActSort::Land`, `Confirm` and `Preview` documentation names version 2: version 2 carries a confirm, and no land and no preview.

### 9. Flume 0.21.0's answers

`crates/boomdrive/tests/fixtures/flume-0.21.0/` holds what the set's Flume printed on a scratch host, byte for byte but the scratch directory's prefix, as the 0.4.1 fixtures write it, with a README saying how each was read, as `flume-0.17.1/` does. treefmt already excludes `crates/*/tests/fixtures/**`. It holds at least:

- `status-dropped-held.json`: `flume --json status <location>@<at>` of a tidied place, dropped, launched naming `scratch/planner` and `boomdrive`, its drop's end pending for `boomdrive` alone, since Flume holds a drop's or a done's end for no role's address;
- `status-done-held.json`: the same of a place done.

`tests/cut.rs` cuts each place of them by version 2: each passes, keeps each end's `key` and `delivery`, and carries no end by its delivery alone. It cuts Flume 0.4.1's places by version 2 as well, which still try every field of a place but its ends: each passes, each of its ends left out.

The hosted tests that build a status from Flume 0.4.1's `status-code.json`, `tests/plane.rs`'s `status_in` among them, give its ends the fields Flume 0.21.0 prints of them, `key`, `owed` and `confirmed`, so a status that passes version 2 goes up whole.

### 10. The branch takes the confirm bolt first

This change builds on the confirm bolt: its act, its statuses taken, its Flume and its rules. Task 1.1 merges it, as `the-runner-confirms-an-end-once-its-control-plane-has-it` took its bolt by a merge (its task 4.4). Where `origin/main` holds the bolt's landing, the branch merges `origin/main`. Otherwise it merges `origin/main` and then `origin/bolt/the-runner-confirms-the-ends-it-carries`. Each conflict in records 0002 and 0030, or in core.54, takes the bolt's side, which is the text the user accepted on 2026-11-09. This change's details of core.42 and core.27 are lines the bolt does not touch, and merge as they are.

The specs of this change are written over the bolt's specs. openspec validates them now, and they archive once the bolt is merged.

### Rules relied on

- **core.42**: version 2 is a new directory beside version 1, which never changes, and each end checks what it is sent against its version.
- **core.27, core.11**: the copy names each end's delivery fields, and no words of a stall, a stop-short or a hold.
- **core.12**: a hand-over's definition is named `handover`, since `delivery` holds a key.
- **core.2**: the confirm is carried by the runner's own code for every implementation, and the protocol lives only in the hosted implementation's crate.
- **core.54**: the runner confirms only what its control plane took, which Decision 3 keeps for a status with an end carried by its delivery alone.
- **core.56**: the confirm's form and the runner's refusals of it.
- **core.22, core.23**: a subprotocol refused is said once and tried again on the backoff.

### Rules departed from

None.

## Risks / Trade-offs

- **[A control plane that speaks version 1 alone cannot be reached by this release.]** → boomdrive-cloud pins version 1 today. A host under it says once that the control plane does not speak `boomdrive.v2`, and tries again on its backoff. The cloud takes version 2 by its own change. No host is known to run a hosted partition now. The orchestrator asks the cloud.
- **[Version 2 is fixed from its first release.]** → A release follows every push to `main` (record 0024), so version 2 freezes when this bolt lands. A land, a preview or another field is then a version 3 (Open Questions).
- **[A status with an end carried by its delivery alone is never taken while that end stands.]** → Each end of that place stays owed in Flume, which shows each expired past its span, as version 1 holds today a drop's or a done's. A version that names the end mends it.
- **[The fixtures need a place with no agent on a scratch host.]** → A place of a kind that does not land, its one stage held at its launch, starts no agent, and Flume 0.21.0 takes both a drop and a done of it. A kind with no stages lands, and Flume refuses a done of a place whose kind lands. Where the set's Flume refuses every such place on a scratch host, task 5.2 stops short rather than write a fixture by hand (Task notes 5.2).
- **[The confirm bolt conflicts with `main`.]** → Its core.54 and its records 0002 and 0030 conflict line by line, and each takes the bolt's text (Decision 10).

## Migration Plan

Nothing on a host moves. A host's enrolled keys stay, and its next connection proves them under version 2. A host whose hosted control plane takes version 2 is reached as before. One whose control plane does not is unreachable until the control plane takes it. The release's notes need name no move by hand, since no file the runner keeps changes (core.36).

## Open Questions

- **Whether version 2 should also carry a land and a preview** (core.53, core.55). The brief names neither, so this change leaves them out. Once version 2 is released, they are a version 3. This is the planner's to decide before the bolt lands.

## Task notes

- **1.1** After the merge, `cargo test -p boomdrive --test confirms` passing shows the bolt's confirm on this branch. `git log --oneline -1 --merges` names the merge.
- **2.1** Copy `protocol/v1/` to `protocol/v2/` with `cp -R`, then set each `$id` and each `protocol_version`. Make the tests' `common` module take a version, `common::v1` and `common::v2` or a `Version` argument, each building its validators from that version's `FILES` alone, so `tests/files.rs` loops over both. Until task 3.1 the other tests keep checking version 1.
- **2.2** Read each digest from `git show v0.7.3:<path> | shasum -a 256`. The test reads the files from disk under `CARGO_MANIFEST_DIR`, and lists every file it finds there, so a file added to `v1/` fails as well.
- **3.1** Move `src/protocol/v1.rs`'s types into `src/protocol/v2.rs` whole, then set the constants. `common::fake` speaks `boomdrive.v2` and makes its contexts from it. The README's two worked signatures and `examples/from-runner/enroll.json` take the bytes `tests/signatures.rs` computes: run it, read the failure, and write the bytes the crate made. `a_message_of_another_version` checks a `hello` naming 1 against version 2 and one naming 2 against version 1.
- **4.1** Every end of version 2's examples gains `key`, `owed` and `confirmed`, and a `delivery` and `expires_at` where its recipients make one, as Flume 0.21.0 prints them. `copies.rs`'s `PLACE_STATUS` gains `ends[].key`, `ends[].delivery`, `ends[].expires_at`, `ends[].owed`, `ends[].owed[].recipient`, `ends[].owed[].expired`, `ends[].owed[].known`, `ends[].confirmed`, `ends[].confirmed[].recipient`, `ends[].confirmed[].at` and `ends[].over`, with the `over` value's own paths, as its list names variants.
- **4.4** Examples to add: `report-a-dropped-place-held-for-the-runner.json`, `report-a-done-place-held-for-the-runner.json`, `report-an-end-handed-over-at-a-sessions-start.json`, `report-an-end-owed-expired-and-one-confirmed.json` and `report-an-end-by-its-delivery-alone.json`.
- **5.2** On a scratch host as `tests/family.rs`'s `Family` makes one, with the set's Flume: a scratch git repository with a bare origin; a kind that does not land, of one stage, declared by the launch itself with `--kinds`; a source of the host whose roles file declares `scratch/planner` carried to another host, so the launch may name it and no run of it wakes; the host's XDG folders short, since Flume names a place's state file by its whole path joined by `%`, which a deep scratch folder takes past the 255 bytes a name may take; `flume --json launch` naming `--hold "code=<words>" --recipient scratch/planner --recipient boomdrive`; `flume --json drop` of one place and `flume --json done` of another; `flume --json settle --every 60s` to tidy each; and `flume --json status <location>@<at>` of each record. Where Flume refuses every such place on a scratch host, stop short with `flume needs`, quoting its refusal.
- **5.3** `an_end_flume_adds` carries a place whose ends hold one of `"of":"moved"`, one stage end whose `state` version 2 does not have, and one that lacks `key`, beside one that passes: the first two go up by their delivery alone, the third is left out, the fourth goes up whole, the rest of the place goes up, one line names the launch and each of the three paths `/ends/<n>`, and no line names a value.
- **6.2** `a_confirm_carried` has the fake control plane give `home` an offer `l-1` and an act `c-1` on it, `{"confirm":{"recipient":"scratch/planner","deliveries":["change--a/0a1b2c3d/stage/code/1/delivered"]}}`, and asserts the part's pull gives `Want::Act` `home/c-1` on `home/l-1` with that recipient and key, and no `message_refused` was sent.
