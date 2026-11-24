## ADDED Requirements

### Requirement: A repository's tokens come from the plane of the accounts that serve it

Where an account names a control plane that holds the App, a repository whose every account serving it names that plane SHALL take its tokens from that plane, by `boomdrive forge-token get` and the running runner's fetches alike. Any other repository SHALL be minted from the App `[forge]` names, or, where it names none, fetched with the host's own helpers, as on a host whose control plane does not hold the App (record 0054; core.33).

#### Scenario: crew's repository and a cloud's side by side

- **WHEN** the account `own` names crew and keeps `agentplot/flume`, the account `cloud` names the hosted control plane and keeps `example/app`, `[forge]` names the App, and git asks `get` for each
- **THEN** `example/app`'s token is asked of the hosted control plane, and `agentplot/flume`'s is minted from the App's key

#### Scenario: A repository both accounts serve

- **WHEN** `agentplot/flume` is kept by the account `own`, which names crew, and by the account `cloud`, which names the hosted control plane, and `[forge]` names the App
- **THEN** `agentplot/flume`'s token is minted from the App's key, and the hosted control plane is not asked
