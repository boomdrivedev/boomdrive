# work-into-flume Specification

## Purpose
How the runner takes the launches its control plane offers and turns each into one `flume launch` in its clone of the launch's repository. It takes them only under their leases, and only while the Flume stream each runs in has room. It turns each act on a launch into Flume's own command on the launch's place, and carries out each act once. It carries back each answer, the host's capacity, and Flume's status of every place it launched, as Flume gave them, under each launch's lease number. It decides nothing the control plane or Flume decides, and reaches Flume only through Flume's command line (core.1, core.9 to core.11; boomdrive-cloud records 0003, 0009; ledger item c-flume.a).

## Requirements

### Requirement: Every pull carries the host's tools, repositories and leases

The runner SHALL hand every pull a hello holding:

- its interface's version, and `boomdrive --version`'s text, as the runner's version;
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

The hello's capacity SHALL be Flume's host part, `running`, `bound`, `room` and `streams`, each as Flume printed it in the pass's listing, the `flume --json status` it reads before its pull. It SHALL leave out Flume's `pools`, `addresses`, `bases` and `terminal_kept`.

#### Scenario: Streams as Flume printed them

- **WHEN** the stub `flume` prints a listing whose host part holds two streams and a pool named `madswan`
- **THEN** the hello's capacity holds the two streams' text as the stub printed it, and the word `madswan` appears nowhere in the hello

### Requirement: An offer is taken only for a repository the runner keeps

The runner SHALL answer `refused` an offer for a repository it neither keeps nor drains, with words naming the repository and saying this host's configuration does not keep it, whatever Flume's listing holds under the offer's mark. It SHALL leave unanswered an offer for a repository the configuration names whose clone is not yet made. It SHALL run no command for either.

#### Scenario: A repository not kept

- **WHEN** an offer of a launch in `agentplot/sluice` is pulled and the configuration does not name it
- **THEN** it is answered `refused`, its words naming `agentplot/sluice`, and no `flume` command is run

#### Scenario: A clone not yet made

- **WHEN** an offer of a launch in `agentplot/flume` is pulled, the configuration names it, and its clone is not yet made
- **THEN** it is left unanswered, and is launched at the first pass after its clone is made

#### Scenario: A mark in a repository not kept

- **WHEN** an offer `a` in `example/old-kit` is pulled, the runner neither keeps nor drains `example/old-kit`, and Flume's listing holds a place marked `boomdrive:a`
- **THEN** `a` is answered `refused`, its words naming `example/old-kit`, and no `flume status <place>` is run

### Requirement: An offer waits for each launch it comes after

An offer whose `after` names a launch whose place has not read landed in Flume's status SHALL be answered `held`. Its words SHALL name each launch it waits on and where that stands: not yet launched, launched at a place not landed, refused with that refusal's words, or dropped. Its time SHALL move only when its words do. At the first pass at which each such place reads landed, and its stream has room, it SHALL be launched.

#### Scenario: A dependent held, then launched

- **WHEN** offer `b` comes after `a`, `a` is launched, and Flume's status of `a`'s place reads live
- **THEN** `b` is answered `held` with words naming `a` as launched at its place and not landed; and once the status of `a`'s place reads landed, the next pass launches `b`

#### Scenario: A dependent of a dropped launch

- **WHEN** offer `b` comes after `a`, and `a` was dropped before it was launched
- **THEN** `b` stays `held`, its words naming `a` as dropped, so its place will never land

### Requirement: Room is read from Flume's status and counted down within a pass

The runner SHALL read room from the pass's listing, the `flume --json status` it reads before its pull. The host SHALL have room while its `room` is `null` or above zero. A stream SHALL have room while the host has room, its own `room` is `null` or above zero, and its `queue` is empty. Each launch the pass makes SHALL count one against its stream's room and the host's.

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

The runner SHALL turn a launch into one `flume --json launch --repository <the clone's container> --kind <kind> --name <name>`. It SHALL add `--base`, `--stream`, `--priority`, `--intent`, each `--recipient`, each `--hold`, `--hold-submission`, `--brief-file` and each `--agent` only where the launch gives it, and `--mark boomdrive:<id>`. It SHALL add no stream, priority or intent of its own.

#### Scenario: A unit's launch

- **WHEN** an offer `madswan/12` of `change` named `a` in `afterthought/crew` on `bolt/x` is pulled, with stream `madswan`, priority 4, intent `madswan/x`, recipient `madswan/planner` and a brief, its clone is made, and its stream has room
- **THEN** the stub `flume` is run once as `flume --json launch --repository <the container> --kind change --name a --base bolt/x --stream madswan --priority 4 --intent madswan/x --recipient madswan/planner --brief-file <a file holding the brief> --mark boomdrive:madswan/12`

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

The runner SHALL keep its report in `<data>/control/report.json`, written whole by rename after every act and command and before the next, and read at start. A want its report holds settled SHALL be answered from its report and not carried out again. An offer with no answer, whose mark `boomdrive:<id>` Flume's listing holds on a place, SHALL be answered `launched` at that place and not launched again. Where the report file is missing, the runner SHALL take the report the pull gives back.

#### Scenario: A retry not made twice

- **WHEN** a pass retries a place, its report then fails, and the runner starts again from its saved report with the same act pulled
- **THEN** no second `flume retry` is run, and the act is reported `done`

#### Scenario: A launch found by its mark

- **WHEN** the report file is missing, and an offer `madswan/12` is pulled whose mark `boomdrive:madswan/12` Flume's listing holds on place `P`
- **THEN** it is answered `launched` at `P`, and no `flume launch` is run

### Requirement: A launch marked before the rename is found by its old mark

An offer with no answer, whose place Flume's listing marks `flywheel:<id>`, as a runner marked it before it was renamed, SHALL be answered `launched` at that place and not launched again, just as one marked `boomdrive:<id>` (record 0016; core.10).

#### Scenario: A launch found by its mark from before the rename

- **WHEN** an offer `scratch/4` is pulled with no answer, and Flume's listing holds the mark `flywheel:scratch/4` on place `P`
- **THEN** it is answered `launched` at `P`, the line naming the mark `flywheel:scratch/4`, and no `flume launch` is run

### Requirement: A lease is renewed by every report and let go once refused

Every report SHALL name each lease the runner holds with its work, and so renew it. Where a report's answer refuses an id because its lease is no longer the runner's, the runner SHALL drop it from its report, act on it no more, report it no more, run nothing against its place, and say in one line which place it leaves on the host (core.10).

#### Scenario: A lease refused

- **WHEN** the fake control plane refuses the report of launch `a` at place `P` as no longer this runner's, and later pulls an act on `a`
- **THEN** `a` leaves the report, a line names `P` as left on the host, the act on `a` is answered `refused`, and no `flume` command names `P`

#### Scenario: Renewed before it lapses

- **WHEN** the runner holds a lease expiring 40 seconds after the end of the pass that took it, and `poll_interval` is one minute
- **THEN** the next control pass is due no later than 20 seconds after that pass ended

### Requirement: Flume's status of each place is carried as Flume gave it

For each launch with a place whose repository's clone stands, whether the configuration names the repository or the runner drains it, the runner SHALL carry Flume's status of the place as Flume printed it, on one line, under the launch's lease number: `flume --json status --repository <container> <place>`'s whole answer. It SHALL ask for that answer again, and move its time, only when the place's entry in the statuses' listing differs, as a JSON value, from the entry inside the carried answer.

#### Scenario: The status byte for byte

- **WHEN** a launched place's entry in the listing changes, and the stub `flume` prints a fixture for `status --json <place>`
- **THEN** the carried status is the fixture's text, byte for byte, on one line, and its time is the pass's

#### Scenario: Only the host changed

- **WHEN** a pass's listing differs from the last only in its host part
- **THEN** no `flume status <place>` is run, and the report is unchanged

### Requirement: A place the listing no longer holds keeps its last status

A place the statuses' listing no longer holds SHALL be asked for once a pass by its location. Where Flume answers, its answer SHALL be carried. Where Flume refuses, the last carried status SHALL stand, marked as not listed.

#### Scenario: A place tidied

- **WHEN** the listing no longer holds a carried place, and the stub `flume` refuses `status <place>`
- **THEN** its last carried status stands, marked as not listed

### Requirement: A landing is told to the keeper once

The first pass that reads a carried place landed SHALL tell the keeper of a landing on its repository, so the clone and its view are fetched at once. A landing in a repository the configuration does not name, one the runner drains among them, or whose clone is not yet made, SHALL be told to no keeper.

#### Scenario: A landing told once

- **WHEN** a carried place's status first reads landed
- **THEN** the keeper is told of a landing on its repository once, and not at later passes

#### Scenario: A landing in a repository not kept

- **WHEN** a carried place whose repository the runner drains first reads landed
- **THEN** its status is carried, and the keeper is told nothing

### Requirement: The runner acts only on its own places

The runner SHALL carry and act on only the places its report holds, or whose mark Flume's listing gives as `boomdrive:<id>`, or `flywheel:<id>` from before the rename, for an offer it holds. Every other place on the host SHALL be read in the listing and left alone.

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

### Requirement: Work under a lapsed lease waits for a live one

By the runner's clock at the start of a control pass, an offer whose lease has expired SHALL be neither launched, held nor refused, an act on a launch whose lease in the report has expired SHALL not be carried out, and a command whose lease has expired SHALL not be taken. Each SHALL wait until a pull gives it under a live lease. An answer already given SHALL stand (core.10, core.24).

#### Scenario: An offer whose lease lapsed

- **WHEN** the fake control plane offers launch `a` under a lease that expired a minute before the pass
- **THEN** no `flume launch` is run, `a` has no answer in the report, and once a pull offers `a` under a live lease it is launched

#### Scenario: An act on a lapsed launch

- **WHEN** the runner holds launch `a` at place `P` under a lease that has expired, and the fake control plane asks a retry of `a`
- **THEN** no `flume` command names `P`, and the retry has no answer in the report

#### Scenario: A command whose lease lapsed

- **WHEN** the fake control plane gives a branch command under a lease that has expired
- **THEN** no `sluice` command is run, and the command has no answer in the report

### Requirement: The statuses are carried against Flume's listing as it stands once the wants are answered

The statuses' listing SHALL be Flume's listing as it stands once the pull's wants are answered. Where answering them ran a `flume launch`, an act of Flume's or a `sluice branch create`, it SHALL be a `flume --json status` read then. Where it ran none, it SHALL be the pass's listing (record 0027).

#### Scenario: A place tidied while the pass answers its wants

- **WHEN** a carried place `P` stands live in the pass's listing, as carried, and while the pass launches another offer Flume tidies `P` landed, keeping its record `P@<at>`
- **THEN** Flume's listing is read again after the launch, and that pass carries Flume's answer for `P` as its status, naming `P@<at>`, tidied and landed

#### Scenario: A quiet pass reads the listing once

- **WHEN** a pass's wants run no `flume` or `sluice` command
- **THEN** it runs `flume --json status` once, before its pull

### Requirement: A pull's launches go ahead of its acts on places

The control pass SHALL answer a pull's wants in two rounds, each in the control plane's order. The first SHALL take every offer, inbox and command, and answer every act that runs no command, leaving each act that runs Flume's command on its launch's place for the second. No launch SHALL wait behind an act on a place of its own pull (core.50; record 0002, as amended on 2026-11-07).

#### Scenario: A launch ahead of acts on places that are over

- **WHEN** the runner holds launch `a` at place `P`, Flume no longer stands `P`, and a pull gives a priority and a retry of `a`, then the offer `a`, then a new offer `b`
- **THEN** `b` is launched first; then `flume --json status P` is read for each act, and each is answered `refused` with Flume's words naming `P`, no `flume priority P` or `flume retry P` being run; and the next pass runs no `flume` command for either act

### Requirement: No answer turns on the round a want is taken in

The launches a pass makes SHALL be those the control plane's order makes. An act answered from the report SHALL be answered where the control plane's order puts it, and an act on a launch the first round took SHALL be carried out in the second (core.50).

#### Scenario: An act ahead of its own launch

- **WHEN** a pull gives a priority of `a`, then the offer `a`, and `a` is launched that pass at place `P`
- **THEN** `flume priority P` is run in the same pass, after the launch, and the act is answered `done`

#### Scenario: A drop ahead of a held launch

- **WHEN** launch `b` is held on its After, and a pull gives a drop of `b`, then the offer `b`, at the pass at which `b`'s After reads landed
- **THEN** the drop is answered `done`, `b` is answered `done` with words naming the drop, and no `flume launch` is run for `b`

### Requirement: A launch waits on no act of the pass before its own

Once the next control pass is due, by its timer or by the control plane's wake, the second round SHALL run no act's command but its first, SHALL leave each act after it to that pass, and SHALL say how many it left. `boomdrive run --once`, after which no pass is due, SHALL carry out every act (core.50; record 0002, as amended on 2026-11-07).

#### Scenario: Acts on places wait for the next pass once it is due

- **WHEN** the runner holds launches `a` and `b` at places Flume stands, a pull gives a priority of `a` and a priority of `b`, then the offers `a`, `b` and a new offer `c`, and the next control pass is due once one act's command has run
- **THEN** `c` is launched, then `a`'s priority is run on its place; `b`'s is left unanswered, and the pass says that 1 act on a place waits for the next; and at the next pass, whose pull also offers a new `d`, `d` is launched before `b`'s priority is run on its place

### Requirement: An act on a place Flume does not stand costs the reading of Flume's refusal

Before it runs an act's command on a place the pass's listing does not hold, the runner SHALL read `flume --json status <place>`, naming no repository. Flume's refusal SHALL be the act's answer, `refused` with Flume's words, and the act's command SHALL NOT be run; a status Flume gives SHALL have the command run; where Flume could not be run, the act SHALL be left to the next pass (core.51; record 0010, as amended on 2026-11-07).

#### Scenario: An act on a place the listing does not hold, which Flume stands

- **WHEN** the runner holds launch `a` at place `P`, the pass's listing does not hold `P`, Flume gives `flume --json status P`, and a pull gives the offer `a`, then a priority of `a`
- **THEN** `flume priority P` is run, and the act is answered `done`

#### Scenario: A place whose status Flume could not be read for

- **WHEN** the runner holds launch `a` at place `P`, Flume no longer stands `P` and cannot be run for `flume --json status P`, and a pull gives the offer `a`, then a drop of `a`
- **THEN** no `flume drop P` is run, and the drop has no answer and is named not carried out; and at the next pass, Flume refusing `P`'s status, the drop is answered `refused` with Flume's words

### Requirement: The hello names the repositories drained apart from those kept

Apart from its repositories, the hello SHALL name the address of each repository the runner drains, as the clones spec has it, and SHALL not name it among its repositories.

#### Scenario: A repository draining in the hello

- **WHEN** the configuration no longer names `example/kit`, its clone is made, and the report holds launch `a` launched at `P` in `example/kit`, whose carried status reads `P` live
- **THEN** the hello names `example/kit` as drained, and not among its repositories

### Requirement: Nothing more is launched in a repository drained

Where the runner drains a repository, an offer there whose place Flume's listing holds under its mark SHALL be answered `launched` at that place, and any other SHALL be refused as one in a repository not kept. It SHALL run no command for either.

#### Scenario: A repository drained

- **WHEN** the runner drains `example/kit` for launch `a`, and an offer `b` of a launch in `example/kit` is pulled, with room on the host
- **THEN** `b` is answered `refused`, its words naming `example/kit`, and no `flume launch` is run

### Requirement: A place whose repository has no clone is asked of Flume no more

For a launch whose repository's clone does not stand, the runner SHALL ask Flume nothing of its place, and the status last carried SHALL stand as it was.

#### Scenario: A place in a repository with no clone

- **WHEN** the pull gives launch `a`, launched at `P` in `example/old-kit`, the runner has no clone of `example/old-kit`, and `P`'s entry in the listing changes
- **THEN** no `flume status P` is run, and `a`'s carried status is as it was

### Requirement: A place in a repository drained is followed to its end through its clone

Where the configuration drops a repository while a place the runner launched there has not ended, the runner SHALL carry that place's status, as each pull gives its launch, through the repository's clone, which it drains, until the place has ended, as the clones spec has it. No old name of a repository SHALL be matched to a new one: a launch is followed only through the clone of the repository it names.

#### Scenario: A place in a repository drained

- **WHEN** the report holds launch `a` launched at `P` in `example/kit`, the configuration no longer names `example/kit`, its clone stands, the pull still gives `a`, and `P` lands, then is tidied while an end of it is pending
- **THEN** each time, `flume --json status --repository <container> P` is run with `example/kit`'s container, and its answer is carried: first `P` landed, then its record `P@<at>` tidied and landed

### Requirement: Each launch's repository is kept with its answer

The runner's report SHALL hold, with each launch it answers, the repository the offer named, as the offer wrote it, from the pass that first answers it, and SHALL never change it. A launch the report holds with no repository SHALL gain the offer's at the first pull that gives it.

#### Scenario: A launch's repository kept with its answer

- **WHEN** offer `a` of a launch in `agentplot/flume` is launched, and a report file of before holds launch `b` launched with no repository
- **THEN** the report file's `a` names `agentplot/flume`, and its `b` names the repository of `b`'s offer once a pull gives `b`
