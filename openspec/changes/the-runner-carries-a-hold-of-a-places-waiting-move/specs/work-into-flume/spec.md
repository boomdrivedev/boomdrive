## MODIFIED Requirements

### Requirement: A retry, a land, a confirm or a done of a launch not launched, and an act on another's launch, are refused

A retry, a land, a confirm, a hold or a done of a launch the runner holds unlaunched SHALL be answered `refused`, naming where the launch stands. An act whose launch is no launch of this runner's SHALL be answered `refused`, naming the launch. Neither SHALL run a command, and a land or a hold so refused SHALL change nothing of the launch when it is made (core.15, core.53, core.56, core.59).

#### Scenario: A retry of a held launch

- **WHEN** launch `b` is held on its After, and an act retries it
- **THEN** the act is answered `refused`, its words saying `b` is held and naming what it waits on

#### Scenario: A land of a held launch

- **WHEN** launch `b` is held on its After, an act lands it, and at a later pass `b`'s After reads landed
- **THEN** the land is answered `refused`, its words saying `b` is held and naming what it waits on, no `flume retry` is run, and `b` is then launched as its offer gives it

#### Scenario: A confirm of a held launch

- **WHEN** launch `b` is held on its After, and an act confirms an end of it as `scratch/planner`
- **THEN** the confirm is answered `refused` with the words a retry of `b` is given, saying `b` is held and naming what it waits on, and no `flume confirm` is run

#### Scenario: A hold of a held launch

- **WHEN** launch `b` is held on its After, and an act holds the move its place waits on under words for 28800 seconds
- **THEN** the hold is answered `refused` with the words a retry of `b` is given, saying `b` is held and naming what it waits on, and no `flume hold` is run

#### Scenario: An act on another's launch

- **WHEN** an act names launch `home/99`, which the runner's report does not hold
- **THEN** it is answered `refused`, naming `home/99`, and no `flume` command is run

## ADDED Requirements

### Requirement: A hold is Flume's hold of the move its launch's place waits on

The runner SHALL carry out a hold as any act on its launch's place, by `flume --json hold <place> --waiting --for=<n>s -- <words>`, `<n>` being the act's bound in whole seconds and `<words>` exactly the act's words, after `--`, so that they are never read as a flag. Its line SHALL name it `hold for <n>s`, holding none of its words (core.11, core.59; Flume's 130).

#### Scenario: A hold

- **WHEN** the runner holds launch `a` at place `P`, and an act `a/hold` holds the move `P` waits on under the words `the user sends it on the landings card, row 3` for 28800 seconds
- **THEN** Flume is run as `flume --json hold P --waiting --for=28800s -- "the user sends it on the landings card, row 3"` and as no other act, the act is answered `done`, the pass says `a/hold: hold for 28800s of a at P: done`, and no line holds the words

#### Scenario: The words not read as a flag

- **WHEN** a hold names the words `--stage code` and a bound of 0 seconds
- **THEN** Flume is run as `flume --json hold P --waiting --for=0s -- "--stage code"`

### Requirement: Flume's answer is the hold's

The runner SHALL judge nothing of what a hold's place waits on, and SHALL renew and end no hold of its own accord. Flume's answer SHALL be the hold's (core.1, core.11, core.59; Flume's 130).

#### Scenario: A hold Flume refuses

- **WHEN** the runner holds launch `a` at place `P`, an act holds the move `P` waits on, and Flume refuses it with the words `the place P waits on no move of its caller's`
- **THEN** the act is answered `refused` with Flume's words, as given

### Requirement: A hold with no words is refused

A hold whose words are empty SHALL be answered `refused`, its words saying it says nothing of who makes the move, and SHALL run no command for it (core.59; Flume's 130).

#### Scenario: A hold with no words

- **WHEN** an act holds the move launch `a`'s place waits on under empty words for 28800 seconds
- **THEN** it is answered `refused`, its words saying it says nothing of who makes the move, and no `flume` command is run for it
