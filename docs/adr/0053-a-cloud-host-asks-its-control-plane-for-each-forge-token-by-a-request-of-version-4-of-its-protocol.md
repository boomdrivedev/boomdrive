---
number: 53
title: A cloud host asks its control plane for each forge token by a request of version 4 of its protocol
status: accepted
date: 2026-11-18
decision-makers:
- the construct stage of change/a-cloud-host-gets-its-github-tokens-from-the-cloud
---

# A cloud host asks its control plane for each forge token by a request of version 4 of its protocol

Status: accepted. The direction is the user's, in record 0007: under a hosted control plane the cloud holds the Boomdrive App and mints a host's tokens, and a tool takes its token from `boomdrive forge-token`, "which reaches the cloud over its own outgoing connection with the host's key, and gives the token on standard output only". The change's brief, 2026-11-18: "On a host that Boomdrive Cloud serves, sluice and the runner get their GitHub tokens from the cloud for the repositories of the drives the host serves, just as they get them from the runner's own key on a host without the cloud." How the protocol carries it is the construct stage's own choice, made on this repository's rules. Each point is cheap to change until a release of the runner speaks version 4. From then on version 4 never changes (core.42).

Deciders: the user, for the direction (record 0007 and its amendment of 2026-11-09); the change's brief; the construct stage of the change `a-cloud-host-gets-its-github-tokens-from-the-cloud`, for the rest.

Sources: record 0007 and its amendments of 2026-11-09, 2026-11-11 and 2026-11-18; records 0015, 0018, 0037, 0048 and 0052; rules core.2, core.3, core.4, core.20, core.26, core.28, core.31 to core.33, core.35, core.39 and core.42; sluice record 0108, which gives a credential command thirty seconds; boomdrivedev/boomdrive-cloud records 0056 and 0073, whose point 2 says the cloud mints a host's token only from the installation of the host's own Account, for one repository registered to a drive of that Account the host serves.

## Context and problem statement

On a host without a cloud, the runner mints a served repository's token from the App's key its `[forge]` names, gives it to every tool by `boomdrive forge-token`, and keeps one for each repository for its own fetches (record 0007 as amended on 2026-11-11). On a cloud host no key is there, and no version of the protocol carries a token, so `boomdrive forge-token` refuses every operation, naming version 3, and the runner's own fetch goes with the host's helpers. sluice on a cloud host has no token to land with.

`boomdrive forge-token` runs as a process of its own for each operation git or sluice asks of it. No command connects to the running runner (core.20), and no token is kept in a file or the data directory (core.4, core.35). So the process that asks must reach the control plane itself. A partition's socket is the running runner's alone: its first message is the whole hello with the host's room and leases, which a process run for one `get` does not hold, and the control plane keeps one socket for each partition.

## Considered options

For how a process asks:

1. **One HTTPS request of its own, `POST runner/forge-token`, proven by the partition's key, as enrollment is one request (chosen).**
2. A second WebSocket of each partition in a role of its own, opened for one ask and closed.
3. The running runner asks on its socket, and hands the token to `boomdrive forge-token` through the data directory or a socket of its own.

For the running runner's own fetches:

- a. **The same request, its answer kept as a minted token is kept (chosen).**
- b. A request on the partition's socket, answered on it.

For which partition asks:

- i. **Each partition whose `repositories` hold the repository, in the table's order, until one gives a token (chosen).**
- ii. The first such partition alone.

For the version:

- **Version 4, version 3 whole but for the ask, its two answers and the path they go by (chosen).**
- Version 3 changed to carry them. Releases from `v0.18.0` spoke version 3, so core.42 rules it out.

## Decision outcome

Options 1, a and i, in version 4.

- **Version 4 is version 3 whole, at `crates/boomdrive-hosted/protocol/v4/`, but for these:**
  - every file's `$id` is under `…/protocol/v4/`, every message's `protocol_version` is 4, the subprotocol is `boomdrive.v4`, and every context a key signs starts `boomdrive.v4`;
  - a twelfth file, `forge-token.schema.json`, holds three kinds: the runner's ask, `forge_token`, `{"repository":…}`; the control plane's `forge_token_given`, `{"forge_token":<the ask's id>,"repository","token","expires_at"}`; and its `forge_token_refused`, `{"forge_token":<the ask's id>,"repository","not_installed","words"}`, where `not_installed` is true alone where the App's installation on the host's own Account does not cover the repository;
  - `from-runner.schema.json`'s body takes `forge_token`, and `to-runner.schema.json`'s takes the two answers.

  Its examples are version 3's, each naming `protocol_version` 4 and `enroll.json` signed over version 4's context, and four more: an ask, a token given, and two refusals, one of an App not installed and one of a repository no drive of the host serves.
- **The ask is one request at a path of its own.** `POST <url>/runner/forge-token`, its body a `forge_token` of version 4, its `Authorization` header `BoomdriveHost <key>.<time>.<nonce>.<signature>`, as an upgrade's is, but signed over a context of its own, `forge token`, whose fields are the control plane's origin, the key's 32 bytes, the time's digits, the nonce's 16 bytes and the repository's address. So a statement made for one repository is never taken for another, nor for a connection, nor a connection's for an ask. The control plane takes the ask as it takes an upgrade's statement, and answers `forge_token_given`, `forge_token_refused` or `message_refused`. The runner reads the body whatever the status, checked against version 4. An answer that is no message of version 4, or none within twenty seconds, refuses the ask, naming the path and the status. Twenty seconds leaves sluice's thirty for git, and the control plane two calls to GitHub of ten seconds each, as the runner's own mint has.
- **A token is named once, in the one place it goes.** The runner-protocol spec forbids a field named `token`, `credential` or the like, so that a connection's grant comes from its enrollment alone. `forge_token_given`'s `token` is the one field excepted, by name and path: it carries the credential the user decided the cloud sends down (record 0007), and naming it otherwise would hide it from the check.
- **The interface names a control plane's forge tokens.** `boomdrive-control` gains `ForgeTokens`, what gives a served repository's token where the control plane holds the App, and its answer: a token and its end, whose `Debug` names its end alone; the App's absence, with words; or a refusal, with words and whether the host must be fixed. Its fake gives a test's answers. The hosted implementation's `ForgeTokens` is opened from its table and the host's store, and opens no part and no socket. `planes::Entry`'s `holds_the_app` becomes the function that opens it, still refusing `[forge]` beside it. Nothing outside the hosted crate names the path, the kinds or the context (core.2).
- **Partitions are tried in the table's order.** The hosted implementation asks each partition whose `repositories` hold the repository, under that partition's key read from the host's store, until one gives a token. A host may serve one repository in two accounts, and only the Account whose installation covers it gives a token (boomdrive-cloud record 0073, point 2). Where none gives one, the answer is the App's absence where every partition said so, and otherwise a refusal naming each partition's words. A repository no partition is served refuses with no request made. A host with no store, or with no partition's key in it, must be fixed, naming `boomdrive enroll <label>`. The asks together stop at twenty seconds.
- **`boomdrive forge-token` on a cloud host answers as on a host that holds the key.** It reads git's request and checks it as it does there, then asks the control plane's `ForgeTokens` in place of minting. It gives the same three lines, `password_expiry_utc` from `expires_at`, keeps nothing and writes nothing. `erase` reads what it is sent and answers nothing, as on any host: nothing is kept, and the next `get` asks again. A refusal is one line, `boomdrive forge-token: <words>`, its words the hosted implementation's with the control plane's own as it gave them.
- **The running runner keeps what it is given as it keeps what it mints.** Where the configuration's control plane holds the App, the runner opens its `ForgeTokens` at start, and its fetch of a served repository is given the token its `Forge` asks for and keeps. It keeps one token for each repository until five minutes before its end, keeps the App's absence for fifty-five minutes, keeps a refusal not at all, and drops a repository's token when its fetch fails. A fetch with no token goes with no credential, as on a host that holds the key.
- **The runner speaks version 4 alone**, as records 0037 and 0052 decided for versions 2 and 3: its upgrade asks for `boomdrive.v4`, its `enroll` is a message of version 4, and its message types move to version 4's module. A key enrolled under an earlier version stays the partition's. Version 3 stays as `v0.18.0` spoke it, held to its digests, as version 2 is.

Not taken:

- **Option 2.** The control plane keeps one socket for each partition, which a second socket of the same key would contend with, and a socket's first message is the whole hello, which a process run for one `get` does not hold.
- **Option 3.** core.20 and core.35, and record 0007's own keeping across processes, which its amendment of 2026-11-11 did not take.
- **b.** Two ways to one token, one per caller, where blueprints principle 6 asks one.
- **ii.** A repository two accounts register would give a token only where the first account's installation covers it.

### Consequences

- Good: sluice and the runner on a cloud host take a served repository's token from the cloud, through the same command and the same keeping as on a host that holds the key, and no App key is on the host.
- Good: the ask is proven by the partition's own key, signed for that one repository and that one attempt, and nothing of it is kept.
- Bad: a control plane that speaks version 3 alone cannot be reached by a release that carries this record until it takes version 4. boomdrive-cloud pins version 2 today, so it takes version 4 and builds `runner/forge-token` under its record 0073, point 2, by a change of its own.
- Bad: each `get` on a cloud host reads the partition's key from the host's store. On macOS a binary the keychain item does not trust asks the person first, so the command sluice is given must be the copy core.60 signs.
- Bad: the tree holds a fourth copy of every schema file.
- Neutral: crew's implementation, and a host that holds the App's key, do not change.
- This record amends the rule and details of core.33, and the details of core.42. Record 0007's amendment of 2026-11-09 left a cloud host's token waiting on the protocol; its amendment of 2026-11-18 names this record as the version that carries one.
