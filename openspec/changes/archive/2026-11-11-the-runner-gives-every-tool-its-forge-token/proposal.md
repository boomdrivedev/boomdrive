# Proposal

## Why

The brief: "On a host whose control plane is not hosted, the runner holds the one Boomdrive App's key, a path in its own settings outside the Nix store, and boomdrive forge-token answers git's credential-helper protocol: get mints, or gives the one it keeps, a short-lived installation token for one repository of a drive the host serves, on standard output alone; erase drops that repository's token so the next get mints a fresh one and never the erased one, revoking nothing at GitHub; anything else is refused, and no token reaches a file, a log or its error output. The runner's own clones and fetches of served repositories take their token the same way. On a cloud host it refuses, naming that the protocol carries no forge token yet."

Record 0007, as amended on 2026-11-09 and accepted on the user's card ("One Boomdrive App; the runner holds its key and gives every tool its tokens"), makes the family's forge identity one GitHub App. On a host whose control plane is not hosted, the runner holds the App's key. Every tool takes its token from `boomdrive forge-token`. Rule core.33 says so. Today the runner has neither a key nor the command. Its fetches go through the host's own credential helper, which gives the person's credentials. sluice keeps the App's key where it reads it, and since record 0108 it can take its token from a credential command instead.

Where the runner keeps a token comes from the rules. `boomdrive forge-token` runs as a process of its own for each operation. No command connects to the running runner (core.20). No credential is kept in a file or the data directory (core.4, core.35). So a `get` mints in its own process and keeps nothing past its answer. Its asker keeps the token, as sluice does. The running runner keeps one token for each repository for its own git calls. Record 0007's amendment of 2026-11-11, written with this change, records these choices.

## What Changes

- **The configuration names the App.** A top-level `[forge]` table names `app_id` and `key_file`, the path of the App's private key outside the Nix store. Both are required where the table is present. Any other key is refused, as is a `key_file` that is relative or under the store. Under a hosted control plane, `[forge]` is refused, because that control plane holds the App (core.33). The local layer may not set it.
- **The runner mints a token from the App's key.** A new module, `crates/boomdrive/src/forge.rs`, signs a JWT with the key and finds the App's installation on a repository. It buys a token for that repository alone, by two calls to GitHub's API, each stopped at ten seconds. It refuses a key file that anyone but its owner may read. Its words never hold the token, the JWT or the key. GitHub's answer that the App has no installation on the repository is an answer of its own, not a failure.
- **`boomdrive forge-token get|erase`** speaks git's credential-helper protocol. It reads the configuration as `run` does. On a cloud host every operation refuses, naming that the protocol carries no forge token yet. `get` reads git's `protocol`, `host` and `path`. For `https`, `github.com` and a repository of a drive the host serves, it mints a token. It prints `username`, `password` and `password_expiry_utc` on standard output alone. `erase` reads what it is sent and answers nothing. Nothing is kept past a process, and nothing is revoked at GitHub, so the next `get` mints afresh and never gives the erased token. Any other operation is a usage error. A refusal is one line of words on standard error and nothing on standard output, and the command exits as core.39 has it. It takes no `--json`, because git reads a helper's standard output as the protocol.
- **The runner's own fetch takes the App's token.** Where the configuration names `[forge]`, the running runner keeps one token for each served repository in memory, until five minutes before its end. A fetch, a new clone's first among them, empties the host's helpers for that call. Its one helper reads the kept token from that call's environment alone. A fetch that fails drops the repository's kept token. A repository the App is not installed on is fetched with no credential. That answer is kept as a token would be. Any other failure to mint leaves the fetch with no credential too, its words after git's where the fetch fails. Where the configuration names no `[forge]`, every fetch goes through the host's own helper, as today.
- **The modules keep the key out of the store.** `services.boomdrive.settings.forge` is declared, its `app_id` an integer and its `key_file` a string. Evaluation fails where `key_file` is under the Nix store.
- **Records and rules.** Record 0007 gains its amendment of 2026-11-11. core.28 lets a mint's calls to GitHub's API leave the host. core.33's details name the keeping and the runner's own helper. core.39's details name `forge-token`'s form. All three are written with this change.

## Capabilities

### New Capabilities

- `forge-tokens`: the runner mints a token for a served repository from the App's key. `boomdrive forge-token` gives one in git's credential-helper protocol. On a cloud host it refuses.

### Modified Capabilities

- `configuration`: the configuration may name the App, by `[forge]`, and holds no key, only where it is kept.
- `clones`: a fetch reaches origin through the App's token where the configuration names `[forge]`, and through the host's own helper where it does not.
- `service-modules`: `settings.forge` is declared, and a key file in the store fails evaluation.

## Impact

- `crates/boomdrive`:
  - `src/config.rs`: `Config::forge`, `ForgeApp`, the `[forge]` table's reading and refusals;
  - `src/forge.rs`: new. The JWT, the two calls, the mint, `Forge` keeping the runner's own tokens;
  - `src/forge_token.rs`: new. `boomdrive forge-token`'s reading of git's request, its answer and its refusals;
  - `src/git.rs`: `Credential`, given to a call to a remote;
  - `src/clone.rs`: `fetch` and `make` take the call's credential;
  - `src/pass.rs`: `pass` and `keep_one` take the forge, and each keep asks it for its repository's credential;
  - `src/run.rs`: the runner holds one `Forge` across passes;
  - `src/planes.rs`: `Entry::holds_the_app`;
  - `src/main.rs` and `src/lib.rs`: the command and the modules;
  - `Cargo.toml`: `jsonwebtoken`, `ureq` and `time`, with `rsa` for the tests;
  - tests: `tests/github/mod.rs`, a fake of GitHub's API on the loopback address; `tests/forge_tokens.rs`, new; `tests/common/mod.rs`, a pass with a forge; `tests/clones.rs`, `tests/module_files.rs` and `tests/readme.rs`.
- `crates/boomdrive-hosted/src/lib.rs`: `NO_FORGE_TOKEN`, the hosted implementation's words for a cloud host's refusal.
- `Cargo.toml` and `Cargo.lock`: `jsonwebtoken` and `rsa` added to the workspace, and the dev profile's build of `rsa`'s arithmetic optimized.
- `nix/modules/options.nix`, `nix/tests/settings.nix`, `nix/tests/file.nix` and `nix/tests/files/crew.toml`: `settings.forge`, its guard and its tests.
- `README.md` and `CLAUDE.md`: `[forge]`, the fetch's credential and `boomdrive forge-token`.
- `docs/adr/0007-…` and `docs/architecture/core.md`: record 0007's amendment of 2026-11-11, and core.28, core.33 and core.39, written with this change.
- The protocol is unchanged. Version 2 carries no forge token, and a version that does is a change of its own.
- Nothing is written to the data directory, a file or the host's credential store. No configuration key but `[forge]`'s, no field of `report.json` and no message of the protocol is added.
- Outside this change: the set's sluice, v0.78.1, reads no credential command, which sluice reads from v0.84.0 (sluice record 0108). Moving its tag, and having the Boomdrive module give sluice `boomdrive forge-token`, is a change of its own (core.45, core.57). Design Non-Goals says so.

## Touches

```
crates/boomdrive/Cargo.toml
crates/boomdrive/src/config.rs
crates/boomdrive/src/forge.rs
crates/boomdrive/src/forge_token.rs
crates/boomdrive/src/git.rs
crates/boomdrive/src/clone.rs
crates/boomdrive/src/pass.rs
crates/boomdrive/src/run.rs
crates/boomdrive/src/planes.rs
crates/boomdrive/src/main.rs
crates/boomdrive/src/lib.rs
crates/boomdrive/tests/github/mod.rs
crates/boomdrive/tests/forge_tokens.rs
crates/boomdrive/tests/common/mod.rs
crates/boomdrive/tests/clones.rs
crates/boomdrive/tests/module_files.rs
crates/boomdrive/tests/readme.rs
crates/boomdrive-hosted/src/lib.rs
Cargo.toml
Cargo.lock
nix/modules/options.nix
nix/tests/settings.nix
nix/tests/file.nix
nix/tests/files/crew.toml
README.md
CLAUDE.md
docs/adr/0007-the-cloud-holds-the-github-app-a-host-holds-no-app-key-and-repositories-change-only-through-the-hosts-git.md
docs/architecture/core.md
openspec/specs/forge-tokens/spec.md
openspec/specs/configuration/spec.md
openspec/specs/clones/spec.md
openspec/specs/service-modules/spec.md
```
