---
number: 57
title: On macOS the runner keeps its key through macOS's own security, so an update reads it with no ask
status: accepted
date: 2026-11-26
decision-makers:
- the construct stage of fix/an-update-keeps-keychain-access
---

# On macOS the runner keeps its key through macOS's own security, so an update reads it with no ask

Status: accepted. The direction is the brief's: a runner update keeps its keychain access with no click. How the runner gets there is the construct stage's own choice, made on the rules as written. It lives in the hosted implementation's keychain store alone.

Deciders: the construct stage of `fix/an-update-keeps-keychain-access`, from its brief.

Sources: the fault on a host on 2026-11-26, where the runner stopped at its first keychain read after each update, from 0.29.0 to 0.30.1 and from 0.30.1 to 0.31.2, until a person clicked "Always Allow"; Apple's open source of the Security project, `securityd/src/clientid.cpp` (`partitionIdForProcess`), `SecurityTool/macOS/keychain_find.c`, `keychain_add.c` and `security.c`; records 0006, 0018 and 0047; rules core.4, core.32, core.47 and core.60; the OpenSpec change `an-update-keeps-keychain-access`, whose design gives each point below.

## Context and problem statement

Before a program reads a keychain item, macOS checks that the item's access list names the program and that the item's partition list holds the program's partition. securityd gives the partition from the program's signature. Apple's code gets `apple:`, and macOS's own `security` gets `apple-tool:`. Code that a Developer ID, the App Store or TestFlight signed gets `teamid:<team>`. Each other signed program gets `cdhash:<the hash of its build>`.

The runner kept its key through the keyring crates, in its own process. Its signature is the linker's ad hoc one, or the host's self-signed identity on nix-darwin (record 0047). Neither is Apple's, so its partition is the hash of each build. Record 0047 kept a grant of macOS privacy access across builds, and said that it did not cover the keychain's ask. After each update, the new build failed the partition check, and the runner waited for a person at the Mac.

## Considered options

1. **The runner reads and writes the key through `/usr/bin/security`** (chosen).
2. **The keychain's access names the signed copy of record 0047.**
3. **A launcher at a stable path that the module owns.**
4. **A Developer ID, or an access group of the data protection keychain.**
5. **The runner sets the item's partition list.**
6. **The item allows any program (`-A`).**

## Decision outcome

Option 1.

- **Before:** the runner read and wrote its key in its own process, through the keyring crates, so macOS knew the reader by the hash of each build, and asked a person after each update.
- **Now:** the runner reads and writes its key through macOS's own `security`, by its path. Its partition, `apple-tool:`, is the same at each build of the runner, so no update asks.

- **Read.** `security find-generic-password -s boomdrive -a <label>@<host> -w`. Exit 44 is no key. `security` prints the 32 bytes of a key as 64 hex digits where a byte is not printable, and as the bytes themselves where each is.
- **Write.** `security -i`, with the line `add-generic-password -U -s boomdrive -a '<label>@<host>' -X <hex>` on standard input. The key is never in an argument (core.32).
- **The item stays as it is.** Its service, its account and its 32 bytes do not change, so a key that the earlier runner kept is read as it is. The runner has no code for an earlier form.

Not taken:

- **Option 2.** The access list already matches the copy across builds. The partition check fails all the same, because the copy's partition is the hash of each build.
- **Option 3.** macOS knows a program by its signature. A launcher that Nix builds gets a new hash when its own inputs change.
- **Option 4.** Each needs Apple's Developer Program, which record 0047 did not take, for its fee, its published name and a build that differs from the one a host makes from source (core.47). A person can still decide on it in a record of its own.
- **Option 5.** A change of an item's partition list needs the keychain's password, which the runner does not hold (core.4).
- **Option 6.** It drops the check for every program, not only for `security`.

### Consequences

- No update of the runner asks for the keychain again.
- Any program that runs as the user can read the key with `security` and no ask. Before, such a program got an ask. On Linux, any program that runs as the user can already decrypt the user's `systemd-creds` credential. So both stores now keep the key for the host and the user, and neither keeps it in a file in the clear (core.32).
- A host that enrolled before this record holds an item that names the earlier runner. At the first read after the update, macOS asks one more time, for `security`. A person clicks "Always Allow" one time, which adds `security` and `apple-tool:` to the item. A host that enrolls after this record gets no ask.
- The keyring crates are no longer dependencies.
- This record amends the details of core.32 and core.60. The change `an-update-keeps-keychain-access` builds it.
