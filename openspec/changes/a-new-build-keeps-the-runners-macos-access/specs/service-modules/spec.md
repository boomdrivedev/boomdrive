## MODIFIED Requirements

### Requirement: On nix-darwin the runner is a launchd agent of the primary user's login

With `services.boomdrive.enable`, the nix-darwin module SHALL declare `launchd.user.agents.boomdrive`, loaded in `system.primaryUser`'s login. It SHALL run `boomdrive run --config <the file>` by the copy the activation signs, `/Library/Application Support/Boomdrive/boomdrive`, where `boomdrive.source` beside it names `<package>/bin/boomdrive`, and by `<package>/bin/boomdrive` where it does not. It SHALL start at load, be kept alive at most once in 30 seconds, run as interactive, leave its process group when stopped, have only `HOME` and `PATH` in its environment, and log to `logFile`. Evaluation SHALL fail where the primary user has no home.

#### Scenario: The agent

- **WHEN** the module is enabled for the primary user `alice`, whose home is `/Users/alice`
- **THEN** the agent runs `boomdrive run --config` with the file, by `/Library/Application Support/Boomdrive/boomdrive` where `/Library/Application Support/Boomdrive/boomdrive.source` holds the package's `bin/boomdrive` and by the package's `bin/boomdrive` where it does not, with `RunAtLoad`, `KeepAlive`, `ThrottleInterval` 30, `ProcessType` `Interactive` and `AbandonProcessGroup`, with `HOME` `/Users/alice`, and logs to `/Users/alice/Library/Logs/boomdrive.log`

#### Scenario: No home for the primary user

- **WHEN** the module is enabled and `users.users.alice.home` is not set
- **THEN** evaluation fails, naming the primary user and the option

### Requirement: On nix-darwin the agent waits for the Nix store

The nix-darwin module's agent SHALL run its command behind `/bin/wait4path /nix/store`, the shell handing its process to the runner by `exec`, whether it runs the signed copy or the package's own binary, so an agent loaded before the Nix store's volume is mounted waits for it, and a stop's signal reaches the runner.

#### Scenario: The agent waits for the Nix store

- **WHEN** the module is enabled, with no `settings` or with a crew control plane's
- **THEN** the agent's program is `/bin/sh -c` with a line that runs `/bin/wait4path /nix/store`, then `exec`s `boomdrive run --config` with the file, by the signed copy or by the package's `bin/boomdrive`, each by `exec`

## ADDED Requirements

### Requirement: On nix-darwin the runner keeps one code identity across builds

At each activation, as root and before nix-darwin loads its agents, the nix-darwin module SHALL keep a copy of `<package>/bin/boomdrive` at `/Library/Application Support/Boomdrive/boomdrive`, signed with the host's code-signing identity under the identifier `boomdrive`, and SHALL name the program it was made from in `boomdrive.source` beside it. It SHALL make a copy only where none is there, where it was made from another program, or where it is not signed with the identity.

#### Scenario: The step runs before the agents load

- **WHEN** the module is enabled
- **THEN** `system.activationScripts.extraActivation.text` runs `activate.sh sign` with `/Library/Keychains/System.keychain`, `/Library/Application Support/Boomdrive` and the package's `bin/boomdrive`, its failure stopping nothing, and `postActivation.text` runs no `sign`

#### Scenario: A new build

- **WHEN** the step runs for a program other than the one `boomdrive.source` names
- **THEN** it copies the program, signs the copy by the identity's SHA-1 with `--identifier boomdrive`, `--timestamp=none` and the keychain, puts the copy at `boomdrive` by a rename, writes the program's path to `boomdrive.source`, and says one line naming the program

#### Scenario: The same build

- **WHEN** the step runs for the program `boomdrive.source` names, and the copy is signed with the identity
- **THEN** it signs nothing, changes no file and says nothing

#### Scenario: A copy signed with another identity

- **WHEN** the step runs for the program `boomdrive.source` names, and the copy is not signed with the identity
- **THEN** it signs a new copy, as for a new build

### Requirement: A copy that cannot be signed stops nothing

Where the copy cannot be made or signed, the step SHALL leave `boomdrive` and `boomdrive.source` as they were and no other file in the folder, SHALL say one line with the words of what failed, and SHALL exit 1, and the activation SHALL go on.

#### Scenario: A signing refused

- **WHEN** `codesign` refuses to sign the copy
- **THEN** `boomdrive` and `boomdrive.source` are as they were, no other file is left in the folder, and the step says one line with `codesign`'s words and exits 1

### Requirement: The host makes its code-signing identity once and keeps it in the System keychain

The identity SHALL be a self-signed certificate for code signing named `Boomdrive runner` with its key, kept in `/Library/Keychains/System.keychain` and made by the step where that keychain holds none. Its key SHALL be imported non-extractable and usable by `codesign` alone, and SHALL be in no file once imported.

#### Scenario: No identity yet

- **WHEN** the step runs and the keychain holds no identity for code signing named `Boomdrive runner`
- **THEN** it makes a key and a self-signed certificate for code signing with that name in a folder of its own, imports the key into the keychain with `-x` and `-T /usr/bin/codesign` and the certificate after it, removes that folder, says one line naming the identity's SHA-1, and signs the copy with it

#### Scenario: No key in a file

- **WHEN** the step has made the identity
- **THEN** no file the step made holds the key or the certificate

#### Scenario: An identity that cannot be made

- **WHEN** the keychain refuses the key's import
- **THEN** the step signs nothing, removes the folder it made, says one line with the keychain's words, and exits 1

#### Scenario: Two identities of the name

- **WHEN** the keychain holds two identities for code signing named `Boomdrive runner`
- **THEN** the step signs nothing, says one line naming the SHA-1 of each, and exits 1

### Requirement: Only root writes the signed copy's folder

The step SHALL sign into its folder only where the folder is owned by the user the step runs as, root under nix-darwin, and is writable by no other user; otherwise it SHALL make no identity, sign nothing and write nothing there.

#### Scenario: A folder another user can write

- **WHEN** the folder is writable by its group or by others, or is owned by another user
- **THEN** the step makes no identity, signs nothing, writes nothing in the folder, says one line naming the folder, and exits 1
