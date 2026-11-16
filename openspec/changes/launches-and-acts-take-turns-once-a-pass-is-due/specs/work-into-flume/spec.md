# Work into Flume

## ADDED Requirements

### Requirement: Once a pass is due, it makes no launch after its first

Once the next control pass is due, by its timer or by the control plane's wake, a control pass SHALL run no `flume launch` after the first it ran, whether that first ran before the pass fell due or after. A launch SHALL count where its `flume launch` runs, whatever Flume answers. `boomdrive run --once`, after which no pass is due, SHALL make every launch it takes (core.50; record 0002, as amended on 2026-11-14 on taking turns).

#### Scenario: A pass due from its start makes one launch

- **WHEN** the next control pass is due from a pass's start, and its pull offers `a`, `b` and `c`
- **THEN** `a` is launched; `b` and `c` are answered `held` with the words `waits for the next control pass, which is due`, each said in one line, and no `flume launch` is run for either; and at the next pass, due from its start, whose pull offers `a`, `b` and `c`, `b` is launched and `c` stays `held`, said in no line

#### Scenario: A pass that falls due among its launches makes none more

- **WHEN** a pull offers `a`, `b`, `c` and `d`, and the next control pass falls due once two launches have been made
- **THEN** `a` and `b` are launched, and `c` and `d` are answered `held`, no `flume launch` being run for either

### Requirement: A launch the pass took and left is held for its turn

Each offer a pass past due would launch after its first, a launch that waited on its clone's keep among them, SHALL be answered `held`, with the words `waits for the next control pass, which is due`, and its room SHALL be counted at its turn, so each offer after it gets the answer the control plane's order gives. A later pass SHALL take it as any held offer, and its acts SHALL be answered as a held launch's are (core.10, core.50).

#### Scenario: A launch held for its turn counts against its stream's room

- **WHEN** the next control pass is due from a pass's start, and its pull offers `a`, `b` and `c`, each naming stream `work`, whose `room` is 2 and whose `queue` is empty
- **THEN** `a` is launched, `b` is answered `held` for its turn, and `c`, for which no room is left, is left unanswered

#### Scenario: A launch that waited on its clone's keep is held without waiting on it

- **WHEN** the next control pass is due from a pass's start, the keeper holds `agentplot/flume`'s clone for its fetch, and the pull offers `a` into `agentplot/flume`, then `b` into `agentplot/sluice`
- **THEN** `b` is launched, and `a` is answered `held` for its turn, no `flume launch` being run for it and the pass not waiting for the fetch

#### Scenario: A drop of a launch held for its turn

- **WHEN** the next control pass is due from a pass's start, and its pull offers `a` and `b`, then gives a drop of `b`
- **THEN** `a` is launched, the drop is answered `done`, `b` is answered `done` with words naming the drop, and no `flume launch` is run for `b`

### Requirement: Once a pass is due, its launches and its acts take turns

Once the next control pass is due, a control pass SHALL run its first launch, where it has run none, then its first act's command on a place, where it has run none, and none of either after, leaving the rest to the next pass. So no act SHALL wait behind more than one launch, nor a launch of the next pull behind more than one act. Each kind SHALL keep the control plane's order among its own (core.50; record 0002, as amended on 2026-11-14 on taking turns).

#### Scenario: An act waits behind one launch once the pass is due

- **WHEN** the runner holds `x` at a place Flume stands, the next control pass is due from a pass's start, and its pull gives a land of `x`, then offers `a` and `b`
- **THEN** `a` is launched, then `flume retry --submission` is run on `x`'s place, and `b` is answered `held`; and at the next pass, due from its start, whose pull gives a priority of `x`, then offers `a`, `b` and `c`, `b` is launched, then `flume priority` is run on `x`'s place, and `c` is answered `held`

#### Scenario: A pass that falls due among its launches runs its first act next

- **WHEN** the runner holds `x` and `y` at places Flume stands, a pull gives a priority of `x` and a priority of `y`, then offers `a`, `b`, `c` and `d`, and the next control pass falls due once two launches have been made
- **THEN** `a` and `b` are launched, `c` and `d` are answered `held`, then `flume priority` is run on `x`'s place; `y`'s priority is left unanswered, and the pass says that 1 act on a place waits for the next

### Requirement: A pass not yet due sends once, at its end

A control pass that ends before the next is due SHALL send its report once, at its end, and every pass's end SHALL send as before, whether the pass fell due or not (core.50; record 0002, as amended on 2026-11-14).

#### Scenario: A pass not yet due sends once

- **WHEN** the next control pass does not fall due while a pass launches the offers `a`, `b` and `c` its pull gives
- **THEN** the pass sends one report, at its end, holding all three launched

## MODIFIED Requirements

### Requirement: A pass past due sends each answer as it makes it

Once the next control pass is due, a control pass SHALL send its report, wherever an answer changed it since its last send, before each tool's command it runs for a want, and once it has answered its last want, so no answer waits on a command. The commands SHALL be a `flume launch`, made in turn or once its clone's keep is done; Flume's status of a place for an act, and Flume's command on the place; and sluice's preview or release (core.50; record 0002, as amended on 2026-11-14).

#### Scenario: A pass past due sends a launch's answer before its act runs

- **WHEN** the runner holds `x` at a place Flume stands, the next control pass is due from a pass's start, and its pull gives a priority of `x`, then offers `a`, `b` and `c`
- **THEN** one report, holding `a` launched and `b` and `c` held, is sent after `a`'s launch and before `flume priority` is run on `x`'s place; one holding `x`'s priority done is sent once its answer is made; and the pass's end sends once more

#### Scenario: A pass past due sends each launch's answer before its next launch

- **WHEN** the next control pass is due from each pass's start, and each pass's pull offers `a`, `b` and `c`
- **THEN** the first pass launches `a`, the second `b` and the third `c`; each pass sends a report holding its launch once its last want is answered, before the next pass's launch is asked of Flume, and its end sends once more

#### Scenario: An act's answer goes up before the next act runs

- **WHEN** the runner holds `a` and `b` at places Flume stands, and `c` at a place Flume no longer stands, a pull gives a priority of `c`, of `a` and of `b`, then the three offers, and the next control pass is due once one act's command has run
- **THEN** `c`'s priority is answered `refused` with Flume's words, `a`'s is run, and a report holding both is sent before the pass's end; `b`'s is left for the next pass, and the pass's end sends once more

### Requirement: What a send of the pass lets go is taken no more that pass

The control plane's answer to a send before a pass's end SHALL be taken as the answer at its end is: what it let go or refused SHALL leave the report, said in one line. Where it lets go or refuses the want about to run its command, or the launch that want acts on, no command SHALL be run for it. The pass SHALL take no want it let go or refused, and no act on a launch it let go or refused, more that pass, though its pull gives them (core.10).

#### Scenario: A want let go at a send of the pass is taken no more

- **WHEN** the runner holds `a` at a place Flume has since tidied, the control plane refuses `a`'s lease, the next control pass is due from a pass's start, and its pull offers `c` in a repository the runner does not keep, then `b`, then `a`
- **THEN** `c` is refused; the report sent before `b`'s launch holds that refusal and has `a` refused; a line names `a`'s place left on the host, and `a` leaves the report; `b` is launched; and `a` is neither launched nor answered that pass

#### Scenario: An act whose launch is let go at the send before it runs nothing

- **WHEN** the runner holds `x` at a place Flume stands, the control plane refuses `x`'s lease, the next control pass is due from a pass's start, and its pull offers `b`, then gives a priority of `x`, then a drop of `x`
- **THEN** `b` is launched; the report sent before `x`'s priority would run has `x` refused, and `x` leaves the report; no `flume` command is run on `x`'s place; and neither act is answered that pass

### Requirement: A send that fails before a pass's end leaves its answers to the end

A send before a pass's end that fails SHALL be said in one line, and the pass SHALL try no send more before its end, whose send is tried as before (core.22).

#### Scenario: A send that fails leaves the pass's answers to its end

- **WHEN** the runner holds `x` at a place Flume stands, the next control pass is due from a pass's start, its pull offers `a`, then gives a priority of `x`, and the control plane cannot take a report until `x`'s priority has run
- **THEN** the send before `flume priority` is run on `x`'s place fails, and one line says the report is sent at the pass's end; no send is tried once the last want is answered; and the pass's end sends a report holding `a` launched and `x`'s priority done
