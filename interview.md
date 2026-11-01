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
