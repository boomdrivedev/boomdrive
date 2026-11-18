# Verify

The verify stage of `the-hosted-protocol-carries-a-hold`, on `change/the-hosted-protocol-carries-a-hold` at `9d0e9d7`, cut from `bolt/a-hosted-control-plane-can-hold-a-waiting-place` at `ebda503b`.

**Outcome: the whole checks pass, and the work meets every requirement and scenario of the change's specs, with every task ticked. The change is archived.**

## The whole checks

`sluice exec main`, run once on `9d0e9d7`, exited 0: `passed: 23 tasks`, recorded under their keys for `9d0e9d7`.

| project | tasks |
|---|---|
| `boomdrive` | format, lint, typecheck, test (727 tests passed, 0 failed) |
| `boomdrive-control` | format, lint, typecheck, test (44 passed, 0 failed) |
| `boomdrive-crew` | format, lint, typecheck, test (171 passed, 0 failed) |
| `boomdrive-hosted` | format, lint, typecheck, test (184 passed, 0 failed) |
| `nix` | typecheck, test |
| `root` | format, specs, changes, releases, versions |

`root:changes` archived this open change, alone, to a scratch copy of the specs, and every spec it wrote passed `openspec validate --strict` (core.61). This is task 5.1's check, made before the archive.

moon warned at each task that it could not package the task's outputs into its cache. The warning is moon's cache, not a check, and failed no task.

## The tasks

Every task of tasks.md, 1.1 to 5.1, is ticked. Each one's named test is in the tree and passed in the gate's run:

| task | tests |
|---|---|
| 1.1 | `files`'s checks of each version, with `V3` among `VERSIONS` |
| 1.2 | `interface`'s `a_hold_is_an_act_of_version_3` |
| 1.3 | `interface`'s `a_confirm_is_an_act_of_version_2` and `_3`, `a_land_is_an_act_of_version_2` and `_3`, `a_preview_is_a_want_of_version_2` and `_3`, `a_hold_is_no_act_of_version_1_or_2` and `every_sort_the_interface_names` |
| 1.4 | `files`'s `version_3_is_version_2_but_for_its_hold` |
| 1.5 | `examples`'s `version_2s_examples_still_pass` |
| 2.1 | the whole of `boomdrive-hosted`'s tests, `connection`'s `a_message_of_version_1` and `a_message_of_version_2`, `refused`'s `a_message_of_another_version`, `fake`'s `a_message_failing_version_3_is_refused`, `enrollment`'s `an_answer_that_fails_version_3` and `plane`'s `a_report_that_fails_version_3_is_not_sent` among them |
| 2.2 | `boomdrive`'s `forge_tokens`'s `a_get_on_a_cloud_host_is_refused` |
| 3.1 | `plane`'s `a_hold_carried` and `a_hold_the_runner_refuses` |
| 3.2 | `refused`'s `a_field_the_version_does_not_name` |
| 4.1 | `boomdrive-control:lint` |
| 4.2, 4.3 | `root:format` |
| 5.1 | `root:specs` and `root:changes` |

## The specs

### runner-protocol

- **Each version is JSON Schema in the hosted crate.** `protocol/v3/` holds version 2's eleven files. Each `$id` is under `https://raw.githubusercontent.com/boomdrivedev/boomdrive/main/crates/boomdrive-hosted/protocol/v3/`, and `protocol::v3::FILES` lists them. `files` proves each scenario for each of the three versions.
- **Every message names its version.** Both envelopes of version 3 hold `protocol_version` `{"const": 3}`. `a_message_of_another_version` checks each of these and finds it refused: 1, 2, 4, 0 and `"3"` against version 3, version 2's hello against version 3, version 3's against version 2, version 1's against version 3, and version 2's against version 1.
- **A message holds only what its version names.** `a_field_the_version_does_not_name` adds a field in a hold, beside its other places, and checks every example against version 3.
- **The interface's shapes.** `the_interfaces_values_pass` gives a hold among its acts. `every_sort_the_interface_names` finds every sort passing version 3's `want`, and every sort but the hold passing version 2's, which refuses the hold.
- **The runner's types.** Version 2's types moved to `src/protocol/v3.rs` whole. `protocol::v2` keeps `VERSION`, `SUBPROTOCOL`, `ADDRESS` and `FILES` alone. `examples` reads every example of version 3 into its direction's type and writes it back.
- **The README.** `protocol/README.md` names `protocol_version` 3, `boomdrive.v3`, its contexts and worked bytes, and a hold among a `pulled`'s acts with a paragraph of its own. It also names what version 3 has no message for, as Decision 9 lists. Its *Version 2* section names `boomdrive.v2`, the releases that spoke it, and how it differs: subprotocol, contexts, `$id`s and the hold. *Version 1* stays. `signatures` holds `tests/worked-bytes.json` to the crate's contexts of `boomdrive.v3`.
- **Version 3 carries a confirm, a land, a preview and its done, and a report's previews.** Each scenario is proven by its test of version 2 and its sibling of version 3.
- **Version 2 stays as a release spoke it.** No file under `protocol/v1/` or `protocol/v2/` differs from `ebda503b`. `version_2_is_as_released` and `version_2s_examples_still_pass` pass.
- **Versions 1 and 2 carry no hold.** `a_hold_is_no_act_of_version_1_or_2` checks version 2 by `common::V2`.
- **Version 3 is version 2 whole, but for a hold.** `version_3_is_version_2_but_for_its_hold` holds each file and example of version 3 to version 2's. It sets aside descriptions, the `$id`, `protocol_version` and the one hold branch, and `enroll.json`'s signature. It then asserts the hold's branch is Decision 1's, with the keys `ActSort::Hold` writes.
- **Version 3 carries a hold.** `act_sort` ends with the hold: `words` a string, and `bound_interval` `shared.schema.json`'s `unsigned_integer`, 0 to 2^53 − 1. `a_hold_is_an_act_of_version_3` covers three holds that pass: one of words, one of no words, and one of bound 0. It finds a hold of bound 2^53 refused, and `pulled-with-a-hold.json` passing.

### hosted-control-plane

- **Enrollment and the connection's proof.** `enrollment.rs` and `key.rs` use `protocol::v3`, and each context starts `boomdrive.v3`. `enrollment` and `connection` prove "A partition enrolled", "A connection proved" and "A control plane that speaks version 1 alone", the last naming `boomdrive.v3`.
- **What goes up is version 3's copy.** `copy.rs`, `check.rs` and `plane.rs` cut and check by version 3's files. Every word naming the version is made from `VERSION`, and `grep` finds no "version 2" in the crate's source but `v2.rs`'s and `v3.rs`'s documentation of version 2. `cut`, `copies`, `ends` and `plane` pass on version 3.
- **Every message the runner is sent is checked.** `a_message_of_version_1` and `a_message_of_version_2` each find a `pulled` of their version answered with `message_refused`, its words those of version 3's check.
- **A hold is given to the runner as any act.** `a_hold_carried` gives `home/hold-1` on `home/l-1`, with lease 1 and `ActSort::Hold` under the words for 28800 seconds, and finds no `message_refused`. `a_hold_the_runner_refuses` gives `home/hold-2`, of no words and bound 0, the same way. `own_wants` reads and refuses nothing of a hold.

### forge-tokens

`NO_FORGE_TOKEN` says version 3 of the protocol carries none yet. `a_get_on_a_cloud_host_is_refused` asserts those words.

## Scope and rules

- **Paths.** Every changed path since `ebda503b` is among proposal.md's Touches. No crate, tool, configuration key or tracked file type was added.
- **Records and rules.** Record 0052 is written. core.42's and core.59's details name version 3 and record 0052, and record 0002's amendment of 2026-11-14 names version 3.
- **Documentation.** `boomdrive-control`'s documentation of a land, a confirm, a hold, a preview and a previewed says that version 3 carries each, version 2 each but a hold, and version 1 none. README's hosted and forge-token sections name version 3 and a hosted control plane's hold.

## Named follow-ups

The design names two follow-ups outside this change: boomdrive-cloud taking version 3, and where record 0048's observing connection goes. Each is the orchestrator's, as design.md says. Nothing found here adds to them.
