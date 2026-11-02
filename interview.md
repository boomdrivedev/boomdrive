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
  A partition's repositories are those of the Flywheels it serves
  (topic 5).

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
- **`a-token-only-for-a-served-repository`** (requirement, the
  cloud's and the runner's). The cloud mints a token for a host only
  for a repository of a Flywheel that host serves, so the customer
  controls which hosts work with which repositories by which Flywheels
  each host serves. The runner names a repository it keeps that the
  cloud gives it no token for, and fetches it not. Source: the user,
  2026-11-01, (6), as topic 5's (1) settles it.
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

## 4. Control planes: crew today, the cloud next

### What the material settles

- One interface, `flywheel-control`'s `ControlPlane`: a pull, which
  takes the runner's hello and whole report and gives the control
  plane's wants in its order (offers under leases, acts, commands,
  inboxes) and the terminals to open; and a report, which takes the
  whole report and gives what is let go and what is refused. Ids are
  the control plane's and never read into. A second report of one
  report changes nothing. Every implementation passes the shared cases
  (the control-plane-interface spec; core.2).
- One `[control_plane]` table, its `type` the implementation's word and
  `poll_interval` its timer; the table `type` names is handed whole to
  the implementation (the configuration spec; the user's ruling).
- Leases: every report names each lease's number and so renews it; a
  report refused for a lease no longer the runner's lets the work go,
  acts on it no more, reports it no more and removes nothing. A control
  pass runs before the earliest lease would lapse, and at once when the
  implementation wakes the runner (core.9, core.10; the running and
  work-into-flume specs).
- crew gives every launch lease number 1 with no expiry, wakes no one,
  and a kit's launch naming no host is claimed on the runner's branch
  (the crew spec).
- The hosted implementation keeps one outgoing WebSocket ending at the
  host's Host object; every message names a protocol version, checked
  against that version's schema, and the cloud accepts the current
  version and the one before; every offer, lease, renewal, command and
  event has an id, and a second delivery changes nothing; a lease not
  renewed ends at its expiry by the object's alarm and a result under
  it is refused; a hello at every reconnect carries the runner's full
  state, and the cloud never asks again on a schedule (cloud core.36 to
  core.39, core.52; records 0022, 0026; design Decision 20).
- Commands built: a bolt's branch by `sluice branch create`; any other
  sort is refused, naming its tool (the branch-commands spec). The
  domain model also lists `boom accept` and its siblings, millpond's
  moves, submitting a bolt to the default branch, cloning a repository,
  and the controls cancel, drain, upgrade and setup, none of them
  defined.

### Asked

1. **crew's future.** One `[control_plane]` with one `type` means a
   runner serves one control plane at a time. Once the hosted control
   plane serves the operator's own work, what becomes of crew's
   implementation?
2. **Commands.** Which does the runner carry: tools' commands plus
   drain; also upgrade and setup; also clone?

### The user's words

1. "Removed once the cloud serves you" (the recommended answer).
2. "Also upgrade and setup".

### What it gives

- **`crew-until-the-cloud`** (requirement). crew's implementation stays,
  held to the shared cases, until the operator's own partitions run on
  the hosted control plane, and one change then removes it. Until
  then, a change to crew's records or its readings (`asks.py`) is a
  chore of this repository's. Source: the user, 2026-11-01, (1).
- **`one-control-plane-per-runner`** (rule, derived, for review). A
  runner serves the one control plane its `[control_plane]` `type`
  names, and every partition it serves is that control plane's. Rules
  out: two implementations opened by one runner; a partition naming a
  `type` of its own. Source: the user's ruling on the configuration's
  names; (1).
- **`a-command-is-a-tools-or-the-runners-own`** (rule). Every command
  the runner carries runs one tool's own command line once, under its
  lease, its answer carried as the tool gave it; or it is one of the
  runner's own: drain (take no new offer, carry on with what it holds),
  resume, upgrade (install a release of itself) and setup (configure
  the host's Flume). No command adds a repository for the host to keep
  (core.5). Rules out: a command the runner carries by a library call
  into a tool; a clone made on a command; a command of the runner's
  own beyond these four, but by a decision recorded with it. Source:
  the user, 2026-11-01, (2).
- **`drain-takes-nothing-new`** (requirement). A drained runner takes
  no new offer and claims nothing, and carries on with every lease it
  holds, its acts, commands and reports among them, until resumed; a
  drain survives a restart. Source: (2), the option's own words.
- Upgrade is taken up with installing (topic 8), and setup below.

### Asked, on setup

3. **How setup works.** By Flume's own commands, or by the runner
   writing Flume's files?
4. **Setup's reach.** Kind sources only; kind sources and streams; or
   anything in Flume's host file? The question said streams stay with
   "whoever keeps the host".
5. **What setup covers**, once the user had said a host is set up by
   the New Host Wizard: the runner's configuration; Flume's streams;
   Flume's kind sources; installing the tools?
6. **Hosts swancloud's nix keeps.** Setup refuses what nix keeps;
   swancloud stops keeping it; or swancloud drives setup?

### The user's words

3. "By Flume's own commands" (the recommended answer).
4. "Why would you recommend and say something like, "Whoever keeps the
   host"? If I understand correctly (and we've already talked about
   this, or maybe it was a different agent), you would be adding the
   start of the host via the New Host Wizard through the cloud setup.
   You'd get one quick install command to run and then, in theory, the
   host would get configured either through a wizard in the TUI or
   through setup from the cloud that gets pushed down"
5. All four, and: "I assume the list of repositories is going to be
   interesting from an ordering perspective because Boom also points at
   repositories. A flywheel instance points at repositories but a host
   could serve multiple flywheel instances. I imagine a host gets its
   repository list inferred from the flywheels it's servicing, not from
   some kind of initial setup. I think we could probably have a similar
   concept for kinds. You probably would assign the kinds to the
   flywheel and those would have to get pushed down to any host"
6. "If the tools support multiple levels of config, and the Flywheel
   Cloud manages one level while we can set defaults at a different
   level with nix, then any configs managed by the Flywheel Cloud would
   stop being managed by nix. That would be desirable"

The pack agrees (agentplot/strategy,
`commercial/river-driver/flywheel-cloud/collaboration-ui-and-agents.md`,
first run 3 and 4): "Connect first host — the one manual step: single
install command with short-lived enrollment token (or Nix flake
module)", then a rail seeded with starter items, "clone repos, install
Flume and Sluice, check toolchain, run a first small task", each
approved and done by the host; "Adding a second host later is just
another rail item."

### What it gives

- **`setup-by-the-tools-own-commands`** (rule). Setup configures a
  host only through each tool's own command line, the runner's own
  included, and never by writing a tool's files: Flume's streams and
  kind sources by Flume's commands. Rules out: the runner writing a
  file under Flume's or sluice's configuration directory; a setup step
  with no tool command behind it. Source: the user, 2026-11-01, (3),
  (5). Asks of Flume: commands that declare and remove a stream and a
  kind source.
- **`a-host-is-set-up-by-the-wizard`** (requirement). A host is added
  by one install command carrying a short-lived enrolment token
  (`enrol-by-command`); after it, the host is configured either by a
  wizard in the runner's terminal interface on the host or by setup the
  control plane sends down, and both run the same tool commands. Setup
  covers the runner's own configuration, Flume's streams and kind
  sources, and installing the tools at the versions the runner is
  checked against. Source: the user, 2026-11-01, (4), (5); the pack's
  first run.
- **`a-flywheels-repositories-are-the-hosts`** (requirement). Under a
  hosted control plane, the repositories a host keeps are inferred
  from the Flywheels it serves, and not set at its initial setup; the
  cloud writes them into the runner's managed level of configuration,
  so the configuration still names every repository kept (core.5,
  topic 5). Source: the user, 2026-11-01, (5).
- **`a-flywheels-kinds-reach-its-hosts`** (requirement). Kinds are
  assigned to a Flywheel, and setup pushes the Flywheel's kind sources
  down to every host that serves it, by Flume's own commands. Source:
  the user, 2026-11-01, (5).
- **`a-managed-level-of-configuration`** (requirement, and an ask of
  each tool). The runner's configuration, and each tool's, has a level
  the control plane manages, apart from the host's own defaults (nix's
  or hand-written); what the managed level holds wins, and nix stops
  writing what the cloud manages. Rules out: setup refused on a host
  because nix keeps its defaults; nix and the cloud writing one file.
  Source: the user, 2026-11-01, (6). Asks of Flume and sluice: a
  managed level of their own (Flume's layers are on their way:
  `an-agents-program-settings-come-from-flumes-layers`).
- **open: `boom-and-the-order-of-repositories`.** Boom also points at
  repositories, and a host may serve several Flywheels; in what order
  a host takes up a Flywheel's repositories, and whether Boom's are
  among them. Source: the user, 2026-11-01, (5).

## 5. Several hosts and partitions

### What the material settles

- Partitions stay in the runner's configuration (the user's ruling);
  each is one enrolment with its own key, connection and Host object,
  and what goes over it names only its own (topic 3).
- The runner never chooses where work runs: the control plane offers
  each lease, and the runner takes or leaves only what it is offered,
  while its stream has room (core.1, core.9). In the cloud, a Flywheel
  balances work across hosts by Flume's points (cloud record 0013), and
  a host is in exactly one Account (cloud core.14).
- Any host that keeps a bolt's repository may take its branch command;
  it waits while none can (cloud core.19).
- Under crew, a request naming a host is that host's; a kit's launch
  naming none is claimed on the runner's own branch and given once no
  other runner's branch answers it; two claims both withdraw and claim
  again in the name order of the hosts (the crew spec). crew's leases
  never expire, so a claim never moves: a runner that claims and then
  sleeps or stops holds the launch (design, *Risks*).
- crew's partitions today: madswan and swancloud on
  `afterthought/crew-state`, main level on deneb; wldn on
  `WilldanGroup/crew-state`, main level on chuck-herdr-alpha (crew's
  plans; the crew and swancloud notes).

### Asked

1. **Repositories.** Under the cloud a host's repositories are inferred
   from the Flywheels it serves, and (from topic 3) the customer maps
   repositories to hosts, while core.5 and record 0002 say the
   configuration names them. How is core.5 amended?
2. **Stale claims.** A crew claim never moves. Should a stale one
   lapse, crew's implementation being temporary?

### The user's words

1. "Both can be true at the same time and you have to be careful with
   your words because you said it is inferred from the flywheels it
   serves. You then said that the customer maps repositories to hosts.
   Maybe you got that from something I said but the former supersedes
   the latter. The idea is that the host repositories are inferred from
   the flywheel and it's just the difference. You have to think about
   it as open source tools that could be configured by hand or a
   simplified cloud management interface that handles the
   configuration for you"
2. "No, crew is temporary" (the recommended answer).

### What it gives

- **`configured-by-hand-or-managed`** (rule). The runner, as each open
  tool, is configured by its own configuration alone, which a person
  can write by hand; a hosted control plane is a management interface
  that writes the same configuration's managed level for the user
  (`a-managed-level-of-configuration`), and the runner does nothing
  under the cloud that a configuration written by hand could not ask
  of it. Rules out: a setting the runner takes only from a control
  plane's message and that no key of its configuration holds; a
  behavior that turns on which control plane wrote a key. Not ruled
  out: the work a control plane offers, which is no configuration.
  Source: the user, 2026-11-01, (1).
- **`a-hosts-repositories-follow-its-flywheels`** (requirement). Under
  the hosted control plane, the repositories a host keeps are those of
  the Flywheels it serves: setup writes them into the runner's managed
  level, and a repository leaves it when no Flywheel the host serves
  holds it, then going by the let-go rules as they stand. Under crew,
  the configuration names them by hand. There is no separate mapping
  of repositories to hosts. Source: the user, 2026-11-01, (1).
- **core.5 stands, its details amended.** "The configuration names the
  repositories" holds, the managed level being part of it. What its
  details rule out narrows to a clone made because work named a
  repository the configuration does not name, and a repository added
  because work named it; what they allow gains the managed level
  written by setup from the Flywheels the host serves. Record 0002's
  first point, and the cloud's ledger item 3.b, read the same. Source:
  the user, 2026-11-01, (1); cloud record 0007.
- **`a-claim-never-moves`** (requirement, kept as built). A crew claim
  stands until its runner answers it; a stranded one shows in crew's
  reading of the runner's branch and is cleared by hand, until the
  cloud's expiring leases replace crew. Source: the user, 2026-11-01,
  (2).

## 6. Failure and offline behaviour

### What the material settles

- A git call that fails is named with git's words and never stops the
  runner; every other repository is still kept, and the failed one is
  tried again at the next pass. A clone that fails leaves nothing
  (the clones and running specs).
- A configuration that cannot be read keeps, removes and acts on
  nothing until it reads again; it is never taken to name no
  repository (the configuration and running specs).
- A pull that cannot be read acts on nothing that pass; a report that
  cannot be sent is kept and sent at the next pass; the repositories
  are kept all the same. A tool that cannot be run or read is named,
  and the work waiting on it is tried again (the control-plane and
  work-into-flume specs).
- A start that finds the host broken (no `HOME`, no configuration, git
  missing or older than 2.48, the data directory held) exits 2 having
  touched nothing, and the service manager's throttle keeps it from
  spinning (the running spec).
- A lease lost is let go: the runner acts on it no more, reports it no
  more and removes nothing, leaving the place as Flume holds it and
  naming it (core.10). In the cloud, a lease not renewed ends at its
  expiry and its work returns to the board (cloud core.39), and the
  Host object tells a Flywheel when its host goes quiet (cloud core.13).
- Known as built: a lapsed lease makes control passes run back to back
  while the control plane cannot be reached (the verify report's
  observations).

### Asked

1. **A lost lease's live place.** A host offline past a lease's expiry
   comes back to a refused report, while its place still runs and may
   land through sluice, and another host may have launched the same
   work. Left, the cloud sending a drop; the runner holding its
   submission; or the runner dropping it?
2. **Rules drawn from the material**, each kept or not: no launch past
   its lease; backoff, never back to back; no act it cannot record;
   presence is the cloud's.

### The user's words

1. "Left; the cloud sends a drop" (the recommended answer).
2. All four kept.

### What it gives

- **`a-stray-place-is-dropped-by-command`** (requirement). A place the
  runner let go stays as Flume holds it, named, until the control plane
  sends a command, under a lease of its own, that drops it by its
  address; the runner carries that command as Flume's `drop`. core.10
  stands: the lost lease's work is not acted on, and the command is
  new work. Its details gain, as not ruled out, a command under a
  lease of its own that drops a place the runner let go. Source: the
  user, 2026-11-01, (1). Asks of the cloud: the command, sent when a
  lapsed lease's work goes to another host.
- **`no-launch-past-its-lease`** (rule). An offer whose lease has
  expired by the runner's clock is not launched, acted on or claimed
  until a pull gives it again under a live lease. Rules out: an offer
  launched from a saved report while the control plane cannot be
  reached and its expiry has passed. Source: the user, 2026-11-01, (2);
  core.9.
- **`backoff-never-back-to-back`** (rule). A control plane that cannot
  be reached is tried again on a bounded backoff with jitter, from
  seconds up to `poll_interval`, and never pass after pass; a lease
  already lapsed sets no pass due at once. Rules out: the back-to-back
  passes the verify report names (`src/carry.rs::next_due` falling to
  the end of the pass); a reconnect loop with no wait. Source: the
  user, 2026-11-01, (2).
- **`no-act-it-cannot-record`** (rule). A pass whose report cannot be
  saved carries out no further act or command that pass, so nothing is
  done that a restart would do again. Rules out: a second act after
  the report's write failed. Source: the user, 2026-11-01, (2); core.10.
- **`presence-is-the-clouds`** (rule). The runner raises no alarm of its
  own about being offline: it says one line when its control plane
  becomes unreachable and one when it returns, and the Host object tells
  the Flywheel that its host is quiet. Rules out: a notification, a
  mail or an inbox item the runner sends about its own reach; a line
  per failed try. Source: the user, 2026-11-01, (2); cloud core.13.

## 7. The command line, its output and exit codes

### What the material settles

- `flywheel --version` and `-V` print `flywheel <version>` and exit 0;
  no command, or one it does not know, prints the usage on standard
  error and exits 2 (the running spec).
- `flywheel run [--config <file>] [--once]`: `run` keeps going until
  SIGTERM or SIGINT and exits 0; `--once` exits 0 when every git call
  succeeded and the control pass pulled and reported, else 1; either
  exits 2 having touched nothing when the host is broken (the running
  spec).
- The runner writes one line on standard error for each thing it does
  or names, prefixed `<owner>/<name>:`, `<id>:` or `flywheel:`, and a
  quiet pass writes nothing; it says when it starts, with its version,
  configuration file, data directory and repository root, and when it
  stops (the running and work-into-flume specs).
- From this interview: `flywheel enroll`, reading the one-time token on
  standard input (topic 3); `flywheel forge-token`, a credential
  command answering on standard output (topic 3); a setup wizard in the
  runner's terminal interface (topic 4).
- The family's tools: Flume prints one JSON form that only ever gains
  fields, a refusal as `{"refused":{"refusal":{"reason":…}}}`, exits 0
  answered, 1 refused, 2 usage; sluice prints `{"error":{"kind",
  "message","next"}}`, exits 2 refused, 1 not found, 3 internal, 64
  usage (the flume and sluice notes).

### Asked

1. **Local commands**, beside `run`, `enroll`, `forge-token` and the
   setup wizard: status; drain and resume; check; upgrade? A local
   command reaches a running runner only through a file in its data
   directory, read at its next pass, since the runner opens no socket.
2. **The output contract**: Flume's; sluice's; or words only?
3. **The running runner's lines**: words with a time; JSON lines; or
   as now?

### The user's words

1. All four, and: "I've been wondering if there should be a Claude Code
   status line concept as well"
2. "Flume's contract" (the recommended answer).
3. "Words with a time" (the recommended answer).

### What it gives

- **`the-runners-commands`** (requirement). Beside `run`, the runner
  has `enroll`, `forge-token`, `setup` (its wizard), `status`, `drain`,
  `resume`, `check` and `upgrade`. `status [--json]` is read-only and
  needs no lock: the repositories kept and their clones, the leases
  held, what it launched and where, whether it is drained, and its last
  pass's time and outcome. `check` reads the configuration, the tools
  and their versions, the reach of origin and of the control plane and
  the enrolment, changes nothing, and exits 0 or names what is wrong.
  `drain`, `resume` and `upgrade` are the control plane's commands of
  the same names, given on the host. Source: the user, 2026-11-01, (1).
- **`a-local-command-reaches-the-runner-by-file`** (rule). A command
  given on the host reaches a running runner only through a file in its
  data directory, which the runner reads at its next pass; no command
  opens or connects to a socket of the runner's (core.3). Rules out: a
  Unix socket, a named pipe served by the runner, a signal other than
  SIGTERM and SIGINT carrying meaning. Not ruled out: the runner woken
  sooner by the file's change, read by polling or a file watch. Source:
  core.3; (1).
- **`a-status-line`** (requirement, its form to design). The runner
  gives a one-line status for Claude Code's status line on the host,
  `flywheel status --line` say: short, read-only, from the data
  directory alone, with no network call, fast enough for a status line
  run at every prompt. Source: the user, 2026-11-01, (1).
- **`flumes-output-contract`** (rule). Every command of the runner's
  that answers takes `--json`, and its JSON within a version only ever
  gains fields; a refusal is `{"refused":{"refusal":{"reason":…}}}` on
  standard output; it exits 0 answered, 1 refused, and 2 for a usage
  error or a host that must be fixed. Rules out: a field renamed or
  removed within a version; an exit code of another meaning; a refusal
  only in words when `--json` was asked. Not ruled out: `run`'s lines,
  which stay words on standard error. Source: the user, 2026-11-01,
  (2); Flume's contract as the model.
- **`a-line-carries-its-time`** (rule). Each line the running runner
  writes starts with its time, RFC 3339 in UTC to the second, then its
  subject, `<owner>/<name>:`, `<id>:` or `flywheel:`, and its words.
  Rules out: a line with no time; a time in local time. Source: the
  user, 2026-11-01, (3).

## 8. Installing and upgrading

### What the material settles

- swancloud installs the runner on the operator's hosts, writes its
  configuration, and keeps `flywheel run` alive under the host's
  service manager: launchd's `KeepAlive` on a Mac, `Restart=always` in
  a systemd user unit on a box; exit 2 means the host needs fixing
  (README; crew record 0052 part 4; the first change's design 10).
  Nothing of the runner is on swancloud's main yet, and this
  repository has no tags or releases (the swancloud notes).
- A customer's host is added by one install command carrying a
  short-lived enrolment token, or a Nix flake module (the pack's first
  run; topic 3); setup installs the tools at the versions the runner is
  checked against, and upgrade is a command of the runner's own, from
  the control plane or on the host (topics 4 and 7); the cloud accepts
  the protocol's current version and the one before (cloud core.36);
  a release rolls back one release with the data directory as it
  stands (topic 2).
- The console never upgrades a runner itself: an upgrade is a rail item
  a member answers (cloud core.96).
- Flume ships release archives; sluice ships by nix and release
  binaries. Agents' hooks call Flume by its absolute path, so a
  Flume upgrade keeps the old path while its agents live (the flume
  notes).
- On deneb, `~/.config/flywheel/` already holds an older `flywheel`
  tool's App keys and env file, under the same binary name (the crew
  and swancloud notes).

### Asked

1. **Releases**, the repository being taken as private: signed from the
   cloud's domain; private GitHub releases; or public binaries only?
2. **Installing**, for the people who will use it, not the operator's
   hosts: one `curl` install for everything; one per tool; or package
   managers?
3. **Tool versions** outside what a release was checked against: named,
   with a floor; exact pins; or named only?

### The user's words

1. "We've already agreed that the runner needed to be open source. I
   was told it had to be if people were going to use it, and that's
   what other systems like GitHub Actions do: they have open-source
   runners. I do think we need signed releases, though, since that's
   what we're trying to sell with Timber and Sluice"
2. "Think about the fucking 10,000 people we want to use this, not me.
   [...] How would they do it? We could try to be in package managers
   but I don't know that. It sounds like it's going to be tough to start
   managing fucking multiple operating systems and installing all the
   different packages. [...] Would it be easier to just do `curl`
   installs? I'm really not sure how happy I am that you pushed me to
   have all of the tools as individual installs. It just seems like a
   lot to fucking manage"; then, of the recommendations put again:
   "Your recommendations are fine. We have to assume people might
   already have Git and a multiplexer installed"
3. "Your recommendations are fine" (named, with a floor).

### What it gives

- **`the-runner-is-open-source`** (requirement, superseding record
  0001's "private for now"). The runner is open source, as GitHub
  Actions' runners are, so that people can run it on their own hosts.
  Source: the user, 2026-11-01, (1); the awesome-lists row S8, taken
  up below.
- **`releases-are-signed`** (rule). Every release of the runner, and
  of each tool its install brings, carries a provenance attestation of
  the release workflow that built it, and an install or an `upgrade`
  runs a release only once it has verified that provenance against that
  workflow's identity; a failed check keeps the running binary. Rules
  out: a release published with no attestation; an installer or
  `upgrade` that runs what it fetched before the check; a check against
  a key of our own in place of the workflow's identity. Source: the
  user, 2026-11-01, (1); the awesome-lists row F9 (self_update checking
  GitHub's artifact attestations with sigstore-verify), kept by the
  user, 2026-11-01.
- **`one-install-for-the-host`** (requirement). One `curl` install puts
  a host's whole set in place: the runner, and the Flume and sluice
  releases it was checked against, each verified, installed together
  under the user's home, with the service unit written (launchd on a
  Mac, a systemd user unit on Linux) and `flywheel enroll` run. An
  upgrade is the same again: fetched, verified, installed beside the
  old, switched, and restarted by the service manager, places running
  on in Flume. git and the multiplexer are the host's own, found on it
  and never installed. Source: the user, 2026-11-01, (2).
- **`a-floor-for-each-tool`** (rule). Each release names the least
  version of each tool it runs with, git and the multiplexer among
  them, and the versions it was checked against; below a floor the
  runner takes no work that needs the tool, saying so, and keeps its
  repositories; above what it was checked against, the version is named
  in the hello and by `check`, and work goes on. Rules out: work taken
  with a tool below its floor; a tool's version judged by anything but
  its own `--version`. Source: the user, 2026-11-01, (3).
- **F51 kept** (a design note for the change that builds the install).
  The one install is built by dist (cargo-dist): the release workflow
  builds each platform's binaries and publishes the shell installer
  beside them, attestations included; the launchd agent and the systemd
  user unit are two templates of our own, which the install fills in.
  Source: the awesome-lists row F51, kept by the user, 2026-11-01.
- **S8 kept** (the user's words in (1)). The repository is made public:
  a record says so, superseding record 0001's "private for now", and
  core.6 already holds nothing commercial. It asks of the cloud that
  the runner protocol's schema, which its record 0046 keeps in the
  private `packages/runner-protocol`, be somewhere a public runner can
  build against (the disposition's note 4; the cloud's D5 re-read).
  Source: the awesome-lists row S8; the user, 2026-11-01, (1).
