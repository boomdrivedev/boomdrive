# Tool Records

## MODIFIED Requirements

### Requirement: Each submission Flume names is carried from sluice's own answer

For each launch whose carried status names a sluice entry, at `submission.read.found.submission.id`, and whose repository's clone stands, whether the configuration names the repository or the runner drains it, the runner SHALL run `sluice status <id> --json` in that clone's container. It SHALL carry these fields of sluice's answer, each as sluice printed it: `id`, `branch`, `head`, `state`, `failed`, each row with only its `task`, `hash`, `flaky` and `system`, and `landed_as`.

#### Scenario: An entry carried

- **WHEN** a launched place's carried status names entry 412, and the stub `sluice` prints its submission
- **THEN** the stub is run as `sluice status 412 --json` in the clone's container, and the carried entry holds its `id`, `branch`, `head`, `state` and `landed_as` as printed

#### Scenario: No submission

- **WHEN** a launched place's carried status names no sluice entry
- **THEN** no `sluice` command is run for it

#### Scenario: An entry carried from a repository drained

- **WHEN** the runner drains `example/kit` for launch `a`, and `a`'s carried status names entry 412
- **THEN** the stub `sluice` is run as `sluice status 412 --json` in `example/kit`'s clone container, and its entry is carried
