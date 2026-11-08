## ADDED Requirements

### Requirement: The shared cache of a release before is removed

At open, crew's implementation SHALL remove the shared cache a release before kept for a state repository, `<data>/control/crew/<owner>/<name>.git`, where it is there, saying one line naming it. A cache is made again from the control plane (core.35).

#### Scenario: The shared cache of a release before

- **WHEN** crew's implementation opens with `<data>/control/crew/example/state.git` there
- **THEN** that cache is removed, one line names it, and each partition's pull fetches into its own cache

## MODIFIED Requirements

### Requirement: crew's records are read at one tip of each partition's drive branch

At each pull, for each partition, crew's implementation SHALL read `requests.rec`, `roles.rec`, `asks.rec` and `plan.rec` at one tip of `<label>/main`, through a bare cache of that partition's own, `<data>/control/crew/<owner>/<name>/<label>.git`. A file missing at the tip SHALL hold nothing. It SHALL read every `<label>/runner/*` branch the state repository lists, taking its own host's answers and places from the report the runner hands the pull (record 0026).

#### Scenario: A file missing at the tip

- **WHEN** `madswan/main`'s tip holds `requests.rec` and no `roles.rec`
- **THEN** the pull gives the requests for this host, and offers no role's run

#### Scenario: Two partitions of one state repository

- **WHEN** crew's table names the partitions `home` and `work`, both at `example/state`
- **THEN** `home`'s part reads through `<data>/control/crew/example/state/home.git`, `work`'s through `<data>/control/crew/example/state/work.git`, and neither writes in the other's

### Requirement: A partition that cannot be read hides only its own work

Each partition SHALL be a part of its own. A part whose `<label>/main` cannot be read SHALL give `Unread`, with its words, while every other part pulls on its own. A part's report SHALL write its partition's runner branch, with its let-go, and SHALL give `Unsent`, naming the branch, where it could not write it.

#### Scenario: One partition missing

- **WHEN** partition `work`'s state repository has no `work/main` and `home`'s holds requests for this host
- **THEN** `home`'s part gives `home`'s requests, and `work`'s part is unread, naming `work/main`

#### Scenario: No partition read

- **WHEN** no partition's state repository can be read
- **THEN** every part is unread, each naming its own

#### Scenario: One partition missing at a report

- **WHEN** this host is `host-a`, partition `work`'s state repository has no `work/main`, and `home`'s unit `u` is tidied and gone from its plan
- **THEN** `home`'s part writes `home/runner/host-a`, its let-go names `u`'s launch, and `work`'s part's report is unsent, naming `work/runner/host-a`
