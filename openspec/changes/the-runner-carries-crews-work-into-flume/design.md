# Design

## Context

See proposal.md for why. This change is the runner's half of the
exchange crew's `teams-cross-to-flume-as-their-bolts-land` specifies
crew's half of (its Decision 4, and its *Dependencies*, "The runner"),
together with the runner's part of crew's `roles-run-as-flume-kinds`
(its *What this change rests on*, 2, R1 to R5). Both crew changes were
read on 2026-10-31 in their places,
`~/Code/github_afterthought/crew/places/change--teams-cross-to-flume-as-their-bolts-land`
at `ce80f8cf` and `~/Code/github_afterthought/crew/places/change--roles-run-as-flume-kinds`
at `6f479a36`, with crew record 0052.

What stands:

- **This repository** is at `2b8b9ec`: one crate, `crates/flywheel`,
  which answers `--version`; the gate's check-fast layer
  (`.moon/workspace.yml`, `.config/sluice.toml`); record 0001; rules
  core.1 to core.8.
- **The sibling change, `the-runner-keeps-its-repositories`**, cut from
  the same `main`, makes the configuration file, the runner's data
  directory, the long-running process the host starts and its timer,
  and the bare clone and view of each repository the configuration
  names. This change builds on it (*What this change rests on*, 1).
- **crew's requests**, `requests.rec` on `<label>/main` of a
  partition's state repository, record set `Request`, numbered from 1,
  append-only. Fields: `Request`, `Act` (`launch`, `retry`, `drop`,
  `priority`), `Host`, `Work`, `Repo`, `Kind`, `Name`, `Base`,
  `Stream`, `Priority`, `Intent`, `Recipient`, `Hold`, `After`
  (repeated), `Brief` (multi-line), `Of`, `Stage`, `Submission`,
  `Words`, `Date` and `By` (teams-cross Decision 4 and Task notes 3.1;
  its descriptor is not yet built).
- **crew's reading of the runner**, the same Decision. A branch
  `<label>/runner/<host>` per host per partition holds `runner.rec`,
  `answers.rec` and `places.rec`. The runner alone writes it, and
  pushes it without force; crew fetches `<label>/runner/*` with
  `<label>/main` and never writes it.
- **crew's role records and asks**, built at `a5cca10a` and `a5afd9ec`
  on `change/roles-run-as-flume-kinds`. `roles.rec` holds the record
  set `Role`: `Role`, `Address`, `Run`, `Host`, `Repository`, `Kinds`,
  `Kind`, `Name`, `Stream`, `Priority`, `Intent`, `Model` and `Effort`.
  `asks.rec` holds `Ask` (`Id`, `Role`, `From`, `What`, `Names`, `Key`,
  `Words`, `At`) and `Turn` (`Turn`, `Role`, `Run`, `Place`, `Host`,
  `Took` repeated, `Began`, `Ended`, `Said`, `Superseded`). What is
  pending for a role is decided in `plugin/lib/asks.py`, by `pending`,
  `answered`, `waits_on`, `waiting`, `landed` and `status_of`.
- **crew's plan**, `plan.rec` on `<label>/main`: record sets `Bolt`
  (key `Bolt`) and `Unit` (key `Unit`, a chore marked `Kind: chore`).
  A bolt's landing takes it and its units out of the plan.
- **Flume 0.3.6** on the host, read 2026-10-31 by its `--help` and its
  site at agentplot/flume `3a126d4`:
  - `flume launch --kind --name [--base] [--brief-file] [--kinds]
    [--hold-submission] [--recipient] [--mark] [--priority]
    [--agent <field>=<value>] [--stream] [--intent]`;
  - `flume retry <place> [--stage] [--words] [--submission]`,
    `flume drop <place>`, `flume priority <place> <n>` and
    `flume done <place>`;
  - `flume status [<place>]`, which with no place gives every place
    on the host from any directory;
  - every command takes `--json` and `--repository <dir>`, exits `0`
    with its answer, and exits `1` with `refused: <reason>`, or with
    `{"refused":{"refusal":{"reason":…}}}` under `--json`.

  A place's `phase` is `"live"`, `{"making":…}`, or `{"over":…}`,
  `{"gone":…}` or `{"tidied":…}`, each holding `over` as
  `{"landed":{"commit":…}}`, `"dropped"` or `"done"`
  (`crates/flume-seams/src/held.rs`, `Phase` and `Over`).
  `waiting_on.why` is one of `on_hold`, `answer`, `lost`,
  `never_begun`, `unready`, `refused`, `stopped`, `past_bound`,
  `submission` and `delivered`, the last meaning the last stage
  delivered and the kind does not land (its `Waits`). `--mark` is free
  text, given back in status. The JSON answer is one line.
- **weir** at `0b6eb5abb806a51fec128b01f97c4dcff6170219`, the revision
  flume pins:
  - weir-branch's `write` forms a write against the tip read, runs
    `recfix --check` on every `.rec` file it writes, makes one commit
    through a temporary index, and moves the remote branch only from
    that tip, by a push leased on it, never with force;
  - weir-branch's `read` gives a branch's files at one tip, through a
    bare cache the caller names;
  - weir-capture's `rec::read` and `rec::write` read and write
    recutils text, `+` continuation lines and repeated fields among it.

There are no area pages; `docs/architecture/core.md` binds every path
this change touches.

## Goals / Non-Goals

**Goals:**

- A runner on a host answers every request crew writes for it, once, in
  the form crew reads, turning each into the matching `flume` command
  in its clone of the request's repository, a launch always naming
  `--stream`, `--priority` and `--intent` (crew record 0052 part 3).
- Flume's status of every place the runner launched reaches crew as
  Flume gave it, on the runner's own branch, with no other branch
  pushed (core.1; crew `state.5`).
- At most one run of each role crew names is kept on its host, made
  while crew's records leave anything pending for it
  (`roles-run-as-flume-kinds` R1 to R4).
- The control plane is reached through one interface. Crew's
  implementation is the only code naming crew, and a hosted control
  plane can take its place behind the same interface (core.2; record
  0001; crew record 0052 part 5).

**Non-Goals:**

- The clones, the views, the configuration file, the data directory,
  the process and its timer: `the-runner-keeps-its-repositories`.
- The hosted control plane's implementation: named here (Decision 16),
  not built.
- Passing a role's `Effort` (`roles-run-as-flume-kinds` R5): Flume
  0.3.6's agent settings carry no effort. The runner passes it once
  they do.
- A held host's role runs: an open question of crew's
  (`roles-run-as-flume-kinds`, *Open Questions*), not settled.
- Waking the runner early when a record is written: crew pulls (record
  0052 part 2), and the runner's timer carries it.
- Anything of sluice's: the runner calls no sluice command. Flume
  submits.

## What this change rests on

1. **`the-runner-keeps-its-repositories`, built first.** This change
   needs from it:
   - the configuration file and its reader, to which this change adds
     the `[control_plane]` table (Decision 3);
   - the runner's data directory, where this change keeps the runner's
     record (Decision 7) and the caches of crew's state repositories
     (Decision 12);
   - the long-running process and its timer, on which a pass runs
     (Decision 4);
   - for each repository the configuration names, by its forge address
     `owner/name`, the directory Flume takes as `--repository`, under
     which Flume makes its places; and whether that clone is made yet
     (Decision 9);
   - the forge rule its clones are reached by, `https://` through the
     host's credential helper, which this change uses for state
     repositories too (core.4);
   - its entry for being told of a landing, which this change calls
     when a place it carries first reads landed (Decision 10).

   Where one is missing when a group needs it, the coder stops short
   with `flume needs --found`, naming it, rather than building it here.
2. **crew's `teams-cross-to-flume-as-their-bolts-land`.** The runner
   reads `requests.rec` by the fields its Decision 4 names. Its reader
   of the runner's branch, its Task notes 3.2, reads the shapes this
   change settles where its Decision 4 left them open (Decision 14).
3. **crew's `roles-run-as-flume-kinds`.** `roles.rec` and `asks.rec` as
   built at `a5cca10a`, and what is pending for a role as `asks.py`
   decides it at `a5afd9ec` (Decision 15).
4. **Flume 0.3.6 or later** on the host, and **recutils** on the
   host's path for weir-branch's `recfix` (*After deploy*).

## Decisions

### 1. Three crates, and only crew's names crew

- **`crates/flywheel-control`** is a library: the control-plane
  interface, its types, the fake control plane, and the cases every
  implementation passes. It depends on no implementation.
- **`crates/flywheel-crew`** is a library: crew's implementation. It
  depends on `flywheel-control`, weir-branch and weir-capture, and is
  the one crate that names crew, its branches or its records.
- **`crates/flywheel`** is the binary. It holds the pass (Decision 4),
  Flume's command line (Decisions 5 and 6), role runs (Decision 11),
  and one list of the implementations it carries, `src/planes.rs`. In
  that list each implementation is named by its configuration word and
  the function that opens it from its configuration table, and nothing
  else.

core.2 rules out "a function of the runner's own with a branch for one
implementation". The list is data, a word against an opener. It is the
one place outside an implementation that names one. Every other module
of the binary takes a `&mut dyn ControlPlane`. A test holds the line
(Task notes 6.2). Each crate is a moon project of its own, each check
one task (core.7).

Alternative: one crate with a module per implementation. Rejected
because a module boundary is not checked by the compiler, while a
crate's dependencies are.

### 2. The interface: a pull and a report

`flywheel-control` gives one trait, `ControlPlane`, with two calls.

- **`pull`** gives what the control plane wants of the host now, read
  fresh, or `Unread` with words saying why.
- **`report`** takes the runner's whole record and gives `Reported`,
  or `Unsent` with words saying why.

Its types:

| Type | Fields |
|---|---|
| `Pulled` | `requests`, the requests for this host in the control plane's order; `runs`, the runs it wants kept; `record`, the runner's record as the control plane last held it, or none |
| `Request` | `key`, the control plane's; `act` |
| `Act` | `Launch(Launch)`; `Retry { of, stage, words, submission }`; `Drop { of }`; `Priority { of, priority }`, where `of` is a launch's key |
| `Launch` | `repository`, a forge address `owner/name`; `kind`; `name`; `base`; `stream`, `priority` and `intent`, each optional so the runner can refuse a launch lacking one (Decision 5); `recipients`; `brief`; `after`, the keys of launches whose places must land first; `hold_submission`; `kinds`, a declaration's path; `agent`, settings as field and value |
| `Run` | `key`; `launch`; `wanted`, whether the control plane wants a run now; `unrunnable`, the control plane's own words when it cannot give a launch, such as a kinds file its configuration does not name |
| `Record` | `answers` by request key; `places` by request key; `runs` by run key |
| `Answer` | `outcome`, one of `Held`, `Launched`, `Done` and `Refused`; `place`, once launched; `words`; `at` |
| `Carried` | `place`, its location; `at`, when the runner first read Flume's status as it now stands; `status`, Flume's text of the place (Decision 10); `listed`, whether Flume's last listing held it |
| `RunHeld` | `place`; `at`; `status`; `words`, why no run stands where none could be launched |
| `Reported` | `let_go`, the keys of carried places the control plane carries no more (Decision 13) |

A key is a string the control plane gives, unique among the host's
wants, which the runner never reads into. Crew's keys are
`<label>/<request>` for a request and the role's address,
`<label>/<role>`, for a run. Role names are words and request numbers
are digits, so the two never meet.

**Why this shape carries a hosted control plane.** Every value is
plain data: no handle into git, no path into a state repository, no
socket. A pull is "what do you want of me now", and a report is "here
is what I have done". A crew implementation answers them by reading and
writing branches. A WebSocket implementation would answer a pull from
the messages its socket received since the last one, and send a report
as one message, or as the part of the record the server has not
acknowledged. The runner's own code is the same for both.

Alternative: a launch call per request pushed into the runner.
Rejected because a pull keeps every connection outgoing (core.3), lets
crew's records be read whole at one tip, and needs no queue in the
runner.

### 3. The configuration's control-plane table

The configuration file is the sibling's. This change adds one table,
read at start:

```toml
[control_plane]
kind = "crew"
every = "60s"

[control_plane.crew]
host = "deneb"

[[control_plane.crew.partitions]]
label = "madswan"
state = "afterthought/madswan-state"

[control_plane.crew.kinds]
crew = "/nix/store/…-crew/plugin/flume/kinds.toml"
```

- **`kind`** is the implementation's word, required.
- **`every`** is the interval between passes, written as Flume writes
  times (`<n>`, `<n>s`, `<n>m`, `<n>h`), `60s` where none is named.
- **The table named by `kind`** is handed whole to that
  implementation's opener, so its keys are its own (core.2).

A `kind` this binary does not carry is refused at start, naming the
ones it carries, and the runner pulls nothing. That includes `hosted`,
which is named and not built (Decision 16). The sibling's own work goes
on: clones and views do not depend on the control plane.

Crew's table:

| Key | |
|---|---|
| `host` | this host as crew's hosts file names it: the `Host` of the requests it answers, and of the role records whose runs it keeps. Required, with no default: the machine's own hostname is not crew's name for it. |
| `partitions` | each partition served: `label`, as crew writes it, and `state`, its state repository as `owner/name`. At least one, each label once. |
| `kinds` | each kinds file a role record's `Kinds` may name, by that name, as the path of the declaration in the crew release the host pins (R1). |

A missing or malformed key is refused at start, naming the key and
the file.

A state repository is reached by the forge rule the configuration
gives the clones, `https://github.com/<owner>/<name>.git` on a host,
through the host's git credential helper. The configuration names no
credential, and none is read but by git's own helper (core.4). A test
hands file URLs of its own.

### 4. A pass

A pass runs on the runner's timer every `every`. Each pass:

1. **Takes the runner's record** (Decision 7): the record in the data
   directory; where that file is missing, the record the first pull
   gives back; where neither exists, an empty one.
2. **Pulls.** An `Unread` ends the pass having acted on nothing, its
   words logged; the next pass tries again.
3. **Reads Flume once**, by `flume status --json`. For each place the
   record carries, it takes Flume's text of that place (Decision 10).
   For each launch or run with no answer or no place yet, it looks for
   a place bearing the runner's mark for its key (Decision 7).
4. **Acts on each request in the control plane's order**
   (Decisions 5 to 9). The record is saved after each act.
5. **Keeps each run** (Decision 11).
6. **Asks again for places Flume no longer listed**, each by its
   location; a place Flume answers nothing of keeps its last status
   (Decision 10).
7. **Reports the whole record.** It drops the carried places the
   report lets go, and saves the record. An `Unsent` keeps the record
   as it is, to be reported at the next pass.

A pass acts on nothing Flume did not list, and on no place it did not
launch (Decision 9). Passes never overlap: the next starts only once
one has ended.

### 5. A request into Flume's command line

The runner runs `flume` from the host's path, with `--json` on every
call. It unsets `FLUME_RECIPIENT` in the command's environment, so a
place's recipients are only those its request names, none for a role's
run (R1).

**A launch** is one `flume --json launch` with these flags, in this
order:

| Flag | From |
|---|---|
| `--repository <dir>` | the runner's clone of `repository` (Decision 9) |
| `--kinds <file>` | `kinds`, where it is given |
| `--kind <kind>` | `kind` |
| `--name <name>` | `name` |
| `--base <base>` | `base`, where it is given |
| `--stream <stream>` | `stream` |
| `--priority <n>` | the newest priority this launch's requests set (Decision 8) |
| `--intent <intent>` | `intent` |
| `--recipient <r>` | each of `recipients` |
| `--brief-file <file>` | `brief`, written whole to a file in the data directory, removed once Flume answers |
| `--hold-submission` | `hold_submission` |
| `--agent <field>=<value>` | each of `agent` |
| `--mark flywheel:<key>` | the runner's own (Decision 7) |

`--stream`, `--priority` and `--intent` are on every launch, so no
launch depends on the host's own file (crew record 0052 part 3; crew
`roles.9`). A launch lacking one is refused without a command run, its
words naming what it lacks.

The other three acts run on the place the launch made, `of`'s answer's
`place`, none of them with `--force`:

- **A retry** is `flume retry <place>`, with `--stage <stage>` and
  `--words <words>` where the request names them, and `--submission`
  where it asks for the submission.
- **A drop** is `flume drop <place>`.
- **A priority** is `flume priority <place> <n>`.

The runner forces nothing Flume refuses. Whether an agent that is not
quiet is ended is the control plane's to ask, and crew asks no force.

### 6. An answer is Flume's

| Flume's exit | The answer |
|---|---|
| `0`, from a launch | `launched`, at the place in Flume's answer, whose status is the place's first carried status |
| `0`, from a retry, drop or priority | `done` |
| `1` | `refused`, its words the `reason` of Flume's `refused` answer as given |
| `2`, or an answer that is not JSON | `refused`, its words Flume's standard error as given |

A `2` means a flag Flume does not know, such as an older Flume
lacking `--stream`.

A failure to run `flume` at all, not found or killed, is no answer.
The act is not made, the request stays unanswered, and the words are
logged; the next pass tries again. The runner rewrites, summarises and
judges no answer (core.1). Its own words appear only where the act was
its own to decline: a launch held (Decision 8), a repository its
configuration does not name (Decision 9), a request it cannot read, or
an act on a launch it did not launch.

### 7. Each request is acted on once

A request with an answer other than `held` is never acted on again.
The runner keeps its record in the data directory,
`<data>/control/record.json`, written whole by rename after every act,
before the next. A restart reads it, so a retry or a drop made just
before a crash is answered from the record, not made again.

Each launch, and each run, carries the mark `flywheel:<key>`. A launch
with no answer whose mark Flume's listing holds is answered `launched`
at that place, not launched again. So an answer lost with the data
directory is found from Flume, and only acts that leave no mark (a
retry, a drop, a priority) depend on the record. A record lost with the
data directory is taken from the control plane, which holds every
answer the runner reported (Decision 4, step 1). The window left is an
act made after the record's last report and lost with the data
directory before the next.

Alternative: report after every act, with no record of the runner's
own. Rejected because it puts a commit on the runner's branch per act,
and still loses an act whose push fails.

### 8. A launch waits for its After work to land

A launch whose `after` names launches is answered `held` until the
place of each of them reads landed on its base: its phase over, gone or
tidied with `over` `landed`. Then, at the first pass that sees it so,
it is launched. While held, its words name each launch it still waits
on and where that stands, in the runner's own words:

- not yet launched;
- launched at a place not landed;
- refused, with the words of that refusal;
- dropped, so its place will never land.

Its `at` moves only when the words do. The order is crew's, written in
the request (teams-cross Decision 5); the runner reads only whether
Flume says a place landed (core.1).

**A priority of a held launch** is answered `done`, and the launch,
when made, carries the newest priority among its own and the
priorities that name it, by the control plane's order (teams-cross
Decision 6 sends priorities for held launches).

**A drop of a launch not yet launched** is answered `done`. The launch
is never made, and is answered `done` with words naming the drop.

**A retry of a launch not yet launched** is refused, naming where the
launch stands.

**A retry, drop or priority whose `of` is no launch this runner
answers** is refused, naming its `of`.

### 9. Only what the configuration names, and only the runner's own places

- **A repository the configuration does not name.** A launch for it,
  or a run in it, is refused, its words naming the repository and that
  this host's configuration does not name it (core.1 "Not ruled out";
  core.5).
- **A clone not yet made.** A launch whose repository's clone is named
  but not yet made is left unanswered until it is, so crew reads it as
  launching.
- **Places it did not launch.** The runner acts on no place it did not
  launch, and carries none: a place is its own when its record holds
  the place's location, or Flume's listing gives the place its mark.
  The user's own places, and any other caller's on the host, are read
  in the listing and left alone.

### 10. Flume's status carried as Flume gave it

Each pass reads `flume status --json` once. Each place the runner
carries is matched by its location, and its status is the text of that
place's object as Flume printed it, byte for byte, sliced out of the
answer: serde_json's `RawValue`, never a value re-serialised, whose key
order and number forms could differ.

The host part of the answer, its queue, room and counts, is left out.
It is the host's status and not the place's, and it changes at every
read, so carrying it would rewrite every place at every pass. This is
not a summary of the place's status: every field Flume gave the place
is carried as Flume gave it. crew's `status_of` reads either form.

A carried place's `at` moves only when Flume's text of it changes. A
place Flume's listing no longer holds is asked for once a pass by its
location, `flume status --json <place>`. Where Flume answers, that
answer is its status; where Flume refuses, its last status stands and
`listed` is false.

The first pass that reads a carried place landed tells the sibling's
keeper of the landing, naming the repository, so its clone and view
are fetched without waiting for the keeper's own timer.

### 11. Role runs: one at a time, while wanted

A run is the control plane's ask that one place be kept on the host
while it is wanted. Crew's are its roles' runs (Decision 15). For each
run a pull gives, each pass:

- **It stands** when its record's place reads live or making in
  Flume's newest status, or Flume's listing gives a live place its
  mark. A run that stands is never launched again, whatever its stage:
  a lost run holds its role until someone retries it with `flume retry`
  (`roles-run-as-flume-kinds` *Risks*).
- **A run that is wanted and does not stand** is launched as Decision 5
  launches, with no brief and no recipient (R1). Crew's is
  `flume --json launch --repository <clone> --kinds <file> --kind
  crew-<role> --name <label> --stream <stream> --priority 0 --intent
  <label>-<role> --agent model=<model> --mark flywheel:<label>/<role>`.
  Its place and status are recorded by the run's key (R4). A refused
  launch is recorded with Flume's words, and the next pass tries again
  while the run is still wanted.
- **A run whose place waits on its caller with `delivered`** is
  declared done, `flume done <place>`: its one stage has delivered and
  its kind does not land, so its place is tidied and the next run can
  be made (R3). Where Flume refuses, its words are the run's, and the
  next pass declares it again.
- **A run whose repository the configuration does not name, or that
  the control plane gives as `unrunnable`,** is recorded with those
  words and not launched.

A run the control plane no longer gives keeps its record until its
place is over, and is then let go.

Only the record's place and Flume's listing say whether a run stands.
Flume's own refusal of a launch whose place stands is the backstop, and
the place it names is then found by its mark.

### 12. crew's implementation reads crew's records at each tip

Each pull, for each partition, crew's implementation:

- **Reads `<label>/main`** by weir-branch's `read` into a bare cache in
  the data directory, `<data>/control/crew/<owner>/<name>.git`, one per
  state repository, and takes `requests.rec`, `roles.rec`, `asks.rec`
  and `plan.rec` at that one tip. A file missing at the tip holds
  nothing, since crew makes each at its first write.
- **Gives each request whose `Host` is the configuration's `host`,** in
  `Request` order, as Decision 2's `Request`:

  | Act | Fields given |
  |---|---|
  | launch | `Repo`, `Kind`, `Name`, `Base`, `Stream`, `Priority`, `Intent`, `Recipient`, `Brief`, each `After` as the key `<label>/<n>`, and `Hold: submission` as `hold_submission` |
  | retry | `Of`, `Stage`, `Words`, and `submission` where the request holds a `Submission` field, whatever its value |
  | drop | `Of` |
  | priority | `Of`, `Priority` |

  A request it cannot read is given as a request the runner refuses,
  with words naming the field: a number that is not whole, an `Act` it
  does not know, a `Hold` other than `submission`. A record with no
  `Request` number is left out and logged, since it can be given no
  key.
- **Gives each role record whose `Host` is this host** as a run
  (Decision 15).
- **Gives back the runner's record** from `<label>/runner/<host>`'s
  tip, read the same way, where the runner holds none of its own
  (Decision 4, step 1).

A partition whose `<label>/main` cannot be read gives nothing that
pass, its words logged. The other partitions are pulled as they
stand. A pull is `Unread` only when no partition could be read.

### 13. crew's implementation writes the runner's branch, and only it

Each report, for each partition, crew's implementation writes
`<label>/runner/<host>` by weir-branch's `write`:

- the three files formed against the branch's tip, each `.rec` file
  checked by `recfix --check`;
- one commit through a temporary index;
- the remote branch moved only from that tip, by a push leased on it,
  a fast-forward, never with force (crew `state.5`);
- nothing written when the files' text equals the tip's.

`ask.who` is `flywheel@<host>`, and `ask.what` says what changed, such
as `answers 12 13, places 12`. The branch is made by its first write,
which writes `runner.rec`. The runner's first pass for a partition
writes that file whatever else it has, so a branch that exists says a
runner answers there (teams-cross Decision 4). No ref but
`<label>/runner/<host>` is ever pushed.

**Let go.** A carried place goes from `places.rec`, and is named in
`Reported.let_go`, once Flume no longer lists it and the work its
request served has left crew's plan at `<label>/main`'s tip:

- `unit/<u>` and `chore/<c>` leave with the plan's `Unit` record of
  that name;
- `fix/<bolt>/<name>` leaves with the plan's `Bolt` record of `<bolt>`.

Work the plan never holds, `fix/main/<kit>/<name>` and
`role/<role>/<name>`, keeps its last record. Crew's design gives no
time it leaves (*Risks*). A Run record goes once the role record it
answered has gone from `roles.rec` and its place is over.

### 14. The formats, field for field

Field names, values and meanings are crew's (teams-cross Decision 4).
The record sets, descriptors, date form and the shapes below are this
change's, settling what crew's design left open, for crew's reader
(its Task notes 3.2) to read:

- dates are RFC 3339 in UTC to the second, as `2026-11-02T10:04:00Z`;
- the `Status` of a place is its object as Flume printed it in
  `flume status --json`, one line (Decision 10);
- a launch dropped before it was launched is answered `done`, with
  words naming the drop;
- a role's run is a `Run` record of `places.rec`, keyed by `Address`.
  Where no run could be launched it holds `Words` and no `Place`.

`runner.rec`:

```
%rec: Runner
%doc: The runner answering for this partition on Host: its version and when it began serving the partition.
%key: Host
%type: Since date
%mandatory: Host Version Since
%allowed: Host Version Since
%unique: Version Since
```

`Version` is what `flywheel --version` prints. `Since` is the time of
the branch's first write, kept by every later one.

`answers.rec`, one record per request the runner has read, in
`Request` order:

```
%rec: Answer
%doc: The runner's answer to a request of requests.rec on <label>/main: held while a launch waits on its After, then launched at its Place; done; or refused, with Flume's words or the runner's own.
%key: Request
%type: Request int
%type: Outcome enum held launched done refused
%type: At date
%mandatory: Request Outcome At
%allowed: Request Outcome Place Words At
%unique: Outcome Place Words At
```

`places.rec`, one `Place` per place launched for a request, in
`Request` order, then one `Run` per role, in `Address` order:

```
%rec: Place
%doc: A place the runner launched for a request, and Flume's status of it as Flume gave it, kept after Flume tidies it until the work it served has left crew's plan.
%key: Request
%type: Request int
%type: At date
%mandatory: Request Place At Status
%allowed: Request Place At Status
%unique: Place At Status

%rec: Run
%doc: A role's latest run on this host, by the role's address, and Flume's status of its place; Words where no run could be launched.
%key: Address
%type: At date
%mandatory: Address At
%allowed: Address Place At Status Words
%unique: Place At Status Words
```

Each file is written whole from the record by weir-capture's
`rec::write`, its descriptor first, so its text depends on nothing but
the record.

### 15. What is pending for a role, read as crew reads it

For each partition, crew's implementation gives each `Role` record
whose `Host` is this host as a run:

- **`key`**, its `Address`.
- **`launch`**: `repository` is `Repository`; `kind`, `name`,
  `stream`, `priority` and `intent` are `Kind`, `Name`, `Stream`,
  `Priority` and `Intent`; no base, brief or recipient; `kinds` is the
  configured path for `Kinds`, the run given `unrunnable` where the
  configuration names none; `agent` is `model=<Model>`.
- **`wanted`**: whether anything is pending for the role.

What is pending is read as crew's `asks.py` at `a5afd9ec` decides it,
from the same files (R2; `roles-run-as-flume-kinds` Decision 6: "the
runner reads the same, from the same files"):

- **`answered`**: the `Took` of every `Turn` of the role with
  `Ended`. A superseded or open turn answers nothing.
- **The role's asks**: each `Ask` with that `Role` whose
  `ask/<Id>` is not answered.
- **The places that wait on the role** (`waits_on`, `waiting`,
  `landed`, `status_of`): for each `Place` record of every host's
  `places.rec`, whose `Request` names a launch with the role's
  `Address` as `Recipient`:
  - `place/<n>@<since>` when its status waits on its caller with `why`
    `answer`, `lost` or `past_bound`, with `on_hold` naming no stage,
    or with `stalled` true (`place/<n>` where Flume gives no
    `since`);
  - `landed/<n>` when the launch's `Work` is `role/…` and its phase
    reads landed.

  Each counts as pending where not answered.

Every host's branch is read: a crossed team's place on another host
may wait on the planner, whose run is on the main level's host. So
crew's implementation reads each `refs/heads/<label>/runner/*` that
`git ls-remote` of the state repository lists, through the same cache.
For its own host it reads the record it last reported, which is newer
than its branch, and its branch before its first report. A host's branch that cannot be read, or whose files do
not parse, adds no waits that pass, and is logged with its host's
name. The asks and the other hosts still count.

The runner decides nothing here: wanted is crew's decision, read by
crew's rule inside crew's implementation (core.1, core.2). The cases
that pin it are crew's own `role-runs` scenarios: a burst, an ask
during a run, nothing pending, a stage that asks the planner, a stage
that delivers, a superseded turn, and a role's landed work. They are
written over crew's text as crew writes it (Task notes 5.5).

### 16. The hosted control plane, named and unbuilt

The hosted control plane's implementation would be a crate of its
own, `flywheel-hosted`:

- its table `[control_plane.hosted]` names the plane's address;
- it keeps one WebSocket it opens and holds, outgoing (core.3);
- it answers a pull from what the socket has received;
- it sends each report on the socket;
- its configuration names where the host's credential for it is read,
  never the credential (core.4).

It joins `src/planes.rs` as one more word and opener, and passes
`flywheel-control`'s shared cases. Nothing of it is built here, and
`kind = "hosted"` is refused as a word this binary does not carry
(Decision 3).

### 17. Tests: fakes, a stub flume and Flume's own answers

Every test stays inside the gate: it reaches no network, uses no real
Flume, and touches no configuration or data of the user's.

- **The pass's decisions** are tested over `flywheel-control`'s fake
  control plane, which holds wants and records reports, and over a
  fake Flume. The fake Flume is the binary's own trait for Flume's
  command line: it scripts each call's answer and records each call.
- **The command line itself**, the argv and the reading of answers, is
  tested against a stub `flume` that a test writes into its own
  temporary directory. The stub appends its arguments and environment
  to a log, prints a fixture, and exits as the test says.
- **The fixtures** are flume 0.3.6's own answers: `status --json` of
  places at each phase and wait the runner reads, a launch's answer,
  and refusals. They are captured on a host and kept under
  `crates/flywheel/tests/fixtures/flume-0.3.6/` with a README saying
  where and when each was read. treefmt excludes the fixtures by name
  (core.7), since a formatter would change the bytes the tests hold
  the runner to carry as written.
- **Crew's implementation** is tested over a bare repository the test
  makes, its file URL standing in for the state repository. Crew's
  files are written into it as crew writes them, and each report is
  read back with `git show`. Its tests run `recfix`, which `devenv.nix`
  pins (core.8).

### Rules relied on

- **core.1**: Decisions 5 to 8, 10, 11 and 15. The runner launches
  only what a request or a run asks, carries every answer and status
  as Flume gave it, and reads only Flume's facts of landing and
  delivery.
- **core.2**: Decisions 1, 2, 12 to 16.
- **core.3**: Decisions 2, 12, 13 and 16, every connection outgoing:
  git to the state repository, and later a WebSocket.
- **core.4**: Decision 3.
- **core.5**: Decision 9.
- **core.6**: nothing commercial is named.
- **core.7**: Decision 1, and the fixtures' exclude.
- **core.8**: recutils, and every tool the tests run, pinned in
  `devenv.nix`.

### Rules departed from

None.

## Risks / Trade-offs

- **crew's halves are still being built.** `requests.rec`'s descriptor
  (teams-cross Task notes 3.1) and crew's reader of the runner's branch
  (its 3.2) are not built. Field names here are crew's design's, and
  shapes crew left open are settled in Decision 14 for crew to read.
  Where crew settles a field otherwise before this change is built,
  the coder takes crew's and names it.
- **The role reading is a copy of crew's.** Decision 15 ports
  `asks.py`'s `pending` into Rust. A later change to crew's reading
  leaves the runner wanting a role crew does not, or the reverse.
  crew's scenarios pin it here, and a change to crew's `asks.py` is a
  chore of this repository's.
- **Work the plan never holds keeps its last record.** A fix of main
  and a role's landed work stay in `places.rec` after Flume tidies
  them, since crew's design names no time they leave. They are few;
  a later change settles their going with crew.
- **The runner's branch takes a commit whenever a carried place's
  text changes.** Flume's status of a working place changes as its
  agent works (`last_words`, `clues`), so a busy host commits once a
  pass, never more. The host part is left out, so an idle host commits
  nothing.
- **A tidied place may go unseen.** Flume keeps a tidied place's
  document while an end of it is pending, and crew's places name a
  role's address as recipient, whose ends wait. A place tidied between
  two passes, with no end pending, keeps the last status the runner
  read. A dependent's launch then waits on a landing it never reads,
  and the runner's words say where that launch stood.
- **Flume missing on the host leaves every request unanswered.** Crew
  reads them as launching, and the runner's log says why.
- **The gate's sandbox reaches only localhost.** The weir revision and
  the crates.io dependencies must be in cargo's cache before the gate
  runs, as flume's weir-kinds is. The revision is the one flume pins.

## Migration Plan

None: nothing exists to move. A host's runner, configured for a
partition, makes its branch at its first pass. Crew gives a crossed
team work on that host once the branch exists.

## After deploy

No step here is a check this repository's gate can hold. Each is
another repository's, or the host's.

- **swancloud installs the runner with recutils and Flume 0.3.6 or
  later**, and writes its `[control_plane]` table for each host that
  serves a partition: the host's name as crew's hosts file has it,
  each partition's label and state repository, and the path of crew's
  kinds file in the crew release the host pins. Why not code here:
  the machine's configuration is swancloud's.
- **The host's git credential helper reaches each state repository
  over https.** Why not code here: the credential is the host's, never
  this repository's (core.4).

## Task notes

**1.1** `crates/flywheel-control/Cargo.toml`: `publish = false`, the
workspace's version and edition, `[lib] doctest = false`, and serde
with derive and time as dependencies, from `[workspace.dependencies]`
in the root `Cargo.toml`, which gains them. Add the crate to `members`.
In `crates/flywheel-control/moon.yml`, copy `crates/flywheel/moon.yml`'s
four tasks with `-p flywheel-control`, `language: 'rust'` and
`layer: 'library'`. Its inputs are the crate's own sources, `/Cargo.toml`,
`/Cargo.lock`, `/devenv.nix`, `/devenv.lock`, `/.prototools` and
`/rust-toolchain.toml`. Add `flywheel-control: 'crates/flywheel-control'`
to `.moon/workspace.yml`, and its line to the comment there.

**1.2** `src/lib.rs` (the trait, `Pulled`, `Unread`, `Unsent`,
`Reported`), `src/want.rs` (`Request`, `Act`, `Launch`, `Run`) and
`src/record.rs` (`Record`, `Answer`, `Outcome`, `Carried`, `RunHeld`),
as Decision 2. `Record` round-trips through serde as JSON, its maps
ordered (`BTreeMap`). `Carried::status` is a `String` holding Flume's
text. One test per type's round trip, and `Record::answer` and
`Record::carry` helpers with tests that a carried place's `at` moves
only when its text changes.

**1.3** `src/fake.rs`: `Fake`, a control plane in memory that holds
wants a test sets, records each report, gives the last report back as
`Pulled::record`, and lets go what a test names. `src/shared.rs`: the
cases every implementation passes, each a function generic over a
harness that makes a plane holding given wants:

- a pull gives the wants in order;
- a reported answer and carried place come back in the next pull's
  record;
- a second report of the same record changes nothing the harness can
  see;
- a run's `wanted` is given as the plane holds it.

`Fake` runs them in the crate's own tests.

**2.1** `crates/flywheel/Cargo.toml` gains serde and serde_json (with
`raw_value`) from `[workspace.dependencies]`, and tempfile as a
dev-dependency. `crates/flywheel/src/flume.rs`: a trait `Flume`, with `launch`,
`retry`, `drop`, `priority`, `done`, `status` (all places) and
`status_of` (one place), and `Cli`, its implementation running a
program path, `flume` in the binary. Each call builds its argv as
Decision 5 has it, puts `--json` first, unsets `FLUME_RECIPIENT`, and
reads its answer as Decision 6. The launch writes its brief to a file
under a directory it is handed and removes it after. Tests against a
stub program written into a `tempfile` directory:

- each act's argv, a launch with every flag among them;
- `FLUME_RECIPIENT` unset;
- exit `0` with a fixture;
- exit `1` with `{"refused":{"refusal":{"reason":"…"}}}`, the reason
  taken as given;
- exit `2` with words on standard error;
- a program that is not there.

Capture the fixtures from flume 0.3.6 on a host into
`crates/flywheel/tests/fixtures/flume-0.3.6/`:

- `status --json` of all places, and of one place: at construct, in
  code with tasks, waiting on its caller with `answer`, a role run
  waiting with `delivered`, landed and tidied;
- a launch's answer, and a refusal.

Strip nothing from them. Add `README.md` beside them, saying where,
when and from what each was read. Add the exclude
`"crates/*/tests/fixtures/**"` to `treefmt.nix` with its reason.

**2.2** `crates/flywheel/src/facts.rs`: the few facts the runner reads
from one place's status, each guarded so that a field absent or of
another type reads as none (crew `read.2`, held here too): its
location (`place`), its `mark`, whether its phase is live or making,
landed (with its commit), dropped or done, and whether `waiting_on.why`
is `delivered`. Slice each place's raw text out of a status answer by
`serde_json::value::RawValue`, keyed by location (serde_json's
`raw_value` feature). Tests over every fixture of 2.1.

**3.1** `crates/flywheel/src/carry.rs`: the pass of Decision 4 over a
`&mut dyn ControlPlane`, a `&dyn Flume`, a `&dyn Clones` (the clone
lookup the sibling's keeper implements: a forge address gives
`Ready(dir)`, `NotYet` or `NotNamed`) and the record's path.
`crates/flywheel/Cargo.toml` gains `flywheel-control` (by path), and
`crates/flywheel/moon.yml`'s four tasks take
`/crates/flywheel-control/**/*` among their inputs in the same task.
Launches
per Decisions 5 and 6, and the refusal of a launch lacking stream,
priority or intent. Tests over `Fake`, a fake `Flume` and a fake
`Clones`, one per scenario of `work-into-flume`'s launch requirements.

**3.2** In `carry.rs`, Decision 8: held words, the newest priority,
the drop of a launch never launched, and the refusals of an act whose
`of` is no launch here. Tests: a dependent held then launched once its
After's fixture status reads landed; a priority of a held launch; a
drop of a held launch; a retry of a held launch.

**3.3** Decision 7: the record saved by rename after each act and read
at start; a launch found by its mark. Tests:

- a pass whose fake Flume's retry answers, then whose report fails,
  then a fresh pass from the saved record that makes no second retry;
- a record file missing, the launch's mark in the fake listing,
  answered `launched` with no launch made.

**3.4** Retry, drop and priority of launched places (Decision 5), each
answered per Decision 6. Tests: one per act, and a refusal carried with
Flume's words.

**3.5** Decisions 9 and 10:

- each carried place's text taken from the listing;
- one `status_of` per place the listing lacks, its last status kept
  on a refusal;
- `at` moved only on a change;
- the user's own place in the listing left alone;
- a repository not named refused;
- a clone not yet made left unanswered;
- the keeper told once of a landing.

Tests: one per scenario of `work-into-flume`'s status, configuration
and own-places requirements.

**4.1** `crates/flywheel/src/runs.rs`: Decision 11, step 5 of the pass.
Tests over `Fake` and a fake `Flume`, one per scenario of `role-runs`:

- three wanted passes making one launch;
- not wanted, no launch;
- `delivered` making `flume done`;
- a lost run standing;
- a refused launch recorded, then tried again;
- `unrunnable`;
- a repository not named;
- the launch's argv with no brief and no recipient.

**5.1** `crates/flywheel-crew` as 1.1 has a crate. It depends on
`flywheel-control`, serde, toml, time, and weir-branch and weir-capture
from `{ git = "https://github.com/agentplot/weir", rev =
"0b6eb5abb806a51fec128b01f97c4dcff6170219" }` in
`[workspace.dependencies]`, with a comment as flume's has, and
tempfile as a dev-dependency. Its moon project is `flywheel-crew`,
whose inputs take `/crates/flywheel-control/**/*` too. `devenv.nix`
gains `pkgs.recutils` among `packages`, with a comment saying
weir-branch's writes run its `recfix`.

**5.2** `src/config.rs`: crew's table of Decision 3 from a
`toml::Table`, each refusal naming the key. Tests: the example of
Decision 3; each missing key; a label twice; `state` not `owner/name`;
a `kinds` value that is not a string.

**5.3** `src/read.rs`: Decision 12. The files at `<label>/main`'s tip
by `weir_branch::read` over a `Branch::remote` into the cache;
`requests.rec` parsed by `weir_capture::rec::read`, descriptor records
(those whose fields begin `%`) marking each record set, `Request`
records kept; each for this host given as a `Request`. Tests over a
bare repository a test makes:

- requests of two hosts, only this one's given;
- a launch's every field, a `Brief` over three lines and two `After`;
- a retry with `Submission`;
- an `Act` crew does not write, given as refused;
- a partition whose branch is missing giving nothing while another
  gives its own.

**5.4** `src/write.rs`: Decisions 13 and 14. The three files from the
record, by `weir_capture::rec::write`, with the descriptors of
Decision 14; `weir_branch::write` with `who` `flywheel@<host>`;
`runner.rec` at the first write, its `Since` kept after; nothing
written when unchanged. The record read back from the branch for
`Pulled::record`. Tests over a bare repository:

- the first report making the branch with `runner.rec`;
- a held answer becoming launched in the same record;
- a place's `Status` byte for byte;
- a `Run` with `Words`;
- a second report writing no commit;
- `<label>/main`'s tip unchanged after a report;
- `git for-each-ref` on the remote showing no new ref but the runner's
  branch;
- `recfix --check` on each file written.

**5.5** `src/roles.rs`: Decision 15. `roles.rec` and `asks.rec` parsed
as crew's `records` and `parse` do; `waits_on` across every host's
`places.rec`, its own from the last report; each role record for this
host given as a `Run`. The cases are crew's `role-runs` scenarios and
`tests/py/test_asks.py` at `a5afd9ec`, written as crew's text into the
bare repository:

- a burst of three asks;
- an ask after a turn began;
- every ask answered;
- a stage of another host's place waiting `answer` on the planner;
- a stage that delivers;
- a superseded turn;
- a `role/…` launch landed;
- `on_hold` naming no stage;
- `stalled`;
- a host whose branch does not parse.

**5.6** In `write.rs`, the let-go of Decision 13 from `plan.rec` at the
tip read. Tests:

- a unit's place tidied while its `Unit` is in the plan, kept;
- the bolt landed and the `Unit` gone, let go and named;
- a fix on a bolt gone with its `Bolt`;
- a fix of main kept.

**5.7** In `crates/flywheel-crew/tests/shared.rs`, run
`flywheel-control`'s shared cases with a harness making a bare
repository with `requests.rec` and `roles.rec` written as crew writes
them.

**6.1** `crates/flywheel/src/planes.rs`: the list of Decision 1, `crew`
and `flywheel_crew::open`. `crates/flywheel/Cargo.toml` gains
`flywheel-crew` (by path), and `crates/flywheel/moon.yml`'s four tasks
take `/crates/flywheel-crew/**/*` among their inputs in the same task. The `[control_plane]` table read at start
from the sibling's configuration: `kind`, `every`, and the table of
`kind` handed to the opener. A word not in the list is refused naming
the list. A pass runs on the sibling's timer every `every`; the
keeper implements `carry::Clones`. Tests: the example of Decision 3
opening crew's implementation; `hosted` refused naming `crew`; `every`
read in each of Flume's time forms.

**6.2** `crates/flywheel/tests/rules.rs`, which reads the tree from
`CARGO_MANIFEST_DIR`:

- core.2: no file under `crates/flywheel/src` but `planes.rs`, and none
  under `crates/flywheel-control`, names `flywheel_crew`; and
  `crates/flywheel-control/Cargo.toml` and
  `crates/flywheel/Cargo.toml`'s dependencies are as Decision 1 has
  them;
- core.3: no file under any `crates/*/src` names `TcpListener`,
  `UnixListener`, `UdpSocket` or `.bind(`;
- core.7: every directory under `crates/` is a project in
  `.moon/workspace.yml`, with a `moon.yml` declaring `language: 'rust'`
  and the tasks format, lint, typecheck and test, each tagged
  `check-fast`. Each task's inputs name the sources of every crate it
  builds: `crates/flywheel/moon.yml`'s take `/crates/flywheel-control/**/*`
  and `/crates/flywheel-crew/**/*`, and `crates/flywheel-crew/moon.yml`'s
  take `/crates/flywheel-control/**/*`. So a change to either runs the
  checks that build it again, never replaying a stale pass.

**6.3** `crates/flywheel/tests/pass.rs`: one pass end to end, with
crew's implementation over a bare state repository, a stub `flume` and
a fake `Clones`:

- crew's give of `a` and `b`, `b` after `a`, writes two launches;
- one pass answers `a` launched and `b` held, with `a`'s place in
  `places.rec`;
- the stub's status then says `a` landed;
- the next pass answers `b` launched with `--base bolt/x` and
  `--stream`, `--priority` and `--intent` in its argv.

**6.4** README: what the runner carries, the `[control_plane]` table
with Decision 3's example in a fenced `toml` block, and the runner's
branch. CLAUDE.md's opening paragraph: what the crate now does. A test
in `crates/flywheel/tests/readme.rs` reads README's `toml` block under
its control-plane heading and opens crew's implementation's
configuration from it.

**7.1** The last task runs only the projects this change touches.
