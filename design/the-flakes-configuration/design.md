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
- **A laptop** writes `services.boomdrive.repositories.<owner>.names`
  and nothing else: no stream, no pool, every place in Flume's default
  stream under the login's own program.

The sketch of deneb is `deneb.nix` beside this file.

## Open

- **Layer names.** The runner's managed level is the one a control plane
  writes over the host's own (record 0009); Flume's managed layer is the
  one the host's manager writes, and its local layer the operator's or
  a control plane's over it (flume record 0032). sluice has no layers
  yet. The family needs one pair of words for both tools, and sluice's
  when it gains layers.
- **A repository added from the cloud.** Under record 0009 the hosted
  control plane writes the runner's managed level with the repositories
  of the drives a host serves, beside the file nix writes, and the
  runner reads both. To confirm with Boomdrive Cloud: that it writes
  that file, and where a cloud-added repository's stream goes, Flume's
  local layer by its address being the one place Flume's design allows.
- **crew's partitions are named twice.** The runner's
  `[control_plane.crew]` and crew's `teams.json` each name the host's
  partitions and their state repository.
- **Flume's usage reader.** How Flume reads an account's allowance, and
  what Flume's statement C.7 keeps of the `usage` file, is Flume's
  design.

## What each repository owes

- **agentplot/boomdrive**: a record amending record 0020 (the set gains
  crew; the modules configure Flume, sluice and crew; the owner block);
  core.5's details (the owner block's stream is no key of the runner's
  file), core.18 and core.46 amended; the configuration and
  service-modules specs through an OpenSpec change. The queued units of
  the bolt `clones-are-found-from-boomdrive`, which give Flume each
  clone's folder, are no longer needed once Flume names a repository by
  its address.
- **agentplot/flume**: a flake with a `services.flume` module; a record
  and statement clauses (C.7, 143) for naming a repository by its
  address; a record and C.7's change for reading an account's usage
  itself.
- **agentplot/sluice**: a flake module for the host's own settings.
- **afterthought/crew**: a flake with its package and a module for its
  host file.
- **agentplot/blueprints**: Flume's statement, clauses 143 and C.7; the
  family's one pair of words for a configuration's layers, in the
  principles or each tool's statement.
- **afterthought/swancloud**: once the modules land, it passes deneb's
  facts as option values and stops writing Flume's, sluice's and crew's
  files and the usage readings for Flume.
