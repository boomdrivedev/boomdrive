# Design

## Context

The runner reads its configuration in two layers of TOML (the configuration spec; record 0009 as amended 2026-11-06 and 2026-11-07): the managed layer, the file `--config` names or `config.toml`, and the local layer, `config.local.toml` in the runner's own configuration directory, read over it. The managed layer names the repositories a host keeps in one list, `repositories`. The local layer sets `repositories`, which add to the managed layer's, and a control plane's `partitions`, and the managed layer's `pin` holds keys against it. `code_workspaces = true` asks for one VS Code workspace per owner, `<repository_root>/<owner>/<owner>.code-workspace` (the views spec; record 0017 as first accepted).

Record 0028, accepted 2026-11-07, names a host's work by accounts and drives. An account is what a runner enrolls with a control plane. A drive is one account's body of work: its repositories, one Flume stream named after it and one branch of the account's crew state. The runner's file names its accounts, each account's drives and each drive's repositories, and still no stream or kind (core.5). Record 0017, revised the same day, makes the workspace one per drive, at the repository root under the drive's name (core.43).

The rules this design relies on:

- **core.5**: the runner keeps a clone or a view only of a repository its configuration names; the configuration names accounts, drives, repositories and where their views go, and no stream or kind.
- **core.43**: one workspace per drive at the root, `folders` exactly the drive's kept repositories, set at every pass, the rest kept; none written or removed where the configuration does not ask.
- **core.12**: keys and types name what they hold.
- **core.2**: no code outside an implementation names crew or the hosted protocol.
- **core.14**, **core.35** and **core.36**: what the runner may keep in a clone and its data directory, and that a field may be added.
- **core.4** and **core.11**: no refusal quotes the file's text.
- **core.41**: neutral names in tests and fixtures.
- **core.46**: the modules' options are the file's keys, by the file's names.
- **core.17**: the local layer sets only the keys the runner lists for it, its repositories adding to the managed layer's, and the managed layer may pin.

This design departs from no rule. core.43's details name the moved case of Decision 8 among what the rule does not rule out, with record 0017's amendment.

## Goals / Non-Goals

**Goals**

- The file names accounts, drives and repositories, and the runner keeps every drive's repositories as it keeps them today.
- One workspace per drive where asked, which goes with its drive.
- The modules' `settings` render the new file.
- The local layer sets each drive's repositories, which add to the managed layer's drive by drive.

**Non-Goals**

- **The control plane's tables.** `[control_plane.crew]` and `[control_plane.hosted]` keep their `partitions` as they are (Decision 9).
- **`services.boomdrive.accounts`.** Record 0028's option, with each drive's stream, machine and the account's state repository, is the flake bolt's unit `each-repository-is-named-once-by-its-owner`. It writes `settings.accounts` as this change declares it.
- **Messages.** No protocol message and no report field changes (core.42).
- **Old workspaces.** The workspaces an earlier runner kept per owner are not migrated: record 0017 leaves them to the person.

## Decisions

### 1. The file's form: accounts, drives, and a list of addresses in each drive

```toml
repository_root = "~/Code"
code_workspaces = true

[accounts.own.drives.home]
repositories = ["agentplot/flume", "afterthought/crew"]

[accounts.own.drives.work]
repositories = ["agentplot/sluice"]

[accounts.client.drives.garden]
repositories = []
```

Each drive's `repositories` is a list of GitHub addresses `owner/name`, read as the configuration spec reads an address today. The list's order is the order kept, and each workspace lists its folders in that order.

Accounts and drives are read in the order they stand in the file, by their keys' spans, as the reader already orders the keys it refuses. A pass keeps every drive's repositories in that order: its accounts in turn, each account's drives in turn, each drive's list in turn.

Not taken:

- **A table of owners in each drive**, `repositories.<owner> = ["<name>"]`, as record 0028's module option writes it. TOML does not order a table's keys, so the order of a drive's folders across owners would be the reader's, not the person's. An address is also the form the clones, the views, Flume and the control plane already name a repository by. The module turns its owners' names into addresses.
- **A flat list with a drive on each entry.** Record 0028 has the file name the accounts and their drives, and each drive's repositories under it.

### 2. Every level is required, and may be empty

`accounts`, each account's `drives` and each drive's `repositories` are required. A missing one is refused, naming its path, such as `accounts.own.drives.home.repositories`. Each may be empty, so a host that keeps none says `accounts = {}`, and a drive that keeps none yet says `repositories = []`.

`repositories` was required for the same reason: a file whose key is missing is a mistake, never a host that keeps nothing, so a mistake never lets go of every repository (crew record 0052 part 2; the configuration spec's "read again at every pass").

An account and a drive hold no other key. Any other is refused, naming its path, as a top-level key is: a `machine`, an account's `state`, a person's guess. That keeps the runner's file free of what record 0028 gives the modules: the drive's stream and machine, and the account's state repository. A `stream`, `streams`, `kind`, `kinds` or `kinds_file` in an account or a drive is refused with the words the top level gives for it, saying whose configuration it is (core.5).

### 3. Names: the hosted label's form, and a drive's name once on the host

An account's and a drive's name is 1 to 32 lower-case letters, digits and `-`, starting with a letter. That is the form the hosted table gives a label, which is an account's enrollment, and it lies within crew's form for a label, which is a drive's branch.

A drive's name is its stream's, its crew branch's and its workspace's file name (records 0017 and 0028). So it is given once on the host, across every account. Two accounts naming one drive are refused, naming the drive and both accounts. TOML refuses an account named twice.

Not taken: crew's own form, which also allows a leading digit. The hosted form is the narrower of the two. A name in it is a plain file name with no dot, so `<drive>.code-workspace` never escapes the root or meets a view's owner folder.

### 4. A repository belongs to one drive

Two entries that are equal without case are refused, naming both entries and their drives, whether they are in one drive or in two (record 0028: "A repository belongs to one drive"). That keeps one answer to "which drive is this repository's", which the workspaces and the clone's record need (Decision 7).

### 5. The flat list of before is refused, naming `accounts`

A top-level `repositories` joins `views` and `fetch_every` among the keys held elsewhere. Its refusal says `accounts` holds it now, each repository named under its drive, and the reader names every such key at once, as it does today.

Not taken: reading a flat list for one release as repositories of no drive. They would be repositories no drive names and no workspace lists, which record 0028 and core.5 have the file never hold.

core.36, which keeps a field a release before read, binds the files the runner keeps: its report and its records in the data directory (core.35). It does not bind the configuration a person or Nix writes. The refusal makes the upgrade loud. A host's runner refuses the file, naming `accounts`, and keeps and removes nothing until it is rewritten (the configuration spec's broken file).

### 6. The configuration's types

`config::Config` holds `accounts: Vec<Account>` in place of `repositories`:

- `Account { name, drives: Vec<Drive> }` and `Drive { name, repositories: Vec<Address> }`, named by record 0028's words (core.12).
- `Config::repositories()` gives every drive's repositories in order (Decision 1). It is what the pass, `keep_one`, the carry's `Keeping` and the start line read in place of the field.
- `Config::drive_of(&Address)` gives the drive that names a repository, compared without case as `Address` compares.

`config::view_path` is unchanged: a view is still `<repository_root>/<owner>/<name>`.

### 7. Each kept repository's clone records its drive

The pass records the drive the configuration names a kept repository under as `boomdrive.drive` in the bare repository's local git configuration. It sits beside `boomdrive.address` and `boomdrive.view`, and is written where the clone records another drive or none. `clone::find` reads it with each clone it finds, through one `git config --local --get-regexp` of the section, so a pass makes no more git calls per clone than it does today.

It is a record of the runner's own in its own clone (core.14, core.35), and a field added, which the release before ignores (core.36). It is written whether or not `code_workspaces` is asked for. So a workspace asked for later still goes with its drive.

Not taken:

- **A list of the workspaces written, in the data directory.** core.35 names everything the data directory holds, and the clones already carry the runner's records of each repository.
- **Removing every `*.code-workspace` at the root that no drive names.** That would take a person's own workspace, which core.43 leaves.

### 8. A drive's workspace goes with its drive

This runs at the end of a pass that was not stopped, where `code_workspaces` is true:

1. Take the drives the clones recorded when the pass found them.
2. Leave out the drives the configuration names.
3. Leave out the drives the clones still standing record once the pass has kept and let go.
4. For each drive left, `<repository_root>/<drive>.code-workspace` is removed where it is a regular file the runner reads as a JSON object, its partial file with it, and one line names it.
5. A link, a folder or a file the runner cannot read there is left as it is, and one line names it.
6. A removal that fails is named, and fails the pass.

The root is the configuration's root. A workspace at an old root is left where it was (record 0017: "A changed repository root starts each drive's workspace afresh at the new root. The old one, with its settings, is left where it was.").

This covers:

- **A drive dropped whose views are all removed.** The last let-go removes the last clone recording the drive.
- **A drive dropped whose views still hold work.** The clones still record it, so its workspace waits until the runner removes the last of them.
- **A drive dropped whose repositories all moved to other drives.** Its clones now record those drives, so no view is removed, and the workspace goes.

Record 0017 says "When a drive leaves the configuration and the runner has removed its last view, it removes the drive's workspace". This change reads a view moved to another drive as no longer the departed drive's. The amendment to record 0017 says so, and core.43's details follow it.

A runner stopped between a let-go and the pass's end has lost the record of a drive whose last clone it removed. That drive's workspace is left, for the person to remove. The pass never guesses a workspace's drive from its file name alone.

`keep_one`, the pass a landing runs, records the drive and removes no workspace.

### 9. The control plane's tables stay as they are

The accounts and drives are the runner's grouping of what it keeps. `[control_plane.crew]` names a partition for each drive this host serves, with its account's state repository. `[control_plane.hosted]` names a partition for each account enrolled, with the repositories it is served. Each table is handed whole to its implementation, as today (core.2).

Not taken: the runner deriving either table from its accounts. That would put crew's state repository or the hosted control plane's address among the runner's own keys, which core.2 rules out. Record 0028 gives that derivation to the modules, which write both from one option.

Renaming the tables' `partitions` to record 0028's words is crew's rename (crew record 0054) and the hosted protocol's next version (core.42), not this change.

### 10. The views' owner folder holds no workspace of the runner's

A removed view takes its owner folder only when the folder is then empty, as before record 0017's first form. `pass::take_workspace`, `code_workspace::alone_in` and the owner's workspace taken with a changed root's old folder are removed.

A workspace an earlier runner kept in an owner's folder is left as it is, with its folder and its settings. The runner writes none there again (record 0017's consequences).

### 11. The modules' `settings` declare `accounts`

`nix/modules/options.nix` declares `settings.accounts` in place of `settings.repositories`:

- `accounts` is an attribute set of accounts, each a submodule whose `drives` is an attribute set of drives. Each drive is a submodule whose `repositories` is a list of addresses, typed as `repositories` is typed today.
- `accounts` defaults to `{ }`, each account's `drives` to `{ }` and each drive's `repositories` to `[ ]`. So the file always holds each level the runner requires, and the empty settings render `[accounts]`.
- The account and drive submodules have no freeform type. A key in either that the runner does not read fails evaluation, naming its option, before the file is written. The top level of `settings` stays freeform, as the flake's design has it.
- An assertion fails evaluation where `settings` names `repositories` at its top. It says each repository is named under its drive, `settings.accounts.<account>.drives.<drive>.repositories`.

`nix/modules/toml.nix` renders nested tables already. `[accounts.<a>.drives.<d>]` is written with its `repositories`, and an empty table as its own header, so it needs no change.

The test settings and their files under `nix/tests/files` use neutral names (core.41):

- `crew` keeps `agentplot/flume` and `agentplot/sluice` in the drive `home` of the account `own`, served by crew's partition `home` at `example/state`.
- `hosted` keeps `agentplot/flume` in the drive `garden` of the account `home`, and nothing in the account `work`, served by the hosted partitions `home` and `work`.

### 12. README

README's configuration example names one account with two drives and says what each level is. Its words on `code_workspaces` say one workspace per drive at the root, titled `— <drive> (Workspace)`, which goes with its drive. Its nix example sets `settings.accounts`. Its hosted section's example is read after the configuration's block, as `tests/readme.rs` reads it, and needs only the configuration's block to keep `agentplot/flume` and `afterthought/crew`.

### 13. The local layer sets each drive's repositories

The key the local layer may set in place of the flat `repositories` is each drive's, `accounts.<account>.drives.<drive>.repositories`, one unit taken whole for each drive. `config::local_keys` gives it as a pattern whose account and drive are any name, beside each implementation's `control_plane.<type>.<key>`. An account or a drive of the local layer holds no other key: any other is a key the local layer may not set, named by its path, as `accounts.own.drives.home.stream`, and the local file is not taken. So is a flat `repositories`.

The units of the accounts are added to the managed layer's accounts, in the local file's order, after its pins are passed over:

- **A drive both layers name** under one account keeps the managed layer's repositories, in their order, then each of the local layer's the managed layer does not name. A repository both name in that drive is kept once, as the managed layer spells it.
- **A drive or an account only the local layer names** is declared by it, after the managed layer's, so a control plane can add the drives of a host it serves to a host whose Nix names none of them.
- **A repository is still in one drive on the host, and a drive's name is given once** (Decisions 3 and 4). A local layer that names a repository the managed layer keeps in another drive, or a drive the managed layer holds under another account, is not taken. So is one that names a repository twice, or a drive under two accounts, or a name out of form. Each fault names the local file and its line, in the words the managed layer's refusal uses.

The managed layer's `pin` may name `accounts`, an account, an account's `drives`, a drive, or a drive's `repositories`, each holding every drive's `repositories` under it. A pin of the flat `repositories` is a pin of nothing, the managed layer's fault.

While the local layer is not taken, a pass sets no workspace and removes none, a departed drive's among them (Decision 8). The drive the local layer could not be read for may be the one a departed drive's workspace belongs to.

Not taken:

- **The local layer setting a drive's `repositories` only where the managed layer declares the drive.** A control plane could then add no drive Nix does not name, which record 0009's amendment of 2026-11-06 has it do.
- **A repository the local layer names in another drive moved there.** A control plane's write would move work out of the drive Nix gave it, and the drive's stream with it.
- **Each layer's drives kept apart, a drive both name twice.** A drive's name is its stream's and its workspace's, given once on the host (record 0028).

Record 0028 gains an amendment of 2026-11-07 holding these choices, the writing agent's, made within records 0009 and 0028 as accepted. core.17's details gain what they rule out and allow.

## Risks / Trade-offs

- **Every host's file must be rewritten when its runner upgrades.** The refusal names `accounts`, and the runner keeps and removes nothing meanwhile. Hosts configured by Nix take the modules at the same commit as the runner. afterthought/swancloud passes deneb's accounts and drives once the modules land (record 0028's consequences).
- **Sibling changes of the flake bolt touch the same files:**
  - `a-dropped-repository-is-drained-and-no-rename-is-followed` changes the pass's let-go. Decision 8's sweep reads the clones standing at the pass's end, whatever kept them.
  - `the-flake-configures-each-tool-by-its-own-module` edits `nix/modules/options.nix` and README.
- **A control plane's local layer naming a drive Nix holds elsewhere is not taken whole.** Its other drives wait with it until it is mended, as any local layer at fault does, and the line naming the fault says which drive or repository.
- **A drive's workspace can outlive it** where a runner is stopped mid-pass, where its repositories' clones were never made, or where the drive left while `code_workspaces` was off. The file is left for the person, as an owner's workspace of before is.
- **A drive keeping no repository has a workspace with no folders.** core.43 gives each drive of the configuration one workspace. An empty window is the honest picture of an empty drive, and the workspace is ready when the drive gains its first repository.

## Migration Plan

1. The change lands with the modules' `settings.accounts` in the same tree, so a host built from one commit renders a file its runner reads.
2. A host's file is rewritten by its owner: by afterthought/swancloud through the modules, or by hand. A local layer naming the flat `repositories` is not taken until it is rewritten under its drives, and meanwhile the pass lets go of nothing (Decision 13).
3. At the first pass after it, each kept repository's clone records its drive. Each drive's workspace is written at the root where asked. The per-owner workspaces of before are left, for the person to remove.

Rollback: the runner of before refuses `accounts` as a key it does not read, so a host rolled back must restore its flat list too, in its managed layer and its local layer alike.
