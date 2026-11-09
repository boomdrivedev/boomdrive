## RENAMED Requirements

- FROM: `### Requirement: A retry, a drop, a priority and a done are Flume's own commands on the launch's place`
- TO: `### Requirement: A retry, a land, a drop, a priority and a done are Flume's own commands on the launch's place`

- FROM: `### Requirement: A retry or a done of a launch not launched, and an act on another's launch, are refused`
- TO: `### Requirement: A retry, a land or a done of a launch not launched, and an act on another's launch, are refused`

- FROM: `### Requirement: A bolt's send is a retry of its submission`
- TO: `### Requirement: A bolt is landed on a land`

## MODIFIED Requirements

### Requirement: A retry, a land, a drop, a priority and a done are Flume's own commands on the launch's place

The runner SHALL act on the place its report holds for the act's launch, never passing `--force`:

- a retry is `flume retry <place>`, with `--stage` and `--words` where the act names them, and `--submission` where it asks for the submission;
- a land is `flume retry <place> --submission`, naming no stage and no words, whatever the launch's kind (core.53; Flume's 203);
- a drop is `flume drop <place>`;
- a priority is `flume priority <place> <n>`;
- a done is `flume done <place>`.

#### Scenario: A retry with words

- **WHEN** an act retries launch `home/12` at stage `code` with the words `use the second option`, and `home/12` was launched at place `P`
- **THEN** the stub `flume` is run as `flume --json retry P --stage code --words "use the second option"`, and the act is answered `done`

#### Scenario: A land

- **WHEN** an act `home/13` lands launch `home/12`, a launch of kind `change`, and `home/12` was launched at place `P`
- **THEN** the stub `flume` is run as `flume --json retry P --submission` and as no other act, the act is answered `done`, and its line begins `home/13: land of home/12`

#### Scenario: A done

- **WHEN** an act declares launch `home/planner/2` done, and it was launched at place `P`
- **THEN** the stub `flume` is run as `flume --json done P`

### Requirement: A retry, a land or a done of a launch not launched, and an act on another's launch, are refused

A retry, a land or a done of a launch the runner holds unlaunched SHALL be answered `refused`, naming where the launch stands. An act whose launch is no launch of this runner's SHALL be answered `refused`, naming the launch. Neither SHALL run a command, and a land so refused SHALL change nothing of the launch when it is made (core.15, core.53).

#### Scenario: A retry of a held launch

- **WHEN** launch `b` is held on its After, and an act retries it
- **THEN** the act is answered `refused`, its words saying `b` is held and naming what it waits on

#### Scenario: A land of a held launch

- **WHEN** launch `b` is held on its After, an act lands it, and at a later pass `b`'s After reads landed
- **THEN** the land is answered `refused`, its words saying `b` is held and naming what it waits on, no `flume retry` is run, and `b` is then launched as its offer gives it

#### Scenario: An act on another's launch

- **WHEN** an act names launch `home/99`, which the runner's report does not hold
- **THEN** it is answered `refused`, naming `home/99`, and no `flume` command is run

### Requirement: A bolt is landed on a land

The runner SHALL carry out a land of a launch whose kind lands with no stages, as a bolt's base place is, as `flume retry <place> --submission`, as it carries out any land. Flume's answer SHALL be the act's, a refusal while a place on its branch is live among them (core.1, core.11, core.52, core.53; Flume's 203).

#### Scenario: A bolt's send

- **WHEN** an act lands launch `home/bolt/3`, which was launched at place `P`
- **THEN** the stub `flume` is run as `flume --json retry P --submission`, and the act is answered `done`

#### Scenario: A send Flume refuses

- **WHEN** an act lands launch `home/bolt/3` at place `P`, and the stub `flume` exits 1 refusing it with the reason `the places on bolt/x are live: Q`
- **THEN** the act is answered `refused` with the words `the places on bolt/x are live: Q`, and no `sluice` command is run
