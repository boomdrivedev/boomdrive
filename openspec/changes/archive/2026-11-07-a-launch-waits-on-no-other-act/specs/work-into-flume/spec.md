# Work into Flume

## MODIFIED Requirements

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

## ADDED Requirements

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
