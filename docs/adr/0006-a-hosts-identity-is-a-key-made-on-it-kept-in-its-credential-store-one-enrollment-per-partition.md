---
number: 6
title: A host's identity is a key made on it, kept in its credential store, with one enrollment per account
status: accepted
date: 2026-11-01
decision-makers:
- Chuck Swanberg
---

# A host's identity is a key made on it, kept in its credential store, with one enrollment per account

Status: accepted by the user on 2026-11-07, on their card. Proposed on 2026-11-01, from the user's words in the constitution interview.

Amended by record 0028, accepted on 2026-11-07: it reads "account" where it said "partition", for what a runner enrolls, and is so written below. The file keeps its name.

Deciders: the user, 2026-11-01.
Sources: the constitution interview of 2026-11-01 (`design/research/the-constitution-interview.md`), its topic on enrollment, identity and credentials; agentplot/boomdrive-cloud's records 0018, 0023 and 0040 and its rules core.13, core.14, core.21, core.24, core.33, core.43 and core.96, cited by id; agentplot/boomdrive-cloud `design/research/awesome-lists-disposition.md`, row F10; the design of `the-runner-carries-crews-work-into-flume`, Decision 20; rule core.4.

## Context and problem statement

The cloud enrolls a host with a one-time token, issued from its console for an install command a person runs. The host's first connection raises its rail item. The runner is a principal of its own, never acting with a member's credential, and a host's signing identity is recorded in the account's chain. A host belongs to exactly one Account and has one Host object.

On the runner's side, several things were open: what the host's lasting identity is, how enrollment runs, where the identity is kept, and how one machine serves two accounts, for example one person's own work and an employer's. The unbuilt hosted implementation named a `token_file`.

## Considered options

For the identity:

1. A key pair made on the host, its private key never leaving it.
2. A long-lived bearer token in a file only the host's user may read.
3. The OS keychain.

For enrollment:

1. `boomdrive enroll`, reading the one-time token on standard input.
2. A token the installer places, exchanged at the first connection.
3. Enrollment left to whatever installs the runner.

For two accounts on one machine:

1. One runner, with one enrollment for each account.
2. One runner for each account.
3. One account for each machine.

For where the key is kept (F10): the host's own credential store with no fallback, the store with a file as fallback, or a file everywhere.

## Decision outcome

Option 1 each time, the user's choice, and F10 kept with no fallback to clear text.

- **A key made on the host.** At enrollment the runner makes a key pair, and the private key never leaves the host. Every connection proves the key is held, and the control plane certifies it for short times (core.31).
- **`boomdrive enroll`.** It reads the one-time token from standard input, never from an argument or the environment. It makes the key and registers its public key over the runner's own outgoing connection. It never writes the configuration.
- **In the host's credential store.** The key is kept only in the macOS keychain, which the runner's launchd agent opens in the user's session, and in an encrypted systemd credential on Linux. It is never in a file readable in the clear. A host with neither store is refused at `enroll`, naming what it lacks (core.32).
- **One enrollment per account.** One runner serves a machine, with one data directory and one clone for each repository. Each account its configuration names is one enrollment, with its own key, its own connection and its own Host object. What goes over one account's connection names only that account's own repositories, leases, places and commands (core.26).

### Consequences

- The hosted implementation's table names no `token_file`: the key is in the credential store, and the table names where the control plane is.
- A Linux host serving through a systemd user unit needs a systemd with user-scoped credentials, version 256 or later.
- The hello becomes one for each account, naming only that account's repositories and leases. The host's room, as Flume gives it, goes to every account.
- A copied home directory carries no usable host key.
