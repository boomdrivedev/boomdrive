## Purpose

How the runner brings up the preview its control plane asks for, of a branch at a revision, in a repository it keeps, through sluice's own preview command; how it carries sluice's answer and listing of the preview, with no output of the preview's command; and how it puts away, on its control plane's done, only a preview it asked sluice for. sluice keeps the preview, its checkout, its stack, its states and its wait at the host's bound; the runner waits on that bound no longer than its patience, so no launch and no other drive's work waits behind it (core.1, core.10, core.11, core.13, core.49, core.50, core.55; record 0031; crew's record 0056; sluice's 98 to 103 and 150).

## ADDED Requirements

### Requirement: A preview is brought up by sluice's own preview command

For a preview not marked done, under a live lease, in a repository the runner keeps with its clone made, the runner SHALL run `sluice preview <branch> <revision> --json` in the clone's container, the branch and revision as the preview names them. Where sluice exits 0 with a preview's document, the preview SHALL be answered `done`, carrying what sluice's answer gives. Where sluice refuses, it SHALL be answered `refused`, with sluice's words (core.55).

#### Scenario: A preview brought up

- **WHEN** a pull gives the preview `home/30` of `main` at `R1` in `agentplot/flume`, and the stub `sluice` answers `preview` with `preview-starting.json`, exit 0
- **THEN** the stub's one `preview` call is `preview main R1 --json`, run in `agentplot/flume`'s container; `home/30` is answered `done`, its text the fixture's `name`, `branch`, `revision`, `state`, `made` and `addresses`; and a line reads `home/30: preview of main at R1: brought up, starting`

#### Scenario: A preview sluice refuses

- **WHEN** a pull gives the preview `home/30`, and the stub `sluice` answers `preview` with `preview-no-table.json`, exit 2
- **THEN** `home/30` is answered `refused` with the message of the fixture's `error`, and a line reads `home/30: preview of main at R1: refused by sluice: ` followed by it

### Requirement: A preview is answered once

A preview the report holds `done` or `refused` SHALL be answered from the report, under the lease the pull gives now, and sluice SHALL NOT be asked for it again, across a restart too (core.10).

#### Scenario: A preview given twice

- **WHEN** `home/30` is brought up at a pass, the runner restarts, and the next pull gives `home/30` again
- **THEN** the second pass runs no `sluice preview`, and reports `home/30`'s answer with its time as it stood

### Requirement: A preview sluice holds waiting is asked again at the next pass

The runner SHALL await `sluice preview` no longer than its patience, 20 seconds, then send it SIGINT and await its exit. Where sluice then exits with no document, the preview SHALL be answered `held`, with no text. A preview held SHALL be asked of sluice again at each pass, until sluice answers it or its done puts it away. The line saying it waits SHALL be said only when it is first held (core.49, core.50, core.55).

#### Scenario: A preview waiting at sluice's bound

- **WHEN** the runner's patience is one second, the stub `sluice` waits at `preview` until it is interrupted, then says it stopped and exits 130, and a pull gives the preview `home/30`
- **THEN** the pass ends within a few seconds; `home/30` is answered `held`; a line reads `home/30: preview of main at R1: waits at sluice's bound, asked again at the next pass`; and at the next pass, the stub answering `preview-starting.json`, `sluice preview main R1 --json` is run again and `home/30` is answered `done`, with no second line saying it waits

### Requirement: A preview is brought up only in a repository the runner keeps

A preview not marked done in a repository the runner neither keeps nor drains, or in one it drains, SHALL be answered `refused`, with the words `this host's configuration does not keep <repository>`. One in a repository the configuration names whose clone is not yet made SHALL be left unanswered. No `sluice` command SHALL be run for either (core.5).

#### Scenario: A preview in a repository not kept

- **WHEN** a pull gives a preview in `example/elsewhere`, which the configuration does not name
- **THEN** it is answered `refused` with the words `this host's configuration does not keep example/elsewhere`, and no `sluice` command is run

#### Scenario: A preview in a repository whose clone is not yet made

- **WHEN** a pull gives a preview in a repository the configuration names whose clone is not yet made
- **THEN** it is left unanswered, and no `sluice` command is run

### Requirement: What is carried of a preview holds no output of its command

Of a preview's document, sluice's answer or a row of its listing, the runner SHALL carry a JSON object of its `name`, `branch`, `revision`, `state`, `made` and `addresses`, in that order, each where sluice printed it, each value's text as sluice printed it, and nothing more. Of a release's document it SHALL carry `preview` and `released`, each as sluice printed it (core.11).

#### Scenario: A failed preview's output left out

- **WHEN** `previews.json`'s row of a failed preview, holding its command's `output`, is carried
- **THEN** the text carried holds its `state` `failed`, its `name` and its `addresses` as the fixture prints them, and no `output` and no text of it

### Requirement: sluice's listing is read once a pass for each repository with a preview held

After the pass's wants are answered, the runner SHALL read `sluice previews --json` once, in the clone's container of a repository named or dropped, for each repository of a preview the pull gives, not marked done, that the report holds `held` or `done`, and whose status is not carried as unlisted (core.55).

#### Scenario: One listing for two previews of one repository

- **WHEN** a pull gives two previews of `agentplot/flume` the report holds `done`
- **THEN** the pass runs `sluice previews --json` once

### Requirement: A preview's status is the row sluice lists of it

The runner SHALL carry as a preview's status the row of sluice's listing whose `name` is the one sluice's answer gave, or, for a preview held with no answer, the row whose `revision` begins with the preview's revision. The status's time SHALL move only when its text does (core.11).

#### Scenario: A preview comes up

- **WHEN** `home/30` is brought up `starting`, and at the pass's end the stub answers `previews` with `previews.json`, in which its preview is `up`
- **THEN** `home/30`'s status holds `state` `up` and its addresses, listed

### Requirement: A preview sluice no longer lists keeps its last status

Where sluice's listing holds no row of a preview, its status SHALL stand, not listed, and SHALL be read no more. Where the listing cannot be read, each status SHALL stand as it was, and one line SHALL name the repository.

#### Scenario: A preview released by a person

- **WHEN** `home/30`'s status is carried listed, and at the next pass the stub answers `previews` with `[]`
- **THEN** `home/30`'s status stands with its text as it was, not listed; and at the pass after, `sluice previews` is not run for it

#### Scenario: A listing that cannot be read

- **WHEN** `home/30`'s status is carried listed, and at the next pass the stub refuses `previews`, exit 1, saying `no repository here`
- **THEN** `home/30`'s status stands as it was, listed, and a line reads `boomdrive: sluice's previews of agentplot/flume could not be read: no repository here`

### Requirement: A done puts away what the runner asked sluice for

For a preview marked done, the runner SHALL put away each preview of the report not marked done whose repository and revision are the done's, answered `done` or `held`, by one `sluice release <name> --json` in its repository's clone's container. `<name>` SHALL be the name sluice gave, in its answer or else its listing, else the revision. sluice's answer SHALL be the done's, `done` carrying it, or `refused` with sluice's words (core.55).

#### Scenario: A preview put away

- **WHEN** `home/30`, of `main` at `R1`, is brought up as `preview-starting.json` names it, and a later pull gives `home/31`, the same preview marked done, the stub answering `release` with `release.json`
- **THEN** the stub's one `release` call is `release <the fixture's name> --json`; `home/31` is answered `done`, its text the fixture's `preview` and `released`; and a line reads `home/31: done of the preview of main at R1: put away`

#### Scenario: A bring-up and its done in one pull

- **WHEN** a pull gives `home/30`, of `main` at `R1`, then `home/31`, its done, and neither is answered
- **THEN** `sluice preview main R1 --json` is run, then `sluice release`, and both are answered `done`

### Requirement: A held preview put away is asked no more

Each preview held that a done puts away SHALL be answered `refused`, with the words `put away before sluice brought it up`, and SHALL be asked of sluice no more (core.55).

#### Scenario: A held preview put away

- **WHEN** `home/30` is held, sluice listing its preview `waiting` under the name `N`, and a pull gives `home/30` and `home/31`, its done
- **THEN** `sluice release N --json` is run once and `sluice preview` not at all; `home/31` is answered as sluice answered; and `home/30` is answered `refused` with the words `put away before sluice brought it up`, and is asked of sluice at no later pass

### Requirement: A done with nothing to put away runs nothing

Where the report holds no preview a done would put away, the done SHALL be answered `refused`, with the words `no preview of <revision> in <repository> was brought up here at the control plane's asking`. Where no clone of its repository stands, it SHALL be answered `refused`, with the words `no clone of <repository> stands on this host`. No `sluice` command SHALL be run for either (core.55).

#### Scenario: A done of a preview never asked for

- **WHEN** a pull gives a preview of `main` at `R9` marked done, and the report holds no preview of `R9`
- **THEN** it is answered `refused` with the words `no preview of R9 in agentplot/flume was brought up here at the control plane's asking`, and no `sluice` command is run

### Requirement: A preview's command runs after every launch

A preview's command, `sluice preview` or `sluice release`, SHALL run in the pass's second round, after every launch the pull offers, in the control plane's order, where a done is matched to what the report then holds. A preview answered from the report, one under a lapsed lease, a bring-up in a repository not kept, and a done whose repository has no clone SHALL be answered in the first. A pass's preview commands SHALL NOT have Flume's listing read again (core.50; record 0027).

#### Scenario: A launch ahead of a preview

- **WHEN** a pull gives the preview `home/30`, then the offer `a`
- **THEN** `a`'s `flume launch` is run before `sluice preview`

### Requirement: Once the next pass is due, a pass runs no preview's command but its first

Once the next control pass is due, the second round SHALL run no preview's command but its first, SHALL leave each after it to that pass, and SHALL say how many it left. The count SHALL be the previews' own, apart from the acts' on places (core.50, core.55).

#### Scenario: Previews wait for the next pass once it is due

- **WHEN** a pull gives the previews `home/30` and `home/32`, of `R1` and `R2`, and the next pass is due once one preview's command has run
- **THEN** `home/30` is brought up, `home/32` is left unanswered, and a line reads `boomdrive: the next control pass is due, so 1 preview waits for it`

### Requirement: A preview's lease is kept as any lease is

A preview under a lease lapsed by the pass's clock SHALL be left untaken, and a held one SHALL NOT be asked again, until a pull gives it under a live lease. A preview the control plane lets go, or whose report it refuses as no longer the runner's, SHALL leave the report, and its preview SHALL be left as sluice holds it, with nothing run against it (core.10, core.24).

#### Scenario: A preview's lease refused

- **WHEN** `home/30` is brought up, and the fake control plane refuses its report as no longer this runner's
- **THEN** `home/30` leaves the report, a line names its preview as left on this host as sluice holds it, and no `sluice release` is run

#### Scenario: A preview whose lease lapsed

- **WHEN** a pull gives the preview `home/30` under a lease that expired before the pass began
- **THEN** it is left unanswered, and no `sluice` command is run
