# Boomdrive

The runner on each host, one binary named `boomdrive`. It keeps the
clones and views of the repositories its configuration names, and turns
the control plane's work into `flume launch` calls, carrying their
results back. It reaches the control plane through one interface,
`crates/boomdrive-control`. crew's implementation, `crates/boomdrive-crew`,
reads and writes records in crew's state repository. A hosted control
plane's, an outgoing WebSocket, is not yet built: its crate,
`crates/boomdrive-hosted`, holds the runner's protocol with it, one JSON
Schema per version under `protocol/`. From this tree:

    devenv shell -- cargo run -p boomdrive -- --version

## The configuration

`boomdrive` reads one TOML file: the one `--config <file>` names, else
`$XDG_CONFIG_HOME/boomdrive/config.toml`, else
`~/.config/boomdrive/config.toml`. It names the repositories the host
keeps by their GitHub address, the root their views are kept under and
how often they are fetched:

```toml
# The root each repository's view is kept under, at <repository_root>/<owner>/<name>; ~/Code where none is named.
repository_root = "~/Code"
# How often every repository is fetched; 2m where none is named.
fetch_interval = "2m"
# Each repository this host keeps, by its GitHub address.
repositories = ["agentplot/flume", "afterthought/crew"]
```

`repositories` is required, so a host that keeps none says
`repositories = []`. Addresses are compared without case, so two that
differ only in case are refused as one repository named twice.
`repository_root` is an absolute path or begins with `~/`.
`fetch_interval` is `<n>s`, `<n>m` or `<n>h`. A key the runner does not
read is refused, so the file has no place for a credential: the host's
own git credential helper answers every fetch. A file written for an
earlier runner is refused with what to write instead: `views` is
`repository_root` now, and `fetch_every` is `fetch_interval`. Nor does
the file name a stream or a kind: those are each host's Flume
configuration, and a key naming one is refused, saying so.

## Running on a host

`boomdrive run` is the long-running process a host starts and keeps
alive. It makes a pass over the configuration at once, then one every
`fetch_interval`, timed from the start of the pass before, and reads the
file again at every pass. A pass that cannot read it names why and
does nothing else, so a file mid-edit never drops a repository. A pass:

- clones each repository the configuration names into the runner's data
  directory, `$XDG_DATA_HOME/boomdrive`, else `~/.local/share/boomdrive`,
  at `repositories/<owner>/<name>/<name>.git`, in the bare-container
  layout flume reads; or fetches the clone it has, over https through
  the host's own credential helper, never prompting;
- keeps the clone's own default branch at origin's while that is a
  fast-forward and no worktree holds it;
- keeps a view of each repository at `<repository_root>/<owner>/<name>`,
  a worktree of the clone detached at `origin/<default>`. It makes the
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
outgoing.

`boomdrive run --once` makes one pass and exits, 0 when every git call
succeeded and 1 when one failed. A repository left by rule, for a
folder of the user's or work the forge lacks, is not a failure. Where
the configuration names a control plane, `--once` makes one control
pass after the keeping pass, and exits 1 too when that pass did not
pull and report. A tool that pass could not run or read is named, and
is not a failure.

Either form refuses to start, exiting 2 and touching nothing, when
`HOME` is unset, the configuration cannot be read, git is missing or
older than 2.48, or another runner holds the data directory: one runner
keeps a data directory at a time.

The host's service manager keeps `boomdrive run` alive: launchd's
`KeepAlive` on a Mac, `Restart=always` in a systemd user unit on a box.
An exit 2 means the host needs fixing, and the service manager's
throttle keeps it from spinning.

## Carrying the control plane's work

With a `[control_plane]` table, the runner carries its control plane's
work into Flume:

```toml
[control_plane]
type = "crew"
poll_interval = "1m"

[control_plane.crew]
host = "deneb"

[[control_plane.crew.partitions]]
label = "madswan"
state_repository = "afterthought/madswan-state"
```

`type` names the implementation, and is required. This binary carries
`crew`. Any other word, `hosted` among them, is named at start with the
word it carries, and no control pass runs, though the repositories are
kept all the same. `poll_interval` is how often a control pass is made,
written as `fetch_interval` is; `1m` where none is named. The table
`type` names is handed whole to the implementation, whose keys are its
own. The table is read when the runner starts, so a change to it takes
effect when the runner starts again.

A control pass runs after the first keeping pass, then each
`poll_interval` after the start of the one before, sooner where a lease
would lapse first, and at once when the control plane wakes the runner.
A control pass:

- reads `flume --json status` once, and hands the control plane the
  host's tools' versions, the repositories kept with their clones made,
  Flume's host part as Flume printed it, and the leases the runner
  holds;
- takes each launch offered under a lease, for a repository the runner
  keeps, once each launch it comes after has landed and while its
  stream has room as Flume's status gives it. Each becomes one
  `flume launch`, naming its kind, stream, priority and intent only as
  the launch gives them, marked `boomdrive:<id>`;
- carries out each retry, drop, priority or done as Flume's own command
  on the launch's place, and each bolt's branch by
  `sluice branch create` in the clone;
- carries back Flume's status of each place, sluice's entry of each
  submission and each inbox the control plane names, each as the tool
  gave it, and never a log; and relays a terminal on a launch's agent
  through herdr, keeping nothing of it.

Each launch, act and command is carried out once, across a restart: the
runner's report is kept at `<data>/control/report.json`, and a launch
whose answer was lost is found by its mark.

## A host from before the rename

The runner was named `flywheel` until record 0016 renamed it. A host
where a runner of that name ran keeps what it made: where
`$XDG_DATA_HOME/boomdrive` is not there and `$XDG_DATA_HOME/flywheel`
is, that is the data directory, and it is never moved, since its clones
hold places and views. A clone recording itself under `flywheel` has
its records carried over to `boomdrive` the first time a pass finds it,
and a launch marked `flywheel:<id>` is found by that mark. The
configuration is the host's own, and is read only where the new name
puts it.

crew's table names `host`, this host as crew's hosts file names it, and
each partition served: its `label`, and its `state_repository` as
`owner/name`, reached over https through the host's own git credential
helper. crew's implementation reads `requests.rec`, `roles.rec`,
`asks.rec` and `plan.rec` at `<label>/main`, keeping its caches under
`<data>/control/crew/`. It answers on the runner's branch,
`<label>/runner/<host>`, in crew's formats: `runner.rec`, `answers.rec`
and `places.rec`. It pushes no other ref, and never with force. A
kit's launch that names no host is claimed on that branch, and taken
once no other runner's branch answers it.

Kinds and streams are Flume's host configuration. The runner passes a
launch's kind to `--kind` as the control plane gave it, a name or an
identity `<source>/<name>@<version>`, and names no kinds file, kind
source or lock. It names a stream only where the launch does. The
host's Flume configuration declares its streams, the stream each
repository's launches run in, and where its kinds come from.

## Rules and checks

The rules every change keeps are `docs/architecture/core.md`; the
decisions behind them are `docs/adr/`. The checks are moon tasks, run
in the devenv shell: `devenv shell -- moon run :#check-fast`. Each crate
is a moon project of its own, `boomdrive`, `boomdrive-control`,
`boomdrive-crew` and `boomdrive-hosted`, beside `root`, which holds what
spans the tree.
