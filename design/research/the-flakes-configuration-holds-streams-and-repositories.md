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
