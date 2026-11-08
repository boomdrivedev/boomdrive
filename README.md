# Boomdrive

The runner on each host, one binary named `boomdrive`. It keeps the
clones and views of the repositories its configuration names, and turns
the control plane's work into `flume launch` calls, carrying their
results back. It reaches the control plane through one interface,
`crates/boomdrive-control`, and carries two implementations of it.
crew's, `crates/boomdrive-crew`, reads and writes records in crew's state
repository. A hosted control plane's, `crates/boomdrive-hosted`, keeps one
outgoing WebSocket for each partition the host is enrolled in, and holds
the runner's protocol with it, one JSON Schema per version under
`protocol/`. From this tree:

    devenv shell -- cargo run -p boomdrive -- --version

## The configuration

`boomdrive` reads one TOML file: the one `--config <file>` names, else
`$XDG_CONFIG_HOME/boomdrive/config.toml`, else
`~/.config/boomdrive/config.toml`. It names the repositories the host
keeps by their GitHub address, the root their views are kept under, how
often they are fetched and whether each owner's views have a VS Code
workspace:

```toml
# The root each repository's view is kept under, at <repository_root>/<owner>/<name>; ~/Code where none is named.
repository_root = "~/Code"
# How often every repository is fetched; 2m where none is named.
fetch_interval = "2m"
# Whether each owner's views have a VS Code workspace, <repository_root>/<owner>/<owner>.code-workspace; false where none is named.
code_workspaces = true
# Each repository this host keeps, by its GitHub address.
repositories = ["agentplot/flume", "example/crew"]
```

`repositories` is required, so a host that keeps none says
`repositories = []`. Addresses are compared without case, so two that
differ only in case are refused as one repository named twice.
`repository_root` is an absolute path or begins with `~/`.
`fetch_interval` is `<n>s`, `<n>m` or `<n>h`. A key the runner does not
read is refused, so the file has no place for a credential: the host's
own git credential helper answers every fetch. A file written for an
earlier runner is refused with what to write instead: `views` is
`repository_root` now, and `fetch_every` is `fetch_interval`. Nor does
the file name a stream or a kind: those are each host's Flume
configuration, and a key naming one is refused, saying so.

`code_workspaces = true` asks for one VS Code workspace for each owner
of the kept repositories, beside the owner's views, so VS Code titles
its window `— <owner> (Workspace)`. Its `folders` are the owner's kept
repositories, in the file's order, set at every pass; the rest of the
file, the settings VS Code writes into it, is kept. A file there the
runner cannot read as a JSON object, one with comments in it say, is
left as it is and named at every pass. `code_workspaces` is `true` or
`false`, and where it is absent or `false` the runner writes and removes
no workspace.

## Running on a host

`boomdrive run` is the long-running process a host starts and keeps
alive. It makes a pass over the configuration at once, then one every
`fetch_interval`, timed from the start of the pass before, and reads the
file again at every pass. A pass that cannot read it names why and
does nothing else, so a file mid-edit never drops a repository. A pass:

- clones each repository the configuration names into the runner's data
  directory, `$XDG_DATA_HOME/boomdrive`, else `~/.local/share/boomdrive`,
  at `repositories/<owner>/<name>/<name>.git`, in the bare-container
  layout flume reads; or fetches the clone it has, over https through
  the host's own credential helper, never prompting;
- points the clone's HEAD at origin's default branch, making that
  branch at origin's tip where the clone lacks it. It moves no branch
  of the clone: it reads each at `origin/<branch>`, and the clone's own
  ref of a branch sluice keeps is sluice's copy, which sluice levels;
- keeps a view of each repository at `<repository_root>/<owner>/<name>`,
  a worktree of the clone detached at `origin/<default>`. It makes the
  view where the folder is missing and never over a folder that is
  there, and moves it only while it stands where the runner last put
  it and holds no uncommitted change, so a view you moved or work in is
  left as it is;
- lets go of each repository the configuration no longer names. It is
  never fetched again, and its clone and view are removed once neither
  holds work the forge lacks and no place the runner launched there is
  left to end; until then they are left, and named at every pass. While
  the runner's report holds a launch at a place there whose status, as
  last carried, Flume still lists and has not read tidied or gone, the
  repository is drained: nothing more is launched in it, and its place
  is followed to its end through its clone;
- where `code_workspaces` is true, keeps each owner's VS Code workspace
  at `<repository_root>/<owner>/<owner>.code-workspace`, writing it only
  when its `folders` are not the owner's kept repositories, whole and
  renamed over the old one.

The runner says each thing it does or names as one line on standard
error, `<owner>/<name>: <words>`. A pass in which nothing changed says
nothing. Told of a landing, it keeps that one repository at once. It
stops on SIGTERM or SIGINT before the next repository it would keep,
and exits 0. It opens no listening socket: every connection it makes is
outgoing.

Each turn of passes ends, before the runner waits, with Flume's settle
of every place, `flume --json settle --every <bound>`, run once and
awaited, so the runner keeps Flume's watch over its host: while it
runs, Flume runs unattended, and every place waiting on it, a submitted
place with no agent among them, moves within the bound. The bound is
`fetch_interval` in whole seconds, or the shorter of it and
`poll_interval` while the runner makes control passes. A turn whose
reading of the configuration failed settles all the same: the watch is
the host's, not the file's. A landing kept between passes is followed by
no settle, and a runner told to stop begins none. The runner reads
nothing of the settle's answer, and carries and keeps none of it. It
says nothing while Flume settles; it says one line when a settle is
refused, `boomdrive: Flume refused the settle of every place: <words>`,
or gives no answer, `boomdrive: Flume gave no answer to the settle of
every place: <why>`, and one when Flume settles again, `boomdrive: Flume
settles every place again`. A refused settle stops nothing. Flume offers
no end to a watch, so once the runner stops, its last watch stands until
twice its bound has passed since that settle began, and the host then
runs attended: Flume settles every place at each command that acts.

`boomdrive run --once` makes one pass and exits, 0 when every git call
succeeded and 1 when one failed, or a workspace could not be read or
written. A repository left by rule, for a folder of the user's or work
the forge lacks, is not a failure, nor is a workspace left because the
runner cannot read it. Where
the configuration names a control plane, `--once` makes one control
pass after the keeping pass, and exits 1 too when that pass did not
pull and report. A tool that pass could not run or read is named, and
is not a failure. `--once` settles every place once, after its passes,
and a settle Flume refused or gave no answer to is named, and is not a
failure either.

Either form refuses to start, exiting 2 and touching nothing, when
`HOME` is unset, the configuration cannot be read, git is missing or
older than 2.48, or another runner holds the data directory: one runner
keeps a data directory at a time.

The host's service manager keeps `boomdrive run` alive: launchd's
`KeepAlive` on a Mac, `Restart=always` in a systemd user unit on a box.
An exit 2 means the host needs fixing, and the service manager's
throttle keeps it from spinning.

## Installing with nix

The flake at this repository's root, `github:agentplot/boomdrive`,
gives the family's host tools pinned together: the runner from this
tree, Flume and sluice each at a release tag, and each tool's herdr
plugin from its tag, for `aarch64-darwin`, `aarch64-linux` and
`x86_64-linux`. Each reports its tag's version, and the set is the one
the runner's tests pass with. nixpkgs builds nothing for an Intel Mac,
so the flake gives it nothing. herdr and git are the host's own.

To put the runner, Flume and sluice in your profile:

    nix profile add github:agentplot/boomdrive github:agentplot/boomdrive#flume github:agentplot/boomdrive#sluice

An older nix spells it `nix profile install`. nix asks whether to
accept the flake's `nixConfig`, which names the family's cache,
`boomdrive` on Cachix, and its key: answer `y`, and `y` again to keep
the answer, and the binaries come from the cache. nix uses a
substituter a flake names only for a trusted user, or where its
daemon's `trusted-substituters` names it, so anyone else trusts the
cache in the host's nix settings, as below.

A host's configuration takes the flake as an input, and nix reads no
input's `nixConfig`. So a host that wants the binaries trusts the
family's cache in its own nix settings:

```nix
nix.settings.extra-substituters = [ "https://boomdrive.cachix.org" ];
nix.settings.extra-trusted-public-keys = [ "boomdrive.cachix.org-1:vHLgmasPg2jKqkvPXWOOrYzspoGL2xxsW4fAuHBtiag=" ];
```

Where nix-darwin does not manage nix, the same two settings go in nix's
own `nix.conf`. Leave the flake's `nixpkgs` its own: the packages are
made with it, so they are the store paths the cache holds. A host that
makes it follow its own, `inputs.boomdrive.inputs.nixpkgs.follows`,
changes every one of them, and builds them all.

A host that trusts no cache, or declines nix's question, gets the same
packages, built from the source the flake's lock names, at the same
store paths. So does a host whose path the cache no longer holds.
Distrust costs only the time to build.

The flake at `main` moves with each change that lands, and its cache
run pushes that commit's binaries. Each landing that git-cliff finds a
new version in, from its Conventional Commits, is released as a tag
`v<version>` on that commit of `main`, with its notes, and the cache
runs again at the tag. To stay on one set, pin a tag,
`github:agentplot/boomdrive/<tag>`, or a commit of `main` whose `cache`
run passed:

    gh run list --repo agentplot/boomdrive --workflow cache.yml --branch main --status success --limit 1 --json headSha --jq '.[0].headSha'

A host's configuration imports its platform's module,
`darwinModules.default` for nix-darwin or `nixosModules.default` for
NixOS, and writes the runner's file as `services.boomdrive.settings`,
each key as the file names it:

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    nix-darwin = {
      url = "github:nix-darwin/nix-darwin";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    boomdrive.url = "github:agentplot/boomdrive";
  };

  outputs =
    { nix-darwin, boomdrive, ... }:
    {
      darwinConfigurations.host-a = nix-darwin.lib.darwinSystem {
        modules = [
          boomdrive.darwinModules.default
          {
            # The rest of the host's configuration, system.primaryUser among it.
            services.boomdrive = {
              enable = true;
              settings = {
                repository_root = "~/Code";
                code_workspaces = true;
                repositories = [ "agentplot/flume" "agentplot/sluice" ];
                control_plane = {
                  type = "crew";
                  crew = { host = "host-a"; partitions = [ { label = "home"; state_repository = "example/state"; } ]; };
                };
              };
            };
          }
        ];
      };
    };
}
```

On nix-darwin the runner is a launchd agent of `system.primaryUser`'s
login, `org.nixos.boomdrive`, writing to `services.boomdrive.logFile`,
`~/Library/Logs/boomdrive.log` where none is named. It waits for
`/nix/store` before it starts the runner, so a runner loaded at boot
before macOS mounts the store's volume still starts. On NixOS it is the
systemd user unit `boomdrive.service` of `services.boomdrive.user`, who
lingers, so it runs from boot. The file is written into the store, the
service runs `boomdrive run --config <that file>`, and each switch
links `~/.config/boomdrive/config.toml` to it, so a command run by hand
reads it too. A file of your own there is moved once to
`config.toml.before-nix`. A switch that changes the file or the runner
restarts it. A key of `settings` named for a credential, `token`, `key`,
`secret` or `password`, or one ending in `_token`, `_key`, `_secret` or
`_password`, fails evaluation, since every user reads the store.

The module puts the runner and the set's Flume and sluice in the system
profile, and the runner's `PATH` is the user's profiles, then the
system's, so the runner, the agents Flume launches and your shell run
one copy of each tool. A host that installs its own Flume and sluice
says so:

```nix
services.boomdrive.tools = [ ];
```

`tools` may name the host's own packages instead, and
`services.boomdrive.package` replaces the runner.

Under a hosted control plane, enroll each partition after the switch,
on the host, as the service's user, the token on standard input:

    boomdrive enroll home

No option holds a token or a key. On NixOS, a hosted control plane
needs systemd 256, whose `systemd-creds` keeps the key, and evaluation
fails on an older one.

The runner refuses to start below git 2.48, and says so in its log. A
Mac whose only git is Apple's, where that is older, puts a newer one in
a profile, as `environment.systemPackages = [ pkgs.git ];` does.

The set gives each tool's herdr plugin, `flume-herdr-plugin` and
`sluice-herdr-plugin`, at its tool's tag. herdr's own command links
one. Build it to a link of your own, which keeps nix's garbage
collector from it, and link that:

    nix build github:agentplot/boomdrive#flume-herdr-plugin --out-link ~/.local/state/herdr-plugins/flume
    herdr plugin link ~/.local/state/herdr-plugins/flume

A tool's version moves by its input, and an override is the host's
own: its paths are not the cache's, so the host builds them, and the
runner's floors judge it, not this repository's tests. On the command
line:

    nix profile add github:agentplot/boomdrive#flume --override-input flume github:agentplot/flume/v0.17.2

In a host's flake it is `inputs.boomdrive.inputs.flume.url`. Flume's
version is its tagged tree's. Its git crates are fetched by the hashes
the flake holds for the tag it names, in
`nix/packages/flume-git-crates.nix`, so a tag whose weir differs fails
to build, and needs a package of the host's own in `tools`. sluice's
tagged tree says `0.0.0`, and the set gives sluice the version of the
tag `flake.nix` names, so an overridden sluice is given its own version
beside it:

```nix
# In the host's flake's inputs:
boomdrive.inputs.sluice.url = "github:agentplot/sluice/v0.75.0";
```

```nix
# In its configuration:
services.boomdrive.tools = [
  boomdrive.packages.aarch64-darwin.flume
  (boomdrive.packages.aarch64-darwin.sluice.overrideAttrs (old: {
    __intentionallyOverridingVersion = true;
    version = "0.75.0";
    env = old.env // { SLUICE_VERSION = "0.75.0"; };
  }))
];
```

## Carrying the control plane's work

With a `[control_plane]` table, the runner carries its control plane's
work into Flume:

```toml
[control_plane]
type = "crew"
poll_interval = "1m"

[control_plane.crew]
host = "host-a"

[[control_plane.crew.partitions]]
label = "home"
state_repository = "example/home-state"
```

`type` names the implementation, and is required. This binary carries
`crew` and `hosted`. Any other word is named at start with the words it
carries, and no control pass runs, though the repositories are kept all
the same. `poll_interval` is how often a control pass is made,
written as `fetch_interval` is; `1m` where none is named. The table
`type` names is handed whole to the implementation, whose keys are its
own. The table is read when the runner starts, so a change to it takes
effect when the runner starts again.

A control pass runs after the first keeping pass, then each
`poll_interval` after the start of the one before, sooner, after a pass
that reported, where a lease not yet lapsed would lapse first, and at
once when the control plane wakes the runner. The runner says once that
its control plane could not be read, and once that it is read again.
Each control pass ends with the settle of every place, and a control
pass that follows a keeping pass at once shares its settle, run once
after both, so no launch waits behind a settle. A control pass:

- reads `flume --json status`, and hands the control plane the
  host's tools' versions, the repositories kept with their clones made,
  apart from them those it drains, Flume's host part as Flume printed
  it, and the leases the runner holds;
- takes each launch offered under a lease, for a repository the runner
  keeps, and none in one it drains but a launch found by its mark, once
  each launch it comes after has landed and while its stream has room
  as Flume's status gives it. Each becomes one `flume launch`, naming its kind, stream, priority and intent only as
  the launch gives them, marked `boomdrive:<id>`. A bolt is a launch
  like any other, of a kind that lands with no stages, whose branch
  sluice makes at Flume's asking, and its units are launches on that
  branch;
- carries out each retry, drop, priority or done as Flume's own command
  on the launch's place, once every launch the pull offers is taken,
  and, once the next pass is due, no further act after its first, so no
  launch waits behind an act. A place Flume's listing does not hold is
  first asked of `flume --json status <place>`, which settles nothing,
  and Flume's refusal of it is the act's answer. A bolt's send is a
  retry naming its submission, `flume retry <place> --submission`,
  whose answer is Flume's;
- refuses every command, naming its tool, `this binary does not carry
  <tool>'s commands`, a branch command's being sluice, and runs nothing
  for it;
- carries back Flume's status of each place, sluice's entry of each
  submission and each inbox the control plane names, each as the tool
  gave it, and never a log; and relays a terminal on a launch's agent
  through herdr, keeping nothing of it. A place is followed through its
  repository's clone, as `flume --json status --repository <container>
  <place>`, to its end, over or tidied, the clone of a repository the
  configuration dropped among them while the runner drains it; where no
  clone stands, Flume is asked nothing of it. No rename is followed: to
  rename a repository, drain its work, drop it, rename it on the forge,
  and add it back under its new name. Where the pass ran a launch or an
  act, Flume's status is read again before the places' are carried, so
  a place tidied meanwhile is carried from Flume's record of it,
  `<location>@<at>`.

Each launch and act is carried out once, and each command answered
once, across a restart: the runner's report is kept at
`<data>/control/report.json`, and a launch whose answer was lost is
found by its mark. An offer, an act or a command whose lease has lapsed
by the runner's clock waits until a pull gives it under a live lease.

## A host from before the rename

The runner was named `flywheel` until record 0016 renamed it. A host
where a runner of that name ran keeps what it made: where
`$XDG_DATA_HOME/boomdrive` is not there and `$XDG_DATA_HOME/flywheel`
is, that is the data directory, and it is never moved, since its clones
hold places and views. A clone recording itself under `flywheel` has
its records carried over to `boomdrive` the first time a pass finds it,
and a launch marked `flywheel:<id>` is found by that mark. The
configuration is the host's own, and is read only where the new name
puts it.

crew's table names `host`, this host as crew's hosts file names it, and
each partition served: its `label`, and its `state_repository` as
`owner/name`, reached over https through the host's own git credential
helper. crew's implementation reads `requests.rec`, `roles.rec`,
`asks.rec` and `plan.rec` at `<label>/main`, keeping its caches under
`<data>/control/crew/`. It answers on the runner's branch,
`<label>/runner/<host>`, in crew's formats: `runner.rec`, `answers.rec`
and `places.rec`. It pushes no other ref, and never with force. A
kit's launch that names no host is claimed on that branch, and taken
once no other runner's branch answers it. A launch a drop names no
longer stands, as crew reads it: crew's implementation offers and
claims no such launch unless the runner launched it, and lets go one
the runner holds unlaunched. A drop is given to the runner that holds
its launch, though its work has left the plan, so the runner answers
it, dropping a launched place by Flume's own command.

Kinds and streams are Flume's host configuration. The runner passes a
launch's kind to `--kind` as the control plane gave it, a name or an
identity `<source>/<name>@<version>`, and names no kinds file, kind
source or lock. It names a stream only where the launch does. The
host's Flume configuration declares its streams, the stream each
repository's launches run in, and where its kinds come from.

## Dialing the hosted control plane

With `type = "hosted"`, the runner dials a hosted control plane:

```toml
[control_plane]
type = "hosted"
poll_interval = "1m"

[control_plane.hosted]
url = "https://control.example"

[[control_plane.hosted.partitions]]
label = "home"
repositories = ["agentplot/flume"]

[[control_plane.hosted.partitions]]
label = "work"
repositories = ["example/crew", "agentplot/flume"]
```

`url` is the control plane's address: `https://`, or `http://` to a
loopback address alone, with no user, query or fragment. It may hold a
path, under which the runner enrolls at `runner/enroll` and connects at
`runner/connect`. Each partition is one enrollment, with a key of its
own. Its `label` is the host's own name for it, 1 to 32 lowercase
letters, digits and `-`, starting with a letter. Its
`repositories` are those it is served, which may be none: only those the
host keeps go in its hello, and two partitions may name one repository,
which the host keeps in its one clone. The table holds no token and no
key, and one that names either is refused, saying the key is made by
`boomdrive enroll`.

Each partition is enrolled once:

    boomdrive enroll [--config <file>] [--json] <label>

It reads the partition's one-time token from standard input, asking for
it without echo where that is a terminal, and never from an argument or
the environment. It makes the partition's Ed25519 key on the host, and
registers its public key with the control plane by one request bearing
the token. The key is kept only once the control plane has enrolled it,
and only in the host's credential store: the login keychain on a Mac, or
on Linux a user's credential encrypted by `systemd-creds` of systemd 256
or later. It never leaves the host, and no file holds it in the clear.
`enroll` writes no configuration and nothing in the data directory, and
takes no lock, so a running runner dials a partition enrolled beside it
at its next try. It answers `<label>: enrolled as host <host>`, or with
`--json` `{"enrolled":{"partition":"<label>","host":"<host>"}}`, and
exits 0. A refusal is its words on standard error, or with `--json`
`{"refused":{"refusal":{"reason":"<words>"}}}`, and exits 1 where the
control plane refused the enrollment or could not be reached, and 2
where the host must be fixed first: its configuration, its credential
store or the token.

Each partition keeps one outgoing WebSocket, its key proved at every
connection by a statement it signs afresh. The socket is kept by the
WebSocket protocol's own pings, and one lost is tried again after a
wait drawn at random, up to a cap that doubles from one second to
`poll_interval`. Its first message is the partition's whole hello,
naming only that partition's repositories and leases, and each id its
control plane gives is the runner's as `<label>/<id>`, so a launch's
mark is `boomdrive:<label>/<id>`. A pull gives each reached partition's
work, and the control plane's work wakes the runner when it comes. A
report goes to each partition as its own part, answered by its id, and
one unanswered is sent again under that id after a reconnect. Of each
tool's answer only the copy the protocol's version names goes up,
checked against it before it goes: a place's status or entry that fails
goes up as `null`, and the host part as `missing`, with a line naming
the schema's path, never a value. Every message the
control plane sends is checked before it is read. A partition says one
line when its control plane becomes unreachable, `<label>: the control
plane is unreachable: <why>`, and one when it is reached again. A
terminal the control plane opens is closed at once, since this host's
Flume relays none.

## Rules and checks

The rules every change keeps are `docs/architecture/core.md`; the
decisions behind them are `docs/adr/`. The checks are moon tasks, run
in the devenv shell: `devenv shell -- moon run :#check-fast`. Each crate
is a moon project of its own, `boomdrive`, `boomdrive-control`,
`boomdrive-crew` and `boomdrive-hosted`, beside `nix`, which holds the
flake, its modules and the cache's workflow, and `root`, which holds
what spans the tree.
