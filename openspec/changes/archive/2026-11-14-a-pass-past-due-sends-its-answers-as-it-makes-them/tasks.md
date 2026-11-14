# Tasks

A task is true once the check it names passes, run as it names it.

- Every test runs on the scratch host of `crates/boomdrive/tests/fakes`, with neutral names and paths (core.41), and no real Flume.
- No task adds a crate, a tool or a tracked file type (core.7, core.8). Nothing outside the files proposal.md's Touches lists is changed.

## 1. A pass past due sends each answer as it makes it

- [x] 1.1 Once the next pass is due, a pass sends its report after each want whose answer changed it, and a pass not yet due sends once, at its end (design Decision 1). True when `devenv shell -- cargo test -p boomdrive --test sends` passes with `a_pass_past_due_sends_each_launchs_answer_before_its_next_launch`, `a_pass_not_yet_due_sends_once_at_its_end` and `an_acts_answer_goes_up_before_the_next_act_runs` proving the work-into-flume spec's "A pass past due sends each launch's answer before its next launch", "A pass not yet due sends once" and "An act's answer goes up before the next act runs".
- [x] 1.2 What the control plane let go or refused at a send is taken no more that pass (design Decision 2). True when `devenv shell -- cargo test -p boomdrive --test sends` passes with `a_want_refused_at_a_send_of_the_pass_is_taken_no_more` proving the spec's "A want let go at a send of the pass is taken no more", under its "What a send of the pass lets go is taken no more that pass".
- [x] 1.3 A send that fails leaves the pass's answers to its end (design Decision 3). True when `devenv shell -- cargo test -p boomdrive --test sends` passes with `a_send_that_fails_leaves_the_passs_answers_to_its_end` proving the spec's "A send that fails leaves the pass's answers to its end", under its "A send that fails before a pass's end leaves its answers to the end".

## 2. The gate

- [x] 2.1 Every scenario of the work-into-flume spec still holds. True when `sluice exec main -- moon run boomdrive:test` passes.
- [x] 2.2 The code passes its lint and format. True when `sluice exec main -- moon run boomdrive:lint boomdrive:format` passes.
- [x] 2.3 core.50's details and record 0002 say how the amendment was built, and the README says when a control pass sends its report. True when `sluice exec main -- moon run root:format root:adrs` passes.
- [x] 2.4 The spec delta archives into specs that pass openspec's strict check. True when, after `openspec archive a-pass-past-due-sends-its-answers-as-it-makes-them --yes`, `sluice exec main -- moon run root:specs root:changes` passes.
