# Design

## Context

See proposal.md for why. There is no area page for `crates/`, `nix/` or `README.md`, so `docs/architecture/core.md` binds every path this change touches. The change relies on these rules:

- core.1: the runner decides nothing a tool decides. Setup passes crew's answer as crew printed it, and gh's, herdr's and `launchctl`'s words as each gave them;
- core.2: no code outside an implementation names crew. Every step that names crew is `boomdrive-crew`'s, reached through `planes` (Decision 11);
- core.4: no token in the repository. Setup handles none: gh holds the person's GitHub sign-in, and git's credential helper answers crew's fetch;
- core.5: the runner keeps what its configuration names. Setup writes the configuration, and the runner keeps and lets go by its own rules;
- core.12: every key names what it holds. The flags `--state-repository`, `--add-repository` and `--remove-repository` name repositories, and `--blueprints`, `--runs` and `--review` are crew's own words for crew's own things;
- core.13: the runner starts no tool's long-running process. Setup loads the runner's own agent into the host's service manager, links herdr's plugins, which starts no server (record 0034, point 1), and names `crew main up <drive>` without running it;
- core.16: setup is one of the runner's own commands;
- core.17 and core.18: the runner is configured by its configuration alone, and writes no other tool's file. Setup writes the runner's own managed layer. crew's file is written by `crew drive set`, Flume's and sluice's by nobody (Risk 2);
- core.39: every command that answers takes `--json`, and exits 0, 1 or 2;
- core.46: the modules' agent and their moving aside of a person's file, which setup's are made as;
- core.57: crew is given each kept repository's clone and view as the runner keeps them;
- core.41: every test runs on a scratch home under neutral names.

It departs from core.29, whose text says no code of the runner's runs a multiplexer's command line. Setup runs `herdr plugin link` and `herdr plugin list`, because record 0009's amendment of 2026-11-09 has setup link herdr's plugins "with herdr's own command", as the modules' activation does. Neither reaches a terminal or a pane. core.29's text gains setup's link, and core.16's, core.18's and core.28's details gain what setup does (Decision 12).

**The decision.** Record 0009's amendment of 2026-11-09. The route and its timing are the user's: "No: Nix installs the programs, a setup command does the rest", "With Boomdrive going public". The rest is the design agent's:

- one command, each question a flag, answering in JSON, which changes nothing run again;
- the person gives their account, their drive, its repositories by owner and its crew state repository, and setup makes the state repository where none exists;
- setup leaves the runner as the login's service, each tool's managed layer written by the keys Nix writes, herdr's plugins linked by herdr's own command, and crew's file naming the drive and its state;
- a person's own file is moved aside once and named;
- adding or removing a repository is setup's;
- on a host Nix manages, setup writes nothing Nix writes and says so;
- sign-ins stay the person's, checked and named.

**crew's side.** crew record 0065 gives `crew drive set <label>`, "as Boomdrive's setup runs it on a person's Mac once it has made the state repository on GitHub". It is declarative: the drive's entry is replaced whole, and run again with the same flags it writes nothing. Every key is a flag with no default. Its consequence names what setup passes: "a session named for the drive, herdr's folder for it, the login's Claude account for each role, and the runner's folders for each kept repository". crew record 0064's amendment of 2026-11-09 replaces `flume` with `runs` and the review's `every` and `hours` with `after_changes` and `after_hours`. crew refuses `--flume` as an argument it does not take. crew v0.59.0 carries all of it.

**What the modules write.** On nix-darwin, Boomdrive's module writes the runner's file from `services.boomdrive.accounts` and `settings` (`nix/modules/accounts.nix`, `toml.nix`). It declares `launchd.user.agents.boomdrive` (`darwin.nix`) and links `~/.config/boomdrive/config.toml` to the file, moving a person's own aside to `config.toml.before-nix` (`activate.sh`). It also links the set's two herdr plugins by `herdr plugin link`, `HERDR_` unset and standard input closed. `nix/tests/files/crew.toml` is the file it renders for one account's drive under crew.

**Two findings that shape the design.**

- Each of the set's herdr plugin packages is its tool's `plugins/herdr` folder at its top, and both have `bin/open.sh`. One Nix profile cannot hold both, and neither the `flume` nor the `sluice` package carries its plugin (Decision 8).
- Flume with no file of its own runs the printed command as its multiplexer, `PRINTED` in Flume's `host/config.rs`. herdr and Ghostty come from Boomdrive's module's `mkDefault`s in `nix/modules/apps.nix`, and Flume has no command that writes its file (Risk 2).

## Goals / Non-Goals

**Goals:**

- A person who ran `nix profile add` for the set runs `boomdrive setup` once, or has an agent run it, and is left with:
  - the runner running for their login, keeping the drive's repositories under crew;
  - crew's file naming the drive and its state;
  - herdr's plugins linked;
  - `crew main up <drive>` to start crew's agents.
- Every question is a flag. A terminal is asked what no flag gives. `--json` answers in one object. Run again with the same answers, nothing changes.
- A repository is added or removed by setup, never by a hand edit.
- On a host Nix manages, nothing is written.

**Non-Goals:**

- Flume's files. Setup writes none (core.18), and Flume has no command that writes them (Risk 2).
- sluice's file. Nix's module sets no key of sluice's either. sluice with no file uses the host's own credential helpers, gh's on a person's Mac, until `boomdrive forge-token` is built (the bolt `the-runner-gives-every-tool-its-forge-token`).
- Linux. A NixOS host is set up by the flake's module. A Linux host Nix does not manage is for a later change.
- Removing a drive. crew has no command that removes one (crew record 0065), and record 0009's amendment asks for none.
- herdr's own `config.toml`, the person's own (record 0034, point 1). An open proposal has setup gather the lines each plugin asks for, after sluice's and Flume's plugins print them.
- The quick start on the site, which is the unit `boomdrives-quick-start-takes-a-stranger-to-a-working-drive`, after this one.

## Decisions

### 1. One command, its answers from the flags, a terminal, the host, then setup's own

`boomdrive setup` takes nine flags, listed in the setup spec. Each answer comes from the first of these that gives it:

1. its flag;
2. on a terminal, the person's answer to its question, for the four the record says the person gives: the account, the drive, the state repository and the repositories;
3. the host as setup left it (Decision 2);
4. setup's own value (Decision 3).

With no terminal, a missing answer of the four is a usage error that names each missing flag, so an agent learns every flag it must give from one run. `--add-repository` and `--remove-repository` change the repositories the host holds, and `--repository` gives them whole, so the two forms are refused together.

Not taken:

- subcommands such as `setup repository add`, which would give one drive's setup two command lines whose answers must agree;
- a wizard with no flags, which the record turns down because an agent cannot run it;
- asking each role's way of running and the review on a terminal. The record names four things the person gives, and a question for each of seven more would make the first setup a form.

### 2. What the host holds is the answer not given

The account, the drive, its repositories and the state repository are read from the runner's file, where setup wrote it and it names one drive. The blueprints, each role's runs and the review are read from crew's file, `~/.config/crew/teams.json`, where it names the drive. crew's implementation reads crew's file, so the binary names none of it (core.2). A later `--add-repository` then runs `crew drive set` with the drive's blueprints, runs and review as crew holds them. crew replaces the drive's entry whole, so anything not passed again would be lost.

Not taken:

- a file of setup's own answers in the data directory, which holds only what core.35 lists, and which could disagree with the files it describes;
- setup's own values at every run, under which `--add-repository` would set a role the person had made on-demand back to standing.

### 3. Setup's own values: the first repository, every role standing, five changes or four hours

Where neither a flag nor the host gives them:

- the blueprints are the drive's first repository, since crew needs one among the drive's repositories;
- each of the five roles runs `standing`;
- the review is `5` changes or `4` hours, the values every drive of the user's runs in crew's own test of its module, `nix/tests/files/deneb.json`.

Standing keeps each role's agent in its pane, started by crew. On demand has Flume run the role in the drive's own stream, and setup writes no Flume stream (Risk 2). A person who has the stream may still give `--runs <role> on-demand`.

Not taken:

- `on-demand` by default, under which a role would run nowhere on a host whose Flume has no stream for the drive;
- no value at all, which would turn three answers the record does not ask the person for into required flags.

### 4. crew is given the Mac's own values

Setup runs `crew drive set <drive> --json` once, with the flags in the order the setup spec lists, and never `--flume`, `--roles` or `--console`:

- `--machine` and `--hosts <host> <host> darwin` are this host as `hostname -s` gives it. crew compares the machine with that, and on a Mac the name ssh reaches is the same.
- `--session` is the drive's name.
- `--folder` is the runner's repository root as an absolute path. Each repository's own folders come from `--repositories`.
- `--accounts <role> <account>` names the account for every role. For crew a Claude account is a name, and one person's Mac has one sign-in, so the account's name names it. Two roles in one session must name one account, and all five do.
- `--repositories <repository> <dir> <checkout>` gives each repository's clone's folder under the runner's data directory and its view under the repository root, from the runner's own layout (core.57; record 0036).

Not taken:

- a flag for the Claude account's name, one more answer the record does not ask for;
- `--folder` at a folder of crew's own, a folder no repository of the drive lives in.

### 5. The state repository is made by gh, private

crew does not make the repository on GitHub. Its record 0065 has setup make it first. crew's implementation runs `gh repo view <owner/name> --json name`, and where gh says it cannot resolve the repository, `gh repo create <owner/name> --private`. It is made under the owner the person names, their own account or an organisation they belong to. gh refuses any other, and setup carries its words. GitHub's sign-in is checked before anything is written (Decision 10), so a missing sign-in never leaves a half-done setup.

Not taken:

- a call of the runner's own to GitHub's API, which would need the person's token in the runner (core.4) and a call core.28 does not allow;
- asking the person to make it by hand, which the record turns down;
- a public repository, since a drive's plan and run record are the person's.

### 6. The runner's file is Nix's, byte for byte, but its first line

Setup renders `accounts`, each account's drives and repositories, and `[control_plane]`:

- `type` is the word of the implementation that sets up a drive;
- its table is the one the implementation gives: crew's `host` and a partition for each drive, its `label` and `state_repository`.

The repositories are ordered as Nix's module orders them, owner by owner in the order of the owners' names. Top-level keys come before tables, each sorted, and strings are TOML's basic strings, as `toml.nix` writes them.

`nix/tests/settings.nix` gains `setup`, one account's drive under crew with nothing else set, and `nix:test` holds its rendering to `nix/tests/files/setup.toml`. Setup's own test holds setup's file to that file past the first line. So the two writers cannot drift without a test failing.

The first line, `# Written by boomdrive setup; a change goes through boomdrive setup.`, is how setup knows its own file. Setup reads answers only from its own file. It moves any other file aside to `config.toml.before-setup`, the suffix naming its writer as crew's `teams.json.before-drive-set` does, and refuses where that path is taken, as the modules' activation does. It writes the file whole, by a rename, only where the bytes differ, and reads it back with the runner's own reader before it loads the agent.

Not taken:

- the local layer, which record 0009's amendment of 2026-11-06 leaves to a control plane. A person's drive is their managed layer, as Nix's is on a host it manages;
- the `toml` crate's own rendering, whose layout is not `toml.nix`'s.

### 7. A host Nix manages is refused, untouched

Where `config.toml` is a link, as the modules' activation leaves it, setup refuses before running anything, naming `services.boomdrive.accounts`. It does not go on to make the state repository or run crew. On such a host crew's file is Nix's too, and `crew drive set` refuses a linked file whose drive differs from its flags.

Not taken: running the steps that write nothing of Nix's. A person would get a state repository and a refusal from crew, and nothing they could use.

### 8. The set gives its plugins in one package, and setup links them from the profile

The flake gains `herdr-plugins`, a `linkFarm` of `share/herdr/plugins/flume` and `share/herdr/plugins/sluice`, each the existing plugin package. README's install line adds it and `#herdr`.

Setup looks under `share/herdr/plugins` of `~/.nix-profile`, else `~/.local/state/nix/profile`, where `nix profile` keeps a profile under XDG's layout. It links each folder there by its path through the profile, so a `nix profile upgrade` moves the plugins with the tools. A folder is skipped where `herdr plugin list --json` gives a plugin whose `plugin_root`, resolved, is the folder resolved. Run again, setup links nothing.

Not taken:

- `herdr plugin install agentplot/<tool>/plugins/herdr --ref v<version>`, which fetches each plugin from GitHub rather than linking the set's;
- `nix build --print-out-paths` run by setup, which builds what the profile already holds;
- moving each plugin package's files under `share/`, which would change the path the modules link on every host.

### 9. The agent is nix-darwin's, under setup's own label, running the profile's runner

`~/Library/LaunchAgents/io.github.agentplot.boomdrive.plist` has nix-darwin's agent's keys (the service-modules spec): the wait on `/bin/wait4path /nix/store`, `RunAtLoad`, `KeepAlive`, `ThrottleInterval` 30, `ProcessType` `Interactive`, `AbandonProcessGroup`, `HOME` and `PATH`, and the log `~/Library/Logs/boomdrive.log`.

- **The program** runs `boomdrive` by name, which the `PATH` finds in the person's profile. The plist does not change when the profile moves, so a second run after an upgrade writes nothing. The new runner runs from the agent's next start (Risk 4).
- **The `PATH`** is fixed: the person's profiles, Nix's default profile, `~/.local/bin`, where Claude Code installs itself, `/opt/homebrew/bin`, where a Mac's gh and git often are, `/usr/local/bin` and the system's folders. It holds no folder of nix-darwin's, which such a host lacks.
- **The label** is `io.github.agentplot.boomdrive`, under the project's own public address, and not nix-darwin's `org.nixos` prefix.

Setup loads the agent into `gui/<uid>`. It bootstraps where `launchctl print` finds none. Where the plist changed it boots the agent out first. Where neither holds it runs nothing.

Not taken:

- the runner's store path in the program, as nix-darwin names it, which a profile's garbage collection may remove under the agent and which changes the plist at every upgrade;
- setup's own `PATH`, which differs with the shell or agent that runs setup and would make two runs disagree.

### 10. Sign-ins: GitHub's before anything, the others named

GitHub's is needed for setup's own steps: gh makes the state repository, and crew's fetch and push go through git's credential helper. So `gh auth status --hostname github.com` failing refuses setup before anything is written.

Claude Code's and Pending You's are needed by crew's agents, not by setup. Each missing one is named in the answer with how the person signs in, and setup goes on:

- Claude Code's is `loggedIn` in `claude auth status --json`;
- Pending You's is the `Sign-in:` line of `pendingyou status`. That command exits 1 whenever any of its lines is missing, so its exit cannot tell (Risk 6).

Not taken: refusing on any missing sign-in, which would hold the runner and crew's file back for a sign-in only the agents need.

### 11. The interface gains a drive's setup, which crew's implementation carries

`planes::Entry` gains `set_up`, beside `enroll`: crew's is `boomdrive_crew::setup`, and the hosted implementation's is none. What setup knows is split on core.2's line.

The binary's `setup.rs` keeps what is the runner's:

- the account, the drive and its repositories, whole or added and removed;
- the host's checks: macOS, a file Nix links, a file to move aside;
- the sign-ins;
- the runner's file;
- the plugins;
- the agent;
- the answer.

It passes `--state-repository`, `--blueprints`, `--runs` and `--review` to the implementation as given. crew's implementation keeps what is crew's:

- **What its tool holds:** the state repository from its own table in the runner's file, and the blueprints, runs and review from crew's file, `~/.config/crew/teams.json`, as crew reads it.
- **Settling its answers:** it checks the forms, the blueprints among the repositories, the roles and the review, and fills each answer not given from what is held, then from its own values.
- **Setting up:** it reads `hostname -s` and refuses a name crew does not take. It makes the state repository by gh and runs `crew drive set`. It gives back its table for the runner's file, and its answer, which holds the state repository, crew's answer and `start`.

Every program either runs is found on setup's `PATH` by one finder the binary hands the implementation. A test hands stand-ins. An implementation's refusal says whether the host must be fixed, as an enrollment's does, for exit 2, or the tool refused, for exit 1. `tests/rules.rs`'s `only_crews_crate_names_crew` holds `planes.rs` to naming crew's setup, and every other file of the binary to naming no `boomdrive_crew`.

### 12. Record 0009's amendment of 2026-11-11, and core.5, core.16, core.18, core.28 and core.29

Record 0009 gains an amendment, the writing agent's, recording Decisions 1 to 11 with the path not taken for each. Its consequence names what Flume is asked for (Risk 2). In `docs/architecture/core.md`:

- core.29's text gains "but setup's link of the set's herdr plugins by herdr's own command";
- core.5's details allow the managed layer written by setup with the repositories of the drive a person names;
- core.16's details allow setup, run on the host, writing a repository added or removed into the configuration;
- core.18's details allow setup writing the runner's own managed layer on a host Nix does not manage;
- core.28's details allow setup running gh and crew, the person's own tools under the person's own sign-ins.

Each cites the record.

## Risks / Trade-offs

1. **[The set's crew has no `drive set`]** The flake pins crew v0.50.2, whose crew has no `drive set` and reads version 5 of its file. → The bolt `the-sets-crew-pin-takes-its-new-names` moves the set to crew v0.59.0. This change's tests run stand-in programs, never the set's crew. Until that bolt lands, a person's crew answers `drive set` with its own refusal, which setup carries as crew's words.
2. **[Flume on such a host runs no multiplexer]** Flume with no file runs the printed command, under which no agent of a place is kept open. On a host Nix manages, Boomdrive's module gives Flume `herdr` and `ghostty`, the drive's stream and each repository's stream. Setup writes none of Flume's files (core.18), and Flume has no command that writes them. So until Flume has one, a person's drive set up here has crew's standing agents to talk to, and the runner's launches into Flume print their commands rather than run them. A role run on demand has no stream. crew's source, `~/.config/flume/sources/crew.toml`, which no module writes yet either (record 0034, point 2), is missing too. → Record 0009's amendment of 2026-11-11 names, as its consequence, what Flume is asked for: a command that sets its managed layer's host keys, a drive's stream and a source, which setup runs, as crew was asked for `crew drive set`. That is the reach of record 0009's own first consequence. The unit after this one, the quick start, needs it before a person's agents can build and land a change.
3. **[A person's file moved aside leaves its repositories]** A runner file a person wrote by hand is moved aside, and the runner keeps only setup's drive from then on. → The answer names both paths. The record asks for the file moved aside as the modules move it.
4. **[An upgrade waits for the agent's next start]** After `nix profile upgrade` the runner keeps running the old binary until the agent restarts. → `launchctl kickstart -k gui/<uid>/io.github.agentplot.boomdrive` restarts it. The quick start can say so. A later change may have setup restart an agent whose running binary is not the profile's, as NixOS's activation does.
5. **[A Mac moved to nix-darwin keeps setup's agent]** Once nix-darwin manages the host, setup refuses there and removes nothing, and nix-darwin's `org.nixos.boomdrive` would run beside setup's agent. → The second runner to start finds the data directory's lock held, exits 2 and touches nothing (the running spec), and launchd starts it again every 30 seconds. The person boots out and removes setup's plist. A later change may have setup, on a host Nix manages, put its own agent away.
6. **[Pending You's sign-in is read from words]** `pendingyou status` has no JSON, and its exit says nothing of the sign-in alone. → Setup reads the one line naming `Sign-in:`, and a change to those words reads as missing. That names a sign-in the person may have, and changes no file.
7. **[A drive's repository root is the runner's default]** Setup writes no `repository_root`, so the views are under `~/Code`. → A person who wants another root has nothing to change it with in setup. It is a key Nix's module writes from `settings`, and a later change may add a flag for it.
