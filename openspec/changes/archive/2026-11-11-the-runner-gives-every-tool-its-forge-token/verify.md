# Verify: the-runner-gives-every-tool-its-forge-token

The work on `change/the-runner-gives-every-tool-its-forge-token` was read at 3ef0f11. The gate passes. The work meets every requirement and scenario of its change, and every task is ticked, so the change stays archived with this report.

- The gate ran once through sluice against `main` at 678ccfc. Every `check-fast` task that 3ef0f11 affects passed: 28 tasks and 987 tests, none failed.
- The change's deltas hold 21 requirements and 46 scenarios. Of these, 2 requirements and 6 scenarios were already in the specs and still hold. A test that passed in the gate asserts the outcome of each scenario.
- All 15 tasks are ticked. The code stage archived the change at 3ef0f11, as task 7.1 says to. This report sits beside the archived change.
- The branch's base is 57b1d1e9, which `origin/bolt/the-runner-gives-every-tool-its-forge-token` is at. `main` has moved three commits past it, to 678ccfc. A trial merge of the branch into `main` has no conflict.

## Checks

- **The gate.** `sluice exec main` at 3ef0f11 ran the `check-fast` tasks that 3ef0f11 affects against `main` at 678ccfc. It exited 0, and sluice recorded 28 results under their keys for 3ef0f11.
  - Passed:
    - `boomdrive`, `boomdrive-bounded`, `boomdrive-control`, `boomdrive-crew` and `boomdrive-hosted`: `format`, `lint`, `typecheck` and `test` each;
    - `nix:format`, `lint`, `typecheck` and `test`, the last saying "every case holds";
    - `root:format` (331 files, none changed), `adrs` (41 records), `specs` (24 of 24, `forge-tokens` among them) and `changes` (no open change).
  - Not run, since 3ef0f11 does not affect them:
    - the `site` project's tasks;
    - `root:toolchain`, `commits`, `releases` and `workflow`.

    Each of the branch's 16 commits since `main` has a Conventional Commits subject (`feat`, `test` or `docs`, each with a scope).
  - The 987 tests include every test the tasks name. Each one is reported `ok` in the gate's output.
  - moon warned that it could not write its shared cache or its output archives outside the sandbox. It also had no internet connection for a dependency install. devenv named `.mcp.json` and `.claude/settings.json` as conflicting files. Neither file is tracked. None of these warnings touched a task's result.
- The gate was run once, as this stage's brief says, and nothing was fixed here.

## Tasks

All 15 tasks of `tasks.md`, groups 1 to 7, are ticked `[x]`, and none is open. What they build is in the tree, as the design names it:

- **The App in the configuration** (1.1, 1.2):
  - `boomdrive_hosted::NO_FORGE_TOKEN` holds the words the task gives.
  - `planes::Entry::holds_the_app` is `None` for crew's entry and `Some(NO_FORGE_TOKEN)` for the hosted one.
  - `ForgeApp { app_id: u64, key_file: PathBuf }` and `Config::forge` are read from the document's tree. `[forge]` is refused, naming the key's path and line and never its value, in each of these cases:
    - it is not a table;
    - it lacks `app_id` or `key_file`;
    - it holds any other key;
    - `app_id` is no positive integer;
    - `key_file` is no string, is neither absolute nor under `~/`, or lies under `/nix/store`;
    - it stands beside a control plane whose entry holds the App.
  - The key file is not read with the configuration. The local layer that sets `[forge]` is not taken, naming `forge`.
- **The mint** (2.1, 2.2):
  - `forge.rs` has `CALL_FOR` (10 s), `RENEW_BEFORE` (5 min) and `Refused`.
  - `Token`'s `Debug` names its end alone.
  - The key is refused as a host to fix where it cannot be read, where its mode has any bit of `0o077`, or where it holds no RSA PEM.
  - The JWT is RS256, with `iss` the App's id, `iat` a minute back and `exp` nine minutes on.
  - A mint is two `ureq` calls, each with the global timeout at the forge's limit and GitHub's `Accept`, `X-GitHub-Api-Version` and `User-Agent` headers:
    - `GET /repos/{owner}/{name}/installation`, whose 404 gives `Minted::NotInstalled`;
    - `POST /app/installations/{id}/access_tokens`, with `{"repositories":["<name>"]}`.
  - Every other answer names its call, its status and GitHub's message. A call that cannot be made names its call.
  - `tests/github/mod.rs` is the loopback fake of Decision 9. It verifies each JWT against the public half of the test binary's key, which is made at run time. No key is committed.
- **`boomdrive forge-token`** (3.1, 3.2):
  - `forge_token::answer` takes Decision 4's steps in order:
    - the control plane's `holds_the_app`;
    - no `[forge]` as a host to fix;
    - git's request read to a blank line;
    - `erase` answering nothing;
    - `get` refusing another protocol or host, no path, no address, an unserved repository or `NotInstalled`, each with Decision 4's words;
    - the three lines, and nothing else, on output.
  - `main.rs` adds `ForgeToken { config, operation }`. `with_usage` gives clap's invalid-value error the subcommand's usage, so `store` prints usage and exits 2. A refusal is `boomdrive forge-token: <words>` on standard error, exiting 1, or 2 for a host to fix.
- **The runner's own fetch** (4.1 to 4.4):
  - `git::Credential` gives `Host` (no argument), `Token` (`-c credential.helper=` then `TOKEN_HELPER`, the token in `BOOMDRIVE_FORGE_TOKEN` alone) and `Nothing` (`-c credential.helper=` alone). Its `Debug` shows no token, and the variable is removed from every other remote call.
  - `Forge::credential` and `Forge::drop` keep each address's token, or its `NotInstalled` for 55 minutes, for the App they were minted for. They hold no lock while GitHub is called, and keep nothing of a refused mint.
  - `clone::fetch` and `clone::make` take the credential.
  - `pass::keep` drops the kept token on a failed fetch or clone, and appends `; no forge token: <words>` where the mint was refused.
  - `Runner` holds one `Forge` for its passes and its landings' `keep_one`.
- **The modules** (5.1, 5.2):
  - `settings.forge` is a submodule of `app_id` (`types.ints.positive`) and `key_file` (`types.str`), unset by default.
  - An assertion refuses a `key_file` under `builtins.storeDir` or `/nix/store`, with Decision 7's words.
  - `nix/tests/file.nix` holds "The App named", "A key file in the store" and "A key given as a Nix path".
  - `crew.toml` is rendered with `[forge]`, and `module_files.rs`'s `the_crew_file_names_the_app` reads it.
- **What is said** (6.1, 6.2):
  - README's configuration block shows `[forge]`, and its words say what task 6.1 lists.
  - README's pass names the three credentials. It gains `## Giving every tool its forge token` after "Running on a host".
  - `readme.rs`'s `the_readme_configuration_reads` reads the App.
  - CLAUDE.md's first paragraph gains the sentence task 6.2 gives.
- **The specs** (7.1): the deltas are archived into `openspec/specs/clones`, `configuration` and `service-modules`, and the new `forge-tokens`. `root:specs` passes 24 of 24.

Record 0007's amendment of 2026-11-11 and the amended details of core.28, core.33 and core.39 are in the tree as the plan names them. The proposal's departure from the brief is recorded there with the paths not taken. The brief says `get` "gives the one it keeps" and `erase` "drops that repository's token". Instead, each `get` mints in its own process, and `erase` drops nothing, since no process keeps a token it gave.

Against `main`, the branch changes these paths:

- the files the proposal's Touches lists;
- the change's own files;
- four files that task 4.4 sends every caller to: `src/carry.rs`'s test configuration (`forge: None`), and `tests/code_workspaces.rs`, `tests/commands.rs` and `tests/inboxes.rs`, each passing the forge or `Credential::Host` it now takes. The tasks' preamble keeps changes to Touches. Task 4.4 names every caller in the sources and the tests, and these four changes are those callers and nothing more.

The archive commit reshaped two delta specs before archiving:

- the clones delta's one modified requirement became the modified "A clone is fetched over https and never asks", with its two existing scenarios, and two added requirements that hold the new scenarios with the same words;
- the forge-tokens delta gained its `# Forge tokens` title and its Purpose.

No requirement or scenario was dropped or reworded in substance.

No tool, tracked file type, `report.json` field or protocol message is added. The one configuration key added is `[forge]`.

## Requirements and scenarios

| spec | requirement | scenario | asserted by |
|---|---|---|---|
| clones | A clone is fetched over https and never asks | Git asks no one | `git::tests` `git_asks_no_one`, `Host` adding no argument |
| | | The host rewrites https to ssh | `clones.rs` `the_host_rewrites_https_to_ssh` |
| | A fetch goes through the App's token where the configuration names the App | Git is given the App's token alone | `git::tests` `a_call_given_a_token_gives_git_that_token_alone` |
| | A fetch with no token goes with no credential | Git is given no credential | `git::tests` `a_call_given_nothing_gives_git_no_credential` |
| | | A repository the App is not installed on | `clones.rs` `a_repository_the_app_is_not_installed_on_is_fetched` |
| | | A mint refused is named after git's words | `clones.rs` `a_mint_refused_is_named_after_gits_words` |
| configuration | The configuration holds nothing the runner does not read | A misspelt key; A token; A key in a drive; A key in an account | `config::tests` `a_misspelt_key`, `a_token`, `a_key_in_a_drive`, `a_key_in_an_account` |
| | | A key's text in the forge table | `config::tests` `a_keys_text_in_the_forge_table` |
| | The configuration may name the Boomdrive App | The App named | `config::tests` `the_app_named` |
| | | No App named | `config::tests` `no_app_named` |
| | | A key file missing from the table | `config::tests` `a_key_file_missing_from_the_forge_table` |
| | The App's key file is a path outside the store | A key file in the store | `config::tests` `a_key_file_in_the_store` |
| | | A relative key file | `config::tests` `a_relative_key_file` |
| | The App is named only in the managed layer of a host that holds it | The App beside a hosted control plane | `config::tests` `the_app_beside_a_hosted_control_plane` |
| | | The App in the local layer | `config::tests` `the_app_in_the_local_layer` |
| forge-tokens | The runner mints a token for one repository from the App's key | A token for one repository | `forge_tokens.rs` `a_token_is_bought_for_its_repository_alone` |
| | | The App is not installed | `forge_tokens.rs` `the_app_not_installed_is_its_own_answer` |
| | A key the runner cannot use refuses the mint | A key others may read | `forge_tokens.rs` `a_key_others_may_read_calls_no_one`; `forge::tests` `a_key_others_may_read_is_refused` |
| | | A key file that is no key | `forge_tokens.rs` `a_key_file_that_is_no_key_calls_no_one`; `forge::tests` `a_key_file_that_is_no_key_is_refused` |
| | GitHub's refusal and silence are named, and hold no secret | GitHub refuses the App | `forge_tokens.rs` `a_refused_mint_names_github_and_holds_no_secret` |
| | | GitHub's API never answers | `forge_tokens.rs` `a_silent_api_is_stopped_at_its_limit` |
| | `get` gives a served repository's token on standard output alone | A served repository | `forge_tokens.rs` `a_get_gives_a_served_repositorys_token_on_standard_output_alone` |
| | | A repository the local layer adds | `forge_tokens.rs` `a_repository_the_local_layer_adds_is_served` |
| | | Each get mints | `forge_tokens.rs` `each_get_mints` |
| | `get` refuses what it may not give | A repository no drive serves | `forge_tokens.rs` `a_repository_no_drive_serves_is_refused`, `the_binary_refuses_a_repository_no_drive_serves` |
| | | Another host | `forge_tokens.rs` `another_host_is_refused` |
| | | No path | `forge_tokens.rs` `a_request_with_no_path_is_refused` |
| | | The App is not installed on the repository | `forge_tokens.rs` `a_repository_the_app_is_not_installed_on_is_refused` |
| | `erase` revokes nothing, and the next get never gives the erased token | A token erased and asked for again | `forge_tokens.rs` `an_erase_revokes_nothing_and_the_next_get_mints_afresh` |
| | Any other operation is a usage error | git asks it to store | `forge_tokens.rs` `a_store_is_a_usage_error` |
| | A host with no App, or a configuration that cannot be read, must be fixed | No [forge] | `forge_tokens.rs` `no_forge_must_be_fixed` |
| | | A configuration it cannot read | `forge_tokens.rs` `a_configuration_it_cannot_read_must_be_fixed` |
| | On a cloud host forge-token refuses until the protocol carries a token | A get on a cloud host | `forge_tokens.rs` `a_get_on_a_cloud_host_is_refused`; `planes::tests` `only_the_hosted_control_plane_holds_the_app` |
| | | An erase on a cloud host | `forge_tokens.rs` `an_erase_on_a_cloud_host_is_refused` |
| | No token reaches a file, a log or error output | Nothing is left behind | `forge_tokens.rs` `nothing_is_left_behind`; `forge::tests` `a_tokens_debug_holds_no_secret`; `git::tests` `a_credentials_debug_holds_no_token` |
| | The running runner keeps one token for each repository for its own git calls | A token kept across passes | `clones.rs` `two_passes_buy_one_token`; `forge_tokens.rs` `a_token_is_given_again_until_five_minutes_before_its_end` |
| | | A token near its end | `forge_tokens.rs` `a_token_near_its_end_is_bought_again` |
| | A kept token is dropped when its fetch fails or its App changes | A failed fetch drops its token | `clones.rs` `a_failed_fetch_drops_its_token`; `forge_tokens.rs` `a_dropped_token_is_bought_again` |
| | | Another App | `forge_tokens.rs` `another_app_drops_every_token` |
| | GitHub's word that the App is not installed is kept as a token is | The App's absence kept | `forge_tokens.rs` `the_apps_absence_is_kept` |
| service-modules | The App's key is named by a path outside the store | The App named | `nix/tests/file.nix` "The App named"; `module_files.rs` `the_crew_file_names_the_app` |
| | | A key file in the store | `nix/tests/file.nix` "A key file in the store" |
| | | A key given as a Nix path | `nix/tests/file.nix` "A key given as a Nix path" |

The JWT's claims and signature are asserted by `forge::tests` `a_jwt_names_the_app_and_lasts_nine_minutes`. git's `path`, with a trailing `.git` or `/` taken off, is asserted by `forge_token::tests` `a_path_is_read_as_an_address`.

## Outside this repository

The design's Non-Goals and record 0007's amendment of 2026-11-11 name one piece of work outside this change. The set's sluice, v0.78.1, reads no `[forge] credential`. Moving sluice's tag to v0.84.0 or later, and having the Boomdrive module give sluice `boomdrive forge-token`, is a change of its own (core.45, core.57). Until then, a host keeps sluice's own `[forge] key`.
