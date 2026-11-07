# The flake's configuration holds streams and repositories

Research, opened 2026-11-06. An exploration talk with the user on what a
host's configuration through the Boomdrive flake says: its repositories,
the stream each runs in, its streams, and its accounts and pools.

## The question

The user, 2026-11-06, looking at the flake on main: "we need to have an
exploration session to decide what the externalized configuration should
look like, do another pass because it just has one flat list of repos,
but we should be able to configure the streams and the repos as well",
and: "my machine, in my current setup that we're trying to replace, has
[...] 4 streams".

## What stands

- **The flake's module** (`nix/modules/options.nix`, record 0020):
  `services.boomdrive.settings` is the runner's file, its
  `repositories` a flat list of addresses, `owner/name`. Nothing in the
  flake writes Flume's file.
- **A repository is named twice on the host it replaces.** The runner's
  file lists 27 addresses. Flume's file gives each a stream by its
  folder, `[repos."<folder>"] stream = "…"`, 81 entries: each clone of
  the runner's, and each view of the former layout and its `main`.
  swancloud writes both (`lib/claude-account-tools.nix`, `repoStreams`;
  `lib/flume-streams.nix`).
- **Flume names a repository by its folder.** A launch names the
  repository's folder, and a place's repository is the clone's common
  git directory (Flume's statement C.1; flume record 0053). So a writer
  of Flume's file must know where the runner made each clone, which is
  why the bolt `clones-are-found-from-boomdrive` queues
  `the-runner-reports-each-repository-it-keeps-and-its-clones-folder`
  and `the-flake-names-each-kept-repositorys-clone-folder`.
- **The rules leave a stream nowhere in the flake.** core.5 keeps every
  stream and kind out of the runner's file (the user, 2026-11-01:
  "Kinds are flume's concern, not the runner's or a control plane's");
  core.9 has the runner name a stream only where a launch names one;
  core.18 and core.46 have no module of the runner's write another
  tool's file.
- **"Managed" means opposite writers in two tools.** The runner's
  managed level is the one a hosted control plane writes over the
  host's own, which nix writes (record 0009). Flume's managed layer is
  the one the host's manager, nix or a fleet, writes, and its local
  layer the one the operator or a control plane writes over it (flume
  record 0032; Flume's statement C.7).

## How Flume would find a repository by its address

The runner's clone records its address in itself: its
`remote.origin.url` is `https://github.com/<owner>/<name>.git` (the
clones spec). A launch hands Flume the clone's folder, as it does today,
and Flume already reads that clone's `origin` to cut a place (C.1). So
Flume can read the address from the clone it is given and match it
against `[repos."<owner>/<name>"]`, then `[repos."<owner>/*"]`. Flume
asks Boomdrive nothing and knows no location of Boomdrive's: the address
is a fact of the clone, the same for a clone made by hand. A repository
with no origin on a forge keeps being named by its folder.

## The user's words

- 2026-11-06, on a stream given by owner, `"agentplot/*".stream =
  "agentplot"`, with `afterthought/paymaster` given its own: "I like the
  star stream mapping, and using the paymaster as an explicit example is
  good. Likely I would just move the paymaster repo to MadSwan-dev so we
  would just have them all be stars."
- 2026-11-06, asking: "the repository is once under BoomDrive, right? It
  is in Flume's config as well. Are you saying that currently Flume
  stores a folder path but we would replace it with the address? How
  would it end up resolving? Would Flume become responsible for taking
  that and resolving it through BoomDrive, or through a known BoomDrive
  location or something?"
- 2026-11-06, answering four questions:
  - A repository's stream: "One block by owner", each owner once with
    its stream and its repositories' names, which the flake splits into
    the runner's addresses and Flume's `<owner>/*` rule, chosen over the
    runner's list beside a block of Flume's own:

    ```nix
    services.boomdrive.repositories = {
      agentplot = {
        stream = "agentplot";
        names  = [ "boomdrive" "flume" "sluice" ];
      };
      afterthought = {
        stream = "afterthought";
        names  = [ "crew" "swancloud" ];
      };
    };
    ```

  - Flume's module: "Flume's flake, re-exported": Flume publishes a
    `services.flume` module in its own flake, the Boomdrive flake
    imports it into its modules, and carries it until Flume's flake
    does, as it carries Flume's package. Chosen over the Boomdrive flake
    owning it, and over each host writing Flume's file.
  - Accounts: "Yes, all of Flume's file": the module writes the whole
    file nix writes, host, streams, pools with their accounts, kinds and
    owner rules, the fleet passing its accounts as option values.
  - deneb's four streams: "three plus wldn. madswan=madswan-dev. wldn i
    will setup so i can track the repos, the the ec2 box will configure
    the flake for just willdangroup/*". So `agentplot`, `afterthought`,
    `madswan` and `wldn`; the owner `madswan-dev` runs in `madswan`.
- 2026-11-06, reading the sketch of deneb's configuration, which gave
  each account the `usage` file swancloud's reader writes: "That usage
  setting comes from our SwanCloud homegrown process that writes it out,
  and that is not [...] appropriate. It should be Flume's responsibility
  to read the usage, not expect it from some external third party."
- The same message: "I want to make sure the flake also packages and
  configures crew and sluice. I don't see any sluice configuration."
- The same message: "I want to confirm with Nick: as he is writing these
  settings, once we have the cloud control plane, can a repository get
  added from the cloud and written into the other config file?"
- The same message: "I don't want to forget the finding you had at the
  beginning: we have a mismatch in terminology between sluice and Flume
  in terms of what we call a managed config." (The mismatch found is the
  runner's and Flume's; sluice has one file of the host's own settings
  and no layers yet, and record 0009 asks it for a managed level.)
- The same message, on one flake: "I thought we were going to create one
  flake for people to use from Boom Drive so they didn't have to set up
  two flakes. I thought that would encapsulate everything underneath
  Boom Drive. I agree that you should set up a flake for Flume as well
  so Flume could be used independently but I want to make sure that
  it's a one-stop shop with the Boom Drive flake".
- The same message: "no wldn agents on deneb".
- The same message, on `willdangroup/*`: "I'm fine with listing the
  names and in fact I think that's preferable."

## What sluice and crew keep on deneb

- sluice: `~/.config/sluice/config.toml`, written by home-manager, the
  host's own settings of sluice's statement (clauses 71, 35): `[runs]`
  `cores`, `max` and `reserved`; `[[caches]]` with their `env`;
  `[peers] name`; `[forge]` `app_id` and the path of the App's key;
  `[signing]` the path of the signing key and its `signer`. No layers.
- crew: `~/.config/crew/teams.json`, written by home-manager: each
  partition's `label`, `machine`, `state` repository, `blueprints`,
  `session` and roles. The runner's `[control_plane.crew]` names the
  same partitions again, each `label` and `state_repository`.
- crew, sluice and Flume are public repositories.

## herdr and a way in

The user, 2026-11-06: "Is there a way to make it so that when somebody
installs this, it could also install Herdr and maybe give them one easy
thing to click to open up the preconfigured Herdr instance? either that
or have a shell they drop into that is preconfigured to launch herdr or
something".

What stands:

- herdr is the host's own today: record 0020 keeps "its git and herdr"
  the host's, and core.44 rules herdr out of the set (record 0012). The
  set carries Flume's and sluice's herdr plugins, from their tags.
- deneb's herdr, 0.9.5, comes from the fleet's flake (swancloud), which
  takes it from a flake of agents' tools; herdr's own repository is
  `herdrdev/herdr`.
- Flume already opens a stream's agents for a person: `flume open
  [<session>]` (Flume's statement 186) starts the stream's herdr session
  where it does not run, and opens a window of the host's terminal app,
  `terminal_app`, attached to it, or prints the line to run where none is
  named. A launcher that runs it is the person's asking; the runner
  still starts no multiplexer (core.13) and names none (core.29).

The user's answers, 2026-11-06:

- herdr: "In the set, overridable": the flake installs herdr at the
  version Flume's terminal profile was checked against, with the family's
  herdr plugins, and a host replaces or drops it through the module's
  options. Chosen over herdr staying the host's own.
- The way in: "An app to click" and "A shell": a Boomdrive app on macOS,
  a desktop entry on Linux, each running `flume open`; and a shell from
  the flake with the set on its `PATH` that starts herdr. Not chosen: a
  `nix run` of the flake with nothing installed.
- The terminal app: "Install Ghostty too": the flake installs Ghostty and
  names it Flume's `terminal_app`, so a fresh Mac opens a window with
  nothing else installed. Chosen over the host's own terminal app.

## The layers

The user, 2026-11-06, on "I want to confirm with Nick": "i don't konw who
Nick is. did I say nick when i meant nix?" Read as Nix: "as Nix is
writing these settings, once we have the cloud control plane, can a
repository get added from the cloud and written into the other config
file?" As the records stand, yes: the cloud writes the runner's managed
level (record 0009) and Flume's local layer (flume record 0032), each
beside the file Nix writes. The two records also differ in how: record
0009 has setup set Flume's streams through Flume's commands (core.18),
and flume record 0032 has no command of Flume's write configuration and
a control plane write the local file.

The user's answers, 2026-11-06:

- The words: "Flume's: managed + local". Every tool's `config.toml` is
  its managed layer, written by Nix, a fleet or a person, and
  `config.local.toml` its local layer, written by the operator or a
  control plane over it; the runner's managed level of record 0009, not
  yet built, becomes its local layer. Chosen over the runner's words,
  host and managed, and over new words naming each file's writer.
- What a local layer may set: "Flume's rule for all": each tool lists
  the keys its local layer may set, and the managed layer may pin them.
  Chosen over the runner's cloud level overriding any key.
- Repositories under the cloud: "Add to Nix's": the host keeps every
  repository Nix names and every one the cloud names for its drives.
  Chosen over the cloud's list replacing Nix's (core.17 as it reads).
