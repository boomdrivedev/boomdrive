## ADDED Requirements

### Requirement: The set's sluice takes each of the runner's asks

The runner's tests SHALL run each sluice command the runner makes against the set's sluice: `status`, `preview`, `previews` and `release`. They SHALL run each through the runner's own reader, with the runner's arguments, in the runner's clone of a scratch repository. Each SHALL exit other than 64. In that repository, `main` and `next` SHALL each hold a submission 1. The ask of 1 on each branch SHALL give that branch's entry (sluice record 0114).

#### Scenario: A number on two branches, from the set's sluice

- **WHEN** the scratch repository's `main` keeps its records on its remote, `next` keeps its records on the host, and each has a submission 1
- **THEN** the set's sluice refuses `status 1 --json`, and gives the entry of 1 on `main` and on `next` to the runner's asks, each naming its branch

#### Scenario: Each ask's command line is read

- **WHEN** the runner's reader asks the set's sluice for an entry, a preview, the previews and a release in that clone
- **THEN** no ask exits 64, the preview is answered, and the previews are a listing
