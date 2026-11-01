# Work into Flume

## Purpose

How the runner takes the launches its control plane offers and turns each into one `flume launch` in its clone of the launch's repository. It takes them only under their leases, and only while the Flume stream each runs in has room. It turns each act on a launch into Flume's own command on the launch's place, and carries out each act once. It carries back each answer, the host's capacity, and Flume's status of every place it launched, as Flume gave them, under each launch's lease number. It decides nothing the control plane or Flume decides, and reaches Flume only through Flume's command line (core.1, core.9 to core.11; flywheel-cloud records 0003, 0009; ledger item c-flume.a).

## ADDED Requirements

### Requirement: Every pull carries the host's tools, repositories and leases

The runner SHALL hand every pull a hello holding:

- its interface's version, and `flywheel --version`'s text, as the runner's version;
- each tool's version: the first line of `flume --version`, `sluice --version`, `herdr --version` and `git --version`, as each printed it, or the tool named missing;
- the address of each repository the configuration names whose clone is made;
- each lease it holds, by id and number.

#### Scenario: Repositories kept

- **WHEN** the configuration names `agentplot/flume` and `afterthought/crew`, and only `agentplot/flume`'s clone is made
- **THEN** the hello names `agentplot/flume` alone

#### Scenario: A tool missing

- **WHEN** no program the runner runs as `herdr` exists
- **THEN** the hello names `herdr` as missing, and the pass goes on

### Requirement: Every pull carries Flume's host part as Flume printed it

The hello's capacity SHALL be Flume's host part, `running`, `bound`, `room` and `streams`, each as Flume printed it in the pass's one `flume --json status`. It SHALL leave out Flume's `pools`, `addresses`, `bases` and `terminal_kept`.

#### Scenario: Streams as Flume printed them

- **WHEN** the stub `flume` prints a listing whose host part holds two streams and a pool named `madswan`
- **THEN** the hello's capacity holds the two streams' text as the stub printed it, and the word `madswan` appears nowhere in the hello

### Requirement: An offer is taken only for a repository the runner keeps

The runner SHALL answer `refused` an offer for a repository the configuration does not name, with words naming the repository and saying this host's configuration does not keep it. It SHALL leave unanswered an offer for a repository whose clone is not yet made. It SHALL run no command for either.

#### Scenario: A repository not kept

- **WHEN** an offer of a launch in `agentplot/sluice` is pulled and the configuration does not name it
- **THEN** it is answered `refused`, its words naming `agentplot/sluice`, and no `flume` command is run

#### Scenario: A clone not yet made

- **WHEN** an offer of a launch in `agentplot/flume` is pulled, the configuration names it, and its clone is not yet made
- **THEN** it is left unanswered, and is launched at the first pass after its clone is made

### Requirement: An offer waits for each launch it comes after

An offer whose `after` names a launch whose place has not read landed in Flume's status SHALL be answered `held`. Its words SHALL name each launch it waits on and where that stands: not yet launched, launched at a place not landed, refused with that refusal's words, or dropped. Its time SHALL move only when its words do. At the first pass at which each such place reads landed, and its stream has room, it SHALL be launched.

#### Scenario: A dependent held, then launched

- **WHEN** offer `b` comes after `a`, `a` is launched, and Flume's status of `a`'s place reads live
- **THEN** `b` is answered `held` with words naming `a` as launched at its place and not landed; and once the status of `a`'s place reads landed, the next pass launches `b`

#### Scenario: A dependent of a dropped launch

- **WHEN** offer `b` comes after `a`, and `a` was dropped before it was launched
- **THEN** `b` stays `held`, its words naming `a` as dropped, so its place will never land

### Requirement: Room is read from Flume's status and counted down within a pass

The runner SHALL read room from the pass's one `flume --json status`. The host SHALL have room while its `room` is `null` or above zero. A stream SHALL have room while the host has room, its own `room` is `null` or above zero, and its `queue` is empty. Each launch the pass makes SHALL count one against its stream's room and the host's.

#### Scenario: Room for one

- **WHEN** two offers name stream `work`, whose `room` is 1 and whose `queue` is empty
- **THEN** the first is launched, and the second is left unanswered

#### Scenario: A stream with a queue

- **WHEN** an offer names stream `work`, whose `room` is 2 and whose `queue` holds a place
- **THEN** it is left unanswered, and no `flume` command is run for it

#### Scenario: A host with no room

- **WHEN** an offer names stream `work`, whose `room` is `null`, and the host's `room` is 0
- **THEN** it is left unanswered

#### Scenario: Unbounded

- **WHEN** an offer names stream `default`, whose `room` and the host's are both `null`
- **THEN** it is launched

### Requirement: A launch is made only while the stream it runs in has room

A launch that names a stream SHALL be launched only while that stream has room, and one that names none only while the host has room. Where Flume's status lists no stream of the name, the launch SHALL be made, so that Flume's refusal is its answer. An offer with no room SHALL be left unanswered, or left `held` with words naming the stream it waits for room in.

#### Scenario: A stream the host does not declare

- **WHEN** an offer names stream `nosuch`, which Flume's status does not list, and the stub `flume` refuses the launch naming `nosuch`
- **THEN** the offer is answered `refused` with Flume's reason as given

#### Scenario: No stream named

- **WHEN** an offer names no stream, and the host's `room` is 0
- **THEN** it is left unanswered; and when the host's `room` is 1, it is launched with no `--stream`

### Requirement: A launch is one `flume launch` naming what the launch gives

The runner SHALL turn a launch into one `flume --json launch --repository <the clone's container> --kind <kind> --name <name>`. It SHALL add `--base`, `--stream`, `--priority`, `--intent`, each `--recipient`, each `--hold`, `--hold-submission`, `--brief-file` and each `--agent` only where the launch gives it, and `--mark flywheel:<id>`. It SHALL add no stream, priority or intent of its own.

#### Scenario: A unit's launch

- **WHEN** an offer `madswan/12` of `change` named `a` in `afterthought/crew` on `bolt/x` is pulled, with stream `madswan`, priority 4, intent `madswan/x`, recipient `madswan/planner` and a brief, its clone is made, and its stream has room
- **THEN** the stub `flume` is run once as `flume --json launch --repository <the container> --kind change --name a --base bolt/x --stream madswan --priority 4 --intent madswan/x --recipient madswan/planner --brief-file <a file holding the brief> --mark flywheel:madswan/12`

#### Scenario: A launch naming no stream, priority or intent

- **WHEN** an offer of `fix` named `b` in `agentplot/flume` gives no stream, priority or intent, and the host has room
- **THEN** the stub `flume`'s arguments hold no `--stream`, `--priority` or `--intent`

### Requirement: A launch's kind is passed as the launch gives it, and no kinds file

The runner SHALL pass a launch's kind to `--kind` as the launch gives it, a name or an identity `<source>/<name>@<version>`, and SHALL pass no `--kinds`: where a host's kinds come from is its Flume configuration (core.5).

#### Scenario: A kind named by its identity

- **WHEN** an offer `madswan/planner/1` of kind `crew/crew-planner` named `madswan` in `afterthought/madswan-blueprints` is pulled with the agent setting `model=opus`, and its stream has room
- **THEN** the stub `flume`'s arguments hold `--kind crew/crew-planner` and `--agent model=opus`, and no `--kinds`

### Requirement: Flume runs with no recipient of the runner's own, and no brief is left

The runner SHALL run every `flume` command with `FLUME_RECIPIENT` unset, so a place's recipients are only those its launch names. It SHALL remove a launch's brief file once Flume answers.

#### Scenario: A launch's environment and brief

- **WHEN** a launch with a brief is answered by the stub `flume`
- **THEN** the stub's environment held no `FLUME_RECIPIENT`, and the brief's file no longer exists

### Requirement: Every answer is Flume's

The runner SHALL answer a launch `launched`, at the place Flume's answer names, where `flume` exits 0, and an act `done`. Where `flume` exits 1, it SHALL answer `refused` with the `reason` of Flume's `refused` answer as given. On any other exit, or an answer that is not JSON, it SHALL answer `refused` with Flume's standard error as given. Where `flume` cannot be run at all, it SHALL keep no answer, name why, and try again at the next pass.

#### Scenario: Flume refuses

- **WHEN** the stub `flume` exits 1 printing `{"refused":{"refusal":{"reason":"no stream nosuch on this host"}}}`
- **THEN** the launch is answered `refused` with the words `no stream nosuch on this host`

#### Scenario: Flume is not there

- **WHEN** the program the runner runs as `flume` does not exist
- **THEN** the launch has no answer, the pass names the failure, and the next pass launches it once `flume` is there

### Requirement: A retry, a drop, a priority and a done are Flume's own commands on the launch's place

The runner SHALL act on the place its report holds for the act's launch, never passing `--force`:

- a retry is `flume retry <place>`, with `--stage` and `--words` where the act names them, and `--submission` where it asks for the submission;
- a drop is `flume drop <place>`;
- a priority is `flume priority <place> <n>`;
- a done is `flume done <place>`.

#### Scenario: A retry with words

- **WHEN** an act retries launch `madswan/12` at stage `code` with the words `use the second option`, and `madswan/12` was launched at place `P`
- **THEN** the stub `flume` is run as `flume --json retry P --stage code --words "use the second option"`, and the act is answered `done`

#### Scenario: A done

- **WHEN** an act declares launch `madswan/planner/2` done, and it was launched at place `P`
- **THEN** the stub `flume` is run as `flume --json done P`

### Requirement: A priority or a drop of a launch not yet launched is answered from its report

An act whose launch the runner holds unlaunched SHALL run no command. A priority SHALL be answered `done`, and the launch, when made, SHALL carry the newest priority among its own and its acts', in the control plane's order. A drop SHALL be answered `done`, and the launch SHALL never be made, its answer `done` with words naming the drop.

#### Scenario: A priority of a held launch

- **WHEN** launch `b` is held on its After with priority 2, and an act sets its priority to 7
- **THEN** the act is answered `done`, and `b`, once launched, is launched with `--priority 7`

#### Scenario: A drop of a held launch

- **WHEN** launch `b` is held, and an act drops it
- **THEN** the act is answered `done`, `b` is answered `done` with words naming the drop, and no `flume launch` is ever run for `b`

### Requirement: A retry or a done of a launch not launched, and an act on another's launch, are refused

A retry or a done of a launch the runner holds unlaunched SHALL be answered `refused`, naming where the launch stands. An act whose launch is no launch of this runner's SHALL be answered `refused`, naming the launch. Neither SHALL run a command.

#### Scenario: A retry of a held launch

- **WHEN** launch `b` is held on its After, and an act retries it
- **THEN** the act is answered `refused`, its words saying `b` is held and naming what it waits on

#### Scenario: An act on another's launch

- **WHEN** an act names launch `madswan/99`, which the runner's report does not hold
- **THEN** it is answered `refused`, naming `madswan/99`, and no `flume` command is run

### Requirement: Each launch, act and command is carried out once

The runner SHALL keep its report in `<data>/control/report.json`, written whole by rename after every act and command and before the next, and read at start. A want its report holds settled SHALL be answered from its report and not carried out again. An offer with no answer, whose mark `flywheel:<id>` Flume's listing holds on a place, SHALL be answered `launched` at that place and not launched again. Where the report file is missing, the runner SHALL take the report the pull gives back.

#### Scenario: A retry not made twice

- **WHEN** a pass retries a place, its report then fails, and the runner starts again from its saved report with the same act pulled
- **THEN** no second `flume retry` is run, and the act is reported `done`

#### Scenario: A launch found by its mark

- **WHEN** the report file is missing, and an offer `madswan/12` is pulled whose mark `flywheel:madswan/12` Flume's listing holds on place `P`
- **THEN** it is answered `launched` at `P`, and no `flume launch` is run

### Requirement: A lease is renewed by every report and let go once refused

Every report SHALL name each lease the runner holds with its work, and so renew it. Where a report's answer refuses an id because its lease is no longer the runner's, the runner SHALL drop it from its report, act on it no more, report it no more, run nothing against its place, and say in one line which place it leaves on the host (core.10).

#### Scenario: A lease refused

- **WHEN** the fake control plane refuses the report of launch `a` at place `P` as no longer this runner's, and later pulls an act on `a`
- **THEN** `a` leaves the report, a line names `P` as left on the host, the act on `a` is answered `refused`, and no `flume` command names `P`

#### Scenario: Renewed before it lapses

- **WHEN** the runner holds a lease expiring 40 seconds after the end of the pass that took it, and `poll_interval` is one minute
- **THEN** the next control pass is due no later than 20 seconds after that pass ended

### Requirement: Flume's status of each place is carried as Flume gave it

For each launch with a place, the runner SHALL carry `flume --json status --repository <container> <place>`'s whole answer as Flume printed it, on one line, under the launch's lease number. It SHALL ask for that answer again, and move its time, only when the place's entry in the pass's listing differs, as a JSON value, from the entry inside the carried answer.

#### Scenario: The status byte for byte

- **WHEN** a launched place's entry in the listing changes, and the stub `flume` prints a fixture for `status --json <place>`
- **THEN** the carried status is the fixture's text, byte for byte, on one line, and its time is the pass's

#### Scenario: Only the host changed

- **WHEN** a pass's listing differs from the last only in its host part
- **THEN** no `flume status <place>` is run, and the report is unchanged

### Requirement: A place the listing no longer holds keeps its last status

A place the pass's listing no longer holds SHALL be asked for once a pass by its location. Where Flume answers, its answer SHALL be carried. Where Flume refuses, the last carried status SHALL stand, marked as not listed.

#### Scenario: A place tidied

- **WHEN** the listing no longer holds a carried place, and the stub `flume` refuses `status <place>`
- **THEN** its last carried status stands, marked as not listed

### Requirement: A landing is told to the keeper once

The first pass that reads a carried place landed SHALL tell the keeper of a landing on its repository, so the clone and its view are fetched at once.

#### Scenario: A landing told once

- **WHEN** a carried place's status first reads landed
- **THEN** the keeper is told of a landing on its repository once, and not at later passes

### Requirement: The runner acts only on its own places

The runner SHALL carry and act on only the places its report holds, or whose mark Flume's listing gives as `flywheel:<id>` for an offer it holds. Every other place on the host SHALL be read in the listing and left alone.

#### Scenario: The user's own place

- **WHEN** Flume's listing holds a place with no mark beside the runner's own
- **THEN** no `flume` command names it, and no report carries it

### Requirement: The runner says each act, one line each

The control pass SHALL write one line on standard error for each launch, act, refusal, lease let go and command, prefixed with its id. A control pass in which nothing changed SHALL write nothing.

#### Scenario: A launch said

- **WHEN** a pass launches `madswan/12` at place `P`
- **THEN** it writes one line beginning `madswan/12:` and naming `P`

#### Scenario: A quiet pass

- **WHEN** a pass finds nothing new to take, act on or carry
- **THEN** it writes nothing
