# Proposal

## Why

On 2026-11-26, a host updated its runner two times: from 0.29.0 to 0.30.1, and from 0.30.1 to 0.31.2. After each update, the new runner stopped at its first read of a partition's key from the macOS keychain. macOS asked a person to let the new program read the key. Until a person clicked "Always Allow" at the Mac, the runner reached no control plane.

macOS lets a program read a keychain item without an ask only when the item's access names the program's partition. securityd gives a partition to each program from its signature:

- A program that Apple signed gets `apple:`. macOS's own `security` command gets `apple-tool:`.
- A program that a Developer ID or the App Store signed gets `teamid:` and its team.
- Each other signed program gets `cdhash:` and the hash of its own build.

The runner is signed ad hoc, or with the host's own identity (record 0047). So its partition is the hash of its build, and each new build has a new partition. The signed copy of record 0047 keeps a grant of macOS privacy access, but it does not keep the keychain's access. So each update through Nix needs a person at the Mac.

## What Changes

- **The runner reads and writes its key through macOS's own `security`.** On macOS, the keychain store runs `/usr/bin/security` by its path. It reads a key with `find-generic-password -s boomdrive -a <label>@<host> -w`. It writes a key with `security -i`, and the line `add-generic-password -U -s boomdrive -a '<label>@<host>' -X <hex>` goes on standard input. So no argument holds the key. The partition of `security` is `apple-tool:` at each build of the runner. An item that `security` made, or that a person once let `security` read, is read with no ask after each update.
- **The item does not change.** The item is the same generic password: its service is `boomdrive`, and its account is `<label>@<host>`. It holds the key's 32 bytes. A key that the earlier runner kept is read as it is.
- **The keyring crates go.** `keyring-core` and `apple-native-keyring-store` are no longer dependencies.
- **The record and the rules.** Record 0057 holds the decision. The details of core.32 and core.60 say the new case.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `hosted-control-plane`: on macOS, the runner keeps and reads the host's key through macOS's own `security` alone.

## Impact

- `crates/boomdrive-hosted/src/key.rs`: `Keychain` runs `security`. `Keychain::login` names `/usr/bin/security`.
- `crates/boomdrive-hosted/tests/key.rs`: the keychain's tests run over a stub `security`.
- `Cargo.toml`, `crates/boomdrive-hosted/Cargo.toml`, `Cargo.lock`: the keyring crates go.
- An existing host: at the first read after this release, macOS asks one more time, for `security`. A person clicks "Always Allow" one time. After that, no update asks. A host that enrolls after this release gets no ask.
- No protocol version changes. No setting changes.
