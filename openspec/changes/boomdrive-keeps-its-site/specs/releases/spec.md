## MODIFIED Requirements

### Requirement: The version is git-cliff's, from the commits since the newest release

`release.sh next` SHALL print the tag `v<version>` of the version git-cliff makes under `cliff.toml` from the commits since the newest tag `v<major>.<minor>.<patch>` in the history, `v0.1.0` where the history holds none. Before `1.0.0`, a feature or a breaking change SHALL bump the minor version, and any other commit the notes keep the patch. A commit that changes only the site's paths, `site/` and `.github/workflows/pages.yml`, SHALL bump nothing (core.48; record 0038). Where that version is tagged already, `release.sh next` SHALL print nothing and say so on standard error. It SHALL write nothing.

#### Scenario: The first release

- **WHEN** a history with no release tag holds a `feat:` and a `fix:` commit
- **THEN** `release.sh next` prints `v0.1.0`

#### Scenario: A feature

- **WHEN** the newest release tag is `v0.1.0` and a `feat:` commit follows it
- **THEN** `release.sh next` prints `v0.2.0`

#### Scenario: A fix

- **WHEN** the newest release tag is `v0.1.0` and a `fix:` commit follows it
- **THEN** `release.sh next` prints `v0.1.1`

#### Scenario: A breaking change before 1.0.0

- **WHEN** the newest release tag is `v0.1.0` and a `fix!:` commit follows it
- **THEN** `release.sh next` prints `v0.2.0`

#### Scenario: Nothing to release

- **WHEN** the newest release tag is `v0.1.0` and only `chore:` and `test:` commits follow it
- **THEN** `release.sh next` prints nothing

#### Scenario: A change to the site alone

- **WHEN** the newest release tag is `v0.1.0` and only a `feat(site): a page` commit follows it, changing a file under `site/` and nothing else
- **THEN** `release.sh next` prints nothing

#### Scenario: A change to the site and the runner

- **WHEN** the newest release tag is `v0.1.0` and a `fix: both` commit follows it, changing a file under `site/` and a file outside it
- **THEN** `release.sh next` prints `v0.1.1`

### Requirement: The notes are git-cliff's, for the same commits

`release.sh notes <tag>` SHALL print git-cliff's notes, under `cliff.toml`, of the commits since the newest release tag, headed by the tag: breaking changes, features, fixes, performance, refactors and documentation, each a group of its own in that order, each commit a line of its scope, its subject and a link to it. A chore, a test, a build, CI, a style change or a commit git-cliff cannot read SHALL be in no group, nor SHALL a commit that changes only the site's paths, `site/` and `.github/workflows/pages.yml` (core.48; record 0038), and no line SHALL hold a `Co-Authored-By` trailer.

#### Scenario: A release's notes

- **WHEN** the newest release tag is `v0.1.0`, followed by `feat(boomdrive): a feature`, `fix(nix): a fix`, `docs(adr, architecture): a record` and `chore: a chore`, the feature carrying a `Co-Authored-By` trailer, and `release.sh notes v0.2.0` runs
- **THEN** the notes are headed `v0.2.0`, hold `**boomdrive**: A feature` under Features, `**nix**: A fix` under Fixes and `**adr, architecture**: A record` under Documentation, in that order, and hold nothing of the chore or the trailer

#### Scenario: The site's commit in no notes

- **WHEN** the newest release tag is `v0.1.0`, followed by `fix(nix): a fix` and by `feat(site): a page`, which changes only a file under `site/`, and `release.sh notes v0.1.1` runs
- **THEN** the notes hold `**nix**: A fix` under Fixes, and nothing of the page
