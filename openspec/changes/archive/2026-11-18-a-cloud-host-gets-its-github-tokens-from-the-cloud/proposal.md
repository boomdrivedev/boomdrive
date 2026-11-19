# Proposal

## Why

The brief: "On a host that Boomdrive Cloud serves, sluice and the runner get their GitHub tokens from the cloud for the repositories of the drives the host serves, just as they get them from the runner's own key on a host without the cloud." Its source is record 0007, under which the cloud holds the Boomdrive App, no App key is on a cloud host, and a tool takes its token from `boomdrive forge-token`.

On a host without the cloud, the runner mints a served repository's token from the App's key `[forge]` names, gives it to every tool by `boomdrive forge-token`, and keeps one for each repository for its own fetches. On a cloud host no key is there, and no version of the protocol carries a token. So `boomdrive forge-token` refuses every operation, naming version 3, the runner's own fetch goes with the host's helpers, and sluice has no token to land with. Releases from `v0.18.0` spoke version 3, so it cannot change (core.42).

## What Changes

- **Record 0053 decides how a cloud host asks.** Record 0007's amendment of 2026-11-18, core.33's rule and details, and core.42's details read with it. All are written with this change.
- **Version 4 of the protocol, at `crates/boomdrive-hosted/protocol/v4/`, is version 3 whole but for these:**
  - each file's `$id` is under `…/protocol/v4/`, every message's `protocol_version` is 4, the subprotocol is `boomdrive.v4`, and every context starts `boomdrive.v4`;
  - a twelfth file, `forge-token.schema.json`, holds the runner's ask, `forge_token`, naming one repository, and the control plane's two answers: `forge_token_given`, with the token and when it expires, and `forge_token_refused`, with whether the App's installation does not cover the repository, and words;
  - `from-runner.schema.json` takes the ask, and `to-runner.schema.json` the two answers.
- **Version 4's examples are version 3's**, each naming `protocol_version` 4, with `enroll.json` signed over version 4's context, and four more: an ask, a token given, and two refusals.
- **A test holds version 4 to version 3**, and version 3 to the files `v0.18.0` spoke.
- **The ask is one request at a path of its own**, `POST runner/forge-token`. Its `BoomdriveHost` statement is signed over a context of its own, `forge token`, which binds the repository. It is stopped at twenty seconds. A forge token's ask or answer is never sent on a partition's socket.
- **`forge_token_given`'s `token` is the one field of any version that names a credential.** The check that no version names a grant or a credential excepts the forge token's three kinds and that field, in version 4 alone.
- **The runner speaks version 4 alone**: its upgrade asks for `boomdrive.v4`, and its enrollment's `enroll` is a message of version 4. A control plane accepts the current version and the one before.
- **The interface names a control plane's forge tokens.** `boomdrive-control` gives `ForgeTokens`, what gives a served repository's token where the control plane holds the App, and its fake gives a test's answers. The hosted implementation's is opened from its table and the host's store, and asks each partition served the repository, in the table's order, until one gives a token.
- **`boomdrive forge-token` on a cloud host gives the control plane's token.** `get` reads and checks git's request as it does on a host that holds the key. It gives the same three lines on standard output alone, and keeps nothing. `erase` answers nothing, as on any host. Its refusals name the hosted implementation's words, with the control plane's as given.
- **The running runner's fetch on a cloud host goes through the control plane's token.** It keeps the token, the App's absence and a refusal as it keeps what it mints: the token until five minutes before its end, the App's absence for fifty-five minutes, and a refusal not at all.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `runner-protocol`: version 4 beside versions 1 to 3, version 3 whole but for a forge token's ask and answers; the ask's path and its context; the one field that names a credential; the runner's types of version 4; version 3 held as `v0.18.0` spoke it; no forge token in versions 1 to 3.
- `hosted-control-plane`: the runner speaks version 4; the hosted implementation asks a served repository's token by `runner/forge-token`, under the key of each partition served it in turn, and checks each answer against version 4.
- `forge-tokens`: on a cloud host, `boomdrive forge-token` gives the control plane's token in place of refusing, and the running runner keeps what the control plane gives as it keeps what it mints.
- `clones`: a cloud host's fetch goes through the control plane's token.

## Impact

- `docs/adr/0053-…md` is new. Record 0007's amendment of 2026-11-18 names it, and `docs/architecture/core.md` amends core.33's rule and details and core.42's details. Each is written with this change.
- `crates/boomdrive-hosted`:
  - `protocol/v4/`, new: version 3's eleven schema files and its examples, with version 4's changes, `forge-token.schema.json` and four examples of its own;
  - `protocol/README.md`: version 4, its subprotocol, contexts and worked bytes, its third path, the ask and its answers, and how version 3 differs;
  - `src/protocol.rs`, `src/protocol/v3.rs` and a new `src/protocol/v4.rs`: version 4's types, and version 3's files alone;
  - a new `src/forge_token.rs`: the hosted implementation's `ForgeTokens`;
  - `src/key.rs`: the ask's context and statement;
  - `src/check.rs`, `src/copy.rs`, `src/connection.rs`, `src/enrollment.rs`, `src/observer.rs`, `src/plane.rs`, `src/table.rs`, `src/lib.rs` and `Cargo.toml`: version 4, and `NO_FORGE_TOKEN` gone;
  - `tests/`: every test on version 4, with versions 1 to 3 still checked; the fake control plane serving `runner/forge-token`; a new `tests/forge_tokens.rs`; and `tests/worked-bytes.json` with version 4's bytes and the ask's.
- `crates/boomdrive-control`: `src/lib.rs` gains `ForgeTokens` and its answer; `src/fake.rs` gives a fake; `src/want.rs` and `src/report.rs` name version 4 where they name the version that carries an act.
- `crates/boomdrive`:
  - `src/planes.rs`: an implementation that holds the App opens its `ForgeTokens`;
  - `src/forge.rs`: a `Forge` that asks a control plane in place of minting;
  - `src/forge_token.rs`, `src/run.rs` and `src/main.rs`: a cloud host's `get` and fetch take the control plane's token;
  - `src/config.rs`: `[forge]` is still refused beside a control plane that holds the App;
  - `tests/forge_tokens.rs` and `tests/clones.rs`: a cloud host's tokens and fetches.
- `README.md`: the hosted section names version 4, and the forge-token section says a cloud host's tokens come from the control plane.
- No crate, tool, configuration key or field of the report file is added, and no tracked file type. Version 4 is a new directory of the protocol, which core.42 provides for. No file under `protocol/v1/`, `protocol/v2/` or `protocol/v3/` changes.
- Outside this repository:
  - boomdrive-cloud pins version 2 (`workers/control-plane/runner-protocol/`). It takes version 4, and builds `runner/forge-token` under its record 0073, point 2, by a change of its own. Until it does, a release that carries this change cannot connect to it, and a cloud host's ask is refused, its fetches going with no credential.
  - The Boomdrive module gives sluice no credential command yet (record 0007 as amended on 2026-11-11). The change that gives it one names, on nix-darwin, the copy of the runner core.60 signs, so the keychain does not ask the person before each `get` reads a partition's key.

  Each is named in design.md.

## Touches

```
docs/adr/0053-a-cloud-host-asks-its-control-plane-for-each-forge-token-by-a-request-of-version-4-of-its-protocol.md
docs/adr/0007-the-cloud-holds-the-github-app-a-host-holds-no-app-key-and-repositories-change-only-through-the-hosts-git.md
docs/architecture/core.md
README.md
crates/boomdrive-control/src/
crates/boomdrive/src/config.rs
crates/boomdrive/src/forge.rs
crates/boomdrive/src/forge_token.rs
crates/boomdrive/src/main.rs
crates/boomdrive/src/planes.rs
crates/boomdrive/src/run.rs
crates/boomdrive/tests/
crates/boomdrive-hosted/Cargo.toml
crates/boomdrive-hosted/protocol/README.md
crates/boomdrive-hosted/protocol/v4/
crates/boomdrive-hosted/src/
crates/boomdrive-hosted/tests/
openspec/changes/a-cloud-host-gets-its-github-tokens-from-the-cloud/
```
