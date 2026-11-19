# Design

## Context

See proposal.md for why. The decision is record 0052, with core.42's and core.59's details and record 0002's amendment of 2026-11-14, each written with this change. No area page binds the paths this change touches, so `docs/architecture/core.md` binds every one. The change relies on these rules:

- core.2: only the hosted implementation names the hosted protocol, and the runner carries a hold with the same code for every implementation;
- core.10: each act is carried out once, under its launch's lease;
- core.11, core.27: no line or failure names a value of a message, paths alone;
- core.22, core.23: a subprotocol refused is said once and tried again at the backoff's cap;
- core.26: a partition's wants are its own, under its prefix;
- core.42: version 3 is a new directory beside versions 1 and 2, which never change, and each end checks what it is sent against its version;
- core.51: a hold of a place Flume no longer stands is answered by Flume's refusal of its status, as any act;
- core.59: a hold's form, and the runner's refusal of one with no words.

It departs from no rule.

### What stands

- **This branch** is `bolt/a-hosted-control-plane-can-hold-a-waiting-place` at `ebda503b`, which holds `main` and the runner bolt whose #214 (`afba153a`) built the hold for crew's control plane:
  - `boomdrive-control`'s `ActSort::Hold { words: String, bound_interval: u64 }`, written `{"hold":{"words":…,"bound_interval":…}}`;
  - the runner's `Act::Hold`, run as `flume --json hold <place> --waiting --for=<n>s -- <words>`, refusing a hold with no words and one of a launch held unlaunched;
  - core.59, and record 0002's amendment of 2026-11-14;
  - `tests/interface.rs`'s `a_hold_is_no_act_of_version_1_or_2`, and `every_sort_the_interface_names`, which finds every sort but the hold passing version 2's `want`.

  The branch also holds the chore `the-protocols-version-two-is-held-as-v0-9-0-spoke-it`: `tests/files.rs`'s `version_2_is_as_released` holds every file under `protocol/v2/` to its digest at `v0.9.0`.
- **Version 2**, `crates/boomdrive-hosted/protocol/v2/`, holds eleven schema files and forty examples. Every release from `v0.8.0` speaks it, and the newest is `v0.15.1`. Each file's `$id` is under `https://raw.githubusercontent.com/agentplot/boomdrive/main/crates/boomdrive-hosted/protocol/v2/`, the namespace version 1 has. The change `boomdrives-addresses-name-boomdrivedev` left those namespaces as spoken.
- **Version 2's `act_sort`** is a `oneOf`: the words `drop`, `done` and `land`, then a retry, a priority and a confirm, each a closed object under its name. `wants.schema.json`'s `$defs` already name `hold`, a launch's held stage, which is no act.
- **`src/protocol/v2.rs`** holds `VERSION` 2, `SUBPROTOCOL` `boomdrive.v2`, `ADDRESS`, `FILES` and every message type. `src/protocol/v1.rs` holds the four constants alone.
  - `check.rs`, `copy.rs`, `connection.rs`, `enrollment.rs`, `key.rs` and `plane.rs` use `protocol::v2`;
  - `key.rs` makes each context from `SUBPROTOCOL`;
  - the words of a failed check name `version 2` as text, as in `it fails version 2's check: /…` and `the report fails version 2's check, so it is not sent: …`;
  - `lib.rs`'s `NO_FORGE_TOKEN` says version 2 of the protocol carries no forge token yet.
- **The tests' `common` module** has a `Version` for each version: its number, address and files, its validators, examples and directory. `VERSIONS` is `[V1, V2]`, and `CURRENT`, which the free functions check, is `V2`.
  - `common::fake` names its own `SUBPROTOCOL`, `boomdrive.v2`, and makes its contexts from it;
  - `tests/worked-bytes.json` holds the worked contexts and signatures of version 2, which `signatures.rs` reads (record 0049), and `the_examples_enroll_carries_the_worked_key_and_signature` reads version 2's `examples/from-runner/enroll.json`.
- **Flume's statement**, boomdrivedev/blueprints `design/flume/requirements.md`, names in the runner's row of callers, at `7cdb007` on its `main`, "a hold of the move a place waits on it for, with its control plane's words and a bound, under the launch's lease (130)".
- **boomdrive-cloud** keeps copies of versions 1 and 2 under `workers/control-plane/runner-protocol/`, and checks a runner's messages against them. Its record 0069 waits for "the version that carries an observing hello" (this repository's record 0048), which no version carries yet.

## Goals / Non-Goals

**Goals:**

- A hosted control plane asks that the move a launch's place waits on it for be held, with its words and bound, as crew's control plane does.
- The runner carries out a hosted control plane's hold as it carries out crew's, through the same code (core.2, core.59).
- Versions 1 and 2 stay exactly as releases spoke them.
- A reader of the specs, or the cloud, can see exactly how version 3 differs from version 2.

**Non-Goals:**

- **Anything else in version 3.** A forge token, health, the runner's own commands, a terminal's certificate and an observing connection each come in a version of their own, or join version 3 before its release on this bolt (*Named follow-ups*).
- **The cloud's side.** boomdrive-cloud takes version 3 by a change of its own.
- **Any change to the interface's types, crew's implementation or the runner's carrying of a hold.** The hold is carried as the runner bolt carries it.
- **Speaking version 2.** The runner speaks version 3 alone (Decision 4).

## Decisions

### 1. Version 3 is version 2 whole, but for a hold

`crates/boomdrive-hosted/protocol/v3/` holds version 2's eleven schema files and its examples. It differs from version 2 by these, and no others:

- **Each file's `$id`** is `https://raw.githubusercontent.com/boomdrivedev/boomdrive/main/crates/boomdrive-hosted/protocol/v3/<file>` (Decision 3).
- **`from-runner.schema.json`'s and `to-runner.schema.json`'s `protocol_version`** is `{"const": 3}`.
- **`wants.schema.json`'s `act_sort`** gains a last branch, after the confirm:

  ```json
  {
    "type": "object",
    "additionalProperties": false,
    "required": ["hold"],
    "properties": {
      "hold": {
        "type": "object",
        "additionalProperties": false,
        "required": ["words", "bound_interval"],
        "properties": {
          "words": { "description": "…", "type": "string" },
          "bound_interval": { "description": "…", "$ref": "shared.schema.json#/$defs/unsigned_integer" }
        }
      }
    }
  }
  ```

  This is `ActSort::Hold` as the interface writes it. `words` describes the control plane's words saying who makes the move and how. `bound_interval` describes how long the hold holds, in whole seconds from when Flume takes it.
- **Each `description`** that names version 2 names version 3. `act_sort`'s description names a hold of the move its launch's place waits on, with the control plane's words and a bound, which the schema bounds no further. `end_delivery`'s names "a form version 3 does not name". Every other description is version 2's.

Nothing else changes: version 3's copies of a place's status, sluice's entry and its answers of a preview, its terminal and enrollment, are version 2's.

Version 3's examples are version 2's forty, each naming `protocol_version` 3, plus `examples/to-runner/pulled-with-a-hold.json`. `examples/from-runner/enroll.json` carries the signature the worked key makes over version 3's enrollment context (Decision 5).

Alternatives:

- **The other things version 2 lacks, with the hold.** Not taken. No brief asks for them, and none is designed. A version may change until a release speaks it (core.42), so a change built on this bolt before it lands may still add one to version 3.
- **Changing version 2.** Not taken: core.42.
- **The hold as a `$def` of its own, `waiting_hold`.** Not taken. A confirm, the act before it, is a branch of `act_sort` in its place, and `hold` is already the name of a launch's held stage.

### 2. The hold's words and bound are bounded no further

`words` is any string, the empty one among them, and `bound_interval` is any integer from 0 to 2^53 − 1.

- The runner refuses a hold with no words, running nothing (core.59), so one with no words reaches the runner as an act and is answered `refused` with the runner's words, as a confirm of no key is (record 0037).
- A bound of 0 is passed to Flume as given, and Flume lets that hold lapse at once (core.59).
- A bound past 2^53 − 1 fails the check, as every integer of the protocol does, since JavaScript reads it inexactly.

Alternative:

- **`minLength: 1` on the words, and `minimum: 1` on the bound.** Not taken. A whole `pulled`, every want in it, would be refused for one act the runner answers itself.

### 3. Version 3's `$id`s name `boomdrivedev/boomdrive`

The runner-protocol spec has each schema file name its own address as its `$id`. Since 2026-11-12 the repository is `boomdrivedev/boomdrive`, so version 3's files are under `https://raw.githubusercontent.com/boomdrivedev/boomdrive/main/crates/boomdrive-hosted/protocol/v3/`. `protocol::v3::ADDRESS` names it.

Versions 1 and 2 keep the `agentplot` namespace their releases spoke (core.42). Every `$ref` is relative and no check fetches a `$id`. So each version resolves within its own files, whatever its namespace, at the runner and in the cloud's copy.

Alternative:

- **The `agentplot` namespace, as version 2 kept version 1's.** Not taken. It names an organisation the repository left, and the file would not be at its own address.

### 4. The runner speaks version 3 alone

`protocol::v3` holds `VERSION` 3, `SUBPROTOCOL` `boomdrive.v3`, `ADDRESS` and `FILES`, and every message type, moved from `protocol::v2` whole. Every module of the crate uses version 3:

- `check.rs`'s validators;
- `copy.rs`'s files;
- `connection.rs`'s upgrade and envelope;
- `enrollment.rs`'s `enroll`;
- `key.rs`'s contexts;
- `plane.rs`'s copies.

`protocol::v2` keeps `VERSION`, `SUBPROTOCOL`, `ADDRESS` and `FILES` alone, as `protocol::v1` does. Its module documentation says which releases spoke it.

Every word the crate says of the version it speaks is made from `VERSION`, as `format!("it fails version {VERSION}'s check: {…}")`. So a later version changes the number in one place. That covers the words of each failed check and the enrollment's answers, `check.rs`'s retriever's refusal, and `NO_FORGE_TOKEN`. `NO_FORGE_TOKEN` is a `const &str`, so it names version 3 as text, and its documentation says a later version changes it with the number. The crate's documentation and `Cargo.toml`'s comment name version 3.

A control plane that does not take `boomdrive.v3` refuses the upgrade. The runner says so once, naming `boomdrive.v3`, and tries again at the backoff's cap (core.22, core.23), as it does for version 2 today. A `pulled` naming `protocol_version` 2 or 1 fails version 3's envelope and is answered with `message_refused`.

Alternatives:

- **Offering `boomdrive.v3` and `boomdrive.v2` at the upgrade, and speaking the one picked.** Not taken (record 0037): the control plane keeps the current version and the one before.
- **Version 2's types kept for version 2 beside version 3's.** Not taken. The types are the same but for the interface's `ActSort`, which already holds the hold, so two modules of types would differ in nothing.

### 5. Each context is named by version 3

`key.rs` makes each context from `SUBPROTOCOL`, so version 3's contexts are `boomdrive.v3 enroll`, `boomdrive.v3 connect` and `boomdrive.v3 terminal share`, each the context's bytes after it as now. A key enrolled under version 2 stays the partition's and is never enrolled again. Its next connection proves it under version 3's context.

- `tests/worked-bytes.json` takes version 3's contexts and signatures, which the crate's own signing makes from the worked key, token, origin, time and nonce. `signatures.rs` checks them.
- Version 3's `examples/from-runner/enroll.json` carries the worked key and the enrollment's signature, and `the_examples_enroll_carries_the_worked_key_and_signature` reads version 3's.
- The protocol's README shows its own copy of the bytes, kept to the file by its writers (record 0049).

### 6. A hosted control plane's hold is carried as crew's

A `pulled` holding an act whose `what` is a hold passes version 3's `to-runner.schema.json` and is read into `Want::Act` with `ActSort::Hold`. `own_wants` gives it to the runner as any act, its id and launch under the partition's prefix, its words and bound as given. The runner carries it out as core.59 says, the same code for every implementation (core.2):

- once, under the launch's lease;
- in the pass's second round;
- after Flume's status of a place the listing does not hold;
- refused with no words, or of a launch held unlaunched.

The hosted implementation reads nothing of it and refuses nothing of it.

`boomdrive-control`'s documentation of `ActSort::Land`, `Confirm` and `Hold`, `Preview` and `Previewed` says version 3 of the hosted protocol carries each as the interface writes it, version 2 each but a hold, and version 1 none of them.

### 7. Versions 1 and 2 stay as their releases spoke them

No file under `protocol/v1/` or `protocol/v2/` changes. `version_1_is_as_released` and `version_2_is_as_released` stay as they are.

- **A new test, `version_2s_examples_still_pass`,** in `tests/examples.rs` beside `version_1s_examples_still_pass`, checks each example of version 2 against its direction's envelope of version 2.
- **`a_hold_is_no_act_of_version_1_or_2`** checks version 2's `want` by `common::V2`, no longer by the current version's free functions.
- **The tests of a confirm, a land and a preview of version 2** check version 2 by `common::V2`, and each gains a sibling of version 3. The runner-protocol spec keeps each scenario of version 2 beside one of version 3, since a scenario a requirement holds is kept when the requirement is changed.
- **`every_sort_the_interface_names`** checks every value against version 3's `want`, each passing, the hold among them. It also checks every value against version 2's, each passing but the hold, which version 2 refuses. It still asserts the three sets of sorts named. So a sort the interface adds fails it until a value of it is given, and fails version 3 until a version names it.

### 8. A test holds version 3 to version 2

`tests/files.rs` gains `version_3_is_version_2_but_for_its_hold`:

- **The schema files.** Version 3's `FILES` and version 2's name the same files. Each of version 3's is read, with every `description` set aside at any depth, its `$id` read as version 2's address and the same name, its envelope's `protocol_version` `const` read as 2, and the hold's branch taken out of `act_sort`'s `oneOf`. Read so, it equals version 2's file with every `description` set aside. The hold's branch, its descriptions set aside, equals Decision 1's. A failure names the file and the JSON pointer of the first difference, and no value (core.11).
- **The examples.** Version 3's examples are version 2's names and `to-runner/pulled-with-a-hold.json`, exactly. Each of version 2's names, read in version 3 with `protocol_version` read as 2, equals version 2's. In `from-runner/enroll.json` its `signature` is also set aside.

So every requirement of what version 2 carries holds of version 3 but for its hold, and a change to version 3 that is not the hold fails here.

Alternative:

- **Each test of version 2's content run again against version 3, with no test of the difference.** Not taken. It shows version 3 carries what those tests check, not that it differs from version 2 by nothing else.

### 9. What the runner says of the protocol

- **`protocol/README.md`** describes version 3 as it describes version 2 now:
  - every message names `protocol_version` 3, and the subprotocol and the contexts are `boomdrive.v3`;
  - the worked bytes are version 3's;
  - each file's `$id` is under `boomdrivedev/boomdrive`;
  - its messages table names a hold among a `pulled`'s acts, and a paragraph says what a hold holds, as a confirm's does;
  - its list of what the version has no message for names a forge token, health, the runner's own commands, a terminal's certificate and an observing connection.

  Its *Versions* section says the runner speaks version 3 alone, that a control plane takes it beside version 2, and that the test `files` holds version 2 to `v0.9.0`'s files and version 1 to `v0.7.3`'s. A section **Version 2** says which releases spoke it, that a control plane may still take it, and that it is version 3 but for its subprotocol, contexts, `$id`s and the hold. The section **Version 1** stays.
- **README.md's hosted section** says the runner speaks version 3 of the protocol, `boomdrive.v3`, under `crates/boomdrive-hosted/protocol/v3/`, which a control plane takes beside the version before. It says a hosted control plane's hold is carried out as crew's is, and drops its sentence that no version carries a hold yet. Its forge-token section says version 3 of the protocol carries no token yet.

## Who owns what

| piece | owner | cited by |
|---|---|---|
| version 3's files, its examples, and the runner's types of it | the hosted implementation, `boomdrive-hosted` (core.2, core.42; record 0052) | boomdrive-cloud, which copies them |
| the hold's form | the interface, `boomdrive-control` (core.59) | version 3's `act_sort` |
| how a hold is carried out | the runner (core.59) | the hosted implementation, the cloud |
| accepting version 3 beside version 2 | the control plane, boomdrive-cloud (record 0015) | the runner, which speaks version 3 alone |
| a hold of the waiting move | Flume (its 130 and core.24) | the runner |

## Named follow-ups

- **boomdrivedev/boomdrive-cloud: take version 3.** Its pin moves from version 2 to version 3, which it accepts beside version 2, and its control plane may send `hold` naming a launch, words and a bound for proven work that waits on a person. Until it does, a host running this release does not reach it: the runner says once that the control plane does not speak `boomdrive.v3`, and tries again on its backoff.
- **This repository and boomdrive-cloud: an observing connection (record 0048; cloud record 0069).** No version carries it. A change that builds it on this bolt before the bolt lands adds it to version 3. Once a release speaks version 3, it is a version 4. Which it is turns on when that change is planned, which is the planner's.

Each is told to the orchestrator.

## Risks / Trade-offs

- **A control plane that speaks version 2 alone cannot be reached by this release.** → boomdrive-cloud pins version 2. A host under it says once that the control plane does not speak `boomdrive.v3`, and tries again on its backoff. No host is known to run a hosted partition. The orchestrator asks the cloud.
- **Version 3 is fixed from its first release.** → A release follows every push to `main` (record 0024), so version 3 freezes when this bolt lands. A field the cloud wants next is then a version 4.
- **The structural test reads every file of two versions.** → Eleven small files and forty examples. It runs in the hosted crate's `files` test, which already reads every file of every version.

## Migration Plan

Nothing on a host moves. A host's enrolled keys stay, and its next connection proves them under version 3. A host whose hosted control plane takes version 3 is reached as before. One whose control plane does not is unreachable until the control plane takes it. No file the runner keeps changes, so the release's notes name no move by hand (core.36).

## Open Questions

None.

## Task notes

- **1.1**
  - Copy `protocol/v2/` to `protocol/v3/` with `cp -R`. In every schema file, replace the address `https://raw.githubusercontent.com/agentplot/boomdrive/main/crates/boomdrive-hosted/protocol/v2/` with `https://raw.githubusercontent.com/boomdrivedev/boomdrive/main/crates/boomdrive-hosted/protocol/v3/`. Set each envelope's `protocol_version` `const` to 3 and each example's `protocol_version` to 3. Change each description that names version 2, as Decision 1 lists. jsonfmt keeps the files' layout.
  - `src/protocol/v3.rs` holds `VERSION`, `SUBPROTOCOL`, `ADDRESS` and `FILES` alone until task 2.1, documented as version 3 (record 0052). `src/protocol.rs` names it.
  - `tests/common/mod.rs` gains `V3`, documented as the version this release speaks (record 0052), and `VERSIONS` becomes `[V1, V2, V3]`. `CURRENT` stays `V2` until task 2.1.
  - `tests/files.rs`'s check of each version's number asserts `common::V3.number` is 3.
- **1.2**
  - `tests/interface.rs` gains `a_hold_is_an_act_of_version_3`. It builds the want `act-1` on `offer-1`, lease 3, `ActSort::Hold { words: "the user sends it on the landings card, row 3", bound_interval: 28800 }`, and checks it with `common::V3`'s `want`. It checks two more against the same `want`: one whose words are `""` and one whose bound is 0. Each passes.
  - It checks a hold of bound 2^53 against the same `want`, and finds it refused.
  - It checks `V3.example("to-runner", "pulled-with-a-hold.json")` against `V3.messages_to_runner()`, and finds it passes.
  - The example is `pulled-with-a-confirm.json` with the confirm's act replaced by `{"act":{"id":"act-7","launch":"offer-1","lease":2,"what":{"hold":{"words":"the user sends it on the landings card, row 3","bound_interval":28800}}}}` and its id `plane:13`.
- **1.3**
  - `a_hold_is_no_act_of_version_1_or_2` builds its second validator with `V2.validator(&V2.definition("wants.schema.json", "want"))`.
  - `every_sort_the_interface_names` builds `V3`'s `want` and `V2`'s. Each value passes `V3`'s. Each passes `V2`'s but the hold, which `V2`'s refuses.
  - `a_confirm_is_an_act_of_version_2`, `a_land_is_an_act_of_version_2` and `a_preview_is_a_want_of_version_2` check version 2 by `common::V2`, its `want`, its `to-runner.schema.json` and its examples. Each gains a sibling of version 3, `a_confirm_is_an_act_of_version_3`, `a_land_is_an_act_of_version_3` and `a_preview_is_a_want_of_version_3`, the same test by `common::V3`.
  - The module's documentation and the test's say version 3 carries every sort the interface names, and version 2 every sort but the hold, since `v0.9.0` spoke version 2 before the interface named one.
- **1.4**
  - `version_3_is_version_2_but_for_its_hold` reads each version's files from `common::V2.files` and `common::V3.files`, and its examples with `files_under` over `version.dir().join("examples")`.
  - It sets aside every `description` with a small recursive function over `serde_json::Value`. It finds the first differing pointer with another, which returns the path alone.
  - The hold's branch is the one item of `$defs.act_sort.oneOf` whose `required` is `["hold"]`. Exactly one item is so, and the test asserts it.
- **1.5** `version_2s_examples_still_pass` is `version_1s_examples_still_pass` with `V2` for `V1`.
- **2.1**
  - **The source.** Move `src/protocol/v2.rs`'s types into `src/protocol/v3.rs` whole, then rewrite `v2.rs` to the four constants and its `FILES`, documented as `v1.rs` is. Point each module at `protocol::v3`.
  - **The words.** Make every word of a check, an enrollment's answer and the retriever's refusal from `VERSION`. `grep -rn "version 2" crates/boomdrive-hosted/src` should then find only documentation of version 2 itself.
  - **The tests' current version.** `tests/common/mod.rs`'s `CURRENT` becomes `V3`. `common::fake`'s `SUBPROTOCOL` is `boomdrive_hosted::protocol::v3::SUBPROTOCOL`.
  - **The worked bytes.** Run `cargo test -p boomdrive-hosted --test signatures`, read the failures, and write the bytes the crate made into `tests/worked-bytes.json`. Then write the worked enrollment signature into version 3's `examples/from-runner/enroll.json`, and point `the_examples_enroll_carries_the_worked_key_and_signature` at `protocol/v3/`.
  - **The tests renamed for version 3:**
    - `fake.rs`'s `a_message_failing_version_2_is_refused`;
    - `enrollment.rs`'s `an_answer_that_fails_version_2`;
    - `plane.rs`'s `a_report_that_fails_version_2_is_not_sent`.

    Each test's and each module's documentation that names version 2 as the version the runner speaks names version 3, `ends.rs`, `cut.rs`, `copies.rs`, `forms.rs` and `shared.rs` among them.
  - **`connection.rs`'s `a_control_plane_that_speaks_version_1_alone`** keeps its name, which its scenario keeps, and asserts `boomdrive.v3`.
  - **`connection.rs`'s `a_message_of_version_1`** keeps its name and asserts words starting `it fails version 3's check: /`. A new `a_message_of_version_2` is the same test sending `V2.example("to-runner", "pulled.json")`.
  - **`refused.rs`'s `a_message_of_another_version`** checks a `hello` naming 2 against version 3, one naming 3 against version 2, and one naming 1 against version 3, each refused.
  - **`interface.rs`'s `the_interfaces_values_pass`** gains `ActSort::Hold { words: "the user sends it", bound_interval: 28800 }` among its acts, which pass the current version, now 3.
  - **`examples.rs`** reads version 3's examples into `protocol::v3`'s types.
  - **`copy.rs`'s unit test** that reads `../protocol/v2/examples/from-runner/report-a-tidied-place.json` reads version 3's. The unit tests of `check.rs` and `connection.rs` that build a message naming `protocol_version` 2 name `VERSION`.
- **2.2** `crates/boomdrive/tests/forge_tokens.rs`'s `a_get_on_a_cloud_host_is_refused` asserts the words `the hosted control plane mints this host's forge tokens, and version 3 of its protocol carries none yet`.
- **3.1**
  - `tests/plane.rs`'s `a_hold_carried` follows `a_land_carried`. `home`'s fake control plane offers `l-1` and asks act `hold-1` on it, `{"hold":{"words":"the user sends it on the landings card, row 3","bound_interval":28800}}`. `home`'s part's pull gives `Want::Act` `home/hold-1` on `home/l-1`, `ActSort::Hold` with those words and that bound, and the fake took no `message_refused`.
  - `a_hold_the_runner_refuses` asks act `hold-2`, a hold whose words are `""` and whose bound is 0. The pull gives `home/hold-2` as given, and no `message_refused` was sent.
- **3.2** `refused.rs`'s `a_field_the_version_does_not_name` adds a field `reason` beside `words` in the hold of version 3's `pulled-with-a-hold.json`, and finds it refused, beside what it refuses now.
