## MODIFIED Requirements

### Requirement: A clone is fetched over https and never asks

Every fetch SHALL reach origin over https, with git never prompting. Where the configuration names no `[forge]`, a fetch SHALL go through the host's git credential helper, with no credential of the runner's own. Where it names `[forge]`, a fetch of a served repository, a new clone's first among them, SHALL go through the App's token alone. The call SHALL empty the host's helpers, and its one helper SHALL read the token the runner keeps for the repository from that call's environment, so the token is on no command line. Where the App is not installed on the repository, or no token could be minted, the fetch SHALL go with the host's helpers emptied and no credential. Where such a fetch fails after a refused mint, the line SHALL give git's words, then `; no forge token: ` and the mint's words. A fetch whose URL the host's git configuration rewrites to ssh SHALL not be run: the repository SHALL be named with the URL it would have used, so a fetch never waits on an ssh agent or on someone at the keyboard (record 0007 as amended on 2026-11-11; core.33, core.34).

#### Scenario: Git asks no one

- **WHEN** the runner fetches a clone, its configuration naming no `[forge]`
- **THEN** git runs with `GIT_TERMINAL_PROMPT=0` and is given no credential, so the host's credential helper answers or the fetch fails without asking

#### Scenario: Git is given the App's token alone

- **WHEN** a call to a remote is given the App's token `t`, and the host's git configuration names a helper of its own that answers with another password
- **THEN** git is given `x-access-token` and `t` by the call's own helper, the host's helper is not asked, and no argument of the call holds `t`

#### Scenario: Git is given no credential

- **WHEN** a call to a remote is given no credential, and the host's git configuration names a helper of its own
- **THEN** git is given no credential, and the host's helper is not asked

#### Scenario: A repository the App is not installed on

- **WHEN** a pass keeps `agentplot/flume`, its configuration naming `[forge]`, GitHub has no installation of the App on it, and its origin needs no credential
- **THEN** it is fetched and its view moved, and no token is bought

#### Scenario: A mint refused is named after git's words

- **WHEN** a pass keeps `agentplot/flume`, its configuration naming `[forge]` whose `key_file` does not exist, and the fetch of `agentplot/flume` fails
- **THEN** a line reads `agentplot/flume: not fetched: <git's words>; no forge token: [forge] key_file <path> cannot be read: <why>`

#### Scenario: The host rewrites https to ssh

- **WHEN** the host's git configuration says `url."git@github.com:".insteadOf = "https://github.com/"` and a pass keeps `agentplot/flume`
- **THEN** no fetch of `agentplot/flume` is run, and it is named with `git@github.com:agentplot/flume.git` as the URL git would have used
