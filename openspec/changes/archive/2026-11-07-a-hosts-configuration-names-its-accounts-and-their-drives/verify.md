# Verify: a-hosts-configuration-names-its-accounts-and-their-drives

The work on `change/a-hosts-configuration-names-its-accounts-and-their-drives`, read at 9f2fe61, meets its change, so the change stays archived with this report.

- The gate run through sluice passes every `check-fast` task it ran, 21 of 21, with 673 tests passed and none failed.
- Every task is ticked, 19 of 19.
- The change adds, modifies or renames 22 requirements with 78 scenarios across three specs, and removes two. A case of `boomdrive:test` or `nix:test` asserts the outcome of every scenario.
- Task 7.1 archives the change, so the code stage archived it at 9f2fe61, before this stage, over #17's specs. The specs it merged pass `root:specs`, and `root:changes` passes. This report is committed into the archived change's directory.
- The branch holds the bolt `the-boomdrive-flake-configures-the-whole-host` at its head, 4a4b7c8, which landed #17, `the-runner-reads-a-local-layer-over-its-managed-one`: the branch merged it at b1a7d15, and origin's bolt is still 4a4b7c8.

## Checks

- **The gate.** `devenv shell -- sluice exec main` at 9f2fe61 exited 0. sluice recorded its 21 results under their keys for 9f2fe61:
  - `boomdrive:format`, `boomdrive:lint`, `boomdrive:typecheck` and `boomdrive:test`;
  - `boomdrive-crew:format`, `boomdrive-crew:lint`, `boomdrive-crew:typecheck` and `boomdrive-crew:test`;
  - `boomdrive-hosted:format`, `boomdrive-hosted:lint`, `boomdrive-hosted:typecheck` and `boomdrive-hosted:test`;
  - `nix:format`, `nix:lint`, `nix:typecheck` and `nix:test`;
  - `root:format`, `root:adrs`, `root:specs`, `root:changes` and `root:releases`.
- **The tests.** 673 tests passed, with none failed or ignored. Among them are `config::tests`, `code_workspace::tests`, `tests/local_layer.rs`, `tests/code_workspaces.rs`, `tests/drive_workspaces.rs`, `tests/dropped.rs`, `tests/views.rs`, `tests/loop.rs`, `tests/running.rs`, `tests/enroll.rs`, `tests/module_files.rs` and `tests/readme.rs`.
- **The warnings.** The run's only warnings were moon's, which could not write the shared cache's blobs, and the OTLP exporter's at shutdown. The sandbox does not let moon write there. They change no task's outcome.
- The checks ran once, as the brief asks.

## Tasks

All 19 tasks of `tasks.md`, groups 1 to 7, are ticked `[x]`, and none is open. What each names is in the tree:

- **The configuration** (1.1 to 1.3). `src/config.rs` reads `accounts` into `Config.accounts`, of `Account` and `Drive`, in the file's order by the keys' spans.
  - `Config::repositories()`, `Config::drives()` and `Config::drive_of()` give what the design names. Every reader in `src/` uses them, and the `repositories` field is gone.
  - An account holds only `drives` and a drive only `repositories`, each other key refused by its path. Flume's keys there are refused with the top level's words.
  - A top-level `repositories` is among the keys held elsewhere.
  - Names are checked by `in_form`. A drive is named once across accounts, and a repository once on the host, each refusal naming both entries, their drives and the first one's line.
- **The local layer's accounts** (1.4, 1.5).
  - Record 0028 gains "Amendment, 2026-11-07: the local layer's accounts", the writing agent's, with each path not taken. core.17's details gain what it rules out and allows.
  - `config::local_keys` gives `accounts.<account>.drives.<drive>.repositories` as a pattern of `Segment`s, a name standing for any account or drive, beside each implementation's keys. `is_local_key` and `leads_to_local_key` match it segment by segment, so a pin may name `accounts`, an account, an account's `drives`, a drive or a drive's `repositories`. A pin of the flat `repositories` is refused as a pin of nothing.
  - `config::over` collects each drive's unit not passed over by a pin and adds them through `add_drives`. A drive both layers name keeps the managed layer's repositories, then the local layer's it lacks. A drive or an account only the local layer names is appended after the managed layer's.
  - `add_drives` refuses, against the local file and its line: a name out of form; a drive the local layer names under two accounts, or under an account the managed layer does not hold it under; an entry that is no address; a repository the local layer names twice, or names in a drive other than the managed layer's.
  - `LocalFile` and the flat list's `addresses` are gone.
  - `pass::wanted_workspaces` gives no workspaces while the local layer is not taken, so `keep_workspaces` and `take_departed` do not run.
- **The modules** (2.1, 2.2).
  - `nix/modules/options.nix` declares `settings.accounts` with submodules for accounts and drives, which have no freeform type, and defaults `{ }`, `{ }` and `[ ]`. Its assertion on a top-level `settings.repositories` names `settings.accounts.<account>.drives.<drive>.repositories`.
  - `nix/tests/settings.nix` and the three files under `nix/tests/files` name the design's neutral accounts and drives.
  - `tests/module_files.rs` reads each file through `Config::accounts` and `Config::repositories()`.
- **README** (3.1, 3.2).
  - Its configuration example keeps `agentplot/flume` in the drive `home` and `example/crew` in the drive `work`, both of the account `own`. Its words cover each point task 3.1 lists. Its nix example sets `settings.accounts.own.drives.home.repositories`, and no owner's-workspace words remain.
  - Its local layer section says the local layer sets each drive's `repositories`, how both layers' drives combine, and what is not taken. Its example adds `agentplot/sluice` to `home` and declares `garden`. Its pin words name `accounts`, a drive and a drive's `repositories`.
- **The tests' configurations** (4.1, 4.2).
  - `tests/common/mod.rs`'s `keeping` writes under `own`'s drive `home`, and `drives` beside it writes several drives.
  - No integration test's configuration text names a flat `repositories`, but the refusals that assert it.
  - `tests/local_layer.rs` pins `accounts`. Its "A local file broken while running" keeps `agentplot/sluice` in the local layer's drive `work`, and asserts `home.code-workspace` and `work.code-workspace` unchanged while the local layer is not taken.
- **One workspace per drive** (5.1 to 5.5).
  - `src/code_workspace.rs` gives `Wanted` per drive, at `<repository_root>/<drive>.code-workspace`.
  - `code_workspace::alone_in` and the workspaces argument of `let_go` are gone. A removed view takes its owner's folder only by `fs::remove_dir`, which removes only an empty folder.
  - The tests the tasks name as replaced are gone: `the_owners_last_view_with_its_workspace`, `an_owner_still_kept_keeps_its_workspace`, `a_workspace_not_asked_for` and `a_new_root_the_old_owners_workspace`.
  - The owner's-workspace function `pass::take_workspace` that task 5.3 removes is gone. Its name now belongs to the drive's removal of Decision 8, called only from `take_departed`, which touches only `<repository_root>/<drive>.code-workspace`.
- **A drive's workspace goes with its drive** (6.1, 6.2).
  - `clone::find` reads `boomdrive.address` and `boomdrive.drive` in one `git config --local -z --get-regexp ^boomdrive\.`.
  - `clone::record_drive` writes `boomdrive.drive`. `pass::keep` calls it where the clone records another drive or none, at a fresh clone and through `keep_one`.
  - `pass::take_departed` runs at the end of a pass not stopped, where `code_workspaces` is true and the local layer is not held back. It takes the drives the clones recorded when found, leaves out those the configuration names and those a clone still standing records, and leaves out any record not of a name's form.
- **The specs** (7.1). The change is under `openspec/changes/archive/2026-11-07-…`. `openspec/specs/configuration`, `views` and `service-modules` hold its deltas over #17's, and `root:specs` passes.

Nothing outside the proposal's Touches changed. `git diff --name-only 4a4b7c8..HEAD` lists only those paths and the change's own files. `crates/boomdrive/tests/enroll.rs`, listed in Touches, did not need to change on this branch.

The records and rules are amended as the proposal says:

- record 0017 gains its amendment of 2026-11-07, on how a drive's workspace goes with its drive;
- record 0028 gains its amendment of 2026-11-07 on the keys of the runner's file, and a second, the local layer's accounts;
- core.43's details say a drive's workspace is removed once no clone the runner holds records the drive;
- core.17's details rule out a workspace removed while the local layer stands not taken, and a local layer taken that names a repository or a drive the managed layer holds elsewhere, and allow each drive's `repositories` as the local layer's key.

## Requirements and scenarios

`config::tests` and `code_workspace::tests` are the crate's unit tests. Every other case is in `crates/boomdrive/tests/<file>`, or in `nix/tests/file.nix` by its case's name.

### Configuration

- **The configuration names its accounts, each account's drives and each drive's repositories.**
  - "One drive": `config::tests::one_drive`.
  - "Drives of two accounts": `config::tests::drives_of_two_accounts`.
  - "A drive with no list": `config::tests::a_drive_with_no_list`.
  - "A drive keeping none": `config::tests::a_drive_keeping_none`.
- **An account's and a drive's name is of one form, and a drive's is given once on the host.**
  - "One drive in two accounts": `config::tests::one_drive_in_two_accounts`.
  - "A name out of form": `config::tests::a_name_out_of_form`.
- **A local layer naming a repository or a drive the managed layer holds elsewhere is not taken.**
  - "A repository the managed layer keeps in another drive": `config::tests::a_repository_the_managed_layer_keeps_in_another_drive`.
  - "A drive the managed layer holds under another account": `config::tests::a_drive_the_managed_layer_holds_under_another_account`.
- **The configuration names each repository by its forge address.**
  - "An address with no owner": `config::tests::an_address_with_no_owner`.
  - "One repository twice": `config::tests::one_repository_twice`.
  - "One repository in two drives": `config::tests::one_repository_in_two_drives`, for drives of one account and of two.
  - "No list": `config::tests::no_accounts`.
  - "Keeping none": `config::tests::keeping_none`.
- **The configuration holds nothing the runner does not read.**
  - "A misspelt key": `config::tests::a_misspelt_key`.
  - "A token": `config::tests::a_token`.
  - "A key in a drive": `config::tests::a_key_in_a_drive`.
  - "A key in an account": `config::tests::a_key_in_an_account`.
- **A refused stream, kind or key of before says where it is held.**
  - "A stream named": `config::tests::a_stream_named`.
  - "A stream in a drive": `config::tests::a_stream_in_a_drive`, with each key of Flume's in an account and in a drive.
  - "A kinds file named": `config::tests::a_kinds_file_named`.
  - "A key of before": `config::tests::a_key_of_before`, with a flat `repositories` named beside `views` and `fetch_every`.
  - "A flat list of before": `config::tests::a_flat_list_of_before`.
- **Whether each drive has a VS Code workspace is a setting.**
  - "No workspaces setting": `config::tests::no_workspaces_setting`.
  - "Workspaces asked for in words": `config::tests::workspaces_asked_for_in_words`.
  - "Workspaces asked for while running": `loop.rs` `workspaces_asked_for_while_running`, at `/h/Code/home.code-workspace` with the folder `agentplot/flume`.
- **The configuration is a managed file and a local one over it.**
  - "The default file": `running.rs` `the_default_file`.
  - "The file named": `running.rs` `the_file_named`.
  - "No file": `running.rs` `no_file`.
  - "No local file": `config::tests::no_local_file` and `running.rs` `no_local_file`.
- **The local layer sets the repositories and a control plane's partitions alone.**
  - "The local layer's repositories": `config::tests::the_local_layers_repositories`, both in the drive `home`.
  - "A partition enrolled from the local layer": `enroll.rs` `a_partition_of_the_local_layer`.
- **The local layer's repositories add to the managed layer's, and its partitions replace them.**
  - "A repository both layers name": `config::tests::a_repository_both_layers_name`.
  - "A drive only the local layer names": `config::tests::a_drive_only_the_local_layer_names`, with an empty drive of the account `client` as well.
  - "A repository the local layer drops": `local_layer.rs` `a_repository_the_local_layer_drops`.
  - "The local layer's partitions": `config::tests::the_local_layers_partitions`.
- **A local layer that sets any other key is not taken.**
  - "A root in the local layer": `config::tests::a_root_in_the_local_layer`.
  - "A key in a local drive": `config::tests::a_key_in_a_local_drive`.
  - "A flat list in the local layer": `config::tests::a_flat_list_in_the_local_layer`.
  - "A control plane named in the local layer": `config::tests::a_control_plane_named_in_the_local_layer`.
  - "A token in the local layer": `config::tests::a_token_in_the_local_layer`.
- **A local layer the runner cannot read is not taken.**
  - "A local file that is not TOML": `config::tests::a_local_file_that_is_not_toml`.
  - "A local address out of form": `config::tests::a_local_address_out_of_form`. A repository named twice, a drive under two accounts and a name out of form are `config::tests::a_local_layer_naming_one_repository_twice` and `a_local_drive_named_twice_or_out_of_form`.
  - "A partition the implementation refuses": `config::tests::a_partition_the_implementation_refuses`.
- **While the local layer is not taken, nothing is let go.**
  - "A local file broken while running": `local_layer.rs` `a_local_file_broken_while_running`.
  - "A local file removed": `local_layer.rs` `a_local_file_removed`.
- **The managed layer may pin a key the local layer could set.**
  - "Repositories pinned": `config::tests::repositories_pinned`, and a drive pinned alone in `a_drive_pinned`.
  - "A table pinned": `config::tests::a_table_pinned`.
- **How the local layer was read is said once each way.**
  - "Taken at the start": `local_layer.rs` `taken_at_the_start`.
  - "A fault said once": `local_layer.rs` `a_fault_said_once`. A pin passed over is said once in `pinned_accounts_passed_over_said_once`.

### Views

- **Each drive has a VS Code workspace at the repository root where the configuration asks.** Each scenario's case is in `code_workspaces.rs`.
  - "The first workspaces": `the_first_workspaces`.
  - "No workspace asked for": `no_workspace_asked_for`.
  - "A kept repository with no view": `a_kept_repository_with_no_view`.
  - "A drive keeping none": `a_drive_keeping_none`, and `code_workspace::tests::a_drive_keeping_none`.
  - "A new root": `a_new_root`.
- **A workspace's folders follow the configuration and the rest of it is kept.** Each scenario's case is in `code_workspaces.rs`.
  - "The user's settings kept": `the_users_settings_kept`.
  - "Folders edited by hand": `folders_edited_by_hand`.
  - "A dropped repository leaves its workspace": `a_dropped_repository_leaves_its_workspace`.
  - "A repository moved to another drive": `a_repository_moved_to_another_drive`.
  - "A quiet pass leaves the workspaces": `a_quiet_pass_leaves_the_workspaces`.
  - The requirement's stopped pass: `a_stopped_pass_writes_no_workspace`.
- **A workspace the runner cannot read is left as it is.** Each scenario's case is in `code_workspaces.rs`.
  - "A comment in the workspace": `a_comment_in_the_workspace`.
  - "A link at the workspace's path": `a_link_at_the_workspaces_path`.
  - "A workspace that cannot be written": `a_workspace_that_cannot_be_written`.
- **A removed view takes its owner's folder only when that is empty.**
  - "The owner's last view": `dropped.rs` `the_owners_last_view`.
  - "The owner's folder holds more": `dropped.rs` `the_owners_folder_holds_more`.
  - "An owner's workspace from before": `dropped.rs` `an_owners_workspace_from_before`.
  - "A new root, an owner's workspace from before": `views.rs` `a_new_root_an_owners_workspace_from_before`.
- **A drive's workspace goes with its drive.** Each scenario's case is in `drive_workspaces.rs`.
  - The clone's record: `the_clone_records_its_drive`.
  - "The drive's last view": `the_drives_last_view`.
  - "A view still holding work": `a_view_still_holding_work`.
  - "Its repositories moved": `its_repositories_moved`.
  - "A workspace of the person's own": `a_workspace_of_the_persons_own`.
  - "A departed drive's workspace the runner cannot read": `a_departed_drives_workspace_the_runner_cannot_read`.
  - "A departed drive's workspace not asked for": `a_departed_drives_workspace_not_asked_for`.

### Service modules

- **The options are the runner's file, its accounts always written.**
  - "Only accounts": `nix/tests/file.nix` "Only accounts", and `module_files.rs` `the_empty_file_keeps_nothing_with_the_runners_defaults`.
  - "A host under crew": `nix/tests/file.nix` "A host under crew", against `nix/tests/files/crew.toml` on both platforms.
  - "A host under a hosted control plane": `nix/tests/file.nix` "A host under a hosted control plane", against `nix/tests/files/hosted.toml`.
  - "The runner reads each file": `module_files.rs` `each_rendered_file_is_one_read_here`, `the_crew_file_opens_crews_implementation` and `the_hosted_file_names_its_partitions`.
- **A flat list of before fails evaluation.**
  - "The flat list in settings": `nix/tests/file.nix` "The flat list in settings".
  - "A stream in a drive's settings": `nix/tests/file.nix` "A stream in a drive's settings".

## Observations

None of these holds the change back.

- **Where the design says a workspace can outlive its drive.** The design names three cases, under Risks:
  - a runner stopped between a let-go and the pass's end;
  - a drive whose repositories' clones were never made;
  - a drive that left while `code_workspaces` was off.

  The code matches each. `take_departed` reads only the drives the clones recorded when the pass found them, so in each case the workspace is left for the person.
- **A record the pass cannot write.** Where `boomdrive.drive` cannot be written to a clone found, the clone's earlier drive stays among those standing. So a failed record leaves a workspace and never removes one.
- **A pinned local drive is not read.** `add_drives` reads only the drives whose units no pin holds. A local drive a pin passes over is named as passed over, and its entries are not checked against the managed layer's, so they neither fault the local layer nor are kept. Decision 13 and the spec ask nothing more of it.
- **Record 0028's second amendment is the writing agent's.** It records choices made within records 0009 and 0028 as accepted, each with the path not taken, and says each is cheap to change, as the capture process has such choices recorded.
