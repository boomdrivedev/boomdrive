# work-into-flume

## RENAMED Requirements

- FROM: `### Requirement: A place's end is held for the runner until its report holds it`
- TO: `### Requirement: A place's ends are owed to the runner until its control plane has taken them`

Each end a carried status gives pending for `boomdrive`, of a live place or of a record Flume holds after its tidy, SHALL stay owed in Flume until the control plane has taken a report holding that status. The runner SHALL then confirm those ends by `flume --json confirm <key>… --recipient boomdrive`, once for each launch, the keys in the status's order, reading the answer only by Flume's exit and keeping nothing of it. It SHALL run no `flume inbox` (core.11, core.54; record 0030 as amended on 2026-11-09).

### Requirement: A place's ends are owed to the runner until its control plane has taken them

The runner SHALL carry each place's status as it carries any, a record Flume holds after the place's tidy, `<location>@<at>`, among them. Each end the status gives pending for `boomdrive`, of a live place or of a held record, SHALL stay owed in Flume until the control plane has taken a report holding that status. Only once a report the pass sent is answered, the answer naming the launch's status taken, and the report file holds the part's share after that answer, SHALL the runner confirm those ends. It SHALL run `flume --json confirm <key>… --recipient boomdrive` once for each launch, naming the delivery key of each such end in the status's order and no repository, and SHALL read the answer only by Flume's exit, carrying and keeping nothing of it. It SHALL run no `flume inbox` and no `flume wait` (core.11, core.54; record 0030 as amended on 2026-11-09; weir record 0006; Flume record 0061).

#### Scenario: A landing held for the runner

- **WHEN** the runner holds launch `a`, which names the recipient `scratch/planner`, at place `P`, carried live, and between two passes `P` lands and Flume tidies it, holding its record `P@<at>` with its landing pending for `boomdrive` alone
- **THEN** the next pass carries Flume's answer for `P` as `a`'s status, naming `P@<at>`, tidied and landed, reports it, and, the fake control plane's answer naming `a`'s status taken, runs `flume --json confirm <the landing's key> --recipient boomdrive` once and no `flume inbox`; and at the pass after, Flume refusing `P`'s status, that status stands, not listed, and nothing is confirmed

#### Scenario: A drop held for the runner

- **WHEN** an act drops launch `a`, which names the recipient `scratch/planner`, at place `P`, and Flume tidies `P`, holding `P@<at>` with its end `over/dropped` pending for `boomdrive`
- **THEN** the pass carries `P@<at>`, tidied and dropped, as `a`'s status, and once the answer to its report names that status taken, confirms the drop's key once

#### Scenario: A live place's end

- **WHEN** the runner holds launch `a` at place `P`, live, and Flume gives `P` a stage's end pending for `boomdrive` and `scratch/planner`
- **THEN** the pass carries the status and, once the answer to its report names it taken, confirms that end's key as `boomdrive`; and the next pass, Flume giving the end pending for `scratch/planner` alone, confirms nothing

#### Scenario: A report not taken

- **WHEN** Flume holds `P@<at>` with its landing pending for `boomdrive`, and the fake control plane refuses the pass's report
- **THEN** nothing is confirmed and `P@<at>` stays pending for `boomdrive`; and the next pass, its report taken, confirms the landing's key

#### Scenario: A status not taken

- **WHEN** Flume holds `P@<at>` with its landing pending for `boomdrive`, and the fake control plane takes the pass's report but leaves `a`'s status
- **THEN** nothing of `P@<at>` is confirmed; and at a later pass whose answer names `a`'s status taken, its landing's key is confirmed

#### Scenario: The report file lost

- **WHEN** the fake control plane took a report carrying `a` at `P` live, `P` then lands and is tidied, held for `boomdrive`, the next pass's report is refused, and the report file is then removed
- **THEN** the pass after takes `a` from its pull, carries `P@<at>` from Flume as `a`'s status, still pending for `boomdrive`, and confirms its landing's key once the answer to its report names that status taken

### Requirement: No launch is made with a Flume older than the least

The least Flume the runner runs with SHALL be `0.21.0`, the first whose ends a recipient confirms by key. Where the first line of `flume --version`, as the hello reads it, names an older version, or none, the runner SHALL launch no offer, leaving each unanswered, or held where it was held, while acts and statuses go on. Where Flume is missing, a launch SHALL be tried as before (core.38; record 0030 as amended on 2026-11-09).

#### Scenario: A Flume older than the least

- **WHEN** an offer `a` is pulled with room for it, and the program the runner runs as `flume` prints `flume 0.20.0` for `--version`
- **THEN** no `flume launch` is run, and `a` has no answer in the report

#### Scenario: A Flume at the least

- **WHEN** the same offer is pulled again, and `flume --version` now prints `flume 0.21.0`
- **THEN** `a` is launched

### Requirement: A Flume older than the least is said once

The runner SHALL say one line naming what Flume said for `--version` and the least it runs with, at the first pass that reads it so, and nothing more of it until Flume's version changes.

#### Scenario: Said once while it lasts

- **WHEN** two passes in turn read `flume 0.20.0` for `--version`, with an offer pulled at each
- **THEN** the first says one line naming `flume 0.20.0` and `0.21.0`, and the second says nothing of it

## REMOVED Requirements

### Requirement: Only a record pending for the runner is collected

**Reason**: Under Flume `0.21.0` an inbox hands an end over and confirms nothing, and the runner confirms each end owed to it by its key once its control plane has taken it, a live place's among them, in place of collecting a tidied record once its report file holds it (record 0030 as amended on 2026-11-09; Flume record 0061).

**Migration**: The work-into-flume spec's "Only an end owed to the runner, in a status its control plane took, is confirmed" says which ends are confirmed, and when none is.

### Requirement: A collection Flume refuses or cannot run is made again

**Reason**: The runner runs no `flume inbox`, so there is no collection to make again (record 0030 as amended on 2026-11-09).

**Migration**: The work-into-flume spec's "A confirmation Flume refuses or cannot run is made again" says how a confirmation Flume refused, or could not run, is said and made again.

## ADDED Requirements

### Requirement: Only an end owed to the runner, in a status its control plane took, is confirmed

The runner SHALL confirm only ends of a launch the pull gives, answered `launched` at a place whose clone stands, whose carried status is listed and named taken in the answer to the pass's report, the report file holding the part's share after that answer. It SHALL confirm nothing of a launch whose lease was refused or which was let go, nor while Flume is older than the least (core.10, core.25, core.38).

#### Scenario: A record pending for a recipient too

- **WHEN** Flume holds `P@<at>` with its landing pending for `boomdrive` and for `scratch/planner`, and the runner confirms it
- **THEN** at the next pass `P@<at>` is carried as Flume gives it, pending for `scratch/planner` alone, listed, and nothing more is confirmed

#### Scenario: A record pending for no end of the runner's

- **WHEN** the runner carries `P@<at>`, tidied and landed, its landing pending for `scratch/planner` alone, as for a place launched before the runner named itself a recipient
- **THEN** nothing is confirmed for it

#### Scenario: A lease refused before its end was confirmed

- **WHEN** the fake control plane refuses the report of launch `a` at place `P` as no longer this runner's, and Flume then tidies `P`, holding `P@<at>` for `boomdrive`
- **THEN** `a` leaves the report, and no confirmation names a key of `P@<at>` at the passes after

#### Scenario: Nothing confirmed below the least

- **WHEN** Flume holds `P@<at>` with its landing pending for `boomdrive`, and the program the runner runs as `flume` prints `flume 0.20.0` for `--version`
- **THEN** the pass carries `P@<at>` as `a`'s status and reports it, and runs no `flume confirm`

### Requirement: A confirmation Flume refuses or cannot run is made again

Where Flume refuses a confirmation, the runner SHALL say one line, prefixed with the launch's id, naming the place and Flume's words; where Flume cannot be run for it, one line naming the place and why. It SHALL confirm the ends the carried status still gives pending for `boomdrive` again at a later pass whose report is taken.

#### Scenario: A confirmation Flume could not run

- **WHEN** the runner carries `P@<at>` held for `boomdrive`, its report is taken, and `flume` cannot be run for the confirmation
- **THEN** the pass says one line beginning `a:` that names `P@<at>` and why; and the next pass, Flume running, confirms the landing's key

#### Scenario: A confirmation Flume refuses

- **WHEN** the runner carries `P@<at>` held for `boomdrive`, its report is taken, and Flume refuses the confirmation
- **THEN** the pass says one line beginning `a:` that names `P@<at>` and Flume's words; and the next pass confirms the landing's key again
