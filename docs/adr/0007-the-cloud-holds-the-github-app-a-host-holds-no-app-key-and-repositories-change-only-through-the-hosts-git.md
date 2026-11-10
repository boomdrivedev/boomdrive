---
number: 7
title: The cloud holds the GitHub App, a host holds no App key, and a repository changes only through the host's git
status: accepted
date: 2026-11-01
decision-makers:
- Chuck Swanberg
---

# The cloud holds the GitHub App, a host holds no App key, and a repository changes only through the host's git

Status: accepted by the user on 2026-11-07, on their card. Proposed on 2026-11-01, from the user's words in the constitution interview. Amended on 2026-11-09, below: one Boomdrive App for every host, its key held by the runner where no cloud holds it, and every tool's token from the runner.

Deciders: the user, 2026-11-01.
Sources: the constitution interview of 2026-11-01 (agentplot/boomdrive-archive's `design/research/the-constitution-interview.md`), its topic on enrollment, identity and credentials; sluice record 0019; agentplot/boomdrive-cloud's rules core.3, core.4, core.47 and core.58, cited by id; agentplot/boomdrive-cloud `design/research/awesome-lists-disposition.md`, row F52, and gitoxide's `crate-status.md`; the clones and crew-control-plane specs; rule core.4.

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

- crew's hosts, which have no cloud, hold the Boomdrive App's key in the runner, as the amendment of 2026-11-09 has it.
- sluice is asked for an identity that runs a command for its token, as git runs a credential helper, naming no control plane (cloud core.47).
- How an agent in a place on a cloud host reaches the forge, for a branch it pushes or a `gh` call, is still open. It is Flume's and the cloud's question, and the runner's only if a token comes through it.
- As built, the runner and weir-branch run `git` for every call, so F52 holds already.

## Amendment, 2026-11-09: one Boomdrive App, its key held by the runner where no cloud holds it

Status: accepted by the user on 2026-11-09, on agentplot-design's card: "One Boomdrive App; the runner holds its key and gives every tool its tokens". They asked it in agentplot-design's pane that day, as the family moved to the boomdrivedev org: "Do we want to mimic the control plane cloud concept here and have a Boomdrive GitHub app that passes the credentials through from the cloud down to Sluice?" How it is built is agentplot-design's own choice, cheap to change.

- **One App.** The family's forge identity is one GitHub App named for the product, owned by the `boomdrivedev` org. It is the App the cloud holds once it serves hosts, and it replaces sluice's App and the older `agentplot-flywheel`. It asks for what sluice's App asks: contents, workflows, commit statuses, checks and pull requests to write, and metadata to read.
- **Where no cloud holds it, the runner does.** On a host whose control plane is not hosted, crew's among them, the runner holds the App's key, a file named by a path in the runner's own settings, outside the Nix store, readable by its owner alone, as sluice's own key is (sluice core.51). The runner mints a short-lived installation token from it, for one repository at a time. Under a hosted control plane, no key is on the host and the cloud mints the token, as decided above.
- **Every tool asks the runner.** On a Boomdrive host, a tool that reaches the forge with the family's identity takes its token from `boomdrive forge-token`. It speaks git's credential-helper protocol, so it serves as a git credential helper set for one call, and sluice runs it the same way. It gives a token only for a repository of a drive the host serves, and only on standard output; no token is written to a file or a log. The runner's own git calls on a served repository take it the same way (core.34).
- **Which tools take it.** sluice takes it, to land, read the forge and post its check (sluice record 0108). The runner takes it for its clones and fetches. crew does not: it acts for the person, with the person's own credentials, on the person's cards and state, which no drive serves. An agent in a place keeps the person's credentials; how a cloud host's agent reaches the forge stays open, as above.
- **sluice alone keeps its own key.** sluice used on a host without Boomdrive names its own App and key, as it does today.
- **A cloud host's token waits on the protocol.** The runner's protocol carries no forge token through version 2. Until a version of its own does, `boomdrive forge-token` on a cloud host refuses, naming that.

### Consequences of the amendment

- Good: one key on a host, in one process, and the same path whether crew or the cloud is the control plane (blueprints principle 6).
- Good: GitHub shows the product's name on every landing, check and request.
- Bad: an agent running as the host's user can run `boomdrive forge-token` as it can read sluice's key today. The ruleset keeps agents off guarded branches because their git never asks for the token, not because they cannot (sluice record 0019).
- Bad: until the runner gives tokens, a host keeps the App's key where sluice reads it today.

