# Tasks

Work through the tasks in order. A task is done once the check it names passes, run as it names it. The groups are headings for reading.

- Record 0007's amendment of 2026-11-11, and core.28's, core.33's and core.39's amendments, are written with this change's plan. A task that relies on a choice the design names cites its Decision.
- Every test runs in a temporary folder or on the scratch host, with neutral names and paths (core.41). No test reaches the network: GitHub's API is the fake of design Decision 9, on `127.0.0.1`. No key is committed: each test binary makes its own RSA key at run time (core.4).
- No task adds a tool, a tracked file type, a field of `report.json`, a message of the protocol, or a configuration key but `[forge]`'s (core.7, core.8, core.17). Nothing outside the files proposal.md's Touches lists is changed.

## 1. The App in the configuration

- [x] 1.1 `crates/boomdrive-hosted/src/lib.rs` gains `pub const NO_FORGE_TOKEN: &str`, `the hosted control plane mints this host's forge tokens, and version 2 of its protocol carries none yet`, documented as the words a cloud host's `boomdrive forge-token` refuses with until a version of the protocol carries a token (record 0007 as amended on 2026-11-09). `planes::Entry` gains `holds_the_app: Option<&'static str>`, documented as design Decision 6 has it: crew's entry `None`, the hosted entry `Some(boomdrive_hosted::NO_FORGE_TOKEN)`. A unit test in `planes.rs`, `only_the_hosted_control_plane_holds_the_app`, asserts both (core.2; design Decision 6). True when `devenv shell -- cargo test -p boomdrive --lib planes::` passes.
- [x] 1.2 `config.rs` gains `ForgeApp { app_id: u64, key_file: PathBuf }`, documented as the Boomdrive App the runner holds the key of (record 0007 as amended; core.33), and `Config::forge: Option<ForgeApp>`. `File` gains `forge`, read from the document's tree as `accounts` is, so each refusal names the key's path and line and never its value. `[forge]` is refused, naming the key by its path:
  - where it is not a table;
  - where it lacks `app_id` or `key_file`;
  - where it holds any other key, as a key the runner does not read;
  - where `app_id` is no positive integer;
  - where `key_file` is no string, is neither absolute nor begins with `~/`, or lies under `/nix/store/`, with the words `lies in the Nix store, which every user reads: name a path outside it`;
  - where `[control_plane]` names an implementation whose `holds_the_app` is some, naming `forge` and saying that control plane holds the App.

  `~/` in `key_file` is expanded against the home, and the file is not read. The local layer may not set `forge`, since `local_keys` lists no key of it. `config.rs`'s module documentation says what `[forge]` holds and that the local layer may not set it (design Decision 1). Unit tests in `config.rs` prove the configuration spec's "The configuration may name the Boomdrive App" and "A key's text in the forge table": `the_app_named`, `no_app_named`, `a_key_file_missing_from_the_forge_table`, `an_app_id_that_is_no_positive_integer`, `a_keys_text_in_the_forge_table`, `a_key_file_in_the_store`, `a_relative_key_file`, `forge_that_is_not_a_table`, `the_app_beside_a_hosted_control_plane` and `the_app_in_the_local_layer`. The existing `a_misspelt_key`, `a_token` and the rest still pass. True when `devenv shell -- cargo test -p boomdrive --lib config::` passes.

## 2. The mint

- [x] 2.1 The workspace's `Cargo.toml` gains `jsonwebtoken = "9"`, commented as the JWT a mint signs with the App's key, RS256, as sluice signs one (record 0007 as amended on 2026-11-11). It gains `rsa` 0.9, with the features that make a key from the system's randomness and write it as PEM, commented as the tests' key, made at run time since no key is committed (core.4). It gains `[profile.dev.package.num-bigint-dig] opt-level = 3`, commented as the tests' key made in well under a second. `crates/boomdrive/Cargo.toml` takes `jsonwebtoken`, `ureq` and `time` from the workspace, each commented with what it does for the mint, and `rsa` as a dev-dependency. `Cargo.lock` moves with them. A new module, `src/forge.rs`, declared in `lib.rs`, holds the key and the JWT of design Decision 2:
  - `CALL_FOR`, ten seconds;
  - `RENEW_BEFORE`, five minutes;
  - `Token { secret, ends }`, whose `Debug` names its end and never its secret, and which gives its secret by a method;
  - `Refused { words, host_to_fix }`;
  - `Forge::new()`, `Forge::at(api)` and `Forge::calling_for(limit)`;
  - the key read for each mint, refused with Decision 2's words where it cannot be read, where its mode lets others read it (`mode & 0o077`), or where it holds no RSA private key in PEM, each a host to fix;
  - the JWT, whose claims are `iss`, the App's id as a string, `iat`, a minute before now, and `exp`, nine minutes after now.

  The module's documentation cites record 0007 as amended on 2026-11-11, core.28 and core.33, and says no token is kept but in a `Forge`'s memory. Its unit tests make a key with `rsa` once, in a `OnceLock`, and write it at mode 0600 into a temporary folder: `a_jwt_names_the_app_and_lasts_nine_minutes` decodes the JWT with the key's public half; `a_key_others_may_read_is_refused` (mode 0644, naming the path, `0644` and `chmod 600`); `a_key_file_that_cannot_be_read_is_refused`; `a_key_file_that_is_no_key_is_refused`; and `a_tokens_debug_holds_no_secret`. True when `devenv shell -- cargo test -p boomdrive --lib forge::` passes.
- [x] 2.2 `Forge::mint(&ForgeApp, &Address) -> Result<Minted, Refused>` makes design Decision 2's two calls by `ureq`, each with ureq's global timeout at the forge's limit, `CALL_FOR` from `Forge::new`. Each carries GitHub's `Accept`, `X-GitHub-Api-Version: 2022-11-28` and `User-Agent: boomdrive` headers and the JWT as its bearer. The first is `GET /repos/{owner}/{name}/installation`, whose 404 gives `Minted::NotInstalled`. The second is `POST /app/installations/{id}/access_tokens` with `{"repositories":["<name>"]}`, whose 201 gives `Minted::Token` with `token` and `expires_at` read as RFC 3339. Every other answer refuses with `GitHub answered <status> to <call>: <message>`, and a call that could not be made, or was stopped at its limit, with `GitHub could not be reached to <call>: <error>`. The words hold no token, JWT or key. `crates/boomdrive/tests/github/mod.rs` is the fake of design Decision 9:
  - a `TcpListener` on `127.0.0.1:0`, answering one HTTP/1.1 request for each connection on a thread of its own;
  - it is told which repositories the App is installed on, the `expires_at` it gives, an answer to give in place of either call, or to hold every connection silent;
  - each token it mints is fresh;
  - it verifies each JWT against the public half of the key it was given, and keeps each request's method, path and body;
  - it holds the tests' key, made by `rsa` once for each test binary and written at mode 0600 into the scratch home.

  `tests/forge_tokens.rs` gains `a_token_is_bought_for_its_repository_alone`, `the_app_not_installed_is_its_own_answer`, `a_key_others_may_read_calls_no_one`, `a_key_file_that_is_no_key_calls_no_one`, `a_refused_mint_names_github_and_holds_no_secret` (a 401 with `A JSON web token could not be decoded`, the words holding no line of the key's PEM and no part of the JWT) and `a_silent_api_is_stopped_at_its_limit` (500 ms, ended within 5 seconds). These prove the forge-tokens spec's "The runner mints a token for one repository from the App's key". True when `devenv shell -- cargo test -p boomdrive --test forge_tokens -- a_token_is_bought_for_its_repository_alone the_app_not_installed_is_its_own_answer a_key_others_may_read_calls_no_one a_key_file_that_is_no_key_calls_no_one a_refused_mint_names_github_and_holds_no_secret a_silent_api_is_stopped_at_its_limit` passes.

## 3. boomdrive forge-token

- [x] 3.1 A new module, `src/forge_token.rs`, declared in `lib.rs`, holds design Decision 4's command. `answer(config, operation, input, output, forge) -> Result<(), Refused>` takes design Decision 4's steps 2 to 6 in order:
  - a configuration whose control plane's entry holds the App refuses every operation with that entry's words;
  - no `[forge]` is a host to fix;
  - git's request is read from `input` to a blank line or its end, `protocol`, `host` and `path` taken and every other key passed over;
  - `erase` answers nothing;
  - `get` refuses another protocol or host, no path, a path that is no address (with a trailing `.git` or `/` taken off), a repository of no drive the configuration keeps, compared as an address is, and `Minted::NotInstalled`, each with Decision 4's words;
  - `get` writes `username=x-access-token`, `password=<token>` and `password_expiry_utc=<seconds>` to `output`, and nothing else.

  `Operation` is a `clap::ValueEnum` of `get` and `erase`. The module's documentation cites record 0007 as amended on 2026-11-11 and core.33, and says why the command takes no `--json` (core.39's details). `tests/forge_tokens.rs` gains in-process tests over the fake:
  - `a_get_gives_a_served_repositorys_token_on_standard_output_alone`;
  - `a_repository_the_local_layer_adds_is_served`;
  - `each_get_mints`;
  - `a_repository_no_drive_serves_is_refused`, the fake asked nothing;
  - `another_host_is_refused`;
  - `a_request_with_no_path_is_refused`;
  - `a_repository_the_app_is_not_installed_on_is_refused`;
  - `an_erase_revokes_nothing_and_the_next_get_mints_afresh`, the fake having seen no request but two installations and two tokens, and the second token not the first;
  - `nothing_is_left_behind`, the scratch host's data and configuration directories holding the same files after a `get` and a refused `get`.

  These prove the forge-tokens spec's `get`, `erase` and "No token reaches a file, a log or error output". True when `devenv shell -- cargo test -p boomdrive --test forge_tokens -- a_get_gives_a_served_repositorys_token_on_standard_output_alone a_repository_the_local_layer_adds_is_served each_get_mints a_repository_no_drive_serves_is_refused another_host_is_refused a_request_with_no_path_is_refused a_repository_the_app_is_not_installed_on_is_refused an_erase_revokes_nothing_and_the_next_get_mints_afresh nothing_is_left_behind` passes.
- [x] 3.2 `main.rs` gains `ForgeToken { config: Option<PathBuf>, operation: Operation }`, documented as giving a served repository's forge token in git's credential-helper protocol. It runs `boomdrive::forge_token::main(config, operation)`. `main` reads both layers as `run` does, with a configuration refused being a host to fix, and runs `answer` with `Forge::new()`, standard input and standard output. Its refusal is one line on standard error, `boomdrive forge-token: <words>`, with exit 1, or 2 for a host to fix. `tests/forge_tokens.rs` gains tests that run the binary, each asserting that standard output is empty:
  - `a_store_is_a_usage_error`, exit 2 with the usage on standard error;
  - `no_forge_must_be_fixed`, exit 2 naming `[forge]`, `app_id` and `key_file`;
  - `a_configuration_it_cannot_read_must_be_fixed`, exit 2 naming the file;
  - `a_get_on_a_cloud_host_is_refused` and `an_erase_on_a_cloud_host_is_refused`, exit 1 with `NO_FORGE_TOKEN`'s words;
  - `the_binary_refuses_a_repository_no_drive_serves`, exit 1.

  None of them reaches GitHub. The existing `the_command_line_is_well_formed` passes. These prove the forge-tokens spec's "Any other operation is a usage error", "A host with no App, or a configuration that cannot be read, must be fixed" and "On a cloud host forge-token refuses until the protocol carries a token". True when `devenv shell -- cargo test -p boomdrive --test forge_tokens -- a_store_is_a_usage_error no_forge_must_be_fixed a_configuration_it_cannot_read_must_be_fixed a_get_on_a_cloud_host_is_refused an_erase_on_a_cloud_host_is_refused the_binary_refuses_a_repository_no_drive_serves` passes.

## 4. The runner's own fetch

- [x] 4.1 `git.rs` gains `Credential`, design Decision 5's `Host`, `Token(String)` and `Nothing`, whose `Debug` never shows a token. It gains `TOKEN_HELPER`, Decision 5's helper. `Git::call_remote(at, remote, args, &Credential)` gives:
  - for `Host`, no argument naming a credential;
  - for `Token`, `-c credential.helper=`, then `-c credential.helper=<TOKEN_HELPER>`, and the token in the call's environment as `BOOMDRIVE_FORGE_TOKEN`;
  - for `Nothing`, `-c credential.helper=` alone.

  `clone::fetch` passes `&Credential::Host` for now. The module's documentation says what a call to a remote is given, citing record 0007 as amended on 2026-11-11 and core.34. Unit tests prove the clones spec's "Git is given the App's token alone" and "Git is given no credential":
  - `a_call_given_a_token_gives_git_that_token_alone` runs `git credential fill` through the call's own command, under a scratch global configuration whose helper answers `password=host`. It asserts git is given `x-access-token` and the token, never `host`, and that no argument holds the token;
  - `a_call_given_nothing_gives_git_no_credential` runs the same, and asserts git is given no password and fails without asking;
  - `a_credentials_debug_holds_no_token`.

  The existing `git_asks_no_one` holds for `Host`. True when `devenv shell -- cargo test -p boomdrive --lib git::` passes.
- [x] 4.2 `Forge` keeps the runner's own tokens, as design Decision 3 has it. It holds a `Mutex` of each address's kept token or `NotInstalled` answer, with the `ForgeApp` they were minted for, and has two methods:
  - `Forge::credential(Option<&ForgeApp>, &Address) -> Asked`. It gives `Credential::Host` where no App is named. It gives the kept token while it ends more than `RENEW_BEFORE` from now, else mints and keeps. It gives `Credential::Nothing` for a kept or new `NotInstalled`, kept for fifty-five minutes. It gives `Credential::Nothing` with the mint's words for a refused mint, kept not at all. A `ForgeApp` other than the one held empties what is kept first;
  - `Forge::drop(&Address)`.

  `tests/forge_tokens.rs` gains, over the fake: `a_token_is_given_again_until_five_minutes_before_its_end`, `a_token_near_its_end_is_bought_again` (an `expires_at` four minutes on), `a_dropped_token_is_bought_again`, `another_app_drops_every_token` (a second key, its JWT verified by its own public half) and `the_apps_absence_is_kept` (one call to find the installation and none to buy a token for two asks). True when `devenv shell -- cargo test -p boomdrive --test forge_tokens -- a_token_is_given_again_until_five_minutes_before_its_end a_token_near_its_end_is_bought_again a_dropped_token_is_bought_again another_app_drops_every_token the_apps_absence_is_kept` passes.
- [x] 4.3 `clone::fetch(git, bare, &Credential)` and `clone::make(git, data, address, &Credential)` pass the credential to their fetch. `pass::pass` and `pass::keep_one` take `&Forge` after their git. `keep_configured` asks `forge.credential(config.forge.as_ref(), address)` and hands what it gives to `keep`. In `keep`:
  - a fetch or a clone that fails drops the address's kept token by `forge.drop`;
  - its line gives git's words, then `; no forge token: ` and the mint's words where the mint was refused.

  `tests/common/mod.rs` gains `Scratch::pass_with_forge(text, &Forge)`, and `pass_with` makes its pass with `Forge::new()`, whose configuration names no `[forge]`. `tests/clones.rs` gains:
  - `a_repository_the_app_is_not_installed_on_is_fetched`;
  - `a_mint_refused_is_named_after_gits_words`: a `key_file` that does not exist, and an origin removed so the fetch fails;
  - `two_passes_buy_one_token`;
  - `a_failed_fetch_drops_its_token`.

  These prove the clones spec's "A repository the App is not installed on" and "A mint refused is named after git's words", and the forge-tokens spec's "A token kept across passes" and "A failed fetch drops its token". Every other scenario of the clones spec still holds, "Git asks no one" and "The host rewrites https to ssh" among them (design Decision 5). True when `devenv shell -- cargo test -p boomdrive --test clones` passes.
- [x] 4.4 `Runner` gains `forge: Forge`, made by `Forge::new()` once when it starts. The loop's `pass::pass` and the landing's `pass::keep_one` are given it, so a token is kept across passes. Every other caller of `pass::pass`, `pass::keep_one`, `clone::fetch` and `clone::make`, in the sources and the tests, passes what it now takes (design Decisions 3 and 5). True when `sluice exec main -- moon run boomdrive:typecheck` passes.

## 5. The modules

- [ ] 5.1 `nix/modules/options.nix` declares `settings.forge`, unset by default and rendered only where set, as a table of `app_id`, `types.ints.positive`, and `key_file`, `types.str`. Each option's description says what it holds, the key file outside the Nix store and readable by its owner alone, citing record 0007 as amended on 2026-11-11. An assertion fails evaluation where `key_file` begins with `builtins.storeDir`, with design Decision 7's words. `nix/tests/settings.nix`'s crew value gains `forge = { app_id = 1; key_file = "~/.config/boomdrive/app.pem"; }`, and `nix/tests/files/crew.toml` is rendered again from it. `nix/tests/file.nix` gains three cases:
  - the crew value renders `[forge]` with `app_id = 1` and its `key_file`, which the credential guard passes;
  - a `key_file` of `"/nix/store/abc-app.pem"` fails evaluation, naming `services.boomdrive.settings.forge.key_file` and the Nix store;
  - a `key_file` given as a Nix path value fails evaluation.

  These prove the service-modules spec's "The App's key is named by a path outside the store" (core.46; design Decision 7). True when `sluice exec main -- moon run nix:test` passes.
- [ ] 5.2 `tests/module_files.rs` reads `crew.toml`'s App: `app_id` 1, and `key_file` at `.config/boomdrive/app.pem` under the scratch home. This proves the service-modules scenario "The App named". True when `devenv shell -- cargo test -p boomdrive --test module_files` passes.

## 6. What is said

- [ ] 6.1 README's configuration block, the first ```toml block, shows `[forge]` with `app_id` and `key_file`, each with a comment line as its neighbours have. Its words say:
  - what the table holds;
  - that `key_file` is a path outside the Nix store, readable by its owner alone, and is read at each mint;
  - that the table is refused beside a hosted control plane;
  - that the local layer may not set it;
  - that the file still holds no credential, only where the key is.

  README's pass says a fetch goes through the App's token alone where `[forge]` is named, with no credential where the App is not installed or no token could be minted, and through the host's own helper where `[forge]` is not named. README gains the section `## Giving every tool its forge token` after "Running on a host", as design Decision 8 has it. `tests/readme.rs` asserts that README's configuration reads the App it shows. True when `devenv shell -- cargo test -p boomdrive --test readme` passes.
- [ ] 6.2 CLAUDE.md's first paragraph gains one sentence: where the configuration's `[forge]` names the Boomdrive App's key, `boomdrive forge-token` gives each tool a short-lived token for a served repository in git's credential-helper protocol, and the runner's own fetches take one the same way (record 0007 as amended; core.33). True when `sluice exec main -- moon run root:format` passes.

## 7. The specs

- [ ] 7.1 The change's deltas archive into specs that pass openspec's strict check. True when, after `devenv shell -- openspec archive the-runner-gives-every-tool-its-forge-token --yes`, `sluice exec main -- moon run root:specs` passes.
