# Design

## Context

See proposal.md for why. The change carries out "The quick start" of `site/docs/site-design.md`, with its row of the site map and the type Step, written with this plan. The decision is record 0043, and its rule site.8.

**The rules it relies on.** `docs/architecture/site.md` binds `site/`, and `core.md` binds every path.

- core.4: no token in the repository. No step holds one; the sign-ins are the reader's, by each tool's own command.
- core.7: every tracked file is a formatter's or excluded by name, and each check is one task. The steps' files are claimed in `treefmt.nix` (Decision 9), and the new tests are tests of the projects that run them.
- core.8 and core.45: every tool a task runs is the tree's pin, and the runner's tests run the set's own binaries. The tests run the runner built from the tree, and the set's Flume and sluice by the paths the devenv shell gives.
- core.18: the runner writes no other tool's file, so setup writes no Flume file, and the Mac's Flume file is the reader's (Decision 4).
- core.41 and site.7: neutral names, and none of ours on the quick start (Decision 3).
- core.46 and core.47: the modules write each tool's file from its options, and a host trusts the family's cache in its own nix settings. The NixOS drive's file does both through options.
- site.4: the glossary is the one vocabulary, and the steps' words pass Vale.
- site.5: a new page type, its schema and its checks are an OpenSpec change. This change writes the steps' first words. Later rewording is the content kind's, and the gate holds each reworded file to its test.
- site.6: every link goes through `url()`.
- site.8: every step's text is files the page shows as they stand, and one test of the gate runs them.

**The sections of `site/docs/site-design.md` it relies on.**

- "The quick start", "Site map" and "The types", written with this plan.
- "Decisions", whose typed collections fail the build on a missing field.
- "The Tools hub", whose masthead lists the sections and folds them under a menu button on a phone. The masthead gains "Start" (Decision 2).

**It departs from one design rule.** `openspec/config.yaml` asks that a reference between types be one the build checks. A step's files are, but a step's test is in another project's tree, which the site's build does not read. `site:test` checks it instead (Decision 7).

**What the reader's way meets today.**

- On a Mac Nix does not manage, `boomdrive setup` sets up the drive: crew's file, the runner's file, the plugins and the agent.
- Flume with no file runs the printed command, so no agent builds. Flume has no command that writes its host keys (record 0009, its amendment of 2026-11-11).
- On NixOS, the set's module writes every tool's file, Flume's host keys among them (record 0023), and crew's module names the drive and its host.
- sluice lands with the host's own git credentials where no `[forge]` is named. gh's sign-in sets those credentials.
- The gate has no network, and its nix evaluates and never builds.

## Goals / Non-Goals

**Goals:**

- One page, `/start/`, that takes a stranger on an Apple silicon Mac or a NixOS machine from nothing of the family to a drive whose agents build and land a change.
- Every step's text is files the page shows as they stand, and one test of the gate runs them as they stand.
- `site:test` fails a step whose test is missing, a file no step shows, a file the page does not show as it stands, a platform out of order, and a name of ours.

**Non-Goals:**

- Installing Nix or Claude Code. Each is the reader's, named and linked (record 0043).
- An Intel Mac, which the flake gives nothing, and a Linux host Nix does not manage, which setup does not set up.
- nix-darwin's way, which README's install with nix gives.
- sluice's protection of `main` and its own GitHub App, which sluice's guides give. The quick start lands with the reader's own credentials.
- A hosted control plane. The drive is crew's, the one control plane a stranger can run.
- A machine booted on CI that runs the whole quick start with the network (record 0043, option 3).
- Home's calls. Home's design is unchanged, and the masthead reaches the quick start.

## Decisions

### 1. A step is a record, and its text is files

A new collection, `start`, holds one Markdown record per step under `site/src/content/start/`. Each is named for the step: `mac-install.md`, `sign-in.md`, and so on. Its body is the step's words, which Vale reads. Its frontmatter:

- `title`;
- `at`: its place on each platform it is on, `mac` and `nixos`, each a whole number from 1, one at least;
- `files`: one or more, each `path`, under `site/src/start/`, and `writes`, the path the reader writes it to, where they write one;
- `test`: `project`, `boomdrive` or `nix`, and `name`.

The files are under `site/src/start/`, in three folders: `mac/`, `nixos/` and `both/`. The page reads each at build, as text, and the build fails where a step names a file that is not there.

Why files and not a Markdown record's code blocks: nix in the gate cannot evaluate a string. It has no store to write one into, and it reads no YAML or Markdown. So a Nix step must be a file nix imports by its path. The shell and TOML steps are files too, so every test reads its step the same way and `site:test` lists one folder. Not taken: blocks inside the records, which the Rust tests could read and nix could not. Not taken: one YAML file of every step, which nix cannot read and Vale does not.

### 2. One page, two ways

`src/pages/start.astro` builds `/start/`, titled `Quick start`. It opens on `src/content/pages/start.md`, through the collection `pages`, as the Tools hub does: its lede, then what the reader brings. That is Nix with flakes on an Apple silicon Mac, or a NixOS configuration that is a flake; a GitHub account; Claude Code. Each is linked to its own installer or guide. Then `On a Mac`, then `On NixOS`. Each shows its steps in the order of their `at`, numbered: the title, the words, then each file in a `pre` block as it stands, with its `writes` above it. A step on both platforms is rendered in both sections from its one record.

There is no script, no tab and no motion, so with scripts off the page is the same. The blocks take the page's tokens and carry no colour of their own: no syntax colouring, so nothing differs between light and dark. Each `pre` scrolls along its own lines, `overflow-x: auto`, so the page never scrolls sideways at phone width. `src/layouts/Base.astro` lists `Start` after `Tools` in the masthead, through `url("/start/")`. Not taken: tabs for the two platforms, which need a script or a CSS trick to show both with scripts off, for a page short enough to read whole.

### 3. The reader's names are placeholders

Every step names the reader's own with the same neutral placeholders:

- the GitHub owner `your-github-name`;
- the repository `your-repository`;
- the state repository `your-github-name/drive-state`;
- the NixOS host `your-host` and its user `you`;
- the account `me` and the drive `home`.

The words tell the reader to put their own. Each is of the form setup and the modules take, so the tests run the files as written. None is a drive or a machine of ours (site.7), and `site:test` checks the page and the files against `contexts.yaml`'s names. Not taken: shell variables set in a first step, which a NixOS file cannot use and a new terminal loses.

### 4. The steps

| Platform and place | Record | Files | What it leaves | Test |
|---|---|---|---|---|
| Mac 1 | `mac-install` | `mac/install.sh` | the set, git, gh and Node.js in the profile | `nix`: `quick start: the Mac's install` |
| Mac 2, NixOS 4 | `sign-in` | `both/sign-in.sh` | GitHub, Claude Code and Pending You signed in | `boomdrive`: `the_sign_ins_are_the_ones_setup_names` |
| Mac 3 | `mac-flume` | `mac/flume.toml`, writes `~/.config/flume/config.toml`; `mac/crew-source.toml`, writes `~/.config/flume/sources/crew.toml` | Flume opening agents in herdr, in the drive's stream, where crew's roles answer | `boomdrive`: `flumes_file_opens_agents_in_herdr` |
| Mac 4 | `mac-setup` | `mac/setup.sh` | the drive set up, the runner running | `boomdrive`: `setup_sets_up_the_drive_as_written` |
| NixOS 1 | `nixos-flake` | `nixos/flake.nix`, writes `/etc/nixos/flake.nix` | the host's flake taking the set | `nix`: `quick start: NixOS's flake` |
| NixOS 2 | `nixos-drive` | `nixos/boomdrive.nix`, writes `/etc/nixos/boomdrive.nix` | the drive in the runner's, crew's and Flume's files | `nix`: `quick start: NixOS's drive` |
| NixOS 3 | `nixos-switch` | `nixos/switch.sh` | the system switched | `nix`: `quick start: NixOS's switch` |
| NixOS 5 | `nixos-state` | `nixos/state.sh` | the state repository | `boomdrive`: `the_state_repository_is_made_as_setup_makes_it` |
| Mac 5, NixOS 6 | `landing` | `both/sluice.toml`, writes `.config/sluice.toml`; `both/flume.toml`, writes `.config/flume.toml`; `both/land.sh` | `main` landing through sluice | `boomdrive`: `sluices_file_lands_on_main` |
| Mac 6, NixOS 7 | `start` | `both/start.sh` | the drive's agents running | `boomdrive`: `the_drives_agents_start_as_setup_says` |
| Mac 7, NixOS 8 | `ask` | `both/watch.sh` | a change built and landed | `boomdrive`: `a_change_the_drive_is_given_lands` |

What each file holds:

- **`mac/install.sh`**: one `nix profile add` of `github:agentplot/boomdrive` and its `#flume`, `#sluice`, `#herdr`, `#herdr-plugins` and `#crew`, then `nixpkgs#git`, `nixpkgs#gh` and `nixpkgs#nodejs`. git, since the runner needs 2.48 at least and a Mac's own may be older; gh, which setup and the sign-in need; Node.js for `npx`. The words say to answer `y` to nix's question about the flake's settings, and that a user nix does not trust builds the same programs from source.
- **`both/sign-in.sh`**: `gh auth login`, `claude auth login`, `npx -y pendingyou@latest init`, one a line, in the order setup checks them.
- **`mac/flume.toml`**: `[host]` with `multiplexer = "herdr"` and `terminal_app = "terminal"`; the drive's stream, `[streams.home]`, named as the drive names it; and `[repos."your-github-name/your-repository"]` with `stream = "home"`, so the repository's work runs in the drive's stream, as `drives.nix` gives a host Nix manages (record 0028). crew declares its roles at `<label>/<role>`, `home/ops` among them, and Flume declares a role only on a stream the host declares, so without the stream every launch crew writes is refused, its recipient waking nothing. Terminal.app is on every Mac, and the profile's Ghostty is in no folder `open` reads (record 0043). The words say a host Nix manages has these from the module, and that this step goes once Flume and setup can set them.
- **`mac/crew-source.toml`**: Flume's source `crew`, `kinds_dir = "~/.local/share/flume/kinds/crew"` and `roles = "~/.local/state/crew/flume/roles.toml"`, the two keys the set's module gives Flume where crew is on (`nix/modules/sources.nix`; record 0034, point 2). crew writes the roles file and makes the folder, and the host declares the source alone (crew record 0057), so on a Mac the reader writes it, as they write Flume's file. No setup command writes a file of Flume's (core.18). `nix:test` holds the file's keys equal to the module's, so the two never drift (Decision 8).
- **`mac/setup.sh`**: `boomdrive setup --account me --drive home --state-repository your-github-name/drive-state --repository your-github-name/your-repository`. Every other answer is setup's own. The words name `launchctl kickstart -k gui/$(id -u)/io.github.agentplot.boomdrive` after a `nix profile upgrade`, as README does.
- **`nixos/flake.nix`**: inputs `nixpkgs`, `github:NixOS/nixpkgs/nixos-unstable`, and `boomdrive`, the address README's install names, its `nixpkgs` its own, as README asks. Outputs `nixosConfigurations.your-host = nixpkgs.lib.nixosSystem { modules = [ ./configuration.nix boomdrive.nixosModules.default ./boomdrive.nix ]; }`. The lambda takes `{ nixpkgs, boomdrive, ... }`, so deadnix finds nothing unused.
- **`nixos/boomdrive.nix`**: a module of `{ pkgs, ... }`. It sets:
  - `nix.settings.extra-substituters` and `extra-trusted-public-keys`, the flake's `nixConfig`'s;
  - `environment.systemPackages`, nixpkgs' `git`, `gh` and `nodejs`;
  - `services.boomdrive`, with `enable`, `user = "you"`, `accounts.me.drives.home.repositories.your-github-name = [ "your-repository" ]` and `settings.control_plane.type = "crew"`;
  - `services.crew`, with `enable`, `host = "your-host"`, the host `your-host` of kind `nixos` under `settings.hosts`, and the drive `home` under `settings.drives`, whose state is `your-github-name/drive-state`, whose machine is `your-host` and whose blueprints are the drive's repository, by the keys crew's module takes at the set's pin. Its other keys are those setup gives on a Mac: the session `home`, the folder the runner's repository root under the user's home, `/home/you/Code`, no `repos`, since the set gives crew the kept repositories (core.57), every role under the account `me` and standing, and the review `5` changes or `4` hours.

  Flume's host keys come from the set's module (record 0023), the drive's stream and its repository's from `drives.nix` (record 0028), and crew's source from `sources.nix` (record 0034, point 2), so the file writes none of them.
- **`nixos/switch.sh`**: `sudo nixos-rebuild switch --flake /etc/nixos#your-host`.
- **`nixos/state.sh`**: `gh repo create your-github-name/drive-state --private`, the call setup's crew implementation makes on a Mac.
- **`both/sluice.toml`**: the smallest policy the set's sluice takes that lands `main` on `origin` after one command: a command runner whose command runs in any repository, and `[branches.main]` naming `origin`. Its comment and the step's words tell the reader to put their own tests' command.
- **`both/flume.toml`**: `[profiles]` with `merge_queue = "sluice"`, the repository's binding of its merge queue. Flume reads it from `.config/flume.toml` at the repository's default branch alone, and guesses none from another tool's files (Flume's 105 and its Part C), so without it Flume refuses every launch of a kind that lands. Its comment and the `landing` step's words say that a drive taking crew's changes binds `change = "openspec"` too, in a repository that holds OpenSpec: crew's units launch Flume's change kinds, which Flume refuses unless the change tool is bound. The quick start's own test lands a fix (Decision 6).
- **`both/land.sh`**: run in a clone of the reader's repository holding both files: `git add .config/sluice.toml .config/flume.toml`, a Conventional Commit of them, and `git push origin HEAD:main`.
- **`both/start.sh`**: `crew main up home`.
- **`both/watch.sh`**: `flume open home`, the drive's stream's session, where its agents run. The words of `ask` say to tell the drive's agents the change wanted, approve its plan on the reader's rail, and watch it build and land in the window `flume open home` shows.

### 5. The Rust tests run the shell steps through sh

`crates/boomdrive/tests/quick_start.rs` reads each file by its path from the crate's manifest folder, `../../site/src/start/…`, as `readme.rs` reads README. A shell step runs as `sh <file>` in a scratch home under `common::Scratch`. Its `PATH` is:

1. a folder holding `boomdrive`, a link to the binary cargo built for the test;
2. the stand-ins' folder;
3. the test's own `PATH`, whose git is the gate's.

The stand-ins are those `tests/setup.rs` writes: `gh`, `crew`, `herdr`, `launchctl`, `hostname`, `claude` and `pendingyou`, each a `sh` script that notes its call and answers what the test put beside it. The test lifts them into a module of `tests/common` that both files use, and changes no test of setup's. The set's Flume and sluice are the paths `BOOMDRIVE_SET_FLUME` and `BOOMDRIVE_SET_SLUICE` give, read as `tests/family.rs` reads them. A test that runs setup is macOS's, `#[cfg(target_os = "macos")]`, since setup runs on macOS alone and the gate runs there. The others run anywhere.

- **Bound to setup's names.** Three steps would only reach a stand-in if run: the sign-ins, the state repository and the start. Each test runs setup over stand-ins and holds the step's lines to what setup names:
  - `the_sign_ins_are_the_ones_setup_names`: gh signed out gives setup's refusal, which names its fix. Claude Code and Pending You signed out give the answer's `missing` fixes. The file is those three, in that order.
  - `the_state_repository_is_made_as_setup_makes_it`: gh finds no state repository, and the `gh repo create` call setup made is the file's one line.
  - `the_drives_agents_start_as_setup_says`: the `--json` answer's `start` is the file's one line.
- **Run as written.**
  - `setup_sets_up_the_drive_as_written` runs `mac/setup.sh` and reads the runner's file and crew's table it left.
  - `flumes_file_opens_agents_in_herdr` writes `mac/flume.toml` and `mac/crew-source.toml` where the reader writes them in the scratch home, with crew's roles file as crew's follower writes it, and reads the set's Flume's `--json status`. No crew runs in the gate and crew has no command that writes its roles file alone, so the test writes it in crew's stead, in crew record 0057's form.
  - `sluices_file_lands_on_main` writes `both/sluice.toml` and `both/flume.toml` into a scratch clone of a bare origin, where the reader writes them, and runs `both/land.sh` there. In a second clone, it asks the set's sluice to read `main`'s policy by the command sluice's reference at the set's tag gives for it. There the set's Flume launches a fix with its stage held, so no agent starts: Flume reads the binding at the default branch alone, and the launch lands and takes its base from the merge queue, where without the binding Flume refuses it.

### 6. The last step's test runs a change from crew's records to main

`a_change_the_drive_is_given_lands` is the drive at work.

- **Its harness.** It is `tests/control.rs`'s: a scratch forge, crew's records written by the `records` module of `boomdrive-crew`'s tests, and the runner run as a program. It differs in two things: the set's Flume and sluice by their paths, never stubs, and a stand-in agent. It runs on macOS, where setup writes the runner's file.
- **The repository.** The bare origin of `your-github-name/your-repository`, reached as the scratch forge's, holds `main` as `landing` leaves it: `both/sluice.toml` and `both/flume.toml` committed by `both/land.sh`.
- **The state repository.** It holds, on `home/main`, one launch request of kind `fix` for that repository, naming this host, as `crew main fix` writes it: a fix of `main`, its recipient the role `home/ops`, its submission held for the reader's send.
- **The runner's file.** It is the one setup writes for the quick start's answers, run over `common::stand`'s stand-ins, its crew table naming the scratch state repository and the stand-ins' host.
- **Flume's files.** They are the Mac's Flume step's, `mac/flume.toml` and `mac/crew-source.toml`, as they stand, with crew's roles file in crew record 0057's form, which the test writes in crew's stead: no crew runs in the gate, and crew has no command that writes the file alone. A local layer, `config.local.toml`, sets the drive's stream's `multiplexer = "printed"`, the one key the gate needs, since it runs no multiplexer's server (site.8), so Flume prints the agent's command. The test runs each command Flume prints, as a reader runs a printed line, and its `claude`, the program Flume runs where no pool names one, is a link to the stand-in agent.
- **The stand-in agent.** It is a binary of the runner's crate, `boomdrive-stand-in`, from `crates/boomdrive/tests/stand-in/claude.rs`, built only under the crate's feature `stand-in`, as Flume's `flume-stand-in` is built only under Flume's `tools`. Flume takes a stage's start only from a hook whose parent process runs the program the place's pool names (Flume's `hook.rs`, `compare`), so the stand-in runs the hooks Flume's plugin names in its `hooks/hooks.json` as its own children, as Claude Code runs them: SessionStart, UserPromptSubmit with the prompt it was given, then, once it has written and committed one file on the place's branch, Stop and SessionEnd. `boomdrive:test` names the feature, and the crate's lint and type check, `--all-targets --all-features`, read the stand-in as they read the runner. The flake's package names no feature, so no release builds it. Not taken: a source the test compiles with `rustc` at test time, which the gate's format, lint and type check would never read. Not taken: the test committing for the agent, or a `sh` stand-in, whose hooks Flume takes no start from, so the stage starts again.
- **The login's PATH.** Flume runs a place's steps, sluice's `equip` and `release` among them, under the PATH the user's login shell sets, which a reader's nix profile gives the set. The scratch home's `.profile` puts the set's Flume and sluice first on that PATH.
- **The passes.** The test runs the runner's passes, each ending with Flume's settle (core.13). Once the runner's branch carries the fix's code delivered and its submission held, the test appends crew's send of it, a retry of the launch naming its submission, as the reader's word appends it. The passes run until the runner's branch carries the place landed, or a bound of two minutes passes.

It holds that origin's `main` has the change, and that the runner's branch carries the place's status as the set's Flume gives it, of `flume/fix@1` and the runner's mark, landed as origin's `main`. It then runs `both/watch.sh` under the same scratch host. It reaches `flume open` for the drive's stream's session, which the set's Flume refuses in its own words, exiting 1, since the printed command holds no session a window could show. A window is what the gate cannot reach (site.8), so no stand-in for herdr's server stands in for it.

Where a tool of the set cannot carry this as its tag stands, the code stage stops short with found work naming the tool and what it lacks. A stub of that tool would break site.8 and core.45. Not taken: a test from the runner's launch alone, which leaves the landing, the brief's end, unproved.

### 7. site:test holds the steps to their tests

`check-site.sh` gains the quick start's checks, read from the built page, the step records, `site/src/start/`, and the two test files:

- the page's lede, then `On a Mac`, then `On NixOS`, each step numbered in order, and no `script` element;
- each platform's places running 1 to their count, each once;
- each file's text, escaped as HTML, as one block under its step, its `writes` before it;
- each `boomdrive` test as `fn <name>(` in `crates/boomdrive/tests/quick_start.rs`, and each `nix` test as `"<name>" =` in `nix/tests/quick-start.nix`;
- every file of `site/src/start/` but `.shellcheckrc` named by a step;
- no drive or machine of `contexts.yaml` on the page, in a record or in a file;
- no step's block with a `style` attribute, and the built stylesheet giving its block `overflow-x: auto`;
- the masthead of every page linking `/boomdrive/start/` as `Start`.

Its frontmatter is read with the YAML package the site holds, through `bun`, as the theme's family is read. `site:test`'s inputs gain `src/content/start/**/*`, `src/content/pages/start.md`, `src/start/**/*`, `/crates/boomdrive/tests/quick_start.rs`, `/nix/tests/quick-start.nix` and `src/content/contexts/contexts.yaml`. Not taken: the build reading the test files, which would make the site's build depend on the runner's and the flake's trees.

### 8. The nix tests evaluate the Nix steps and the Mac's install

`nix/tests/quick-start.nix` holds five cases, reading each file by its path, `../../site/src/start/…`:

- **`quick start: the Mac's install`.** It splits `mac/install.sh` into words: `nix profile add`, then exactly the nine installables. Each `github:` one names the address README's install names, read as `cache.nix` reads it, and a package of `t.outputs.packages.aarch64-darwin`, `default` for none. Each `nixpkgs#` one names a package of the lock's nixpkgs for `aarch64-darwin`.
- **`quick start: NixOS's flake`.** It imports `nixos/flake.nix`. Its `inputs.boomdrive.url` is README's address. Its `outputs` are called with a stand-in `nixpkgs`, whose `lib.nixosSystem` gives back its arguments, and a stand-in `boomdrive`, whose `nixosModules.default` is a marker. `nixosConfigurations.your-host.modules` is then `./configuration.nix`, the marker and `./boomdrive.nix`, the last the drive step's file by its path.
- **`quick start: NixOS's drive`.** It evaluates `t.nixosSystem` with the user `you` and `nixos/boomdrive.nix` imported. The runner's rendered file names the account `me`'s drive `home` keeping `your-github-name/your-repository`, under crew with host `your-host`. crew's drive `home` names `your-github-name/drive-state`, `your-host` and the drive's repository, the file itself sets nothing of Flume's, Flume's `settings.host.multiplexer` is `herdr`, Flume is given the stream `home` and the repository's `stream = "home"` (`drives.nix`) and the source `crew` (`sources.nix`), and the substituter and key are those the root's `flake.nix` `nixConfig` gives.
- **`quick start: the Mac's crew source`.** `mac/crew-source.toml`, read by `builtins.fromTOML`, is the drive's system's `services.flume.sources.crew`, key for key, so the file a Mac's reader writes and the source the module gives never drift.
- **`quick start: NixOS's switch`.** `nixos/switch.sh` is `sudo nixos-rebuild switch --flake /etc/nixos#your-host`. The drive's system's `config.system.build.toplevel.drvPath` evaluates; the test gives it a root file system and a boot loader where `t.nixosSystem`'s base has none.

`nix/tests/default.nix` imports it, and `nix:test`'s inputs gain `/site/src/start/**/*`.

### 9. The steps' files are claimed by the tree's formatters

- `.nix` is nixfmt's and deadnix's, as every Nix file is.
- `.sh` is shellcheck's and shfmt's. A step's file holds no shebang, since the reader pastes its lines. `site/src/start/.shellcheckrc` holds `shell=sh`, which shellcheck reads from a file's folder upward. It is excluded from treefmt by name, with its reason, and `site:test` leaves it out of the files a step must show.
- `.toml` under `site/src/start/` is taplo's: `settings.formatter.taplo.includes` gains `site/src/start/**/*.toml` beside `.prototools`.

`root:format` holds all of it (core.7).

### 10. README names the quick start

README's "Installing with nix" opens with a line naming the quick start, `https://agentplot.github.io/boomdrive/start/`, as a stranger's way from nothing. `tests/readme.rs` gains `the_readme_names_the_quick_start`. README's examples keep their own tests: the quick start does not replace them, and the two share the flake's address, which `nix:test` holds equal.

## Risks / Trade-offs

1. **[The last step's test crosses three tools]** It runs the set's Flume and sluice as tagged, so a gap in either fails it. Flume takes a stage's start only from its agent's own hooks, it refuses crew's launches unless the host declares crew's source and the drive's stream, and sluice must land with the scratch policy's executor. → A gap is found work for that tool's repository, named by the code stage's stop-short (Decision 6). It is not stubbed, so the page never claims a landing no test saw.
2. **[crew's module renames its settings]** The bolt `the-sets-crew-pin-takes-its-new-names` renames crew's module settings. `nixos/boomdrive.nix` is written by this branch's names. → When either branch takes the other, `nix:test`'s `quick start: NixOS's drive` fails until the file follows, which is the point of the test.
3. **[The opening moves the address]** At the opening, the flake's address becomes `boomdrivedev/boomdrive` (record 0025, its amendment of 2026-11-09). → `nix:test` holds the quick start's address to README's, so the change that rewrites README's rewrites both. The site's own address, which `tests/readme.rs` reads, moves with it.
4. **[A Mac whose nix does not trust the reader]** nix uses a flake's substituter only for a trusted user, so the reader may build the set from source. → The words say so, and README's install with nix says how a host trusts the cache. Distrust costs only time.
5. **[A Flume file by hand]** A Mac's reader writes one file that a module writes on a host Nix manages. → Record 0043 names it, and principle 11 retires it: the change that has setup run Flume's command removes `mac-flume`, its file and its test.
6. **[A NixOS system evaluated whole]** `quick start: NixOS's switch` evaluates a whole system's derivation, the slowest case of `nix:test`. → It evaluates and never builds, as the gate's nix must (record 0020), and it is one system.
7. **[Placeholders copied as written]** A reader who keeps `your-github-name` reaches GitHub with a name not theirs. → The words name each placeholder before its step, and gh's refusal names the repository it could not reach.
8. **[The page waits for the opening]** The site's Pages deploy waits on the opening (record 0025, its amendment of 2026-11-09). → The page is built and tested at every gate, and publishes with the site.
