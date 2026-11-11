# Design

## Context

See proposal.md for why. There is no area page for `crates/` or `nix/`, so `docs/architecture/core.md` binds every path this change touches. The change relies on these rules:

- core.2: the control plane is reached through one interface, and no code outside an implementation names the hosted protocol. A cloud host's refusal and its words come from the hosted implementation, through the list of implementations (Decision 6);
- core.3 and core.20: no connection comes in to a host, and no command connects to the running runner. So `boomdrive forge-token` cannot ask the running runner for a token it keeps (Decision 3);
- core.4 and core.35: no credential is in the repository, a file or the data directory. A token is kept in memory alone, and the tests make their key at run time (Decision 9);
- core.12: `forge`, `app_id` and `key_file` name what they hold, and `forge` is sluice's word for the same thing, the App on the forge;
- core.17: the runner is configured by its configuration alone. `[forge]` is a key of the managed layer, which the local layer may not set (Decision 1);
- core.28: what leaves the host. This change adds a mint's calls to GitHub's API (Decision 2);
- core.33: the rule this change builds, as record 0007's amendment of 2026-11-09 has it;
- core.34: the runner changes a repository only through the host's `git`, with a credential helper set for one call (Decision 5);
- core.39: a command's form, from which `forge-token` departs for its answer alone (Decision 4);
- core.46: no option, file or store path the modules make holds a key (Decision 7);
- core.58: a git call to a remote is stopped at thirty seconds, and its words name the call. A mint's calls take a limit of their own, inside sluice's limit on the whole command (Decision 2).

It departs from core.39 for `forge-token`'s answer, and from record 0007's amendment of 2026-11-09 where it says the runner keeps one token and hands it to every tool that asks (Decision 3). Record 0007's amendment of 2026-11-11 records both, with core.28's, core.33's and core.39's amendments, all written with this change.

**How the family mints today.** sluice mints as a GitHub App (sluice record 0019, `crates/sluice/src/profiles/github/app.rs`):

- it signs a JWT with the App's key, RS256, by `jsonwebtoken` 9;
- it finds the App's installation on the repository, `GET /repos/{owner}/{name}/installation`;
- it buys a token for that repository alone, `POST /app/installations/{id}/access_tokens` with `{"repositories":["<name>"]}`;
- it keeps the token in its process's memory until five minutes before its end.

Since sluice record 0108, `[forge] credential` names a command that gives the token in place of the key. sluice runs it as git runs a credential helper:

- `get` writes `protocol`, `host` and `path`, the repository as `<owner>/<name>`, and reads `password` and `password_expiry_utc`;
- `erase` writes the same with `username` and `password`, and reads no answer;
- a renewal that gets back the token it erased fails;
- the command has thirty seconds, `COMMAND_LIMIT`, and its standard error is scrubbed before it reaches a failure.

sluice reads `credential` from v0.84.0. The flake's set holds v0.78.1.

**The runner today.** It has no App and no key. `clone::fetch` runs `git fetch` through `Git::call_remote`, the host's own helpers answering, with `GIT_TERMINAL_PROMPT=0` (the clones spec's "A clone is fetched over https and never asks"). `pass::keep` fetches a clone it has, and `clone::make` makes a new one by a first fetch. `keep_one` keeps a landing's repository by the same `keep`. The configuration's `File` refuses every key it does not name. The module refuses a `settings` key named `key` or ending in `_key`.

## Goals / Non-Goals

**Goals:**

- On a host whose control plane is not hosted, the runner mints a token for a served repository from the App's key its configuration names.
- `boomdrive forge-token` speaks git's credential-helper protocol to git and to sluice: `get` gives a served repository's token on standard output alone, and `erase` revokes nothing. Anything else is refused.
- The runner's own fetches of served repositories take the App's token.
- On a cloud host, `forge-token` refuses, naming that the protocol carries no forge token yet.
- No token reaches a file, a log, the data directory or error output.

**Non-Goals:**

- **The set giving sluice the credential command.** sluice v0.78.1, the set's, reads no `[forge] credential`. Moving sluice's tag (core.45), and having the Boomdrive module give sluice `credential = [ "<runner>/bin/boomdrive" "forge-token" ]` and the runner's `app_id` (core.57), is a change of its own. Until then a host keeps sluice's own `[forge] key`, as record 0007's amendment allows. Record 0007's amendment of 2026-11-11 names it.
- **A version of the protocol that carries tokens.** A cloud host's token waits on it.
- **crew's state repository and an agent's git.** crew acts with the person's credentials (record 0007's amendment). crew's listing and weir-branch's fetch and push keep the host's helpers.
- **A pause or retry policy for GitHub's allowance.** A mint is tried once. Its asker tries again, sluice under its own policy and the runner at its next pass. A token is bought at most once an hour for each repository and asker.
- **Narrowing a token's permissions, or revoking one.** The record does not take either.

## Decisions

### 1. `[forge]` names the App, in the managed layer alone

The configuration gains a top-level table:

```toml
# The Boomdrive App the runner holds the key of, and gives every tool its tokens from.
[forge]
app_id = 123456
key_file = "~/.config/boomdrive/app.pem"
```

- `Config` gains `forge: Option<ForgeApp>`. `ForgeApp { app_id: u64, key_file: PathBuf }` has `~/` expanded against the home, as `repository_root` is.
- `File` gains `forge`, read from the document's tree as `accounts` is, so each refusal names its key's path and line and never its value (core.4).
- Refused, naming the key's path:
  - `forge` that is not a table;
  - a table missing `app_id` or `key_file`;
  - any other key in it, as a key the runner does not read;
  - an `app_id` that is not a positive integer;
  - a `key_file` that is not a string, is neither absolute nor under `~/`, or lies under the Nix store directory, `/nix/store/`, with the words `lies in the Nix store, which every user reads: name a path outside it`.
- The key file itself is not read with the configuration. Its absence or its mode refuses a mint (Decision 2), not the configuration, so a pass keeps every repository whatever stands at the path.
- A configuration whose `[control_plane]` names an implementation whose control plane holds the App refuses `forge`, naming it: `forge` names an App's key, and under the hosted control plane the App is the control plane's, so no App's key or id is on this host (core.33). The configuration asks `planes::carried(type).holds_the_app` (Decision 6) and names no implementation itself (core.2).
- The local layer may not set `forge`. It is no key `local_keys` lists, so a local layer that sets it is not taken (core.17; the configuration spec's "A local layer that sets any other key is not taken").

Not taken:

- **`key` as sluice names it.** core.12 has a path's key end in `_file`. The module's guard, which refuses `key` and `*_key`, passes `key_file`, since it names where the key is and not the key.
- **The table under `[control_plane.crew]`.** A host with no control plane keeps repositories and fetches them too. The App is the host's whatever control plane it serves.
- **Reading the key at the configuration's reading.** A key moved or mis-moded would then stop every pass, the keeping of public repositories among it.

### 2. A mint is two calls to GitHub's API, stopped at ten seconds each

A new module, `crates/boomdrive/src/forge.rs`, holds the mint:

- **The key.** `ForgeApp::key_file` is read once for each mint. It is refused where:
  - it cannot be read: `[forge] key_file <path> cannot be read: <error>`;
  - anyone but its owner may read it, as sluice refuses a key: `[forge] key_file <path> has mode <mode>, which lets others read it: give it mode 0600 (chmod 600 <path>)`;
  - it holds no RSA private key in PEM, PKCS#1 or PKCS#8: `[forge] key_file <path> holds no RSA private key in PEM`.

  Each of these is a host to fix.
- **The JWT.** It is signed RS256 by `jsonwebtoken` 9, as sluice signs one. Its claims are `iss`, the App's id as a string, `iat`, a minute before now for a clock a little ahead, and `exp`, nine minutes after now, within GitHub's ten.
- **The calls.** Both go to `https://api.github.com` by `ureq` 3, the workspace's, over rustls with the host's own roots. Each carries `Accept: application/vnd.github+json`, `X-GitHub-Api-Version: 2022-11-28` and `User-Agent: boomdrive`:
  1. `GET /repos/{owner}/{name}/installation`, with the JWT. A 404 is the answer `NotInstalled`, GitHub's word that the App has no installation there, and no failure. Any other status but 200 is a refusal.
  2. `POST /app/installations/{id}/access_tokens`, with the JWT and the body `{"repositories":["<name>"]}`. A 201 gives `token` and `expires_at`, RFC 3339. Any other status is a refusal.
- **The limit.** Each call is stopped at ten seconds, `CALL_FOR`, ureq's global timeout for it. So a `get` answers within the thirty seconds sluice gives a credential command (sluice record 0108), whatever GitHub does.
- **What a mint gives.** `Forge::mint(&ForgeApp, &Address) -> Result<Minted, Refused>`. `Minted` is `Token(Token)` or `NotInstalled`. `Token { secret, ends }` has a `Debug` that names its end and never its secret. `Refused { words, host_to_fix }` is the key's refusals above, or GitHub's:
  - `GitHub answered <status> to <call>: <message>`, where `<message>` is the answer's `message` field, or nothing where it has none;
  - `GitHub could not be reached to <call>: <error>`;
  - where `<call>` is `finding app <id>'s installation on <owner>/<name>` or `buying a token for <owner>/<name> from installation <id>`.

  No refusal holds the token, the JWT or the key's text (core.4).
- **The API's address is a field.** `Forge::new()` calls `https://api.github.com`. `Forge::at(api)` gives a forge that calls a test's fake, and `Forge::calling_for(limit)` one whose calls stop at a test's shorter limit. No configuration key, environment variable or flag sets either (core.17), as record 0042's Decision 6 has a test's limit.

core.28 gains these calls: "the calls to the forge's API that mint a token from the App's key it holds (core.33)".

Not taken:

- **sluice's retry policy and pauses.** They guard a process making many calls to GitHub. A mint is two calls, made at most once an hour for each repository and asker, and its asker tries again.
- **Thirty seconds for each call.** Two such calls could outlast sluice's limit on the whole command, which would then be killed with no words of the runner's.

### 3. A token is kept in the process that minted it, and nowhere else

`boomdrive forge-token` is a process of its own for each operation git or sluice asks. The rules leave it nowhere to keep a token for the next process:

- no command connects to the running runner (core.20, core.3);
- no credential is kept in a file or the data directory (core.4, core.35);
- a shared thing is decided in one process, never by separate processes coordinating through storage (blueprints `design/principles.md` 6).

So:

- **A `get` mints in its own process,** gives the token, and keeps it no longer. Its asker keeps what it was given, as git's protocol has a caller or a storage helper keep a credential, and as sluice keeps its token until five minutes before its end.
- **An `erase` drops nothing kept,** since nothing is. It reads what it is sent and answers nothing. The next `get` mints afresh, and GitHub never mints a token twice, so it never gives the erased one. Nothing is revoked at GitHub.
- **The running runner keeps one token for each repository** for its own git calls (Decision 5). `Forge` holds them in a `Mutex<HashMap<Address, Kept>>` with the `ForgeApp` they were minted for. A `ForgeApp` other than the one held, a changed `app_id` or `key_file`, empties it first. `Forge::credential(Option<&ForgeApp>, &Address)` gives:
  - the held token, while it ends more than five minutes from now, `RENEW_BEFORE`, as sluice's `REFRESH` has it;
  - else it mints and keeps what it minted;
  - `NotInstalled` is kept too, for fifty-five minutes, the hour of a token less `RENEW_BEFORE`, so a repository the App is not installed on costs one call an hour;
  - a refused mint is kept not at all, and the next pass asks again.

  `Forge::drop(&Address)` drops a repository's kept token or answer.

Record 0007's amendment of 2026-11-09 says "the runner keeps one token for each repository and hands it to every tool that asks". This decision keeps that only within the runner's own process. Its amendment of 2026-11-11 records why, from the brief's own words that no token reaches a file.

Not taken:

- **The host's credential store as the keeping across processes.** A keychain item made by one build of the runner asks the person before another build reads it, so a new release would block every `get` behind a dialog. The login keychain is locked to a process started over ssh. On Linux, `systemd-creds` keeps a credential as a file, and the kernel's keyring is shared only with processes that possess it, which a user's services and their shells may not.
- **A socket to the running runner,** which core.3 and core.20 rule out.

### 4. `boomdrive forge-token` answers git's protocol, and refuses in words on standard error

`main.rs` gains the command:

```
boomdrive forge-token [--config <file>] <get|erase>
```

The operation is a `clap::ValueEnum`, so any other, `store` among them, is clap's usage error on standard error, exit 2. `forge_token::main(config, operation)` runs it. `forge_token::answer(config, operation, input, output, forge) -> Result<(), Refused>` is the whole of it but the reading of the configuration, so the tests run it in-process against a fake API (Decision 9).

In order:

1. **The configuration** is read as `run` reads it, its local layer over its managed one, so a repository the local layer adds is served. A configuration refused is a host to fix, with its own words.
2. **A cloud host refuses every operation.** Where the configuration's control plane's entry holds the App (Decision 6), the refusal is the entry's words, `boomdrive_hosted::NO_FORGE_TOKEN`: `the hosted control plane mints this host's forge tokens, and version 2 of its protocol carries none yet`. Exit 1.
3. **No `[forge]`** is a host to fix: `the configuration names no [forge], the Boomdrive App's app_id and key_file`.
4. **What git sends** is read from standard input as git writes it, `key=value` lines to a blank line or the end. `protocol`, `host` and `path` are read, and every other key, `capability[]` and `wwwauth[]` among them, is passed over as git passes it over. `erase` reads it to its end and answers nothing, exit 0.
5. **`get` refuses**, exit 1:
   - a request for anything but `https` and `github.com`, the host compared without case: `git asked for <protocol>://<host>, and the runner gives tokens for https://github.com alone`;
   - a request with no `path`: `git named no repository: set credential.useHttpPath so git sends its path`;
   - a path that is no address, read as `owner/name` with a trailing `.git` or `/` taken off: `<path> is no repository's address`;
   - a repository of no drive the host serves, compared without case as an address is: `<owner>/<name> is a repository of no drive this host serves` (core.33);
   - the App not installed on it: `GitHub finds no installation of app <id> on <owner>/<name>: install the app on it`.
6. **`get` answers** with a token minted by `Forge::mint`, on standard output alone, exit 0:

   ```
   username=x-access-token
   password=<token>
   password_expiry_utc=<expires_at in seconds since the epoch>
   ```

   A mint refused refuses the `get`: exit 2 where the host must fix its key, else 1.

Every refusal is one line on standard error, `boomdrive forge-token: <words>`, and nothing on standard output, which git and sluice read as the protocol. No refusal holds a token, the JWT or the key. Nothing is written to the data directory or a file, and nothing is logged.

`forge-token` takes no `--json`, a departure from core.39 for this one command. git reads a helper's standard output as the protocol, and a refusal written there in JSON would be read as a malformed credential. Its exits are core.39's: 0 answered, 1 refused, 2 for a usage error or a host to fix. core.39's details name it.

Not taken:

- **`store` answered as a no-op,** as `sluice credential` answers it. The brief has anything but `get` and `erase` refused. A person who names `forge-token` as git's own helper sees git's `store` refused after a call, which says the runner keeps nothing.
- **`--json` giving the token in JSON.** No reader needs it, and it would be one more way for the token to land in a file.

### 5. The runner's own fetch is handed the token its process keeps

- **`git::Credential`** is what a call to a remote is given:
  - `Host`: the host's own helpers, as today, with no argument naming a credential;
  - `Token(String)`: the App's token alone. The call is given `-c credential.helper=`, which empties every helper the host's configuration names, its URL-scoped ones among them. It is then given `-c credential.helper=<TOKEN_HELPER>`, and the token in that call's environment as `BOOMDRIVE_FORGE_TOKEN`;
  - `Nothing`: no credential. The call is given `-c credential.helper=` alone.

  Its `Debug` never shows the token.
- **`TOKEN_HELPER`** is git's shell form of a helper, which answers a `get` from the call's environment, reads what git sends to its end, and answers nothing else:

  ```
  !f() { cat >/dev/null; test "$1" != get || printf 'username=x-access-token\npassword=%s\n' "$BOOMDRIVE_FORGE_TOKEN"; }; f
  ```

  The token is never on a command line, so no process listing shows it. It reaches git and the processes git starts for that call alone, as sluice's own git calls are given theirs (sluice record 0019).
- **`Git::call_remote(at, remote, args, &Credential)`** takes the credential. `clone::fetch(git, bare, &Credential)` and `clone::make(git, data, address, &Credential)` pass it to their fetch.
- **The pass asks the forge.** `pass::pass` and `pass::keep_one` take `&Forge` after their git. `pass` takes `#[allow(clippy::too_many_arguments)]` where clippy's limit asks, as `keep` does. `keep_configured` asks `forge.credential(config.forge.as_ref(), address)` for the repository it keeps, and hands what it gives to `keep`:
  - no `[forge]`: `Host`;
  - a token: `Token`;
  - `NotInstalled`: `Nothing`;
  - a refused mint: `Nothing`, its words kept for the line.

  The runner holds one `Forge` across passes, `Runner::forge`, which the loop and the keeping of a landing share. Parts do not fetch.
- **A failed fetch** drops the repository's kept token or answer, so the next pass mints again. Where the mint was refused, the line is `<address>: not fetched: <git's words>; no forge token: <the mint's words>`, or `not cloned` for a new clone. Where the fetch succeeds, a public repository's, nothing more is said.
- **`not_by_ssh`'s `ls-remote --get-url`** reaches no remote and takes no credential.

Not taken:

- **`boomdrive forge-token` as the runner's own helper.** Each fetch would run a process of its own, which mints, so the runner would buy a token for each repository at every `fetch_interval`.
- **`http.extraHeader`.** It is no credential helper (core.34), and a refused header would still go on to the host's helpers unless they were emptied too.
- **The host's helpers where a mint fails.** They give the person's credentials, which record 0007's amendment moves the runner's fetches off.
- **A fetch of a repository the App is not installed on left unrun.** A drive may keep a public repository of someone else's, which the App can never be installed on.

### 6. The hosted implementation says why a cloud host refuses

`planes::Entry` gains `holds_the_app: Option<&'static str>`. Where an implementation's control plane holds the Boomdrive App and mints the host's tokens, it holds the words `forge-token` refuses with until the protocol carries a token. It is none where the runner holds the App's key:

- crew's entry: `None`;
- hosted's entry: `Some(boomdrive_hosted::NO_FORGE_TOKEN)`.

`boomdrive-hosted`'s `lib.rs` gains `NO_FORGE_TOKEN`, `the hosted control plane mints this host's forge tokens, and version 2 of its protocol carries none yet`. A version that carries tokens changes these words with it. The configuration's refusal of `[forge]` (Decision 1) and `forge-token`'s refusal (Decision 4) both read the field, so no code outside an implementation names hosted (core.2).

Not taken: **comparing `[control_plane]`'s `type` with `"hosted"`** in `config.rs` or `forge_token.rs`, which core.2 rules out.

### 7. The modules keep the key out of the store

`nix/modules/options.nix` declares `settings.forge`, unset by default and rendered only where set:

- `app_id`, `types.ints.positive`;
- `key_file`, `types.str`. A Nix path value fails the type, so a key is never copied into the store.

An assertion fails evaluation where `key_file` begins with the store directory, `builtins.storeDir`. Its words: `services.boomdrive.settings.forge.key_file lies in the Nix store, which every user reads: name the App's key by a path outside it, absolute or under ~/`. The credential guard passes `key_file`, which ends in no credential's word.

`nix/tests/settings.nix`'s crew value gains `forge = { app_id = 1; key_file = "~/.config/boomdrive/app.pem"; }`. `nix/tests/files/crew.toml` is rendered again with it, and `tests/module_files.rs` reads it with the runner's reader. The runner refuses `[forge]` beside a hosted control plane, one home for that refusal, as sluice's core.51 leaves a value sluice refuses to sluice. The module does not refuse it too.

### 8. What is said of it

- README's configuration block shows `[forge]`, and its words say what it holds, that the key file stays outside the store and readable by its owner alone, and that the local layer may not set it.
- README's pass says that a fetch goes through the App's token where `[forge]` is named, and through the host's own helper where it is not.
- README gains a section, `## Giving every tool its forge token`, after "Running on a host". It covers `boomdrive forge-token get|erase`, its answer and its refusals, that each `get` mints, that `erase` revokes nothing, and the cloud host's refusal. It says that the set's sluice takes no token from it yet, and that the set gives sluice the command once it holds a sluice that reads one, so a host writes no credential command of its own (core.57).
- CLAUDE.md's paragraph on the runner gains one sentence on `[forge]` and `boomdrive forge-token`.

### 9. The tests make their key and fake GitHub on the loopback address

- **No key is committed** (core.4). A test makes an RSA key of 2048 bits at run time with the `rsa` crate, a dev-dependency, once for each test binary. It writes the key as PEM, mode 0600, into its scratch home. The workspace's dev profile builds `rsa`'s arithmetic, `num-bigint-dig`, at `opt-level = 3`, so the key is made in well under a second.
- **A fake of GitHub's API**, `crates/boomdrive/tests/github/mod.rs`, is a `TcpListener` on `127.0.0.1:0` answering HTTP/1.1, one request for each connection:
  - `GET /repos/{owner}/{name}/installation`: 200 with an `id` for a repository it was told the App is installed on, else 404 with GitHub's `message`;
  - `POST /app/installations/{id}/access_tokens`: 201 with a fresh `token` each time and the `expires_at` it was told, an hour by default;
  - an answer it was told to give in place of either, a 401 or a 403 with a `message`;
  - silence, holding a connection open without a byte.

  It verifies each JWT against the key's public half and keeps each request's method, path and body. No test reaches the network.
- In-process tests use `Forge::at(fake).calling_for(limit)`. `Scratch::pass_with_forge(config, &Forge)` makes a pass with one. The binary's tests run `boomdrive forge-token` only where it refuses before any mint, so the binary never calls GitHub.

## Risks / Trade-offs

1. **[An agent can run `boomdrive forge-token`]** An agent runs as the host's user, as it can read sluice's key today. → Record 0007's amendment takes this as its "Bad". The ruleset keeps agents off guarded branches because their git never asks for the token (sluice record 0019).
2. **[Each `get` costs two calls to GitHub's API]** → sluice asks once an hour for each repository and process, and the runner's own keeping mints once an hour for each repository. A host far busier than that would want a keeping across processes. That is a design of its own against core.20 and principle 6 (Decision 3).
3. **[The token is in the environment of the runner's git call]** A process of the same user can read another's environment on Linux, by `/proc/<pid>/environ`, while the fetch runs. → It is the same exposure as sluice's own git calls, and an agent of that user can run `forge-token` anyway (Risk 1).
4. **[A repository the App is not installed on fails if it is private]** With `[forge]` named, the host's helpers no longer answer the runner's fetch. → The line names git's words. The App is installed on it, or the host names no `[forge]`. A public repository is fetched with no credential.
5. **[sluice does not take the token yet]** The set's sluice reads no credential command. → A host keeps sluice's own `[forge] key` until the change that moves sluice's tag gives sluice `forge-token` (Non-Goals).
6. **[A mint holds the clone's lock a little longer]** A keep that mints holds its clone's lock for its two calls, up to twenty seconds, so a launch into that repository waits as long, once an hour (record 0026). → The calls are bounded by `CALL_FOR`, and a kept token costs nothing.
7. **[git's `store` is refused when a person makes `forge-token` git's own helper]** git asks every helper to `store` after a call that succeeded. → git goes on, and the refusal's line says the runner keeps nothing. The runner's own calls use `TOKEN_HELPER`, which answers `store` with nothing.
