# Tasks

Build the groups in order. Each group stands on the ones before it, and ends green on the moon tasks it names. A task is true once the moon task it names passes, run as it names it.

- Run each check as `devenv shell -- moon run <project>:<task>`, or through sluice while iterating, as CLAUDE.md says.
- Every test runs on its crate's fakes and stubs, in a temporary folder, with neutral names and paths (core.41). No test reads the user's configuration, touches a real place, clone or state repository, or reaches the network.
- No task adds a crate, a tool or a tracked file type (core.7, core.8). Nothing outside the files proposal.md's Touches lists is changed.

## 1. Every place the runner launched is followed to its end

- [x] 1.1 A pass carries the status of a place whose repository the configuration no longer keeps, asked for by its location alone, through its landing and its tidied record, and tells no keeper of the landing (design Decisions 1 and 2; Task notes 1.1). True when `devenv shell -- moon run boomdrive:test` passes with `tests/statuses.rs` proving the work-into-flume spec's "A place whose repository the runner keeps no more", "A place found tidied by its mark" and "A landing in a repository not kept", and `tests/flume.rs` the call with and without a container.
- [x] 1.2 crew's implementation gives a launch the runner answered whatever the hello names (design Decision 3; Task notes 1.2). True when `devenv shell -- moon run boomdrive-crew:test` passes with `tests/claims.rs` proving the crew-control-plane spec's "A launch answered before its repository left the hello" and "An answered launch whose name two kept addresses share".
- [x] 1.3 README says a place is followed to its end. True when `devenv shell -- moon run root:format` passes.
- [x] 1.4 Run this group's checks. True when `devenv shell -- moon run boomdrive:format boomdrive:lint boomdrive:typecheck boomdrive:test boomdrive-crew:format boomdrive-crew:lint boomdrive-crew:typecheck boomdrive-crew:test` passes.

## 2. The change's checks

- [x] 2.1 Run the checks of every moon project this change touches. True when `devenv shell -- moon run boomdrive:format boomdrive:lint boomdrive:typecheck boomdrive:test boomdrive-crew:format boomdrive-crew:lint boomdrive-crew:typecheck boomdrive-crew:test root:format root:adrs root:specs root:changes` passes.
- [x] 2.2 The spec delta archives into specs that pass openspec's strict check. True when, after `openspec archive the-runner-follows-each-place-it-launched-to-its-end --yes`, `devenv shell -- moon run root:specs root:changes` passes on the tree.
