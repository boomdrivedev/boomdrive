# Verify: a-view-writes-its-vs-code-workspace

The work on `change/a-view-writes-its-vs-code-workspace`, read at 9c10725, holds, and the change is archived with this report.

- Every task is ticked.
- Every check passes, and so does the gate run through sluice.
- The change has 5 requirements and 19 scenarios, and a test asserts the outcome of every scenario.
- A trial archive merges both deltas into specs that pass `openspec validate --specs --strict`, which `root:specs` runs.

`main` stands at c5bff41, the branch's merge base, as the design's Context names it.

## Tasks

All 11 tasks of `tasks.md`, groups 1 to 4, are ticked `[x]`, and none is open. Each group's closing task names the checks that ran it. Those checks pass at 9c10725 (below).

What the tasks build is in the tree:

- **The setting.** `Config` has `code_workspaces: bool`, false where the file has none. A value that is not a TOML boolean is refused at its line, `` `code_workspaces` is <value>: it is `true` or `false` ``.
- **The README.** Its configuration block sets `code_workspaces = true` after `fetch_interval`. Its words say what the key asks for, that a pass keeps the workspaces, and that `--once` exits 1 for a workspace not read or written but not for one left.
- **The dependency.** `boomdrive` takes `serde_json` with `preserve_order`. `Cargo.lock` gains no package: `serde_json`'s entry gains `foldhash` and `indexmap`, both already in it.
- **The module.** `src/code_workspace.rs` holds `wanted`, `stands`, `decide` and `write` as Task notes 2.1 names them, with `partial` and `alone_in` for Decision 7.
  - `wanted` gathers owners without case, takes the first repository's spelling, and gives `../<owner as written>/<name>` for a repository written otherwise.
  - `decide` makes `{"folders": [...], "settings": {}}` with tabs, leaves equal `folders` compared as JSON values, sets `folders` where they stand or adds them first, and names a link, a folder, JSON that is not an object, and text `serde_json` cannot read, with its error and never the file's text.
  - `write` makes the owner's folder, writes the partial file with `create_new`, keeps an old file's permissions, renames it over, and removes the partial file where any step fails.
- **The pass.** `pass::pass` keeps the workspaces after the configured repositories and the dropped ones' let-go, before the unfinished clones are cleared, and only where the pass was not stopped. `keep_one` keeps none. A read or write that fails fails the pass; something left is named and does not.
- **The removal.** `view::remove` gives the owner's folder it left standing. `let_go` and `let_go_of_old_view` hand it to `take_workspace`, which, only where `code_workspaces` is true, removes a workspace alone in its folder, with or without its partial file, that is none of the wanted ones, then the folder, saying the line. It compares paths as `view::same_path` does.
- **The docs.**
  - `docs/architecture/core.md` holds core.43 and its details.
  - `docs/adr/0017` holds the decision, proposed until the user says yes to it.

The branch touches only what the proposal's Touches names, beside the change itself.

Design Decision 4 says unifying `preserve_order` changes no text the other crates write. That holds at 9c10725:

- `report.json`, the one JSON `boomdrive` writes, holds `boomdrive_control`'s types, which have no `serde_json::Value` or `Map`.
- `boomdrive-crew` and `boomdrive-control` write no `Value` as text outside their tests.
- `boomdrive-hosted` is not a dependency of `boomdrive`, so a build of `boomdrive` does not take the feature for it.

## Checks

All were run through sluice, so its sandbox and `network = "localhost"` applied.

- **The gate.** `sluice exec main` passed all 19 `check-fast` tasks and recorded its result under 9c10725.
- **Task 4.1's command, forced.** `sluice exec main -- moon run --force boomdrive:format boomdrive:lint boomdrive:typecheck boomdrive:test root:format root:adrs root:specs root:changes` passed all 8 tasks, none replayed from moon's cache.
  - `boomdrive:test` passed every test, with none failed or ignored. Among them are `code_workspace`'s 18 unit tests, `config`'s three for the key, and `tests/code_workspaces.rs`'s 11.
  - `root:adrs` read 17 records, each with a number of its own.
  - `root:changes` validated this change under `--strict`, and `root:specs` passed main's 11 specs.
  - `root:format` formatted 141 files and changed none.
- **Task 4.2.** On a copy of `openspec/`, the pinned openspec 1.14.3 archived the change, adding 4 requirements, modifying 1 and renaming 1. `openspec validate --specs --strict` then passed all 11 specs. The archive warned that the proposal's Why exceeds 1000 characters; that is a warning, and fails no check.

## Requirements and scenarios

Each scenario's test, and what it asserts beyond the scenario's words where that matters.

### Configuration: Whether each owner's views have a VS Code workspace is a setting

| scenario | test |
|---|---|
| No workspaces setting | `config::tests::no_workspaces_setting`, and `code_workspaces = false` read as false |
| Workspaces asked for in words | `config::tests::workspaces_asked_for_in_words`, the refusal's exact words at line 2, and an integer refused alike |
| Workspaces asked for while running | `tests/loop.rs` `workspaces_asked_for_while_running`: no workspace before the key, the written line after it, `folders` `[{"path": "flume"}]` |

`config::tests::a_whole_file` reads `code_workspaces = true`, and `tests/readme.rs` asserts the README's block reads it as true.

### Views: Each owner's views have a VS Code workspace where the configuration asks

| scenario | test (`tests/code_workspaces.rs`) |
|---|---|
| The first workspaces | `the_first_workspaces`: both files as JSON values, `folders` before `settings`, and the two written lines last, in the configuration's order of owners |
| No workspace asked for | `no_workspace_asked_for`: no `.code-workspace` under the root, and no workspace line |
| A kept repository with no view | `a_kept_repository_with_no_view`: `empty`'s view absent, `folders` `flume` then `empty` |

### Views: A workspace's folders follow the configuration and the rest of it is kept

| scenario | test (`tests/code_workspaces.rs`) |
|---|---|
| The user's settings kept | `the_users_settings_kept`: `settings` and `extensions` as they were, members in order |
| Folders edited by hand | `folders_edited_by_hand`: one line, the written one, and the folders set back |
| A dropped repository leaves its workspace | `a_dropped_repository_leaves_its_workspace`: `folders` only `flume`, `sluice`'s change and HEAD as they were |
| A quiet pass leaves the workspaces | `a_quiet_pass_leaves_the_workspaces`: no line, each file's bytes and inode as they were |

The requirement's "A pass that is stopped SHALL write no workspace" is `a_stopped_pass_writes_no_workspace`.

### Views: A workspace the runner cannot read is left as it is

| scenario | test (`tests/code_workspaces.rs`) |
|---|---|
| A comment in the workspace | `a_comment_in_the_workspace`: the file byte for byte, one line not quoting `mine`, the pass kept, and the same line at the next pass |
| A link at the workspace's path | `a_link_at_the_workspaces_path`: the link, where it points and its target as they were, `it is a link` |
| A workspace that cannot be written | `a_workspace_that_cannot_be_written`: the owner's folder at 0555, the pass failed, one `not written` line, the file as it was, and no partial file left |

### Views: A removed view takes its owner's folder only when nothing but a workspace no owner keeps is left there

| scenario | test |
|---|---|
| The owner's last view | `tests/dropped.rs` `the_owners_last_view`, as it stood |
| The owner's folder holds more | `tests/dropped.rs` `the_owners_folder_holds_more`, as it stood |
| The owner's last view, with its workspace | `tests/dropped.rs` `the_owners_last_view_with_its_workspace`, with and without a partial file: the removed line, the folder gone, and a quiet pass after |
| An owner still kept keeps its workspace | `tests/dropped.rs` `an_owner_still_kept_keeps_its_workspace`: the folder holds only the workspace, `folders` only `empty`, other members in order |
| A workspace not asked for | `tests/dropped.rs` `a_workspace_not_asked_for`: the view gone, the workspace byte for byte |
| A new root, the old owner's workspace | `tests/views.rs` `a_new_root_the_old_owners_workspace`: the old folder gone, the new workspace listing `flume`, the lines in order, and a quiet pass after |

## Rules

The diff keeps the rules the design relies on:

- **core.43**, which it adds: a workspace per owner beside its views, `folders` from the configuration, the rest kept, nothing written or removed with the option off.
- **core.5**: `wanted` reads only `config.repositories`, and gathers by owner.
- **core.11**: a line names a path and what is wrong, and `serde_json`'s error gives a line and column, never the text; the comment test asserts it.
- **core.12**: the key and module say `code_workspace`, not herdr's `workspace`.
- **core.14 and core.35**: the files are under the repository root, in no view, container or the data directory.
- **core.2 and core.42**: no control plane's implementation or protocol file changes.
- **core.7 and core.8**: no crate, tool or tracked file type is added.
- **core.41**: tests use the scratch home and neutral names.

## Left for later

- Record 0017 is `proposed`. It is accepted when the user says yes to it, as the record says.
- The hosts' configuration keeps its own workspaces while crew is the control plane (design, What this change asks of others). That is the hosts' change, told to the orchestrator by the design.
