# Tasks

Build the groups in order. A task is true once the check it names passes, run as it names it.

- Every test runs on the scratch host of `crates/boomdrive/tests/fakes`, with neutral names and paths (core.41), and no real Flume.
- No task adds a crate, a tool or a tracked file type (core.7, core.8). Nothing outside the files proposal.md's Touches lists is changed.

## 1. A pull's launches go ahead of its acts on places

- [x] 1.1 A pass answers the pull in two rounds, each in the control plane's order, the acts that run Flume's command on a place in the second (design Decisions 1 and 2). True when `devenv shell -- cargo test -p boomdrive --test acts` passes with `a_launch_ahead_of_acts_on_places_that_are_over`, `an_act_on_a_launch_the_same_pass_takes` and `a_drop_ahead_of_its_held_launch_keeps_it_unmade` proving the work-into-flume spec's "A launch ahead of acts on places that are over", "An act ahead of its own launch" and "A drop ahead of a held launch", under its "A pull's launches go ahead of its acts on places" and "No answer turns on the round a want is taken in".
- [x] 1.2 Every scenario of the work-into-flume spec still holds. True when `sluice exec main -- moon run boomdrive:test` passes.
- [x] 1.3 The pass's code passes its lint. True when `sluice exec main -- moon run boomdrive:lint` passes.
- [x] 1.4 core.49 is added, core.1's details allow it, records 0002 and 0010 carry its amendments, and the README says when a control pass carries out an act on a place. True when `sluice exec main -- moon run root:format root:adrs` passes.
- [x] 1.5 The spec delta archives into specs that pass openspec's strict check. True when, after `openspec archive a-launch-goes-ahead-of-other-acts --yes`, `sluice exec main -- moon run root:specs root:changes` passes.
