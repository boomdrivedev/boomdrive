# work-into-flume

## RENAMED Requirements

- FROM: `### Requirement: Flume runs with no recipient of the runner's own, and no brief is left`
- TO: `### Requirement: The runner is one more recipient of a launch that names recipients, and no brief is left`

## MODIFIED Requirements

### Requirement: The runner is one more recipient of a launch that names recipients, and no brief is left

The runner SHALL run every `flume` command with `FLUME_RECIPIENT` unset. Where a launch names recipients, its `flume launch` SHALL pass them as given and name the runner one more after them, `--recipient boomdrive`, so Flume holds the place's end for it; where it names none, it SHALL name none, so Flume's default stands. It SHALL remove a launch's brief file once Flume answers (core.15, core.54; record 0030).

#### Scenario: A launch's environment and brief

- **WHEN** a launch with a brief is answered by the stub `flume`
- **THEN** the stub's environment held no `FLUME_RECIPIENT`, and the brief's file no longer exists

#### Scenario: The runner is one more recipient

- **WHEN** an offer that names the recipient `home/planner` is launched, and then an offer that names no recipient
- **THEN** the first launch's arguments hold `--recipient home/planner` then `--recipient boomdrive`, and no other `--recipient`; and the second's hold no `--recipient`

### Requirement: A launch is one `flume launch` naming what the launch gives

The runner SHALL turn a launch into one `flume --json launch --repository <the clone's container> --kind <kind> --name <name>`. It SHALL add `--base`, `--stream`, `--priority`, `--intent`, each `--recipient` with `--recipient boomdrive` after them, each `--hold`, `--hold-submission`, `--brief-file` and each `--agent` only where the launch gives it, and `--mark boomdrive:<id>`. It SHALL add no stream, priority or intent of its own.

#### Scenario: A unit's launch

- **WHEN** an offer `home/12` of `change` named `a` in `example/crew` on `bolt/x` is pulled, with stream `home`, priority 4, intent `home/x`, recipient `home/planner` and a brief, its clone is made, and its stream has room
- **THEN** the stub `flume` is run once as `flume --json launch --repository <the container> --kind change --name a --base bolt/x --stream home --priority 4 --intent home/x --recipient home/planner --recipient boomdrive --brief-file <a file holding the brief> --mark boomdrive:home/12`

#### Scenario: A launch naming no stream, priority or intent

- **WHEN** an offer of `fix` named `b` in `agentplot/flume` gives no stream, priority or intent, and the host has room
- **THEN** the stub `flume`'s arguments hold no `--stream`, `--priority` or `--intent`

## ADDED Requirements

### Requirement: A place's end is held for the runner until its report holds it

Where Flume holds a place's record for the runner after its tidy, `<location>@<at>`, the runner SHALL carry it as the place's status, as it carries any. Once the report file holds that status, in the control pass that carried it, the runner SHALL collect the record by `flume --json inbox <location>@<at> --recipient boomdrive`, naming no repository, and SHALL read the answer only by Flume's exit, carrying and keeping nothing of it (core.11, core.54; record 0030).

#### Scenario: A landing held for the runner

- **WHEN** the runner holds launch `a`, which names the recipient `scratch/planner`, at place `P`, carried live, and between two passes `P` lands and Flume tidies it, holding its record `P@<at>` for `boomdrive` alone
- **THEN** the next pass carries Flume's answer for `P` as `a`'s status, naming `P@<at>`, tidied and landed, then runs `flume --json inbox P@<at> --recipient boomdrive` once, and the report file held that status when it ran; and at the pass after, Flume refusing `P`'s status, that status stands, not listed, and no inbox is run

#### Scenario: A drop held for the runner

- **WHEN** an act drops launch `a`, which names the recipient `scratch/planner`, at place `P`, and Flume tidies `P`, holding its record `P@<at>` for `boomdrive`
- **THEN** the pass carries `P@<at>`, tidied and dropped, as `a`'s status, and collects it once

### Requirement: Only a record pending for the runner is collected

The runner SHALL collect a record only for a launch the pull gives, whose repository's clone stands, and whose carried status is listed and reads the place tidied, its `pending_for` naming `boomdrive`. It SHALL collect nothing for a launch whose lease the control plane refused, nor in a pass whose report could not be saved (core.10, core.25).

#### Scenario: A record pending for a recipient too

- **WHEN** Flume holds `P@<at>` for `boomdrive` and for `scratch/planner`, and the runner collects it
- **THEN** at the next pass `P@<at>` is carried as Flume gives it, pending for `scratch/planner` alone, listed, and no second inbox is run

#### Scenario: A record pending for no end of the runner's

- **WHEN** the runner carries `P@<at>`, tidied and landed, pending for `scratch/planner` alone, as for a place launched before the runner named itself a recipient
- **THEN** no inbox is run for it

#### Scenario: A lease refused before its end was collected

- **WHEN** the fake control plane refuses the report of launch `a` at place `P` as no longer this runner's, and Flume then tidies `P`, holding `P@<at>` for `boomdrive`
- **THEN** `a` leaves the report, and no inbox names `P@<at>` at the passes after

### Requirement: A collection Flume refuses or cannot run is made again

Where Flume refuses a collection, the runner SHALL say one line, prefixed with the launch's id, naming the record and Flume's words; where Flume cannot be run for it, one line naming the record and why. It SHALL collect the record again at the next pass.

#### Scenario: A collection Flume could not run

- **WHEN** the runner carries `P@<at>` held for `boomdrive`, and `flume` cannot be run for the inbox
- **THEN** the pass says one line beginning `a:` that names `P@<at>` and why; and the next pass, Flume running, collects it

#### Scenario: A collection Flume refuses

- **WHEN** the runner carries `P@<at>` held for `boomdrive`, and Flume refuses the inbox
- **THEN** the pass says one line beginning `a:` that names `P@<at>` and Flume's words; and the next pass collects it again

### Requirement: What waits on a place reads its landing from the record Flume holds

An offer whose `after` names a launch whose place Flume has tidied, landed, and holds for the runner SHALL read that place landed from the record carried, and SHALL be launched at the first pass at which that holds and its stream has room, as for any place that reads landed.

#### Scenario: A dependent of a landing held for the runner

- **WHEN** offer `b` comes after `a`, `a` is launched at place `P` and held, and between two passes `P` lands and Flume tidies it, holding `P@<at>` for `boomdrive`
- **THEN** the next pass launches `b`, its listing holding `P@<at>` landed, and carries `P@<at>`, tidied and landed, as `a`'s status

### Requirement: No launch is made with a Flume older than the least

The least Flume the runner runs with SHALL be `0.20.0`, the first that holds a drop's and a done's end for a recipient no pane holds. Where the first line of `flume --version`, as the hello reads it, names an older version, or none, the runner SHALL launch no offer, leaving each unanswered, or held where it was held, while acts and statuses go on. Where Flume is missing, a launch SHALL be tried as before (core.38; record 0030).

#### Scenario: A Flume older than the least

- **WHEN** an offer `a` is pulled with room for it, and the program the runner runs as `flume` prints `flume 0.18.0` for `--version`
- **THEN** no `flume launch` is run, and `a` has no answer in the report

#### Scenario: A Flume at the least

- **WHEN** the same offer is pulled again, and `flume --version` now prints `flume 0.20.0`
- **THEN** `a` is launched

### Requirement: A Flume older than the least is said once

The runner SHALL say one line naming what Flume said for `--version` and the least it runs with, at the first pass that reads it so, and nothing more of it until Flume's version changes.

#### Scenario: Said once while it lasts

- **WHEN** two passes in turn read `flume 0.18.0` for `--version`, with an offer pulled at each
- **THEN** the first says one line naming `flume 0.18.0` and `0.20.0`, and the second says nothing of it
