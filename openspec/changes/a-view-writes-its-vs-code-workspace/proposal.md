# Proposal

## Why

The user browses code in VS Code through one workspace per context, which lists the context's repositories. The hosts' own configuration writes those workspaces today, under afterthought/swancloud's rule herdr.14 and its record 0031. Beside each space's views is a workspace listing exactly the space's repositories. Its `folders` are set at every deploy, and the rest of the file, the user's VS Code settings, is kept. VS Code titles the window `— <name> (Workspace)`, and a window manager that files windows by their title puts each context in a row of its own.

The runner keeps each repository's view under its repository root, from the repositories its configuration names (the views spec). Once it keeps a context's repositories, a workspace kept anywhere else is a second list that can drift from the views. And under a hosted control plane, the hosts' configuration no longer knows the list at all, since the managed level holds it (record 0009).

The user, 2026-11-04: "I don't know if we should build it as a feature into Boom Drive, where we automatically generate VS Code workspaces that point to your view. Maybe it's an option that goes in." On 2026-11-05 they asked for it to be rushed.

## What Changes

- **Rule core.43 and record 0017**, in this change's commit beside its design. Where its configuration asks, the runner keeps one VS Code workspace for each owner of its kept repositories, beside the owner's views. Its `folders` are exactly that owner's kept repositories, and the rest of the file is kept.
- **A setting.** `code_workspaces = true` asks for the workspaces. Absent, it is false. It is read again at every pass, as `repositories` is.
- **One workspace per owner**, at `<repository_root>/<owner>/<owner>.code-workspace`:
  - its `folders` are the owner's kept repositories, in the configuration's order, each `{"path": "<name>"}`;
  - a repository is listed whether or not its view stands this pass;
  - a file the runner makes holds `folders` and an empty `settings`.
- **The rest of the file is kept.**
  - At every pass the runner sets `folders` and keeps every other member as it was, in its order.
  - It writes the file only when its folders differ, as a file beside it renamed over it, and says one line.
  - A file it cannot read as a JSON object, or a link or a folder at its path, is left and named at every pass, and does not fail it.
- **It goes with its owner.** When the runner removes a view and the owner's folder then holds only a workspace that no kept owner has there, the workspace and the folder go too. Where the option is off, the runner writes and removes no workspace.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `views`: each owner's VS Code workspace beside its views, its folders following the configuration and the rest of it kept, and a file the runner cannot read left. A removed view's owner folder is also taken when only a workspace no kept owner has is left in it.
- `configuration`: the `code_workspaces` setting.

## Impact

- `docs/architecture/core.md` gains core.43, and `docs/adr/` gains record 0017, in this change's commit.
- `crates/boomdrive`:
  - `src/config.rs` reads the key;
  - a new module, `src/code_workspace.rs`, decides and writes each owner's workspace;
  - `src/pass.rs` keeps the workspaces at the end of a pass, and removes one with its owner's last view;
  - `serde_json` takes its `preserve_order` feature, whose `indexmap` is already in `Cargo.lock`;
  - tests under `crates/boomdrive/tests/` and in the modules.
- `README.md`: the configuration's example and words name the key.
- Nothing else changes: no crate, no tool, no tracked file type, no message of the protocol, no control plane's implementation.
- Outside this repository, the hosts' configuration keeps writing its spaces' workspaces while crew is the control plane, and leaves the option off. Once the runner keeps a context's views, the host asks for the option and the hosts' module retires.

## Touches

```
docs/architecture/core.md
docs/adr/0017-each-owners-views-have-a-vs-code-workspace-beside-them-where-the-configuration-asks.md
Cargo.lock
crates/boomdrive/Cargo.toml
crates/boomdrive/src/lib.rs
crates/boomdrive/src/config.rs
crates/boomdrive/src/carry.rs
crates/boomdrive/src/code_workspace.rs
crates/boomdrive/src/pass.rs
crates/boomdrive/src/view.rs
crates/boomdrive/tests/code_workspaces.rs
crates/boomdrive/tests/dropped.rs
crates/boomdrive/tests/views.rs
crates/boomdrive/tests/loop.rs
crates/boomdrive/tests/readme.rs
crates/boomdrive/tests/common/**
README.md
```
