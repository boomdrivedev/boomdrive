## ADDED Requirements

### Requirement: crew's implementation takes the runner's halt

crew's implementation SHALL take the runner's halt, `weir-bounded`'s, at its opening, and give it to each part, each part to each partition's store, and each store to its listing and to every branch it gives weir-branch, so its reads and its writes carry it (weir record 0008 as amended 2026-11-12; record 0042 as amended on 2026-11-12; core.58).

#### Scenario: A store's branches carry its halt

- **WHEN** a store is made, and its branch `home/main` is asked for
- **THEN** the store's halt is never set, and the branch carries the store's halt

#### Scenario: A pull after the runner's stop

- **WHEN** a part is made by `parts` with the runner's halt already set, over a state repository whose `home/main` holds a launch for this host, and it is pulled
- **THEN** the pull is unread, and `home/runner/host-a` is not written

### Requirement: A call to a state repository in flight ends at the runner's stop

The store's listing of `<label>/runner/*`, and each try of weir-branch's fetch, listing and push of a branch the store gives, SHALL end at the first of its limit and the runner's halt. A try in flight when the halt is set SHALL have its group asked to end, given two seconds, then killed; a try begun once it is set SHALL run no git (weir record 0008 as amended 2026-11-12; core.58).

#### Scenario: A listing in flight when the runner stops

- **WHEN** crew's implementation lists `home/runner/*` of a state repository that takes the connection and never answers, with a limit of thirty seconds, and its store's halt is set 300 ms in
- **THEN** the listing fails within five seconds

#### Scenario: A read in flight when the runner stops

- **WHEN** crew's implementation reads `home/main` of a state repository that takes the connection and never answers, and its store's halt is set 300 ms in
- **THEN** the read fails within five seconds

#### Scenario: A write in flight when the runner stops

- **WHEN** crew's implementation writes `home/runner/host-a` to a state repository that takes the connection and never answers, and its store's halt is set 300 ms in
- **THEN** the write fails within five seconds, having formed nothing

### Requirement: A listing ended at the stop leaves its partition unread

A listing ended at the runner's stop, or begun after it, SHALL fail with `weir-bounded`'s words for a halted call, `git ls-remote with <url> was stopped as its caller asked`, naming the state repository's URL, after git's last line on standard error and `; ` where git wrote one. Its partition SHALL be unread with those words, as any part that could not be read (record 0042 as amended on 2026-11-12).

#### Scenario: A listing's words at the runner's stop

- **WHEN** a listing of `home/runner/*` is halted in flight
- **THEN** it fails with words ending `git ls-remote with <url> was stopped as its caller asked`, naming the state repository's URL and no limit

### Requirement: A read or write ended at the stop leaves its partition unread or its report unsent

A read or a write of weir-branch's ended at the runner's stop, or begun after it, SHALL fail with weir-branch's words, ending `git fetch with <url> was stopped as its caller asked` where its fetch was halted. A halted read SHALL leave its partition unread with those words, and a halted write its report unsent, as any that fails, neither tried again within its pull or report (weir record 0008 as amended 2026-11-12).

#### Scenario: A read's words at the runner's stop

- **WHEN** a read of `home/main` is halted in flight
- **THEN** it fails with words ending `git fetch with <url> was stopped as its caller asked`, naming the state repository's URL and no limit

#### Scenario: A write's words at the runner's stop

- **WHEN** a write of `home/runner/host-a` is halted while it reads the branch
- **THEN** it fails with words ending `git fetch with <url> was stopped as its caller asked`

#### Scenario: A pull's words after the runner's stop

- **WHEN** a part made with the runner's halt already set is pulled
- **THEN** its pull is unread with words holding `was stopped as its caller asked`
