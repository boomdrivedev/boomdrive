## ADDED Requirements

### Requirement: A preview request is a preview of what it names

A `preview` request naming this host SHALL be given as a preview with the id `<label>/<Request>` and the lease number 1, with no expiry: its repository its `Repo`, resolved as a launch naming this host resolves it, its `Branch` and its `Revision`. It SHALL be done where its `Done` reads `yes`, `true` or `1`, and not done where it reads `no`, `false` or `0`, or where it has none (core.55; crew record 0056).

#### Scenario: A preview

- **WHEN** request 30 says `Act: preview`, naming this host, `Repo: crew`, `Branch: main` and `Revision: R1`, and the hello names `example/crew`
- **THEN** the pull gives a preview `home/30` of `main` at `R1` in `example/crew`, not done, under lease 1

#### Scenario: A preview marked done

- **WHEN** request 31 says `Act: preview`, naming this host, with the same `Repo`, `Branch` and `Revision` as request 30, and `Done: yes`
- **THEN** the pull gives a preview `home/31` of `main` at `R1` in `example/crew`, done

### Requirement: A preview request crew's implementation cannot read is refused by its host's runner

A `preview` request naming this host that lacks its `Branch` or its `Revision`, or whose `Done` reads anything but `yes`, `true`, `1`, `no`, `false` or `0`, SHALL be given as one the runner refuses, with words naming the field.

#### Scenario: A preview with no revision

- **WHEN** request 32 says `Act: preview`, naming this host, with no `Revision`
- **THEN** the pull gives it as one the runner answers `refused`, with words naming `Revision`

### Requirement: A preview naming no host is given nothing

A `preview` request naming no `Host` SHALL be given nothing, written nothing of, and named in the log, since a preview is brought up only on the host its control plane names and two runners keeping its repository would each bring it up.

#### Scenario: A preview naming no host

- **WHEN** request 33 says `Act: preview`, naming no `Host`, of a repository the hello names
- **THEN** the pull gives nothing for it, the runner's branch holds no answer to it, and a line names request 33 as a preview that names no Host

### Requirement: A preview the runner answered is the runner's

A preview the runner's report holds, or this host's `previews.rec` answers, SHALL be the runner's, and SHALL be given while its work has left crew's plan, as any request the runner answered is.

#### Scenario: A preview whose work has left the plan

- **WHEN** the runner's report holds preview `home/30` of `unit/u`, answered `done`, and `plan.rec` holds no `Unit: u`
- **THEN** the pull gives the preview `home/30`

### Requirement: A preview's answer is written in previews.rec

crew's implementation SHALL write `previews.rec` on the runner's branch, under crew's descriptor of a `Preview`, one `Preview` per preview request the report answers. Each SHALL hold its `Request` and `Outcome`; sluice's answer as carried as its `Text`; its `Words`; sluice's listing of it as carried as its `Status`, with `Listed` `yes` or `no`; and its `At`, the status's time where a status is carried, else the answer's. The file SHALL pass `recfix --check`.

#### Scenario: A preview brought up

- **WHEN** the runner reports preview `home/30` answered `done`, its text and its status carried listed
- **THEN** `home/runner/host-a`'s `previews.rec` opens with crew's descriptor of a `Preview`, byte for byte, and holds one `Preview` with `Request: 30`, `Outcome: done`, `Text` and `Status` as carried, byte for byte, `Listed: yes` and the status's time as its `At`, and the file passes `recfix --check`

### Requirement: previews.rec is written only where a preview is or was

crew's implementation SHALL write `previews.rec` where the partition's report holds a preview, and as the descriptor alone where the tip holds one and the report none. Otherwise the runner's branch SHALL hold no `previews.rec`.

#### Scenario: A branch with no preview

- **WHEN** the runner first reports for `home`, its report holding a launch and no preview
- **THEN** `home/runner/host-a` holds no `previews.rec`

### Requirement: A preview is read back from previews.rec

`previews.rec` SHALL be read with the runner's other files at each pull, and checked as they are: no record before its `%rec`, and no request twice. Where the runner hands the pull no report of its own, the report the pull gives back SHALL hold each `Preview` of this host's branch as the preview its request names, under lease 1. Its repository SHALL be resolved as at a pull, its outcome, text and words as written, and its status the `Status`, at its `At`, listed as `Listed` says.

#### Scenario: A preview read back

- **WHEN** preview `home/30` has been reported `done`, its status listed, and a pull is handed no report
- **THEN** the pull's report holds `home/30` answered `done`, its text, its status listed and its repository as reported

### Requirement: A preview is let go once its work has left crew's plan and it holds no preview up

At each report, a preview the report holds SHALL leave the runner's files, and be named in the report's let-go, once its work has left `plan.rec` at `<label>/main`'s tip and it holds no preview up: a done, a preview refused, or a preview brought up or held whose status sluice's listing no longer holds. A preview whose status sluice still lists SHALL stay, whatever the plan holds.

#### Scenario: A preview put away, its work gone

- **WHEN** the runner's report holds preview `home/30` of `unit/u` answered `done`, its status not listed, and its done `home/31` answered `done`, and `plan.rec` holds no `Unit: u`
- **THEN** the report's let-go names `home/30` and `home/31`, and `home/runner/host-a`'s `previews.rec` holds neither

#### Scenario: A preview still up, its work gone

- **WHEN** the runner's report holds preview `home/30` of `unit/u` answered `done`, its status listed, and `plan.rec` holds no `Unit: u`
- **THEN** the report's let-go does not name `home/30`, and `previews.rec` still holds it
