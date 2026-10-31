# Proposal

## Why

The runner on each host is there to turn the control plane's work into
`flume launch` calls and carry their results back (record 0001), and so
far it prints its version. crew, the stand-in control plane, is now
writing that work. `teams-cross-to-flume-as-their-bolts-land` has crew
append each launch, retry with words, drop and priority of a crossed
team's places to `requests.rec` on a partition's flywheel branch, and
read each runner's answers and Flume's status of its places from the
runner's own branch, `<label>/runner/<host>` of the partition's state
repository. `roles-run-as-flume-kinds` has crew name in `roles.rec`
each main-level role that runs as a Flume kind, and asks the runner to
keep one run of it while anything waits on it (its R1 to R5). Until a
runner on a host reads those records and answers them, crew refuses a
give to a crossed team there, and no role's run is ever made.

The user settled the shape on 2026-10-31. The runner reaches its
control plane through one interface: crew's implementation now, and a
hosted control plane's, one outgoing WebSocket, later, not built yet
but kept possible. It talks to Flume only through Flume's command line,
never its crates, and it may use the Weir crates for the shared record
formats.

## What Changes

- **One control-plane interface**, `crates/flywheel-control`. A pull
  gives what the control plane wants of the host: its requests (a
  launch, or a retry, drop or priority of one) and the runs it wants
  kept. A report sends the runner's record: each request's answer, each
  place it launched with Flume's status of it, and each run's place.
  Keys are the control plane's. Nothing in the interface assumes git,
  files or a socket. Its crate holds the cases every implementation
  passes.
- **crew's implementation**, `crates/flywheel-crew`, the one crate that
  names crew. For each partition its configuration names, it reads
  `requests.rec`, `roles.rec`, `asks.rec` and `plan.rec` at
  `<label>/main`'s tip, and every `<label>/runner/*` branch for what
  waits on a role. It writes `runner.rec`, `answers.rec` and
  `places.rec` on `<label>/runner/<host>` by weir-branch's write, and
  pushes no other branch. Its formats match crew's field for field.
- **The hosted control plane is named and not built.** A configuration
  naming it is refused, naming the one implementation this binary
  carries.
- **Requests into Flume.** A launch becomes `flume launch` in the
  runner's clone of its repository, always naming `--stream`,
  `--priority` and `--intent`. A retry, drop or priority becomes
  `flume retry`, `flume drop` or `flume priority` on the place the
  launch made. A launch whose `After` names other launches is held
  until each of their places has landed. Each request is acted on once,
  across a restart too. Work for a repository the configuration does
  not name is refused, with the runner saying so. Each answer is
  Flume's, as Flume gave it.
- **Flume's status carried.** One `flume status --json` per pass, and
  each place the runner launched carried as Flume printed it, changed
  only when Flume's text changes.
- **Role runs kept.** At most one run of each role crew names, launched
  in its clone while crew's records leave anything pending for the
  role, declared done once its turn has delivered, and recorded by the
  role's address.
- **Configuration.** A `[control_plane]` table in the runner's
  configuration file, which `the-runner-keeps-its-repositories` makes:
  the implementation's word, the pass's interval, and crew's host,
  partitions and kinds files.
- recutils pinned in `devenv.nix`, since weir-branch runs `recfix` on
  every record file it writes. The weir crates are pinned at the
  revision flume pins. Fixtures of flume 0.3.6's answers sit under the
  runner's tests.

## Capabilities

### New Capabilities

- `control-plane-interface`: the one interface through which the
  runner reaches its control plane, its implementations chosen by the
  configuration, and the cases every implementation passes.
- `work-into-flume`: each request turned into its `flume` command in
  the runner's clone, acted on once, and each answer and place's status
  carried back as Flume gave it.
- `role-runs`: the runs the control plane wants kept on the host, a
  crew role's among them, at most one of each, launched while wanted
  and declared done once delivered.
- `crew-control-plane`: crew's implementation, reading crew's records
  on each partition's flywheel branch and writing the runner's own
  branch in crew's formats.

### Modified Capabilities

None.

## Impact

- New crates `crates/flywheel-control` and `crates/flywheel-crew`, each
  a moon project of its own in `.moon/workspace.yml`, and members of
  the cargo workspace.
- `crates/flywheel`: the pass, Flume's command line, role runs, the
  wiring that opens the control plane its configuration names, and the
  tests of all of it, with flume 0.3.6 fixtures under `tests/fixtures/`.
- `Cargo.toml` and `Cargo.lock`: serde, serde_json, toml and time from
  crates.io; weir-branch and weir-capture from agentplot/weir at
  `0b6eb5abb806a51fec128b01f97c4dcff6170219`.
- `devenv.nix` gains recutils; `treefmt.nix` excludes the fixtures.
- `crates/flywheel/moon.yml` takes the new crates' sources as inputs.
- `README.md` and `CLAUDE.md` say what the runner now does.
- Depends on `the-runner-keeps-its-repositories` (the configuration
  file, the data directory, the long-running process and its timer,
  the clones). Reads what crew's `teams-cross-to-flume-as-their-bolts-land`
  and `roles-run-as-flume-kinds` write, and writes what they read.

## Touches

```
Cargo.toml
Cargo.lock
.moon/workspace.yml
devenv.nix
treefmt.nix
crates/flywheel-control/**
crates/flywheel-crew/**
crates/flywheel/Cargo.toml
crates/flywheel/moon.yml
crates/flywheel/src/**
crates/flywheel/tests/**
README.md
CLAUDE.md
```
