# Verify: the-runner-keeps-its-protocol-as-one-json-schema-per-version

The work on `change/the-runner-keeps-its-protocol-as-one-json-schema-per-version`, read at b64b341, does not yet hold, and the change is not archived. Every task is ticked. Every check passes, and so does the gate run fresh through sluice. The change has 13 requirements and 30 scenarios, and a test asserts the outcome of every scenario.

What fails is the archive. A trial archive merges every delta, but the merged specs then fail `openspec validate --specs --strict`. That is what `root:specs` runs on main, so the archived tree would fail the gate. Both failures are in the change's spec deltas, and they go back to the code stage.

## What fails

1. **`runner-protocol` has no Purpose.**
   - The delta `specs/runner-protocol/spec.md` creates the capability, but it has no `## Purpose`.
   - So the archive writes openspec's placeholder: "TBD - created by archiving change the-runner-keeps-its-protocol-as-one-json-schema-per-version. Update Purpose after archive."
   - `openspec validate runner-protocol --type spec --strict` fails on it: "Purpose section is still a placeholder rather than a Purpose anyone wrote".
   - Each new capability of the two archived changes carries its `## Purpose` in its delta, which openspec reads when it creates the spec.
2. **The `control-plane-interface` requirement is too long.**
   - The MODIFIED requirement "The control plane is reached through one interface" has 625 characters of text. On main it has 424, and openspec's limit is 500.
   - `openspec validate control-plane-interface --type spec --strict` fails on it: "Requirement text is very long (>500 characters)".
   - `root:changes` passes, so the open change does not show it. Only the merged spec does.

To hold, the change needs:
- a `## Purpose` in the `runner-protocol` delta, saying what the capability is for;
- the `control-plane-interface` requirement brought within 500 characters.

One way to do the second is to leave the requirement as it stands on main, and add the protocol's crate as a requirement of its own. That new requirement would take core.42 and the scenario "Only the hosted implementation's crate holds the protocol". Splitting it this way changes no test: `only_the_hosted_implementations_crate_holds_the_protocol` proves the scenario wherever it sits.

The check is a trial archive on a copy of `openspec/`, then `openspec validate --specs --strict` on the result. It should pass all eleven specs.

## Tasks

All 15 tasks of `tasks.md`, groups 1 to 4, are ticked `[x]`, and none is open. Each group's closing task names the checks that ran it, and those checks pass at b64b341 (below).

What the tasks build is in the tree:
- **The crate.** `crates/flywheel-hosted` is a member of the cargo workspace.
  - Its manifest sets `publish = false`, `doctest = false` and the workspace's version and edition.
  - Its dependencies are `flywheel-control` by path, `serde`, and `serde_json`, which takes `raw_value` from the workspace.
  - Its one dev-dependency is `jsonschema`, which the workspace takes at 0.60 with `default-features = false`. `Cargo.lock` already held `jsonschema` 0.60.1 on main, so the lock only gains the new crate.
- **The moon project.** It is `flywheel-hosted` in `.moon/workspace.yml`, with `language: 'rust'`.
  - It has format, lint, typecheck and test, one task per check, each tagged `check-fast`.
  - Each task names `/crates/flywheel-control/**/*`, `/devenv.nix`, `/devenv.lock`, `/.prototools` and `/rust-toolchain.toml` among its inputs.
  - `flywheel`'s test task names `/crates/flywheel-hosted/**/*`.
- **treefmt.** `treefmt.nix` turns on `programs.jsonfmt`, and still excludes `crates/*/tests/fixtures/**` by name, with its reason.
- **The schema.** `protocol/v1/` holds the nine schema files of Decision 2, with examples of every kind in both directions. `protocol/README.md` says what Decision 2 lists for it:
  - the versions, and when one is fixed;
  - the messages and their order;
  - the copies;
  - the terminal's frames, and the context their signatures are made over;
  - the subprotocol, `flywheel-runner.v1`;
  - that no validator fetches a `$id`;
  - that the records and rules are its source.
- **The types.** `flywheel_hosted::protocol::v1` holds `VERSION`, `ADDRESS`, `FILES` and the types of Decision 13.
  - It uses `flywheel_control`'s `Want`, `Lease`, `Id`, `At`, `Answer`, `ToolVersion`, `Acted`, `Commanded` and `InboxCarried` as they are.
  - Every struct of version 1's own takes `deny_unknown_fields`.
  - A copy, and each field of `HostPart`, is a `Box<RawValue>`.
- **The docs.** `docs/architecture/core.md` holds core.42 and its details, and `docs/adr/0015` holds the decision, which is proposed. README and CLAUDE.md name the crate, its project and `protocol/`.

The branch touches only what the proposal's Touches names, beside the change itself. Task notes 1.1 also asks for a line in the root `moon.yml`'s comment, but that file keeps no list of projects, so there is nothing to add. `main` stands at 8ef1992, the branch's merge base.

## Checks

Both were run through sluice, so its sandbox and `network = "localhost"` applied:

- **The gate.** `sluice exec main` ran all 21 `check-fast` tasks afresh and passed every one. It recorded their results under b64b341.
  - `flywheel-hosted:test` ran 30 tests: files 3, forms 2, examples 5, interface 1, copies 5, refused 13, names 1. None failed and none was ignored.
  - `flywheel:test` ran `rules::the_crates_depend_as_decision_1_has_them` and `rules::only_the_hosted_implementations_crate_holds_the_protocol`, and both passed.
  - `root:changes` validated this change under `--strict`.
- **Task 4.1's command.** `sluice exec main -- moon run flywheel-hosted:format flywheel-hosted:lint flywheel-hosted:typecheck flywheel-hosted:test flywheel:test root:format root:adrs root:specs root:changes` passed all 9 tasks. Five of them replayed the gate's passes, whose inputs were the same.

moon warned that the sandbox could not write its output archives or its shared cache. The tasks' results were unaffected.

## Trial archive

`openspec archive the-runner-keeps-its-protocol-as-one-json-schema-per-version --yes` was run with the pinned openspec 1.14.3 on a copy of `openspec/` in a scratch directory. It applied every delta, + 12 and ~ 1:
- it created `runner-protocol`;
- it updated `control-plane-interface`, and the MODIFIED header found its requirement.

The archive also warned, without blocking, that the Why section passes 1000 characters and that the change has more than 10 deltas.

`openspec validate --specs --strict` then passed 9 specs and failed 2, as *What fails* says.

## Scenarios

Each scenario's test was read against its WHEN and THEN. A test `x::y` is the function `y` in `crates/flywheel-hosted/tests/x.rs`, and `rules::y` is in `crates/flywheel/tests/rules.rs`.

### runner-protocol (12 requirements, 28 scenarios)

| Scenario | Proved by | Note |
|---|---|---|
| Each file is a schema of its draft | `files::each_file_is_a_schema_of_its_draft` | each file against `jsonschema::draft202012::meta`, its `$schema` and its `$id` |
| Nothing outside the version is referenced | `files::nothing_outside_the_version_is_referenced` | builds every file and every `$defs` entry from `FILES` alone. The retriever refuses every address, so nothing is fetched whatever features cargo turns on. An address outside the version fails to resolve |
| Every file is listed | `files::every_file_is_listed` |  |
| A message with no id | `refused::a_message_with_no_id` | an id that is missing, empty or holds a space, and one of 129 characters |
| A message of another version | `refused::a_message_of_another_version` | 2, and also 0 and `"1"` |
| A body of two kinds, or of none | `refused::a_body_of_two_kinds_or_of_none` | two kinds, an empty body, an unknown kind, and no body |
| A body of the other direction | `refused::a_body_of_the_other_direction` | the two named, then every example of a kind only one direction sends |
| A field the version does not name | `refused::a_field_the_version_does_not_name` | the envelope, a launch, a lease, a status copy and a share. The runner's types also refuse the envelope's and the share's |
| A time not in UTC to the second | `forms::a_time_not_in_utc_to_the_second` | the two times named |
| An integer past what JavaScript reads exactly | `forms::an_integer_past_what_javascript_reads_exactly` | 2^53 and 0 refused, and 2^53 − 1 passes |
| The interface's values pass | `interface::the_interfaces_values_pass` | every `Want`, `ActSort` and `CommandSort` variant the interface has |
| An act's outcome | `refused::an_acts_outcome` |  |
| The copy's fields are the list | `copies::the_copys_fields_are_the_list` | the test's list, read against Decision 7's table, matches row for row |
| Text the cloud does not keep | `refused::text_the_cloud_does_not_keep` | all eight named |
| Flume's answers as copies | `copies::flumes_answers_as_copies` | the seven examples named |
| The capacity's fields are the list | `copies::the_capacitys_fields_are_the_list` |  |
| A stream's queue | `refused::a_streams_queue` |  |
| The entry's fields are the list | `copies::the_entrys_fields_are_the_list` |  |
| A row's log | `refused::a_rows_log` |  |
| A reported with no report named | `refused::a_reported_with_no_report_named` | and one naming an empty id |
| A message refused | `examples::a_message_refused` | each direction's |
| A frame with no offset | `refused::a_frame_with_no_offset` |  |
| A frame in the clear | `refused::a_frame_in_the_clear` | both kinds: bytes as `text`, bytes beside `sealed`, and four `sealed` that are not base64url |
| An unsigned share | `refused::an_unsigned_share` |  |
| A terminal's messages | `examples::a_terminals_messages` | each kind, and a frame both with and without a redraw |
| Every name read | `names::every_name_read` | each member's name at any depth, and each field a `required` lists |
| Every example read and written back | `examples::every_example_read_and_written_back` |  |
| Every kind has an example | `examples::every_kind_has_an_example` |  |

### control-plane-interface (1 requirement, 2 scenarios)

| Scenario | Proved by | Note |
|---|---|---|
| Only crew's crate names crew | `rules::only_crews_crate_names_crew`, `rules::the_crates_depend_as_decision_1_has_them` | unchanged from main |
| Only the hosted implementation's crate holds the protocol | `rules::only_the_hosted_implementations_crate_holds_the_protocol` | it reads `crates/flywheel/moon.yml` with its one input taken out |

## Beyond the scenarios

These were checked by hand or in scratch, and nothing of them is in the tree:

- **Closed objects.** A walk of every schema file finds only these objects open:
  - the maps: a hello's `tool_versions` and `leases`, a report's four parts, an inbox's `files`, a reported's `refused`, and a copy's `hold_words`. Each constrains its keys with `propertyNames` and its values with a closed definition or a string.
  - the made thing, carried whole.

  A row's `flaky` is untyped, as Decision 4 names it.
- **Required fields.** Outside the place's copy, every field is required, except sluice's entry and its rows' fields. Decision 9 keeps those where sluice printed each.
- **Patterns.** All eight patterns use only anchors, classes, counts and alternation, as Decision 2 asks.
- **Real statuses, cut.** All 11 of Flume 0.4.1's status fixtures with a status hold 23 places and 11 host parts. Each was cut by the test's lists, with `hold_words`, `recipients` and a made thing kept whole. All 34 copies passed `place_status` and `host_part`, under a validator built from `FILES` as the tests build theirs. So 1.15's cut of a real status will pass the cloud's check.
- **core.41.** The examples' paths are `/home/runner/places/…` and their repositories `example/…`, as Task notes 2.3 asks. No path of a fixture's is in them. The fixtures' own home paths are main's, and `_open.md` already names them under core.41.
