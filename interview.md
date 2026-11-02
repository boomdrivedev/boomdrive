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

## 3. Enrolment, identity and credentials on a host

### What the material settles

- No token, key or credential is in this repository, and the runner
  reads each from the host when it runs (core.4). The configuration
  refuses any key it does not read, `token` among them, and never
  echoes a value (the configuration spec).
- Every fetch and push reaches the forge over https through the host's
  own git credential helper, with `GIT_TERMINAL_PROMPT=0`; a host whose
  git rewrites https to ssh is named and not fetched (the clones spec).
  crew's state repositories are reached the same way (the crew spec).
- On the cloud's side: a host is enrolled by a token issued from the
  console for the install command a person runs, and the new host's
  first connection raises its rail item; the console never enrols a
  host or upgrades its runner itself (cloud record 0040, core.96). The
  runner acts as a principal of its own, issued by the control plane,
  never with a member's credential; a host's token is minted at its
  enrolment (cloud core.24, record 0018), and one module of the Worker
  verifies it (core.21). A host's signing identity, and every
  certificate issued on it, is an event in the chain (cloud core.43;
  record 0023: a host signs "with a certificate for its own
  principal"). Each host belongs to exactly one Account and has one
  Host object (cloud core.13, core.14). The control plane keeps no
  customer credential: a forge token is minted short-lived at its use,
  or stays on the host (cloud core.58).
- The hosted implementation, as named and unbuilt, reads `url` and a
  `token_file` from `[control_plane.hosted]`, never the token (design
  Decision 20).

### Asked

1. **Host identity.** A bearer token at rest, a key pair made on the
   host, or the OS keychain?
2. **Enrolment.** `flywheel enroll`, a token the installer places, or
   enrolment left to swancloud?
3. **Two accounts on one machine** (madswan's and Willdan's work, say):
   one runner with an enrolment per partition, a runner per account, or
   one account per machine?
4. **Forge credentials** under the cloud: the host's own credential
   helper only, or a token the cloud mints for one command?

### The user's words

1. "Key pair made on the host" (the recommended answer).
2. "`flywheel enroll`" (the recommended answer).
3. "One runner, one enrolment each" (the recommended answer).
4. "The thing is, though, I'm running a Sluice GitHub app right now to
   push and pull from GitHub. We need to reconcile with what I'm
   currently doing" (taken up below).

### What it gives

- **`host-key-never-leaves`** (rule). A host's identity with a hosted
  control plane is a key pair the runner makes on the host at
  enrolment; the private key never leaves the host, in a message, a
  log, a report or a file outside the runner's own, and every
  connection proves the key is held, the control plane certifying it
  for short times. Rules out: a bearer token the runner keeps and
  presents as the host's lasting identity; the private key sent at
  enrolment or read from the configuration; a key printed or logged.
  Not ruled out: a short-lived certificate or session the control
  plane issues on the key; the key made in a hardware key store where
  the host has one. Source: the user, 2026-11-01, (1); cloud record
  0023 and core.43.
- **`enrol-by-command`** (requirement). `flywheel enroll` reads the
  one-time enrolment token from standard input, never from an argument
  or the environment, makes the host's key pair, registers its public
  key over the runner's own outgoing connection, and writes the key and
  what the control plane certified under the runner's own directory,
  in files only the host's user may read; it never writes the
  configuration. Source: the user, 2026-11-01, (2); cloud record 0040.
- **`one-runner-one-enrolment-per-partition`** (requirement). One
  runner serves a machine, with one data directory and one clone per
  repository. Each partition its configuration names is one enrolment,
  with its own key, its own connection and its own Host object. Source:
  the user, 2026-11-01, (3).
- **`a-partition-sees-only-its-own`** (rule). What the runner sends
  over one partition's connection names only that partition's
  repositories, leases, places and commands: no repository, stream
  name, place or answer of another partition's reaches it. Rules out:
  one hello listing every repository the host keeps sent to every
  partition; a place launched for one partition carried in another's
  report. Not ruled out: the host's room as Flume gives it, which every
  partition reads. Source: the user, 2026-11-01, (3); cloud core.33.
  It asks that a partition say which repositories are its own (open,
  topic 5).

### Asked, on forge credentials

5. **The runner's own forge identity.** sluice mints installation
   tokens from the App key the host's `[forge]` names and gives them
   only to its own git calls (sluice record 0019). The runner's own
   forge calls are its fetches and crew's runner branch, through the
   host's helper today. Options put: an App of the runner's own as
   sluice has; sluice's App with narrowed tokens; the host's helper,
   with every guarded write a tool's.
6. **The cloud holds the App and sends short tokens down.** Which
   mitigations of the cloud's reach into a customer's repositories are
   rules: tokens narrowed per call and kept in memory; every mint an
   event on the chain; the cloud never using one; the customer picking
   the repositories?
7. **sluice's token under the cloud**, with no App key on the host: a
   credential command; a token in each call's environment; or sluice
   keeping its own App?

### The user's words

5. "None of this feels right. Do we really want to be distributing
   GitHub app keys and IDs to everybody's hosts in a sort of
   self-hosted setup? I forget exactly why I switched loose to use the
   GitHub app, but maybe it was for unattended running. It seems like
   we need a cleaner design and maybe the cloud should have the app and
   send short tokens down. You seem concerned that it lets the cloud
   have access to the repos and I don't know if there are any
   mitigating strategies for that"
6. "The cloud never uses one, Customer picks repos, Maybe the customer
   also has to map permissions from repos to hosts so they can have
   control over which hosts can work with which repos"
7. "A credential command" (the recommended answer).

### What it gives

- **`no-app-key-on-a-host`** (rule). Under a hosted control plane, no
  GitHub App key or App id is on a host for the runner or a tool to
  mint with: the cloud holds the App, and a host's forge calls are made
  with short-lived tokens the cloud mints and sends down the runner's
  own connection. Rules out: an App's private key file named in the
  runner's configuration or installed with it; the runner minting an
  installation token. Not ruled out: crew's hosts, which have no cloud,
  keeping sluice's App and the host's own credential helper as today;
  a token never written to a file or a log (core.4). Source: the user,
  2026-11-01, (5) and (6).
- **`the-cloud-never-uses-a-token`** (rule for the cloud, its ask). No
  cloud code makes a forge call with an installation token it minted:
  a token goes only into a message to a runner. Source: the user,
  2026-11-01, (6); with cloud core.3 and core.4.
- **`the-customer-picks-the-repositories`** (rule for the cloud, its
  ask). The App is installed on the repositories the customer chooses,
  and uninstalling it ends the cloud's reach. Source: the user,
  2026-11-01, (6).
- **`hosts-are-granted-repositories`** (requirement, the cloud's and
  the runner's). The customer maps repositories to hosts, and the cloud
  mints a token for a host only for a repository mapped to it. The
  runner names a repository its configuration keeps that the cloud
  gives it no token for, and fetches it not. Source: the user,
  2026-11-01, (6). How the mapping meets the configuration's own list
  is taken up in topic 5.
- **`a-tool-asks-the-runner-for-a-token`** (requirement). A tool on a
  cloud host that needs a forge identity of its own, sluice to land
  among them, asks for it by a credential command, `flywheel
  forge-token`, which reaches the cloud over its own outgoing
  connection with the host's key and gives the token on standard
  output only; it opens no socket and keeps nothing. Source: the user,
  2026-11-01, (7). An ask of sluice: an identity that runs a command
  for its token, as git runs a credential helper, naming no control
  plane (cloud core.47).
- Not taken: tokens narrowed per call, and every mint an event on the
  chain; neither was chosen, and neither is a rule (core.4 still rules
  out a token written or logged).
- **open: `a-places-forge-identity`.** How an agent in a place on a
  cloud host reaches the forge, for a branch it pushes or a `gh` call
  it makes, with no credential of the host user's. Flume's and the
  cloud's, the runner's only if a token comes through it.
