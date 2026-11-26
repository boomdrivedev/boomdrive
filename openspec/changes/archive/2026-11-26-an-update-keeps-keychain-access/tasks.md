# Tasks

## 1. The keychain through `security`

- [x] 1.1 `Keychain` reads and writes the key through `/usr/bin/security`, the key on standard input and standard output alone (Decision 1). True when `devenv shell -- cargo test -p boomdrive-hosted --test key` passes, proving "Kept in the keychain through security", "A key of printable bytes" and "The keychain refuses".
- [x] 1.2 The keyring crates go (Decision 3). True when `Cargo.lock` holds neither `keyring-core` nor `apple-native-keyring-store`.

## 2. The record and the rules

- [x] 2.1 Record 0057 holds the decision. The details of core.32 and core.60 say the new case.

## 3. The gate

- [x] 3.1 The projects' tasks pass. True when `sluice exec main -- moon run boomdrive-hosted:format boomdrive-hosted:lint boomdrive-hosted:typecheck boomdrive-hosted:test root:specs root:changes` passes.
