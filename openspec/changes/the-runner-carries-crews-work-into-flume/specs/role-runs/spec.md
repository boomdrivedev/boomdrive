# Role runs

## Purpose

The runs the control plane wants kept on the host, each a place that
one launch makes: a crew role's run among them, made from its role
record (crew's `roles-run-as-flume-kinds`, R1 to R4). The runner keeps
at most one of each, launched while the control plane wants it,
declared done once its stage has delivered, and recorded by its key.
Whether a run is wanted is the control plane's to say (core.1).

## ADDED Requirements

### Requirement: At most one run of each, launched while it is wanted

For each run a pull gives, the runner SHALL launch it only while it is
wanted and no place of it stands, a place standing when its record's
place or a place bearing its mark reads live or making in Flume's
status. A run whose place stands SHALL NOT be launched again, whatever
its stage.

#### Scenario: Wanted three passes running

- **WHEN** a run is wanted at three passes in a row and its first launch makes a place that stays live
- **THEN** one `flume launch` of it is run, at the first pass

#### Scenario: Not wanted

- **WHEN** a run is given not wanted, and no place of it stands
- **THEN** no `flume launch` of it is run

#### Scenario: A lost run

- **WHEN** a run is wanted and its place reads live with its stage `lost`
- **THEN** no `flume launch` of it is run

### Requirement: A run is launched with no brief and no recipient

A run's launch SHALL be `flume launch` in the runner's clone of its
repository, naming `--kinds` where given, `--kind`, `--name`,
`--stream`, `--priority`, `--intent`, each agent setting and the
runner's mark, with no `--brief`, no `--brief-file` and no
`--recipient`.

#### Scenario: The planner's run

- **WHEN** a run of kind `crew-planner`, named `madswan`, in `afterthought/blueprints`, with kinds file `<crew's kinds>`, stream `madswan-roles`, priority 0, intent `madswan-planner` and model `claude-fable-5-1`, is wanted and none stands
- **THEN** the stub `flume` is run as `flume --json launch --repository <the clone> --kinds <crew's kinds> --kind crew-planner --name madswan --stream madswan-roles --priority 0 --intent madswan-planner --agent model=claude-fable-5-1 --mark flywheel:madswan/planner`, with no brief, no recipient and no `FLUME_RECIPIENT`

### Requirement: A run whose stage delivered is declared done

A run whose place waits on its caller because its last stage delivered
and its kind does not land SHALL be declared done by
`flume done <place>`.

#### Scenario: A turn delivered

- **WHEN** a run's place reads `waiting_on` with `why` `delivered`
- **THEN** the stub `flume` is run as `flume --json done <the place>`

### Requirement: A run's place is recorded by its key

The runner SHALL record each run's latest place, and Flume's status of
it as Flume gave it, by the run's key, and carry it each pass as it
carries a launched place.

#### Scenario: Recorded by the address

- **WHEN** the planner's run of madswan is launched
- **THEN** the runner's record holds, under the key `madswan/planner`, the place Flume named and its status

### Requirement: A run that cannot be launched says why

A run whose launch Flume refuses, whose repository the configuration
does not name, or which the control plane gives as unrunnable, SHALL be
recorded with those words and no place. A refused run still wanted
SHALL be tried again at the next pass.

#### Scenario: Flume refuses the run

- **WHEN** a wanted run's launch is refused with `the place, its branch or its directory already stands`
- **THEN** the run's record holds those words and no place, and the next pass, the run still wanted, runs its launch again

#### Scenario: No kinds file for it

- **WHEN** the control plane gives a run as unrunnable, saying the configuration names no kinds file for `crew`
- **THEN** the run's record holds those words, and no `flume` command names it
