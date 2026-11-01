# Proposal

## Why

The runner on each host turns the control plane's work into `flume launch` calls and carries the results back (record 0001). Today it only keeps the repositories its configuration names (the landed `the-runner-keeps-its-repositories`).

Two things now ask for more:

- **crew**, the stand-in control plane, writes that work. Its `teams-cross-to-flume-as-their-bolts-land` and `roles-run-as-flume-kinds` are built and landed on crew's `bolt/crew-runs-its-work-through-flume`. crew appends a launch, retry, drop or priority of a crossed team's places to `requests.rec` on a partition's flywheel branch. It names in `roles.rec` each main-level role that runs as a Flume kind. It reads each runner's answers, and Flume's status of its places, from the runner's own branch, `<label>/runner/<host>`. Until a runner reads those records and answers them, crew refuses to give work to a crossed team on that host, and no role's run is ever made.
- **The cloud's constitution and domain model**, on agentplot/flywheel-cloud main (records 0001 to 0042, rules core.1 to core.101, `design/domain-model.html`), say what a runner is to a hosted control plane. It reaches its host's Host object over one outgoing socket. It takes a launch under a numbered lease. It takes work only while its host's Flume stream has room. It makes a bolt's branch when the control plane sends that Command. It carries Flume's, sluice's and the inbox's records back as references.

The user settled the shape on 2026-10-31 and 2026-11-01 (record 0002). The runner reaches its control plane through one interface. crew's implementation is built now. The hosted control plane's is named, and the interface is kept able to carry it. The runner talks to Flume, sluice and herdr only through their command lines, and may use the Weir crates for the shared record formats.

## What Changes

- **Record 0002, and the rules it holds**, are written in this change's first commit beside its design:
  - core.1 now covers a tool command beside a launch;
  - core.5 now says the configuration names the repositories and where their views go, and no stream (ledger item 3.b);
  - core.9 to core.11 are new: work only under a lease and while its stream has room, each act once under its lease's number, and only a tool's own answer or a reference carried back.
- **One control-plane interface**, `crates/flywheel-control`. A pull gives what the control plane wants of the host, in its order: launches offered under leases, acts on a launch, commands, inboxes to carry and terminals to relay. The runner hands each pull a hello: its tools' versions, the repositories it keeps, each Flume stream's room and the leases it holds. A report sends the runner's whole record. Ids are the control plane's, and nothing assumes git, files or a socket. The crate holds a fake control plane and the cases every implementation passes.
- **crew's implementation**, `crates/flywheel-crew`, is the one crate that names crew. It reads `requests.rec`, `roles.rec`, `asks.rec` and `plan.rec` at `<label>/main`, and every `<label>/runner/*` branch. It gives each request for its host as an offer or an act, with the lease number 1 and no expiry. It gives a role's run, offered while crew's own reading leaves anything pending for the role, and a run's `done` once its turn has delivered. It writes `runner.rec`, `answers.rec` and `places.rec` on `<label>/runner/<host>` with crew's own descriptors, and pushes no other branch.
- **The hosted control plane is named and not built.** A configuration naming it is refused, naming the one implementation this binary carries.
- **Leased work into Flume** (c-flume.a):
  - An offer is taken only while the runner keeps its repository, the clone is made, each launch it comes after has landed, and its stream has room as `flume status --json` gives it.
  - It becomes one `flume launch`, which names `--stream`, `--priority` and `--intent` exactly where the launch does.
  - A retry, drop, priority or done becomes Flume's own command on the launch's place.
  - Each act is carried out once, across a restart.
  - Each place's status is carried as Flume gave it, under the launch's lease number.
  - A launch whose lease the control plane refuses is let go and left as Flume holds it.
- **sluice's entries and the inbox carried** (c-sluice.a, c-inbox.a). Each submission Flume names is carried from `sluice status <n> --json`: its id, branch, head, state, verdict rows and `landed_as`, never a log. An inbox the control plane names, in a repository the runner keeps, is read whole through weir-signals at each new tip. Its records are carried as the tip holds them, and each item's brief by its digest.
- **A bolt's branch** (record 0017 of the cloud). A branch command runs `sluice branch create <branch> --from <default> --json` in the clone. It reports sluice's answer as given, with the commit origin's branch stands at afterwards, so a branch already made counts as done.
- **A terminal relayed** (N1.b). A terminal the control plane opens on a launch is relayed to the agent's pane that Flume's status names, through herdr's command line, and nothing of it is kept.
- **Configuration**: a `[control_plane]` table, naming the implementation, the pass's interval and the implementation's own table. A key naming a stream is refused.
- **Running**: `flywheel run` makes a control pass beside the keeping pass, on its own timer, and before the earliest lease runs out. `--once` makes one of each.
- weir-branch, weir-capture and weir-signals are pinned at the revision flume pins. recutils is pinned in `devenv.nix`, since weir-branch runs `recfix` on every record file it writes. Fixtures of Flume's own answers sit under the runner's tests.

## Capabilities

### New Capabilities

- `control-plane-interface`: the one interface through which the runner reaches its control plane, what passes over it, the implementation the configuration chooses, and the cases every implementation passes.
- `work-into-flume`: offers taken under leases while the stream has room and turned into `flume launch`; acts on a launch's place; each carried out once; leases held, renewed and lost; Flume's status of each place and the host's capacity carried back.
- `tool-records`: each submission's sluice entry, and each inbox the control plane names, carried back as references.
- `branch-commands`: a bolt's branch made by sluice when the control plane sends its command, an existing branch counting as made.
- `terminal-relay`: a terminal on a launch's agent relayed through herdr's command line, with nothing kept.
- `crew-control-plane`: crew's implementation, reading crew's records on each partition's flywheel branch, offering its role runs as crew reads them, and writing the runner's own branch in crew's formats.

### Modified Capabilities

- `configuration`: the configuration reads a `[control_plane]` table, and names no stream.
- `running`: `flywheel run` and `flywheel run --once` make the control pass beside the keeping pass.

## Impact

- Record 0002; `docs/architecture/core.md`, with core.1 and core.5 amended and core.9 to core.11 added, in this change's first commit.
- New crates `crates/flywheel-control` and `crates/flywheel-crew`, each a moon project of its own in `.moon/workspace.yml` and a member of the cargo workspace.
- `crates/flywheel` gains:
  - Flume's, sluice's and herdr's command lines;
  - the control pass, with leases and the runner's record;
  - the inbox read, the branch command and the terminal relay;
  - the list of implementations, and the loop's control timer;
  - tests of all of it, with fixtures of Flume's answers under `tests/fixtures/`.
- `Cargo.toml` and `Cargo.lock` take serde_json and time from crates.io, and weir-branch, weir-capture and weir-signals from agentplot/weir at `0b6eb5abb806a51fec128b01f97c4dcff6170219`.
- `devenv.nix` and `check-toolchain.sh` gain recutils. `treefmt.nix` excludes the fixtures.
- `crates/flywheel/moon.yml` takes the new crates' sources as inputs.
- `README.md` and `CLAUDE.md` say what the runner now carries.
- Reads what crew's `teams-cross-to-flume-as-their-bolts-land` and `roles-run-as-flume-kinds` write, and writes what they read.
- A hosted control plane's implementation, built later, carries what the interface already holds.

## Touches

```
docs/adr/0002-the-runner-takes-leased-work-while-its-flume-stream-has-room-and-carries-each-tools-record-back-as-given.md
docs/architecture/core.md
Cargo.toml
Cargo.lock
.moon/workspace.yml
devenv.nix
check-toolchain.sh
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
