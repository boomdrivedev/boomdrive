# Tasks

Build the groups in order. A task is true once the check it names passes, run as it names it.

- Every test runs on the scratch host of `crates/boomdrive/tests/fakes`, with neutral names and paths (core.41), and no real Flume.
- No task adds a crate, a tool or a tracked file type (core.7, core.8). Nothing outside the files proposal.md's Touches lists is changed.

## 1. An act on a place Flume does not stand costs the reading of Flume's refusal

- [x] 1.1 An act on a place the pass's listing does not hold reads Flume's status of it first, and Flume's refusal of it is the act's answer (design Decision 1). True when `devenv shell -- cargo test -p boomdrive --test acts` passes with `a_launch_ahead_of_acts_on_places_that_are_over`, `an_act_on_a_place_the_listing_does_not_hold_which_flume_stands` and `an_act_whose_places_status_flume_could_not_be_read_for` proving the work-into-flume spec's "A launch ahead of acts on places that are over", "An act on a place the listing does not hold, which Flume stands" and "A place whose status Flume could not be read for".

## 2. A launch waits on no act of the pass before its own

- [x] 2.1 Once the next control pass is due, a pass runs no act's command but its first (design Decision 2). True when `devenv shell -- cargo test -p boomdrive --test acts` passes with `acts_on_places_wait_for_the_next_pass_once_it_is_due` proving the spec's "Acts on places wait for the next pass once it is due", under its "A launch waits on no act of the pass before its own".
- [x] 2.2 A wake is read by the pass under way through every clone of the runner's notices. True when `devenv shell -- cargo test -p boomdrive --lib run::tests::a_wake_is_read_by_the_pass_under_way` passes.

## 3. The gate

- [x] 3.1 Every scenario of the work-into-flume spec still holds. True when `sluice exec main -- moon run boomdrive:test` passes.
- [x] 3.2 The code passes its lint and format. True when `sluice exec main -- moon run boomdrive:lint boomdrive:format` passes.
- [x] 3.3 core.50 and core.51 are in core.md, core.1's details allow them, records 0002 and 0010 carry their amendments, and the README says when a control pass carries out an act on a place. True when `sluice exec main -- moon run root:format root:adrs` passes.
- [x] 3.4 The spec delta archives into specs that pass openspec's strict check. True when, after `openspec archive a-launch-waits-on-no-other-act --yes`, `sluice exec main -- moon run root:specs root:changes` passes.
