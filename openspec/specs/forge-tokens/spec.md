# forge-tokens Specification

## Purpose
How the runner gives every tool on its host a short-lived forge token for a served repository, on a host whose control plane does not hold the Boomdrive App: it mints one from the App's key its configuration's `[forge]` names, gives it in git's credential-helper protocol by `boomdrive forge-token`, keeps one for each repository for its own fetches, and refuses on a cloud host until the protocol carries a token (record 0007 as amended on 2026-11-09 and 2026-11-11; core.28, core.33).

## Requirements

### Requirement: The runner mints a token for one repository from the App's key

Where its configuration names `[forge]`, the runner SHALL mint a token for a repository from the App's key that `key_file` names. It SHALL sign a JWT with the key, RS256, whose `iss` is the App's id, find the App's installation on the repository, and buy a token from that installation for that repository alone. GitHub's answer that the App has no installation there SHALL be an answer of its own, and nothing SHALL then be bought (record 0007 as amended; core.28, core.33).

#### Scenario: A token for one repository

- **WHEN** the runner mints for `agentplot/flume` with `[forge] app_id = 1`, and GitHub has the App installed on it
- **THEN** it finds the installation and buys a token from it with the body `{"repositories":["flume"]}`, each call with a JWT whose `iss` is `1` and which the key's public half verifies, and it gives the token and its end

#### Scenario: The App is not installed

- **WHEN** GitHub answers 404 to finding the App's installation on `agentplot/flume`
- **THEN** the mint gives that the App is not installed there, and buys no token

### Requirement: A key the runner cannot use refuses the mint

A key file that cannot be read, that anyone but its owner may read, or that holds no RSA private key in PEM SHALL refuse the mint as a host to fix, naming the file, with no call made to GitHub (record 0007 as amended on 2026-11-11; core.4).

#### Scenario: A key others may read

- **WHEN** the key file has mode 0644
- **THEN** the mint is refused as a host to fix, naming the file, its mode and `chmod 600`, and GitHub is not called

#### Scenario: A key file that is no key

- **WHEN** the key file holds no RSA private key in PEM
- **THEN** the mint is refused as a host to fix, naming the file, and GitHub is not called

### Requirement: GitHub's refusal and silence are named, and hold no secret

Each of a mint's two calls to GitHub's API SHALL be stopped at ten seconds. A call GitHub refuses, or one that cannot be made or is stopped, SHALL refuse the mint with words naming the call and GitHub's status and message, or why it could not be made. No refusal's words SHALL hold the token, the JWT or the key (record 0007 as amended on 2026-11-11; core.4).

#### Scenario: GitHub refuses the App

- **WHEN** GitHub answers 401 with the message `A JSON web token could not be decoded`
- **THEN** the mint is refused, naming the call, 401 and the message, and its words hold neither the JWT nor any line of the key

#### Scenario: GitHub's API never answers

- **WHEN** the API takes the connection and never answers, and the forge's calls stop at 500 ms
- **THEN** the mint is refused within 5 seconds, naming the call

### Requirement: boomdrive forge-token get gives a served repository's token on standard output alone

`boomdrive forge-token get` SHALL read the configuration as `boomdrive run` reads it, and git's `protocol`, `host` and `path` from standard input. For `https`, `github.com` and a repository of a drive the host serves, it SHALL mint a token in its own process and keep it no longer. It SHALL print `username=x-access-token`, `password=<token>` and `password_expiry_utc=<its end>` on standard output alone, and exit 0 (record 0007 as amended; core.33).

#### Scenario: A served repository

- **WHEN** git sends `protocol=https`, `host=github.com` and `path=agentplot/flume.git`, and the drive `home` keeps `agentplot/flume`
- **THEN** standard output holds exactly the three lines with the token GitHub minted for `agentplot/flume` and its end, standard error is empty, and the command exits 0

#### Scenario: A repository the local layer adds

- **WHEN** git asks for `agentplot/sluice`, which the local layer adds to the drive `home`
- **THEN** it is given a token for `agentplot/sluice`

#### Scenario: Each get mints

- **WHEN** two `get`s ask for `agentplot/flume`
- **THEN** each gives a token GitHub minted for it, and the second is not the first

### Requirement: boomdrive forge-token get refuses what it may not give

`get` SHALL refuse with exit 1, printing nothing on standard output and its words on standard error: a request for another protocol or host, one with no path, a path that is no repository's address, a repository of no drive the host serves, and one the App is not installed on (core.33).

#### Scenario: A repository no drive serves

- **WHEN** git asks for `someone/else`, which no drive of the host keeps
- **THEN** it exits 1, standard output is empty, standard error names `someone/else` as a repository of no drive this host serves, and GitHub is not called

#### Scenario: Another host

- **WHEN** git sends `protocol=https`, `host=example.com` and `path=agentplot/flume`
- **THEN** it exits 1, standard output is empty, and standard error says the runner gives tokens for `https://github.com` alone

#### Scenario: No path

- **WHEN** git sends `protocol=https` and `host=github.com` and no `path`
- **THEN** it exits 1, standard output is empty, and standard error names `credential.useHttpPath`

#### Scenario: The App is not installed on the repository

- **WHEN** git asks for `agentplot/flume`, and GitHub has no installation of the App on it
- **THEN** it exits 1, standard output is empty, and standard error says to install the app on `agentplot/flume`

### Requirement: boomdrive forge-token erase revokes nothing, and the next get never gives the erased token

`boomdrive forge-token erase` SHALL read what it is sent to its end, print nothing and exit 0. Since no process keeps a token it gave, it SHALL drop nothing kept and revoke nothing at GitHub, and the next `get` SHALL mint afresh, never giving the erased token (record 0007 as amended on 2026-11-11).

#### Scenario: A token erased and asked for again

- **WHEN** a tool erases the token a `get` gave it for `agentplot/flume`, with `username` and `password`, and then gets again
- **THEN** the `erase` prints nothing and exits 0, the second `get` gives a token GitHub minted afresh and not the erased one, and GitHub was asked to revoke nothing

### Requirement: Any other operation is a usage error

`boomdrive forge-token` with an operation other than `get` and `erase`, `store` among them, SHALL print its usage on standard error, nothing on standard output, and exit 2 (core.39).

#### Scenario: git asks it to store

- **WHEN** `boomdrive forge-token store` is run
- **THEN** it prints its usage on standard error and nothing on standard output, and exits 2

### Requirement: A host with no App, or a configuration that cannot be read, must be fixed

`boomdrive forge-token` SHALL exit 2, printing nothing on standard output, where the configuration cannot be read or names no `[forge]`, with words on standard error naming what is wrong (core.39).

#### Scenario: No [forge]

- **WHEN** the configuration names no `[forge]` and git asks for a served repository
- **THEN** it exits 2, standard output is empty, and standard error names `[forge]`, `app_id` and `key_file`

#### Scenario: A configuration it cannot read

- **WHEN** the configuration file is not TOML
- **THEN** it exits 2, standard output is empty, and standard error names the file

### Requirement: On a cloud host forge-token refuses until the protocol carries a token

Where the configuration's control plane is hosted, `boomdrive forge-token` SHALL refuse every operation with exit 1, printing nothing on standard output. Standard error SHALL say that the hosted control plane mints the host's forge tokens and that version 2 of its protocol carries none yet. The words SHALL be the hosted implementation's own (record 0007 as amended on 2026-11-09; core.2, core.33).

#### Scenario: A get on a cloud host

- **WHEN** the configuration names the hosted control plane and git asks `get` for a served repository
- **THEN** it exits 1, standard output is empty, standard error says the hosted control plane mints this host's forge tokens and version 2 of its protocol carries none yet, and nothing is minted

#### Scenario: An erase on a cloud host

- **WHEN** the configuration names the hosted control plane and git asks `erase`
- **THEN** it exits 1 with the same words, and standard output is empty

### Requirement: No token reaches a file, a log or error output

No token the runner mints SHALL be written to a file, the data directory, the host's credential store or a log. A token SHALL reach no output but `get`'s standard output, and, for the runner's own call, that call's environment. Every refusal SHALL be one line on standard error, `boomdrive forge-token: <words>`, holding no token, JWT or key (core.4, core.35).

#### Scenario: Nothing is left behind

- **WHEN** a `get` answers with a token, and a later `get` is refused by GitHub
- **THEN** the scratch host's data and configuration directories hold no file they did not hold before, and neither standard error holds the token

### Requirement: The running runner keeps one token for each repository for its own git calls

Where its configuration names `[forge]`, the running runner SHALL keep in memory one token for each served repository it fetches, minted for the App the configuration names, and SHALL give it again until five minutes before its end, then mint another (record 0007 as amended on 2026-11-11).

#### Scenario: A token kept across passes

- **WHEN** two passes keep `agentplot/flume`, with `[forge]` named and the App installed on it
- **THEN** GitHub is asked for one token

#### Scenario: A token near its end

- **WHEN** the token GitHub gives for `agentplot/flume` ends in four minutes, and the runner asks for its credential twice
- **THEN** GitHub is asked for a token each time

### Requirement: A kept token is dropped when its fetch fails or its App changes

A fetch that fails SHALL drop its repository's kept token. A different App, by `app_id` or `key_file`, SHALL drop every token kept. A refused mint SHALL be kept not at all (record 0007 as amended on 2026-11-11).

#### Scenario: A failed fetch drops its token

- **WHEN** a pass's fetch of `agentplot/flume` fails, and the next pass keeps it
- **THEN** GitHub is asked for a token at each pass

#### Scenario: Another App

- **WHEN** the runner asks for `agentplot/flume`'s credential, then asks again with `[forge]`'s `key_file` naming another key
- **THEN** GitHub is asked for a token each time, the second with a JWT the other key signed

### Requirement: GitHub's word that the App is not installed is kept as a token is

GitHub's answer that the App has no installation on a repository SHALL be kept by the running runner for fifty-five minutes, the hour of a token less the five minutes before its end, so such a repository costs one call an hour (record 0007 as amended on 2026-11-11).

#### Scenario: The App's absence kept

- **WHEN** GitHub has no installation of the App on `agentplot/flume`, and the runner asks for its credential twice
- **THEN** GitHub is asked for the App's installation on it once, and for no token
