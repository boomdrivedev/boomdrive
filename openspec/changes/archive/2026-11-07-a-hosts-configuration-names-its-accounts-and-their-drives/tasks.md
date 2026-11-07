# Tasks

Work through the tasks in order. Each stands on the ones before it. The groups are headings for reading, and none is checked as a whole: the verify stage runs the repository's whole checks once, after every task is done. A task is done once the check it names passes, run exactly as it names it. A test binary a task does not name may fail between tasks while the behaviour it asserts is being moved. The task that moves it names it.

- A quick check runs as written, `devenv shell -- cargo test -p boomdrive …`; a project's task runs through sluice, `sluice exec main -- moon run <project>:<task>` (CLAUDE.md).
- Every test runs on the scratch host of `crates/boomdrive/tests/common`, or on its crate's unit fixtures, with neutral names (core.41): the accounts `own`, `client`, `home` and `work`, the drives `home`, `work` and `garden`. No test reads the user's configuration or reaches the network.
- Every key, type and function a task adds names what it holds (core.12), as the design's decisions name them.
- No task adds a crate, a tool or a tracked file type (core.7, core.8). Nothing outside the files proposal.md's Touches lists is changed.

## 1. The configuration names accounts and drives

- [x] 1.1 `src/config.rs` reads `accounts` into `Config.accounts`, each `Account` with its `drives` and each `Drive` with its `repositories`, in the file's order by the keys' spans. `Config::repositories()` gives every drive's repositories in order, and `Config::drive_of()` the drive naming an address. `accounts`, each account's `drives` and each drive's `repositories` are required and may be empty. The `repositories` field is gone. Every reader in `src/` (`pass.rs`, `carry.rs`, `run.rs`, `code_workspace.rs`) reads `Config::repositories()`, and `carry.rs`'s unit tests build their configuration through the new types (design Decisions 1, 2 and 6). Done when `devenv shell -- cargo test -p boomdrive --lib config::tests` passes, with:
  - a unit test for each of the configuration spec's "One drive", "Drives of two accounts", "No accounts", "A drive with no list", "Keeping none" and "A drive keeping none";
  - the module's existing tests naming their repositories under a drive.
- [x] 1.2 An account and a drive hold no key but `drives` and `repositories`. Any other is refused, naming its path. A `stream`, `streams`, `kind`, `kinds` or `kinds_file` there is refused with the words the top level gives it. A top-level `repositories` joins the keys held elsewhere, saying `accounts` holds it now, each repository named under its drive (design Decisions 2 and 5). Done when `devenv shell -- cargo test -p boomdrive --lib config::tests` passes, with:
  - a unit test for each of the configuration spec's "A key in a drive", "A key in an account", "A stream in a drive" and "A flat list of before";
  - "A key of before" asserting a flat `repositories` named with `views` and `fetch_every` when all three are there;
  - no refusal quoting a value.
- [x] 1.3 An account's and a drive's name is 1 to 32 lower-case letters, digits and `-`, starting with a letter, and a drive's name is given once across the accounts. A repository is named once on the host, in one drive or in two, each refusal naming both entries and their drives (design Decisions 3 and 4). Done when `devenv shell -- cargo test -p boomdrive --lib config::tests` passes, with a unit test for each of the configuration spec's "One drive in two accounts", "A name out of form", "An address with no owner", "One repository twice" and "One repository in two drives", each refusal naming the file's line.

## 2. The modules render accounts

- [x] 2.1 `nix/modules/options.nix` declares `settings.accounts` in place of `settings.repositories`, as design Decision 11 has it:
  - accounts, then drives, then a drive's `repositories`, typed as `repositories` is typed today;
  - defaults `{ }`, `{ }` and `[ ]`, so each level is always written;
  - no freeform type in an account or a drive;
  - an assertion failing where `settings` names `repositories` at its top.

  `nix/tests/settings.nix` names the design's neutral accounts and drives, and the three files under `nix/tests/files` are rendered from them. `nix/tests/file.nix` holds:
  - "Only accounts", the empty settings rendering `[accounts]` alone after the header;
  - "The flat list in settings", its assertion's message naming `settings.repositories` and `settings.accounts.<account>.drives.<drive>.repositories`;
  - "A stream in a drive's settings", whose render fails under `builtins.tryEval`.

  The option's descriptions and the file's header comment name accounts and drives. Done when `sluice exec main -- moon run nix:test` passes, proving the service-modules spec's "Only accounts", "A host under crew", "A host under a hosted control plane", "The flat list in settings" and "A stream in a drive's settings".
- [x] 2.2 `tests/module_files.rs` reads each rendered file with the accounts, drives and repositories its settings named, through `Config::accounts` and `Config::repositories()`. Done when `devenv shell -- cargo test -p boomdrive --test module_files` passes, proving the service-modules spec's "The runner reads each file".

## 3. README

- [x] 3.1 README's configuration example keeps `agentplot/flume` and `afterthought/crew` under one account's drives, and its words say:
  - what an account, a drive and a drive's `repositories` are;
  - that each level is required and may be empty;
  - that a repository belongs to one drive, and a drive's name is given once;
  - that a flat `repositories` is refused, naming `accounts`;
  - that `code_workspaces = true` asks for one workspace per drive at the root, titled `— <drive> (Workspace)`, which goes with its drive.

  Its nix example sets `settings.accounts`. Its owner's-workspace words are gone (design Decision 12). Done when `devenv shell -- cargo test -p boomdrive --test readme` passes, with `the_readme_configuration_reads` asserting the example's drives and their repositories.

## 4. The tests name their repositories under drives

- [x] 4.1 `tests/common/mod.rs`'s `keeping` writes its addresses under the account `own`'s drive `home`, and a helper beside it writes several drives, each with its addresses. Every configuration text of the crate's integration tests names its repositories under a drive: `planes.rs`, `running.rs`, `loop.rs`, `control.rs` and `enroll.rs`, and the helpers of `dropped.rs`, `views.rs` and `code_workspaces.rs`. Each test asserts what it asserted before, its owners' workspaces among them, since the workspaces move in group 5. Done when `sluice exec main -- moon run boomdrive:test` passes.

## 5. One workspace per drive

- [x] 5.1 `src/code_workspace.rs` gives each drive of the configuration its workspace, in the file's order:
  - `Wanted` names its drive;
  - its file is `<repository_root>/<drive>.code-workspace`;
  - its folders are the drive's repositories as `<owner>/<name>`, owners as the configuration writes them;
  - a drive keeping none has no folders.

  A workspace's written line lists its folders as they are written (design Decision 1; the views spec's "Each drive has a VS Code workspace at the repository root where the configuration asks"). Done when `devenv shell -- cargo test -p boomdrive --lib code_workspace::tests` passes, with unit tests for two drives sharing an owner, an owner written in two cases, and a drive keeping none.
- [x] 5.2 `tests/code_workspaces.rs` proves each drive's workspace at the root. Done when `devenv shell -- cargo test -p boomdrive --test code_workspaces` passes, proving the views spec's:
  - "The first workspaces", "No workspace asked for", "A kept repository with no view", "A drive keeping none" and "A new root";
  - "The user's settings kept", "Folders edited by hand", "A dropped repository leaves its workspace", "A repository moved to another drive" and "A quiet pass leaves the workspaces";
  - "A comment in the workspace", "A link at the workspace's path" and "A workspace that cannot be written";
  - a stopped pass writing no workspace.
- [x] 5.3 A removed view takes its owner's folder only when that folder is then empty. `pass::take_workspace`, `code_workspace::alone_in` and the workspaces argument of `let_go` and of a changed root's old view are removed, so the pass touches no workspace in an owner's folder (design Decision 10). Done when `devenv shell -- cargo test -p boomdrive --test dropped` passes:
  - proving the views spec's "The owner's last view", "The owner's folder holds more" and "An owner's workspace from before";
  - with `the_owners_last_view_with_its_workspace`, `an_owner_still_kept_keeps_its_workspace` and `a_workspace_not_asked_for` replaced by "An owner's workspace from before".
- [x] 5.4 A changed root leaves an owner's workspace of before in the old owner's folder. Done when `devenv shell -- cargo test -p boomdrive --test views` passes, proving the views spec's "A new root, an owner's workspace from before" in place of `a_new_root_the_old_owners_workspace`.
- [x] 5.5 A running runner asked for workspaces writes the drive's. Done when `devenv shell -- cargo test -p boomdrive --test loop workspaces_asked_for_while_running` passes, proving the configuration spec's "Workspaces asked for while running" at `/h/Code/home.code-workspace` with the folder `agentplot/flume`.

## 6. A drive's workspace goes with its drive

- [x] 6.1 Each kept repository's clone records its drive as `boomdrive.drive` in its bare repository's local configuration, written by the pass, and by `keep_one`, where the clone records another drive or none. `clone::find` gives each clone's recorded drive with its address, through one `git config --local --get-regexp` of the `boomdrive` section (design Decision 7). Done when `devenv shell -- cargo test -p boomdrive --test drive_workspaces the_clone_records_its_drive` passes. That new test asserts `boomdrive.drive` is `home` after a pass keeping `agentplot/flume` in `home`, and `work` after a pass naming it in `work`.
- [x] 6.2 At the end of a pass not stopped, where `code_workspaces` is true, the pass removes each departed drive's workspace at the root, as design Decision 8 has it, and fails where a removal fails. Done when `devenv shell -- cargo test -p boomdrive --test drive_workspaces` passes, proving the views spec's "The drive's last view", "A view still holding work", "Its repositories moved", "A workspace of the person's own", "A departed drive's workspace the runner cannot read" and "A departed drive's workspace not asked for".

## 7. The specs

- [x] 7.1 The change is archived into `openspec/specs`, and the specs it changes pass openspec's strict check. Done when, after `devenv shell -- openspec archive a-hosts-configuration-names-its-accounts-and-their-drives --yes`, `sluice exec main -- moon run root:specs` passes.
