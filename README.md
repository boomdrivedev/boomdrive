# flywheel-runner

The runner on each host, one binary named `flywheel`. It keeps the
clones and views of the repositories its configuration names, and turns
the control plane's work into `flume launch` calls, carrying their
results back. It reaches the control plane through one interface, with
two implementations: crew's, which reads and writes records in crew's
state repository, and a hosted control plane's, an outgoing WebSocket.

Today it keeps the repositories; the control plane's work is still to
come. From this tree:

    devenv shell -- cargo run -p flywheel -- --version

## The configuration

`flywheel` reads one TOML file: the one `--config <file>` names, else
`$XDG_CONFIG_HOME/flywheel/config.toml`, else
`~/.config/flywheel/config.toml`. It names the repositories the host
keeps by their GitHub address, where their views are kept and how often
they are fetched:

```toml
# Where each repository's view is kept, at <views>/<owner>/<name>; ~/Code where none is named.
views = "~/Code"
# How often every repository is fetched; 2m where none is named.
fetch_every = "2m"
# Each repository this host keeps, by its GitHub address.
repositories = ["agentplot/flume", "afterthought/crew"]
```

`repositories` is required, so a host that keeps none says
`repositories = []`. Addresses are compared without case, so two that
differ only in case are refused as one repository named twice. `views`
is an absolute path or begins with `~/`. `fetch_every` is `<n>s`, `<n>m` or `<n>h`. A key the runner does
not read is refused, so the file has no place for a credential: the
host's own git credential helper answers every fetch.

## Running on a host

`flywheel run` is the long-running process a host starts and keeps
alive. It makes a pass over the configuration at once, then one every
`fetch_every`, timed from the start of the pass before, and reads the
file again at every pass. A pass that cannot read it names why and
does nothing else, so a file mid-edit never drops a repository. A pass:

- clones each repository the configuration names into the runner's data
  directory, `$XDG_DATA_HOME/flywheel`, else `~/.local/share/flywheel`,
  at `repositories/<owner>/<name>/<name>.git`, in the bare-container
  layout flume reads; or fetches the clone it has, over https through
  the host's own credential helper, never prompting;
- keeps the clone's own default branch at origin's while that is a
  fast-forward and no worktree holds it;
- keeps a view of each repository at `<views>/<owner>/<name>`, a
  worktree of the clone detached at `origin/<default>`. It makes the
  view where the folder is missing and never over a folder that is
  there, and moves it only while it stands where the runner last put
  it and holds no uncommitted change, so a view you moved or work in is
  left as it is;
- lets go of each repository the configuration no longer names. It is
  never fetched again, and its clone and view are removed once neither
  holds work the forge lacks; until then they are left, and named at
  every pass.

The runner says each thing it does or names as one line on standard
error, `<owner>/<name>: <words>`. A pass in which nothing changed says
nothing. Told of a landing, it keeps that one repository at once. It
stops on SIGTERM or SIGINT before the next repository it would keep,
and exits 0. It opens no listening socket: every connection it makes is
git's, outgoing.

`flywheel run --once` makes one pass and exits, 0 when every git call
succeeded and 1 when one failed. A repository left by rule, for a
folder of the user's or work the forge lacks, is not a failure.

Either form refuses to start, exiting 2 and touching nothing, when
`HOME` is unset, the configuration cannot be read, git is missing or
older than 2.48, or another runner holds the data directory: one runner
keeps a data directory at a time.

The host's service manager keeps `flywheel run` alive: launchd's
`KeepAlive` on a Mac, `Restart=always` in a systemd user unit on a box.
An exit 2 means the host needs fixing, and the service manager's
throttle keeps it from spinning.

## Rules and checks

The rules every change keeps are `docs/architecture/core.md`; the
decisions behind them are `docs/adr/`. The checks are moon tasks, run
in the devenv shell: `devenv shell -- moon run :#check-fast`.
