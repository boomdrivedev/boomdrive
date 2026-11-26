# Design

## Context

A partition's key is an Ed25519 key of 32 bytes. On macOS, the runner keeps it in the user's keychain as a generic password (record 0006, record 0018, core.32). The service of the item is `boomdrive`, and its account is `<label>@<host>`. Until this change, the runner used the keyring crates, which call the Security framework in the runner's own process. So the program that reads the item is the runner.

The keychain checks two things before a program reads an item:

- The item's access list names the program. For a program without a bundle, it names the program's path and its designated requirement.
- The item's partition list holds the program's partition. securityd gives the partition from the program's signature (`securityd/src/clientid.cpp`, `partitionIdForProcess`). Apple's code gets `apple:`, and `security` gets `apple-tool:`. Code that a Developer ID, the App Store or TestFlight signed gets `teamid:<team>`. Each other signed program gets `cdhash:<the hash of its build>`.

When a check fails, SecurityAgent asks a person. "Always Allow" adds the program to the access list and its partition to the partition list.

The set builds the runner with the linker's ad hoc signature. On nix-darwin, the activation signs a copy with the host's own self-signed identity (record 0047). Its designated requirement does not change between builds, but no certificate of Apple's signed it. So its partition is `cdhash:` of each build, and each update fails the partition check.

## Decision 1: the keychain is reached through `/usr/bin/security`

`Keychain` runs macOS's own `security` by its path, `/usr/bin/security`, so no program on `PATH` stands in for it.

- **Read.** `security find-generic-password -s boomdrive -a <label>@<host> -w`, with standard input closed. Exit 44, the low byte of `errSecItemNotFound`, is no key. Each other failure is `security`'s words on standard error. `-w` prints the item's bytes, then a newline. Where each byte is printable, it prints the bytes themselves. Where a byte is not printable, it prints each byte as two lowercase hex digits (`keychain_find.c`). A key's item holds 32 bytes, so 64 hex digits are the key in hex, and 32 bytes are the key itself. Other output is no key.
- **Write.** `security -i`, with one line on standard input: `add-generic-password -U -s boomdrive -a '<label>@<host>' -X <the key in hex>`. `-U` replaces an item that is there. `-X` keeps the 32 bytes that the hex spells. The account is in single quotes, with a backslash before each quote and backslash in it, as `security -i` splits a line. In `-i`, `security` says a command that failed as `<command>: returned <status>` on standard error. So a write fails where `security` exits other than 0 or says anything on standard error.

The key is never in an argument. It goes in on standard input and comes back on standard output, as with `systemd-creds` (core.32). The runner keeps the hex text and the line in memory that it zeroes.

An item that `security` makes has `security` in its access list and `apple-tool:` in its partition list. So `security` reads it with no ask, whatever build of the runner runs it.

## Decision 2: the item stays as it is

The service, the account and the 32 bytes do not change. A key that the keyring crates kept is read as it is. The runner has no code for an earlier form. On such an item, macOS asks one more time, for `security`, and "Always Allow" adds `security` and `apple-tool:` to the item. After that, no build asks again.

## Decision 3: the keyring crates go

Nothing else uses `keyring-core` or `apple-native-keyring-store`. The tests ran the keychain store over keyring's mock store. Now they run it over a stub `security` that the test writes, as the tests of `systemd-creds` do. `Keychain::open` takes the program, as `SystemdCreds::open` does. `Keychain::login` names `/usr/bin/security`.

## What the runner gives up

Any program that runs as the user can run `security find-generic-password` and read the key with no ask. Before this change, such a program got an ask. On Linux, any program that runs as the user can already decrypt the user's `systemd-creds` credential. So this change puts both stores at one level: the key is kept for the host and the user. It is still never in a file in the clear (core.32).

## Options not taken

- **The keychain's access names the signed copy of record 0047.** The access list already matches the copy's designated requirement across builds. But the partition of the copy is the hash of each build, so the partition check fails at each update.
- **A stable launcher path that the module owns.** The keychain knows a program by its signature, not only by its path. A launcher that Nix builds gets a new hash when its own inputs change.
- **A Developer ID for the runner.** It would give `teamid:`, which does not change. Record 0047 did not take a Developer ID. It costs Apple's yearly fee, and it publishes a name. Also, a signed build differs from the build that a host makes from source (core.47). A person can still decide on it in a record of its own.
- **The data protection keychain with an access group.** An access group needs a provisioning profile from Apple's Developer Program, for the same reason.
- **The partition list set by the runner.** A change of an item's partition list needs the keychain's password. The runner holds no password (core.4).
- **Any program allowed (`-A`).** It drops the access list and the partition check for every program, not only for `security`.
