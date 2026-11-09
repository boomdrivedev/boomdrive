# hosted-control-plane

## MODIFIED Requirements

### Requirement: What goes up is version 1's copy, checked before it goes

Each place's status, sluice entry and host part SHALL go up cut to version 1's copy by walking its schema, and checked against it. A copy that fails SHALL go up `null`, or the host part `missing`, with one line naming the launch and the schema's path, never a value. Every message SHALL be checked against `from-runner.schema.json` before it is sent. The answer to a part's report SHALL name taken each launch whose status went up as its copy in the report it answers, and none whose status went up `null`; an answer to an earlier report SHALL name none (core.11, core.27, core.42; record 0030 as amended on 2026-11-09).

#### Scenario: Flume's places cut

- **WHEN** each place of Flume 0.4.1's fixtures is carried in a report
- **THEN** its copy passes `place-status.schema.json`, and holds no `last_words`, `screen`, `printed`, task `text`, `readiness` or `repository`

#### Scenario: A value Flume adds

- **WHEN** a place's status names a stage's state Flume 0.4.1 does not have
- **THEN** that launch's status goes up `null`, the report passes and is sent, one line names the launch and the path, and not the value, and the report's answer names no status of that launch taken

#### Scenario: A status that goes up as its copy

- **WHEN** a place's status passes version 1
- **THEN** its copy goes up with the time it was read and no `repository`, and the report's answer names that launch's status taken
