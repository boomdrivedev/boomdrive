# Tasks

Work through the tasks in order. A task is done once the check it names passes, run as it names it. The groups are headings for reading.

- **The rules.** Record 0043, rule site.8, `site/CLAUDE.md`'s line for it and "The quick start" of `site/docs/site-design.md` are written with this change's plan, before task 1.1. A task that relies on a choice the design names cites its Decision.
- **Names.** Every step's file names the reader's own by the placeholders of design Decision 3: `your-github-name`, `your-repository`, `your-github-name/drive-state`, `your-host`, `you`, the account `me` and the drive `home`. A test's own scratch names are core.41's neutral ones, as the tests already take them.
- **The steps' files are read where they are.** Every test reads a step's file by its path under `site/src/start/`, and runs or evaluates it as it stands, never a copy of its text (site.8).
- **Stand-ins, never the host's programs.** No test runs the host's `gh`, `crew`, `herdr`, `launchctl`, `hostname`, `claude`, `pendingyou` or `nix`. The family's tools are the runner cargo built for the test and the set's Flume and sluice by the paths the devenv shell gives, never stubs of them (core.45). Nothing reaches the network.
- **What stays as it is.** No task adds a configuration key, a field of `report.json`, a protocol message, a crate, a tool or a command of the runner's, and no test of setup's changes what it holds. The stand-in agent is a binary of the runner's crate that only its feature `stand-in` builds, which no release names (design Decision 6).

## 1. Shared ground

- [x] 1.1 Setup's stand-ins move from `crates/boomdrive/tests/setup.rs` into a module of `crates/boomdrive/tests/common`, so the quick start's tests write the same `gh`, `crew`, `herdr`, `launchctl`, `hostname`, `claude` and `pendingyou`. Each notes its call and answers what the test put beside it, as each does now. `tests/setup.rs` uses the module, and none of its tests changes what it holds (design Decision 5). Done when `devenv shell -- cargo test -p boomdrive --test setup` passes.
- [x] 1.2 `site/src/start/.shellcheckrc` holds `shell=sh`, with a comment saying a step's file holds no shebang since the reader pastes its lines. `treefmt.nix` excludes it by name, with that reason. `settings.formatter.taplo.includes` gains `site/src/start/**/*.toml` beside `.prototools`, with a comment citing site.8 (design Decision 9; core.7). Done when `sluice exec main -- moon run root:format` passes.

## 2. The Mac's steps

- [x] 2.1 `site/src/start/mac/install.sh` is one `nix profile add` of these, in this order:
  - `github:agentplot/boomdrive`;
  - its `#flume`, `#sluice`, `#herdr`, `#herdr-plugins` and `#crew`;
  - `nixpkgs#git`, `nixpkgs#gh` and `nixpkgs#nodejs`.

  `nix/tests/quick-start.nix` is new, a function of `t` as the other cases' files are. Its case `"quick start: the Mac's install"` splits the file into words and holds it to that. Each `github:` installable names the address README's install names, read as `cache.nix` reads README, and a package of `t.outputs.packages.aarch64-darwin`, `default` for none. Each `nixpkgs#` installable names a package of the lock's nixpkgs for `aarch64-darwin`. `nix/tests/default.nix` imports the file into `test`, and `nix/moon.yml`'s `test` inputs gain `/site/src/start/**/*`. This proves the quick-start spec's "The Mac's install" (design Decisions 4 and 8). Done when `sluice exec main -- moon run nix:test` passes.
- [x] 2.2 `site/src/start/both/sign-in.sh` holds `gh auth login`, `claude auth login` and `npx -y pendingyou@latest init`, one a line, in that order. `crates/boomdrive/tests/quick_start.rs` is new. Its documentation cites record 0043, site.8 and design Decisions 5 and 6, and it reads each step's file from `../../site/src/start/` beside the crate's manifest. `crates/boomdrive/moon.yml`'s `test` inputs gain `/site/src/start/**/*`. Its test `the_sign_ins_are_the_ones_setup_names`, macOS's, runs setup over the stand-ins twice:
  1. GitHub signed out: it takes the fix the refusal names;
  2. GitHub signed in, Claude Code and Pending You out: it takes the two fixes the answer's `missing` names.

  The file's lines are those three, in that order, and nothing else. This proves "Each sign-in as setup names it" (design Decision 5). Done when `devenv shell -- cargo test -p boomdrive --test quick_start the_sign_ins_are_the_ones_setup_names` passes.
- [x] 2.3 `site/src/start/mac/flume.toml` holds `[host]` with `multiplexer = "herdr"` and `terminal_app = "terminal"`. A comment says a host Nix manages has these from the set's module. The test `flumes_file_opens_agents_in_herdr` writes the file as the Flume file of a scratch home, by Flume's own path under that home's configuration folder. It runs the set's Flume `--json status` there, read as `tests/family.rs` reads it. Flume answers with no refusal, and its default stream's part gives the multiplexer `herdr` and the terminal app `terminal`. This proves "The set's Flume takes the file" (design Decision 4; record 0043). Done when `devenv shell -- cargo test -p boomdrive --test quick_start flumes_file_opens_agents_in_herdr` passes.
- [x] 2.4 `site/src/start/mac/setup.sh` is `boomdrive setup --account me --drive home --state-repository your-github-name/drive-state --repository your-github-name/your-repository`. The test `setup_sets_up_the_drive_as_written`, macOS's, runs it as `sh <file>`. Its `PATH` is a folder linking `boomdrive` to the binary cargo built, then the stand-ins, then the test's own. GitHub is signed in, and gh finds no state repository. It exits 0, and the runner's file it wrote is read as the runner reads it. That file names the account `me`'s drive `home` keeping `your-github-name/your-repository`, and crew's table the partition `home` with `your-github-name/drive-state`. This proves "Setup as written" (design Decision 5). Done when `devenv shell -- cargo test -p boomdrive --test quick_start setup_sets_up_the_drive_as_written` passes.

- [x] 2.5 `site/src/start/mac/flume.toml` also holds the drive's stream, `[streams.home]`, and `[repos."your-github-name/your-repository"]` with `stream = "home"`. `site/src/start/mac/crew-source.toml` is Flume's source `crew`: `kinds_dir = "~/.local/share/flume/kinds/crew"` and `roles = "~/.local/state/crew/flume/roles.toml"`, the reader writing it as `~/.config/flume/sources/crew.toml` (design Decision 4; record 0034, point 2; crew record 0057). The test `flumes_file_opens_agents_in_herdr` writes both where the reader writes them, and crew's roles file in crew record 0057's form, in crew's stead. The set's Flume answers with no refusal; the streams `default` and `home` each give `herdr` and `terminal` from the host; and the source `crew` names the kinds folder and roles file under the scratch home and reads the five roles at `home/<role>`. This proves "The set's Flume takes the file". Done when `devenv shell -- cargo test -p boomdrive --test quick_start flumes_file_opens_agents_in_herdr` passes.

## 3. NixOS's steps

- [x] 3.1 `site/src/start/nixos/flake.nix` is the host's flake of design Decision 4:
  - its inputs are `nixpkgs` and `boomdrive`, the address README's install names, its `nixpkgs` its own;
  - its outputs, of `{ nixpkgs, boomdrive, ... }`, give `nixosConfigurations.your-host` from `./configuration.nix`, `boomdrive.nixosModules.default` and `./boomdrive.nix`.

  The case `"quick start: NixOS's flake"` imports it and reads `inputs.boomdrive.url` as README's address. It calls `outputs` with a `nixpkgs` whose `lib.nixosSystem` gives back its arguments, and a `boomdrive` whose `nixosModules.default` is a marker. The configuration's `modules` are then `./configuration.nix`, the marker and the path of `site/src/start/nixos/boomdrive.nix`. This proves "The host's flake" (design Decision 8). Done when `sluice exec main -- moon run nix:test` passes.
- [x] 3.2 `site/src/start/nixos/boomdrive.nix` is the module of design Decision 4. It sets:
  - the cache and its key as the root's `flake.nix` `nixConfig` names them;
  - nixpkgs' `git`, `gh` and `nodejs` in the system's packages;
  - `services.boomdrive`, with `user = "you"`, the account `me`'s drive `home` keeping `your-github-name`'s `your-repository`, and `control_plane.type = "crew"`;
  - `services.crew`, with the host `your-host` and the partition `home` of `your-github-name/drive-state`, by the keys crew's module takes on this branch.

  The case `"quick start: NixOS's drive"` evaluates `t.nixosSystem` with the normal user `you` and the file imported. The runner's rendered file names the account `me`'s drive `home` keeping `your-github-name/your-repository`, under crew with host `your-host`. crew's partition `home` names `your-github-name/drive-state`, Flume's `settings.host.multiplexer` is `herdr`, and the substituter and key are `nixConfig`'s. This proves "The drive on NixOS" (design Decision 8). Done when `sluice exec main -- moon run nix:test` passes.
- [x] 3.3 `site/src/start/nixos/switch.sh` is `sudo nixos-rebuild switch --flake /etc/nixos#your-host`. The case `"quick start: NixOS's switch"` holds the line to that folder and that configuration. It evaluates the drive's system's `config.system.build.toplevel.drvPath`, giving the system a root file system and a boot loader where `t.nixosSystem`'s base has none. This proves "The switch's system" (design Decision 8; Risk 6). Done when `sluice exec main -- moon run nix:test` passes.
- [x] 3.4 `site/src/start/nixos/state.sh` is `gh repo create your-github-name/drive-state --private`. The test `the_state_repository_is_made_as_setup_makes_it`, macOS's, runs setup over the stand-ins where gh finds no `your-github-name/drive-state`. The `gh repo create` call the stand-in noted is, word for word, the file's one line. This proves "Made as setup makes it" (design Decision 5). Done when `devenv shell -- cargo test -p boomdrive --test quick_start the_state_repository_is_made_as_setup_makes_it` passes.

- [ ] 3.5 `main`, once it holds the bolt `the-set-declares-crews-source-to-flume`, is merged into this branch, bringing `nix/modules/sources.nix` (record 0034, point 2). `nixos/boomdrive.nix` writes neither crew's source nor the drive's stream. The case `"quick start: NixOS's drive"` also holds that the set gives Flume the stream `home`, the repository's work in it, and the source `crew`. A case `"quick start: the Mac's crew source"` holds `site/src/start/mac/crew-source.toml`, read by `builtins.fromTOML`, equal to that system's `services.flume.sources.crew`, so the Mac's file and the module never drift. This proves "The drive on NixOS" and "The Mac's source is the set's" (design Decision 8). Done when `sluice exec main -- moon run nix:test` passes.

## 4. The steps both ways share

- [x] 4.1 `site/src/start/both/sluice.toml` is the smallest policy the set's sluice takes that lands `main` on `origin` after one command. The command is one any repository runs, and its comment tells the reader to put their own tests' command. Its keys are those sluice's configuration reference gives at the set's tag. `site/src/start/both/land.sh` holds a `git add` of the landing step's files, a Conventional Commit of them, and `git push origin HEAD:main`. The test `sluices_file_lands_on_main`:
  1. writes the policy as `.config/sluice.toml` in a scratch clone of a bare origin;
  2. runs `land.sh` there through `sh`;
  3. in a second clone, asks the set's sluice to read `main`'s policy, by the command its reference gives for that.

  Origin's `main` holds the file, and sluice refuses nothing. This proves "The policy on main" (design Decision 5). Done when `devenv shell -- cargo test -p boomdrive --test quick_start sluices_file_lands_on_main` passes.
- [x] 4.2 `site/src/start/both/start.sh` is `crew main up home`. The test `the_drives_agents_start_as_setup_says`, macOS's, runs setup with `--json` over the stand-ins for the drive `home`. The answer's `start` is the file's one line. This proves "Started as setup says" (design Decision 5). Done when `devenv shell -- cargo test -p boomdrive --test quick_start the_drives_agents_start_as_setup_says` passes.
- [x] 4.3 `site/src/start/both/flume.toml` is `[profiles]` with `merge_queue = "sluice"`, which the reader writes as `.config/flume.toml`. Its comment says a drive that takes crew's changes binds `change = "openspec"` too, in a repository that holds OpenSpec (design Decision 4). `land.sh` adds it beside the policy. The test `sluices_file_lands_on_main` writes it in the first clone and holds origin's `main` to both files. In the second clone, the set's Flume launches a fix, its stage held with `--hold` so no agent starts: Flume refuses nothing, and the place lands, its `base_from` the merge queue. This proves "The policy on main" for Flume's binding, which Flume reads from the default branch alone (Flume's 105). Done when `devenv shell -- cargo test -p boomdrive --test quick_start sluices_file_lands_on_main` passes.
- [x] 4.4 `crates/boomdrive/tests/stand-in/claude.rs` is a stand-in for Claude Code, the `[[bin]]` `boomdrive-stand-in` of `crates/boomdrive/Cargo.toml`, with `test = false` and `required-features = ["stand-in"]`, the feature `stand-in` added with a comment citing design Decision 6, as Flume's `flume-stand-in` is built. Run by Flume's printed command, it runs the hooks Flume's plugin names under `--plugin-dir` as its own children, each with Claude Code's input on standard input: SessionStart, UserPromptSubmit with the prompt it was given, then, once it has written and committed `change.txt` in the place, Stop and SessionEnd. `crates/boomdrive/moon.yml`'s `test` runs `cargo test -p boomdrive --features stand-in`. Done when `sluice exec main -- moon run boomdrive:lint` passes.
- [ ] 4.5 `site/src/start/both/watch.sh` is `flume open`. The test `a_change_the_drive_is_given_lands` is design Decision 6, on `tests/control.rs`'s scratch forge and crew's `records` module, with the set's Flume and sluice:
  - the bare origin of `your-github-name/your-repository` holds `main` as tasks 4.1 and 4.3 leave it;
  - the scratch state repository holds, on `home/main`, a launch request of kind `fix` for it naming the test's host, as crew writes one once a plan is approved;
  - the runner's file holds the keys setup writes for the quick start's answers, its crew table naming that state repository;
  - Flume's scratch file names a pool whose account's program is `claude`, a link to the stand-in of task 4.4, and no multiplexer's server, so Flume prints the agent's command, which the test runs; the scratch home's `.profile` puts the set's Flume and sluice on the login's PATH.

  The runner's passes run until origin's `main` has the change, or two minutes pass. Then origin's `main` holds the change, and the runner's branch carries the place's status as the set's Flume gives it, landed. `watch.sh`, run through `sh` on the same scratch host, is answered by the set's Flume, exiting 0. This proves "From crew's launch to main". Where a tool of the set cannot carry it as its tag stands, stop short with found work naming the tool and what it lacks, and stub nothing of the set's (site.8; Risk 1). Done when `devenv shell -- cargo test -p boomdrive --features stand-in --test quick_start a_change_the_drive_is_given_lands` passes.

## 5. The page

- [ ] 5.1 `site/src/content/pages/start.md` holds the title `Quick start`, a description and the page's words. Its lede says where the steps end: a drive whose agents build and land a change. A list says what the reader brings, each linked to its own installer or guide:
  - an Apple silicon Mac with Nix and its flakes on, or a NixOS machine whose configuration is a flake;
  - a GitHub account;
  - Claude Code.

  The words also name the placeholders of design Decision 3. One record per step of design Decision 4 is under `site/src/content/start/`. Each holds `title`, `at`, `files` with `path` and, where the reader writes one, `writes`, and `test` with `project` and `name`, as the design's table gives them. Its body is the step's words: what it does, what it leaves, and for `mac-install`, `mac-setup`, `mac-flume`, `landing` and `ask` what the design's Decision 4 says of each. Each term Vale asks for is added to `site/src/content/terms/terms.yaml` (site.4). No word names a drive or a machine of ours (site.7). Done when `sluice exec main -- moon run site:prose` passes with no error and no warning on these files.
- [ ] 5.2 `site/src/content.config.ts` gains the collection `start`: a glob over `src/content/start/*.md`. Its schema:
  - `title`;
  - `at`, with optional whole numbers from 1 for `mac` and `nixos`, refusing a record with neither;
  - `files`, one at least, each a `path` and an optional `writes`;
  - `test`, a `project` of `boomdrive` or `nix` and a non-empty `name`.

  Its comment cites site.8 and design Decision 1. This proves "Every step is whole". Done when `sluice exec main -- moon run site:check` passes.
- [ ] 5.3 `site/src/pages/start.astro` builds `/start/` on `Base`, titled `Quick start`. It opens on `pages`' `start` and refuses the build where it is missing, as the Tools hub does. Then it shows `On a Mac` and `On NixOS`, each its steps in the order of `at`, numbered: the title, the words, and each file read as text at build, in a `pre` block as it stands, its `writes` above it. The build fails where a step names a file `src/start/` does not hold. The page has no script. `site/src/layouts/Base.astro` lists `Start`, `url("/start/")`, after `Tools`. `site/src/styles/site.css` gives a step's block the page's tokens and `overflow-x: auto`, and no colour of its own (design Decision 2; site.6). Done when `sluice exec main -- moon run site:build` passes.
- [ ] 5.4 `site/check-site.sh` gains the quick start's checks of design Decision 7. They prove the quick-start spec's:
  - "The page and its two ways" and "Reached from the masthead";
  - "A platform's places are its order" and "Each file as it stands";
  - "Each step's test is there" and "No file is left unshown";
  - "None of our drives or machines";
  - "No colour of its own" and "A long line at phone width".

  It reads the records' frontmatter with the site's `yaml` package through `bun`. Each check says one line when it holds and fails naming what is wrong, as the file's checks do. `site/moon.yml`'s `test` inputs gain `src/content/start/**/*`, `src/content/pages/start.md`, `src/start/**/*`, `src/content/contexts/contexts.yaml`, `/crates/boomdrive/tests/quick_start.rs` and `/nix/tests/quick-start.nix`. Done when `sluice exec main -- moon run site:test` passes.

## 6. README

- [ ] 6.1 README's "Installing with nix" opens with a line naming the quick start, `https://agentplot.github.io/boomdrive/start/`, as a stranger's way from nothing to a working drive. `crates/boomdrive/tests/readme.rs` gains `the_readme_names_the_quick_start`, which finds that address in README (design Decision 10). Done when `devenv shell -- cargo test -p boomdrive --test readme the_readme_names_the_quick_start` passes.
