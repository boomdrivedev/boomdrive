# Design

## Context

See proposal.md for why. This change adds one thing the runner keeps beside its views, and one key that asks for it.

What stands:

- **This repository**, at `c5bff41`.
  - A pass (`crates/boomdrive/src/pass.rs`) keeps each configured repository in the file's order: its clone, its HEAD, its own default branch and its view. It then lets go of each dropped one, and clears the unfinished clones.
  - A view is at `config::view_path`, `<repository_root>/<owner>/<name>`, the owner and name as the configuration writes them. `view::remove` removes a view and then its owner's folder where that leaves it empty. Both a dropped repository's let-go and a changed root's let-go of the old view call it.
  - The configuration (`src/config.rs`) refuses a key it does not name (`deny_unknown_fields`). It is read again at every pass, but for `[control_plane]`, which is taken at the start. `Address` compares owner and name without case, as GitHub does.
  - A pass says each line to `say`, `<owner>/<name>: <words>` for a repository and `boomdrive: <words>` for anything else. The running runner prefixes each with its time (core.40).
  - `serde_json` is a workspace dependency with `raw_value`. `indexmap` 2.14.2 is in `Cargo.lock`, through `toml`.
- **The model**, afterthought/swancloud's herdr.14 and its record 0031, read 2026-11-05 and cited by id (core.41):
  - each space's workspace is a real file beside the space's views, never a link, since VS Code writes the user's settings into it;
  - its `folders` are set at every deploy to exactly the space's repositories, one `{"path": "<repo>"}` each, and everything else in the file is kept;
  - a file its JSON reader cannot read, one with comments say, is left as it is and named, and the deploy goes on;
  - a new file is `{"folders": [...], "settings": {}}`, indented with tabs, as VS Code writes it.
- **VS Code** titles a workspace's window `<what is open> — <file name less .code-workspace> (Workspace)`. It reads a workspace as JSON with comments and trailing commas, but adds neither when it writes one, and the workspaces it makes are indented with tabs.

There are no area pages, so `docs/architecture/core.md` binds every path this change touches.

## Goals / Non-Goals

**Goals:**

- A host whose configuration asks gets one VS Code workspace for each owner of its kept repositories, beside that owner's views, listing exactly those repositories (core.43).
- The user's settings in a workspace survive every pass.
- A workspace the runner cannot read is never rewritten, and never fails a pass.
- A workspace goes with its owner's folder once nothing else holds that folder.

**Non-Goals:**

- **A workspace's name chosen by the person**, or a workspace gathering several owners. A later key can add a name per owner should a host need one (Decision 1).
- **A label for a folder.** Each folder shows the repository's name, as VS Code shows a folder with no `name`.
- **Keeping a person's comments.** A file with comments is left whole and named (Decision 5).
- **The hosts' change.** The hosts' configuration keeps its own spaces' workspaces while crew is the control plane, and retires them once the runner keeps a context's views (What this change asks of others).
- **The managed level and setup** (record 0009). They are unbuilt. Once built, the managed level may set `code_workspaces` as it sets `repositories` (core.17).

## Decisions

### 1. One workspace per owner, beside the owner's views

An owner's workspace is `<repository_root>/<owner>/<owner>.code-workspace`. The owner's folder holds the owner's views as a space's folder holds its views under herdr.14, and the file sits among them under the folder's name. VS Code titles its window `— <owner> (Workspace)`.

- **Owners compare without case**, as `Address` does, so `AgentPlot/flume` and `agentplot/sluice` share one workspace. Its folder and file take the owner as the first of its repositories in the file writes it, which is where `config::view_path` puts that repository's view.
- **Each folder's `path` is the view's path relative to the workspace's folder.** That is `<name>` for a repository whose owner is written the same way. It is `../<owner as written>/<name>` for one written otherwise, which a file system that ignores case finds in the same folder.

Alternatives:

- **Workspaces named in the configuration**, a table of names each listing addresses. Not taken: each repository is listed twice, in two lists that can differ. Under a hosted control plane the managed level would write each group, naming a drive under another key (core.5).
- **One workspace for the repository root.** Not taken: a host keeping several owners' repositories would get one window for all of them, which a window manager cannot file by context.
- **Each owner's workspace at the root**, `<repository_root>/<owner>.code-workspace` with paths `<owner>/<name>`. Not taken: the owner's folder is where its views are, and a file beside them keeps each folder's path to the view's own name.

### 2. `code_workspaces`, off where it is absent

`code_workspaces = true` asks for the workspaces, in the configuration's top level beside `repository_root`. Absent, it is false. Any other value is refused, naming the key and the value: `` `code_workspaces` is "yes": it is `true` or `false` ``. Like `repositories`, it is read at every pass, so a host turns the workspaces on or off without a restart.

- `File` gains `code_workspaces: Option<Spanned<Value>>`, and `Config` gains `code_workspaces: bool`.
- The user's words make it an option ("Maybe it's an option that goes in"). Off where absent, since a host with no editor, a server or a box, has no use for the files.
- **The name** (core.12): it names VS Code's `.code-workspace` file as VS Code means it. A bare `workspace` is herdr's word, which Flume uses for the workspaces its panes open in. So the key and the module say `code_workspace`.

Alternatives:

- **`vscode_workspaces`.** Not taken: `.code-workspace` is the file type, which VS Code and the editors built on it all open.
- **A `[views]` table holding the option.** Not taken: the views' other key, `repository_root`, is at the top level, and a table for one value adds a level for nothing.

### 3. What a workspace's folders list

A workspace's `folders` are its owner's kept repositories, in the configuration's order, each `{"path": <Decision 1's path>}` and nothing else. A repository is listed whether or not its view stands this pass: its clone failed, its origin has no branch yet, or a folder of the user's stands at its path.

- The workspace and the views follow one list, the configuration's (core.5). So a workspace never names a repository the host does not keep, and a pass that fails for one repository never takes it out of the window.
- A dropped repository leaves its owner's workspace at the next pass, even while its view is left holding work.

Alternative: **the views that stand.** Not taken: a failed fetch or clone would take a folder out of the window for a pass and put it back at the next, and VS Code reloads the workspace each time.

### 4. The file: what is kept, when it is written, and how

`src/code_workspace.rs` reads what stands at an owner's workspace path, decides, and writes:

- **Nothing there:** a new file, `{"folders": [...], "settings": {}}`.
- **A regular file holding a JSON object:** where the object's `folders` equal the wanted ones, as JSON values, nothing is done. Otherwise `folders` is set where it stands, or added where the file has none, and every other member is kept as it was, in its order.
- **Anything else** is left (Decision 5).

The JSON is read and written by `serde_json`'s `Value`, with its `preserve_order` feature, so an object's members keep their order. The `boomdrive` crate takes the feature on its own dependency, `serde_json = { workspace = true, features = ["preserve_order"] }`. Its `indexmap` is already in `Cargo.lock`. Cargo unifies the feature across a build of `boomdrive`, so `boomdrive-control` and `boomdrive-crew` take it there too. That changes nothing they print: read 2026-11-05, every `serde_json::Value` in those crates' sources and in `boomdrive`'s is only read, and none is written back as text. The tests' fakes print `json!` values, which now keep the order they are written in. No test compares such a print with a literal: the JSON literals compared as text are sluice's entries in `src/sluice.rs`, which are cut from the answer's own text and keep its order already.

A file is written as VS Code writes one:

- indented with tabs, by `serde_json::ser::PrettyFormatter::with_indent(b"\t")`, with a newline at the end;
- whole, to `.<owner>.code-workspace.partial` beside it, then renamed over it, so VS Code never reads half a file;
- with the old file's permissions set on the new one, where there was an old file.

A partial file left by a write cut short is written over at the next write.

Alternatives:

- **Write the whole file at every pass.** Not taken: it drops the settings VS Code writes into the file.
- **A JSON-with-comments editor that keeps a person's comments**, such as the `jsonc-parser` crate's tree. Not taken now: it is a dependency of its own for comments VS Code never adds, and a file with comments is left and named, as herdr.14's writer leaves it.

### 5. What is left, and what fails

What stands at the path is read with `fs::symlink_metadata`, so a link is seen as a link.

| what stands | done | the pass |
|---|---|---|
| nothing | made | goes on |
| a regular file, a JSON object | set, or nothing where its folders are the wanted ones | goes on |
| a regular file that is not JSON `serde_json` reads, comments and trailing commas among it | left, named with `serde_json`'s error, which gives a line and a column and quotes nothing | goes on |
| a regular file holding JSON that is not an object | left, named | goes on |
| a link | left, named | goes on |
| a folder, or anything else | left, named | goes on |
| a file that cannot be read, or a write or rename that fails | named with the error, the partial file removed | fails |

Something left is left by rule, as a folder of the user's at a view's path is, so it does not fail the pass (`pass::Outcome`). It is named at every pass, as that folder is.

### 6. Where in the pass, and its lines

At the end of `pass::pass`, after every configured repository is kept and every dropped one let go, and before the unfinished clones are cleared, each owner's workspace is kept where `code_workspaces` is true. A pass that is stopped keeps none. `pass::keep_one`, the keeping of one repository a landing told of, keeps none either: the next whole pass does.

Writing after the let-go means a removal in the same pass never takes a workspace that the end of the pass would make again (Decision 7).

The lines, each said to `say`:

- `boomdrive: workspace written at <file>: <path>, <path>, …`, naming each folder's path;
- `boomdrive: workspace left at <file>: <what is wrong>`, where what is wrong is one of: `it is a link`; `it is not a file`; `it is not a JSON object`; `it is not JSON the runner reads: <serde_json's error>`;
- `boomdrive: workspace not written at <file>: <the error>`, and `boomdrive: workspace not read at <file>: <the error>`;
- `boomdrive: workspace removed from <file>, with its folder` (Decision 7).

A workspace whose folders are already the wanted ones says nothing, so a quiet pass stays silent (the running spec).

### 7. A workspace goes with its owner's folder

When `view::remove` has removed a view, and `code_workspaces` is true, the folder the view stood in is looked at. Where it then holds only `<folder name>.code-workspace`, with or without its partial file, and that file is not the workspace of an owner the configuration keeps now, the runner removes both and then the folder. Where the option is off, the folder is left as it is now: it is removed only when empty.

- **A dropped owner.** When an owner's last kept repository is dropped and its view removed, its workspace and folder go.
- **A changed root.** The old root's owner folder goes with its workspace, once its last old view is removed. The workspace at the new root is made at the end of the same pass, afresh.
- **A dropped repository of an owner still kept.** Its workspace stays, even where the removal leaves the folder holding only that file, since it is the kept owner's workspace now. That happens when the owner's other views do not stand, as for an origin with no branch.

`view::remove` gives the pass the folder it left. The pass, which holds the configuration, decides and removes. `let_go` and `let_go_of_old_view` take the configuration's wanted workspace paths for it.

Alternative: **never remove a workspace.** Not taken: an owner's folder would stay for good after its last view went, with a workspace naming folders that are gone.

### 8. The tests

- **Unit tests** in `src/config.rs` for the key, and in `src/code_workspace.rs` for the wanted workspaces and for each row of Decision 5's table. `decide` is a pure function of what stands and the wanted folders, so each case is a value, with no file.
- **`tests/code_workspaces.rs`**, a new file, runs passes on the scratch host of `tests/common` (`Scratch::pass`) for the scenarios of the views spec's three added requirements.
- **`tests/dropped.rs` and `tests/views.rs`** gain the modified requirement's new scenarios, beside its two that stand.
- **`tests/loop.rs`** proves the configuration spec's "Workspaces asked for while running", as it proves "A repository added while running".

A test reads a workspace as JSON and compares values, and compares bytes only where a scenario says "byte for byte". Paths are the scratch home's, with neutral names (core.41).

### Rules relied on

- **core.5**: a workspace lists the configuration's kept repositories, and is gathered by owner, never by a stream, a kind or a drive.
- **core.12**: `code_workspaces` and `code_workspace` name VS Code's file as VS Code means it, apart from herdr's workspaces.
- **core.14** and **core.35**: the workspace is in the user's repository root, in no view, no clone's container and not the data directory.
- **core.17**: the key is the configuration's, which a person writes by hand, and a managed level may set it.
- **core.2** and **core.42**: no control plane's implementation reads the key, and no message of the protocol changes.
- **core.11**: a workspace holds paths and names only, and a line names a file and an error, never the file's text.
- **core.40**: each line is the pass's, prefixed with its time by the running runner.
- **core.7** and **core.8**: no crate, tool or tracked file type is added. Tests write workspaces only in their scratch folders.
- **core.43**, which this change adds.

### Rules departed from

None.

## What this change asks of others

- **The hosts' configuration**, afterthought/swancloud: its herdr.14 module keeps writing its spaces' workspaces while crew is the control plane, and leaves `code_workspaces` off in the runner's configuration. Once a host's runner keeps a context's repositories and views, the host asks for the option and the module retires. A window manager's rules then match each owner's name. This is told to the orchestrator, not done here.

## Risks / Trade-offs

- **A file with a person's comments stays as it is.** Its folders stop following the configuration, and it is named at every pass until the comments go.
- **The window's title is the owner's name.** A person whose window manager matches another name changes the rule, or keeps a workspace of their own elsewhere.
- **A changed root starts each workspace afresh.** The old workspace's settings go with the old owner's folder.
- **VS Code and the runner may write the file at once.** The runner reads, decides and renames within one call, and writes only when the configuration's list has changed, so the window is small. Where VS Code wins, the next pass sets the folders again. Where the runner wins, a setting VS Code wrote in that moment is lost.
- **`preserve_order` is unified across a build of `boomdrive`.** A later change that writes a `serde_json::Map` back as text keeps the order it read, not a sorted one.

## Migration Plan

Nothing on a host changes until its configuration says `code_workspaces = true`. A host that asks gets its workspaces at its next pass. A workspace already at an owner's path is taken over: its `folders` are set, and the rest is kept.

## After deploy

Nothing. No step of this change is outside the gate.

## Task notes

**1.1** `src/config.rs`: `File` gains `code_workspaces: Option<Spanned<Value>>`, and `Config` gains `pub code_workspaces: bool`, with its doc comment. Absent, false. A value that is not a TOML boolean is refused at its line: `` `code_workspaces` is <value as `named` gives it>: it is `true` or `false` ``. Unit tests:

- `no_workspaces_setting`;
- `workspaces_asked_for_in_words`, and an integer value refused alike;
- `a_whole_file` reads `code_workspaces = true`.

Every test that builds a `Config` by hand gains the field.

**1.2** README's configuration block gains, after `fetch_interval`:

```toml
# Whether each owner's views have a VS Code workspace, <repository_root>/<owner>/<owner>.code-workspace; false where none is named.
code_workspaces = true
```

The words under the block say what it asks for: one workspace per owner, its folders the owner's kept repositories set at every pass, the rest of the file kept, and one it cannot read left and named. `the_readme_configuration_reads` asserts `config.code_workspaces`.

**2.1** `src/code_workspace.rs`, a module of the library, named in `src/lib.rs`:

- `wanted(config) -> Vec<Wanted>`: each owner of `config.repositories`, compared without case, with its file's path and its folders' paths (Decisions 1 and 3), in the order its first repository stands in the file;
- `stands(path) -> io::Result<Stands>`: `Missing`, `File(text)`, `Link` or `Other`, read with `symlink_metadata`;
- `decide(&Stands, &[String]) -> Act`: `Leave`, `Write(text)` or `Name(what is wrong)`;
- `write(path, text) -> io::Result<()>`: the partial file, the permissions and the rename of Decision 4.

Unit tests:

- `wanted` for two owners, owners differing in case, and a configuration keeping none;
- `decide` for each row of Decision 5's table but the failures, including a file without `folders`, a file whose folders are equal but written in another layout, and the new file's exact text;
- `write` keeps an old file's permissions and leaves no partial file.

**2.2** `pass::pass` keeps the workspaces where `config.code_workspaces` is, at the point Decision 6 names, and says its lines. A failure to read or write fails the pass. `tests/code_workspaces.rs` asserts each scenario's files as JSON values and its lines by their beginnings, `boomdrive: workspace written at <file>` or `boomdrive: workspace left at <file>`.

- "A workspace that cannot be written" sets the owner's folder to mode 0555 after the first pass, and back before the scratch folder is removed.
- "A link at the workspace's path" links to a file outside the repository root.

**2.3** `tests/loop.rs`: a running runner keeping `agentplot/flume` with no `code_workspaces`, the key then added, and the workspace waited for, as "A repository added while running" waits for its clone.

**3.1** `view::remove` gives the owner's folder it left, or the pass reads the view's parent. `pass::let_go` and `pass::let_go_of_old_view` take the configuration's wanted workspace paths and `code_workspaces`, and remove the workspace, its partial file and the folder as Decision 7 has it, saying the line. The two scenarios of the requirement that stand keep passing as they are.

- `tests/dropped.rs` proves "The owner's last view, with its workspace", "An owner still kept keeps its workspace" and "A workspace not asked for".
- `tests/views.rs` proves "A new root, the old owner's workspace".
