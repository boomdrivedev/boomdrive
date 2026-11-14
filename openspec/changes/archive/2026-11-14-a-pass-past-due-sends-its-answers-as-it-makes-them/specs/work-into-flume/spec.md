# Work into Flume

## ADDED Requirements

### Requirement: A pass past due sends each answer as it makes it

Once the next control pass is due, by its timer or by the control plane's wake, a control pass SHALL send its report after each want whose answer changed it, and after each launch that waited on its clone's keep, then go on in the order core.50 gives. A pass that ends before the next is due SHALL send once, at its end, and every pass's end SHALL send as before. What a pass takes, and in what order, SHALL NOT change (core.50; record 0002, as amended on 2026-11-14).

#### Scenario: A pass past due sends each launch's answer before its next launch

- **WHEN** the next control pass is due from a pass's start, and its pull offers `a`, `b` and `c`
- **THEN** a report holding `a` launched is sent after `a`'s launch and before `b`'s is asked of Flume, one holding `a` and `b` before `c`'s, one holding all three after it, and the pass's end sends once more

#### Scenario: A pass not yet due sends once

- **WHEN** the next control pass does not fall due while a pass launches the offers `a`, `b` and `c` its pull gives
- **THEN** the pass sends one report, at its end, holding all three launched

#### Scenario: An act's answer goes up before the next act runs

- **WHEN** the runner holds `a` and `b` at places Flume stands, and `c` at a place Flume no longer stands, a pull gives a priority of `c`, of `a` and of `b`, then the three offers, and the next control pass is due once one act's command has run
- **THEN** `c`'s priority is answered `refused` with Flume's words, `a`'s is run, and a report holding both is sent before the pass's end; `b`'s is left for the next pass, and the pass's end sends once more

### Requirement: What a send of the pass lets go is taken no more that pass

The control plane's answer to a send before a pass's end SHALL be taken as the answer at its end is: what it let go or refused SHALL leave the report, said in one line, and the pass SHALL take no want of it more, though its pull gives it (core.10).

#### Scenario: A want let go at a send of the pass is taken no more

- **WHEN** the runner holds `a` at a place Flume has since tidied, the control plane refuses `a`'s lease, the next control pass is due from a pass's start, and its pull offers `b`, then `a`
- **THEN** `b` is launched, the report sent after it has `a` refused, a line names `a`'s place left on the host, `a` leaves the report, and no `flume launch` is run for `a`

### Requirement: A send that fails before a pass's end leaves its answers to the end

A send before a pass's end that fails SHALL be said in one line, and the pass SHALL try no send more before its end, whose send is tried as before (core.22).

#### Scenario: A send that fails leaves the pass's answers to its end

- **WHEN** the next control pass is due from a pass's start, its pull offers `a` and `b`, and the control plane cannot take a report until both are launched
- **THEN** the send after `a`'s launch fails and one line says it is sent at the pass's end, no send is tried after `b`'s, and the pass's end sends a report holding both
