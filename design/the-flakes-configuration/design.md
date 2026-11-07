# The flake configures the whole host

Status: proposed, from the talk of 2026-11-06
(`design/research/the-flakes-configuration-holds-streams-and-repositories.md`,
which holds the user's words). Not yet a record or a rule.

## The shape

- **One flake.** A host adds the Boomdrive flake alone. Its modules,
  `darwinModules.default` and `nixosModules.default`, bring the runner,
  Flume, sluice and crew, each packaged and each configured. Flume,
  sluice and crew each publish a flake of their own with their own
  module, so each can be used without Boomdrive; the Boomdrive flake
  takes each as a locked input and re-exports its module, and carries a
  tool's module itself only until that tool's flake gives one, as it
  carries Flume's package today (record 0020).
- **Each tool's module writes its own file.** `services.flume.settings`
  is Flume's file, `services.sluice.settings` sluice's host settings,
  `services.crew.settings` crew's, `services.boomdrive.settings` the
  runner's, each key named as its file names it. No module writes
  another tool's file; the Boomdrive module may set another tool's
  module options (core.18 and core.46 amended to say so).
- **Repositories by owner, each named once.** A host writes each owner
  once, with its repositories' names and, where its work runs on this
  host, the stream it runs in:

  ```nix
  services.boomdrive.repositories = {
    agentplot   = { stream = "agentplot"; names = [ "boomdrive" "flume" ]; };
    madswan-dev = { stream = "madswan";   names = [ "paymaster" ]; };
    willdangroup.names = [ "…" ];   # kept to follow, no agents here
  };
  ```

  The module writes each address into the runner's `repositories`, and
  gives each address its owner's stream in Flume's module,
  `repos."agentplot/boomdrive".stream = "agentplot"`. The person writes
  no star and no folder; Flume's file holds one exact address per
  repository. An owner with no stream gives Flume no rule.
- **Flume names a repository by its address.** Flume reads the address
  from the clone a launch names, its `remote.origin.url`, and matches it
  against `[repos."<owner>/<name>"]`; a repository with no forge origin
  is still named by its folder. Flume asks Boomdrive nothing.
- **Flume reads each account's usage itself.** An account names its
  login's folder and its program; Flume reads the allowance left from
  the program's own source, and no host writes a reading for it.
- **herdr and Ghostty are in the set.** The flake installs herdr at the
  version Flume's terminal profile was checked against, with Flume's and
  sluice's herdr plugins in it, and Ghostty, which the module names
  Flume's `terminal` and `terminal_app` where the host names neither.
  Each is overridable, as Flume and sluice are: a host replaces either
  through the module's options, or drops it for its own.
- **One way in.** The module installs a Boomdrive app, in
  `/Applications/Nix Apps` on macOS and as a desktop entry on Linux,
  which runs `flume open`: Flume starts the default stream's herdr
  session where it does not run and opens a Ghostty window on it
  (Flume's statement 186). The flake also gives a shell, its `PATH` the
  set's tools, that starts herdr on entering. Each is the person's
  asking: the runner still starts no multiplexer (core.13) and names
  none (core.29).
- **A laptop** writes `services.boomdrive.repositories.<owner>.names`
  and nothing else: no stream, no pool, every place in Flume's default
  stream under the login's own program, and clicks the Boomdrive app to
  see its agents.

- **crew's partitions are written once, in crew's module.**
  `services.crew.settings.partitions` is crew's file, the fleet's every
  partition with its `machine`, and `services.crew.host` names this host
  to crew. Where `control_plane.type` is `crew` and no
  `[control_plane.crew]` is written, the Boomdrive module writes it:
  `host` from `services.crew.host`, and a `label` and `state_repository`
  for each partition whose `machine` is this host. A table written out
  wins. The runner's file still names its partitions (core.17), and
  neither the runner nor crew changes. Not taken: the runner reading
  crew's `teams.json`, which core.17 rules out and which ties the runner
  to crew's file versions; and asking crew by a command, which crew has
  not and which teams are leaving. A host without Nix writes both files,
  as it does today; under a hosted control plane a partition is an
  enrollment in the runner's file alone.
- **Two layers in every tool, by one pair of words.** Each tool's
  configuration is its managed layer, `config.toml`, written by Nix, a
  fleet or a person, and its local layer, `config.local.toml` beside it,
  written by the operator or a control plane, read over it. The runner
  takes Flume's words and Flume's rule (flume record 0032), and sluice
  takes both when it gains layers:
  - **The local layer sets only what runs the work.** Each tool lists
    the keys its local layer may set; for the runner, its repositories
    and its control plane's partitions. Accounts, pools, credentials'
    paths and every other key are the managed layer's alone, and a local
    file that sets one is not taken.
  - **The managed layer may pin** any key the local one could set, which
    the local layer then cannot change.
  - **Repositories add.** The runner keeps every repository its managed
    layer names and every one its local layer names. A control plane
    that drops a drive drops only the repositories it wrote.
  - **A control plane writes a tool's local file.** Setup under a hosted
    control plane writes the runner's and Flume's local layers, the
    repositories of the drives a host serves and each one's stream by
    its address, and Nix's files stay as Nix wrote them.

The sketch of deneb is `deneb.nix` beside this file.

## Open

- **Flume's usage reader.** How Flume reads an account's allowance, and
  what Flume's statement C.7 keeps of the `usage` file, is Flume's
  design.

## What each repository owes

- **agentplot/boomdrive**: a record amending record 0020 (the set gains
  crew, herdr and Ghostty; the modules configure Flume, sluice and crew;
  the owner block; crew's partitions written once and the runner's
  `[control_plane.crew]` derived from them; the Boomdrive app and the
  shell); core.44 amended,
  herdr no longer the host's own;
  core.5's details (the owner block's stream is no key of the runner's
  file), core.18 and core.46 amended; a record amending record 0009 (the
  local layer in Flume's words in place of the managed level, its keys
  listed and pinnable, repositories added, a control plane writing a
  tool's local file in place of setup through each tool's commands),
  with core.17 and core.18 amended; the configuration and
  service-modules specs through an OpenSpec change. The queued units of
  the bolt `clones-are-found-from-boomdrive`, which give Flume each
  clone's folder, are no longer needed once Flume names a repository by
  its address.
- **agentplot/flume**: a flake with a `services.flume` module; a record
  and statement clauses (C.7, 143) for naming a repository by its
  address; a record and C.7's change for reading an account's usage
  itself.
- **agentplot/sluice**: a flake module for the host's own settings;
  its layers, when it gains them, in the family's words.
- **afterthought/crew**: a flake with its package and a module for its
  host file.
- **agentplot/blueprints**: Flume's statement, clauses 143 and C.7; the
  family's one pair of words for a configuration's layers, managed and
  local, and Flume's rule for what a local layer may set, in the
  principles.
- **afterthought/swancloud**: once the modules land, it passes deneb's
  facts as option values and stops writing Flume's, sluice's and crew's
  files and the usage readings for Flume.
