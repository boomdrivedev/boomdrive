---
number: 7
title: The cloud holds the GitHub App, a host holds no App key, and a repository changes only through the host's git
status: accepted
date: 2026-11-01
decision-makers:
- Chuck Swanberg
---

# The cloud holds the GitHub App, a host holds no App key, and a repository changes only through the host's git

Status: accepted by the user on 2026-11-07, on their card. Proposed on 2026-11-01, from the user's words in the constitution interview. Amended on 2026-11-09, below: one Boomdrive App for every host, its key held by the runner where no cloud holds it, and every tool's token from the runner. Amended again on 2026-11-11, below, with what the build chose within that amendment.

Deciders: the user, 2026-11-01.
Sources: the constitution interview of 2026-11-01 (agentplot/boomdrive's `design/research/the-constitution-interview.md`), its topic on enrollment, identity and credentials; sluice record 0019; agentplot/boomdrive-cloud's rules core.3, core.4, core.47 and core.58, cited by id; agentplot/boomdrive-cloud `design/research/awesome-lists-disposition.md`, row F52, and gitoxide's `crate-status.md`; the clones and crew-control-plane specs; rule core.4.

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

Status: accepted by the user on 2026-11-09, on the design agent's card: "One Boomdrive App; the runner holds its key and gives every tool its tokens". They asked it in the design agent's pane that day, as the family moved to the boomdrivedev org: "Do we want to mimic the control plane cloud concept here and have a Boomdrive GitHub app that passes the credentials through from the cloud down to Sluice?" How it is built is the design agent's own choice, cheap to change.

- **One App.** The family's forge identity is one GitHub App named for the product, owned by the `boomdrivedev` org. It is the App the cloud holds once it serves hosts, and it replaces sluice's App and the older `agentplot-flywheel`. It asks for what sluice's App asks: contents, workflows, commit statuses, checks and pull requests to write, and metadata to read.
- **Where no cloud holds it, the runner does.** On a host whose control plane is not hosted, crew's among them, the runner holds the App's key, a file named by a path in the runner's own settings, outside the Nix store, readable by its owner alone, as sluice's own key is (sluice core.51). The runner mints a short-lived installation token from it, for one repository at a time. Under a hosted control plane, no key is on the host and the cloud mints the token, as decided above.
- **Every tool asks the runner.** On a Boomdrive host, a tool that reaches the forge with the family's identity takes its token from `boomdrive forge-token`. It speaks git's credential-helper protocol, so it serves as a git credential helper set for one call, and sluice runs it the same way. It gives a token only for a repository of a drive the host serves, and only on standard output; no token is written to a file or a log. The runner's own git calls on a served repository take it the same way (core.34).
- **`erase` drops a token and revokes nothing.** The runner keeps one token for each repository and hands it to every tool that asks, until it nears its end. A tool's `erase` of the token it was given drops that token from the runner's keeping, so the next `get` for that repository mints a fresh one and never gives the erased one again. It does not revoke the token at GitHub, so another tool still holding it goes on using it until GitHub refuses it or it ends. An `erase` of a token the runner no longer holds does nothing. The command's error output never carries a token. sluice reads `erase` this way, failing a renewal that gets back the token it erased (sluice record 0108 as amended).
- **Which tools take it.** sluice takes it, to land, read the forge and post its check (sluice record 0108). The runner takes it for its clones and fetches. crew does not: it acts for the person, with the person's own credentials, on the person's cards and state, which no drive serves. An agent in a place keeps the person's credentials; how a cloud host's agent reaches the forge stays open, as above.
- **sluice alone keeps its own key.** sluice used on a host without Boomdrive names its own App and key, as it does today.
- **A cloud host's token waits on the protocol.** The runner's protocol carries no forge token through version 2. Until a version of its own does, `boomdrive forge-token` on a cloud host refuses, naming that.

### Consequences of the amendment

- Good: one key on a host, in one process, and the same path whether crew or the cloud is the control plane (blueprints principle 6).
- Good: GitHub shows the product's name on every landing, check and request.
- Bad: an agent running as the host's user can run `boomdrive forge-token` as it can read sluice's key today. The ruleset keeps agents off guarded branches because their git never asks for the token, not because they cannot (sluice record 0019).
- Bad: until the runner gives tokens, a host keeps the App's key where sluice reads it today.


## Amendment, 2026-11-11: what the build chose within the decision

Status: accepted as the construct stage's own choices within the amendment of 2026-11-09, whose build is the design agent's to choose and cheap to change. Made by the construct stage of `change/the-runner-gives-every-tool-its-forge-token`, from its brief: "get mints, or gives the one it keeps, a short-lived installation token for one repository of a drive the host serves, on standard output alone; erase drops that repository's token so the next get mints a fresh one and never the erased one, revoking nothing at GitHub; anything else is refused, and no token reaches a file, a log or its error output." The OpenSpec change `the-runner-gives-every-tool-its-forge-token` gives each point below in its design.

- **A token is kept in the process that minted it, and nowhere else.** `boomdrive forge-token` runs as a process of its own for each operation git or sluice asks of it. No command connects to the running runner (core.20), no credential is kept in a file or the data directory (core.4, core.35), and a shared thing is decided in one process, never by separate processes coordinating through storage (blueprints `design/principles.md` 6). So a `get` mints a fresh token in its own process, gives it, and keeps it no longer. Its asker keeps it, as git's protocol has a caller or a storage helper keep a credential, and as sluice keeps a token in its own memory until five minutes before its end (sluice record 0108). The running runner keeps one token for each repository for its own git calls, until five minutes before its end. An `erase` therefore drops nothing kept: it reads what git or sluice sends, and answers nothing. The next `get` mints afresh, so it never gives the erased token. Nothing is revoked at GitHub.
- **The runner's own git call is handed the token its process keeps.** A fetch of a served repository, a new clone's first among them, is given a credential helper for that one call. The call first empties the host's helpers. Its one helper reads the token from that call's environment alone, as sluice hands its own git calls their token (sluice record 0019). A fetch that fails drops the repository's kept token, and the next pass mints again.
- **A repository the App is not installed on is fetched with no credential.** GitHub's answer that the App has no installation on a served repository is kept, as a token would be. That repository's fetch goes with the host's helpers emptied and no credential, so a public repository is fetched and a private one fails with git's words. Any other failure to mint leaves that fetch with no credential too, its words after git's where the fetch fails, and the next pass asks again.
- **A mint is two calls to GitHub's API, each stopped at ten seconds.** A JWT signed with the App's key, as sluice's `app.rs` signs one, finds the App's installation on the repository and buys a token for that repository alone. Each call is stopped at ten seconds, so a `get` answers within the thirty seconds sluice gives a credential command (sluice record 0108). The API's address is a field of the minter for a test's fake, and no key, variable or flag sets it (core.17).
- **`[forge]` is the managed layer's alone.** The runner's configuration names the App by `[forge]`, its `app_id` and its `key_file`, under no control plane or crew's. Under a hosted control plane, `[forge]` is refused, because the hosted control plane holds the App. A `key_file` in the Nix store is refused. A key file anyone but its owner may read refuses the mint, as sluice refuses one.
- **`forge-token`'s answer is git's protocol.** It takes no `--json`, a departure from core.39 for this one command. git reads a helper's standard output as the protocol, so a refusal there would be read as a credential. A refusal is one line of words on standard error, never holding a token, the JWT or the key. It exits as core.39 has it: 0 answered, 1 refused, 2 for a usage error or a host to fix. An operation other than `get` and `erase`, `store` among them, is a usage error.

Not taken:

- **The host's credential store as the keeping across processes.** A keychain item made by one build of the runner asks the person before another build reads it, so a new release would block every `get` behind a dialog. The login keychain is locked to a process started over ssh. On Linux, `systemd-creds` keeps a credential as a file, and the kernel's keyring is shared only with processes that possess it, which a user's services and their shells may not.
- **A socket to the running runner.** core.3 and core.20 rule it out.
- **`boomdrive forge-token` as the runner's own helper.** Each fetch would then run a process of its own, which mints, so the runner would buy a token for each repository at every `fetch_interval`.
- **`http.extraHeader` for the runner's own calls.** It is no credential helper (core.34), and a refused header would still go on to the host's helpers unless they were emptied too.

### Consequences of the amendment

- Good: no token is held outside a process's memory, and no process but the one that asked holds one.
- Bad: every `get` costs two calls to GitHub's API. sluice asks once an hour for each repository and process, so the cost is small.
- Rule core.28 lets the runner's calls to the forge's API leave the host. Rule core.33's details name the keeping and the runner's own helper. Rule core.39's details name `forge-token`'s form.
- The set gives sluice no credential command yet. The sluice the flake holds, v0.78.1, reads none; sluice reads `[forge] credential` from v0.84.0. Moving sluice's tag, and having the Boomdrive module give sluice `boomdrive forge-token`, is a change of its own (core.45, core.57).
