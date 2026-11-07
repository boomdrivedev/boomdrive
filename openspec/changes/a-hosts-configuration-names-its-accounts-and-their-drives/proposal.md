# Proposal

## Why

The runner's file names the repositories a host keeps in one flat list, `repositories`, and keeps one VS Code workspace per owner of them. Record 0028, accepted by the user on 2026-11-07, gives a host's work two words. An **account** is what a runner enrolls with a control plane. A **drive** is one account's body of work: its repositories, one Flume stream and one branch of the account's crew state. "The runner's file names its accounts and their drives", grouping the repositories it keeps by drive, so the runner knows each repository's drive, as each drive's workspace needs. It still names no stream and no kind (core.5, as accepted with the record).

Record 0017, revised on the user's note on its card the same day ("Per recent discussion with designer agent, this becomes one workspace per boomdrive drive"), moves the workspaces from owners to drives. Where the configuration asks, there is one workspace per drive at the repository root, `<repository_root>/<drive>.code-workspace`. It lists exactly that drive's kept repositories and goes with its drive (core.43, as accepted with the record).

An owner is not a drive. One owner's repositories can belong to two drives, so a workspace per owner gathers into one window work the user does not open together (record 0028's context).

## What Changes

- **The file names accounts, drives and repositories.** `[accounts.<account>.drives.<drive>]` holds `repositories`, a list of GitHub addresses `owner/name`, in the order kept. `accounts`, each account's `drives` and each drive's `repositories` are required and may be empty, so a host that keeps none says `accounts = {}`. An account and a drive hold no other key: no stream, no kind, no state repository, no machine (core.5).
- **One repository, one drive; one drive's name, once.**
  - A repository named twice on the host, in one drive or in two, is refused, naming both.
  - A drive's name is given once on the host, across its accounts.
  - An account's or a drive's name is 1 to 32 lower-case letters, digits and `-`, starting with a letter.
- **The flat list of before is refused** with what to write instead, as `views` and `fetch_every` are: `accounts` holds it now. A `stream` or a kind named in an account or a drive is refused, saying whose configuration it is.
- **Views are kept as before**, at `<repository_root>/<owner>/<name>`, from every drive's repositories.
- **One workspace per drive** where `code_workspaces = true`:
  - it is `<repository_root>/<drive>.code-workspace`;
  - its `folders` are the drive's kept repositories, each `{"path": "<owner>/<name>"}`, in the configuration's order, whether or not a view stands;
  - it is set at every pass, and the rest of the file is kept as it is today.
- **A drive's workspace goes with its drive.**
  - The runner records each kept repository's drive in its clone.
  - Once a drive has left the configuration and no clone the runner holds records it, the runner removes the drive's workspace.
  - It removes no other workspace.
- **No workspace in an owner's folder.** A removed view takes its owner's folder only when that folder is then empty. A workspace an earlier runner kept per owner is left as it is, with its folder.
- **The modules' `settings` name `accounts`** in place of `repositories`, rendered always. A host's `settings` that still names `repositories` fails evaluation, saying `accounts` holds it now.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `configuration`: `accounts` in place of `repositories`, with its drives, names and refusals; `code_workspaces` asks for a workspace per drive.
- `views`: one workspace per drive at the repository root, set at every pass and removed with its drive. A removed view's owner folder is taken only when empty.
- `service-modules`: `settings` holds `accounts`, rendered always, and the flat list of before fails evaluation.

## Impact

- `crates/boomdrive`:
  - `src/config.rs` reads the accounts and their drives, and gives every drive's repositories in order;
  - `src/code_workspace.rs` gives each drive's workspace;
  - `src/clone.rs` and `src/pass.rs` record each kept repository's drive in its clone, keep each drive's workspace, and remove a departed drive's;
  - `src/pass.rs` and `src/view.rs` lose the owner's workspace taken with its folder;
  - `src/carry.rs` and `src/run.rs` read the kept repositories through the configuration's accessor;
  - tests under `crates/boomdrive/tests/`, their configuration texts among them.
- `nix/`: the options declare `settings.accounts` in place of `settings.repositories`, with an assertion for the flat list. The module's test settings and their files under `nix/tests/files` name accounts.
- `README.md`: the configuration's example and words, the hosted section's example and the nix example.
- `docs/adr/0028-…` and `docs/adr/0017-…` gain an amendment each, and core.43's details follow the second, in this change's commit: the runner's file's keys, and how a drive's workspace goes with its drive.
- Nothing else changes:
  - no control plane's table, implementation, interface or protocol message (core.2, core.42);
  - no crate, tool or tracked file type (core.7, core.8);
  - the `[control_plane.crew]` and `[control_plane.hosted]` tables keep their `partitions` as they are.
- Not in this change: `services.boomdrive.accounts`, record 0028's option that writes `settings.accounts` and gives each drive's stream to Flume's module. It is the flake bolt's unit `each-repository-is-named-once-by-its-owner` (record 0028's consequences).
- A host upgrading to this runner rewrites its file: the runner refuses a flat `repositories`, naming `accounts`, and keeps and removes nothing until the file is rewritten. Clones, views and the data directory carry over unchanged.

## Touches

```
docs/architecture/core.md
docs/adr/0017-each-owners-views-have-a-vs-code-workspace-beside-them-where-the-configuration-asks.md
docs/adr/0028-a-hosts-configuration-names-its-accounts-each-account-its-drives-and-each-drive-its-repositories.md
crates/boomdrive/src/config.rs
crates/boomdrive/src/code_workspace.rs
crates/boomdrive/src/clone.rs
crates/boomdrive/src/pass.rs
crates/boomdrive/src/view.rs
crates/boomdrive/src/carry.rs
crates/boomdrive/src/run.rs
crates/boomdrive/tests/common/mod.rs
crates/boomdrive/tests/code_workspaces.rs
crates/boomdrive/tests/drive_workspaces.rs
crates/boomdrive/tests/dropped.rs
crates/boomdrive/tests/views.rs
crates/boomdrive/tests/loop.rs
crates/boomdrive/tests/running.rs
crates/boomdrive/tests/planes.rs
crates/boomdrive/tests/control.rs
crates/boomdrive/tests/enroll.rs
crates/boomdrive/tests/readme.rs
crates/boomdrive/tests/module_files.rs
nix/modules/options.nix
nix/tests/settings.nix
nix/tests/file.nix
nix/tests/files/empty.toml
nix/tests/files/crew.toml
nix/tests/files/hosted.toml
README.md
```
