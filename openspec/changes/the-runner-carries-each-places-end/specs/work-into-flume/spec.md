# work-into-flume

## RENAMED Requirements

- FROM: `### Requirement: Flume runs with no recipient of the runner's own, and no brief is left`
- TO: `### Requirement: Every launch names the runner its caller, with no recipient of the runner's own, and no brief is left`

## MODIFIED Requirements

### Requirement: Every launch names the runner its caller, with no recipient of the runner's own, and no brief is left

The runner SHALL run every `flume` command with `FLUME_RECIPIENT` unset, so a place's recipients are only those its launch names, or Flume's default where it names none. Every `flume launch` SHALL name the runner its caller, `--caller boomdrive`, so Flume holds the place's end for the runner after its tidy. The caller SHALL be no recipient: the runner SHALL add no `--recipient`, and leave out none the launch gives. It SHALL remove a launch's brief file once Flume answers (core.15, core.53; record 0030).

#### Scenario: A launch's environment and brief

- **WHEN** a launch with a brief is answered by the stub `flume`
- **THEN** the stub's environment held no `FLUME_RECIPIENT`, and the brief's file no longer exists

#### Scenario: Every launch names the runner its caller

- **WHEN** an offer that names the recipient `home/planner` is launched, and then an offer that names no recipient
- **THEN** both launches' arguments hold `--caller boomdrive` once; the first holds `--recipient home/planner` and no other `--recipient`, and the second holds no `--recipient`

### Requirement: A launch is one `flume launch` naming what the launch gives

The runner SHALL turn a launch into one `flume --json launch --repository <the clone's container> --kind <kind> --name <name>`. It SHALL add `--base`, `--stream`, `--priority`, `--intent`, each `--recipient`, each `--hold`, `--hold-submission`, `--brief-file` and each `--agent` only where the launch gives it, then `--mark boomdrive:<id>` and `--caller boomdrive`. It SHALL add no stream, priority or intent of its own.

#### Scenario: A unit's launch

- **WHEN** an offer `home/12` of `change` named `a` in `example/crew` on `bolt/x` is pulled, with stream `home`, priority 4, intent `home/x`, recipient `home/planner` and a brief, its clone is made, and its stream has room
- **THEN** the stub `flume` is run once as `flume --json launch --repository <the container> --kind change --name a --base bolt/x --stream home --priority 4 --intent home/x --recipient home/planner --brief-file <a file holding the brief> --mark boomdrive:home/12 --caller boomdrive`

#### Scenario: A launch naming no stream, priority or intent

- **WHEN** an offer of `fix` named `b` in `agentplot/flume` gives no stream, priority or intent, and the host has room
- **THEN** the stub `flume`'s arguments hold no `--stream`, `--priority` or `--intent`

### Requirement: A bolt is a launch

The runner SHALL take a launch of a kind that lands with no stages, as a bolt's base place is, as it takes any launch: under its lease, for a repository it keeps, while its stream has room as Flume's status gives it, counted against that room within the pass, and as one `flume launch` naming what the launch gives. It SHALL read no kind to tell such a launch from another, and SHALL ask no tool to make, move or remove a branch (core.9, core.15, core.52; Flume's 202).

#### Scenario: A bolt and its unit

- **WHEN** a pull offers `home/bolt/3` of kind `bolt` named `x` in `example/crew` on `main`, then `home/14` of kind `change` named `a` on `bolt/x`, both with stream `home`, whose `room` is 2, and the clone is made
- **THEN** the stub `flume` is run as `flume --json launch --repository <the container> --kind bolt --name x --base main --stream home --mark boomdrive:home/bolt/3 --caller boomdrive`, then as `flume --json launch --repository <the container> --kind change --name a --base bolt/x --stream home --mark boomdrive:home/14 --caller boomdrive`; both are answered `launched`, and no `sluice` command is run

#### Scenario: A bolt counts against its stream's room within its pass

- **WHEN** a pull offers the bolt `home/bolt/3`, then its unit `home/14`, both with stream `home`, whose `room` is 1
- **THEN** the bolt is launched and the unit is left unanswered; and at the next pass, whose listing gives `home` a `room` of 1 again, the unit is launched

## ADDED Requirements

### Requirement: A place's end is held for the runner until its report holds it

Where Flume holds a place's record for the runner after its tidy, `<location>@<at>`, the runner SHALL carry it as the place's status, as it carries any. Once the report file holds that status, in the control pass that carried it, the runner SHALL collect the record by `flume --json inbox --caller boomdrive <location>@<at>`, naming no repository, and SHALL read the answer only by Flume's exit, carrying and keeping nothing of it (core.11, core.53; record 0030).

#### Scenario: A landing held for the runner

- **WHEN** the runner holds launch `a` at place `P`, carried live, and between two passes `P` lands and Flume tidies it, holding its record `P@<at>` for `boomdrive` alone
- **THEN** the next pass carries Flume's answer for `P` as `a`'s status, naming `P@<at>`, tidied and landed, then runs `flume --json inbox --caller boomdrive P@<at>` once, and the report file held that status when it ran; and at the pass after, Flume refusing `P`'s status, that status stands, not listed, and no inbox is run

#### Scenario: A drop held for the runner

- **WHEN** an act drops launch `a` at place `P`, and Flume tidies `P`, holding its record `P@<at>` for `boomdrive`
- **THEN** the pass carries `P@<at>`, tidied and dropped, as `a`'s status, and collects it once

### Requirement: Only a record kept for the runner is collected

The runner SHALL collect a record only for a launch the pull gives, whose repository's clone stands, and whose carried status is listed and names `boomdrive` among those its record is kept for. It SHALL collect nothing for a launch whose lease the control plane refused, nor in a pass whose report could not be saved (core.10, core.25).

#### Scenario: A record kept for a recipient too

- **WHEN** Flume holds `P@<at>` for `boomdrive` and for `scratch/planner`, and the runner collects it
- **THEN** at the next pass `P@<at>` is carried as Flume gives it, kept for `scratch/planner` alone, listed, and no second inbox is run

#### Scenario: A record kept for no end of the runner's

- **WHEN** the runner carries `P@<at>`, tidied and landed, kept for `scratch/planner` alone, as for a place launched before the runner named itself its caller
- **THEN** no inbox is run for it

#### Scenario: A lease refused before its end was collected

- **WHEN** the fake control plane refuses the report of launch `a` at place `P` as no longer this runner's, and Flume then tidies `P`, holding `P@<at>` for `boomdrive`, while the pull still gives `a`
- **THEN** `a` leaves the report, and no inbox names `P@<at>` at the passes after

### Requirement: A collection Flume refuses or cannot run is made again

Where Flume refuses a collection, the runner SHALL say one line, prefixed with the launch's id, naming the record and Flume's words; where Flume cannot be run for it, one line naming the record and why. It SHALL collect the record again at the next pass.

#### Scenario: A collection Flume could not run

- **WHEN** the runner carries `P@<at>` held for `boomdrive`, and `flume` cannot be run for the inbox
- **THEN** the pass says one line beginning `a:` that names `P@<at>` and why; and the next pass, Flume running, collects it

### Requirement: What waits on a place reads its landing from the record Flume holds

An offer whose `after` names a launch whose place Flume has tidied, landed, and holds for the runner SHALL read that place landed from the record carried, and SHALL be launched at the first pass at which that holds and its stream has room, as for any place that reads landed.

#### Scenario: A dependent of a landing held for the runner

- **WHEN** offer `b` comes after `a`, `a` is launched at place `P` and held, and between two passes `P` lands and Flume tidies it, holding `P@<at>` for `boomdrive`
- **THEN** the next pass launches `b`, its listing holding `P@<at>` landed, and carries `P@<at>`, tidied and landed, as `a`'s status

### Requirement: No launch is made with a Flume older than the least

The runner SHALL name the least Flume it runs with: the first release that holds a place's end for its caller. Where the first line of `flume --version`, as the hello reads it, names a version older than that, or names none, the runner SHALL launch no offer: each SHALL be left unanswered, or stay held where it was held. Acts and statuses SHALL go on. Where Flume is missing, a launch SHALL be tried and left unanswered, as where Flume cannot be run (core.38; record 0030).

#### Scenario: A Flume older than the least

- **WHEN** an offer `a` is pulled with room for it, and the program the runner runs as `flume` prints `flume 0.18.0` for `--version`
- **THEN** no `flume launch` is run, and `a` has no answer in the report

#### Scenario: A Flume at the least

- **WHEN** the same offer is pulled again, and `flume --version` now prints the least the runner names
- **THEN** `a` is launched

### Requirement: A Flume older than the least is said once

The runner SHALL say one line naming what Flume said for `--version` and the least it runs with, at the first pass that reads it so, and nothing more of it until Flume's version changes.

#### Scenario: Said once while it lasts

- **WHEN** two passes in turn read `flume 0.18.0` for `--version`, with an offer pulled at each
- **THEN** the first says one line naming `flume 0.18.0` and the least, and the second says nothing of it
