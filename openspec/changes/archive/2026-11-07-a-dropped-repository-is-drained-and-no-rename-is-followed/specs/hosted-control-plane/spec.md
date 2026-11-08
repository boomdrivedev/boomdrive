# Hosted Control Plane

## MODIFIED Requirements

### Requirement: A partition's connection carries only its own repositories and work

Each partition's hello SHALL name only the repositories its table names, of those kept with their clones made, and only its own leases. No hello SHALL name a repository the runner drains, and no report the repository a launch was answered in, since version 1 carries neither. Each report SHALL carry only the entries under its prefix. A want naming a repository the partition does not name SHALL not be given to the runner, and one line SHALL name it (core.26; ledger 3.b).

#### Scenario: Each hello its own

- **WHEN** the runner keeps `example/app` and `example/service`, `home` names `example/app`, and the report holds `home/l-1` and `work/l-2`
- **THEN** `home`'s hello names `example/app` and lease `l-1` alone

#### Scenario: Another partition's repository

- **WHEN** `home`'s control plane offers a launch on `example/service`, which `home` does not name
- **THEN** the pull does not give it, and one line names the offer, `home` and `example/service`

#### Scenario: A repository drained goes up in no hello

- **WHEN** the runner keeps `example/service` and drains `example/app`, which `home` names, and its report's launch `home/l-1` names `example/app` as the repository it was answered in
- **THEN** `home`'s hello names no repository, and its report's `l-1` holds no repository
