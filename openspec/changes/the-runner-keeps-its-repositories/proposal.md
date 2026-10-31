# The runner keeps its repositories

## Why

A host's repositories are machine configuration today: swancloud lists each space's repositories in Nix, `space-clone` clones them into the bare-container layout (its `names.3`), and the views timer fetches them every two minutes and keeps a detached view of each under `~/Code/views/<space>/<repo>` (its `herdr.13`, `herdr.14`). Crew's record 0052, accepted 2026-10-30, moves all of that to a thin runner on each host. The user settled on 2026-10-31 that the runner is `flywheel` from this repository (record 0001), and that its own configuration names the repositories it keeps (core.5; crew keeps no registry). The crate still only prints its version. Every later part of the runner, starting with the sibling change `the-runner-carries-crews-work-into-flume`, launches into a clone this change keeps.

## What Changes

- **The configuration file.** `flywheel` reads one TOML file, `~/.config/flywheel/config.toml` (or under `$XDG_CONFIG_HOME`, or `--config <file>`). It names the repositories the host keeps by forge address (`owner/name`), the views root (`~/Code` where it names none) and how often they are fetched (`2m` where it names none). A file that cannot be read never counts as one that names no repository.
- **A clone of each repository.** Each configured repository has one bare clone in the runner's own data directory (`~/.local/share/flywheel/repositories/<owner>/<name>/<name>.git`, in the bare-container layout Flume reads), hidden from the user. It is fetched over https through the host's credential helper, never prompting, on the timer and at once when the runner is told of a landing. The clone's own default branch follows origin's while that is a fast-forward and no worktree holds it.
- **A view of each repository.** One view at `<views>/<owner>/<name>`, a worktree of that clone detached at `origin/<default>` and never on a branch. It is made where the folder is missing and never over a folder that is there. It is moved only while it holds no uncommitted change and stands where the runner last put it, so a view the user moved stays where they put it. A changed views root moves the view by removing it and making it again at the new root, only once it holds nothing the forge lacks.
- **A dropped repository.** A repository the configuration no longer names is not fetched again. Its clone and view are removed once neither holds work the forge lacks; until then they are named on every pass and left. core.5's detail is amended to say so.
- **How the runner runs.** `flywheel run` is the long-running process the host starts and keeps alive. It is one per data directory. It makes a pass over the configuration at start and then on the timer, reading the configuration again at every pass. It stops cleanly on SIGTERM or SIGINT. `flywheel run --once` makes one pass and exits, saying in its status whether any git call failed. It names what it does, one line per act, and opens no listening socket (core.3). `flywheel --version` prints its version.

## Capabilities

### New Capabilities

- `configuration`: the runner's configuration file, where it is read from, what it names, its defaults and refusals, and its reading at every pass.
- `clones`: one bare clone per configured repository in the runner's data directory, how it is made, fetched and told of a landing, its default branch, and how a dropped repository's clone is let go.
- `views`: one view per configured repository under the views root, made, moved and left as herdr.14 has it, and moved when the root changes.
- `running`: `flywheel --version`, `flywheel run` and `--once`, one runner per data directory, its passes and stop, what it says, and no listening socket.

### Modified Capabilities

None.

## Impact

- `crates/flywheel`: a library beside the binary (the configuration, git by its command line, clones, views, a pass, the run loop), `clap`, `serde`, `toml` and `signal-hook` as dependencies, `tempfile` and `libc` for tests, and integration tests under `crates/flywheel/tests/` against a scratch forge on disk.
- `devenv.nix` and `check-toolchain.sh`: git, which the tests now run, is pinned by version and checked (core.8).
- `docs/architecture/core.md`: core.5's detail allows a dropped repository's clone and view, while they hold work the forge lacks, to be named and left.
- `README.md` and `CLAUDE.md`: what the runner does and how a host runs it.
- Outside this repository: swancloud installs `flywheel`, writes its configuration and keeps `flywheel run` alive (crew record 0052 part 4). This change touches none of swancloud's containers or views. The sibling change `the-runner-carries-crews-work-into-flume` builds on the clones and the run loop.
