# Tasks

Build the tasks in order. A task is true once the check it names passes, run as it names it. The groups are headings, for reading.

- Every test runs on the scratch host of `crates/boomdrive/tests/fakes`, with neutral names and paths (core.41), and no real Flume or sluice.
- No task adds a crate, a tool or a tracked file type (core.7, core.8). Nothing outside the paths proposal.md's Touches lists is changed. Record 0002's amendment and core.52 are already written with the change.

## 1. Every command is refused, naming its tool

- [x] 1.1 `Work::command` answers every command under a live lease `refused`, with `this binary does not carry <tool>'s commands`, `sluice` for a branch command, and reads neither its repository nor its clone and runs nothing. `Work::branch` is gone, and `tests/branches.rs` becomes `tests/commands.rs` (design Decision 4; Task notes 1.1). True when `devenv shell -- cargo test -p boomdrive --test commands` passes with `a_branch_command`, `a_clone_not_yet_made` and `a_boom_command`, proving the commands spec's "A branch command", "A clone not yet made" and "A boom command".
- [x] 1.2 A command the report holds is answered from it, and a branch command an earlier release carried out keeps its text and origin commit and runs nothing (design Decisions 4 and 5; Task notes 1.2). True when `devenv shell -- cargo test -p boomdrive --test commands` passes with `a_command_given_twice` and `a_branch_command_an_earlier_release_carried_out`, proving the commands spec's "A command given twice" and "A branch command an earlier release carried out".
- [x] 1.3 A branch command whose lease lapsed has no answer, and under a live lease is refused, naming sluice. True when `devenv shell -- cargo test -p boomdrive --test leases a_command_whose_lease_lapsed` passes, proving the work-into-flume spec's "A command whose lease lapsed".
- [x] 1.4 A pass whose wants answered only commands reads Flume's listing once (design Decision 6; Task notes 1.4). True when `devenv shell -- cargo test -p boomdrive --test statuses a_pass_that_answers_a_command_reads_the_listing_once` passes, proving the work-into-flume spec's "A pass that answers a command reads the listing once".

## 2. The runner keeps nothing that asks sluice for a branch

- [x] 2.1 `Sluice::branch`, `Cli::branch` and `Printed` are gone from `crates/boomdrive/src/sluice.rs`, whose module documentation no longer names a branch it makes. `clone::origin_commit` is gone. sluice 0.61.1's `branch-made.json`, `branch-there.json` and `branch-ungated.json` are gone, with the fixtures README's section *A bolt's branch*, and so are `tests/sluice.rs`'s tests of a branch and its `READ` entries for them (design Decision 4; Task notes 2.1). True when `devenv shell -- cargo test -p boomdrive --test sluice every_fixture_is_read` passes, and `sluice exec main -- moon run boomdrive:lint` passes, finding nothing left unused.
- [x] 2.2 No crate's source asks sluice for a branch (core.52; Task notes 2.2). True when `devenv shell -- cargo test -p boomdrive --test rules no_crate_asks_sluice_for_a_branch` passes.
- [x] 2.3 `tests/bolts.rs` passes clippy's `needless_borrows_for_generic_args`: the send's act id is made as `Id::new(format!("{BOLT}/send"))`, not `Id::new(&format!("{BOLT}/send"))`. True when `sluice exec main -- moon run boomdrive:lint` passes.

## 3. A bolt is a launch, and its send a retry of its submission

- [x] 3.1 A launch of a kind that lands with no stages is taken, counted against its stream's room and launched as any launch, and its unit after it on its branch (design Decisions 1 and 2; Task notes 3.1 and 3.2). True when `devenv shell -- cargo test -p boomdrive --test bolts` passes with `a_bolt_and_its_unit` and `a_bolt_counts_against_its_streams_room_within_its_pass`, proving the work-into-flume spec's "A bolt and its unit" and "A bolt counts against its stream's room within its pass".
- [x] 3.2 A bolt's send is `flume --json retry <place> --submission`, and Flume's refusal of it is the act's answer (design Decision 3; Task notes 3.1 and 3.2). True when `devenv shell -- cargo test -p boomdrive --test bolts` passes with `a_bolts_send` and `a_send_flume_refuses`, proving the work-into-flume spec's "A bolt's send" and "A send Flume refuses".

## 4. The words

- [x] 4.1 `CommandSort::Branch`'s documentation says a control plane may still send it, as version 1 of the hosted protocol names it, and this binary refuses it, naming sluice (core.42, core.52). `Commanded::origin_commit`'s says no command this release answers fills it, and that it is kept for a branch command an earlier release carried out (core.36). No shape changes (design Decision 5). True when `devenv shell -- cargo test -p boomdrive-control --test json` passes.
- [x] 4.2 The README's control pass says a bolt is a launch like any other, its send a retry naming its submission, and that every command is refused, naming its tool, and no longer names `sluice branch create` (design Decision 8). `tests/readme.rs` gains `the_readme_names_no_branch_command`, which fails where the README names `sluice branch create`. True when `devenv shell -- cargo test -p boomdrive --test readme` passes, `the_readme_names_no_branch_command` among it.
- [ ] 4.3 The spec deltas archive into specs that pass openspec's strict check, the branch-commands spec retired and the commands spec made. True when, after `devenv shell -- openspec archive a-bolt-is-a-launch --yes`, `sluice exec main -- moon run root:specs` passes.
