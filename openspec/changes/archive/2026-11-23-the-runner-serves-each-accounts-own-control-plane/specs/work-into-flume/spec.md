## ADDED Requirements

### Requirement: Every place in a kept repository that a part did not launch is reported to it, with no lease

At each control pass the runner SHALL give the part's share, as `unleased`, each place Flume's listing holds in the clone of a repository it keeps or drains whose mark is not the runner's mark of a launch the share holds, by its `place`, with the repository's address and its entry carried as Flume printed it. It SHALL act on none, and keep them in memory alone (record 0054; core.1, core.10, core.11, core.35).

#### Scenario: A place started by hand

- **WHEN** Flume's listing holds the place `/p/places/note--by-hand` in the clone of `agentplot/flume`, with no mark, and the part `home` holds one launch in `agentplot/flume`
- **THEN** `home`'s report names its launch with its lease, and `/p/places/note--by-hand` among `unleased`, with `agentplot/flume` and its entry as Flume printed it

#### Scenario: Another part's launch

- **WHEN** the part `home` launched the place of `home/1` in `agentplot/flume`, and the part `cloud` holds no launch there
- **THEN** `cloud`'s report names that place among `unleased`, and `home`'s names it under `home/1` alone

#### Scenario: A place in no kept repository

- **WHEN** Flume's listing holds a place whose `repository` is no clone the runner keeps or drains
- **THEN** no part's report names it

#### Scenario: Carried again as it stood

- **WHEN** an unleased place's entry is the same at two control passes
- **THEN** the second report gives it with the first pass's time
