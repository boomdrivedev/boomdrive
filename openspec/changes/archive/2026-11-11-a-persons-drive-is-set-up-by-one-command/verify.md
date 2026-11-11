# Verify: a-persons-drive-is-set-up-by-one-command

The work on `change/a-persons-drive-is-set-up-by-one-command` was read at 3f08075. The gate passes, the work meets every requirement and scenario of its change, and every task is ticked, so the change stays archived with this report.

- The gate ran once through sluice against `main` at 678ccfc. Every `check-fast` task 3f08075 affects passed, with 768 tests passed and none failed.
- The change's deltas add or modify 26 requirements and 49 scenarios: 24 requirements and 46 scenarios in the new `setup` spec, and 2 requirements and 3 scenarios in `family-packages`. A test asserts the outcome of each scenario.
- All 19 tasks are ticked. The code stage archived the change at 3f08075, as task 9.1 says to. This report sits beside the archived change.
- `bolt/one-command-sets-up-a-persons-drive` is at 57b1d1e, the branch's base. `main` is at 678ccfc, three commits past it, which touch only `devenv.nix` and record 0022. The branch merges into each with no conflict.

## Checks

- **The gate.** `sluice exec main` at 3f08075 ran the `check-fast` tasks that 3f08075 affects against `main` at 678ccfc. It exited 0, and moon completed 16 tasks, each passed:
  - `boomdrive:format`, `lint`, `typecheck` and `test`;
  - `boomdrive-crew:format`, `lint`, `typecheck` and `test`;
  - `nix:format`, `lint`, `typecheck` and `test`, each "every case holds";
  - `root:format`, with 332 files and none changed, `adrs`, with 41 records, `specs`, with 24 of 24, and `changes`, with no open change.
- **What the gate did not run.** sluice ran no task the commit does not affect: `boomdrive-control`'s, `boomdrive-hosted`'s, `boomdrive-bounded`'s, the site's, and `root:toolchain`, `commits`, `releases` and `workflow`.
- **The tests.** The 768 tests include every test the tasks name and every test the table below names. Each passed, `crates/boomdrive/tests/setup.rs` on macOS among them.
- **Not recorded.** sluice did not record the results under 3f08075's key. This report's draft was in the worktree, untracked, while the checks ran, and sluice said so. The draft is this file alone, which no check reads but `root:format`. The pass is as above, and the gate was not run again.
- **Warnings.** moon warned that it could not write its shared cache or its output archives, could not hash a few folders and missing incremental files, and had no internet connection for a dependency install. None of these touched a task's result.
- The gate was run once, as this stage's brief says, and nothing was fixed here.

## Tasks

All 19 tasks of `tasks.md`, groups 1 to 9, are ticked `[x]`, and none is open. What they build is in the tree, as the design names it:

- **The set's herdr plugins in one package** (1.1, 1.2):
  - `nix/packages/herdr-plugins.nix` is a `linkFarm` named `herdr-plugins` with the runner's version. It holds `share/herdr/plugins/flume` and `share/herdr/plugins/sluice` as the two plugin packages, and its comment cites record 0009's amendment of 2026-11-11 and the `bin/open.sh` clash.
  - `flake.nix` gives it, and `nix/tests/typecheck.nix` names it.
  - `nix/tests/packages.nix`'s "The set's herdr plugins in one package" holds its entries to exactly the two plugins, each that system's, with no `bin`.
- **The file setup's file is held to** (2.1, 2.2):
  - `nix/tests/settings.nix` gains `setup`, and `nix/tests/files/setup.toml` is its rendering: the account `own`'s drive `home` with `agentplot/flume` and `agentplot/sluice`, under crew with `host-a` and the partition `home` at `example/state`.
  - `module_files.rs` lists `setup.toml`, and `the_setup_file_keeps_one_drive_under_crew` reads it as the runner reads it.
- **crew's implementation sets up a drive** (3.1 to 3.4):
  - `crates/boomdrive-crew/src/setup.rs` holds `ROLES` in crew's order and `Asked`.
    - `held` reads the state repository from crew's table and the blueprints, runs and review from crew's file at version 6.
    - `settle` fills each answer from the flags, then the host, then setup's own: the first repository, `standing`, `5 4`. It refuses each of the spec's forms before any program runs.
  - `drive_set` gives `crew drive set <drive> --json` and the flags in the spec's order. It never gives `--flume`, `--roles` or `--console`.
  - `prepare` finds `hostname`, `gh` and `crew`, naming `github:agentplot/boomdrive#crew` for crew. It refuses a host name crew does not take, naming `sudo scutil --set HostName`.
  - `set_up` reads the state repository by `gh repo view` and makes it private only on gh's `Could not resolve to a Repository`. It then runs crew. A refusal from either is the tool's with its words, exit 1. It gives crew's table with the drive's partition set and the others kept in order.
  - `planes::Entry` gains `set_up` for crew's entry alone. `rules.rs`'s `only_crews_crate_names_crew` asserts `planes.rs` names `boomdrive_crew::setup`.
- **The command and its answers** (4.1):
  - `main.rs`'s `Setup` takes the nine flags and `--json`, with `--repository` conflicting with the two changes.
  - `setup::answers` gathers from the flags, then setup's own file where it names one drive, then a terminal, in the spec's order. It refuses each usage error the task names.
- **The host** (5.1 to 5.3):
  - `check_host` refuses a system other than macOS, a linked runner file (as Nix's, exit 1), a missing `gh`, `herdr` or `launchctl`, and both files there, before any program runs.
  - The binary's tests run `setup` over stand-in programs on a scratch home, `#![cfg(target_os = "macos")]`.
  - `sign_ins` reads Claude Code's `loggedIn` and Pending You's `Sign-in:` line, naming each missing one with its fix and going on.
- **What setup writes** (6.1 to 6.4):
  - `runner_file` renders the managed layer with setup's first line, repositories owner by owner, keys before tables, each sorted, and TOML's basic strings. Past its first line it is `nix/tests/files/setup.toml` byte for byte.
  - The file is moved aside where it is a person's, written by a renamed temporary only where its bytes differ, and read back by `config::read_layers`.
  - `plugins` links each folder of the profile's `share/herdr/plugins` that herdr's list does not hold. It runs herdr with every `HERDR_` variable removed and standard input closed. It names the plugins missing with `github:agentplot/boomdrive#herdr-plugins` where no profile holds them.
  - `agent` renders the plist as the spec's "The agent" names it.
  - `load_agent` writes it only where it changed. It bootstraps where `launchctl print` finds none, boots out first where the plist changed and one is loaded, and otherwise runs neither.
- **The answer** (7.1): `{"set_up": …}` holds `account`, `drive`, `repositories`, `control_plane`, `config`, `service`, `plugins` and `missing`. A refusal is `{"refused":{"refusal":{"reason":…}}}`. Without `--json`, one line a step ends `start: crew main up <drive>`. Exits are 0, 1 and 2 as the spec says.
- **README** (8.1):
  - The install line adds `#herdr` and `#herdr-plugins` and says why the plugins come as one package.
  - "Setting up a drive on a Mac Nix does not manage" covers every point the task lists.
  - `the_readme_names_how_a_persons_drive_is_set_up` holds it.
- **The specs** (9.1): the deltas are archived into `openspec/specs/setup`, new with its Purpose, and `openspec/specs/family-packages`.

Record 0009's amendments of 2026-11-11, "how setup is built" and "what the build of setup decided", are in the tree. So are core.29's text and core.5's, core.16's, core.18's, core.28's and core.29's details, as the plan wrote them.

Against the branch's base, 57b1d1e, the branch changes only these paths:

- the files the proposal's Touches lists;
- `crates/boomdrive/tests/rules.rs`, which task 3.4 names;
- the change's own files.

No configuration key, `report.json` field, protocol message, tool, tracked file type or crate is added.

## Requirements and scenarios

| spec | requirement | scenario | asserted by |
| --- | --- | --- | --- |
| setup | Setup takes each answer as a flag | Every answer given | `tests/setup.rs` `every_answer_given_asks_nothing` |
| setup | Setup takes each answer as a flag | The whole list beside a change to it | `setup::answers` `the_whole_list_beside_a_change_to_it` |
| setup | A terminal is asked what nothing else gives | An answer missing with no terminal | `setup::answers` `an_answer_missing_with_no_terminal_names_each_flag` |
| setup | A terminal is asked what nothing else gives | Asked on a terminal | `setup::answers` `a_terminal_is_asked_in_order` |
| setup | An answer not given is taken from the host | A later setup keeps what crew's file holds | crew `setup` `a_later_setup_keeps_what_crews_file_holds` |
| setup | An answer not given is taken from the host | One role changed | crew `setup` `one_role_changed` |
| setup | An answer not given is taken from the host | Two drives and no drive named | `setup::answers` `two_drives_and_no_drive_named` |
| setup | Setup's own answers stand where the host holds none | The first setup's own answers | crew `setup` `the_first_setups_own_answers` |
| setup | Each answer is checked before any program runs | A blueprints repository the drive does not keep | crew `setup` `a_blueprints_repository_the_drive_does_not_keep` |
| setup | Each answer is checked before any program runs | A role crew does not have | crew `setup` `a_role_crew_does_not_have` |
| setup | Each answer is checked before any program runs | A review of nothing | crew `setup` `a_review_of_nothing` |
| setup | A drive's repositories are added and removed by setup | A repository added | `tests/setup.rs` `a_repository_added_is_kept_and_given_crew` |
| setup | A drive's repositories are added and removed by setup | A repository removed | `tests/setup.rs` `a_repository_removed_leaves_both` |
| setup | A drive's repositories are added and removed by setup | A repository added twice | `setup::answers` `a_repository_added_twice` |
| setup | A drive's repositories are added and removed by setup | The last repository removed | `setup::answers` `the_last_repository_removed` |
| setup | Setup runs on macOS alone | On Linux | `setup::check_host` `on_linux_setup_is_refused` |
| setup | On a host Nix manages, setup writes nothing and says so | The runner's file is Nix's | `tests/setup.rs` `the_runners_file_is_nixs` |
| setup | Each program setup runs is found before it runs one | crew missing | `tests/setup.rs` `crew_missing` |
| setup | Each program setup runs is found before it runs one | A host name crew refuses | crew `tests/setup.rs` `a_host_name_crew_refuses` |
| setup | A person's own file is moved aside once and named | A person's own file | `tests/setup.rs` `a_persons_own_file_is_moved_aside` |
| setup | A person's own file is moved aside once and named | Both files there | `tests/setup.rs` `both_files_there`; `setup::check_host` `both_files_there_are_named` |
| setup | GitHub's sign-in is needed, and the others are named | GitHub not signed in | `tests/setup.rs` `github_not_signed_in` |
| setup | GitHub's sign-in is needed, and the others are named | Claude Code not signed in | `tests/setup.rs` `claude_code_not_signed_in` |
| setup | GitHub's sign-in is needed, and the others are named | Pending You not installed | `tests/setup.rs` `pending_you_not_installed` |
| setup | The state repository is made where none exists | No state repository | crew `tests/setup.rs` `no_state_repository_is_made_private` |
| setup | The state repository is made where none exists | The state repository there | crew `tests/setup.rs` `a_state_repository_there_is_left` |
| setup | The state repository is made where none exists | The forge refuses | crew `tests/setup.rs` `the_forges_refusal_is_carried` |
| setup | crew names the drive by its own command | crew's flags | crew `setup::drive_set` `crews_flags_are_the_drives_answers`; `tests/setup.rs` `crews_flags_through_setup` |
| setup | crew is never given --flume, and its answer is carried | No --flume for any role | crew `setup::drive_set` `no_flume_is_ever_passed` |
| setup | crew is never given --flume, and its answer is carried | crew's answer carried | crew `tests/setup.rs` `crews_answer_is_carried_as_crew_printed_it` |
| setup | crew is never given --flume, and its answer is carried | crew refuses | crew `tests/setup.rs` `crews_refusal_is_carried`; `tests/setup.rs` `crews_refusal_writes_no_runner_file` |
| setup | The runner's file holds the keys Nix writes | One drive under crew | `tests/setup.rs` `one_drive_under_crew_is_the_modules_file` |
| setup | The runner's file holds the keys Nix writes | A second drive | `tests/setup.rs` `a_second_drive_keeps_the_first`; crew `tests/setup.rs` `the_table_names_each_drive_set_up` |
| setup | Setup knows its own file and writes it only where it changed | Nothing changed | `tests/setup.rs` `nothing_changed_writes_nothing` |
| setup | Setup knows its own file and writes it only where it changed | A local layer beside it | `tests/setup.rs` `a_local_layer_beside_it_is_left` |
| setup | herdr's plugins are linked from the person's profile | Both plugins linked | `tests/setup.rs` `both_plugins_linked` |
| setup | herdr's plugins are linked from the person's profile | A plugin already linked | `tests/setup.rs` `a_plugin_already_linked_is_left` |
| setup | A plugin missing or refused is named, and setup goes on | No plugin in the profile | `tests/setup.rs` `no_plugin_in_the_profile` |
| setup | The runner runs as a launchd agent of the login | The agent | `setup` `the_agent_is_nix_darwins_under_setups_label` |
| setup | The agent is loaded, and loaded again only where it changed | First loaded | `tests/setup.rs` `first_loaded` |
| setup | The agent is loaded, and loaded again only where it changed | Changed and loaded | `tests/setup.rs` `a_changed_plist_is_booted_out_first` |
| setup | The agent is loaded, and loaded again only where it changed | Loaded and unchanged | `tests/setup.rs` `loaded_and_unchanged` |
| setup | Setup answers in one object | The answer | `tests/setup.rs` `the_answer` |
| setup | Setup's exit says how it ended | A refusal in JSON | `tests/setup.rs` `a_refusal_in_json` |
| setup | Run again, setup changes nothing | Run again | `tests/setup.rs` `run_again_changes_nothing` |
| setup | Only crew's implementation sets up a drive | Only crew's crate names crew | `tests/rules.rs` `only_crews_crate_names_crew` |
| family-packages | The flake gives the family's host tools as packages | Every system's packages | `nix/tests/typecheck.nix` "Every system's packages", whose set names `herdr-plugins`, `nix:typecheck` |
| family-packages | The flake gives the family's host tools as packages | No tool of the host's own | `nix/tests/typecheck.nix` "No tool of the host's own", `nix:typecheck` |
| family-packages | The set's herdr plugins are one package a profile holds | The plugins in one package | `nix/tests/packages.nix` "The set's herdr plugins in one package", `nix:test` |

Where the table names `tests/setup.rs` alone, it is the binary's, `crates/boomdrive/tests/setup.rs`. A test named by its module is a unit test of `crates/boomdrive/src/setup.rs` or, with "crew", of `crates/boomdrive-crew/src/setup.rs`.

## Notes

- **Repositories are ordered owner by owner.** The setup spec says `--add-repository` adds each "after the drive's own". Setup orders a drive's repositories owner by owner before crew is given them and the runner's file is written, as Nix's module orders them. So a repository whose owner sorts first lands first, and the default blueprints repository is the first after that order. Record 0009's amendment "what the build of setup decided" takes this, so a run again gives crew the same flags. Every scenario holds, since each adds a repository of the same owner. Nothing here falls short of the change.
- **crew's pin.** Setup runs `crew drive set` with crew v0.59.0's flags. The set's lock still pins crew at v0.50.2, which has no `drive set`. The proposal and design Risk 1 leave this to the bolt `the-sets-crew-pin-takes-its-new-names`. Until that lands, setup on a host with the set's crew is refused by crew, and setup carries crew's words. Every test of this change runs a stand-in `crew`.
- **Flume's file.** Setup writes none of Flume's files, as design Risk 2 and README say. Record 0009's amendment names what Flume is asked for.
