# Tasks

Work through the tasks in order. A task is true once the check it names passes, run as it names it. The groups are headings for reading.

- This branch holds the runner bolt's hold (#214): `ActSort::Hold`, the runner's `Act::Hold`, core.59 and record 0002's amendment of 2026-11-14. Record 0052, core.42's and core.59's details, and record 0002's line naming version 3 are already written with the change.
- Version 3 is version 2 whole, but for what design Decision 1 lists. No file under `crates/boomdrive-hosted/protocol/v1/` or `protocol/v2/` changes (core.42). A task that would edit one is wrong.
- Every test runs on the crate's fakes, in a temporary folder, with neutral names and paths (core.41). No task runs a real Flume, sluice or control plane.
- No line the runner says, and no failure a check gives, names a value of a message: paths alone (core.11).
- No task adds a crate, a tool or a tracked file type (core.7, core.8). Nothing outside the paths proposal.md's Touches lists is changed. A task that relies on a choice the design names cites its Decision.

## 1. Version 3 beside version 2

- [x] 1.1 `crates/boomdrive-hosted/protocol/v3/` holds version 2's eleven schema files and its examples. Each file's `$id` is under `https://raw.githubusercontent.com/boomdrivedev/boomdrive/main/crates/boomdrive-hosted/protocol/v3/`, and each envelope's and example's `protocol_version` is 3. Each description naming version 2 names version 3. `src/protocol/v3.rs` holds `VERSION` 3, `SUBPROTOCOL` `boomdrive.v3`, that `ADDRESS` and `FILES`, and `src/protocol.rs` names it. The tests' `common` module gains `V3`, and `VERSIONS` holds the three. This proves the runner-protocol spec's "Each file is a schema of its draft", "Nothing outside the version is referenced" and "Every file is listed", and keeps "Version 2 as released" (design Decisions 1 and 3; Task notes 1.1). True when `devenv shell -- cargo test -p boomdrive-hosted --test files` passes.
- [x] 1.2 Version 3's `act_sort` gains the hold, as design Decision 1 gives it. Version 3's `examples/to-runner/` gains `pulled-with-a-hold.json`. `tests/interface.rs`'s new `a_hold_is_an_act_of_version_3` checks against version 3's `want`:
  - a hold, one of no words and one of bound 0, each passing;
  - a hold of bound 2^53, refused;
  - the new example against version 3's `to-runner.schema.json`, passing.

  This proves the runner-protocol spec's "A hold is an act of version 3" and "A hold's bound past what JavaScript reads exactly" (design Decisions 1 and 2; Task notes 1.2). True when `devenv shell -- cargo test -p boomdrive-hosted --test interface a_hold_is_an_act_of_version_3` passes.
- [x] 1.3 In `tests/interface.rs`:
  - `a_confirm_is_an_act_of_version_2`, `a_land_is_an_act_of_version_2`, `a_preview_is_a_want_of_version_2` and `a_hold_is_no_act_of_version_1_or_2` check version 2 by `common::V2`;
  - the first three each gain a sibling of version 3, checking it by `common::V3`;
  - `every_sort_the_interface_names` checks every value against version 3's `want`, each passing, and against version 2's, each passing but the hold, which version 2 refuses. It still asserts every sort named.

  This proves the runner-protocol spec's "Every sort the interface names", "A hold is no act of version 1 or 2", and "A confirm is an act", "A land is an act" and "A preview is a want", each of version 2 and of version 3 (design Decision 7; Task notes 1.3). True when `devenv shell -- cargo test -p boomdrive-hosted --test interface -- a_confirm_is_an_act a_land_is_an_act a_preview_is_a_want a_hold_is_no_act_of_version_1_or_2 every_sort_the_interface_names` passes.
- [ ] 1.4 `tests/files.rs`'s new `version_3_is_version_2_but_for_its_hold` holds each schema file and example of version 3 to version 2's, but for what design Decision 8 sets aside. A failure names the file and the first differing JSON pointer, and no value. This proves the runner-protocol spec's "Version 3 is version 2 but for its hold" and "Version 3's examples are version 2's" (design Decision 8; Task notes 1.4). True when `devenv shell -- cargo test -p boomdrive-hosted --test files version_3_is_version_2_but_for_its_hold` passes.
- [ ] 1.5 `tests/examples.rs`'s new `version_2s_examples_still_pass` checks each example of version 2 against its direction's envelope of version 2. This proves the runner-protocol spec's "Version 2's examples still pass" (design Decision 7; Task notes 1.5). True when `devenv shell -- cargo test -p boomdrive-hosted --test examples version_2s_examples_still_pass` passes.

## 2. The runner speaks version 3

- [ ] 2.1 The runner's message types move from `src/protocol/v2.rs` to `src/protocol/v3.rs`. `protocol::v2` keeps `VERSION`, `SUBPROTOCOL`, `ADDRESS` and `FILES` alone.
  - `check.rs`, `copy.rs`, `connection.rs`, `enrollment.rs`, `key.rs`, `plane.rs` and `lib.rs` use version 3.
  - Every word the crate says naming the version it speaks is made from `VERSION`, and `NO_FORGE_TOKEN` names version 3. The crate's documentation and `Cargo.toml`'s comment name version 3.
  - The tests' `CURRENT` is `V3`, and `common::fake` speaks `boomdrive.v3`.
  - `tests/worked-bytes.json` and version 3's `examples/from-runner/enroll.json` hold the bytes the crate makes for version 3's contexts.
  - Each test that names version 2 as the version the runner speaks names version 3, by Task notes 2.1's list, and `interface.rs`'s `the_interfaces_values_pass` gives a hold among its acts.
  - `connection.rs`'s `a_message_of_version_1` and a new `a_message_of_version_2` each find a `pulled` of their version refused with version 3's words, and `refused.rs`'s `a_message_of_another_version` checks versions 2 and 3 against each other.

  This proves the hosted-control-plane spec's "A partition enrolled", "A connection proved", "A control plane that speaks version 1 alone", "A pulled with a field unnamed", "A message of version 1" and "A message of version 2". It also proves the runner-protocol spec's "A message of another version", "The interface's values pass", "Every example read and written back", "Every kind has an example", "The README's worked contexts", and every scenario they name as checked against version 3 (design Decisions 4 and 5; Task notes 2.1). True when `devenv shell -- cargo test -p boomdrive-hosted` passes.
- [ ] 2.2 `crates/boomdrive/tests/forge_tokens.rs`'s `a_get_on_a_cloud_host_is_refused` asserts the words naming version 3. This proves the forge-tokens spec's "A get on a cloud host" (design Decision 4; Task notes 2.2). True when `devenv shell -- cargo test -p boomdrive --test forge_tokens a_get_on_a_cloud_host_is_refused` passes.

## 3. A hosted control plane's hold

- [ ] 3.1 A hold the fake control plane asks is given to the runner as an act, its id and launch under the partition's prefix, its words and bound as given. A hold of no words and bound 0 is given alike. No `message_refused` is sent for either. This proves the hosted-control-plane spec's "A hold carried" and "A hold the runner refuses" (design Decision 6; Task notes 3.1). True when `devenv shell -- cargo test -p boomdrive-hosted --test plane -- a_hold_carried a_hold_the_runner_refuses` passes.
- [ ] 3.2 `tests/refused.rs`'s `a_field_the_version_does_not_name` adds a field to a hold, beside what it adds now, and checks every example against version 3. This proves the runner-protocol spec's "A field the version does not name" (design Decision 1; Task notes 3.2). True when `devenv shell -- cargo test -p boomdrive-hosted --test refused a_field_the_version_does_not_name` passes.

## 4. What the runner says of it

- [ ] 4.1 `boomdrive-control`'s documentation of `ActSort::Land`, `ActSort::Confirm`, `ActSort::Hold`, `Want::Preview`, `Preview` and `Previewed` says version 3 of the hosted protocol carries each as the interface writes it, version 2 each but a hold, and version 1 none (design Decision 6). True when `sluice exec main -- moon run boomdrive-control:lint` passes.
- [ ] 4.2 `crates/boomdrive-hosted/protocol/README.md` describes version 3 throughout:
  - its envelope, subprotocol, contexts and worked bytes are version 3's;
  - each file's `$id` is under `boomdrivedev/boomdrive`;
  - its messages table names a hold among a `pulled`'s acts, with a paragraph on a hold;
  - its list of what the version has no message for is design Decision 9's;
  - *Versions* says the runner speaks version 3 alone;
  - a section **Version 2** says which releases spoke it and how it differs, and the section **Version 1** stays.

  This keeps the runner-protocol spec's "The README of version 2" and "The README of version 3" (design Decision 9). True when `sluice exec main -- moon run root:format` passes.
- [ ] 4.3 README's hosted section says the runner speaks version 3 of the protocol, `boomdrive.v3`, under `crates/boomdrive-hosted/protocol/v3/`, which a control plane takes beside the version before. It says a hosted control plane's hold is carried out as crew's is, and no longer says no version carries a hold. README's forge-token section says version 3 of the protocol carries no token yet (design Decision 9). True when `sluice exec main -- moon run root:format` passes.

## 5. The specs

- [ ] 5.1 The change's deltas archive into specs that pass openspec's strict check. True when, after `devenv shell -- openspec archive the-hosted-protocol-carries-a-hold --yes`, `sluice exec main -- moon run root:specs root:changes` passes.
