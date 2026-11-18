## MODIFIED Requirements

### Requirement: On nix-darwin the runner is a launchd agent of the primary user's login

With `services.boomdrive.enable`, the nix-darwin module SHALL declare `launchd.user.agents.boomdrive`, loaded in `system.primaryUser`'s login. It SHALL run `boomdrive run --config <the file>`, start at load, be kept alive at most once in 30 seconds, run as interactive, leave its process group when stopped, be given `HOME` and `PATH` alone by the module, beside any variable the host gives the job, and log to `logFile`. Evaluation SHALL fail where the primary user has no home.

#### Scenario: The agent

- **WHEN** the module is enabled for the primary user `alice`, whose home is `/Users/alice`
- **THEN** the agent runs `boomdrive run --config` with the file, `RunAtLoad`, `KeepAlive`, `ThrottleInterval` 30, `ProcessType` `Interactive` and `AbandonProcessGroup`, with `HOME` `/Users/alice`, and logs to `/Users/alice/Library/Logs/boomdrive.log`

#### Scenario: No home for the primary user

- **WHEN** the module is enabled and `users.users.alice.home` is not set
- **THEN** evaluation fails, naming the primary user and the option

### Requirement: On nix-darwin the agent waits for the Nix store

The nix-darwin module's agent SHALL run its command behind `/bin/wait4path /nix/store`, the shell handing its process to the runner by `exec`, whether it runs the signed copy or the package's own binary, so an agent loaded before the Nix store's volume is mounted waits for it, and a stop's signal reaches the runner.

#### Scenario: The agent waits for the Nix store

- **WHEN** the module is enabled, with no `settings` or with a crew control plane's
- **THEN** the agent's program is `/bin/sh -c` with a line that runs `/bin/wait4path /nix/store`, then `exec`s `boomdrive run --config` with the file, by the signed copy or by the package's `bin/boomdrive`, each by `exec`

## ADDED Requirements

### Requirement: On nix-darwin the agent runs the signed copy of its own package

The nix-darwin module's agent SHALL run the runner by `/Library/Application Support/Boomdrive/boomdrive` where `boomdrive.source` beside it names `<package>/bin/boomdrive`, and by `<package>/bin/boomdrive` where it does not, so it never runs a copy of another build.

#### Scenario: The agent runs the copy of its own package

- **WHEN** the module is enabled
- **THEN** the agent's line compares `/Library/Application Support/Boomdrive/boomdrive.source` with the package's `bin/boomdrive`, `exec`s `/Library/Application Support/Boomdrive/boomdrive run --config` with the file where they are one, and `exec`s the package's `bin/boomdrive run --config` with the file where they are not

### Requirement: On nix-darwin the runner keeps one code identity across builds

With `services.boomdrive.enable`, each activation SHALL, as root and before nix-darwin loads its agents, keep a copy of `<package>/bin/boomdrive` at `/Library/Application Support/Boomdrive/boomdrive`, signed with the host's code-signing identity under the identifier `boomdrive`, and name its program in `boomdrive.source` beside it. It SHALL make a copy only where none is there, or where it was made from another program or is not signed with the identity.

#### Scenario: The step runs before the agents load

- **WHEN** the module is enabled
- **THEN** `system.activationScripts.extraActivation.text` runs `activate.sh sign` with `/Library/Keychains/System.keychain`, `/Library/Application Support/Boomdrive` and the package's `bin/boomdrive`, its failure stopping nothing, and `postActivation.text` runs no `sign`

#### Scenario: A new build

- **WHEN** the step runs for a program other than the one `boomdrive.source` names
- **THEN** it copies the program, signs the copy by the identity's SHA-1 with `--identifier boomdrive`, `--timestamp=none` and the keychain, removes `boomdrive.source`, puts the copy at `boomdrive` by a rename, then writes the program's path to `boomdrive.source`, and says one line naming the program

#### Scenario: The same build

- **WHEN** the step runs for the program `boomdrive.source` names, and the copy is signed with the identity
- **THEN** it signs nothing, changes no file and says nothing

#### Scenario: A copy signed with another identity

- **WHEN** the step runs for the program `boomdrive.source` names, and the copy is not signed with the identity
- **THEN** it signs a new copy, as for a new build

### Requirement: No step signs without the runner

Where a nix-darwin system imports the module and `services.boomdrive.enable` is off, its activation SHALL run no `activate.sh sign`.

#### Scenario: No signing without the runner

- **WHEN** a nix-darwin system imports the module and leaves `services.boomdrive.enable` off
- **THEN** neither `extraActivation.text` nor `postActivation.text` runs `activate.sh sign`

### Requirement: The copy's source never names a program the copy was not made from

The step SHALL remove `boomdrive.source` before it replaces the copy, and SHALL write it only once the copy is in place, so an agent never runs a copy made from a program other than its package's.

#### Scenario: A copy not put in place

- **WHEN** the signed copy's rename to `boomdrive` fails
- **THEN** `boomdrive` is as it was, `boomdrive.source` is not there, no other file is left in the folder, and the step says one line with the rename's words and exits 1

### Requirement: A copy that cannot be signed stops nothing

Where the copy cannot be made, signed or put in place, the step SHALL leave `boomdrive` as it was and no other file in the folder, SHALL say one line with the words of what failed, and SHALL exit 1, and the activation SHALL go on. Where the copy cannot be made or signed, it SHALL leave `boomdrive.source` as it was.

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

### Requirement: The README says the runner's grant is asked once

The README's install with nix SHALL say that on nix-darwin each switch signs a copy of the runner at `/Library/Application Support/Boomdrive/boomdrive` with an identity the host makes once, `Boomdrive runner`, kept in its System keychain, that the agent runs that copy, and that macOS asks once for what the runner reads, a later build asking nothing again. It SHALL say that where the copy cannot be signed the agent runs the package's own runner and the switch says why.

#### Scenario: The README's signed runner

- **WHEN** the README's install with nix is read
- **THEN** its words name `/Library/Application Support/Boomdrive/boomdrive`, `Boomdrive runner` and the System keychain, and say that macOS asks once and a later build asks nothing again, and that an unsigned copy leaves the agent running the package's own runner
