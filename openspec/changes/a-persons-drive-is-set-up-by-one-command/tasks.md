# Tasks

Work through the tasks in order. A task is done once the check it names passes, run as it names it. The groups are headings for reading.

- **Names and paths.** Every test runs on a `Scratch` home, under core.41's neutral names: the account `own`, the drives `home` and `work`, the host `host-a`, the repositories `agentplot/flume`, `agentplot/sluice`, `agentplot/crew` and `example/app`, and the state repository `example/state`.
- **Stand-ins, never the host's programs.** No test runs the host's `crew`, `gh`, `herdr`, `hostname`, `launchctl`, `claude` or `pendingyou`. Each is a stand-in `sh` script in the scratch `bin`, which appends each call's arguments to a calls file and answers what the test has put beside it. Nothing reaches the network.
- **Where the binary's tests run.** The binary's own tests of setup run on macOS, `#![cfg(target_os = "macos")]`, where the gate runs. Each piece they cover that is not macOS's own also has a unit test.
- **The rules.** Record 0009's amendment of 2026-11-11, and core.5, core.16, core.18, core.28 and core.29, are written with this change's plan, before task 1.1. A task that relies on a choice the design names cites its Decision.
- **What stays as it is.** No task adds a configuration key, a field of `report.json`, a protocol message, a tool, a tracked file type or a crate. crew's flags are crew v0.59.0's (crew records 0064 and 0065).

## 1. The set's herdr plugins in one package

- [x] 1.1 `nix/packages/herdr-plugins.nix` is a `linkFarm` named `herdr-plugins`, version the runner's, holding `share/herdr/plugins/flume` as the `flume-herdr-plugin` package and `share/herdr/plugins/sluice` as the `sluice-herdr-plugin` package. Its comment cites record 0009's amendment of 2026-11-11 and says why: each plugin has `bin/open.sh` at its top, so one profile cannot hold the two. `flake.nix`'s `packagesOf` gives it as `herdr-plugins`. `nix/tests/typecheck.nix`'s set names it (the family-packages spec's "Every system's packages"; design Decision 8). Done when `sluice exec main -- moon run nix:typecheck` passes.
- [x] 1.2 `nix/tests/packages.nix` gains `"The set's herdr plugins in one package"`. For each system it holds `herdr-plugins`'s entries to exactly `share/herdr/plugins/flume` and `share/herdr/plugins/sluice`, each that system's plugin package, with no `bin` (the family-packages spec's "The plugins in one package"). Done when `sluice exec main -- moon run nix:test` passes.

## 2. The file setup's file is held to

- [x] 2.1 `nix/tests/settings.nix` gains `setup`:
  - `accounts.own.drives.home.repositories.agentplot = [ "flume" "sluice" ]`;
  - `settings.control_plane`, with `type = "crew"` and `crew` holding `host = "host-a"` and the one partition `home` with `state_repository = "example/state"`;
  - nothing else set.

  Its comment says it is the file setup writes for one account's drive (design Decision 6). `nix/tests/file.nix` holds its rendering to `nix/tests/files/setup.toml`, written as the module renders it. Done when `sluice exec main -- moon run nix:test` passes.
- [x] 2.2 `tests/module_files.rs`'s `each_rendered_file_is_one_read_here` lists `setup.toml`. A new test, `the_setup_file_keeps_one_drive_under_crew`, reads the file as the runner reads it and holds it to:
  - the account `own`'s drive `home` with `agentplot/flume` and `agentplot/sluice`;
  - `type` `crew`, with no other key of the runner's set;
  - a table crew's implementation reads: `host` `host-a` and the partition `home` with `example/state`.

  Done when `devenv shell -- cargo test -p boomdrive --test module_files` passes.

## 3. crew's implementation sets up a drive

- [x] 3.1 `crates/boomdrive-crew/src/setup.rs` holds `ROLES`, crew's five roles in crew's order, `design`, `planner`, `reviewer`, `ops` and `operator`, and `Asked`: the state repository, blueprints, runs and review as setup's flags gave them, as text.
  - **`held(table, teams_file, drive)`** reads, for the drive, the state repository from its partition in crew's table of the runner's file, and the blueprints, each role's runs and the review from the drive's entry in crew's file at version 6. A missing file, another version, or a drive the file does not name gives none of the file's.
  - **`settle(asked, held, repositories)`** fills each answer not asked from what is held, then from its own values: the first repository, `standing` for each role, `5` and `4`. A `--runs` changes its role alone. It refuses, as the host's to fix:
    - an address out of form;
    - a blueprints repository not among the repositories;
    - a role not of `ROLES`, or a way of running but `standing` or `on-demand`;
    - a review whose changes are not a whole number of at least 1, or whose hours are not a number above 0.

  `lib.rs` declares the module, its documentation citing record 0009's amendment of 2026-11-11 and design Decisions 2, 3 and 11. Its unit tests prove the setup spec's "The first setup's own answers", "A later setup keeps what crew's file holds", "One role changed", "A blueprints repository the drive does not keep", "A role crew does not have" and "A review of nothing". They are `the_first_setups_own_answers`, `a_later_setup_keeps_what_crews_file_holds`, `one_role_changed`, `a_blueprints_repository_the_drive_does_not_keep`, `a_role_crew_does_not_have` and `a_review_of_nothing`, with `held` reading a file the test writes. Done when `devenv shell -- cargo test -p boomdrive-crew --lib setup::` passes.
- [x] 3.2 `drive_set(settled, drive, account, host, folder, kept)` gives `crew drive set`'s arguments. They are `drive set <drive> --json`, then each flag in the order the setup spec's "crew names the drive by its own command" lists. `kept` is each repository with its clone's folder and its view, and no `--flume`, `--roles` or `--console` is ever given (design Decision 4). Its unit tests are:
  - `crews_flags_are_the_drives_answers`, which proves "crew's flags" with `/h` as the home;
  - `no_flume_is_ever_passed`, which proves "No --flume for any role" over every role run either way.

  Done when `devenv shell -- cargo test -p boomdrive-crew --lib setup::drive_set` passes.
- [x] 3.3 `prepare(find)` takes a finder of programs on the `PATH`. It finds `hostname`, `gh` and `crew`, refusing each it cannot find as the host's to fix, naming it and, for `crew`, `github:agentplot/boomdrive#crew`. It then runs `hostname -s`, refusing as the host's to fix a name crew does not take, naming it and `sudo scutil --set HostName`. `set_up(prepared, …)` runs, in order:
  1. `gh repo view <state> --json name`, then `gh repo create <state> --private` where gh says `Could not resolve to a Repository`;
  2. `crew drive set` with task 3.2's arguments.

  Where gh or crew exits other than 0, it refuses with the tool's words as the tool's, never the host's to fix. Otherwise it gives:
  - its table: the table of the runner's file with this drive's partition set, every other partition kept in its order, and `host`;
  - its answer: `state_repository`, with `repository` and `made` or `there`, `drive`, crew's JSON as crew printed it, and `start`, `crew main up <drive>`.

  `crates/boomdrive-crew/tests/setup.rs`, with stand-in `hostname`, `gh` and `crew`, proves the setup spec's "No state repository", "The state repository there", "The forge refuses", "crew's answer carried", "crew refuses" and "A host name crew refuses". It also proves that the table names a second drive beside the first. The tests are `no_state_repository_is_made_private`, `a_state_repository_there_is_left`, `the_forges_refusal_is_carried`, `crews_answer_is_carried_as_crew_printed_it`, `crews_refusal_is_carried`, `a_host_name_crew_refuses` and `the_table_names_each_drive_set_up` (design Decisions 4, 5 and 11). Done when `devenv shell -- cargo test -p boomdrive-crew --test setup` passes.
- [x] 3.4 `planes::Entry` gains `set_up`, crew's entry wrapping `boomdrive_crew::setup`'s `held`, `settle`, `prepare` and `set_up`, and the hosted entry none. `planes::set_up_entry()` gives the one entry that has it. A refusal carries `host_to_fix`, as `planes::Refusal` does. `tests/rules.rs`'s `only_crews_crate_names_crew` also asserts that `planes.rs` names `boomdrive_crew::setup`. That proves the setup spec's "Only crew's crate names crew" (core.2; design Decision 11). Done when `devenv shell -- cargo test -p boomdrive --test rules -- only_crews_crate_names_crew` passes.

## 4. The command and its answers

- [x] 4.1 `src/main.rs` gains `Setup`, with the setup spec's nine flags and `--json`:
  - `--runs` takes two values and may be given again;
  - `--review` takes two values;
  - `--repository` conflicts with `--add-repository` and `--remove-repository`.

  `src/setup.rs` and its module in `lib.rs` are documented as record 0009's amendments of 2026-11-09 and 2026-11-11. `setup::answers(flags, runner_file, ask)` gathers the account, the drive and the repositories from the flags. It then takes them from the runner's file where its first line is setup's and it names one drive, then from `ask`, a terminal's questions, given only where standard input is a terminal, for the four answers the setup spec names in its order. The repositories are asked one a line until an empty one.

  A missing answer with no terminal is a usage error naming each missing flag. So are two drives in setup's file with no `--drive`, naming the drives, and the whole list beside a change to it. `--add-repository` adds after the drive's own, a repository the drive holds changing nothing. `--remove-repository` removes, one it does not hold changing nothing. A drive left with none, a name out of form and a repository the runner's file keeps in another drive are usage errors (design Decision 1). Its unit tests prove the setup spec's "An answer missing with no terminal", "Asked on a terminal", "The whole list beside a change to it", "Two drives and no drive named", "The last repository removed" and "A repository added twice". They are `an_answer_missing_with_no_terminal_names_each_flag`, `a_terminal_is_asked_in_order`, `the_whole_list_beside_a_change_to_it`, `two_drives_and_no_drive_named`, `the_last_repository_removed`, `a_repository_added_twice`, `a_repository_in_another_drive` and `an_account_out_of_form`. `main.rs`'s `the_command_line_is_well_formed` still passes. Done when `devenv shell -- cargo test -p boomdrive --bin boomdrive --lib -- setup::answers the_command_line_is_well_formed` passes.

## 5. The host

- [x] 5.1 `setup::check_host(host)` refuses, before any program runs:
  - a system other than `macos`, naming it and `services.boomdrive`, as the host's to fix;
  - a runner's managed file that is a link, as Nix's, never the host's to fix, naming the file and `services.boomdrive.accounts`;
  - each of `gh`, `herdr` and `launchctl` not found on the `PATH`, with where it comes from, as the host's to fix;
  - a file at the managed path whose first line is not setup's, beside a `config.toml.before-setup`, naming both, as the host's to fix.

  `host` gives the system, the home, the managed path and a finder, so its unit tests run on any system. They are `on_linux_setup_is_refused`, which proves the setup spec's "On Linux", `a_linked_runner_file_is_nixs`, `a_program_missing_is_named` and `both_files_there_are_named` (design Decision 7). Done when `devenv shell -- cargo test -p boomdrive --lib -- setup::check_host` passes.
- [ ] 5.2 `crates/boomdrive/tests/setup.rs` gains its stand-ins: a `Stand` over `Scratch` that writes each program, reads its calls back and sets the binary's `PATH` to the stand-ins' folder, `/usr/bin` and `/bin`. Under `--json`, setup runs, each refusal ending it:
  1. task 5.1's checks;
  2. the implementation's `settle` and `prepare` (tasks 3.1 and 3.3);
  3. `gh auth status --hostname github.com`, whose exit other than 0 refuses as the host's to fix, naming `gh auth login`.

  Its tests prove the setup spec's "The runner's file is Nix's", "crew missing", "GitHub not signed in" and "Both files there". They are `the_runners_file_is_nixs`, `crew_missing`, `github_not_signed_in` and `both_files_there`. Each asserts its exit and words, that no file setup writes is there, and that no `gh repo` or `crew` call was made (design Decisions 7 and 10). Done when `devenv shell -- cargo test -p boomdrive --test setup -- the_runners_file_is_nixs crew_missing github_not_signed_in both_files_there` passes.
- [ ] 5.3 `setup::sign_ins(find)` reads two sign-ins, after GitHub's:
  - Claude Code's from `claude auth status --json`, signed in where `loggedIn` is true;
  - Pending You's from `pendingyou status`, signed in where the line holding `Sign-in:` begins `ok` once trimmed.

  Each missing one, or one whose program is not found, is `{"what", "fix"}`, with `claude auth login` or `npx -y pendingyou@latest init`. Its tests prove the setup spec's "Claude Code not signed in" and "Pending You not installed", with `claude_code_not_signed_in` and `pending_you_not_installed` (design Decision 10; Risk 6). Done when `devenv shell -- cargo test -p boomdrive --test setup -- claude_code_not_signed_in pending_you_not_installed` passes.

## 6. What setup writes

- [ ] 6.1 `setup::runner_file(accounts, word, table)` renders the runner's managed layer as design Decision 6 says:
  - its first line `# Written by boomdrive setup; a change goes through boomdrive setup.`;
  - each drive's repositories ordered owner by owner;
  - top-level keys before tables, each sorted;
  - TOML's basic strings, as `nix/modules/toml.nix` writes them.

  Setup keeps every account and drive its own file names and sets this drive's. After task 5.2's checks and task 5.3's sign-ins, setup runs, each refusal ending it:
  1. the implementation's `set_up` (task 3.3);
  2. the runner's file. A person's file is moved to `config.toml.before-setup`. The file is written to a temporary file beside it and renamed only where its bytes differ, then read back by `config::read_layers`.

  Its tests prove the setup spec's "One drive under crew", which holds the file past its first line to `nix/tests/files/setup.toml` past its first line, "A second drive", "Nothing changed", "A local layer beside it", "A person's own file", "crew's flags" and "crew refuses". They are `one_drive_under_crew_is_the_modules_file`, `a_second_drive_keeps_the_first`, `nothing_changed_writes_nothing`, `a_local_layer_beside_it_is_left`, `a_persons_own_file_is_moved_aside`, `crews_flags_through_setup` and `crews_refusal_writes_no_runner_file` (design Decisions 4 and 6). Done when `devenv shell -- cargo test -p boomdrive --test setup -- one_drive_under_crew_is_the_modules_file a_second_drive_keeps_the_first nothing_changed_writes_nothing a_local_layer_beside_it_is_left a_persons_own_file_is_moved_aside crews_flags_through_setup crews_refusal_writes_no_runner_file` passes.
- [ ] 6.2 `setup::plugins(home, herdr)`, run after the runner's file, looks for folders under `share/herdr/plugins` of `~/.nix-profile`, else `~/.local/state/nix/profile`. It reads `herdr plugin list --json`, and runs `herdr plugin link <folder>` for each folder whose resolved path no listed `plugin_root` resolves to. herdr runs with every `HERDR_` variable removed and standard input closed. Each folder gets `linked`, `there` or `refused` with herdr's words. No folder at all is `missing` with `github:agentplot/boomdrive#herdr-plugins`. Its tests prove the setup spec's "Both plugins linked", "A plugin already linked" and "No plugin in the profile", with `both_plugins_linked`, `a_plugin_already_linked_is_left` and `no_plugin_in_the_profile`. The first also asserts the stand-in `herdr` saw no `HERDR_` variable (design Decision 8; core.29 as amended). Done when `devenv shell -- cargo test -p boomdrive --test setup -- both_plugins_linked a_plugin_already_linked_is_left no_plugin_in_the_profile` passes.
- [ ] 6.3 `setup::agent(home, config_file)` renders `io.github.agentplot.boomdrive`'s plist as design Decision 9 says. The config file's path is quoted for the shell. The `PATH` is the person's two profiles' `bin`, `/nix/var/nix/profiles/default/bin`, `~/.local/bin`, `/opt/homebrew/bin`, `/usr/local/bin`, `/usr/bin`, `/bin`, `/usr/sbin` and `/sbin`. Its unit test `the_agent_is_nix_darwins_under_setups_label` holds the plist for `/h` to the text the setup spec's "The agent" names. Done when `devenv shell -- cargo test -p boomdrive --lib -- setup::the_agent_is_nix_darwins_under_setups_label` passes.
- [ ] 6.4 Setup writes the plist to `~/Library/LaunchAgents` only where its bytes differ, after the plugins, and then:
  - runs `launchctl print gui/<uid>/io.github.agentplot.boomdrive`;
  - where that finds none, runs `launchctl bootstrap gui/<uid> <plist>`;
  - where the plist changed and it finds one, runs `launchctl bootout gui/<uid>/io.github.agentplot.boomdrive` and then the bootstrap;
  - otherwise runs neither.

  A `launchctl` refusal ends setup with its words, exit 1. The tests `first_loaded`, `loaded_and_unchanged` and `a_changed_plist_is_booted_out_first` prove the setup spec's "First loaded", "Changed and loaded" and "Loaded and unchanged", reading `<uid>` from `id -u` (design Decision 9). Done when `devenv shell -- cargo test -p boomdrive --test setup -- first_loaded loaded_and_unchanged a_changed_plist_is_booted_out_first` passes.

## 7. The answer

- [ ] 7.1 Setup answers as the setup spec's "Setup answers in one object", "Setup's exit says how it ended" and "Run again, setup changes nothing" say:
  - with `--json`, one `{"set_up": …}` object, or `{"refused":{"refusal":{"reason":…}}}`;
  - without it, one line for each step, ending with the implementation's `start`;
  - exit 0 answered, 1 refused by a tool or on a host Nix manages, 2 for a usage error or a host to fix.

  Its tests prove "The answer", "Run again", "A refusal in JSON", "Every answer given", "A repository added" and "A repository removed". They are `the_answer`, `run_again_changes_nothing`, `a_refusal_in_json`, `every_answer_given_asks_nothing`, `a_repository_added_is_kept_and_given_crew` and `a_repository_removed_leaves_both` (design Decisions 1 and 2). Done when `devenv shell -- cargo test -p boomdrive --test setup -- the_answer run_again_changes_nothing a_refusal_in_json every_answer_given_asks_nothing a_repository_added_is_kept_and_given_crew a_repository_removed_leaves_both` passes.

## 8. README

- [ ] 8.1 README's "Installing with nix" install line adds `github:agentplot/boomdrive#herdr` and `github:agentplot/boomdrive#herdr-plugins`, and says why the plugins come as one package. A new section after it, "Setting up a drive on a Mac Nix does not manage", says:
  - setup's one command, with an example of every answer as a flag;
  - what it asks on a terminal;
  - what it makes and writes, and the order;
  - `--add-repository` and `--remove-repository`;
  - the sign-ins it checks;
  - that it refuses on a host Nix manages;
  - `crew main up <drive>`;
  - `launchctl kickstart -k gui/$(id -u)/io.github.agentplot.boomdrive` after `nix profile upgrade` (design Risk 4);
  - that Flume's file is not written, with what that leaves (design Risk 2).

  `tests/readme.rs` gains `the_readme_names_how_a_persons_drive_is_set_up`. It asserts README names `boomdrive setup`, each of the setup spec's flags, `--json`, `crew main up` and `github:agentplot/boomdrive#herdr-plugins`. Done when `devenv shell -- cargo test -p boomdrive --test readme -- the_readme_names_how_a_persons_drive_is_set_up` passes.

## 9. The specs

- [ ] 9.1 The change's deltas archive into specs that pass openspec's strict check. Done when, after `devenv shell -- openspec archive a-persons-drive-is-set-up-by-one-command --yes`, `sluice exec main -- moon run root:specs` passes.
