## ADDED Requirements

### Requirement: No try of weir-branch's call to a state repository outlasts its limit

crew's implementation SHALL read `<label>/main` and each runner's branch, and write its own, by a weir-branch that runs each try of its fetch, listing and push to `weir-bounded`'s limit, thirty seconds, stopped with everything it started. A stopped read SHALL leave its partition unread, and a stopped write its report unsent, each with weir-branch's words (weir record 0008; record 0042 as amended on 2026-11-11; core.58).

#### Scenario: A state repository that never answers the read

- **WHEN** crew's implementation reads `home/main` of a state repository that takes the connection and never answers
- **THEN** the read ends within thirty seconds, its grace and a margin, and fails with words ending `git fetch with <url> was still going after 30s, the most a try may take, and was stopped`, naming the state repository's URL

#### Scenario: A state repository that never answers a write

- **WHEN** crew's implementation writes `home/runner/host-a` to a state repository that takes the connection and never answers
- **THEN** the write ends within thirty seconds, its grace and a margin, having formed nothing, and fails with words ending `git fetch with <url> was still going after 30s, the most a try may take, and was stopped`, naming the state repository's URL
