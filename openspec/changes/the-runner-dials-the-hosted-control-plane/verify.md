# Verify: the-runner-dials-the-hosted-control-plane

The work on `change/the-runner-dials-the-hosted-control-plane`, read at 6857999, meets its change, but it cannot land as it stands, so the change is not archived. It goes back to its code stage for one fault, below.

- Every task is ticked.
- Every check passes on the branch, and so does the gate run through sluice.
- The change has 23 requirements and 49 scenarios, and a test asserts the outcome of every scenario.
- A trial archive merges the five deltas into 12 specs that pass `openspec validate --specs --strict`.

## What fails

**Two records numbered 0017.** `main` has moved past the branch: it stands at 00bda37, after #1 (`a-view-writes-its-vs-code-workspace`), #8 and #9. #1 landed its own record 0017, `docs/adr/0017-each-owners-views-have-a-vs-code-workspace-beside-them-where-the-configuration-asks.md`. Its core.43, now on `main`, cites it as "record 0017". This change adds a different record under the same number, `docs/adr/0017-the-hosted-implementation-enrolls-each-partition-once-proves-the-hosts-key-at-every-connection-and-keeps-nothing-but-the-key.md`.

- `git merge-tree` of the branch and `origin/main` merges with no textual conflict.
- `check-adrs.sh`, the task `root:adrs` of `check-fast` that `main` requires, fails on that merged tree: `check-adrs: 0017 is the number of docs/adr/0017-each-owners-views-… docs/adr/0017-the-hosted-implementation-…`, and `the later of each takes the next free number, from 0018`.
- With both on `main`, "record 0017" in this change's text would name two decisions. The change's own record is the later one, since #1 has landed, so it takes 0018.

What the code stage owes:

1. Take `main` as it stands, by a merge with no rebase, as task 8.2 took it.
2. Renumber this change's record to 0018: its file name, its frontmatter `number: 17`, and every place the change cites it as record 0017:
   - `docs/architecture/core.md`, the details of core.26, core.31 and core.32;
   - the change's `proposal.md` (three places, the Touches path among them), `design.md` (*Rules relied on*, core.31 and core.32) and `tasks.md` (8.1);
   - `specs/hosted-control-plane/spec.md`, its Purpose;
   - `specs/runner-protocol/spec.md`, both requirements;
   - `crates/boomdrive-hosted/src/protocol/v1.rs` and `crates/boomdrive-hosted/tests/signatures.rs`, a comment each.

   Some citations of 0017 name the cloud's own record and stay as they are: `boomdrive-cloud record 0017` in core.1's details, `cloud record 0017` in `crates/boomdrive-control/src/want.rs` and record 0002, and the cloud's records listed in records 0002 and 0008.
3. Run the gate on the merge, `sluice exec main`. It has not been run on the merged tree, beyond `check-adrs.sh` above.

No rule id collides: this change adds no rule, and #1's core.43 is the only rule `main` gained.

## Tasks

All 29 tasks of `tasks.md`, groups 1 to 8, are ticked `[x]`, and none is open. What they build is in the tree, as Task notes name it:

- **The control pass** (group 1). `next_due` takes `reported` and the time left to the earliest lease not yet lapsed. `Reach` says the unread line once and the read-again line once. `offer`, `act` and `command` leave work under a lease lapsed by the pass's one clock.
- **Version 1** (group 2). `enrollment.schema.json` holds `enroll`, `enrolled` and `enroll_refused`, each closed. The envelopes take them in their directions, `FILES` lists the file, and `v1.rs` has the three types. The README names both paths, the `BoomdriveHost` header and each context's worked bytes.
- **The table, key and enrollment** (group 3). These are `table.rs`, `key.rs` with `Keychain`, `SystemdCreds` (refused below 256) and `Memory`, `enrollment.rs`, and the fake hosted control plane in `tests/common/fake.rs`, which listens on the loopback address and is test code alone.
- **The connection** (group 4). `connection.rs` has the upgrade on `boomdrive.v1` with a fresh statement, the whole hello first and again when it changes, pings every 30 seconds, and the full-jitter backoff capped at `poll_interval`. It says its reach once each way, checks every message both ways, and closes with 1007 on a message whose id cannot be read.
- **The implementation** (group 5). These are `plane.rs`'s `Hosted`, `copy.rs`'s cut walked from the schema, and `check.rs`'s validators built from `FILES` with no retriever.
- **The binary** (group 6). These are `planes.rs`'s `Entry` list of `crew` and `hosted`, with `planes::enroll`, `enroll.rs` and `Command::Enroll`, the rules tests and `moon.yml`, and the README and CLAUDE.md.
- **The names** (group 8). No line the change adds holds `flywheel`, `tailrace`, `enrol` not followed by `l`, or `enrolment`, but tasks 8.1's and 8.2's own words. CLAUDE.md holds both the family section and the hosted implementation.

## Checks

All were run through sluice, so its sandbox applied.

- **The gate.** `devenv shell -- sluice exec main` passed all 21 `check-fast` tasks at 6857999, against `main` at 00bda37, and recorded its result under 6857999. Every test passed, with none failed or ignored.
- **Task 7.2.** On a copy of `openspec/`, the pinned openspec 1.14.3 archived the change: 18 requirements added and 5 modified across five specs. `openspec validate --specs --strict` then passed all 12 specs.
- **Task 8.1's and 8.2's grep** found nothing outside `tasks.md`.

## Requirements and scenarios

Each scenario's test. Tests named as `tests/…` are the crate's integration tests; `table::tests`, `carry::tests` and `run::tests` are unit tests in `src`.

### Control-plane interface

| scenario | test |
|---|---|
| The hosted control plane named | `boomdrive` `tests/control.rs` `the_hosted_control_plane_named`; `tests/planes.rs` `the_hosted_table_is_the_hosted_implementations` |
| A word not carried | `tests/control.rs` `a_word_not_carried`, naming `crew` and `hosted`; `tests/planes.rs` `a_word_not_carried` |
| The fake control plane passes | `boomdrive-control`'s shared cases over its fake |
| crew's implementation passes | `boomdrive-crew` `tests/shared.rs` |
| The hosted implementation passes | `boomdrive-hosted` `tests/shared.rs`, the four shared cases over the fake for one partition |
| Only the hosted implementation's crate holds the protocol | `tests/rules.rs` `only_the_hosted_implementations_crate_holds_the_protocol` and `the_crates_depend_as_decision_1_has_them` |

### Hosted control plane

| scenario | test (`boomdrive-hosted`, but where named) |
|---|---|
| The table read | `table::tests::the_table_read` |
| A control plane in the clear | `table::tests::a_control_plane_in_the_clear`, the refusal's exact words |
| A token in the table | `table::tests::a_token_in_the_table`: `token`, `token_file` and `key`, at the top and in a partition, the value in neither refusal |
| A label twice | `table::tests::a_label_twice` |
| A partition enrolled | `tests/enrollment.rs` `a_partition_enrolled`: the bearer, the `enroll` passing version 1, its signature checked, the key kept under `home`, host `h-1` |
| A lapsed token | `tests/enrollment.rs` `a_lapsed_token`, not the host's to fix, the store as it was |
| The answer as JSON | `boomdrive` `tests/enroll.rs` `the_answer_as_json`, one object, exit 1 |
| The token never shown | `boomdrive` `tests/enroll.rs` `the_token_never_shown`, in words and as JSON |
| crew's control plane | `boomdrive` `tests/enroll.rs` `crews_control_plane`, exit 2 |
| An unknown label | `boomdrive` `tests/enroll.rs` `an_unknown_label`, exit 2; `tests/enrollment.rs` `an_unknown_label`, no request |
| Nothing listening | `boomdrive` `tests/enroll.rs` `nothing_listening`, exit 1, no key; `tests/enrollment.rs` `nothing_listening` |
| Kept by systemd-creds | `tests/key.rs` `kept_by_systemd_creds`: `encrypt --user --name=boomdrive.home.control.example`, the key on standard input alone, in no argument in hex or base64 |
| An old systemd | `tests/key.rs` `an_old_systemd`; `tests/enrollment.rs` `an_old_systemd_makes_no_request`, the host's to fix |
| A connection proved | `tests/connection.rs` `a_connection_proved`: `boomdrive.v1`, the fake's check of each signature, two nonces |
| A key refused | `tests/connection.rs` `a_key_refused`: one line, `home`, the fake's words, `boomdrive enroll home`, each retry at the cap |
| The hello first | `tests/connection.rs` `the_hello_first`; and `the_hello_again_when_it_changes` |
| No heartbeat message | `tests/connection.rs` `no_heartbeat_message`: four pings, the hello the one message; and `a_connection_whose_pong_does_not_come_is_lost` |
| Never back to back, never past the poll interval | `tests/connection.rs` `never_back_to_back_never_past_the_poll_interval`, each wait within its doubling cap |
| An outage | `tests/connection.rs` `an_outage`, three lines |
| Ids under their partition | `tests/plane.rs` `ids_under_their_partition`, `after` prefixed too |
| One partition unreachable | `tests/plane.rs` `one_partition_unreachable`; and `no_partition_read_is_unread_naming_each` |
| Woken | `tests/plane.rs` `woken` |
| Each hello its own | `tests/plane.rs` `each_hello_its_own`; and `each_report_its_own` |
| Another partition's repository | `tests/plane.rs` `another_partitions_repository`, one line over two pulls |
| A lease refused | `tests/plane.rs` `a_lease_refused` |
| A report resent | `tests/plane.rs` `a_report_resent`: hello, report, hello, report, the same id and body |
| Flume's places cut | `tests/cut.rs` `flumes_places_cut`, over Flume 0.4.1's fixtures |
| A value Flume adds | `tests/plane.rs` `a_value_flume_adds`: `null`, the report passing, one line naming `/stages/1/state` and not the value |
| A pulled with a field unnamed | `tests/connection.rs` `a_pulled_with_a_field_unnamed`; and `a_message_whose_id_cannot_be_read_closes_the_connection` |
| An enrollment's answer on the socket | `tests/connection.rs` `an_enrollments_answer_on_the_socket_is_refused` |
| A terminal opened | `tests/plane.rs` `a_terminal_opened`, a stray `terminal_input` dropped |

### Runner protocol

| scenario | test (`boomdrive-hosted`) |
|---|---|
| A body of the other direction | `tests/refused.rs` `a_body_of_the_other_direction` |
| Enrollment's kinds in the other direction | `tests/refused.rs` `enrollments_kinds_in_the_other_direction` |
| Enrollment's examples | `tests/examples.rs` `enrollments_examples` |
| An enroll unsigned, or holding a token | `tests/refused.rs` `an_enroll_unsigned_or_holding_a_token` |
| The README's worked contexts | `tests/signatures.rs` `the_enrollments_worked_bytes` and `the_connections_worked_bytes`, read from the README, the header too |

### Running

| scenario | test (`boomdrive`) |
|---|---|
| On the control timer | `run::tests::on_the_control_timer`; `tests/control.rs` `control_passes_on_their_own_timer` |
| A lapsed lease | `carry::tests::a_lapsed_lease`; `tests/leases.rs` `a_lapsed_lease`; `tests/control.rs` `a_lapsed_lease_sets_no_pass_sooner`, over the loop |
| A pass that did not report | `carry::tests::a_pass_that_did_not_report`; `tests/leases.rs` `a_pass_that_did_not_report`; `tests/control.rs` `a_pass_that_did_not_report_sets_no_pass_sooner` |
| Three passes unread, then read | `tests/control.rs` `three_passes_unread_then_read`, nothing said of it after |

### Work into Flume

| scenario | test (`boomdrive` `tests/leases.rs`) |
|---|---|
| An offer whose lease lapsed | `an_offer_whose_lease_lapsed`, then launched under a live lease |
| An act on a lapsed launch | `an_act_on_a_lapsed_launch`, the launch's answer standing, then carried out |
| A command whose lease lapsed | `a_command_whose_lease_lapsed`, then sluice run |
