# Verify: boomdrives-quick-start-takes-a-stranger-to-a-working-drive

The work on `change/boomdrives-quick-start-takes-a-stranger-to-a-working-drive` was read at 32a8ed6. The gate fails: `boomdrive:test`'s `a_change_the_drive_is_given_lands` fails, which is the test of the scenario "From crew's launch to main". So the change is not archived and goes back to its code stage.

The rest of the work matches the change:

- All 23 tasks are ticked.
- The change passes `openspec validate --strict`.
- Each of its 17 requirements and 22 scenarios has a test or a check that asserts it.

## Checks

- **The gate.** `sluice exec main` at 32a8ed6 ran the `check-fast` tasks that 32a8ed6 affects against `main` at 62835d2, with a clean worktree. It exited 1. moon finished 36 tasks: 35 passed and `boomdrive:test` failed. The tests that ran reported 921 passed and 1 failed.
- **The tasks that passed.**
  - `nix:format`, `lint`, `typecheck` and `test`. `nix:test` reported "every case holds", including the five `quick start:` cases.
  - `site:install`, `workflow`, `vocab`, `build`, `check` (0 errors, 0 warnings) and `prose` (0 errors, 0 warnings and 0 suggestions in 24 files).
  - `site:test`, every quick-start line of which held:
    - each way's places run 1 to 7 on a Mac and 1 to 8 on NixOS;
    - each of the 11 steps names a test its project holds;
    - all 14 files of `src/start/` but `.shellcheckrc` belong to a step;
    - the page is its lede, then On a Mac and On NixOS, with each file shown as it stands and no script;
    - each block takes the page's tokens and scrolls within itself;
    - all 12 pages link Start;
    - none of our 3 drives and machines is named.
  - `boomdrive-control`, `boomdrive-crew` and `boomdrive-hosted`: format, lint, typecheck and test, each.
  - `root:format` (376 files, none changed), `changes`, `commits`, `releases`, `specs` (25 of 25), `adrs` (43 records), `toolchain`, `versions` and `workflow`.
  - `boomdrive:format`, `lint` and `typecheck`. lint and typecheck cover the stand-in under `--all-features`.
- **The failure.** `quick_start.rs`'s first six tests passed: `the_sign_ins_are_the_ones_setup_names`, `flumes_file_opens_agents_in_herdr`, `setup_sets_up_the_drive_as_written`, `the_state_repository_is_made_as_setup_makes_it`, `sluices_file_lands_on_main` and `the_drives_agents_start_as_setup_says`. `a_change_the_drive_is_given_lands` panicked at `crates/boomdrive/tests/quick_start.rs:733`, after 19 s of the test's 120 s bound. The sequence was:
  1. The runner launched the fix. The test ran Flume's first printed command, and the stand-in committed `change.txt`. The crew send was appended and the submission went to sluice.
  2. The scratch repository's sluice failed it: `[sluice] #1 failed on main: [tests]. sluice wait 1 shows why.`
  3. Flume sent the code stage back with those words and printed a second command for the stage (`…:code:2.env`).
  4. The test ran that second command. The stand-in wrote `change.txt` again, which was already committed. Its `git commit` found "nothing to commit, working tree clean", and it panicked at `crates/boomdrive/tests/stand-in/claude.rs:152`, exiting 101. The test's assertion that each printed command exits 0 then failed.
- **What the failure shows.** There are two faults:
  - Under the gate, the scratch policy's sluice failed its own `[tests]` run of `site/src/start/both/sluice.toml`, whose command is `true`.
  - Neither the test nor the stand-in handles a stage that Flume sends back. The stand-in makes the same change again and fails, so the test reports the stand-in's panic rather than sluice's refusal.

  The scratch home was removed with the test, so `sluice wait 1`'s reason could not be read.
- **Not shown, and likely.** No task ran this test under the gate's sandbox executor. Task 4.5's done-check is `devenv shell -- cargo test -p boomdrive --features stand-in --test quick_start a_change_the_drive_is_given_lands`, run outside sluice. `.config/sluice.toml` runs the gate with `[executor] kind = "sandbox"` and `network = "localhost"`. The scratch policy names no executor, so the inner sluice runs its `[tests]` under its own default executor, inside the gate's sandbox. The code stage should establish why the inner run fails there before changing anything. Rule site.8 and design Decision 6 hold the test to the set's own sluice, with no stub.
- **What the gate did not reach.** cargo stops at the first failing test binary, so `boomdrive:test` never ran these test files: `readme.rs` (`the_readme_names_the_quick_start`, task 6.1), `room.rs`, `rules.rs`, `running.rs`, `scratch.rs`, `setup.rs` (task 1.1's move of the stand-ins), `sluice.rs`, `statuses.rs`, `terminals.rs` and `views.rs`. The 921 tests above do not include them.
- **Warnings.** moon warned that it could not package task outputs into its archives under `.moon/cache/outputs`. That did not change any task's result.
- The gate was run once, as this stage's brief says, and nothing was fixed here.

## Tasks

All 23 tasks of `tasks.md`, groups 1 to 6, are ticked `[x]`, and none is open. What they name is in the tree:

- **Shared ground** (1.1, 1.2):
  - `tests/common/stand.rs` holds setup's stand-ins. `tests/setup.rs` uses them, its 25 tests unchanged in number, and none of its assertions moved but those of the helpers.
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
  - `main` at 62835d2 is merged in (28ce00e9).
- **The steps both ways share** (4.1 to 4.5):
  - `both/sluice.toml`, `both/flume.toml`, `both/land.sh`, `both/start.sh` and `both/watch.sh` are as the tasks give them.
  - The stand-in is the `[[bin]]` `boomdrive-stand-in`, built under the feature `stand-in` alone, and `boomdrive:test` runs `cargo test -p boomdrive --features stand-in`.
  - Task 4.5's test is the one that fails in the gate.
- **The page** (5.1 to 5.4):
  - `pages/start.md` and the eleven step records exist. The collection `start` has its schema.
  - `pages/start.astro` builds `/start/` with no script, and the masthead lists Start after Tools.
  - `site.css` gives the step blocks the page's tokens and `overflow-x: auto`.
  - `check-site.sh` holds the quick start's checks, and `site/moon.yml`'s `test` inputs name every file the task names.
- **README** (6.1): "Installing with nix" opens with the quick start's address, and `readme.rs` holds `the_readme_names_the_quick_start`. The gate did not reach that test, as said above.

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
| A change the drive is given lands | From crew's launch to main | `quick_start.rs` `a_change_the_drive_is_given_lands` | **failed** |

## What goes back to the code stage

- `a_change_the_drive_is_given_lands` has to pass in the gate, `sluice exec main`, under its sandbox executor, and not only in a plain `devenv shell`. First, find out why the scratch policy's sluice fails its `[tests]` there, which `sluice wait 1` names inside the test's scratch home. Then fix that, keeping the set's own sluice and Flume (site.8; core.45; design Decision 6).
- A stage that sluice refuses should come back as that refusal. At present the stand-in panics on a second run, and the test reports that panic instead of what sluice said.
- After that, the gate has to run `boomdrive:test` whole, so that `readme.rs` and `setup.rs`, which this gate never reached, pass too.
