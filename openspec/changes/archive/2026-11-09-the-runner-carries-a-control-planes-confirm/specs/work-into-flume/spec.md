## RENAMED Requirements

- FROM: `### Requirement: A retry, a land or a done of a launch not launched, and an act on another's launch, are refused`
- TO: `### Requirement: A retry, a land, a confirm or a done of a launch not launched, and an act on another's launch, are refused`

## MODIFIED Requirements

### Requirement: A retry, a land, a confirm or a done of a launch not launched, and an act on another's launch, are refused

A retry, a land, a confirm or a done of a launch the runner holds unlaunched SHALL be answered `refused`, naming where the launch stands. An act whose launch is no launch of this runner's SHALL be answered `refused`, naming the launch. Neither SHALL run a command, and a land so refused SHALL change nothing of the launch when it is made (core.15, core.53, core.56).

#### Scenario: A retry of a held launch

- **WHEN** launch `b` is held on its After, and an act retries it
- **THEN** the act is answered `refused`, its words saying `b` is held and naming what it waits on

#### Scenario: A land of a held launch

- **WHEN** launch `b` is held on its After, an act lands it, and at a later pass `b`'s After reads landed
- **THEN** the land is answered `refused`, its words saying `b` is held and naming what it waits on, no `flume retry` is run, and `b` is then launched as its offer gives it

#### Scenario: A confirm of a held launch

- **WHEN** launch `b` is held on its After, and an act confirms an end of it as `scratch/planner`
- **THEN** the confirm is answered `refused` with the words a retry of `b` is given, saying `b` is held and naming what it waits on, and no `flume confirm` is run

#### Scenario: An act on another's launch

- **WHEN** an act names launch `home/99`, which the runner's report does not hold
- **THEN** it is answered `refused`, naming `home/99`, and no `flume` command is run

## ADDED Requirements

### Requirement: A confirm is Flume's confirm of the keys it names, as the recipient it names

The runner SHALL carry out a confirm as any act on its launch's place, by `flume --json confirm --recipient=<recipient> -- <key>…`, naming the one recipient and exactly the keys the act names, in its order, and no other, so that neither is read as a flag. Flume's answer SHALL be the confirm's, and its line SHALL name it `confirm as <recipient>` (core.1, core.11, core.56; Flume's 43).

#### Scenario: A confirm

- **WHEN** the runner holds launch `a` at place `P`, whose ends' deliveries in the pass's listing are `K1` and `K2`, and an act `a/confirm` confirms `K1` as `scratch/planner`
- **THEN** Flume is run as `flume --json confirm --recipient=scratch/planner -- K1` and as no other act, the act is answered `done`, and the pass says `a/confirm: confirm as scratch/planner of a at P: done`

#### Scenario: A confirm Flume refuses

- **WHEN** the same confirm is run, and Flume refuses it with the words `the delivery key K1 was not confirmed as scratch/planner: it is not owed to scratch/planner`
- **THEN** the act is answered `refused` with Flume's words, as given

#### Scenario: A tidied place's record

- **WHEN** Flume has tidied `P`, keeping its record `P@<at>`, whose end of delivery `K1` is pending for `scratch/planner`, and an act confirms `K1` as `scratch/planner`
- **THEN** Flume's confirm of `K1` as `scratch/planner` is run, and the act is answered `done`

#### Scenario: Neither the recipient nor a key read as a flag

- **WHEN** a confirm names the recipient `-x` and the keys `--recipient=boomdrive` and `K1`
- **THEN** Flume is run as `flume --json confirm --recipient=-x -- --recipient=boomdrive K1`

### Requirement: Each key a confirm names is the delivery of an end of its launch's place

Before it runs a confirm, the runner SHALL read its launch's place as Flume gives it: its entry in the pass's listing, standing or kept tidied, else Flume's answer to `flume --json status <place>` (core.51). A confirm naming a key that is the `delivery` of no end of that entry SHALL be answered `refused`, naming the key and the place, and SHALL run no `flume confirm` (core.10, core.26, core.56).

#### Scenario: A key of another place

- **WHEN** the runner holds launch `a` at place `P`, whose end's delivery is `K1`, and launch `b` at place `Q`, whose end's delivery is `K3`, and an act on `a` confirms `K1` and `K3` as `scratch/planner`
- **THEN** the act is answered `refused`, its words naming `K3` and `P`, and no `flume confirm` is run

#### Scenario: A place the listing does not hold

- **WHEN** the pass's listing does not hold `P`, Flume answers `flume --json status P` with `P`'s entry, whose end's delivery is `K1`, and an act confirms `K1` as `scratch/planner`
- **THEN** Flume's confirm of `K1` is run after that status, and the act is answered `done`

### Requirement: A confirm as the runner, or of no key, is refused

A confirm naming as its recipient the runner's own name, `boomdrive`, whose ends the runner carries and collects for itself, or naming no key, SHALL be answered `refused`, its words saying which, and SHALL run no command for it (core.54, core.56).

#### Scenario: A confirm as the runner

- **WHEN** an act confirms `K1` of launch `a`'s place as `boomdrive`
- **THEN** it is answered `refused`, its words naming `boomdrive`, and no `flume` command is run for it

#### Scenario: A confirm of no key

- **WHEN** an act confirms no key of launch `a`'s place as `scratch/planner`
- **THEN** it is answered `refused`, its words saying it names no delivery key, and no `flume` command is run for it
