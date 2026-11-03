# Verify: the-runner-keeps-its-protocol-as-one-json-schema-per-version

The work on `change/the-runner-keeps-its-protocol-as-one-json-schema-per-version`, read at 7c51191, holds, and the change is archived with this report.

- Every task is ticked.
- Every check passes, and so does the gate run through sluice.
- The change has 13 requirements and 29 scenarios, and a test asserts the outcome of every scenario.
- A trial archive merges every delta into specs that pass `openspec validate --specs --strict`, which `root:specs` runs.

`main` stands at 8ef1992, the branch's merge base. The code, the schema and the tests are as they were at b64b341. The commits after it touch only the change's own files.

## Tasks

All 16 tasks of `tasks.md`, groups 1 to 4, are ticked `[x]`, and none is open. Each group's closing task names the checks that ran it. Those checks pass at 7c51191 (below).

What the tasks build is in the tree:

- **The crate.** `crates/flywheel-hosted` is a member of the cargo workspace.
  - Its manifest sets `publish = false` and `doctest = false`, and takes the workspace's version and edition.
  - It depends on `flywheel-control` by path, on `serde`, and on `serde_json`, which takes `raw_value` from the workspace.
  - Its one dev-dependency is `jsonschema`, which the workspace takes at 0.60 with `default-features = false`. `Cargo.lock` gains only the new crate, since it already held `jsonschema` 0.60.1.
- **The moon project.** It is `flywheel-hosted` in `.moon/workspace.yml`, with `language: 'rust'`.
  - It has format, lint, typecheck and test, one task per check, each tagged `check-fast`.
  - Each task names `/crates/flywheel-control/**/*`, `/devenv.nix`, `/devenv.lock`, `/.prototools` and `/rust-toolchain.toml` among its inputs.
  - `flywheel`'s test task names `/crates/flywheel-hosted/**/*`.
- **treefmt.** `treefmt.nix` turns on `programs.jsonfmt`. It still excludes `crates/*/tests/fixtures/**` by name, with its reason.
- **The schema.** `protocol/v1/` holds the nine schema files of Decision 2, and examples of every kind in both directions: 17 from the runner and 8 to it.
- **The README.** `protocol/README.md` says what Decision 2 lists for it:
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
- **The docs.**
  - `docs/architecture/core.md` holds core.42 and its details.
  - `docs/adr/0015` holds the decision, proposed until the user approves the change in its review.
  - README and CLAUDE.md name the crate, its project and `protocol/`.

The branch touches only what the proposal's Touches names, beside the change itself.

## Checks

All were run through sluice, so its sandbox and `network = "localhost"` applied.

- **The gate.** `sluice exec main` passed all 21 `check-fast` tasks and recorded its result under 7c51191.
- **Task 4.1's command, forced.** `sluice exec main -- moon run --force flywheel-hosted:format flywheel-hosted:lint flywheel-hosted:typecheck flywheel-hosted:test flywheel:test root:format root:adrs root:specs root:changes` passed all 9 tasks, none replayed from moon's cache.
  - `flywheel-hosted:test` ran 30 tests, and none failed or was ignored. By file: copies 5, examples 5, files 3, forms 2, interface 1, names 1, refused 13.
  - `flywheel:test`'s rules test passed four tests:
    - `the_crates_depend_as_decision_1_has_them`;
    - `only_the_hosted_implementations_crate_holds_the_protocol`;
    - `only_crews_crate_names_crew`;
    - `every_crate_is_a_moon_project_of_its_own`.
  - `root:changes` validated this change under `--strict`, and `root:specs` passed main's 10 specs.
  - `root:format` formatted 138 files and changed none.

moon warned that the sandbox could not write its output archives. The tasks' results were unaffected.

## Trial archive (task 4.2)

`openspec archive the-runner-keeps-its-protocol-as-one-json-schema-per-version --yes` was run with the pinned openspec 1.14.3, on a copy of `openspec/` in a scratch directory. It applied every delta, + 13 in all:

- it created `runner-protocol`, + 12;
- it added the requirement "The protocol is held in the hosted implementation's crate alone" to `control-plane-interface`, + 1.

The archive also warned, without blocking, that the Why section passes 1000 characters and that the change has more than 10 deltas.

`openspec validate --specs --strict` then passed all 11 specs.

- `runner-protocol` holds its delta's own `## Purpose`.
- No requirement of either spec passes openspec's 500 characters. The longest are "Each version of the protocol is JSON Schema in the hosted implementation's crate", at 476, and "A terminal's frames are sealed end to end and carry their offsets", at 466. The new `control-plane-interface` requirement has 239.
- `control-plane-interface` is main's, with the new requirement and its scenario appended. Its one-interface requirement stands as on main, so its scenario "Only crew's crate names crew" is unchanged.
- `openspec validate --changes --strict` on the archived copy finds no open change and exits 0.

## Scenarios

Each scenario's test was read against its WHEN and THEN. A test `x::y` is the function `y` in `crates/flywheel-hosted/tests/x.rs`, and `rules::y` is in `crates/flywheel/tests/rules.rs`.

### runner-protocol (12 requirements, 28 scenarios)

| Scenario | Proved by | Note |
|---|---|---|
| Each file is a schema of its draft | `files::each_file_is_a_schema_of_its_draft` | each file against `jsonschema::draft202012::meta`, its `$schema`, and its `$id` its own address under `v1/` |
| Nothing outside the version is referenced | `files::nothing_outside_the_version_is_referenced` | builds every file and every `$defs` entry from `FILES` alone. The retriever refuses every address, so nothing is fetched whatever features cargo turns on. An address outside the version fails to resolve |
| Every file is listed | `files::every_file_is_listed` | and no file listed twice |
| A message with no id | `refused::a_message_with_no_id` | an id missing, empty, or holding a space, and one of 129 characters |
| A message of another version | `refused::a_message_of_another_version` | 2, and also 0 and `"1"` |
| A body of two kinds, or of none | `refused::a_body_of_two_kinds_or_of_none` | two kinds, an empty body, an unknown kind, and no body |
| A body of the other direction | `refused::a_body_of_the_other_direction` | the two named, then every example of a kind only one direction sends |
| A field the version does not name | `refused::a_field_the_version_does_not_name` | the envelope, a launch, a lease, a status copy and a share. The runner's types also refuse the envelope's field and the share's |
| A time not in UTC to the second | `forms::a_time_not_in_utc_to_the_second` | the two times named |
| An integer past what JavaScript reads exactly | `forms::an_integer_past_what_javascript_reads_exactly` | 2^53 and 0 refused, and 2^53 − 1 passes |
| The interface's values pass | `interface::the_interfaces_values_pass` | every `Want`, `ActSort` and `CommandSort` variant `flywheel-control` has, each written by its own types |
| An act's outcome | `refused::an_acts_outcome` | |
| The copy's fields are the list | `copies::the_copys_fields_are_the_list` | the test's list, read against Decision 7's table, matches row for row |
| Text the cloud does not keep | `refused::text_the_cloud_does_not_keep` | all eight named, each from an example that passes first |
| Flume's answers as copies | `copies::flumes_answers_as_copies` | the seven examples named |
| The capacity's fields are the list | `copies::the_capacitys_fields_are_the_list` | matches Decision 8's table |
| A stream's queue | `refused::a_streams_queue` | |
| The entry's fields are the list | `copies::the_entrys_fields_are_the_list` | matches Decision 9's table |
| A row's log | `refused::a_rows_log` | |
| A reported with no report named | `refused::a_reported_with_no_report_named` | and one naming an empty id |
| A message refused | `examples::a_message_refused` | each direction's |
| A frame with no offset | `refused::a_frame_with_no_offset` | |
| A frame in the clear | `refused::a_frame_in_the_clear` | both kinds: bytes as `text`, bytes beside `sealed`, and four `sealed` values that are not base64url |
| An unsigned share | `refused::an_unsigned_share` | |
| A terminal's messages | `examples::a_terminals_messages` | each kind, and a frame both with and without a redraw |
| Every name read | `names::every_name_read` | each member's name at any depth, and each field a `required` lists, in the singular or the plural |
| Every example read and written back | `examples::every_example_read_and_written_back` | |
| Every kind has an example | `examples::every_kind_has_an_example` | |

### control-plane-interface (1 requirement, 1 scenario)

| Scenario | Proved by | Note |
|---|---|---|
| Only the hosted implementation's crate holds the protocol | `rules::only_the_hosted_implementations_crate_holds_the_protocol` | reads every file under `crates/` outside the crate, `crates/flywheel/moon.yml` with its one input taken out. `planes.rs` does not name the crate yet, which the scenario allows |

## Beyond the scenarios

These were checked by hand, and nothing of them is in the tree:

- **core.41.** The examples' paths are all `/home/runner/…`, and their repositories `example/…`. No person's path, host, account or private repository is named in the crate.
- **The other tests.** `examples::every_example_passes_its_directions_schema` and `copies::a_thing_an_end_made_is_carried_whole` hold what Decisions 7 and 14 add beyond the scenarios.
