# Design

## Context

See proposal.md for why. This change is the runner's half of two exchanges.

- **crew's.** crew's `teams-cross-to-flume-as-their-bolts-land` specifies its half, in its Decision 4 and its *Dependencies*, "The runner". crew's `roles-run-as-flume-kinds` asks five things of the runner (its *What this change rests on*, 2, R1 to R5). crew's `the-team-machinery-leaves`, in its code stage, asks three more (its *What this change rests on*, 3).
- **The cloud's.** agentplot/flywheel-cloud specifies the hosted control plane's half, in its records and rules and its domain model. Its ledger names five items as the runner's: 3.b, c-flume.a, c-sluice.a, c-inbox.a and N1.b.

Record 0002 holds what the user settled on 2026-11-01, and core.1, core.5 and core.9 to core.11 hold it. The user ruled the same day that kinds are Flume's concern, not the runner's or a control plane's (core.5), and that every key and type names what it holds (core.12).

What stands:

- **This repository** is at `3e61c31`, where `the-runner-keeps-its-repositories` landed.
  - `flywheel run` reads the configuration file (`config::Config`: `views`, `fetch_every`, `repositories`, with unknown keys refused) again at every pass.
  - It keeps a bare clone of each repository it names, in Flume's layout, at `<data>/repositories/<owner>/<name>/`. `clone::container` gives that path, and `clone::find` gives the clones that exist, by `flywheel.address`.
  - It keeps each repository's view under the root `views` names, and lets a dropped repository go once nothing in it would be lost.
  - It runs one loop on one thread. `Event::Landed` keeps one repository at once (`pass::keep_one`), and `Notices` hands out the sender. `Event::Stop` stops the loop.
  - Its tests run under a scratch home, git configuration and forge (`tests/common`).
- **The cloud**, agentplot/flywheel-cloud, read 2026-11-01: its records 0002, 0003, 0005, 0007, 0008, 0009, 0013, 0016, 0017, 0022 and 0025, its rules core.3, core.4, core.13, core.15, core.18, core.19, core.28, core.36 to core.39, core.47 and core.52, and its `design/domain-model.html`. What this change takes from each is said where it is cited below.
- **crew's side**, built and landed on crew's `bolt/crew-runs-its-work-through-flume` at `9913f249` (`plugin/lib/runner.py`, `plugin/lib/asks.py`), with `the-team-machinery-leaves` on `change/the-team-machinery-leaves` at `2b97ee57`:
  - **`requests.rec`** on `<label>/main` holds the record set `Request`, numbered from 1 and only ever appended to.
    - Every request has `Request`, `Act` (`launch`, `retry`, `drop`, `priority`), `Work`, `Date` and `By`.
    - A launch has `Repo`, `Kind`, `Name`, `Base`, `Priority`, `Intent`, `Recipient` and `Brief`, and may have `Hold: submission` and repeated `After`.
    - A retry has `Of`, and may have `Stage`, `Submission` and `Words`. A drop has `Of`. A priority has `Of` and `Priority`.
    - At `9913f249` every request names its `Host`, and every launch its `Stream`. From `the-team-machinery-leaves` (its Decision 4), a launch of a role's work names its `Host` and `Stream`, a launch of a kit's work names neither, and a retry, drop or priority names no `Host`. A kit's `Repo` is its forge address or, where crew's plan names the kit so, its name alone. A runner whose configuration keeps the repository takes a kit's launch, under a lease, and the runner that took a launch takes its acts.
  - **The runner's branch**, `<label>/runner/<host>`, holds `runner.rec` (`Runner`), `answers.rec` (`Answer`) and `places.rec` (`Place`, then `Run`). crew reads them by `host_read`, with the descriptors `RUNNER`, `ANSWERS`, `PLACES` and `RUNS`, and never writes the branch. `the-team-machinery-leaves` changes none of the four.
    - A place's `Status` is `flume status --json <place>`'s whole answer, on one line, as Flume gave it. crew reads its one place under `status.places` (`flume_facts`, `asks.status_of`), and from `the-team-machinery-leaves` its `repository` and `stream` too.
    - An `Answer` is `held` while a launch waits, then `launched`, `done` or `refused`. A launch with no answer reads to crew as launching.
    - crew reads the runner that took a request as the one whose `answers.rec` answers it (`runner.taken`, from `the-team-machinery-leaves`). Two runners answering one request read as unknown, naming both.
  - **`roles.rec`** holds `Role` records: `Role`, `Address`, `Run`, `Host`, `Repository`, `Kinds`, `Kind`, `Name`, `Stream`, `Priority`, `Intent`, `Model`, `Effort`. `Kinds` is `crew` and `Kind` is `crew-<role>` (`asks.role_record`). **`asks.rec`** holds `Ask` and `Turn` records.
  - A role is wanted while `asks.pending` leaves anything: an ask, or a place of any host's `places.rec` waiting on the role (`waits_on`), that no ended turn took.
  - **`plan.rec`** holds `Bolt` and `Unit` records.
- **The tools on the host**, read 2026-11-01:
  - **Flume 0.4.1.**
    - `flume launch --kind --name [--repository] [--base] [--brief-file] [--kinds] [--hold <stage>=<words>] [--hold-submission] [--recipient] [--mark] [--priority] [--agent <field>=<value>] [--stream] [--intent]`.
    - `retry <place> [--stage] [--words] [--submission]`, `drop`, `done` and `priority <place> <n>`.
    - `status [<place>]`. Every command takes `--json`. A refusal exits 1 with `{"refused":{"refusal":{"reason":…}}}`.
    - `status --json` gives `status.places` and `status.host`. The host part is `running`, `bound`, `room`, `queue` and `streams`, each stream being `{name, priority, running, bound, room, queue}`, beside `pools`, `addresses`, `bases` and `terminal_kept`.
    - Each place gives `repository`, `stream` as `{name, from}`, and `clues.shown_at.agent` as `herdr:<pane>@<socket>`.
    - A place's stream is the launch's, else its intent's, else the host configuration's for its repository and kind, its repository, or its kind, else `default` (flume record 0020). No command says which stream a launch naming none would take.
  - **Flume's kinds.** Flume's record 0022, accepted 2026-10-29, names a kind `<source>/<name>@<version>`. A launch names its kind as `change`, `change@1` or `flume/change@1`. A name with no source resolves among the launch's declaration, the repository's and the one other source holding it. A launch with no version takes the version a lock records, else the newest. flume's change `a-host-declares-where-its-kinds-come-from`, not yet built, lets a host's Flume configuration declare the sources its kinds come from, so a launch can name a kind of any declared source by its identity.
  - **sluice 0.61.1.**
    - `sluice branch create <branch> --from <from> --json` answers `{"branch","created","from"}`. Otherwise it answers `{"error":{"kind","message","next"}}`: `already there` among its refusals, and a branch the policy at `<from>` gives no gate among them.
    - `sluice status <n> --json` gives one submission's record: `id`, `branch`, `head`, `state`, `failed` (each row's `task`, `hash`, `log`, `flaky`, `system`), `landed_as` and more.
  - **herdr 0.9.5.** `herdr pane read <pane> --source visible --format ansi`, `pane send-text` and `pane send-keys`, pointed at a session's socket by `HERDR_SOCKET_PATH`, as Flume points it.
  - **millpond** is designed and not built. An inbox is weir's: `inbox/` at the root of a tip of a shared branch, read whole from that tip by `weir_signals::InboxAt` (weir's statement 40 to 43, C.2).
- **weir** at `0b6eb5abb806a51fec128b01f97c4dcff6170219`, the revision flume pins, gives these crates:
  - **weir-branch**: `read`, a branch's files at one tip through a bare cache, and `write`, a write formed against the tip read, every `.rec` checked by `recfix --check`, one commit, and the branch moved only from that tip by a leased push, never forced;
  - **weir-capture**: `rec::read` and `rec::write`;
  - **weir-signals**: the inbox's layout and records.

There are no area pages, so `docs/architecture/core.md` binds every path this change touches.

## Goals / Non-Goals

**Goals:**

- The runner reaches its control plane through one interface that carries the cloud's runner contract (cloud record 0022): presence and capacity; launches offered under numbered leases; acts on a launch; commands; what the runner carries back; terminals. crew's implementation is the only code naming crew. A hosted control plane's can take its place (core.2).
- Every offer is taken only under its lease, while the runner keeps its repository and its Flume stream has room. It becomes one `flume launch` that names a stream only where the launch does (core.9; cloud record 0003). Each act and command is carried out once (core.10).
- A launch's kind reaches Flume as the control plane gave it, a name or an identity. The runner names no kinds file, kind source or lock, so a host's kinds can come to its Flume from wherever its Flume configuration says, a marketplace among them (core.5).
- Every configuration key and type the runner has names what it holds (core.12), the landed keys among them.
- Flume's status of each place, sluice's entry of each submission, and each inbox the control plane names are carried back as the tools gave them, as references (core.11; c-flume.a, c-sluice.a, c-inbox.a).
- A bolt's branch is made when its command comes, and a branch already there counts as made (cloud record 0017).
- A terminal on a launch's agent is relayed with nothing kept (N1.b).
- crew's requests are answered and its role runs kept, in crew's formats, field for field (crew record 0052; R1 to R4). A kit's launch that names no host is taken by one runner of those that keep its repository.

**Non-Goals:**

- **The hosted control plane's implementation**, named in Decision 20. Until it is built, the inbox, sluice's entries, the branch command and terminals reach no control plane: crew's records have no place for them. They are built and proved against the interface's fake.
- **Points** (cloud record 0013, ledger item 10.b). Flume's launch takes none yet. The change that adds `--points` adds the field.
- **A role's `Effort`** (R5). Flume carries no effort setting.
- **Commands other than a bolt's branch**: `boom accept`, millpond's moves, a marketplace's configuring of a host's kinds, and cancel, drain and upgrade (the Command card). Each is a change of its own. A command of a sort this binary does not carry is refused, naming it.
- **Roles as host capabilities** (the pack's #12, PROPOSED). The runner reports its tools' versions and the repositories it keeps, and nothing called a role.
- **A held host's role runs**: crew's open question (`roles-run-as-flume-kinds`, *Open Questions*).
- **Who runs a repository's `sluice run`**, and a bolt's landing onto its default branch. Both are left to the plan.

## What this change rests on

1. **`the-runner-keeps-its-repositories`, landed.**
   - It gives the configuration and its reader, whose keys this change names anew (Decision 3) and to which it adds a table (Decision 4).
   - It gives the data directory, where this change keeps the runner's report (Decision 11) and crew's caches (Decision 17).
   - It gives the loop and its events, which gain a control timer and `Event::Wake` (Decision 5).
   - `clone::find` and `clone::container` give the directory `flume launch --repository` names.
   - `pass::keep_one` and `Notices::landed` fetch a repository at once.
   - It gives the scratch host of `tests/common`.
2. **The cloud's records and rules above**, which the user has accepted. Where the hosted plane's messages and schema are left open (cloud record 0022: where the schema lives), this change fixes nothing of them.
3. **crew's halves**: `9913f249` by the fields, descriptors and readings named above, and `the-team-machinery-leaves` at `2b97ee57` for a launch and an act that name no host. The runner reads both forms, so it serves crew before and after that change lands.
4. **flume's `a-host-declares-where-its-kinds-come-from`, not yet built.** A role's run is launched by its kind's identity, `crew/crew-<role>` (Decision 10), which Flume resolves only once a host's Flume configuration can declare the source `crew`. Until that change is on a host's Flume, Flume refuses a role's run, and the refusal is the run's `Words`. Every other launch of crew's names its kind by name alone (`change`, `chore`, `fix`), which Flume 0.4.1 resolves today. The gate does not wait on it: the tests run a stub `flume`.
5. **On the host**: Flume 0.4.1 or later, and the Flume release carrying 4 for roles' runs; sluice; herdr; and recutils for weir-branch's `recfix` (*After deploy*).

## Decisions

### 1. Three crates, and only crew's names crew

- **`crates/flywheel-control`**, a library: the interface, its types, a fake control plane, and the cases every implementation passes. It depends on no implementation.
- **`crates/flywheel-crew`**, a library: crew's implementation. It is the one crate that names crew, its branches or its records. It depends on `flywheel-control`, weir-branch and weir-capture.
- **`crates/flywheel`**, the binary and its library. It gains these modules:

  | module | what it holds |
  |---|---|
  | `flume` | Flume's command line: the `Flume` trait and `Cli` (Decisions 7 and 9) |
  | `facts` | the facts the runner reads from Flume's status, each guarded (Decisions 6, 8, 12, 13, 16) |
  | `hello` | the hello: tools' versions, repositories kept, the host's capacity (Decision 6) |
  | `report` | the runner's report file (Decision 11) |
  | `carry` | the control pass (Decisions 5 to 13) |
  | `sluice` | sluice's command line: an entry's status and a branch's making (Decisions 13 and 15) |
  | `inbox` | an inbox the control plane names, read through weir (Decision 14) |
  | `terminal` | the relay through herdr's command line (Decision 16) |
  | `planes` | the list of implementations: a word against the function that opens it (Decision 4) |

core.2 rules out "a function of the runner's own with a branch for one implementation". `planes.rs` is data, a word against an opener, and the one place outside an implementation that names one. Every other module takes a `&mut dyn ControlPlane`. A test holds the line (Task notes 9.2). Each crate is a moon project of its own, one task per check (core.7).

Alternative: one crate with a module per implementation. Not taken: the compiler does not check a module's boundary, and it does check a crate's dependencies.

### 2. The interface: a pull, a report, and what passes between them

`flywheel-control` gives one trait, `ControlPlane`, with two calls:

- **`pull(hello, report)`** gives what the control plane wants of the host now, or `Unread` with words saying why. The runner hands every pull its hello and its whole report. A pull is the hosted plane's hello at every reconnect (cloud core.52), and an implementation reads from the report what it has already been told.
- **`report(report)`** takes the runner's whole report and gives `Reported`, or `Unsent` with words saying why. A report renews every lease it names (cloud record 0009). A second report of the same report changes nothing (cloud core.38).

Each exchange of the runner contract (the cloud's records 0005, 0008, 0009, 0016 and 0017, and the ledger's c-inbox items) has its part of the interface: the `Hello`, given to every pull; `Want::Offer`, `Want::Act`, `Want::Command` and `Want::Inbox`; `Pulled::terminals`; and the `Report`, sent whole, which carries Flume's status of each place, an agent's question among it. The runner raises nothing of its own.

The types, each named for what it holds (core.12):

| type | fields |
|---|---|
| `Hello` | `interface_version`, this interface's version, 1; `runner_version`, `flywheel --version`'s text; `tool_versions`, each tool's name and its `--version` line as it printed it; `repositories`, the addresses the runner keeps with their clones made; `capacity`, Flume's host part (Decision 6), or why it could not be read; `leases`, each id with the number held |
| `Pulled` | `wants`, in the control plane's order; `terminals`; `report`, the runner's report as the control plane last held it, or none |
| `Want` | `Offer(Offer)`, `Act(Act)`, `Command(Command)` or `Inbox(Inbox)` |
| `Offer` | `id`, the launch's `Id`; `lease`; `launch` |
| `Lease` | `number`; `expires`, a time or none, where the control plane never moves the work |
| `Launch` | `repository`, an address; `kind`, a name or an identity `<source>/<name>@<version>` (Decision 10); `name`; `base`; `brief`; `stream`, `priority` and `intent`, each optional; `recipients`; `holds`, each a stage and its words; `hold_submission`; `after`, launch ids; `agent_settings`, each a field and its value |
| `Act` | `id`; `launch`, its launch's `Id`; `lease`, its number; `what`: `Retry { stage, words, submission }`, `Drop`, `Priority(n)` or `Done` |
| `Command` | `id`; `lease`; `what`: `Branch { repository, branch, from }`, or `Other { tool, words }` for a sort this binary does not carry |
| `Inbox` | `id`; `repository`; `branch` |
| `Terminal` | `id`; `launch`; `lease`; `link`, the implementation's two-way channel: `input(wait)` gives `Text`, `Keys` or `Close`, and `screen(text)` sends a screen and says whether the far end still listens |
| `Report` | `launches`, `acts`, `commands` and `inboxes`, each by id |
| `Launched` | `lease`; `answer`; `status` and `entry`, each a `Carried` or none |
| `Answer` | `outcome`, one of `Held`, `Launched`, `Done` or `Refused`; `place_dir`; `words`; `at` |
| `Carried` | `text`, the tool's answer as it printed it; `at`, when the runner first read it as it now stands; `listed`, whether the tool's last listing held it |
| `Acted` | `launch`; `lease`; `outcome`, `Done` or `Refused`; `words`; `at` |
| `Commanded` | `lease`; `outcome`, `Done` or `Refused`; `text`, the tool's answer as given; `words`; `origin_commit`, the commit the branch stands at on origin, or none; `at` |
| `InboxCarried` | `tip`; `files`, each path under `inbox/` with its text or digest (Decision 14); `words`; `at` |
| `Reported` | `let_go`, the ids the control plane carries no more; `refused`, each id whose report was refused because its lease is no longer the runner's, with the control plane's words |
| `Wake` | what an implementation's opener is handed, to call when its control plane pushes work: the runner then makes a control pass at once (Decision 5) |

An `Id` is a string the control plane gives, unique among what it wants of the host. The runner never reads into it. crew's ids are given in Decision 17. Times are RFC 3339 in UTC to the second. `kind` and `stream` hold Flume's kind and stream as Flume means them, which core.12 allows.

**Why this shape carries a hosted control plane.** Every value is plain data, with no handle into git, no path into a state repository and no socket, apart from a terminal's link, which is a channel. crew's implementation answers a pull by reading branches, and a report by writing one. A WebSocket implementation would answer a pull from the messages its socket had received, and a report by sending what the server has not acknowledged, each message under its id. The runner's own code is the same for both.

Alternatives:

- A call per message pushed into the runner. Not taken: a pull keeps every connection outgoing (core.3). It also lets crew's records be read whole at one tip, and needs no queue in the runner.
- A "kept run" want, the control plane asking for one run standing while wanted. Not taken: the cloud models a role's run as a launch (its domain model), each with its own lease. So crew's implementation gives each run as an offer (Decision 19).

### 3. The configuration's keys say what they hold

The landed configuration's keys take names that say what they hold (core.12). The configuration reads:

```toml
repositories = ["example/crew", "agentplot/flume"]
repository_root = "~/Code"
fetch_interval = "2m"
```

- **`repository_root`** is the directory each kept repository's view is made under, at `<repository_root>/<owner>/<name>`. Its value, default and refusals are the landed `views`'s.
- **`fetch_interval`** is how often every kept repository is fetched. Its form, default and refusals are the landed `fetch_every`'s.
- **`repositories`** stands as it is.

`views` and `fetch_every` are refused as keys the runner does not read, as every unknown key is. The refusal adds the key that holds each now, so a host's file written for the landed runner says what to write. The configuration file is the one place either name changes: the data directory, the clones and the views stand where they are.

The configuration names no stream and no kind (core.5). A key naming either is refused as one the runner does not read, and the refusal adds that streams, or kinds, are Flume's host configuration: `stream`, `streams`, `kind`, `kinds` and `kinds_file`.

Type names follow the same rule. `Config`'s fields are `repository_root` and `fetch_interval`. The runner's start line names its repository root.

### 4. The control-plane table

The configuration may hold one table more:

```toml
[control_plane]
type = "crew"
poll_interval = "1m"

[control_plane.crew]
host = "host-a"

[[control_plane.crew.partitions]]
label = "home"
state_repository = "example/home-state"
```

- **`type`** is the implementation's word, and is required.
- **`poll_interval`** is the interval between control passes, written as `fetch_interval` is (`<n>s`, `<n>m`, `<n>h`). It is `1m` where none is named.
- **The table named by `type`** is handed whole to that implementation's opener, so its keys are the implementation's own (core.2).

Without `[control_plane]` the runner keeps its repositories and carries no work, as it does today.

A `type` this binary does not carry is refused at start, naming the ones it carries, and no control pass runs. `hosted` is among them: it is named and not built (Decision 20). The keeping pass goes on, since clones and views do not depend on the control plane. The table is read at start, and a change to it takes effect when the runner starts again. The file is still read whole at every keeping pass, so a table broken while the runner runs is named there. A control pass that follows a read that failed acts on nothing, as the keeping pass keeps nothing, until the file reads again.

crew's table:

| key | |
|---|---|
| `host` | this host as crew's hosts file names it: the `Host` of the requests it answers, and of the role records whose runs it keeps. It is required, with no default: the machine's own hostname is not crew's name for it. |
| `partitions` | each partition served: its `label`, as crew writes it, and its `state_repository`, as `owner/name`. At least one, and each label once. |

A missing or malformed key is refused, naming the key and the file. Any other key is refused as one crew's implementation does not read. A `kinds` key is refused with the words that kinds are Flume's host configuration (core.5). A state repository is reached at `https://github.com/<owner>/<name>.git` through the host's git credential helper, by the sibling's git builder (core.4). The configuration names no credential.

### 5. The control pass, and its place in the loop

The loop stays one thread, so no two of the runner's git calls touch one clone at once. It gains a control timer and one event.

- **When.** A control pass runs at start, after the first keeping pass. After that it runs `poll_interval` after the start of the one before. It runs sooner, by the midpoint between the end of the pass before and the earliest expiry of a lease the runner holds, so a lease is renewed before it lapses. It also runs at once on `Event::Wake`, which an implementation whose control plane pushes sends by calling the `Wake` its opener is handed. crew's never does.
- **`flywheel run --once`** makes one keeping pass, then one control pass.

One control pass:

1. **The report.** It takes the report file (Decision 11). Where the file is missing, it takes the report the first pull gives back. Where neither exists, it starts empty.
2. **The host.** It reads `flume --json status` once, the listing of every place and the host part. It reads each tool's version, and the repositories kept with their clones made (`clone::find` against the configuration). From these it forms the hello (Decision 6).
3. **The pull.** An `Unread` ends the pass, its words logged, having acted on nothing. Terminals already open keep relaying.
4. **Each want, in the control plane's order**: an offer (Decisions 7 and 8), an act (Decision 9), a command (Decision 15) or an inbox (Decision 14). The report is saved after each act or command, before the next.
5. **Carrying.** It carries each launch's status (Decision 12), each entry (Decision 13) and each inbox (Decision 14). The first pass that reads a launch's place landed sends `Notices::landed` for its repository.
6. **Terminals.** Each terminal pulled is opened (Decision 16).
7. **The report sent.** It reports the whole report, drops what `let_go` names, lets go what `refused` names (Decision 11), and saves. An `Unsent` keeps the report as it is, to be sent at the next pass.

A pass acts on no place it did not launch. The runner says each act on standard error, one line each, prefixed with the launch's or the command's id, as the keeping pass prefixes a repository's address. A pass that changes nothing writes nothing.

### 6. The hello: what the Host object copies

At every control pass the runner reads, and hands to the pull:

- **`tool_versions`**: the first line of `flume --version`, `sluice --version`, `herdr --version` and `git --version` as each printed it. A tool not found reads as missing, with its words.
- **`repositories`**: each address the configuration names whose clone is made (core.5). The control plane registers none of them by this (cloud record 0007).
- **`capacity`**: the text of Flume's `status.host`'s `running`, `bound`, `room` and `streams` as Flume printed each, sliced from the pass's one `status --json` by `serde_json`'s `RawValue` and never re-serialised. `pools`, `addresses`, `bases` and `terminal_kept` are left out. They are Flume's bookkeeping, which the cloud sees only through its effect (the cloud's domain model), and account names are among them.
- **`leases`**: each launch's and each command's id with its number.

This is the Host object's copy of the host's capacity (the cloud's domain model and record 0005), carried as Flume gave it. crew's implementation reads `repositories` and the host's `room` from it (Decision 18), and writes of it only `runner.rec`'s `Version` (Decision 17).

### 7. Taking an offer

An offer's answer, once given, is the runner's (Decision 11). It takes, holds, launches or refuses only an offer its report does not already hold settled. Each pass, for each offer in order:

1. **Settled.** An offer whose answer in the report is `Launched`, `Done` or `Refused` is left as it stands.
2. **Found.** An offer whose mark, `flywheel:<id>`, the pass's listing holds on a place is answered `Launched` at that place, and nothing is launched (Decision 11).
3. **A repository not kept.** An offer for a repository the configuration does not name is answered `Refused`. Its words name the repository and say this host's configuration does not keep it (core.1 "Not ruled out"; core.5).
4. **A clone not yet made.** An offer for a repository the configuration names whose clone is not yet made is left unanswered, so crew reads it as launching.
5. **After.** An offer whose `after` names a launch whose place has not read landed is answered `Held`. Its words name each launch it still waits on and where that stands, in the runner's own words: not yet launched; launched at a place not landed; refused, with the refusal's words; or dropped, so its place will never land. Its `at` moves only when the words do. The order is the control plane's (crew's teams-cross Decision 5). The runner reads only whether Flume says a place landed (core.1).
6. **Room.** An offer with no room (Decision 8) is left unanswered where it is new. Where it is `Held` on After that has since landed, it stays `Held`, its words saying it waits for room in its stream.
7. **Launch.** Otherwise it is launched (Decision 9).

An offer is taken when it is first answered: `Held`, `Launched` or `Refused`. From then on the report holds it under its lease, and every report renews the lease. An offer left unanswered is not taken by the runner: a hosted control plane may offer it elsewhere. crew's implementation holds a claim for it meanwhile (Decision 18).

### 8. Room, as Flume's status gives it

Room is read from the pass's one `flume --json status`, and counted down as the pass launches:

- **The host has room** while its `room` is `null`, which means unbounded, or above zero.
- **A stream has room** while the host has room, its own `room` is `null` or above zero, and its `queue` is empty. A place already waiting in a stream is ahead of anything the runner would add to it.
- **Each launch the pass makes** counts one against its stream's room and the host's, where they are numbers.
- **A launch that names a stream** is judged by that stream. A stream Flume's host part does not list is no room question: the launch is made, and Flume's refusal of an undeclared stream (flume record 0020) is its answer. The runner never refuses in Flume's place.
- **A launch that names no stream** runs in the stream Flume's host configuration gives it (cloud record 0003), which Flume's status does not say before the launch. It is judged by the host's room alone. It may then wait in Flume's queue on this host for a stream that is full. That is the gap *What this change asks of others* names. crew's role runs name a stream, and are judged exactly. A kit's launch from `the-team-machinery-leaves` names none, and meets the gap.

This is core.9. Room is Flume's: the runner reads it and adds no bound of its own.

### 9. A launch and its acts, into Flume's command line

The runner runs `flume` from the host's `PATH`, with `--json` first on every call and `FLUME_RECIPIENT` unset, so a place's recipients are only those its launch names. It never passes `--force`.

**A launch** is one `flume --json launch`:

| flag | from |
|---|---|
| `--repository <dir>` | the clone's container (`clone::container`) |
| `--kind <kind>` | `kind`, as the control plane gave it (Decision 10) |
| `--name <name>` | `name` |
| `--base <base>` | `base`, where given |
| `--stream <stream>` | `stream`, where given |
| `--priority <n>` | the newest priority this launch's acts set before it was launched, else `priority`, where either is given |
| `--intent <intent>` | `intent`, where given |
| `--recipient <r>` | each of `recipients` |
| `--hold <stage>=<words>` | each of `holds` |
| `--hold-submission` | `hold_submission` |
| `--brief-file <file>` | `brief`, written whole to a file under `<data>/control/briefs/` and removed once Flume answers |
| `--agent <field>=<value>` | each of `agent_settings` |
| `--mark flywheel:<id>` | the runner's own (Decision 11) |

The runner adds no stream, priority or intent of its own (core.9), and passes no `--kinds` (Decision 10).

**An act** runs on the place its launch's answer names:

- a retry is `flume retry <place>`, with `--stage` and `--words` where given, and `--submission` where asked;
- a drop is `flume drop <place>`;
- a priority is `flume priority <place> <n>`;
- a done is `flume done <place>`.

**Answers** are Flume's:

| Flume | the answer |
|---|---|
| exit 0 from a launch | `Launched`, at the place Flume's answer names |
| exit 0 from an act | `Done` |
| exit 1 | `Refused`, its words the `reason` of Flume's `refused` answer as given |
| exit 2, or an answer that is not JSON | `Refused`, its words Flume's standard error as given |

A failure to run `flume` at all, not found or killed, is no answer. Nothing is kept, the words are logged, and the next pass tries again. The runner rewrites, summarises and judges no answer (core.1). Its own words appear only where it declined: a repository not kept; a launch held; an act on a launch not yet launched, or not its own; a want it cannot read; a command of a sort it does not carry.

**Acts on a launch not yet launched.** Such an act is answered from the report, with no command run:

- A **priority** is answered `Done`, and the launch, when made, carries the newest priority among its own and its acts', in the control plane's order (crew's teams-cross Decision 6).
- A **drop** is answered `Done`. The launch is never made, and is answered `Done` with words naming the drop.
- A **retry** or a **done** is answered `Refused`, its words naming where the launch stands.
- **An act whose launch is no launch of this runner's** is answered `Refused`, naming the launch.

### 10. Kinds are Flume's

A launch names its kind, and that is all the runner and its control plane say of kinds. The runner passes `kind` to `--kind` as the control plane gave it: a name, `change`, or an identity, `<source>/<name>@<version>` (flume record 0022). It passes no `--kinds`, and names no kinds file, kind source or lock, in its configuration, in an implementation's table or on a command line (core.5). Which sources a host's Flume declares, which packages lie on the host and which versions a lock records are the host's Flume configuration (flume's `a-host-declares-where-its-kinds-come-from`).

- **crew's requests** name a kind by name alone, `change`, `chore` or `fix`, which Flume resolves among the repository's own kinds and Flume's.
- **crew's role runs** are launched by their kind's identity, `<Kinds>/<Kind>` of the role record, `crew/crew-planner` for the planner. `Kinds` names the source and `Kind` the name, each as crew wrote it. crew writes no version, so Flume takes the one the host's lock records, else the newest of the source (flume record 0022). The source `crew` is crew's kinds, which the host's Flume configuration declares (*After deploy*).
- **Later, a marketplace.** The cloud's marketplace may configure each host's Flume. Nothing in the runner names what it configures, so nothing here changes when it comes. Where the marketplace reaches a host through its hosted control plane, it does so by a command the runner carries out through Flume's own command line, as it carries a bolt's branch through sluice's (core.1; cloud record 0025). That command's sort is its own change's, and `Command::Other` refuses it until then.

Alternative: a kinds file per implementation's table, passed as `--kinds`. Not taken: it makes the control plane, and the runner's configuration, say where a kind comes from, which is Flume's host configuration and later the marketplace's (core.5).

### 11. The runner's report, each act once, and leases lost

The runner keeps its report in `<data>/control/report.json`. The file is written whole, through a temporary file and a rename, after every act and command and before the next. A restart reads it, so a retry, a drop or a branch made just before a crash is answered from the report and not made again (core.10).

Each launch carries the mark `flywheel:<id>`. An offer with no answer whose mark Flume's listing holds is answered `Launched` at that place, and not launched again. So an answer lost with the data directory is found from Flume. Only an act or a command, which leaves no mark, depends on the report file. A report file lost with the data directory is taken from the control plane, which holds every answer the runner reported (Decision 5, step 1). The window left is an act made after the last report sent and lost with the data directory before the next.

**Leases.** Every report names each lease's number with its work, and so renews it (cloud record 0009). A report the control plane refuses for a lease the runner no longer holds names the id in `Reported::refused`. The runner then acts on that launch or command no more, reports it no more, drops it from the report, and says in one line which place it leaves on the host, as Flume holds it. It removes nothing (core.10). Dropping a stray place is a command the control plane may send, and is not built here. crew's implementation gives the lease number 1 with no expiry and refuses no report, so for crew a lease is never lost.

Alternative: report after every act, with no report file of the runner's own. Not taken: it puts a commit on crew's branch for every act, and still loses an act whose push fails.

### 12. Flume's status of each place, carried as Flume gave it (c-flume.a)

For each launch the report holds with a place, the carried status is `flume --json status --repository <container> <place>`'s whole answer, as Flume printed it, on one line. It is the text crew reads (`flume_facts`, `asks.status_of`), its `repository` and `stream` among it.

That answer holds Flume's host part too, which changes at every read. So the pass's one listing decides when to read it again. The place's entry in the listing is compared, as a JSON value, with the place's entry inside the carried answer. Only where they differ is the place asked for again, and the new answer carried with a new `at`. The text is never re-serialised or trimmed (core.11). An idle place keeps its text, and so writes nothing to crew's branch.

A place the listing no longer holds, tidied with nothing pending, is asked for once a pass by its location. Where Flume answers, its answer is carried. Where Flume refuses, the last carried status stands, with `listed` false.

The report names each launch's lease number with its status: the status reaches the control plane under the lease the runner holds (c-flume.a; cloud record 0009).

### 13. sluice's entry of each submission (c-sluice.a)

Flume's status of a place names the sluice entry its submission is: `submission.read.found.submission.id`, a fact `facts` reads with a guard. For each launch whose carried status names one, the runner runs `sluice status <id> --json` in the launch's clone container. It carries a JSON object built from these fields of sluice's answer, each field's text as sluice printed it:

- `id`, `branch`, `head` and `state`;
- `failed`, each row with only its `task`, `hash`, `flaky` and `system`;
- `landed_as`.

These are the copy's fields (the cloud's domain model). A row's `log`, which locates a task's output, is left out, and nothing is read from `refs/sluice/logs` or `refs/sluice/heads` (core.11; cloud core.4, core.28). The entry is read again at each pass until its `state` is `landed`, `failed`, `conflict` or `withdrawn`. It is carried anew only when the object changes.

crew's records have no place for an entry. crew reads a submission from Flume's status, so crew's implementation writes nothing of it.

### 14. An inbox the control plane names (c-inbox.a)

A `Want::Inbox` asks the runner to carry the inbox kept on `branch` of `repository`. The runner keeps no repository for it (core.5):

- An inbox in a repository the configuration does not keep is answered with words naming it.
- One whose clone is not yet made waits.
- One whose branch origin does not hold is answered with words naming the branch.

At each control pass, the runner reads the tip of `refs/remotes/origin/<branch>` in the clone, which the keeper's fetch brings, since the clone fetches every head. Where the tip has moved since the last carried, it reads the tip's files under `inbox/` by `git ls-tree -r` and `git cat-file` through the sibling's git builder, which fetches nothing, and checks them whole with `weir_signals::InboxAt`:

- where weir takes them, it carries the tip's commit and every record file under `inbox/` by its path, its text as the tip holds it. Each item's `brief.md` is carried as `sha256:<hex>` of its bytes, since the copy of an item names its record and not its brief (the cloud's domain model);
- where weir refuses them, it carries the tip with weir's words, and the files carried before stand.

An inbox holds no raw material: a capture holds a pointer to where its material was banked (weir's statement 17). The runner reads nothing a pointer names (core.11).

crew's records have no place for an inbox, and crew's implementation names none.

### 15. A bolt's branch (cloud record 0017)

A `Command::Branch { repository, branch, from }` asks for `branch` to be made from `from`:

1. **Settled.** A command the report holds settled is left as it stands (core.10).
2. **A repository not kept.** It is answered `Refused`, naming the repository (core.5).
3. **A clone not yet made.** It is left untaken. Any host keeping the repository may take it, and with none it waits (cloud core.19).
4. **Otherwise** it is taken under its lease. The runner runs `sluice branch create <branch> --from <from> --json` in the clone's container, with no room asked (core.9 "Not ruled out"). sluice's standard output is the command's `text`, as given, and its exit gives `Done` (0) or `Refused` (any other, with sluice's `message` or standard error as words). Then the runner reads `git ls-remote origin refs/heads/<branch>` through the sibling's git builder. The commit origin's branch stands at is `origin_commit`, or none where origin holds no such branch.

A branch already there is refused by sluice, `already there`, and its `origin_commit` names the commit it stands at. The runner judges neither. The control plane counts the branch made from `origin_commit` (cloud record 0017 and core.19). The runner never reads sluice's words for that. Where sluice refuses for want of a gate, the answer goes up as given, and what the bolt shows is the control plane's (cloud record 0017).

`Command::Other` is answered `Refused`, naming the tool and saying this binary does not carry it.

### 16. A terminal on a launch's agent (N1.b)

A `Terminal` pulled names a launch and its lease. The runner opens it only where the report holds that launch under that lease number, and the launch's carried status names its agent's pane, as `clues.shown_at.agent` gives it, `herdr:<pane>@<socket>`. Otherwise it closes the link at once with words saying why, such as "no agent is running in its place". The pane's address is Flume's fact (flume record 0018; ledger item N1.a).

An open terminal runs on a thread of its own, since it runs herdr and never git:

- **out**: `herdr pane read <pane> --source visible --format ansi`, with `HERDR_SOCKET_PATH=<socket>`, every quarter of a second. A screen is sent on the link only when it differs from the last one sent;
- **in**: each `Text` from the link is `herdr pane send-text <pane> <text>`, and each `Keys` is `herdr pane send-keys <pane> <keys>…`.

It closes when the link closes or the far end stops listening, when herdr refuses a read (the pane is gone), or when the runner lets the launch go.

Nothing of a terminal is kept (core.11; cloud core.18). No screen or input is written to the data directory, a log line or a report, and the thread holds only the last screen sent, to compare against. Its log lines name the terminal's id, the launch and the pane, never what passed.

This is a screen relay through herdr's command line, not a stream of the pane's bytes. herdr's command line gives snapshots and input, and the runner reaches herdr through nothing else (record 0001). crew's implementation opens no terminal.

### 17. crew's implementation: reading at each tip, writing the runner's branch

**Reading.** At each pull, for each partition, crew's implementation:

- Reads `<label>/main` by weir-branch's `read` into a bare cache, `<data>/control/crew/<owner>/<name>.git`, one per state repository, outside `repositories/`, so the keeper never takes it for a clone. It takes `requests.rec`, `roles.rec`, `asks.rec` and `plan.rec` at that one tip. A file missing at the tip holds nothing.
- Reads every `refs/heads/<label>/runner/*` that `git ls-remote` of the state repository lists, through the same cache: other hosts' answers, for the claims of Decision 18, and their places, for the places that wait on a role (Decision 19). For its own host it reads the report the runner hands the pull, which is newer than its branch, and its branch for the claims it holds and before its first report.
- Gives each request for this host, in `Request` order. A request naming a `Host` is for the host it names. A launch naming none, and an act on one, are for this host as Decision 18 gives them.

  | `Act` | want |
  |---|---|
  | `launch` | `Offer`. `id` is `<label>/<Request>`. `lease` is 1 with no expiry. `launch` is the repository (Decision 18 gives it for a `Repo` named by name alone), `Kind`, `Name`, `Base`, `Stream` where named, `Priority`, `Intent`, each `Recipient` and `Brief`, each `After` as `<label>/<n>`, and `Hold: submission` as `hold_submission` |
  | `retry` | `Act::Retry`. `id` is `<label>/<Request>`, `launch` is `<label>/<Of>`, and `lease` is 1. It takes `Stage` and `Words`, and `submission` where the request holds a `Submission` field |
  | `drop` | `Act::Drop`, likewise |
  | `priority` | `Act::Priority(Priority)`, likewise |

  A request naming this host that it cannot read is given as an offer or an act the runner refuses, with words naming the field: a number that is not whole, an `Act` it does not know, or a `Hold` other than `submission`. Such a request naming no host is given nothing and logged, since no one runner's answer to it would be the one. A record with no `Request` number is left out and logged, since it can be given no id.
- Gives each role record whose `Host` is this host as Decision 19 has it.
- Gives back, as `Pulled::report`, the runner's report read from `<label>/runner/<host>`'s tip.

A partition whose `<label>/main` cannot be read gives nothing that pass, its words logged, and the others are pulled as they stand. A pull is `Unread` only when no partition could be read.

**Writing.** At each report, and at a pull that claims or withdraws (Decision 18), for each partition, crew's implementation writes `<label>/runner/<host>` by weir-branch's `write`. Its `who` is `flywheel@<host>`, and its `what` says what changed, such as `answers 12 13, places 12` or `claims 20 21`. Nothing is written when the files' text equals the tip's. No ref but `<label>/runner/<host>` is ever pushed, and never with force. The branch is made by its first write, which writes `runner.rec` whatever else there is, so a branch that exists says a runner answers there (crew's teams-cross Decision 4).

The files are formed from the report and the claims standing by weir-capture's `rec::write`, so their text depends on nothing else. Each opens with crew's own descriptor, as `runner.py` holds it, byte for byte: `RUNNER`, `ANSWERS`, `PLACES`, then `RUNS`.

- **`runner.rec`**: one `Runner`, with `Host`, `Version` (`flywheel --version`'s text) and `Since` (the first write's time, kept by every later write).
- **`answers.rec`**: one `Answer` per request of the partition's the report answers, by `Request`, with `Outcome`, `Place`, `Words` and `At`, and one per claim standing whose launch the report does not yet answer (Decision 18). A launch's outcome is its answer's. An act's is `done` or `refused`.
- **`places.rec`**: one `Place` per request launch with a place, in `Request` order, with `Request`, `Place`, `At` and `Status` (Decision 12's text). Then one `Run` per role, in `Address` order, for its latest run, with `Address`, `Place`, `At`, `Status` and `Words`. `Words` holds the refusal of its launch.

**Let go.** A launch goes from the files, and is named in `Reported::let_go`, once Flume no longer lists its place and the work it served has left crew's plan at `<label>/main`'s tip:

- `unit/<u>` and `chore/<c>` leave with the plan's `Unit` record of that name;
- `fix/<bolt>/<name>` leaves with the plan's `Bolt` record of `<bolt>`.

Work the plan never holds, `fix/main/<kit>/<name>` and `role/<role>/<name>`, keeps its last record: crew's design names no time it leaves (*Risks*). A role's earlier run goes from the report once a later run of the role is recorded and the earlier one's place is over. A role's last run goes once its role record has gone from `roles.rec` and its place is over. A request's answers go with its launch.

### 18. crew's launches that name no host: claimed on the runner's own branch

crew's `the-team-machinery-leaves` writes a kit's launch with no `Host` or `Stream`. A runner whose configuration keeps the repository takes it, several hosts may keep one repository, and the lease decides which takes each launch (its Decision 4, settled with the user on 2026-11-01). crew names no arbiter, and reads the runner that took a request as the one whose answers hold it. So crew's implementation takes the lease itself, from crew's own records and the runners' branches, as it reads a role's wanting (core.1 "Not ruled out"). Each runner pushes only its own branch.

**Whose it can be.** A launch naming no `Host` is for this host only where its `Repo` is a repository the runner keeps: an address the hello names, or a name alone that exactly one address the hello names has as its name, which becomes the offer's repository. A launch of a repository the runner does not keep is given nothing and written nothing, never refused, so another host's runner may take it. A name two kept addresses share is given nothing and logged, naming both. A retry, drop or priority naming no `Host` is for this host only where the runner's report, or its own branch, answers the launch its `Of` names.

**Claiming.** A launch that is for this host and that no runner's branch answers is claimed before it is given:

1. **Only while the host has room**: the hello's capacity gives the host's `room` as `null` or above zero. A host with no room claims nothing, and its work is left for a host that has.
2. **The claim.** crew's implementation writes its branch with the launch's `Answer` `held`, its `Words` the runner's own, `claimed by <host>'s runner; launching once no other runner has claimed it`. Every launch it claims at that pull goes in one write.
3. **The check.** It then reads every other `<label>/runner/*` branch again.
4. **Alone**: each claimed launch no other branch answers is given as an offer, under lease 1.
5. **Not alone**: each claimed launch another branch answers is withdrawn. A second write of the pull removes its claim, and nothing is given for it.

A claim stands on the branch until the runner's report answers its launch, which then replaces it, so a launch left unanswered for want of room in its stream keeps its claim. At every later pull, a restart's among them, a launch the runner's report answers is given; one only its own claim answers is given while no other branch answers it, and found by its mark if it was launched (Decision 11); and one its own claim and another branch both answer, as after a stop between a claim and its check, is withdrawn as at the check.

**Why one runner launches.** Each runner checks only after its own claim is pushed. Of two runners claiming one launch, the one that checks second sees the first's claim, so at least one withdraws, and a runner launches only where it saw no other claim. Where both see each other, both withdraw. A runner that withdrew claims the launch again only while no branch answers it, and no sooner than as many pulls later as its host's place, counting from 1, in the name order of the hosts whose claims it saw. So the first in name order claims again first, and the other sees its claim. That wait is held in memory, and a restart forgets it.

crew reads a launch two branches answer as unknown, naming both (`runner.taken`). Between two claims and the withdrawal of one, that lasts one pass.

**The hosted control plane** gives a lease to one runner itself (cloud record 0009). Nothing of claims is in the interface: they are how crew's implementation stands in for an arbiter crew does not have.

Alternatives:

- A shared branch of leases, moved by a leased push whose first writer wins. Not taken: a runner pushes no ref but its own branch, which crew reads as its own.
- A deterministic choice, the first host in name order keeping the repository. Not taken: a runner learns another keeps a repository only from its branch, which names no repositories, and the first host may be down or full.

### 19. A role's run, read as crew reads it (R1 to R4)

For each partition, for each `Role` whose `Host` is this host, crew's implementation decides from crew's own files, as crew's `asks.py` at `9913f249` decides:

- **`wanted`**: `pending(asks, waits, turns, role)` leaves anything:
  - `answered` is the `Took` of every `Turn` of the role with `Ended`. A superseded or open turn answers nothing.
  - The asks are each `Ask` with that `Role` whose `ask/<Id>` is not answered.
  - The waits are `waits_on(address, requests, places)` over every host's `places.rec` (this host's from the runner's report):
    - `place/<n>@<since>`, or `place/<n>` where Flume gives no `since`, is a place whose launch names the role's `Address` as `Recipient` and whose status waits on its caller with `why` `answer`, `lost` or `past_bound`, with `on_hold` naming no stage, or `stalled`;
    - `landed/<n>` is a `role/…` launch whose place has landed.

    Each counts where it is not answered.
- **`stands`**: the role's latest run, the highest `k` of its runs in the runner's report (else its `Run` record on the branch), has a place whose carried status reads `live` or `making` (crew's `role_run`).

Then:

- **Offered while wanted and not standing**: one `Offer`. Its `id` is `<label>/<role>/<k+1>`, where `k` is the highest the report holds of the role and 0 where it holds none. Its `lease` is 1. Its `launch`:
  - `repository` is `Repository`; `kind` is the identity `<Kinds>/<Kind>` (Decision 10); `name`, `stream`, `priority` and `intent` are `Name`, `Stream`, `Priority` and `Intent`;
  - `agent_settings` is `model=<Model>`;
  - there is no base, brief or recipient (R1).

  The offer keeps its id from pass to pass until the runner records it, so a run waiting for room is offered as the same launch. A refused launch is recorded with Flume's words, and the next pass offers `k+2` while the role is still wanted.
- **Done once delivered (R3).** A `done` is given where the latest run's carried status waits on its caller with `why` `delivered`: its one stage has delivered and its kind does not land. It is `Act::Done` with `id` `<label>/<role>/<k>/done`. The place is then tidied by Flume, and the role's next run can be made. A refusal is the run's `Words`.
- **Recorded by address (R4)**: the latest run is written as the role's `Run` record (Decision 17).

A role record names its `Host`, so a role's run is never claimed. The runner decides nothing here. Wanted, standing and done are crew's rules, read from crew's files inside crew's implementation (core.1 "Not ruled out"; core.2; crew's `roles-run-as-flume-kinds` Decision 6: "the runner reads the same, from the same files"). The runner's part is Decisions 7 to 11, as for any offer. Flume's refusal of a second place of one name is the backstop that keeps one run at a time.

### 20. The hosted control plane, named and unbuilt

The hosted control plane's implementation would be a crate of its own, `flywheel-hosted`:

- its table, `[control_plane.hosted]`, names the control plane's `url`, and the `token_file` the host's token is read from, never the token (core.4; cloud core.24);
- it keeps one WebSocket it opens and holds, outgoing (core.3; cloud core.37), which ends at the host's Host object (cloud core.13);
- every message names the protocol version and is checked against that version's schema (cloud core.36). Where that schema lives is left open by cloud record 0022, and that change settles it;
- a pull is its hello at every reconnect, the hello and the report whole (cloud core.52). It is answered from the messages received since, and the socket's thread calls the `Wake` its opener was handed when one arrives;
- a report sends what the server has not acknowledged, each piece under its id (cloud core.38). A refusal under a replaced lease number fills `Reported::refused` (cloud core.39);
- a terminal's link is a stream of the socket's messages for that terminal's id (cloud record 0016).

It joins `planes.rs` as one more word and opener, and passes `flywheel-control`'s shared cases. Nothing of it is built here. `type = "hosted"` is refused as a word this binary does not carry (Decision 4).

### 21. Tests: a fake control plane, stub tools and Flume's own answers

Every test stays inside the gate. It reaches no network, runs no real Flume, sluice or herdr, and touches no configuration or data of the user's.

- **The pass's decisions** are tested over `flywheel-control`'s `Fake`, which holds wants and terminals a test sets and records each report. They are also tested over a fake `Flume` behind the binary's own trait, which scripts each call's answer and records each call.
- **The command lines** are tested against stub programs that a test writes into its own temporary directory. The argv, the environment and the reading of answers are what each test checks. A stub appends its arguments and environment to a log, prints a fixture, and exits as the test says. One stub each is written for `flume`, `sluice` and `herdr`.
- **The fixtures** are Flume's own answers, captured on a host from the Flume it runs (0.4.1 at this writing), under `crates/flywheel/tests/fixtures/flume-<version>/`. Each comes with a README saying where, when and from what it was read. They are `status --json` of places at each phase and wait the runner reads, a launch's answer, and a refusal. treefmt excludes the fixtures by name (core.7), since a formatter would change the bytes the tests hold the runner to carry as written.
- **crew's implementation** is tested over a bare repository the test makes, its file URL standing in for the state repository. crew's files are written into it as crew writes them, and each report is read back with `git show`. Two runners' claims are tested as two implementations over one bare repository, with a test hook between a claim's write and its check.
- **The inbox** is tested over a scratch forge branch holding an `inbox/` written in weir's layout.
- `recfix` runs in the tests, through weir-branch's write, at the version `devenv.nix` pins (core.8).

### Rules relied on

- **core.1**: Decisions 7, 9, 10, 12 and 15; crew's rules read inside crew's implementation (Decisions 18 and 19).
- **core.2**: Decisions 1, 2 and 17 to 20.
- **core.3**: Decisions 2, 16 and 20. Every connection is outgoing: git to the state repository and to origin, and later one WebSocket.
- **core.4**: Decisions 4 and 20.
- **core.5**, as amended in this change's commit to name no kind: Decisions 3, 4, 6, 7, 10, 14, 15 and 18.
- **core.6**: nothing commercial is named.
- **core.7**: Decision 1, and the fixtures' exclude.
- **core.8**: recutils, and every tool the tests run, pinned in `devenv.nix`.
- **core.9**: Decisions 7, 8 and 18.
- **core.10**: Decisions 11, 15 and 18.
- **core.11**: Decisions 6, 12 to 14 and 16.
- **core.12**, added in this change's commit: Decisions 2 to 4, and every module and file name of Decision 1.

### Rules departed from

None.

### Departures from crew's asks

- **R1's `--kinds`.** crew's `roles-run-as-flume-kinds` R1 asks for `flume launch --kinds <crew's kinds> --kind <Kind>`, the kinds file named by the runner's own configuration. Here a role's run is `--kind <Kinds>/<Kind>`, and no configuration names a kinds file, since kinds are Flume's (core.5; the user, 2026-11-01). Its source is declared in the host's Flume configuration (Decision 10).
- **`the-team-machinery-leaves`, *What this change rests on* 3**, asks the runner to ignore, rather than refuse, a launch of a repository it does not keep. crew's implementation gives such a launch nothing, which is that, while a request naming this host for a repository not kept is still refused (core.1 "Not ruled out"). It asks that a kit's launch be taken "under its lease": Decision 18 is that lease.

### Departures from flywheel-next's design

The original Flywheel's host (flywheel-next's `definitions/engine/host.yaml` and `lease.yaml`) differs from this runner in six ways, each cited by flywheel-next's requirement:

- **Who takes a lease** (128, 149). Here the control plane offers each lease, and the runner takes or leaves only what it was offered (core.9; cloud records 0005, 0009). crew's implementation, standing in for a control plane with no arbiter, claims a kit's launch on the runner's own branch and gives it only once no other runner has (Decision 18).
- **How a lease ends.** Here a lease ends at the expiry the control plane gave it, by the control plane's alarm, and the runner learns of it from a refused report (cloud core.39; Decision 11).
- **Stray places** (55, 222). Here the runner removes no place: a place whose lease it lost is left as Flume holds it, and named (core.10).
- **The manifest** (149, 205). Here the configuration names repositories alone and no kind or stream (core.5; cloud records 0003, 0007), and kinds are Flume's.
- **The tick** (231, 297). Here the runner runs no machine. It reaches its control plane through one interface: crew's records now, and one outgoing socket later (core.2, core.3; cloud core.37).
- **Capacity.** Here capacity is Flume's streams and room, as Flume reports them (cloud record 0003; Decision 6).

## What this change asks of others

- **Flume.**
  - `a-host-declares-where-its-kinds-come-from`, built and released, so a host's Flume configuration declares the source `crew` and a launch of `--kind crew/crew-<role>` runs (Decision 10).
  - A way to read, before a launch, the stream that a launch naming none would run in. This could be status naming the stream for a repository, kind and intent, or a launch that refuses rather than queue. Until there is one, such a launch is judged by the host's room (Decision 8). crew's kit launches and the hosted control plane's launches name no stream, so they meet this gap.
- **crew.**
  - R1 read as Decision 10 has it: a role's `Kinds` names the kind source the host's Flume declares for crew's kinds, and no runner configuration names a kinds file.
  - A claim is an `Answer` `held` with the runner's words, standing a pass or more before its launch is answered, and two branches may answer one launch for a pass (Decision 18). crew reads both as it reads any `held` answer and any launch two runners answer.
- **The cloud.** Where the protocol's schema lives (cloud record 0022), whether hosts advertise roles (the pack's #12, PROPOSED), and the command by which the marketplace configures a host's kinds (Decision 10). Each belongs to the change that builds the hosted implementation or the marketplace.
- **millpond.** Which branch of which repository holds a Flywheel's inbox. The control plane names it in a `Want::Inbox`, and the runner names none of its own.

## Risks / Trade-offs

- **crew's halves are on crew's bolt and in crew's code stage, not yet on crew's main.** The fields and descriptors here are those of `9913f249` and `2b97ee57`. Where crew changes one before this change is built, the coder takes crew's and names it.
- **A role's run is refused until Flume's kind sources are on the host.** Until `a-host-declares-where-its-kinds-come-from` is released and the host declares `crew`, Flume refuses `--kind crew/crew-<role>`, and each wanted role's run is written with that refusal. crew's requests run meanwhile.
- **The role reading is a copy of crew's.** Decision 19 ports `asks.py`'s `pending`, `waits_on` and `role_run` into Rust. A later change to crew's reading leaves the runner wanting a role crew does not, or the reverse. crew's own scenarios pin it here, and a change to crew's `asks.py` is a chore of this repository's.
- **A claim never moves.** A runner that claims a kit's launch and then stops for good holds it, and no other runner takes it, since crew's leases have no expiry. A claimed launch waiting for room in its stream waits on its host though another host keeping the repository has room. Both are seen in crew's reading of the runner's branch.
- **Two runners claiming at once both withdraw** and claim again a pull or more later, in name order. A launch so contested starts a pass or two late.
- **Work the plan never holds keeps its last record.** A fix of main and a role's landed work stay in `places.rec` after Flume tidies them, since crew's design names no time they leave. They are few, and a later change settles their going with crew.
- **A busy host commits each pass.** Flume's status of a working place changes as its agent works, so the runner's branch takes a commit once a pass while any place works, and one or two more at a pass that claims. An idle host commits nothing (Decision 12).
- **A tidied place may go unseen.** A place tidied between two passes, with no end pending, keeps the last status the runner read. A launch after it then waits on a landing it never reads, and its held words say where the launch it waits on stood.
- **A launch that names no stream may queue in Flume** on a host whose other streams have room (Decision 8).
- **Most of the hosted contract is proved only against the fake.** The branch command, sluice's entries, the inbox and terminals reach no live control plane until the hosted implementation is built, since crew's records have no place for them.
- **The terminal is a screen relay at a quarter of a second**, not a byte stream. Typing shows at that pace. A byte stream would need herdr to give one on its command line.
- **A base the runner's clone holds only as origin's.** The sibling's clones keep only the default branch locally, and Flume resolves a launch's base through its merge-queue profile. A launch on `bolt/<name>` whose base Flume cannot resolve there is refused by Flume, and the refusal goes up as given. The runner fetches the repository (`keep_one`) before a launch whose base origin holds and the last fetch lacked.
- **The gate's sandbox reaches only localhost.** The weir revision and the crates.io dependencies must be in cargo's cache before the gate runs, as flume's weir-kinds is.

## Migration Plan

- **The configuration's keys.** A host's file written for the landed runner says `views` and `fetch_every`. The release that carries this change refuses both, naming `repository_root` and `fetch_interval`, and a file that is refused keeps nothing and removes nothing (the configuration spec). So the file is written with the new keys in the same deploy as the release.
- **Carrying work.** A host's runner, configured for a partition, makes its branch at its first control pass, and crew gives work on that host once the branch exists. To stop carrying work, remove `[control_plane]`: the runner goes on keeping its repositories, and crew reads its branch as it last stood.

## After deploy

No step here is a check this repository's gate can hold. Each is another repository's, or the host's.

- **afterthought/swancloud installs the runner with recutils, Flume 0.4.1 or later, sluice and herdr**, and writes its configuration with `repository_root` and `fetch_interval`, and its `[control_plane]` table for each host that serves a partition: the host's name as crew's hosts file has it, and each partition's label and state repository. Why not code here: the machine's configuration is afterthought/swancloud's.
- **Flume's host configuration declares each stream crew's launches and roles name** (crew's `roles-run-as-flume-kinds`, *What this change rests on* 4), and maps each repository kept to its stream for crew's kit launches. Why not code here: streams are Flume's host configuration (core.5).
- **Flume's host configuration declares the kind source `crew`**, crew's kinds in the crew release the host pins, once Flume's `a-host-declares-where-its-kinds-come-from` is released; later the marketplace configures it. Why not code here: kinds are Flume's host configuration (core.5).
- **The host's git credential helper reaches each state repository over https.** Why not code here: the credential is the host's, never this repository's (core.4).

## Task notes

**1.1** In `crates/flywheel/src/config.rs`, `Config`'s `views` becomes `repository_root` and `fetch_every` becomes `fetch_interval`, and the raw table's keys likewise. Their reading, defaults and refusals are unchanged but for the key named. The unknown-key refusal adds, for `views` and `fetch_every`, the key that holds each now, and for `stream`, `streams`, `kind`, `kinds` and `kinds_file`, that streams, or kinds, are Flume's host configuration. `run.rs` takes the new names, and its start line names the repository root. Every test under `crates/flywheel/tests` and every unit test writing either key writes the new one. README's configuration block and its words, and CLAUDE.md's "under the views root", say the new keys. Tests:

- the configuration spec's scenarios "No views setting", "A home-relative root", "A relative root", "No period", "A period of nothing", "A misspelt key", "A stream named", "A kinds file named" and "A key of before";
- the running spec's "Passes on the timer" with `fetch_interval`.

**2.1** `crates/flywheel-control/Cargo.toml`: `publish = false`, the workspace's version and edition, and `[lib] doctest = false`. Its dependencies are serde with derive and time, from `[workspace.dependencies]` in the root `Cargo.toml`, which gains time (`formatting`, `parsing`, `macros`). Add the crate to `members`. In `crates/flywheel-control/moon.yml`, copy `crates/flywheel/moon.yml`'s four tasks with `-p flywheel-control`, `language: 'rust'` and `layer: 'library'`. Their inputs are the crate's own sources, `/Cargo.toml`, `/Cargo.lock`, `/devenv.nix`, `/devenv.lock`, `/.prototools` and `/rust-toolchain.toml`. Add `flywheel-control: 'crates/flywheel-control'` to `.moon/workspace.yml`, and its line to the comment there.

**2.2** The types of Decision 2 go in three files:

- `src/lib.rs`: the trait, `Hello`, `Pulled`, `Unread`, `Unsent` and `Reported`;
- `src/want.rs`: `Id`, `Lease`, `Want`, `Offer`, `Launch`, `Act`, `Command`, `Inbox`, and `Terminal` with its `Link` trait;
- `src/report.rs`: `Report`, `Launched`, `Answer`, `Outcome`, `Carried`, `Acted`, `Commanded`, `InboxCarried` and `At`.

`Report` round-trips through serde as JSON, its maps ordered (`BTreeMap`). `Carried::text` is a `String` holding the tool's text. `Report::answer` and `Report::carry` are helpers, with tests that a carried text's `at` moves only when its text changes and that an answer's `at` moves only when its words do.

**2.3** `src/fake.rs`: `Fake`, a control plane in memory. It holds the wants and terminals a test sets, refuses a pull or a report when the test says so, and records each report. It gives the last report back as `Pulled::report`, and lets go and refuses what a test names. Its terminal link is a pair of channels the test holds. `src/shared.rs` holds the cases every implementation passes, each a function generic over a harness that makes a control plane holding given wants:

- a pull gives the wants in order;
- a reported answer and carried status come back in the next pull's report;
- a second report of the same report changes nothing the harness can see;
- a want the report holds settled is still given, and the runner, not the control plane, answers it from its report.

`Fake` runs them in the crate's own tests.

**3.1** `crates/flywheel/Cargo.toml` gains serde_json (with `raw_value`) from `[workspace.dependencies]`. `src/flume.rs` holds `Flume`, a trait with `launch`, `act`, `status` (every place and the host) and `status_of` (one place), and `Cli`, its implementation running a program path (`flume` in the binary). Each call builds its argv as Decision 9 has it, puts `--json` first, unsets `FLUME_RECIPIENT`, and reads its answer as Decision 9. The launch writes its brief under a directory it is handed and removes it after. Its tests run against a stub program written into a `tempfile` directory:

- each act's argv, and a launch with every flag and one with none of stream, priority and intent;
- a launch whose kind is an identity, `crew/crew-planner`, passed whole to `--kind`, and no `--kinds` in any argv;
- `FLUME_RECIPIENT` unset;
- exit 0 with a fixture;
- exit 1 with `{"refused":{"refusal":{"reason":"…"}}}`, the reason taken as given;
- exit 2 with words on standard error;
- a program that is not there.

Capture the fixtures from Flume on a host into `crates/flywheel/tests/fixtures/flume-<version>/`:

- `status --json` of every place, holding host streams with and without room and a queue;
- `status --json <place>`: at construct, in code with tasks, waiting on its caller with `answer`, a role's run waiting with `delivered`, submitted with a sluice entry, landed and tidied, and a live agent with `clues.shown_at.agent`;
- a launch's answer, and a refusal.

Strip nothing. Add `README.md` beside them, saying where, when and from what each was read. Add the exclude `"crates/*/tests/fixtures/**"` to `treefmt.nix` with its reason.

**3.2** `src/facts.rs` holds the facts the runner reads from Flume's status, each guarded so that a field that is absent or of another type reads as none (crew `read.2`):

- of a place: its location, its `mark`, whether its phase is live or making, whether it landed (with its commit), dropped or done, whether `waiting_on.why` is `delivered`, its submission's entry id, and its agent's pane address split into pane and socket;
- of the host: `room`, and each stream's `name`, `room` and whether its `queue` is empty.

It slices a place's entry out of a listing by its location, comparing entries as `serde_json::Value`. It slices the host part's four fields by `RawValue`. Its tests run over every fixture of 3.1.

**4.1** `src/report.rs` holds the report file: read at start, missing read as none, written whole through a temporary file and a rename at `<data>/control/report.json`. `src/hello.rs` holds the hello of Decision 6, over a `Tools` trait whose real form runs each `--version`. `src/carry.rs` holds the pass of Decision 5, over a `&mut dyn ControlPlane`, a `&dyn Flume`, a `&dyn Clones` (a forge address gives `Ready(dir)`, `NotYet` or `NotNamed`, which the binary implements over `clone::find` and the configuration) and the report file's path. This task covers its steps 1 to 4 and 7 for offers alone, Decision 7's steps 1 to 4 and 7, and Decision 9's launch and answers. `crates/flywheel/Cargo.toml` gains `flywheel-control` by path, and `crates/flywheel/moon.yml`'s four tasks take `/crates/flywheel-control/**/*` among their inputs in the same task. Tests run over `Fake`, a fake `Flume` and a fake `Clones`, one per scenario of `work-into-flume`'s offer, launch, repository and hello requirements.

**4.2** Decision 8's room, counted down within a pass. Tests:

- a stream with room 1 and two offers: one launched, one unanswered;
- a stream with a queue: none launched;
- a host with room 0: none launched;
- `room` null: launched;
- a stream Flume does not list: launched, and Flume's refusal is the answer;
- no stream: judged by the host's room.

**4.3** Decision 7's After and Decision 9's acts on a launch not yet launched. Tests:

- a launch held, then launched once its After's fixture status reads landed;
- held words for each of the four standings;
- a held launch whose After landed with no room, its words naming the stream;
- a priority, a drop, a retry and a done of a launch not yet launched;
- an act whose launch is no launch here.

**4.4** Decision 11's report file and marks. Tests:

- a pass whose fake Flume's retry answers and whose report then fails, followed by a fresh pass from the saved report that makes no second retry;
- the report file missing and the launch's mark in the fake listing: answered `Launched`, with no launch made;
- the report file missing and `Pulled::report` holding an act: no second act.

**4.5** Acts on launched places (Decision 9). Tests: one per act, and a refusal carried with Flume's words.

**4.6** Decision 11's leases. Tests:

- the report's lease numbers named in the hello and the report sent;
- a report refusing one launch: the launch dropped from the report, no act on it afterwards, nothing run against its place, and one line naming the place left;
- the control timer's due time before a lease's expiry (a unit test of the timing function).

**4.7** Decision 12. Tests:

- a carried place read again only when its entry changes, its text byte for byte the stub's `status --json <place>` answer;
- a host part that changes alone writing nothing;
- `status_of` once a pass for a place the listing lacks, its last status kept with `listed` false on a refusal;
- the user's own place in the listing left alone;
- `Notices::landed` once for a landing.

**5.1** `src/sluice.rs`: `Sluice`, a trait with `entry(container, id)` and `branch(container, branch, from)`, and `Cli` over a program path. Decision 13's object, built from sluice's answer field by field, `log` left out. In `carry.rs`, entries are read until final and carried anew only on a change. Tests run against a stub `sluice` printing a submission's JSON with a `log` in a failed row:

- the carried object holds no `log`, and its fields' text is as printed;
- a landed entry is not read again;
- a launch with no entry runs no `sluice`.

**5.2** `src/inbox.rs`: Decision 14, its files read through `git::Git`. The root `Cargo.toml`'s `[workspace.dependencies]` gains weir-signals at `{ git = "https://github.com/agentplot/weir", rev = "0b6eb5abb806a51fec128b01f97c4dcff6170219" }`, with a comment as flume's has, and `sha2`; `crates/flywheel/Cargo.toml` takes both. Tests over a scratch forge whose branch holds an `inbox/` written by weir-signals' own write in the test:

- the record files carried with their text;
- `brief.md` carried as its digest;
- an unchanged tip not read again;
- an inbox weir refuses, carried with weir's words;
- a repository not kept, and a branch origin lacks, each answered with words.

**6.1** In `carry.rs`, Decision 15 over `Sluice` and the sibling's `git::Git` for `ls-remote`. Tests run against a stub `sluice` and a scratch forge:

- a branch made: `Done`, with `origin_commit` the forge's commit;
- `already there`: `Refused`, its text as the stub printed it, with `origin_commit` the forge's commit;
- a refusal for want of a gate: `Refused`, with no `origin_commit`;
- a repository not kept;
- a clone not yet made: left untaken;
- a command given twice: run once;
- `Command::Other` refused.

**7.1** `src/terminal.rs`: Decision 16, over a `Herdr` trait (`read(pane, socket)`, `text(pane, socket, text)`, `keys(pane, socket, keys)`) and `Cli` over a program path, run on a thread per terminal. Tests run against a stub `herdr` and `Fake`'s link:

- the argv and `HERDR_SOCKET_PATH`;
- a screen sent once while unchanged and again when changed;
- text and keys passed in;
- closed when the link closes, when the stub refuses a read, and when the launch is let go;
- refused for a launch not held and for a status naming no pane.

A test also asserts that the data directory holds no file the relay wrote and that the log lines hold no screen's text.

**8.1** `crates/flywheel-crew` is a crate as 2.1 has one. It depends on `flywheel-control`, serde, serde_json, toml and time, and on weir-branch and weir-capture, which `[workspace.dependencies]` gains at the revision 5.2 names, with tempfile as a dev-dependency. Its moon project is `flywheel-crew`, whose inputs take `/crates/flywheel-control/**/*` too. `devenv.nix` gains `recutilsVersion` and `pkgs.recutils` among `packages`, asserted equal as git is, with a comment saying weir-branch's writes run its `recfix`. `check-toolchain.sh` checks `recfix --version` against it.

**8.2** `src/config.rs`: crew's table of Decision 4 from a `toml::Table`, each refusal naming the key. Tests:

- the example of Decision 4;
- each missing key;
- a label twice;
- `state_repository` not `owner/name`;
- `kinds` refused, saying kinds are Flume's host configuration;
- `token` refused, its value in no output.

**8.3** `src/read.rs`: Decision 17's reading. It takes the files at `<label>/main`'s tip by `weir_branch::read` into the cache, and parses `requests.rec` by `weir_capture::rec::read`, descriptor records marking each record set. Each request naming this host is given as a want. Tests run over a bare repository a test makes:

- requests naming two hosts, only this host's given;
- a launch's every field, a `Brief` over three lines and two `After`;
- a retry with `Submission`;
- an `Act` crew does not write, naming this host, given as refused; and naming no host, given nothing;
- a partition whose branch is missing giving nothing while another gives its own.

**8.4** `src/write.rs`: Decision 17's writing. The three files are formed from the report and the claims standing, with crew's descriptors copied byte for byte from `runner.py` at `9913f249`, and written by `weir_branch::write` with `who` `flywheel@<host>`. `runner.rec` is written at the first write, and its `Since` kept after. Nothing is written when unchanged. The report is read back from the branch for `Pulled::report`. Tests run over a bare repository:

- the first report making the branch with `runner.rec`;
- a held answer becoming launched in the same report;
- a place's `Status` byte for byte;
- an act's `done`;
- a `Run` with `Words`;
- a second report writing no commit;
- `<label>/main`'s tip unchanged after a report;
- `git for-each-ref` on the remote showing no new ref but the runner's branch;
- `recfix --check` passing on each file written;
- each file parsed by a port of crew's `host_read` checks: one `Runner` naming the host, no request twice, and every `Status` JSON.

**8.5** `src/claim.rs`: Decision 18. A launch naming no `Host` is matched to a kept repository by address or by name alone, against the hello's `repositories`. The claim's write and withdrawal go through `write.rs`, and the check reads every other runner's branch through `read.rs`. The wait after a withdrawal is a count of pulls per launch, held in the implementation. An act naming no `Host` is given where the report or the branch answers its launch. Tests run two implementations, `host-a` and `host-b`, over one bare repository, with a hook between a claim's write and its check:

- a launch claimed alone: given as an offer, `held` with the claim's words on `host-a`'s branch;
- a launch `host-b` answered first: nothing claimed or given by `host-a`;
- `host-b`'s claim pushed between `host-a`'s write and its check: `host-a` withdraws and gives nothing;
- both claiming before either checks: both withdraw, `host-a` claims again at its next pull and is given the launch, and `host-b` sees its claim;
- the hello's `room` 0: nothing claimed;
- `Repo: crew` against `example/crew`: given as `example/crew`; against two addresses named `crew`: given nothing and logged;
- a repository not kept: given nothing and written nothing;
- a claim left unanswered for want of room in its stream: kept by the report's write and given again;
- an implementation opened afresh over a branch holding its claim: the launch given at once; and with `host-b`'s branch answering it too: withdrawn;
- an act naming no `Host` on a launch `host-b` answered: given nothing; on a launch this branch answers: given.

**8.6** `src/roles.rs`: Decision 19. It parses `roles.rec` and `asks.rec` as crew's `records` and `parse` do, and reads `waits_on` across every host's `places.rec`, its own host's from the report. Each role record for this host is given as an offer or a done. The cases are crew's `role-runs` scenarios and `tests/py/test_asks.py` at `9913f249`, written as crew's text into the bare repository:

- a burst of three asks, offered once, of kind `crew/crew-planner`;
- an ask after a turn began;
- every ask answered: no offer;
- a stage of another host's place waiting `answer` on the planner;
- a run standing: no offer;
- a run waiting `delivered`: a done;
- a superseded turn;
- a `role/…` launch landed;
- `on_hold` naming no stage;
- `stalled`;
- a host whose branch does not parse;
- `k` kept while unrecorded, and `k+1` after a refusal.

**8.7** In `write.rs`, Decision 17's let-go from `plan.rec` at the tip read. Tests:

- a unit's place tidied while its `Unit` is in the plan: kept;
- the bolt landed and the `Unit` gone: let go and named;
- a fix on a bolt gone with its `Bolt`;
- a fix of main kept;
- a role's earlier run let go once a later one is recorded.

**8.8** In `crates/flywheel-crew/tests/shared.rs`, run `flywheel-control`'s shared cases with a harness that makes a bare repository with `requests.rec` and `roles.rec` written as crew writes them.

**9.1** `src/planes.rs`: the list of Decision 1, holding `crew` and `flywheel_crew::open`. `crates/flywheel/Cargo.toml` gains `flywheel-crew` by path, and `crates/flywheel/moon.yml`'s four tasks take `/crates/flywheel-crew/**/*` among their inputs in the same task. `config.rs` reads `[control_plane]`: `type`, `poll_interval`, and the table of `type` handed on whole. `run.rs` gains the control timer and `Event::Wake` of Decision 5, with `--once` making one pass of each. Tests:

- the example of Decision 4 opening crew's implementation;
- `hosted` refused, naming `crew`;
- `poll_interval` read in each of its forms, and `0s` refused;
- no `type`: refused;
- no `[control_plane]`: no control pass;
- `--once` running one of each, against a stub `flume` and a scratch state repository.

**9.2** `crates/flywheel/tests/rules.rs`, which reads the tree from `CARGO_MANIFEST_DIR`:

- **core.2**: no file under `crates/flywheel/src` but `planes.rs`, and none under `crates/flywheel-control`, names `flywheel_crew`; and the crates' dependencies are as Decision 1 has them.
- **core.3**: the sibling's `no_listener.rs` reads every `crates/*/src`.
- **core.7**: every directory under `crates/` is a project in `.moon/workspace.yml`, with a `moon.yml` declaring `language: 'rust'` and the tasks format, lint, typecheck and test, each tagged `check-fast`. Each task's inputs name the sources of every crate it builds.

**9.3** `crates/flywheel/tests/pass.rs`: one control pass end to end, with crew's implementation over a bare state repository, a stub `flume`, a scratch forge and the sibling's clones:

- crew's start of `a` and `b`, `b` after `a`, writes two launch requests naming no `Host` or `Stream`, of a repository the configuration keeps;
- the first pass claims both, answers `a` launched and `b` held, with `a`'s place in `places.rec`;
- the stub's status then says `a` landed;
- the next pass answers `b` launched, with `--base bolt/x`, `--priority` and `--intent` in its argv and no `--stream`.

**9.4** README gains "Carrying the control plane's work": what the runner carries, the `[control_plane]` table with Decision 4's example in a fenced `toml` block, the runner's branch, and that kinds and streams are Flume's host configuration. CLAUDE.md's opening paragraph says what the crate now does. `crates/flywheel/tests/readme.rs` reads README's `toml` block under that heading and opens crew's implementation's configuration from it.

**10.1** The last task runs only the projects this change touches.
