## ADDED Requirements

### Requirement: A commit that changes only the site releases nothing

A commit that changes only the site's paths, `site/` and `.github/workflows/pages.yml`, SHALL bump no version that `release.sh next` prints, and SHALL be in no group of `release.sh notes`, whatever its type (core.48; record 0038). A commit that changes the site's paths and any other path SHALL bump the version and be in the notes as any commit is.

#### Scenario: A change to the site alone

- **WHEN** the newest release tag is `v0.1.0` and only a `feat(site): a page` commit follows it, changing a file under `site/` and nothing else
- **THEN** `release.sh next` prints nothing

#### Scenario: A change to the site and the runner

- **WHEN** the newest release tag is `v0.1.0` and a `fix: both` commit follows it, changing a file under `site/` and a file outside it
- **THEN** `release.sh next` prints `v0.1.1`

#### Scenario: The site's commit in no notes

- **WHEN** the newest release tag is `v0.1.0`, followed by `fix(nix): a fix` and by `feat(site): a page`, which changes only a file under `site/`, and `release.sh notes v0.1.1` runs
- **THEN** the notes hold `**nix**: A fix` under Fixes, and nothing of the page
