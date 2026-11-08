## MODIFIED Requirements

### Requirement: Every pull carries the host's tools, repositories and leases

The runner SHALL hand every pull a hello holding:

- its interface's version, and `boomdrive --version`'s text, as the runner's version;
- each tool's version: the first line of `flume --version`, `sluice --version`, `herdr --version` and `git --version`, as each printed it, or the tool named missing;
- the address of each repository the configuration names whose clone is made;
- each lease it holds, by id and number.

#### Scenario: Repositories kept

- **WHEN** the configuration names `agentplot/flume` and `example/crew`, and only `agentplot/flume`'s clone is made
- **THEN** the hello names `agentplot/flume` alone

#### Scenario: A tool missing

- **WHEN** no program the runner runs as `herdr` exists
- **THEN** the hello names `herdr` as missing, and the pass goes on

### Requirement: Every pull carries Flume's host part as Flume printed it

The hello's capacity SHALL be Flume's host part, `running`, `bound`, `room` and `streams`, each as Flume printed it in the pass's one `flume --json status`. It SHALL leave out Flume's `pools`, `addresses`, `bases` and `terminal_kept`.

#### Scenario: Streams as Flume printed them

- **WHEN** the stub `flume` prints a listing whose host part holds two streams and a pool named `pool-a`
- **THEN** the hello's capacity holds the two streams' text as the stub printed it, and the word `pool-a` appears nowhere in the hello

### Requirement: A launch is one `flume launch` naming what the launch gives

The runner SHALL turn a launch into one `flume --json launch --repository <the clone's container> --kind <kind> --name <name>`. It SHALL add `--base`, `--stream`, `--priority`, `--intent`, each `--recipient`, each `--hold`, `--hold-submission`, `--brief-file` and each `--agent` only where the launch gives it, and `--mark boomdrive:<id>`. It SHALL add no stream, priority or intent of its own.

#### Scenario: A unit's launch

- **WHEN** an offer `home/12` of `change` named `a` in `example/crew` on `bolt/x` is pulled, with stream `home`, priority 4, intent `home/x`, recipient `home/planner` and a brief, its clone is made, and its stream has room
- **THEN** the stub `flume` is run once as `flume --json launch --repository <the container> --kind change --name a --base bolt/x --stream home --priority 4 --intent home/x --recipient home/planner --brief-file <a file holding the brief> --mark boomdrive:home/12`

#### Scenario: A launch naming no stream, priority or intent

- **WHEN** an offer of `fix` named `b` in `agentplot/flume` gives no stream, priority or intent, and the host has room
- **THEN** the stub `flume`'s arguments hold no `--stream`, `--priority` or `--intent`

### Requirement: A launch's kind is passed as the launch gives it, and no kinds file

The runner SHALL pass a launch's kind to `--kind` as the launch gives it, a name or an identity `<source>/<name>@<version>`, and SHALL pass no `--kinds`: where a host's kinds come from is its Flume configuration (core.5).

#### Scenario: A kind named by its identity

- **WHEN** an offer `home/planner/1` of kind `crew/crew-planner` named `home` in `example/home-blueprints` is pulled with the agent setting `model=opus`, and its stream has room
- **THEN** the stub `flume`'s arguments hold `--kind crew/crew-planner` and `--agent model=opus`, and no `--kinds`

### Requirement: A retry, a drop, a priority and a done are Flume's own commands on the launch's place

The runner SHALL act on the place its report holds for the act's launch, never passing `--force`:

- a retry is `flume retry <place>`, with `--stage` and `--words` where the act names them, and `--submission` where it asks for the submission;
- a drop is `flume drop <place>`;
- a priority is `flume priority <place> <n>`;
- a done is `flume done <place>`.

#### Scenario: A retry with words

- **WHEN** an act retries launch `home/12` at stage `code` with the words `use the second option`, and `home/12` was launched at place `P`
- **THEN** the stub `flume` is run as `flume --json retry P --stage code --words "use the second option"`, and the act is answered `done`

#### Scenario: A done

- **WHEN** an act declares launch `home/planner/2` done, and it was launched at place `P`
- **THEN** the stub `flume` is run as `flume --json done P`

### Requirement: A retry or a done of a launch not launched, and an act on another's launch, are refused

A retry or a done of a launch the runner holds unlaunched SHALL be answered `refused`, naming where the launch stands. An act whose launch is no launch of this runner's SHALL be answered `refused`, naming the launch. Neither SHALL run a command.

#### Scenario: A retry of a held launch

- **WHEN** launch `b` is held on its After, and an act retries it
- **THEN** the act is answered `refused`, its words saying `b` is held and naming what it waits on

#### Scenario: An act on another's launch

- **WHEN** an act names launch `home/99`, which the runner's report does not hold
- **THEN** it is answered `refused`, naming `home/99`, and no `flume` command is run

### Requirement: Each launch, act and command is carried out once

The runner SHALL keep its report in `<data>/control/report.json`, written whole by rename after every act and command and before the next, and read at start. A want its report holds settled SHALL be answered from its report and not carried out again. An offer with no answer, whose mark `boomdrive:<id>` Flume's listing holds on a place, SHALL be answered `launched` at that place and not launched again. Where the report file is missing, the runner SHALL take the report the pull gives back.

#### Scenario: A retry not made twice

- **WHEN** a pass retries a place, its report then fails, and the runner starts again from its saved report with the same act pulled
- **THEN** no second `flume retry` is run, and the act is reported `done`

#### Scenario: A launch found by its mark

- **WHEN** the report file is missing, and an offer `home/12` is pulled whose mark `boomdrive:home/12` Flume's listing holds on place `P`
- **THEN** it is answered `launched` at `P`, and no `flume launch` is run

### Requirement: The runner says each act, one line each

The control pass SHALL write one line on standard error for each launch, act, refusal, lease let go and command, prefixed with its id. A control pass in which nothing changed SHALL write nothing.

#### Scenario: A launch said

- **WHEN** a pass launches `home/12` at place `P`
- **THEN** it writes one line beginning `home/12:` and naming `P`

#### Scenario: A quiet pass

- **WHEN** a pass finds nothing new to take, act on or carry
- **THEN** it writes nothing
