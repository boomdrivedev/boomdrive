# Work into Flume

## ADDED Requirements

### Requirement: A pull's launches go ahead of its acts on places

The control pass SHALL answer a pull's wants in two rounds, each in the control plane's order. The first SHALL take every offer, inbox and command, and answer every act that runs no command, leaving each act that runs Flume's command on its launch's place for the second. No launch SHALL wait behind an act on a place. An act Flume refuses because its place no longer stands SHALL be answered with Flume's refusal, once (core.49; records 0002 and 0010, as amended on 2026-11-07).

#### Scenario: A launch ahead of acts on places that are over

- **WHEN** the runner holds launch `a` at place `P`, Flume no longer stands `P`, and a pull gives a priority and a retry of `a`, then the offer `a`, then a new offer `b`
- **THEN** `b` is launched before any `flume` act is run; then `flume priority P` and `flume retry P` are run, each answered `refused` with Flume's words naming `P`; and the next pass runs no `flume` command for either

### Requirement: No answer turns on the round a want is taken in

The launches a pass makes SHALL be those the control plane's order makes. An act answered from the report SHALL be answered where the control plane's order puts it, and an act on a launch the first round took SHALL be carried out in the second (core.49).

#### Scenario: An act ahead of its own launch

- **WHEN** a pull gives a priority of `a`, then the offer `a`, and `a` is launched that pass at place `P`
- **THEN** `flume priority P` is run in the same pass, after the launch, and the act is answered `done`

#### Scenario: A drop ahead of a held launch

- **WHEN** launch `b` is held on its After, and a pull gives a drop of `b`, then the offer `b`, at the pass at which `b`'s After reads landed
- **THEN** the drop is answered `done`, `b` is answered `done` with words naming the drop, and no `flume launch` is run for `b`
