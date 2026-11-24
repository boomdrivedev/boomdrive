## ADDED Requirements

### Requirement: Each unleased place of the drive's repositories is written in unleased.rec

crew's implementation SHALL write `unleased.rec` on the runner's branch, under the descriptor of an `Unleased` this repository proposes for crew, one `Unleased` per unleased place of the share in a repository the partition's drive keeps, each holding its `Place`, `Repository`, `At` and `Status` as carried. crew's implementation SHALL act on none. The file SHALL pass `recfix --check` (record 0054; core.10, core.26).

#### Scenario: A place started by hand

- **WHEN** the runner reports for `home`, whose drive keeps `agentplot/flume`, a share holding the unleased place `/p/places/note--by-hand` in `agentplot/flume` and one in `agentplot/sluice`, which `home` does not keep
- **THEN** `home/runner/host-a`'s `unleased.rec` opens with the descriptor of an `Unleased`, byte for byte, holds one `Unleased`, with `Place: /p/places/note--by-hand`, `Repository: agentplot/flume`, the status's time as its `At` and its `Status` as carried, byte for byte, and passes `recfix --check`

### Requirement: unleased.rec is written only where an unleased place is or was

crew's implementation SHALL write `unleased.rec` where the partition's share holds an unleased place of its drive's repositories, and as the descriptor alone where the tip holds one and the share none. Otherwise the runner's branch SHALL hold no `unleased.rec` (record 0054).

#### Scenario: A branch with no unleased place

- **WHEN** the runner first reports for `home`, its share holding a launch and no unleased place
- **THEN** `home/runner/host-a` holds no `unleased.rec`
