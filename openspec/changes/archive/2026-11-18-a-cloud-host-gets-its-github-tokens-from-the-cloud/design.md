# Design

## Context

See proposal.md for why. The decision is record 0053, with record 0007's amendment of 2026-11-18, core.33's rule and details and core.42's details, each written with this change. No area page binds the paths this change touches, so `docs/architecture/core.md` binds every one. The change relies on these rules:

- core.1: the control plane's refusal is carried as it gave it;
- core.2: only the hosted implementation names the protocol, its path, its kinds and its contexts; the runner reaches a control plane's tokens through the interface;
- core.3: the ask is an outgoing request, and nothing listens;
- core.4, core.35: no token is in the tree, a file, the data directory, a log or error output, and none is kept past its process but the running runner's in memory;
- core.11: no line or refusal names a value of a message but the control plane's own words;
- core.20: no command connects to the running runner, so `boomdrive forge-token` asks the control plane itself;
- core.26: an ask goes under the key of a partition served the repository;
- core.31, core.32: every ask proves a partition's key, read from the host's store alone;
- core.33: under a hosted control plane the host's tokens are the control plane's, each for a served repository;
- core.39: `forge-token`'s exits, and its answer as git's protocol;
- core.42: version 4 is a new directory beside versions 1 to 3, which never change, and each end checks what it is sent against its version.

It departs from no rule.

### What stands

- **Version 3**, `crates/boomdrive-hosted/protocol/v3/`, holds eleven schema files and its examples. Every release from `v0.18.0` to `v0.19.1` speaks it, and each holds the same files as `main`. Its `$id`s are under `https://raw.githubusercontent.com/boomdrivedev/boomdrive/main/crates/boomdrive-hosted/protocol/v3/`. `src/protocol/v3.rs` holds `VERSION` 3, `SUBPROTOCOL` `boomdrive.v3`, `ADDRESS`, `FILES` and every message type; `v1.rs` and `v2.rs` hold their four constants alone. `check.rs`, `copy.rs`, `connection.rs`, `enrollment.rs`, `key.rs`, `observer.rs`, `plane.rs` and `lib.rs` use `protocol::v3`, and the words of a failed check are made from `VERSION`.
- **Enrollment** is one `POST runner/enroll` by a `ureq` agent that treats no status as an error, follows no redirect, stops at a global timeout and verifies the platform's roots. It checks the answer against `to-runner.schema.json` whatever its status, then reads it. `Table` gives `enroll_address` and `connect_address` under the `url`.
- **A connection's statement** is `key::statement`: `BoomdriveHost <key>.<time>.<nonce>.<signature>` over `key::connection_context`, the context `connect` of the origin, the key's bytes, the time's digits and the nonce. `key::context` starts every context with `SUBPROTOCOL`. `Store::read` gives a partition's key by its `KeyName`, none where none is kept; `key::store_for_host` opens the keychain on macOS and `systemd-creds` on Linux, or says why it cannot.
- **`lib.rs`'s `NO_FORGE_TOKEN`** is the words a cloud host's `forge-token` refuses with. `planes::Entry`'s `holds_the_app: Option<&'static str>` holds it for `hosted` and none for `crew`. `forge_token::answer` refuses every operation with it where the configuration's control plane holds the App, and `config.rs` refuses `[forge]` beside such a control plane.
- **`forge.rs`'s `Forge`** mints from the App's key: `mint(app, address)` gives `Minted::Token(Token)` or `Minted::NotInstalled`, or `Refused`. `credential(app, address)` gives the running runner's fetch its `Asked`, keeping one `Answer` for each repository: the token until `RENEW_BEFORE` its end, the App's absence for `NOT_INSTALLED_FOR`, a refusal not at all, and every answer emptied for another App. `drop(address)` drops one after a failed fetch. `run.rs` holds one `Forge::new()` for the runner's life, and `pass.rs` asks it with `config.forge` before each fetch.
- **The interface**, `boomdrive-control`, names a pull, a report and `owns`, served as parts. It depends on `serde` and `time`. Its `fake` gives a fake control plane.
- **The tests' `common` module** of the hosted crate has a `Version` for each version, `VERSIONS` `[V1, V2, V3]`, and `CURRENT` `V3`. Its `fake` serves `runner/enroll` and `runner/connect` under `boomdrive.v3`. `tests/files.rs` holds versions 1 and 2 to their releases' digests, and version 3 to version 2 but for its hold and observing connection. `tests/names.rs` finds no name of any version holding a grant's or a credential's word, and catches `forge_token` as one.
- **The runner's tests** of `forge-token` and of a fetch's token run on a fake GitHub (`crates/boomdrive/tests/github`), and `Scratch::pass_with_forge` makes a keeping pass with a given `Forge`. `tests/enroll.rs` runs the binary under a scratch host whose `HOME` holds no keychain and a stub `systemd-creds` first on its `PATH`, so no test reaches a real store.
- **boomdrive-cloud** pins versions 1 and 2 under `workers/control-plane/runner-protocol/`, and has not taken version 3. Its record 0073, point 2, says the cloud mints a host's token only from the installation of the host's own Account, naming one repository by its id, registered to a drive of that Account the host serves.

## Goals / Non-Goals

**Goals:**

- On a cloud host, sluice and the runner take a served repository's token from the control plane by `boomdrive forge-token` and the running runner's fetch, as they take one minted from the App's key elsewhere.
- One path to a token for both callers, and one keeping, whatever gives the token.
- Versions 1 to 3 stay exactly as releases spoke them, and a reader sees exactly how version 4 differs from version 3.

**Non-Goals:**

- **The cloud's side.** boomdrive-cloud takes version 4 and serves `runner/forge-token` by a change of its own.
- **sluice's credential command on a host.** The Boomdrive module gives sluice none yet (record 0007 as amended on 2026-11-11), and giving it one is a change of its own.
- **An agent in a place.** It keeps the person's own credentials (record 0007).
- **crew's implementation, and a host that holds the App's key.** Neither changes.
- **Anything else in version 4.** Health, the runner's own commands and a terminal's certificate each come in a version of their own.

## Decisions

### 1. Version 4 is version 3 whole, but for a forge token's ask and answers

`crates/boomdrive-hosted/protocol/v4/` holds version 3's eleven schema files and its examples, and differs from version 3 by these alone:

- every file's `$id` is under `https://raw.githubusercontent.com/boomdrivedev/boomdrive/main/crates/boomdrive-hosted/protocol/v4/`, every envelope's and example's `protocol_version` is 4, and each description naming version 3 names version 4;
- a twelfth file, `forge-token.schema.json`, whose `$defs` are three closed objects, every field required:
  - `forge_token`, the runner's ask: `repository`, `shared.schema.json`'s `repository`;
  - `forge_token_given`: `forge_token`, the ask's id as `shared.schema.json`'s `message_id`; `repository`; `token`, a string of one character or more, bounded no further, since GitHub's form of a token is its own; and `expires_at`, `shared.schema.json`'s `time`;
  - `forge_token_refused`: `forge_token`; `repository`; `not_installed`, a boolean, true alone where the App's installation on the host's Account does not cover the repository; and `words`, a string;
- `from-runner.schema.json`'s `body` gains a branch `{"forge_token":…}`, and `to-runner.schema.json`'s gains `{"forge_token_given":…}` and `{"forge_token_refused":…}`, each referring to `forge-token.schema.json`.

Its examples are version 3's, each naming `protocol_version` 4, `enroll.json` signed over version 4's context, and four more: `from-runner/forge_token.json`, `to-runner/forge_token_given.json`, `to-runner/forge_token_refused-not-installed.json` and `to-runner/forge_token_refused-a-repository-no-drive-serves.json`. The example's token is `ghs_example`, no token of anyone's (core.4).

`not_installed` is a boolean, not a reason among several, because the runner reads one thing of a refusal: whether to keep it as the App's absence for fifty-five minutes, as it keeps GitHub's 404. Every other refusal is carried by its words (core.1). Not taken: a closed list of reasons, each of which would need a new version to add.

### 2. The ask goes by one request at a path of its own

The ask is `POST <url>/runner/forge-token`, with `Content-Type: application/json`, its body a message of version 4 whose body is `forge_token`, its `Authorization` header `BoomdriveHost <key>.<time>.<nonce>.<signature>`. The signature is over the context `forge token`, whose fields are the control plane's origin, the key's 32 bytes, the time's digits, the nonce's 16 bytes and the repository's address as its UTF-8 bytes. The control plane takes the statement as it takes an upgrade's (the protocol's README, *The connection's proof*): a key it enrolled and has not revoked, the signature, the time within 300 seconds, and a nonce not seen with that key within that window. Since the context's name and its last field differ from a connection's, a statement made for one repository is never taken for another, nor an ask's for a connection's.

The control plane answers, by its README's table:

- `200` with `forge_token_given`;
- `401` or `403` with `forge_token_refused`, where it does not take the key or the statement, or the repository is not one the host's Account registers to a drive the host serves, or its installation does not cover it;
- `400` with `message_refused`, where the ask fails its check;
- `502` with `forge_token_refused`, where GitHub fails it.

The runner reads the body whatever the status, as enrollment does, and takes only `forge_token_given` or `forge_token_refused` naming its ask's id and repository. `Table` gains `forge_token_address`. `key` gains `forge_token_context` and `forge_token_statement`, made as `connection_context` and `statement` are.

No forge token's kind goes on a partition's socket, as no enrollment's does: the runner answers a `forge_token_given` or `forge_token_refused` received there with `message_refused`, its words naming `runner/forge-token`, and a control plane refuses a `forge_token` received there. The schema checks a message's form, not the path it came by.

Not taken: a second socket for each ask, and the ask on a partition's socket (record 0053, *Not taken*).

### 3. One field names a credential, and the check names it

The runner-protocol spec's check finds a name holding `grant`, `role`, `permission`, `token`, `credential`, `secret` or `password` in no version, so a connection's account comes from its enrollment alone. Version 4's forge token is a credential by design (record 0007). `tests/names.rs` excepts, in version 4 alone, the names `forge_token`, `forge_token_given`, `forge_token_refused` and `token`, and asserts each is found in version 4 and in no other version. Every other name of every version is held to the check as before.

Not taken: naming the token's field so the check misses it, which would hide the one credential the protocol carries.

### 4. The runner speaks version 4 alone

As records 0037 and 0052 decided for versions 2 and 3, the runner speaks the newest version alone:

- the message types move from `src/protocol/v3.rs` to a new `src/protocol/v4.rs`, with `VERSION` 4, `SUBPROTOCOL` `boomdrive.v4`, `ADDRESS` and `FILES`, twelve files; `protocol::v3` keeps its four constants alone, as `v1` and `v2` do;
- `ToRunner` gains `ForgeTokenGiven` and `ForgeTokenRefused`, and `FromRunner` gains `ForgeToken`. `ForgeTokenGiven`'s `Debug` names its ask, repository and `expires_at`, and never its token (core.4);
- `check.rs`, `copy.rs`, `connection.rs`, `enrollment.rs`, `key.rs`, `observer.rs`, `plane.rs` and `lib.rs` use version 4; `connection.rs` answers a forge token's answer as it answers an enrollment's (Decision 2);
- every word naming the version is made from `VERSION`, and the crate's documentation and `Cargo.toml`'s comment name version 4;
- `tests/worked-bytes.json` holds version 4's worked contexts and signatures and the ask's, and version 4's `examples/from-runner/enroll.json` the bytes the crate makes;
- the tests' `common` gains `V4`, `VERSIONS` holds four, `CURRENT` is `V4`, and `common::fake` speaks `boomdrive.v4` and serves `runner/forge-token` (Decision 7).

A key enrolled under version 3 stays the partition's and is proved under version 4. A control plane that speaks version 3 alone refuses the upgrade, which the runner says once and tries again at the backoff's cap (core.22, core.23).

### 5. Tests hold version 4 to version 3, and version 3 to its release

`tests/files.rs` gains `version_4_is_version_3_but_for_a_forge_token`. Each schema file of version 4 but `forge-token.schema.json` is read with every `description` set aside, its `$id` read under version 3's address, `protocol_version` read as 3, and the forge token's branches taken out of the two bodies, which must be exactly the three kinds. It is then equal to version 3's. Each example of version 3 has an example of version 4 by the same name, equal to it but for `protocol_version` and, in `enroll.json`, its signature, and version 4's examples are those and the four of Decision 1. A failure names the file and the first differing JSON pointer, and no value.

`version_3_is_as_released` holds every file under `protocol/v3/` to its SHA-256 at `v0.18.0`, by `held_as_released`, as versions 1 and 2 are held. `tests/examples.rs` gains `version_3s_examples_still_pass`, and `tests/forms.rs` the forge token's forms and their refusals. `no_forge_token_in_version_1_2_or_3` checks version 4's forge token examples against each earlier version.

### 6. The interface names a control plane's forge tokens

`boomdrive-control` gains, beside `ControlPlane`:

- `ForgeTokens`, a trait, `Send + Sync`, with one call: the token of a repository, by its address, `owner/name`;
- its answer: a `ForgeToken`, the token and its end as `time`'s `OffsetDateTime`, whose `Debug` names its end alone; the App's absence, with words; or a refusal, with words and whether the host must be fixed (core.39's exit 2);
- in `fake`, `Tokens`: a fake `ForgeTokens` a test sets each repository's answer on, which counts each repository's asks.

It is no part and opens nothing: it is what an implementation whose control plane holds the App gives for that App's tokens (core.2, core.33). Nothing in the interface names a path, a kind or a version.

`planes::Entry`'s `holds_the_app` becomes `Option<TokensOpener>`, `fn(table: &toml::Table, file: &Path) -> Result<Box<dyn ForgeTokens>, String>`: `hosted`'s opens the hosted implementation's (Decision 7), and `crew`'s is none. `config.rs` still refuses `[forge]` where it is some. `planes` gains `forge_tokens(config)`, which opens the configuration's control plane's tokens where its entry holds the App, from its `implementation_table` and the configuration's file, and is none elsewhere. `NO_FORGE_TOKEN` goes.

Not taken: a call on `ControlPlane`, which an implementation serves only as parts the running runner opens, and which `boomdrive forge-token`, a process of its own for one `get`, does not open (core.20).

### 7. The hosted implementation's forge tokens

A new `src/forge_token.rs` gives `Tokens`, the hosted implementation's `ForgeTokens`, and `lib.rs`'s `forge_tokens(table, file)` opens it from the hosted table and the host's store. `Tokens` holds the table, the store or why `key::store_for_host` could not open it, and its limit, `ASK_FOR`, twenty seconds; a test makes one over `key::Memory`, a fake's `url` and a shorter limit. It opens no socket and no part, and says no line. An ask:

1. finds each partition whose `repositories` hold the repository, in the table's order; none refuses at once, naming the repository as one no partition of the hosted control plane is served;
2. for each, reads its key from the store; a store that could not be opened, or a partition with no key, is passed over with words naming `boomdrive enroll <label>`;
3. signs a statement for that partition, that repository and that attempt (Decision 2), writes a `forge_token` of version 4 under a fresh message id, and checks it against `from-runner.schema.json` before sending it;
4. sends it by a `ureq` agent made as enrollment's is, its global timeout what remains of `ASK_FOR` since the first ask began, and reads the answer as enrollment reads one, checked against `to-runner.schema.json` whatever its status;
5. gives the first `forge_token_given` naming the ask and the repository as the token and its end, and asks no further partition.

Where no partition gives a token, the answer is the App's absence where every partition asked answered `forge_token_refused` with `not_installed` true, and a refusal otherwise. Where no partition could be asked for want of a key or a store, the refusal is the host's to fix. Each partition's words are joined in the table's order, each naming its label, the control plane's words as it gave them, and no token, statement or key. An answer that is no message of version 4, fails its check, or names another ask or repository refuses that partition's ask, naming `runner/forge-token` and the status, and the schema's path where its check failed.

The fake control plane, `tests/common/fake.rs`, serves `POST runner/forge-token`: it checks the statement over the `forge token` context as it checks an upgrade's, records each ask with its partition, nonce and body, and answers what a test set for that partition and repository, a status with a body, or none.

### 8. A `Forge` gives a token from the App's key or the control plane

`Forge` gains a source. `Forge::new()` and `Forge::at(api)` mint from the App's key as now. `Forge::asking(tokens)` asks the given `ForgeTokens` in place of minting, whatever `[forge]` names:

- `fresh(app, address)` gives what `boomdrive forge-token get` answers with: a token minted, or asked, afresh and never one kept. For the App's key it is `mint`, and where the forge asks, it is the control plane's answer. `Minted::NotInstalled` carries words, the implementation's where it asked;
- `credential(app, address)` keeps what it is given as it keeps what it mints: a token until `RENEW_BEFORE` its end, the App's absence for `NOT_INSTALLED_FOR`, and a refusal not at all, its words given with no credential. `drop` drops one after a failed fetch. Where it asks, no App is held and `app` is not read.

`run.rs` makes its `Forge` at start: `Forge::asking` with `planes::forge_tokens(config)` where the configuration's control plane holds the App, and `Forge::new()` elsewhere. `pass.rs` is unchanged: it asks the forge with `config.forge`, and a fetch takes the credential the forge gives through its one helper, or none (core.34). Opening the tokens reads only the hosted table the configuration already passed, so it fails nowhere the configuration was read; were it to fail, the runner's `Forge` gives every fetch no credential, with the open's words as each refused ask's.

### 9. `boomdrive forge-token` on a cloud host

`forge_token::main` reads the configuration as now, and makes its `Forge` as `run.rs` does. An open that fails is a host to fix, exit 2. `answer` takes that `Forge`:

- it requires `[forge]` only where the forge mints from the App's key; where it asks, `[forge]` is not read, since `config.rs` refuses it beside a control plane that holds the App;
- it reads git's request, answers `erase` with nothing, and checks the protocol, the host, the path and the repository's address, and that a drive of the host serves the repository, as now, before any mint or ask;
- it gives `fresh`'s token in the same three lines, `password_expiry_utc` its end's Unix seconds;
- the App's absence refuses with exit 1, saying to install the app on the repository, with the implementation's words where it asked. A refusal is exit 1, or 2 where the host must be fixed, its words the implementation's as given.

Every refusal stays one line on standard error, `boomdrive forge-token: <words>`, and nothing is written to a file, the data directory or a log (core.4, core.35, core.39).

### 10. What the runner says of it

- `crates/boomdrive-hosted/protocol/README.md` describes version 4: its envelope, `boomdrive.v4`, its three paths, the forge token's kinds and statuses in the messages and paths tables, a paragraph on a forge token, the context `forge token` under *The signatures* with its worked bytes beside the enrollment's and the connection's, its list of what version 4 has no message for (health, the runner's own commands, a terminal's certificate), *Versions* saying the runner speaks version 4 alone, a section **Version 3** naming the releases from `v0.18.0` that spoke it and how it differs, and **Version 2** and **Version 1** as they stand.
- README's hosted section names version 4, `boomdrive.v4`, under `crates/boomdrive-hosted/protocol/v4/`. Its forge-token section says a cloud host's `get` asks the hosted control plane for the token, under the key of a partition served the repository, and a cloud host's fetch does the same, and no longer says no version carries a token.
- `boomdrive-control`'s documentation of each act and of a preview says versions 3 and 4 carry it as the interface writes it.

## Risks / Trade-offs

- [A release with this change cannot connect to a control plane that speaks version 3 or earlier alone] → boomdrive-cloud pins version 2 and takes version 4 by a change of its own, as it takes each version. The runner says the refused subprotocol once and tries again at the backoff's cap, and a host keeps its repositories meanwhile.
- [A cloud host's ask before the cloud serves `runner/forge-token`] → The ask is refused, naming the path and the status. `get` exits 1, and the runner's fetch goes with no credential, so a public repository is still fetched and a private one fails with git's words, then the ask's.
- [On macOS, each `get` reads a partition's key from the login keychain, which asks the person first where the binary is not one the item trusts, and is locked to a process started over ssh] → The change that gives sluice its credential command names, on nix-darwin, the copy of the runner core.60 signs, whose grant holds for every later build. A `get` that waits on the keychain past sluice's thirty seconds fails that renewal, and sluice asks again.
- [Two partitions served one repository each take up to twenty seconds] → The asks together stop at `ASK_FOR`, so a `get` answers within sluice's thirty seconds.
- [A token in a message] → No line or refusal names a value of a message, the token's type's `Debug` hides it, and the token goes nowhere but `get`'s standard output and the runner's own call's environment (core.4, core.11).
- [Every `get` costs one request and the cloud two calls to GitHub] → sluice asks once an hour for each repository and process, as on a host that holds the key.
- [The tree holds a fourth copy of every schema file] → The test of Decision 5 shows exactly what version 4 adds.

## Migration Plan

Nothing on a host moves by hand: no key of the runner's file or field of its report changes (core.36). A cloud host's enrollment stays, its key proved under version 4. Rollback is the release before, which speaks version 3.

## Named follow-ups

Outside this repository, each named for whoever plans the next work:

- boomdrive-cloud takes version 4, beside version 3, and serves `runner/forge-token` under its record 0073, point 2.
- The Boomdrive module gives sluice `boomdrive forge-token` as its credential command, naming the signed copy on nix-darwin (record 0007 as amended on 2026-11-11; core.60).

## Task notes

- **1.1** Copy version 3's directory whole, then change each `$id`, each `protocol_version` and each description naming version 3. `src/protocol/v4.rs` starts as a copy of `v3.rs`'s constants; its types move in 2.1.
- **1.2** `forge-token.schema.json`'s `$id` is `…/protocol/v4/forge-token.schema.json`, and it is listed in `FILES`. Its descriptions say what each field holds, as the other files' do.
- **1.3** The four examples each pass their direction's envelope. `forge_token_refused-a-repository-no-drive-serves.json` has `not_installed` false and words naming the repository.
- **2.1** The tests that name version 3 as the version the runner speaks name version 4: `connection.rs`, `enrollment.rs`, `observer.rs`, `plane.rs`, `refused.rs`, `signatures.rs`, `copies.rs`, `cut.rs`, `ends.rs`, `examples.rs`, `forms.rs`, `shared.rs`, `interface.rs` and `fake.rs`, each by `CURRENT` where it can, and each test that checks version 3's own content by `common::V3` stays. `connection.rs` gains `a_message_of_version_3`, and `refused.rs`'s `a_message_of_another_version` checks versions 3 and 4 against each other.
- **3.3** The fake ask's statement is checked by `key`'s public functions, not a copy of them, so the fake proves the bytes the crate signs.
- **4.3** `forge_tokens.rs`'s cloud tests build `answer`'s `Forge` with `Forge::asking` over `boomdrive_control::fake::Tokens`. The binary test runs under the scratch host and the stub `systemd-creds`, as `tests/enroll.rs` does, so no test reaches a real store.
