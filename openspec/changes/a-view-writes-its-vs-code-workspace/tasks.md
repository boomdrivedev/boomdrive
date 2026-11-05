# Tasks

Build the groups in order. Each group stands on the ones before it, is one session's work for an agent that has read this change (proposal, design, specs) and its Task notes, and ends green on the moon tasks it names. A task is true once the moon task it names passes, run as it names it.

- Run each check as `devenv shell -- moon run <project>:<task>`, or through sluice while iterating, as CLAUDE.md says.
- Every test runs on the scratch host of `crates/boomdrive/tests/common`, with neutral names and paths (core.41). No test reads the user's configuration or reaches the network.
- Every key, type and module a task adds names what it holds (core.12), as design Decisions 2 and 4 name them.
- No task adds a crate, a tool or a tracked file type (core.7, core.8). Nothing outside the files proposal.md's Touches lists is changed.

## 1. The setting

- [x] 1.1 `src/config.rs` reads `code_workspaces`: false where it is absent, and a value that is not `true` or `false` refused, naming the key and the value (configuration spec: "Whether each owner's views have a VS Code workspace is a setting"; design Decision 2; Task notes 1.1). True when `devenv shell -- moon run boomdrive:test` passes with a unit test for each of that requirement's scenarios "No workspaces setting" and "Workspaces asked for in words", and `a_whole_file` reading the key.
- [x] 1.2 README's configuration example sets `code_workspaces = true`, and its words say what the key asks for (Task notes 1.2). True when `devenv shell -- moon run boomdrive:test` passes with `the_readme_configuration_reads` asserting the key is read as true.
- [x] 1.3 Run this group's checks. True when `devenv shell -- moon run boomdrive:format boomdrive:lint boomdrive:typecheck boomdrive:test` passes.

## 2. Each owner's workspace

- [x] 2.1 `crates/boomdrive/Cargo.toml` takes `serde_json` with `preserve_order`, and `Cargo.lock` gains no package. `src/code_workspace.rs` gives each owner's workspace and its folders from the configuration. It decides from what stands at the workspace's path whether it is made, set, left as it is, or left and named, and writes a file beside it renamed over it (design Decisions 1, 3, 4 and 5; Task notes 2.1). True when `devenv shell -- moon run boomdrive:test` passes with the unit tests Task notes 2.1 lists.
- [x] 2.2 The pass keeps each owner's workspace after the configured repositories are kept and the dropped ones let go, says its lines, and fails where a workspace cannot be read or written (design Decision 6; Task notes 2.2). True when `tests/code_workspaces.rs` proves the views spec's scenarios "The first workspaces", "No workspace asked for", "A kept repository with no view", "The user's settings kept", "Folders edited by hand", "A dropped repository leaves its workspace", "A quiet pass leaves the workspaces", "A comment in the workspace", "A link at the workspace's path" and "A workspace that cannot be written", in `devenv shell -- moon run boomdrive:test`.
- [x] 2.3 A running runner reads `code_workspaces` again at every pass (Task notes 2.3). True when `tests/loop.rs` proves the configuration spec's "Workspaces asked for while running" in `devenv shell -- moon run boomdrive:test`.
- [x] 2.4 Run this group's checks. True when `devenv shell -- moon run boomdrive:format boomdrive:lint boomdrive:typecheck boomdrive:test` passes.

## 3. A workspace goes with its owner's folder

- [ ] 3.1 A view removed, by a dropped repository's let-go or by a changed root, takes its owner's workspace and folder as design Decision 7 has it, and leaves them where `code_workspaces` is off (Task notes 3.1). True when, in `devenv shell -- moon run boomdrive:test`, `tests/dropped.rs` proves the views spec's "The owner's last view, with its workspace", "An owner still kept keeps its workspace" and "A workspace not asked for", and `tests/views.rs` its "A new root, the old owner's workspace", with "The owner's last view" and "The owner's folder holds more" still passing.
- [ ] 3.2 Run this group's checks. True when `devenv shell -- moon run boomdrive:format boomdrive:lint boomdrive:typecheck boomdrive:test` passes.

## 4. The change's checks

- [ ] 4.1 Run the checks of every moon project this change touches. True when `devenv shell -- moon run boomdrive:format boomdrive:lint boomdrive:typecheck boomdrive:test root:format root:adrs root:specs root:changes` passes.
- [ ] 4.2 The spec deltas archive into specs that pass openspec's strict check, each requirement's text within openspec's 500 characters. True when, after `openspec archive a-view-writes-its-vs-code-workspace --yes` on a copy of `openspec/` with the pinned openspec, `openspec validate --specs --strict` passes all 11 specs there, and `devenv shell -- moon run root:specs root:changes` passes on the tree.
