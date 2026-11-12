# Verify: boomdrives-quick-start-takes-a-stranger-to-a-working-drive

The work on `change/boomdrives-quick-start-takes-a-stranger-to-a-working-drive` was read at a11db4e. The gate passes, and the work holds:

- All 24 tasks are ticked.
- The change passes `openspec validate --strict`.
- Each of its 17 requirements and 22 scenarios has a test or a check that asserts it, and each passed in the gate.

So the change is archived with this report.

## Checks

- **The gate.** `sluice exec main` at a11db4e ran the `check-fast` tasks that a11db4e affects against `main` at 8da7b7f, with a clean worktree. It exited 0. moon completed 36 tasks, and sluice recorded 34 passes under their keys for a11db4e. No task failed. The cargo tests that ran reported 1060 passed and 0 failed.
- **The tasks that ran.**
  - `nix:format`, `lint`, `typecheck` and `test`. `nix:test` reported "every case holds", including the five `quick start:` cases.
  - `site:install`, `workflow`, `vocab`, `build`, `check` (32 files, 0 errors, 0 warnings) and `prose` (0 errors, 0 warnings and 0 suggestions in 24 files). `site:build` built `/start/index.html`.
  - `site:test`, every quick-start line of which held:
    - the 7 steps On a Mac run 1 to 7, and the 8 On NixOS run 1 to 8;
    - each of the 11 steps names a test its project holds;
    - all 14 files of `src/start/` but `.shellcheckrc` belong to a step;
    - the page is its lede, then On a Mac and On NixOS, each step numbered in order, each file shown as it stands, and no script;
    - each block takes the page's tokens and scrolls within itself;
    - all 12 pages' mastheads link `/boomdrive/start/` as Start;
    - none of our 3 drives and machines is named.
  - `boomdrive-control`, `boomdrive-crew` and `boomdrive-hosted`: format, lint, typecheck and test, each.
  - `root:adrs`, `changes`, `commits`, `format`, `releases`, `specs` (25 of 25), `toolchain`, `versions` and `workflow`.
  - `boomdrive:format`, `lint`, `typecheck` and `test`. lint and typecheck cover the stand-in under `--all-features`, and `test` runs `cargo test -p boomdrive --features stand-in` whole.
- **The quick start's tests.** `quick_start.rs` ran its 7 tests under the gate's sandbox, and all passed in 11.38 s: `the_sign_ins_are_the_ones_setup_names`, `flumes_file_opens_agents_in_herdr`, `setup_sets_up_the_drive_as_written`, `the_state_repository_is_made_as_setup_makes_it`, `sluices_file_lands_on_main`, `the_drives_agents_start_as_setup_says` and `a_change_the_drive_is_given_lands`. `readme.rs` passed with `the_readme_names_the_quick_start`. `setup.rs` passed its 25 tests over the stand-ins that task 1.1 moved into `tests/common`.
- **Warnings.** moon warned 33 times that it could not package task outputs into its archives under `.moon/cache/outputs`, and once that it had no internet connection, so it skipped the rust toolchain's dependency install. Neither changed any task's result.
- The gate was run once, as this stage's brief says, and nothing was fixed here.

## Tasks

All 24 tasks of `tasks.md`, groups 1 to 6, are ticked `[x]`, and none is open. What they name is in the tree:

- **Shared ground** (1.1, 1.2):
  - `tests/common/stand.rs` holds setup's stand-ins, and `tests/setup.rs` uses them, its 25 tests unchanged in number.
  - `site/src/start/.shellcheckrc` holds `shell=sh`. `treefmt.nix` excludes it with its reason, and taplo claims `site/src/start/**/*.toml`.
- **The Mac's steps** (2.1 to 2.5):
  - `mac/install.sh` is one `nix profile add` of the nine installables, in order.
  - `both/sign-in.sh` holds the three sign-ins.
  - `mac/flume.toml` holds `[host]`, `[streams.home]` and the repository's stream, and `mac/crew-source.toml` names crew's kinds folder and roles file.
  - `mac/setup.sh` is the one `boomdrive setup` line.
  - Each test the tasks name is in `quick_start.rs` or `quick-start.nix`.
- **NixOS's steps** (3.1 to 3.6):
  - `nixos/flake.nix`, `nixos/boomdrive.nix`, `nixos/switch.sh` and `nixos/state.sh` are as the design gives them.
  - `boomdrive.nix` sets crew's host and drive by the keys of crew's module at the pin `main` brings, and nothing of Flume's.
  - `main` at 62835d2 is merged in (28ce00e9). `main` has since gained only 8da7b7f, a record's amendment in `docs/adr/`, which the gate ran against.
- **The steps both ways share** (4.1 to 4.6):
  - `both/sluice.toml`, `both/flume.toml`, `both/land.sh`, `both/start.sh` and `both/watch.sh` are as the tasks give them.
  - The stand-in is the `[[bin]]` `boomdrive-stand-in`, built under the feature `stand-in` alone, and `boomdrive:test` runs `cargo test -p boomdrive --features stand-in`.
  - Task 4.6: `quick_start.rs`'s `on_scratch` gives every program on the scratch host `SLUICE_USER_SCRATCH`, the scratch host's `sluice` folder. `both/sluice.toml` is unchanged. Its `sent_back` checks each place before any printed command runs. Where Flume's status gives a send-back, the test fails with the merge queue's verdict and the set's sluice's `wait` of the submission, run in the runner's clone. Design Decision 6 gains "sluice's scratch" and "A stage sent back" to match. The task's test passes in the gate.
- **The page** (5.1 to 5.4):
  - `pages/start.md` and the eleven step records exist. The collection `start` has its schema.
  - `pages/start.astro` builds `/start/` with no script, and the masthead lists Start after Tools.
  - `site.css` gives the step blocks the page's tokens and `overflow-x: auto`.
  - `check-site.sh` holds the quick start's checks, and `site/moon.yml`'s `test` inputs name every file the task names.
- **README** (6.1): "Installing with nix" opens with the quick start's address, and `readme.rs`'s `the_readme_names_the_quick_start` passes.

Record 0043, site.8, `site/CLAUDE.md`'s line and "The quick start" of `site/docs/site-design.md` are in the tree.

Apart from `bolt/one-command-sets-up-a-persons-drive` at b839cea and `main` at 62835d2, the change's own commits touch these paths:

- the paths the proposal's Touches lists, except `site/src/content/terms/terms.yaml`, which Vale asked for no term in;
- `crates/boomdrive/tests/common/` and `tests/setup.rs`, from task 1.1;
- `tests/readme.rs`, from task 6.1.

## Requirements and scenarios

| requirement | scenario | asserted by | in the gate |
| --- | --- | --- | --- |
| The quick start is one page of a Mac's steps and NixOS's | The page and its two ways | `site:test`, the page's order, numbering and no script | passed |
| The quick start is one page of a Mac's steps and NixOS's | Reached from the masthead | `site:test`, every page links Start | passed |
| A step is a record of its words, its files and its test | Every step is whole | `content.config.ts`'s `start` schema, by `site:check` and `site:build` | passed |
| A step is a record of its words, its files and its test | A platform's places are its order | `site:test`, places 1 to their count | passed |
| The page shows each step's files as they stand | Each file as it stands | `site:test`, each file escaped as one block, `writes` above it | passed |
| Every step names a test its project holds, and every file is a step's | Each step's test is there | `site:test`, `fn <name>(` and `"<name>" =` | passed |
| Every step names a test its project holds, and every file is a step's | No file is left unshown | `site:test`, `src/start/` but `.shellcheckrc` | passed |
| The quick start names none of ours | None of our drives or machines | `site:test`, `contexts.yaml`'s names | passed |
| The page's blocks follow the reader's theme and width | No colour of its own | `site:test`, no `style` and no colour in the block's rule | passed |
| The page's blocks follow the reader's theme and width | A long line at phone width | `site:test`, `overflow-x:auto` and no wrap | passed |
| The Mac's install brings the set and what the steps run | The Mac's install | `nix:test` `quick start: the Mac's install` | passed |
| The sign-ins are those setup names | Each sign-in as setup names it | `quick_start.rs` `the_sign_ins_are_the_ones_setup_names` | passed |
| A Mac's Flume files open agents in herdr, where crew's roles answer | The set's Flume takes the file | `quick_start.rs` `flumes_file_opens_agents_in_herdr` | passed |
| A Mac's Flume files open agents in herdr, where crew's roles answer | The Mac's source is the set's | `nix:test` `quick start: the Mac's crew source` | passed |
| The Mac's setup sets up the drive as written | Setup as written | `quick_start.rs` `setup_sets_up_the_drive_as_written` | passed |
| NixOS's flake takes the set's module and the drive's file | The host's flake | `nix:test` `quick start: NixOS's flake` | passed |
| NixOS's drive file gives the drive to the runner, crew and Flume | The drive on NixOS | `nix:test` `quick start: NixOS's drive` | passed |
| NixOS's switch builds the configuration the flake gives | The switch's system | `nix:test` `quick start: NixOS's switch` | passed |
| The state repository is made as setup makes it | Made as setup makes it | `quick_start.rs` `the_state_repository_is_made_as_setup_makes_it` | passed |
| The landing step's files are the ones sluice and Flume land main by | The policy on main | `quick_start.rs` `sluices_file_lands_on_main` | passed |
| The drive's agents start as setup says | Started as setup says | `quick_start.rs` `the_drives_agents_start_as_setup_says` | passed |
| A change the drive is given lands | From crew's launch to main | `quick_start.rs` `a_change_the_drive_is_given_lands`, under the gate's sandbox | passed |
