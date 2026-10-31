# Work into Flume

## Purpose

How the runner turns each request the control plane makes of its host
into the matching `flume` command, in its clone of the request's
repository, acting on each request once. It carries back each answer,
and Flume's status of every place it launched, as Flume gave them. It
decides nothing the control plane or Flume decides, and talks to Flume
only through Flume's command line (core.1; crew record 0052 part 3).

## ADDED Requirements

### Requirement: A launch is one `flume launch` naming its stream, priority and intent

The runner SHALL turn a launch into one `flume launch --json` in its
clone of the launch's repository. It SHALL name each field the launch
gives by its flag, the brief by `--brief-file`, always `--stream`,
`--priority` and `--intent`, and the runner's mark `flywheel:<key>`,
with `FLUME_RECIPIENT` unset. A launch lacking a stream, a priority or
an intent SHALL be refused naming what it lacks, with no command run.

#### Scenario: A unit's launch

- **WHEN** a launch of `change` named `a` in `afterthought/crew` on `bolt/x`, with stream `madswan`, priority 4, intent `madswan/x`, recipient `madswan/planner` and a brief, is pulled, and its clone is made
- **THEN** the stub `flume` is run once as `flume --json launch --repository <the clone> --kind change --name a --base bolt/x --stream madswan --priority 4 --intent madswan/x --recipient madswan/planner --brief-file <a file holding the brief> --mark flywheel:<its key>`, with no `FLUME_RECIPIENT` in its environment

#### Scenario: A launch with no stream

- **WHEN** a launch is pulled with no stream
- **THEN** it is answered `refused` with words naming the stream, and no `flume` command is run

### Requirement: A retry, a drop and a priority are Flume's own commands on the launch's place

The runner SHALL act on the place its answer to the named launch
records:

- a retry is `flume retry <place>`, with `--stage` and `--words` where
  the request names them, and `--submission` where it asks for the
  submission;
- a drop is `flume drop <place>`;
- a priority is `flume priority <place> <n>`.

None of them SHALL be run with `--force`.

#### Scenario: A retry with words

- **WHEN** a retry of a launched launch names stage `construct` and the words `madswan's`
- **THEN** the stub `flume` is run as `flume --json retry <the launch's place> --stage construct --words madswan's`, and the retry is answered `done`

#### Scenario: A retry of the submission

- **WHEN** a retry of a launched launch asks for its submission
- **THEN** the stub `flume` is run as `flume --json retry <the launch's place> --submission`

### Requirement: An answer is Flume's, as Flume gave it

A launch Flume takes SHALL be answered `launched` at the place Flume
names, and its answer taken as the place's first status. A retry, drop
or priority Flume takes SHALL be answered `done`. An act Flume refuses
SHALL be answered `refused`, its words the reason Flume gave, as
given; a usage error, its words Flume's standard error. A `flume` that
cannot be run SHALL leave the request unanswered, to be tried at the
next pass. The runner SHALL rewrite, summarise or judge no answer.

#### Scenario: Flume refuses a launch

- **WHEN** the stub `flume` exits 1 with `{"refused":{"refusal":{"reason":"the stream night is one the host's configuration does not declare"}}}`
- **THEN** the launch is answered `refused` with the words `the stream night is one the host's configuration does not declare`

#### Scenario: No flume on the host

- **WHEN** the program the runner runs as `flume` is not there
- **THEN** the request has no answer after the pass, and the next pass runs it again

### Requirement: Each request is acted on once

A request answered other than `held` SHALL never be acted on again,
across a restart too. The runner SHALL save its record in its data
directory after each act, before the next, and read it at start. A
launch with no answer whose mark Flume's listing holds SHALL be
answered `launched` at that place, and not launched again.

#### Scenario: A restart after a retry

- **WHEN** a pass runs a retry, its report fails, and the runner starts again from its data directory
- **THEN** the next pass reports the retry `done` and runs no second `flume retry`

#### Scenario: An answer lost with the data directory

- **WHEN** the runner's record is gone, the control plane holds no answer to a launch, and Flume's listing holds a place with that launch's mark
- **THEN** the launch is answered `launched` at that place, and no `flume launch` is run

### Requirement: A launch waits for its After work to land

A launch naming launches it waits for SHALL be answered `held`, its
words naming each of them not yet landed and where it stands. It SHALL
be launched at the first pass whose Flume status shows each of their
places landed on its base. A launch, when made, SHALL carry the newest
priority among its own and those of the priorities naming it.

#### Scenario: A dependent held, then launched

- **WHEN** `b`'s launch waits for `a`'s, and `a`'s place reads live in Flume's status
- **THEN** `b` is answered `held` with words naming `a`'s launch; and at the first pass whose status reads `a`'s place landed, `b` is launched and answered `launched`

#### Scenario: A priority of a held launch

- **WHEN** a priority of 7 names `b`'s launch while it is held
- **THEN** the priority is answered `done`, and `b`'s launch, once made, names `--priority 7`

### Requirement: A launch dropped before it was launched is never made

A drop naming a launch not yet launched SHALL be answered `done`, and
the launch SHALL be answered `done` with words naming the drop, and
never launched. A retry naming a launch not yet launched SHALL be
refused, naming where it stands. A retry, drop or priority naming no
launch this runner answers SHALL be refused, naming what it names.

#### Scenario: A held launch dropped

- **WHEN** a drop names `b`'s launch while it is held
- **THEN** the drop and `b`'s launch are answered `done`, and no `flume launch` of `b` is run then or at any later pass

#### Scenario: A retry of nothing launched here

- **WHEN** a retry names a key that is no launch this runner answers
- **THEN** the retry is answered `refused`, naming that key

### Requirement: Work only in the repositories its configuration names

A launch in a repository the configuration does not name SHALL be
refused, its words naming the repository and that this host's
configuration does not name it. A launch whose repository is named but
whose clone is not yet made SHALL be left unanswered until it is.

#### Scenario: A repository not named

- **WHEN** a launch names `afterthought/swancloud`, which this host's configuration does not name
- **THEN** it is answered `refused`, naming `afterthought/swancloud` and this host's configuration, and no `flume` command is run

#### Scenario: A clone not yet made

- **WHEN** a launch names a repository the configuration names, whose clone is not yet made
- **THEN** it has no answer after the pass, and it is launched at the first pass once its clone is made

### Requirement: Flume's status of every place it launched is carried as Flume gave it

Each pass the runner SHALL read `flume status --json` once, and carry
each place it launched as the text of that place's object in Flume's
answer, byte for byte. The time a carried place was read SHALL move
only when that text changes. A place Flume's listing no longer holds
SHALL be asked for by its location; where Flume refuses, its last
status SHALL stand. The first pass that reads a carried place landed
SHALL tell the keeper of the clones that its repository landed.

#### Scenario: A place in code

- **WHEN** Flume's listing holds a launched place in code with three of seven tasks ticked
- **THEN** that place's carried status is its object's text from the listing, byte for byte, with nothing of the host's part

#### Scenario: Nothing changed

- **WHEN** two passes read the same text of a carried place
- **THEN** its carried status and the time it was read are the same after both

#### Scenario: A tidied place

- **WHEN** a carried place is in Flume's listing at one pass, gone from it at the next, and `flume status --json <its location>` is refused
- **THEN** its carried status is the last one read

### Requirement: The runner acts only on its own places

The runner SHALL act on and carry only places it launched: those its
record holds, and those Flume's listing gives its mark. Every other
place on the host SHALL be left alone.

#### Scenario: A place of the user's own

- **WHEN** Flume's listing holds a live place the runner did not launch, with no mark of the runner's
- **THEN** no command names it, and the runner's record carries nothing of it

### Requirement: No connection comes in to the host

No crate of the runner SHALL open a listening socket. Every connection
the runner makes SHALL be outgoing.

#### Scenario: The sources hold no listener

- **WHEN** every file under `crates/*/src` is read
- **THEN** none names `TcpListener`, `UnixListener`, `UdpSocket` or `.bind(`
