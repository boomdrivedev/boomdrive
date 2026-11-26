## MODIFIED Requirements

### Requirement: A partition is enrolled once by boomdrive enroll

`boomdrive enroll --control-plane <url> --drive <drive> --repository <owner/name>…` SHALL enroll the partition `<drive>` at `<url>`. It SHALL make an Ed25519 key in memory and send one `POST runner/enroll`, with the token as its bearer and an `enroll` of version 5 as its body. It SHALL keep the key only once `enrolled` answers, and print `enrolled` or the refusal's words. It SHALL write nothing in the data directory (core.31, core.39, records 0006 and 0053).

#### Scenario: A partition enrolled

- **WHEN** enroll runs with `--drive home --repository example/app` and a token, where the configuration names the partition `home` at the fake's url. The fake control plane answers `enrolled` with host `h-1`.
- **THEN** the request bore the token, and its `enroll` passed version 5. Its signature checked over version 5's context. The key is in the store under `home`, and the enrollment names host `h-1`.

#### Scenario: A lapsed token

- **WHEN** the fake control plane answers `enroll_refused` with the words `the token has lapsed`
- **THEN** enroll refuses with `the token has lapsed`, as the control plane's refusal and not the host's to fix. The store and the runner's file are as they were.

#### Scenario: The answer as JSON

- **WHEN** `boomdrive enroll --json` runs with a token, and `--control-plane` names a loopback port with nothing on it
- **THEN** standard output is one object, `{"refused":{"refusal":{"reason":…}}}`. Its reason names the control plane as unreachable, and it exits 1.

#### Scenario: The token never shown

- **WHEN** `boomdrive enroll` gets the token `secret-enrollment-token`, and `--control-plane` names a loopback port with nothing on it
- **THEN** neither standard output nor standard error holds `secret-enrollment-token`

### Requirement: Every connection proves the host's key

Each upgrade of `runner/connect` SHALL ask for `boomdrive.v5` alone. It SHALL carry `Authorization: BoomdriveHost` with the key, the time, a nonce and a signature over version 5's context, new for each attempt. The runner SHALL keep nothing of enrollment but the key. It SHALL say a refused key or subprotocol once and try again at the backoff's cap (core.22, core.23, core.31, record 0053).

#### Scenario: A connection proved

- **WHEN** partition `home` dials the fake control plane twice
- **THEN** each upgrade asks for `boomdrive.v5` alone, its signature checks over its context of version 5 with `home`'s key, and the two nonces differ

#### Scenario: A key refused

- **WHEN** the fake control plane refuses `home`'s key three times
- **THEN** one line names `home` and the fake's words. The line names the command `boomdrive enroll --control-plane <the fake's url> --drive home --repository example/app`.

#### Scenario: A control plane that speaks version 1 alone

- **WHEN** the fake control plane answers `home`'s upgrades without the subprotocol, three times
- **THEN** one line names `home` and `boomdrive.v5`, each later attempt waits for the backoff's cap, and no message is sent

### Requirement: A host with no partition's key must be fixed

The host can have no credential store, or no partition served the repository can have its key in the store. There, the runner SHALL refuse the ask as the host's to fix, with no request made. The refusal SHALL name the command that enrolls each such partition. The command is `boomdrive enroll --control-plane <url> --drive <label>`, with a `--repository` for each of its repositories (core.32 and core.39, with record 0053).

#### Scenario: No key of the partition

- **WHEN** `home` is served `example/app`, and the store holds no key of `home`
- **THEN** the runner refuses the ask as the host's to fix, naming `boomdrive enroll --control-plane https://control.example --drive home --repository example/app`. The fake control plane took no request.

## REMOVED Requirements

### Requirement: boomdrive enroll refuses what it cannot enroll

**Reason**: enroll takes no label, and it can now write the runner's file. The requirement "boomdrive enroll refuses before the token what it cannot set up" holds its refusals.

**Migration**: Run the command the control plane's page gives, `echo <token> | boomdrive enroll --control-plane <url> --drive <drive> --repository <owner/name>…`.

## ADDED Requirements

### Requirement: boomdrive enroll refuses before the token what it cannot set up

`boomdrive enroll` SHALL exit 2 before it reads the token and before any request in each of these cases:

- A flag is missing or out of form.
- The runner's file neither names the enrollment nor is one that enroll writes.
- The file that enroll would write fails the runner's reading or the hosted implementation's.
- The host has no credential store.

An empty token SHALL also exit 2, with no request. A control plane that enroll cannot reach SHALL exit 1, and enroll SHALL write no key and no file.

#### Scenario: A flag missing

- **WHEN** `boomdrive enroll --control-plane https://control.example --repository example/app` runs
- **THEN** it exits 2, naming `--drive`, and reads no token and makes no request

#### Scenario: A drive name out of form

- **WHEN** `boomdrive enroll --control-plane https://control.example --drive Home --repository example/app` runs
- **THEN** it exits 2, naming `Home` and the form, and reads no token

#### Scenario: A drive another account holds

- **WHEN** enroll runs with `--drive home`. The runner's file holds the drive `home` under the account `own`, which crew serves.
- **THEN** it exits 2, naming the drive `home` and the account `own`. It reads no token and makes no request.

#### Scenario: Nothing listening

- **WHEN** enroll runs with a token on a host with no runner file, and `--control-plane` names a loopback port with nothing on it
- **THEN** it refuses, naming the control plane as unreachable, and exits 1. It writes no key and no runner file.

### Requirement: The token comes on standard input alone

`boomdrive enroll` SHALL take the token from standard input alone, never from a flag, an argument or the environment (core.31). The control plane's page SHALL so give the whole command as `echo <token> | boomdrive enroll …`. Where standard input is a terminal, enroll SHALL ask for the token on standard error and read it with no echo.

#### Scenario: A token flag

- **WHEN** enroll runs with `--token secret-enrollment-token` beside its other flags
- **THEN** it exits 2 as a usage error, naming `--token`, and its output holds no `secret-enrollment-token`

### Requirement: The drive names the account, the drive and the partition

enroll SHALL name the account, the drive and the hosted partition after `--drive`, so no person chooses a partition's label. The account SHALL say `control_plane = "hosted"`. The drive and the partition SHALL each name the `--repository` flags, in their order (core.5 and core.19, with record 0028).

#### Scenario: The names

- **WHEN** enroll writes the runner's file for `--control-plane https://control.example --drive atlas --repository madswan/atlas`
- **THEN** the file holds the account `atlas` with `control_plane = "hosted"`, and its drive `atlas` keeps `madswan/atlas`. `[control_plane.hosted]` names the `url` and the partition `atlas` with `madswan/atlas`.

### Requirement: A file that names the enrollment is left as it is

Both layers, as `run` reads them, can name the enrollment. They then name the hosted `url` given, a partition of the drive's name, and that drive under a hosted account. The partition and the drive each name every repository given. There, enroll SHALL write no file, touch no agent, and enroll the partition.

#### Scenario: A host Nix set up

- **WHEN** enroll runs with a token that the fake control plane enrolls. The runner's file is a link that names the account `atlas`, its drive and the partition `atlas` at the fake's url, each with `madswan/atlas`.
- **THEN** the key is in the store under `atlas`, and the link is as it was. enroll writes no agent, and `config`'s outcome is `unchanged`.

### Requirement: enroll writes the runner's own file on macOS

On macOS, the runner's managed file can be missing, or its first line can be setup's. There, enroll SHALL write the file once the control plane enrolls the partition, and never before. It SHALL set the account, the drive and the partition. It SHALL keep every other account, drive and table as it stands, by setup's own writer and with setup's first line (record 0054 and core.18). Where the file's hosted table names another `url`, enroll SHALL exit 2 before it reads the token, naming both.

#### Scenario: A host with no file

- **WHEN** enroll runs on macOS with a token that the fake control plane enrolls, and the home has no runner file
- **THEN** `~/.config/boomdrive/config.toml` starts with setup's first line and holds the account, the drive and the partition. It reads as the runner's configuration, and `config`'s outcome is `written`.

#### Scenario: Beside crew's drives

- **WHEN** enroll runs with `--drive atlas` on macOS. Setup's file holds the account `own`, which crew serves, with its drive `home` and `[control_plane.crew]`.
- **THEN** the file holds the account `own`, its drive `home` and `[control_plane.crew]` as they stood. It also holds the account `atlas`, its drive and `[control_plane.hosted]`.

#### Scenario: Another hosted control plane

- **WHEN** enroll runs with `--control-plane https://control.example`, and setup's file holds `[control_plane.hosted]` with `url = "https://other.example"`
- **THEN** it exits 2, naming both addresses. It reads no token and writes nothing.

#### Scenario: The token refused

- **WHEN** enroll runs on a host with no file, and the fake control plane answers `enroll_refused`
- **THEN** enroll writes no runner file and no agent

### Requirement: A file Nix or a person wrote is not changed

The runner's managed file can be a link, or a file whose first line is not setup's, that does not name the enrollment. There, enroll SHALL change nothing and read no token. It SHALL exit 2, naming the file and the keys to add. Its words SHALL say that the same command enrolls the host after the change. They SHALL say that the token is not spent (core.17 and core.18).

#### Scenario: A person's own file

- **WHEN** enroll runs, and `~/.config/boomdrive/config.toml` is a file whose first line is not setup's and that names no hosted control plane
- **THEN** it exits 2, naming the file and the TOML keys to add. The file is as it was, and enroll makes no request.

### Requirement: For a file Nix wrote, enroll names the module's options

Where the runner's managed file is a link and does not name the enrollment, enroll SHALL name the keys to add as the options `services.boomdrive.accounts.<drive>` and `services.boomdrive.settings.control_plane.hosted`.

#### Scenario: The file is Nix's

- **WHEN** enroll runs with `--drive atlas --repository madswan/atlas`, and `~/.config/boomdrive/config.toml` is a link that names no hosted control plane
- **THEN** it exits 2, naming the file, `services.boomdrive.accounts.atlas` and `services.boomdrive.settings.control_plane.hosted`. The link and `~/Library/LaunchAgents` are as they were, and enroll makes no request.

### Requirement: Off macOS, enroll writes no file

On a system other than macOS, where the file does not name the enrollment, enroll SHALL change nothing, read no token and exit 2. It SHALL name `services.boomdrive` as where a host of that system sets up its runner.

#### Scenario: On Linux with no file

- **WHEN** enroll's host is Linux and the home has no runner file
- **THEN** it exits 2, naming `linux` and `services.boomdrive`. It writes nothing and makes no request.

### Requirement: The runner's agent runs the file enroll wrote

Where enroll wrote the runner's file on macOS, it SHALL write and load setup's agent as setup does. The runner reads `[control_plane]` when it starts. So where the agent is loaded with its plist unchanged, enroll SHALL run `launchctl kickstart -k gui/<uid>/io.github.agentplot.boomdrive`. Where enroll wrote no file, it SHALL run no `launchctl` (core.13).

#### Scenario: No agent loaded

- **WHEN** enroll writes the runner's file, and `launchctl print` finds no agent
- **THEN** it writes the plist and runs `launchctl bootstrap gui/<uid>` with it, and `service`'s outcome is `loaded`

#### Scenario: The agent running

- **WHEN** enroll changes the runner's file, and `launchctl print` finds the agent with its plist unchanged
- **THEN** it runs `launchctl kickstart -k gui/<uid>/io.github.agentplot.boomdrive` and no `bootstrap`. `service`'s outcome is `restarted`.

### Requirement: The answer names the file and the agent

The answer SHALL keep `partition` and `host`, and add `config`, with `file` and `outcome`, `written` or `unchanged` (core.39). Where enroll wrote the file, the answer SHALL add `service`, with `file` and `outcome`, `loaded`, `reloaded`, `restarted` or `running`. Without `--json`, enroll SHALL say one line for each step.

#### Scenario: The answer

- **WHEN** enroll runs with `--json` on macOS with no runner file, and the fake control plane enrolls host `h-1`
- **THEN** it prints `{"enrolled":{"partition":"atlas","host":"h-1","config":{…,"outcome":"written"},"service":{…,"outcome":"loaded"}}}` and exits 0
