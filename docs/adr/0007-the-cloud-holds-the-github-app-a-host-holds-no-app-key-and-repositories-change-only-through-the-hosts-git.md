---
number: 7
title: The cloud holds the GitHub App, a host holds no App key, and a repository changes only through the host's git
status: proposed
date: 2026-11-01
decision-makers:
- Chuck Swanberg
---

# The cloud holds the GitHub App, a host holds no App key, and a repository changes only through the host's git

Status: proposed on 2026-11-01, from the user's words in the constitution interview; accepted on the user's yes once they have read it as a record.

Deciders: the user, 2026-11-01.
Sources: the constitution interview of 2026-11-01 (`design/research/the-constitution-interview.md`), its topic on enrollment, identity and credentials; sluice record 0019; agentplot/boomdrive-cloud's rules core.3, core.4, core.47 and core.58, cited by id; agentplot/boomdrive-cloud `design/research/awesome-lists-disposition.md`, row F52, and gitoxide's `crate-status.md`; the clones and crew-control-plane specs; rule core.4.

## Context and problem statement

Every fetch and push the runner makes goes through the host's own git credential helper. sluice lands as a GitHub App: the host's `[forge]` names the App's id and its key file, and sluice mints installation tokens that it hands only to its own git calls, so agents running as the same user cannot push to the branches sluice guards (sluice record 0019).

For a hosted product on many people's hosts, the question was which identity a host's forge calls use. One option was an App key of the runner's own, built as sluice's is. The user asked: "Do we really want to be distributing GitHub app keys and IDs to everybody's hosts in a sort of self-hosted setup? [...] maybe the cloud should have the app and send short tokens down."

## Considered options

1. An App of the runner's own on each host, as sluice has.
2. sluice's App, with tokens narrowed to the least permission.
3. The host's own helper, with every guarded write a tool's.
4. The cloud holds the App and sends short-lived tokens down the runner's socket.

Mitigations of the cloud's reach under option 4: tokens narrowed for each call and kept in memory; every mint an event in the account's chain; the cloud never using a token itself; the customer picking the repositories the App is installed on.

For sluice's own identity on a cloud host: a credential command, a token in each call's environment, or sluice keeping its own App.

## Decision outcome

Option 4, the user's choice, with two of the mitigations.

- **No App key on a host.** Under a hosted control plane, no GitHub App key or App id is on a host for the runner or a tool to mint with. The cloud holds the App, and a host's forge calls are made with short-lived tokens the cloud mints and sends down the runner's own connection (core.33).
- **The cloud never uses a token, and the customer picks the repositories.** These are the user's mitigations, and both are the cloud's to hold: no cloud code makes a forge call with a token it minted, and the App is installed only on the repositories the customer chooses.
- **A token only for a served repository.** The cloud mints a token for a host only for a repository of a drive the host serves (record 0009). The user said a mapping of repositories to hosts is superseded by that: the customer controls which hosts work with which repositories by which drives each host serves.
- **A tool asks the runner.** A tool on a cloud host that needs a forge identity of its own, sluice to land among them, gets its token from a credential command, `boomdrive forge-token`. That command reaches the cloud over its own outgoing connection with the host's key, and gives the token on standard output only.
- **Writes through the host's git**, F52 kept. The runner changes a repository only through the host's own `git` command, by a clone, a fetch, a worktree, a moved ref or a push, so the host's helpers and configuration apply, a helper set for one call among them. A library may only read (core.34).

Not taken: tokens narrowed for each call, and every mint as an event on the chain. core.4 still rules out a token written to a file or a log.

### Consequences

- crew's hosts, which have no cloud, keep sluice's App and the host's own credential helper as they are.
- sluice is asked for an identity that runs a command for its token, as git runs a credential helper, naming no control plane (cloud core.47).
- How an agent in a place on a cloud host reaches the forge, for a branch it pushes or a `gh` call, is still open. It is Flume's and the cloud's question, and the runner's only if a token comes through it.
- As built, the runner and weir-branch run `git` for every call, so F52 holds already.
