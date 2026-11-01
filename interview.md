# The runner's constitution interview

The interview of 2026-11-01 with the user, to capture every requirement
the flywheel runner must meet and the falsifiable rules a change to it
must hold to. Each topic says what the material already settles, what
was asked, the user's words, and the requirement or rule each answer
gives. What nobody has answered yet is marked **open**.

An item's id here is a name, fixed when the item is written. A rule
takes its `core.<n>` number only when it is written into
`docs/architecture/core.md` at the end, the next free number on main as
main then stands, and a record its number likewise in `docs/adr/`. So a
landing on main while this branch is open renumbers nothing here.

Each item is one of:

- **requirement**: what the runner must do or never do, for a spec;
- **rule**: what a change to the runner must hold to, for `core.md`;
- **open**: a question nobody has answered yet.

## Sources

- This repository at `dc000d9`: `docs/architecture/core.md` (core.1 to
  core.12), records 0001 and 0002, `README.md`, the ten specs under
  `openspec/specs/`, the two archived changes
  (`the-runner-keeps-its-repositories`,
  `the-runner-carries-crews-work-into-flume`) with their designs and
  verify reports, and the code in `crates/`.
- agentplot/flywheel-cloud at `24baa4e`: records 0001 to 0042, rules
  core.1 to core.101, `design/domain-model.html` and
  `design/architecture.html`; and how its constitution interview ran.
- Flume's and sluice's READMEs, records and rules, and their statements
  in agentplot/blueprints `design/flume` and `design/sluice`; flume's
  change `a-host-declares-where-its-kinds-come-from`.
- afterthought/crew at `9d590e9f` (#417, "Crew runs its work through
  flume, without teams"), its record 0052 and `plugin/lib/runner.py`.
- swancloud, which installs and keeps the runner on each host.
- The rulings the user gave before the interview, kept and not
  reopened (below).

## Rulings kept from before

- The configuration is `~/.config/flywheel/config.toml`, with names
  that say what a thing is: `repository_root`, `fetch_interval`,
  `control_plane.poll_interval`, `control_plane.type` (core.12).
- Partitions stay in the runner's configuration.
- Kinds are Flume's concern (flume's change
  `a-host-declares-where-its-kinds-come-from`), fitting a cloud
  marketplace of kinds that configures each host's Flume (core.5).
- A bolt's branch is made by a runner when its rail card is approved;
  any host that keeps the repository may take it; it waits while no
  host is online; a branch that exists counts as done (cloud record
  0017, core.19).
- Streams are defined on hosts (cloud record 0003).

## 1. What the runner owns, and what it never does

### What the material settles

- The runner decides nothing the control plane or Flume decides. It
  starts work only as a `flume launch` or a tool command the control
  plane asked for, and carries each tool's answer back as the tool gave
  it (core.1; record 0002).
- It keeps a clone and a view only of a repository its configuration
  names, and its configuration names no stream or kind (core.5).
- Flume owns a place's whole life: its stages, the room it starts in,
  the account it takes, its submission through sluice, sending it back
  after a failed submission, and its tidying. The host's Flume
  configuration owns streams, bounds, priorities, pools and kind
  sources (flume's statement clauses 1, 70, 143 to 147; flume core.26,
  core.34; flume record 0020). Under Flume, Flume submits to sluice, so
  the runner is no sluice driver, though sluice's statement still lists
  "flywheel" among its drivers.
- sluice owns order, landing, blame and trust, and makes a branch only
  when asked (sluice clauses 104 to 106; sluice record 0026).
- The cloud never talks to a tool but through the runner, and asks
  nothing of a tool the tool does not do on its own (cloud core.47).

### Asked

1. **Terminals.** The runner relays a person's keys into an agent's
   pane through herdr's command line (cloud record 0016), while Flume's
   clause 107 says Flume opens, sends to and closes every agent's
   terminal and no caller does. Options put: a relayed hand counts as a
   person; screen out and answers in; or the relay through a command of
   Flume's.
2. **Tool daemons.** Does the runner start or keep `sluice run`,
   `flume settle` or herdr's server?
3. **The clone.** Flume makes places in the runner's clone container,
   and crew makes `places/land-<bolt>` there and submits landings from
   it. What is the runner's claim on its clone?

### The user's words

1. "a combo of #1 and #3 might make the most sense. Remember that Herdr
   should be a seam for multiplexers. We will want to support TuiOS
   shortly"
2. "None, host's service manager" (the recommended answer).
3. "Host's one clone, shared" (the recommended answer).

### What it gives

- **`terminal-through-flumes-seam`** (rule). The runner reaches an
  agent's terminal only through Flume's command line, which reaches the
  host's multiplexer through Flume's own terminal seam (herdr now, TuiOS
  next); no code of the runner's names a multiplexer or runs one's
  command line. Rules out: `herdr pane read`, `send-text` or
  `send-keys` run by the runner; `HERDR_SOCKET_PATH` or a pane address
  parsed by the runner; a multiplexer's version in the hello read by
  running it. Source: the user, 2026-11-01, (1). This stands against
  the terminal-relay spec as built, which runs herdr's command line.
- **`a-relayed-hand-is-a-persons`** (requirement). A terminal the runner
  relays carries only a person's screen out and that person's keys in;
  the runner sends nothing of its own into it, and keeps nothing of it
  (core.11). Flume counts that person as a person at the keyboard, not
  as a caller. Source: the user, 2026-11-01, (1). An ask of Flume:
  clause 107 says so, and Flume gives a command that relays a place's
  terminal through its seam, the screen out and keys in.
- **`no-tool-daemon`** (rule). The runner starts, keeps and stops no
  tool's long-running process and runs no tool's housekeeping of its
  own accord: sluice's driver (`sluice run`), the multiplexer's server
  and any `flume settle` are the host's service manager's, as the
  runner itself is. Rules out: a `sluice run`, `flume settle` or
  multiplexer server spawned, watched or restarted by the runner; a
  tool's process started because no control plane asked. Source: the
  user, 2026-11-01, (2).
- **`the-hosts-one-clone`** (rule). The runner's clone of a repository
  is the host's one clone of it: Flume, sluice and what a launch runs
  may work in its container, and the runner alone fetches it and moves
  its default branch and its view, and never removes, moves or rewrites
  a ref, worktree or file another tool made there. Rules out: a second
  clone of a kept repository made for a tool; the runner pruning a
  place, a sluice ref or a worktree it did not make; another tool
  fetching on the runner's timer. Not ruled out: a dropped repository's
  clone held, and named, by a worktree another tool made (the clones
  spec). Source: the user, 2026-11-01, (3). It asks of crew that its
  main-level checkouts come from the runner's clone, not swancloud's
  spaces, and of swancloud that its own containers retire.

## 2. The state directory, and what survives a restart

### What the material settles

- The data directory is `$XDG_DATA_HOME/flywheel`, else
  `~/.local/share/flywheel` (the clones spec). It holds `lock`; each
  clone at `repositories/<owner>/<name>/`, in the bare-container layout
  Flume reads, known as the runner's by the address it records in the
  clone (`flywheel.address`); `control/report.json`, the runner's
  report; `control/briefs/`, a launch's brief while Flume reads it; and
  each implementation's caches, crew's at `control/crew/` (the running,
  clones and work-into-flume specs; design Decisions 11 and 17).
- One runner holds a data directory at a time, by an exclusive lock;
  a second exits 2 and touches nothing (the running spec).
- Each launch, act and command is carried out once across a restart:
  the report is written whole, by rename, after every act and command
  and before the next, and read at start. A launch whose answer was
  lost is found by its mark `flywheel:<id>` in Flume's listing. A
  report file lost with the data directory is taken from the control
  plane, which holds every answer reported. The one window left is an
  act made after the last report sent and lost with the data directory
  before the next (core.10; design Decision 11).
- The configuration file is read at every keeping pass, and the
  control-plane table at start (the configuration spec).

### Asked

1. **The relay meanwhile** (a follow-up to topic 1's first answer).
   Today the runner's relay runs herdr directly, and only the unbuilt
   hosted control plane opens terminals; crew opens none. What happens
   to it until Flume relays one?
2. **Upgrades.** Across an upgrade or a rollback, what must hold of the
   runner's own files: `report.json`, the address recorded in each
   clone, the commit each view was last put at?

### The user's words

1. "Remove it, build on Flume" (the recommended answer).
2. "Roll back one release" (the recommended answer).

### What it gives

- **`no-relay-until-flumes`** (requirement). The runner's herdr relay
  comes out. A terminal the control plane opens is closed at once with
  words saying the host's Flume relays none, until a change builds the
  relay over Flume's command (`terminal-through-flumes-seam`). Source:
  the user, 2026-11-01, (1). Roadmap: a change that removes the relay
  and its spec's herdr requirements.
- **`roll-back-one-release`** (rule). A release of the runner never
  drops, renames or changes the meaning of a field that the release
  before it reads in a file it keeps (its report, a clone's or a view's
  record), so a host goes back one release with its data directory as
  it stands; a file the runner cannot read makes it refuse to start,
  exit 2, and never is discarded or overwritten. Rules out: a field of
  `report.json` renamed or removed in one release; a record moved out
  of a clone's git configuration with no reading of the old place; a
  report the runner cannot parse replaced by an empty one. Not ruled
  out: a field added, which the release before ignores; a field the
  release before no longer reads dropped a release later. Source: the
  user, 2026-11-01, (2); the cloud's core.67 as the model.
- **`the-data-directory-holds`** (rule, derived from the material, for
  the user's review). The data directory holds only the lock, the
  clones, the runner's report, a launch's brief while Flume reads it,
  and each implementation's caches; everything in it but the report is
  made again from origin, Flume and the control plane, and the report
  is taken from the control plane where it is lost. Rules out: a log
  file, a terminal's bytes, a credential or a copy of a tool's own
  records kept there; a brief left after Flume answered (the verify
  report's "a brief left by a killed launch" is a fix: a start removes
  the briefs no launch holds). Source: design Decisions 11 and 17;
  core.11.
