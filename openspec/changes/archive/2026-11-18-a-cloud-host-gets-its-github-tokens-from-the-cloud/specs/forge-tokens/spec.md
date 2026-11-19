## RENAMED Requirements

- FROM: `### Requirement: On a cloud host forge-token refuses until the protocol carries a token`
- TO: `### Requirement: On a cloud host forge-token get gives the control plane's token`
- FROM: `### Requirement: GitHub's word that the App is not installed is kept as a token is`
- TO: `### Requirement: The word that the App is not installed is kept as a token is`

## MODIFIED Requirements

### Requirement: On a cloud host forge-token get gives the control plane's token

Where the configuration's control plane holds the App, `boomdrive forge-token get` SHALL read and check git's request as it does where the runner holds the App's key, then ask the control plane for the repository's token in its own process and keep it no longer. It SHALL print `username=x-access-token`, `password=<token>` and `password_expiry_utc=<its end>` on standard output alone, and exit 0. `erase` SHALL answer as on any host (record 0053; core.2, core.33).

#### Scenario: A get on a cloud host

- **WHEN** the configuration names the hosted control plane, git asks `get` for `agentplot/flume`, which the drive `home` keeps, and the control plane gives a token for it ending at `2026-11-18T13:00:00Z`
- **THEN** standard output holds exactly the three lines with that token and that end in Unix seconds, standard error is empty, and the command exits 0

#### Scenario: Each get on a cloud host asks

- **WHEN** two `get`s on a cloud host ask for `agentplot/flume`
- **THEN** the control plane is asked twice, and each `get` gives the token it was given

#### Scenario: A repository no drive serves on a cloud host

- **WHEN** the configuration names the hosted control plane, and git asks `get` for `someone/else`, which no drive of the host keeps
- **THEN** it exits 1, standard output is empty, standard error names `someone/else` as a repository of no drive this host serves, and the control plane is not asked

#### Scenario: An erase on a cloud host

- **WHEN** the configuration names the hosted control plane, and git asks `erase` with `username` and `password`
- **THEN** it prints nothing, exits 0, and the control plane is not asked

### Requirement: A host with no App, or a configuration that cannot be read, must be fixed

`boomdrive forge-token` SHALL exit 2, printing nothing on standard output, where the configuration cannot be read, or names neither `[forge]` nor a control plane that holds the App, with words on standard error naming what is wrong (core.39; record 0053).

#### Scenario: No [forge]

- **WHEN** the configuration names no `[forge]` and no control plane that holds the App, and git asks for a served repository
- **THEN** it exits 2, standard output is empty, and standard error names `[forge]`, `app_id` and `key_file`

#### Scenario: A configuration it cannot read

- **WHEN** the configuration file is not TOML
- **THEN** it exits 2, standard output is empty, and standard error names the file

### Requirement: No token reaches a file, a log or error output

No token the runner mints, or its control plane gives, SHALL be written to a file, the data directory, the host's credential store or a log. A token SHALL reach no output but `get`'s standard output, and, for the runner's own call, that call's environment. Every refusal SHALL be one line on standard error, `boomdrive forge-token: <words>`, holding no token, JWT or key (core.4, core.35; record 0053).

#### Scenario: Nothing is left behind

- **WHEN** a `get` answers with a token, and a later `get` is refused by GitHub
- **THEN** the scratch host's data and configuration directories hold no file they did not hold before, and neither standard error holds the token

#### Scenario: Nothing is left behind on a cloud host

- **WHEN** a `get` on a cloud host answers with the control plane's token, and a later `get` is refused by the control plane
- **THEN** the scratch host's data and configuration directories hold no file they did not hold before, and neither standard error holds the token

### Requirement: The running runner keeps one token for each repository for its own git calls

Where its configuration names `[forge]`, or its control plane holds the App, the running runner SHALL keep in memory one token for each served repository it fetches, minted for the App the configuration names or given by the control plane, and SHALL give it again until five minutes before its end, then mint or ask for another (record 0007 as amended on 2026-11-11 and 2026-11-18; record 0053).

#### Scenario: A token kept across passes

- **WHEN** two passes keep `agentplot/flume`, with `[forge]` named and the App installed on it
- **THEN** GitHub is asked for one token

#### Scenario: A token near its end

- **WHEN** the token GitHub gives for `agentplot/flume` ends in four minutes, and the runner asks for its credential twice
- **THEN** GitHub is asked for a token each time

#### Scenario: A cloud host's token kept across passes

- **WHEN** two passes keep `agentplot/flume` on a cloud host, its control plane giving a token for it that ends in an hour
- **THEN** the control plane is asked once

#### Scenario: A cloud host's token near its end

- **WHEN** the token the control plane gives for `agentplot/flume` ends in four minutes, and the runner asks for its credential twice
- **THEN** the control plane is asked each time

### Requirement: A kept token is dropped when its fetch fails or its App changes

A fetch that fails SHALL drop its repository's kept token. A different App, by `app_id` or `key_file`, SHALL drop every token kept. A refused mint, or a refused ask of the control plane, SHALL be kept not at all (record 0007 as amended on 2026-11-11; record 0053).

#### Scenario: A failed fetch drops its token

- **WHEN** a pass's fetch of `agentplot/flume` fails, and the next pass keeps it
- **THEN** GitHub is asked for a token at each pass

#### Scenario: Another App

- **WHEN** the runner asks for `agentplot/flume`'s credential, then asks again with `[forge]`'s `key_file` naming another key
- **THEN** GitHub is asked for a token each time, the second with a JWT the other key signed

#### Scenario: A cloud host's failed fetch drops its token

- **WHEN** a pass's fetch of `agentplot/flume` on a cloud host fails, and the next pass keeps it
- **THEN** the control plane is asked for a token at each pass

#### Scenario: A cloud host's refusal kept not at all

- **WHEN** the control plane refuses `agentplot/flume`'s token, and the runner asks for its credential twice
- **THEN** the control plane is asked each time

### Requirement: The word that the App is not installed is kept as a token is

GitHub's answer that the App has no installation on a repository, or the control plane's that the App's installation does not cover it, SHALL be kept by the running runner for fifty-five minutes, the hour of a token less the five minutes before its end, so such a repository costs one ask an hour (record 0007 as amended on 2026-11-11; record 0053).

#### Scenario: The App's absence kept

- **WHEN** GitHub has no installation of the App on `agentplot/flume`, and the runner asks for its credential twice
- **THEN** GitHub is asked for the App's installation on it once, and for no token

#### Scenario: The control plane's word kept

- **WHEN** the control plane says the App's installation does not cover `agentplot/flume`, and the runner asks for its credential twice
- **THEN** the control plane is asked once

## ADDED Requirements

### Requirement: On a cloud host forge-token get refuses what the control plane does not give

`get` on a cloud host SHALL exit 1, printing nothing on standard output, where the control plane gives no token: standard error SHALL say to install the App on the repository where its installation does not cover it, and otherwise SHALL carry the hosted implementation's words, with the control plane's as given. Where the host must be fixed, as where no partition's key is in its store, it SHALL exit 2 (core.1, core.39; record 0053).

#### Scenario: The App not installed on a cloud host

- **WHEN** git asks `get` for `agentplot/flume` on a cloud host, and the control plane says the App's installation does not cover it
- **THEN** it exits 1, standard output is empty, and standard error says to install the app on `agentplot/flume` and holds the control plane's words

#### Scenario: A cloud host's refusal

- **WHEN** git asks `get` for `agentplot/flume` on a cloud host, and the control plane refuses it with the words `the drive's account is closed`
- **THEN** it exits 1, standard output is empty, and standard error is one line, `boomdrive forge-token: ` and words holding `the drive's account is closed`

#### Scenario: A cloud host with no partition's key

- **WHEN** `boomdrive forge-token get` runs on a scratch host whose configuration names the hosted control plane at a loopback port with nothing on it, its partition `home` served `agentplot/flume`, the host's store holds no key of `home`, and git asks for `agentplot/flume`
- **THEN** it exits 2, standard output is empty, and standard error is one line naming `boomdrive enroll home`
