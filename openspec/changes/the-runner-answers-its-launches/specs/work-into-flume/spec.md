## MODIFIED Requirements

### Requirement: The runner is one more recipient of a launch that names recipients, and no brief is left

The runner SHALL run every `flume` command with `FLUME_RECIPIENT` unset. Where a launch names recipients, its `flume launch` SHALL give them as the launch gives them. It SHALL then name the runner, `--recipient boomdrive`, so Flume holds the place's end for it. Where a launch names none, and other readers of its control plane are on the host, it SHALL name none. It SHALL remove a launch's brief file once Flume answers (core.15, core.54, record 0030).

#### Scenario: A launch's environment and brief

- **WHEN** the stub `flume` answers a launch with a brief
- **THEN** the stub's environment held no `FLUME_RECIPIENT`, and the brief's file no longer exists

#### Scenario: The runner is one more recipient

- **WHEN** the runner launches an offer that names `home/planner`, then an offer that names no recipient, each from crew's kind of control plane
- **THEN** the first launch names `--recipient home/planner` then `--recipient boomdrive`, and no other recipient. The second names no recipient.

## ADDED Requirements

### Requirement: A launch of a control plane that the runner alone reads names the runner alone

Where a launch names no recipient, and the runner alone reads its control plane, its `flume launch` SHALL name the runner alone, `--recipient boomdrive`. Flume then takes the launch and holds each end of the place for the runner. The runner SHALL carry the place's status, with its question, as it carries every status (core.54, record 0030 as amended on 2026-11-26).

#### Scenario: A launch read through the runner alone

- **WHEN** the runner launches an offer with no recipient over the stub `flume`, from a control plane that the runner alone reads
- **AND** the place stops short on the question "Which width is a tab: 4 or 8?"
- **THEN** the launch names `--recipient boomdrive` and no other recipient
- **AND** the report that the control plane takes holds the place's status, with the question, as Flume gave it
