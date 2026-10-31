# Design

## Context

See proposal.md for why. This change builds crew record 0052's part 2, carried whole, in the runner the user settled on 2026-10-31: `flywheel`, from this repository (record 0001). Here, the runner's own configuration names what a host keeps, and crew keeps no registry (core.5; crew's `teams-cross-to-flume-as-their-bolts-land`, Non-Goals and its `roles.9` departure).

What stands today:

- `crates/flywheel` is one binary that answers `--version` and `-V` and prints its usage for anything else, exiting 2. It has no dependencies and no library.
- swancloud does on every host what this change moves into the runner:
  - `space-clone` clones each space's repositories into the bare-container layout, `<dir>/<repo>/<repo>.git` with a `.git` gitfile and worktrees as peer folders, and never touches a folder already there (`names.3`).
  - Every two minutes, `packages/swancloud-views` fetches each repository over https, so git's credential helper answers and never the 1Password agent.
  - It levels the bare repository's own default branch to origin's tip as a fast-forward, while no worktree holds it (`herdr.13` as amended 2026-10-26).
  - It keeps `~/Code/views/<space>/<repo>`, a worktree detached where the repository's work is. It records the commit it last set in `$XDG_STATE_HOME/swancloud/views/`, and moves the view only while it is detached, clean of tracked changes (`status --porcelain --untracked-files=no`) and at that commit (`herdr.14`).
  - The views timer follows the local `<default>` for a repository that does not land through sluice. Record 0052 part 2 makes every view follow `origin/<default>`: the runner's clone holds no one's work but Flume's places, and they land through sluice.
- Flume 0.3.6 finds a repository named by `--repository <dir>` as either a repository git finds there or a directory holding `<name>.git` and its worktrees. It makes each place at `<dir>/places/<kind>--<name>` (flume `crates/flume/src/git.rs`, `crates/flume-core/src/launch.rs`). sluice lands a place by a merge commit, so a landed branch's commits are reachable from `origin/<default>`.
- The sibling change `the-runner-carries-crews-work-into-flume` is being constructed beside this one. It defines the control-plane interface and crew's implementation, and runs `flume launch` in the clones this change keeps. Decision 12 says what it builds on.

Git is the only tool this change runs. It is run by its command line, as flume runs it, and never through a library (record 0001: the runner reaches every tool by its command line).

## Goals / Non-Goals

**Goals:**

- Every repository a host's configuration names has one bare clone, hidden in the runner's data directory and laid out as Flume reads a repository. It is fetched over https on a timer and at once on a landing the runner is told of.
- Every such repository has one view the user browses, kept at `origin/<default>` until the user moves it or works in it. A view the user moved is never moved back under them.
- A repository dropped from the configuration stops being fetched, and goes away only once nothing in its clone or view would be lost.
- `flywheel run` is a process a host's service manager can start and keep alive: one per data directory, quiet when nothing changes, stopping cleanly. Every connection it makes is outgoing.

**Non-Goals:**

- The control-plane interface, crew's implementation, and turning requests into `flume launch` (the sibling change).
- Who runs a repository's `sluice run` rounds on a host (record 0052, for the plan to settle). The clone is a plain bare container that a `sluice run` could be pointed at.
- Adopting swancloud's existing containers and views. The runner clones afresh into its own data directory and never touches a folder it did not make (record 0052, for the plan to settle; Decision 3).
- A `.code-workspace` file beside the views, a host without views, a view per branch, or a forge other than GitHub.
- The host's service definition, a launchd agent or a systemd unit. That is swancloud's, which installs the runner (record 0052 part 4).

## Decisions

### 1. One crate, a library beside its binary

`crates/flywheel` gains `src/lib.rs`. `src/main.rs` becomes the command line over it. There is no new crate, so no new moon project (core.7). The modules, each named for what it keeps:

| module | what it holds |
|---|---|
| `config` | `Config`, `Address`, `Period`, the file's path and its reading (Decision 2) |
| `git` | every git call: one builder, its environment, a failure's words, the version check (Decisions 5, 10) |
| `clone` | a clone's paths and record, making it, fetching it, its default branch and levelling it (Decisions 3 to 6) |
| `view` | a view's path, the decision to make, move or leave it, and the move with the views root (Decision 7) |
| `lacks` | the work the forge lacks in a clone and in its view (Decision 8) |
| `pass` | one pass over the configuration, its outcome and its lines (Decision 9) |
| `run` | the lock, the start's checks, the loop, its events and its stop (Decision 10) |

Each decision is a plain function of facts read beforehand, with no git call inside it, and is unit-tested in its module. Examples are `view::decide(ViewFacts) -> ViewAct`, `clone::level(LevelFacts) -> LevelAct`, `lacks::holdings(...)` and `run::next_pass(...)`. The acts around them run git. Integration tests under `crates/flywheel/tests/` run the binary, or the library's loop, against a scratch forge on disk (Decision 11).

The dependencies are workspace dependencies in the root `Cargo.toml`, so the sibling's code shares them, at flume's versions where flume has the crate: `clap` 4 (derive) for the command line, `serde` 1 (derive) and `toml` 1 for the configuration, and `signal-hook` 0.3 for SIGTERM and SIGINT. The tests add `tempfile` 3 and `libc` 0.2, the latter to send a signal. No crate of flume's, sluice's or crew's is used, and no git library is used. The lock is std's `File::try_lock`, stable since Rust 1.89; the tree pins 1.98.1.

Alternative: a crate per concern (`flywheel-config`, `flywheel-repositories`). Not taken. Nothing outside the binary uses them yet. The sibling can split a crate out when its control-plane implementations need a boundary of their own (core.2).

### 2. The configuration is one TOML file, read at every pass

The file is `--config <file>`, else `$XDG_CONFIG_HOME/flywheel/config.toml`, else `~/.config/flywheel/config.toml`, the layout flume's host file uses. An `XDG_*` variable that is empty or not absolute is taken as unset, as the XDG base directory specification says.

```toml
# Where each repository's view is kept, at <views>/<owner>/<name>; ~/Code where none is named.
views = "~/Code"
# How often every repository is fetched; 2m where none is named.
fetch_every = "2m"
# Each repository this host keeps, by its GitHub address.
repositories = ["agentplot/flume", "afterthought/crew"]
```

- `repositories` is required, so an empty file never reads as "keep nothing" and lets everything go (Decision 8). Addresses are compared without case, as GitHub compares them, and two equal ones are refused.
- `views` is absolute or `~/…`. Nothing else is expanded, so `$HOME/Code` is refused rather than read as a relative path.
- `fetch_every` is `<n>s`, `<n>m` or `<n>h` with `n` above zero, the form flume's `--stall-after` reads.
- Unknown keys are refused (`deny_unknown_fields`), so a misspelt key never silently does nothing, and the file has no place for a credential (core.4).
- A refusal names the file, the line and the key or entry. It is built from `toml`'s message and span and never from its rendered excerpt, which would print the line's text, a value someone put there by mistake among it.

The address gives the clone's URL, `https://github.com/<owner>/<name>.git`, and the paths of its container and view. There is no `forge` setting: a test points git at a scratch forge with `url.<file>.insteadOf` in its own git configuration (Decision 11), so no code path exists only for tests.

The running runner reads the file again at the start of every pass. A pass that cannot read it names why and does nothing else, because a file mid-edit or deleted must never look like one that dropped every repository. At start, a file that cannot be read stops the runner with exit 2 (Decision 10).

Alternatives:

- Repositories as tables (`[repositories."agentplot/flume"]`), leaving room for settings per repository. Not taken: there are none yet, and a list can become tables when one comes.
- A `default_branch` per repository. Not taken: origin's HEAD says it, and the fetch keeps it current (Decision 6).

### 3. A clone lives in the runner's data directory, in Flume's layout, and carries its own record

The data directory is `$XDG_DATA_HOME/flywheel`, else `~/.local/share/flywheel`. It holds `lock` (Decision 10) and `repositories/`. The `repositories/` level leaves room beside it for the sibling's clone of a partition's state repository. Each repository's container is `repositories/<owner>/<name>/`:

```
<data>/repositories/agentplot/flume/
  flume.git/        the bare clone
  .git              gitdir: ./flume.git
  places/           Flume's, made by flume launch --repository <this folder>
```

This is swancloud's bare-container layout (`names.3`), less the anchor worktree. No one works in the clone's own checkout, and the view takes its place for the user. The container is the directory the sibling passes to `flume launch --repository`. Flume finds `flume.git` in it, or follows `.git`.

The runner's record of a clone is kept in the clone itself:

- `flywheel.address` in its git config, the address as written when it was made;
- `flywheel.view`, the view's path once one is made;
- `refs/flywheel/view`, the commit the runner last put the view at.

So everything about a repository goes with its container, a dropped one leaves nothing behind elsewhere, and the recorded commit stays reachable. A pass finds its clones by listing `repositories/*/*` and reading each one's `flywheel.address`, not by trusting a path, so a change of case in the configuration finds the same clone. A folder there with no record is not the runner's. It is never fetched, removed or made over, and is named when it stands where a configured repository's container would go.

Alternative: a state file under `$XDG_STATE_HOME`, as swancloud's views timer keeps one. Not taken: a second place to keep in step with the clone, and to clean up when the clone goes.

### 4. A clone is made whole or not at all

The runner makes a clone in `repositories/<owner>/.<name>.partial`, removing any such folder left by an earlier run first, since the name is the runner's own:

1. `git init --bare <name>.git`.
2. Set origin to the https URL with the refspec `+refs/heads/*:refs/remotes/origin/*`, `remote.origin.followRemoteHEAD = always` (Decision 6), and `flywheel.address`.
3. Apply the ssh guard (Decision 5).
4. Fetch.
5. Point HEAD at `refs/heads/<default>` and create that branch at `origin/<default>`, where origin names a default.
6. Write the `.git` gitfile.
7. Rename the folder into place.

A step that fails removes the partial folder and names the repository with git's words. The next pass tries again. `git init` and a fetch are used rather than `git clone --bare`, which copies every branch of origin as a local branch. Those copies would go stale and read as branches of the clone's own, both to Flume and to Decision 8.

### 5. A fetch is https through the host's credential helper, and never asks

Every git call is built in one place (`git::Git`). It sets `GIT_TERMINAL_PROMPT=0` and removes `GIT_ASKPASS` and `SSH_ASKPASS` from git's environment. It gives git no credential, no token and no `-c credential.*`: the host's own credential helper (`gh` on a Mac, a token helper on a box) answers, or the fetch fails without asking anyone (core.4). It takes an environment overlay from its caller, which is empty in production and holds the scratch git configuration in tests. A failure's words are the last line git wrote on standard error, as swancloud's timer names them.

The clone's URL is https because the runner sets it. A host's git configuration can still rewrite it: `url."git@github.com:".insteadOf = "https://github.com/"` is a common one, and it would send the fetch through the ssh agent, which on a Mac waits on 1Password and someone at the keyboard. Before each fetch, the runner reads the URL git would use (`git ls-remote --get-url origin`). If that is in ssh form (`git@…:` or `ssh://`), it runs no fetch and names the repository with that URL. Any other scheme passes, which is what lets a test's `file://` forge stand in for GitHub.

Every fetch is `fetch --prune --quiet origin`, and each repository is fetched on its own. One failure is named, and the rest of the pass goes on.

### 6. The default branch is origin's HEAD, and the clone's own follows origin's tip

`remote.origin.followRemoteHEAD = always` makes every fetch keep `refs/remotes/origin/HEAD` at origin's default. This is git 2.48 and later, the least version the runner starts with (Decision 10). It also means no `ls-remote` call per pass and no `default_branch` setting. After each fetch:

- **The default.** `<default>` is what `refs/remotes/origin/HEAD` names. Where origin names none, or names a branch the fetch did not bring, the repository is named as having no default branch, and gets no view this pass. Where the clone's HEAD names another branch, origin having renamed its default, HEAD is pointed at the new branch, and that branch is created at origin's tip if the clone lacks it.
- **Levelling** (`herdr.13` as amended 2026-10-26, carried whole). The clone's own `refs/heads/<default>` is moved to `origin/<default>` by `update-ref <ref> <new> <old>` only when the move is a fast-forward and no worktree of the clone has the branch checked out, read from `worktree list --porcelain`. Otherwise it is left and named: with how many commits it is ahead, or with the worktree that holds it. sluice moves the branch at a landing on this host; levelling keeps it at origin's tip between landings, so whatever reads the clone's default branch reads origin's.

### 7. The view: made where missing, moved only where the runner last put it

A view's path is `<views>/<owner>/<name>`. Owner and name together are the address, so two owners' repositories of one name never meet, as `agentplot/blueprints` and `afterthought/blueprints` would under `<views>/<name>`. This settles record 0052's question of how views are grouped once spaces have gone: by owner, the address's own first part.

`view::decide` takes plain facts and gives one act:

- **The facts:** the configured path `P`; the recorded path `R`; the recorded commit `C`; the target `T`, which is `origin/<default>` or none. For each folder at `P` and at `R`, whether it is missing, a folder that is not a worktree of this clone, or this clone's worktree. For the clone's own worktree: whether HEAD is detached, its HEAD, whether it holds a change to a tracked file, and what it holds that the forge lacks (Decision 8). A folder is the clone's worktree when `git -C <folder> rev-parse --path-format=absolute --git-common-dir` is the clone's bare repository, swancloud's own test.
- **No target.** Do nothing. The clone already named it.
- **`R` is set and is not `P`** (the views root changed). If `R` is the clone's worktree and holds nothing the forge lacks, remove it (Decision 8's removal), forget `R`, and go on to `P`. If it holds work, leave it, name it with what it holds, and make no view at `P`. If `R` is missing or no longer the clone's worktree, forget it and go on to `P`, leaving any folder there untouched.
- **`P` is missing.** Make the view: `worktree add --detach P T`, creating the folders above `P`. Add `--force` only when git still registers `P` as a worktree whose folder is gone, as after the user deleted it. That flag overrides only that safeguard, because no branch is named.
- **`P` is not this clone's worktree.** Leave it and name it. It is never made over, and no view is made elsewhere.
- **`P` is the view.** Move it with `switch --quiet --detach T` only when HEAD is detached, at `C`, holds no change to a tracked file, and `C` is not `T`. In every other case, leave it without a word. Untracked files do not hold it (`--untracked-files=no`, carried from `herdr.14`): git carries them across the switch, and refuses one the new commit would overwrite. That refusal is named with git's words.

After a make or a move, `refs/flywheel/view` is set to `T` and `flywheel.view` to `P`. A view is only ever made or moved detached, so the runner never puts it on a branch. A user's branch in it is theirs. A removed view also takes its owner's folder, `<views>/<owner>`, by `rmdir` only, so only when the folder is empty.

### 8. Letting go: work the forge lacks, judged against origin as last fetched

A clone whose `flywheel.address` the configuration no longer names is dropped. It is never fetched again, by the timer or by a landing told of it. At every pass, `lacks` reads what it holds:

- **Refs.** Each ref of the clone, excluding `refs/remotes/origin/*` and `refs/flywheel/*`, whose `rev-list --count <ref> --not --remotes=origin` is above zero, named with the count. Local branches, tags and the stash are all counted. Flume's place branches count until their commits are reachable from origin. sluice's merge landing makes a landed branch's commits reachable, so only unlanded work holds a clone.
- **Worktrees.** Each worktree of the clone, from `worktree list --porcelain`, other than the bare repository and the view, whose folder exists and is the clone's worktree or lies inside the container. A Flume place is one. Its work is Flume's, and the runner removes nothing from under it (core.1). A registered worktree whose folder is gone holds nothing. Nor does a folder outside the container that git still registers but that is no longer the clone's worktree, as an old view's that the user replaced (Decision 7): removing the container never touches it.
- **The view, when it is the clone's worktree.** `status --porcelain` not empty: changes to tracked files, and untracked files, but not files git ignores. Or a HEAD that `--not --remotes=origin` leaves commits on.

With nothing held, the runner removes the view (`worktree remove`), then its owner's folder if empty, then the container (`remove_dir_all`), then `repositories/<owner>` if empty, and names the repository as removed. A view's folder that is not the clone's worktree is left untouched, and does not hold the container. With anything held, nothing is touched, and each holding is named, at every pass, until the user pushes, deletes or commits it away.

The forge is judged by origin's refs as the last fetch or push left them. A dropped clone is never fetched again, which is what the brief says, so a branch that reached the forge some other way holds the clone until a push from the view updates `refs/remotes/origin/*`, or the branch is deleted. Erring toward keeping costs disk. Erring the other way would lose work.

### 9. A pass

One pass:

1. Read the configuration (Decision 2).
2. Find the runner's clones (Decision 3).
3. For each configured address in the file's order: make its clone if it has none (Decision 4), else fetch it (Decision 5). Then settle its default branch and level it (Decision 6), then keep its view (Decision 7).
4. Let go of each dropped clone (Decision 8).
5. Remove any `.partial` folder left over.

Each repository is independent: a failure in one is named and the next is kept. The pass's outcome is whether any git call failed. Something left and named by rule is not a failure: a view that is not the clone's worktree, a held dropped repository, a default branch left ahead. `flywheel run --once` exits 1 on a failed outcome and 0 otherwise.

`pass::keep_one(address)` is step 3 for one address, read against the configuration as it stands. This is what a landing notice runs (Decision 10). For an address the configuration does not name, it does nothing but name it (core.5).

The pass says each act and each thing named as one line on standard error, `<owner>/<name>: <words>`. Examples: `cloned from https://github.com/agentplot/flume.git`, `main moved to origin/main at 1a2b3c4`, `view made at /h/Code/agentplot/flume, at origin/main 1a2b3c4`, `view moved to origin/main at 5d6e7f8`, `left: held by branch wip, 2 commits the forge lacks`, `removed: clone and view`. Commits are given at seven characters. A view the user moved or is working in is left without a word, as `herdr.14` has it. A pass with nothing to say writes nothing.

### 10. flywheel run: one long-running process per data directory, all outgoing

The command line (`clap`):

- `flywheel --version` and `-V` print `flywheel <version>`.
- `flywheel run [--config <file>] [--once]`.
- A usage error prints the usage on standard error and exits 2, as today.

The start's checks come first, in this order, each refusing with exit 2 and touching nothing:

1. `HOME` is set.
2. The configuration reads (Decision 2).
3. `git --version` runs and says 2.48 or later. Apple's `git version 2.50.1 (Apple Git-155)` reads as 2.50.1.
4. The data directory exists or can be made.
5. `File::try_lock` on `<data>/lock` succeeds, so a second runner, or a `--once` beside a running one, is refused, naming the data directory as held.

The lock is held until exit. Then it says one line: `flywheel <version>: keeping <n> repositories from <config>; data <data>; views <views>`.

`--once` makes one pass and exits 0 or 1 (Decision 9).

Without `--once`, the loop is the runner's one thread of work, so no two of the runner's git calls ever touch one clone at once:

- It makes a pass at once, then waits on a channel of events until the next pass is due. `run::next_pass(last_start, every, now)` gives `max(last_start + every, now)`: passes are timed from the start of the one before, and one that overran is followed at once.
- `Event::Landed(Address)` runs `keep_one` at once and leaves the timer where it was.
- `Event::Stop` comes from a `signal-hook` thread on SIGTERM or SIGINT. It also sets a flag that a pass reads between repositories, so the runner finishes the git call in hand, then says `flywheel: stopped` and exits 0.
- `Runner::notices()` hands out a `Notices`, a clonable sender with `landed(address)`, for whatever is told of landings.

A host's service manager starts `flywheel run` and keeps it alive: launchd's `KeepAlive` on a Mac, `Restart=always` in a systemd user unit on a box. Both are swancloud's to write (record 0052 part 4). Exit 2 means the host must be fixed, and the service manager's throttle keeps it from spinning.

The runner opens no listening socket, so no connection comes in to a host (core.3). This change's only connections are git's fetches over https. A test reads the crate's sources for `TcpListener`, `UnixListener`, `UdpSocket` or a socket's `bind`, and reads `Cargo.lock` for a server crate (`hyper`, `axum`, `actix-web`, `warp`, `tiny_http`, `rouille`, `tide`, `poem`). The sibling's state-repository fetch and push, and the hosted control plane's WebSocket, are outgoing too.

Alternative: a separate `flywheel keep` command for one pass. Not taken: `run --once` is the same pass under the same lock, so there is one code path, not two.

### 11. Tests run on a scratch host against a scratch forge

`crates/flywheel/tests/common/mod.rs` gives each test a temporary directory holding:

- `home/`, used as `HOME`, with `XDG_CONFIG_HOME` and `XDG_DATA_HOME` removed or set inside it;
- `gitconfig`, used as `GIT_CONFIG_GLOBAL`, with `GIT_CONFIG_NOSYSTEM=1`. It holds a user name and email and `url."file://<forge>/".insteadOf = "https://github.com/"`;
- `forge/<owner>/<name>.git`, bare repositories made by plain git through helpers: a repository with a first commit on `main`, a commit on a branch, a branch deleted, HEAD pointed elsewhere.

Every git call a test makes, and every one the runner makes in it, runs under that environment. The binary is run as `env!("CARGO_BIN_EXE_flywheel")` with that environment. The library's loop is run in-process with the same overlay given to `git::Git`. So no test reads the user's git configuration, reaches the network, or holds a credential (core.4; `.config/sluice.toml`'s `network = "localhost"`). The views root in a test is inside `home/`.

Git is now a tool `flywheel:test` runs, so the tree pins it (core.8). `devenv.nix` names `gitVersion = "2.55.0"`, asserts `pkgs.git.version == gitVersion` where it puts git in `packages`, and `check-toolchain.sh` checks `git --version` against it, as it checks OpenSpec. `flywheel:test` gains `/README.md` among its inputs, because a test reads README's example configuration (Task notes 1.2).

### 12. What the sibling change builds on

`the-runner-carries-crews-work-into-flume` depends on this change. It uses:

- `config::Config`. It adds its own table for the control plane, by a MODIFIED requirement of `configuration`'s "holds nothing the runner does not read".
- `pass::container_of(&Config, &Address) -> Option<PathBuf>`: the directory `flume launch --repository` names, for an address the configuration names and whose clone exists. A request for any other repository is refused and said to the control plane (core.1's "not ruled out", core.5).
- `pass::keep_one`, to fetch at once before a launch whose base the last fetch may lack.
- `Notices::landed`, which the control-plane side calls when it learns of a landing, a place it launched landing among them.
- The run loop. It adds its own events and timers to this loop, its pull of the state repository among them, rather than running a second loop. That keeps the runner's git calls one at a time.
- The scratch host and forge of `tests/common`.

Nothing in this change names crew, its records or a hosted control plane (core.2).

### Rules relied on

- **core.1.** The runner keeps and fetches, and starts no work. A Flume place in a dropped clone holds it rather than being removed from under Flume.
- **core.2.** Nothing here names a control plane. A landing reaches the loop as a plain event.
- **core.3.** No listening socket.
- **core.4.** No credential in the configuration, the code or the tests. The host's credential helper answers.
- **core.5,** as amended in this change's commit, below.
- **core.7.** No new crate. No new file type: Rust, TOML and Markdown are already claimed or excluded. The test task's inputs name what it reads.
- **core.8.** git is pinned by version and checked.

### Rules this design departs from

- **core.5.** The brief, record 0052 part 2 carried whole, leaves a dropped repository's clone and view untouched and named while either holds work the forge lacks. core.5's detail as written ruled out any clone or view left after its repository leaves the configuration. The commit that writes this design amends that detail. It now rules out a dropped repository's clone or view being fetched, moved or made, or being left once neither holds work the forge lacks. It allows one left and named while either does. The rule's statement, which keeps no clone or view of a repository the configuration does not name, is unchanged: a held clone is neither fetched nor viewed, only left.

## Risks / Trade-offs

- **A second full clone of every repository** while swancloud's containers remain → the runner's are the ones Flume launches into. swancloud retires its own when record 0052's asks of it are taken up, and this change touches none of them.
- **A host whose git rewrites https to ssh** fetches nothing → each pass names the repository and the URL. Fixing the host's git configuration is swancloud's.
- **A dropped clone held by an unlanded branch** stays until someone acts → it is named at every pass with the branch and its count. Deleting or pushing the branch lets it go.
- **A view path that holds a folder of the user's** gets no view → named at every pass. The user moves the folder, or the views root.
- **Flume and the runner in one clone at once** (a place made while a fetch runs) can meet on git's lock files → the failed call is named and tried again at the next pass. Nothing is forced or retried in a loop.
- **Origin judged as last fetched** for a dropped repository keeps some clones longer than needed → disk, never work.

## Migration Plan

Nothing converts in code. On a host:

1. swancloud installs `flywheel`, writes `~/.config/flywheel/config.toml` listing the host's repositories, and starts `flywheel run` under its service manager.
2. The first pass clones each repository afresh into `~/.local/share/flywheel/repositories/` and makes its view under `~/Code/<owner>/`.
3. swancloud's containers, `~/Code/views/` and its views timer go on as they are until swancloud retires them.

To roll back, stop the service. A host's runner data is let go by setting `repositories = []` and running one pass, which removes everything that holds no work of its own, and names the rest.

## Task notes

**1.1** `config.rs`:

- `Address { owner, name }` with `parse(&str)` and its refusals.
- `Period` with `parse` and `Duration`.
- `Config { views: PathBuf, fetch_every: Duration, repositories: Vec<Address> }`, read by `parse(text, file, home)` from a serde struct with `deny_unknown_fields`.
- `config_path(arg, env) -> PathBuf` and `data_dir(env) -> PathBuf`, both over an explicit environment so tests need no process environment.
- `view_path(&Config, &Address)`.

One unit test per scenario of the configuration spec but the first three and the last two. The token test asserts the refusal's words hold `token` and not its value.

**1.2** README gains "The configuration", holding the example of Decision 2 as a fenced `toml` block. `crates/flywheel/tests/readme.rs` reads `README.md` from the workspace root and parses its first `toml` block with `config::parse`. `crates/flywheel/moon.yml`'s `test` task gains `/README.md` in `inputs`.

**2.1** `git.rs`:

- `Git::new(overlay)` and `Git::run(at, args) -> Result<String, Words>`, where `at` is `GitDir(path)` or `Dir(path)`.
- The environment of Decision 5.
- `version(text) -> Option<(u32, u32, u32)>` and `starts_with(version) -> Result<(), String>`, for 2.48.
- `is_ssh(url)`.

Unit tests:

- A built command's environment holds `GIT_TERMINAL_PROMPT=0` and neither askpass variable, and its arguments hold no `credential`.
- Versions: `2.47.1` refused; `2.48.0` and `2.50.1 (Apple Git-155)` taken; junk refused.
- URLs: `git@github.com:a/b.git` and `ssh://git@github.com/a/b.git` are ssh; `https://…` and `file://…` are not.

**2.2** `tests/common/mod.rs` per Decision 11, and `tests/scratch.rs` proving it: plain `git clone` of `https://github.com/o/r.git` under the scratch environment reads the forge's commit, with no network.

**2.3** `devenv.nix`: `gitVersion` beside `openspecVersion`, and `packages` takes `assert pkgs.git.version == gitVersion; pkgs.git`. `check-toolchain.sh`: `release git "$(pinned gitVersion)" git --version`.

**3.1** `clone.rs`: `container(data, &Address)`, `find(data) -> Vec<(Address, PathBuf)>` by `flywheel.address`, `make(...)` per Decision 4. Tests in `tests/clones.rs`: a new repository is cloned; origin cannot be reached (an address the forge lacks); a folder already there.

**3.2** `clone.rs`: `fetch` with the ssh guard, `default_of`, `settle_head`, `level` (decision) and its act. Tests in `tests/clones.rs`, one per scenario named in task 3.2. The worktree that holds `main` is one the test adds to the clone.

**3.3** `pass.rs` without views or drops yet, and `main.rs` on `clap` with `run`, `--config` and `--once`. The start's checks and the lock go in `run.rs`. Tests in `tests/running.rs`:

- the version;
- no command;
- every repository kept;
- a clone failed;
- a second runner, where the test holds `<data>/lock` with `File::lock`;
- a bad configuration at start;
- the default file, the file named, and no file, from the configuration spec.

An old git is `starts_with`'s unit test.

**4.1** `view.rs`: `ViewFacts`, `ViewAct` and `decide` per Decision 7. One unit test per case of Decision 7.

**4.2** `view.rs`: reading the facts and the acts, called from the pass after levelling. Tests in `tests/views.rs`, one per views scenario named in task 4.2, plus the clones spec's "The clone is not under the views root", "Origin's default is not main" and "An empty origin".

**5.1** `lacks.rs`: `Holding` (`Ref { name, ahead }`, `Worktree { path }`, `ViewChanges { count }`, `ViewCommits { ahead }`), the reads, and its words. Unit tests over facts for each holding, and for ignored files holding nothing.

**5.2** `pass.rs`: dropped clones per Decision 8, and the owner folders' `rmdir`. Tests in `tests/dropped.rs`, one per scenario named in task 5.2. The place is a worktree the test adds under the container's `places/`.

**5.3** `view.rs`: the `R` is not `P` cases. Tests in `tests/views.rs` for the three new-root scenarios.

**6.1** `run.rs`: `next_pass` with its unit test; the `Event` channel; `Notices`; the signal thread; the stop flag the pass reads; the configuration read at every pass. Tests:

- `tests/running.rs`: stopped while waiting, where the test waits for the start line, then sends SIGTERM with `libc::kill` and asserts exit 0 within a second.
- `tests/loop.rs`, running the library's loop in a thread under the scratch overlay with `fetch_every` one second or one hour as the scenario needs: told between passes; told of a repository it does not keep; a failure does not stop it; a repository added while running; a file broken while running.

**6.2** `tests/no_listener.rs` per Decision 10. Tests in `tests/running.rs` for a quiet pass and a view moved, asserting standard error.

**6.3** README: "Running on a host", which covers `flywheel run`, `--once`, the data directory, views, and that the host's service manager keeps it alive. CLAUDE.md's opening paragraph drops "For now the crate prints its version and nothing more", and says it keeps the repositories its configuration names.
