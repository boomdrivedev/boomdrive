# Tasks

Work through the tasks in order. A task is true once the check it names passes, run as it names it.

- Every test runs on the crate's stubs and fakes, in a temporary folder, with neutral names and paths (core.41). No test runs a Flume of the host's.
- No task adds a crate, a tool or a tracked file type (core.7, core.8). Nothing outside the files proposal.md's Touches lists is changed.
- Record 0029's amendment and core.13's, core.1's and core.49's are in the commit that holds this change; no task writes them again.

## 1. Each pass ends with a settle, on its own thread

- [x] 1.1 A keeping pass ends with the settle on the loop, and each part's control pass on the part's thread; the first keeping pass shares the parts' first settles, proving the running spec's "Passes back to back share one settle", "Each control pass ends with a settle", "Each keeping pass ends with a settle" and "A pass on a broken configuration settles" (design Decisions 1 and 4). True when `devenv shell -- cargo test -p boomdrive --test control -- passes_back_to_back_share_one_settle each_control_pass_ends_with_a_settle` and `devenv shell -- cargo test -p boomdrive --test loop -- each_keeping_pass_ends_with_a_settle a_file_broken_while_running` pass.
- [x] 1.2 A pass that ends while another's settle is under way shares it, running none and waiting for nothing, proving the running spec's "A pass ends while a settle is under way" (design Decision 2). True when `devenv shell -- cargo test -p boomdrive --lib run::tests::a_settle_under_way_is_shared` passes.

## 2. What the runner says of it

- [x] 2.1 README says each pass, keeping or a part's control pass, ends with the settle on its own thread, the first keeping pass sharing the parts' first, and a pass ending during another's settle sharing it (design Decisions 1 and 2). True when `sluice exec main -- moon run root:format boomdrive:test` passes.

## 3. The specs

- [x] 3.1 The change's delta archives into a spec that passes openspec's strict check. True when, after `devenv shell -- openspec archive each-parts-pass-ends-with-a-settle --yes`, `sluice exec main -- moon run root:specs` passes.
