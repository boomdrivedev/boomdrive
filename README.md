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

`boomdrive` reads its configuration in two layers of TOML, in Flume's
words. The managed layer is the file `--config <file>` names, else
`$XDG_CONFIG_HOME/boomdrive/config.toml`, else
`~/.config/boomdrive/config.toml`, written by Nix, a fleet or a person.
The local layer is `config.local.toml` in that same directory,
`$XDG_CONFIG_HOME/boomdrive/` or `~/.config/boomdrive/`, whatever
`--config` names, written by the operator or a control plane and read
over the managed one. A host with no local file reads the managed layer
alone. The managed layer names the accounts the host serves, each
account's drives and each drive's repositories by their GitHub address,
the root their views are kept under, how often they are fetched and
whether each drive has a VS Code workspace:

```toml
# The root each repository's view is kept under, at <repository_root>/<owner>/<name>; ~/Code where none is named.
repository_root = "~/Code"
# How often every repository is fetched; 2m where none is named.
fetch_interval = "2m"
# Whether each drive has a VS Code workspace, <repository_root>/<drive>.code-workspace; false where none is named.
code_workspaces = true

# The account `own`'s drive `home`: each repository it keeps, by its GitHub address.
[accounts.own.drives.home]
repositories = ["agentplot/flume"]

# The account `own`'s drive `work`.
[accounts.own.drives.work]
repositories = ["example/crew"]
```

An account is what the runner enrolls with a control plane. A drive is
one account's body of work, `[accounts.<account>.drives.<drive>]`, and
its `repositories` are the repositories it keeps, in the order kept.
The runner keeps every drive's repositories, in the file's order.
`accounts`, each account's `drives` and each drive's `repositories` are
required, and each may be empty: a host that keeps none says
`accounts = {}`, and a drive that keeps none yet says
`repositories = []`. An account holds only its `drives` and a drive
only its `repositories`. An account's or a drive's name is 1 to 32
lower-case letters, digits and `-`, starting with a letter, and a
drive's name is given once on the host, across its accounts. A
repository belongs to one drive: addresses are compared without case,
and two that differ only in case, in one drive or in two, are refused
as one repository named twice. `repository_root` is an absolute path or
begins with `~/`. `fetch_interval` is `<n>s`, `<n>m` or `<n>h`. A key the
runner does not read is refused, so the file has no place for a
credential: the host's own git credential helper answers every fetch. A
file written for an earlier runner is refused with what to write
instead: `views` is `repository_root` now, `fetch_every` is
`fetch_interval`, and a flat `repositories` is `accounts`, each
repository named under its drive. Nor does the file name a stream or a
kind, at its top, in an account or in a drive: those are each host's
Flume configuration, and a key naming one is refused, saying so.

`code_workspaces = true` asks for one VS Code workspace for each drive,
at the repository root under the drive's name,
`<repository_root>/<drive>.code-workspace`, so VS Code titles its window
`— <drive> (Workspace)`. Its `folders` are the drive's kept
repositories, each by its view's path from the root, `<owner>/<name>`,
in the file's order, set at every pass; the rest of the file, the
settings VS Code writes into it, is kept. A drive's workspace goes with
its drive: once the drive has left the configuration and no clone the
runner holds was kept for it, the runner removes the workspace, and no
other. A file there the runner cannot read as a JSON object, one with
comments in it say, is left as it is and named at every pass.
`code_workspaces` is `true` or `false`, and where it is absent or
`false` the runner writes and removes no workspace.

The local layer sets each drive's `repositories`,
`accounts.<account>.drives.<drive>.repositories`, and in a control
plane's table, `[control_plane.<type>]`, its `partitions`, and nothing
else; each is read whole, as the managed layer's is. Its repositories
add to the managed layer's: a drive both layers name keeps the managed
layer's repositories, in their order, then each of the local layer's
the managed layer does not name, and a drive or an account only the
local layer names is declared by it, after the managed layer's. So a
control plane that drops a drive drops only the repositories it wrote.
A repository is still in one drive on the host, and a drive's name is
given once: a local layer that names a repository the managed layer
keeps in another drive, or a drive the managed layer holds under
another account, is not taken. Its `partitions` replace the managed
layer's. A local layer a control plane might write:

```toml
# Added to the managed layer's drive `home`.
[accounts.own.drives.home]
repositories = ["agentplot/sluice"]

# A drive the managed layer does not name, declared here.
[accounts.own.drives.garden]
repositories = ["agentplot/timber"]
```

The managed layer may pin a key the local layer could set: `pin`, at its
top, lists keys in TOML's dotted spelling, such as
`"accounts.own.drives.home.repositories"` or
`"control_plane.hosted.partitions"`, or a table holding one, such as
`"accounts"`, `"accounts.own.drives.home"` or `"control_plane"`. A
pinned key the local layer sets is passed over and named, and the rest
of the local layer is taken. A pin of anything else is refused as the
managed layer's fault.

A local file that sets any other key, `repository_root`, `pin`, a flat
`repositories`, a key of an account or a drive other than `drives` and
`repositories`, or a control plane's `type` among them, that is not
TOML, or that holds a value the runner or the control plane's
implementation refuses, is not taken at all, and the managed layer is
read alone. While such a file stands, a pass keeps the managed layer's
repositories, lets go of none and sets or removes no workspace, since
the layer it could not take may name them. Removing the file lets go of
what only it named. Each pass says one line of the local file where its
reading changes: taken, with each key passed over; not taken, with why;
or gone.

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
  the host's own credential helper, never prompting. A fetch, a new
  clone's among them, still going at thirty seconds is stopped with
  everything it started, as agentplot/sluice record 0109 bounds every
  try of a call to a remote. The repository is then named not fetched,
  or not cloned with nothing left, and is fetched again at the next
  pass, while the pass keeps every other repository;
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
- where `code_workspaces` is true, keeps each drive's VS Code workspace
  at `<repository_root>/<drive>.code-workspace`, writing it only when its
  `folders` are not the drive's kept repositories, whole and renamed
  over the old one, and removes the workspace of a drive that has gone
  with its last clone.

The runner says each thing it does or names as one line on standard
error, `<owner>/<name>: <words>`. A pass in which nothing changed says
nothing. Told of a landing, it keeps that one repository at once,
between passes or between two repositories of a pass. It stops on
SIGTERM or SIGINT before the next repository it would keep, lets each
part of its control plane end the pass it has in hand, and exits 0. It
opens no listening socket: every connection it makes is outgoing.

Where the configuration names a control plane, each part of it, a crew
partition or a hosted partition, runs its control passes on its own,
on a thread of its own from the end of the first keeping pass: on its
own timer, at once when its control plane wakes it, and saying its own
reach. No part's pull, act, command or report waits on another part's,
or on a keeping pass. Each line the runner says of a part's pass names
its label after `boomdrive:`, as `boomdrive: <label>: <words>`, or
begins with the id of the want it is about. What the parts share on the
host is a clone, which the keeper keeps and a launch goes into in turn,
the report file, each part saving its own share of it, and Flume's
room. A hosted partition is an account's enrollment, so the drives of
one hosted account share that account's connection, in the order its
control plane gives their work.

Each pass ends, before its thread waits, with Flume's settle of every
place, `flume --json settle --every <bound>`, run once and awaited: a
keeping pass on the runner's loop, and each part's control pass on the
part's thread. So the runner keeps Flume's watch over its host: while
it runs, Flume runs unattended, and every place waiting on it, a
submitted place with no agent among them, moves within the bound. The
first keeping pass, which each part's first control pass follows at
once, shares their settles, so no launch waits behind a settle. No two
settles run at once: a pass that ends while another's settle is under
way shares it, and waits for nothing. The bound is `fetch_interval` in
whole seconds, or the shorter of it and `poll_interval` while the
runner makes control passes. A keeping pass whose reading of the
configuration failed settles all the same: the watch is the host's, not
the file's. A landing kept between passes is followed by no settle, and
a runner told to stop begins none. The runner reads
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
pass of each part after the keeping pass, side by side, and exits 1 too
when no part both pulled and reported. A part that could not be read
while another reported, or a tool a pass could not run or read, is
named, and is not a failure. `--once` settles every place once, after
its passes, and a settle Flume refused or gave no answer to is named,
and is not a failure either.

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
gives the family's host tools pinned together, for `aarch64-darwin`,
`aarch64-linux` and `x86_64-linux`:

- the runner, from this tree;
- Flume, sluice and crew, each at a release tag through its own flake,
  and Flume's and sluice's herdr plugins from their tags;
- herdr, through its own flake at the release Flume's tag pins, which
  Flume's terminal profile was checked against;
- Ghostty, from the flake's nixpkgs at the release Flume's window
  profile was checked against: built from source on Linux, and on
  macOS, where nothing builds its app from source, Ghostty's own
  release as nixpkgs' `ghostty-bin` gives it;
- the Boomdrive launcher, `boomdrive-launcher`, of this tree: an app,
  `Boomdrive`, on macOS and a desktop entry on Linux, which runs `flume
  open` to show the host's agents.

Each tool reports its tag's version, and the set is the one the
runner's tests pass with. nixpkgs builds nothing for an Intel Mac, so
the flake gives it nothing. git alone is the host's own.

To put the runner, Flume and sluice in your profile, and crew where the
host runs crew:

    nix profile add github:agentplot/boomdrive github:agentplot/boomdrive#flume github:agentplot/boomdrive#sluice github:agentplot/boomdrive#crew

The same line may add `github:agentplot/boomdrive#herdr` and
`github:agentplot/boomdrive#ghostty`.

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
NixOS, names its accounts in `services.boomdrive.accounts`, and writes
the rest of the runner's file as `services.boomdrive.settings`, each key
as the file names it. The module brings Flume's, sluice's and
crew's own modules, `services.flume`, `services.sluice` and
`services.crew`, each from its tool's flake at the tag this flake names,
and the flake gives each again as `darwinModules.<tool>` and
`nixosModules.<tool>`. Each writes its own tool's file from its
`settings`, each key as that file names it, and links it at the tool's
own path, `~/.config/flume/config.toml`, `~/.config/sluice/config.toml`
and `~/.config/crew/teams.json`. A file of your own there is moved once
to `config.toml.before-nix`, and crew's to `teams.json.before-nix`, so
move its values into the tool's `settings` in the switch that takes this
flake. `services.boomdrive.enable` turns on Flume's and sluice's
modules; a host that runs crew turns on crew's, with its `host` and its
partitions. Where `control_plane.type` is `crew` and no
`control_plane.crew` is written, the module writes the runner's
`[control_plane.crew]` from crew's partitions: its `host` is
`services.crew.host`, and it holds each partition whose `machine` is
that host, by its `label`, with its `state` as `state_repository`. A
table written out wins, whole:

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
              accounts.own.drives.home.repositories.agentplot = [ "flume" "sluice" ];
              settings = {
                repository_root = "~/Code";
                code_workspaces = true;
                control_plane.type = "crew";
              };
            };
            services.crew = {
              enable = true;
              host = "host-a";
              settings.partitions = [
                {
                  label = "home";
                  partition = "home";
                  blueprints = [ "example/blueprints" ];
                  state = "example/state";
                  machine = "host-a";
                  session = "home";
                  review = { every = 5; hours = 4; };
                  flume = { design = false; planner = false; reviewer = false; ops = false; operator = false; };
                }
              ];
            };
          }
        ];
      };
    };
}
```

Each drive names its repositories by owner,
`services.boomdrive.accounts.<account>.drives.<drive>.repositories.<owner>`,
the names of the owner's repositories it keeps, and the module writes
them into the runner's file as their addresses, `<owner>/<name>`, owner
by owner in the order of the owners' names, each owner's names in the
order given. A repository is named once on the host, and a drive's name
once across the host's accounts, or evaluation fails. Each drive has its
stream in Flume, and the module gives Flume's module the drive's
`stream`, and each of the drive's repositories that stream, by its
address:

```nix
services.boomdrive.accounts.own.drives.home = {
  stream = {
    name = "home";                  # the drive's name where unset
    agents = 4;                     # 4 where unset
    priority = 11;                  # Flume's own where unset
    labels.review.reserve = 1;      # beside operation 1 and planning 2
  };
  repositories.agentplot = [ "flume" "sluice" ];
};
```

A stream's `agents`, `priority` and each label's `reserve` are the
drive's, and a label the drive does not set keeps the module's reserve,
`operation` 1 and `planning` 2. A drive's reserves are within its room:
Flume refuses a stream whose reserves pass its `agents`, naming it. Any
other key of a stream, a label's `keep_free` among them, is Flume's own,
set in `services.flume.settings.streams.<stream>`, beside the drive's;
and a value the host sets there, or in a repository's
`services.flume.settings.repos."<owner>/<name>"`, wins over the
drive's. Two drives of this host that give one stream fail evaluation. A
drive whose work runs on another computer names it, `machine = "host-b"`,
as crew's partitions name a host, and the module compares it with
`services.crew.host`, which the host sets whether or not crew runs
there: a drive of another machine gives Flume nothing, and the runner
still keeps its repositories, to read. A drive that names a `machine`
where `services.crew.host` is not set fails evaluation.

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
`config.toml.before-nix`, and the local layer, `config.local.toml`
beside it, is left as it is. A host pins a key through
`services.boomdrive.settings.pin`. A switch that changes the file or the
runner restarts it. `accounts` is always written, each account's
`drives` and each drive's `repositories` with it, and a key in an
account or a drive that the module does not declare fails evaluation.
The accounts are named in `services.boomdrive.accounts` alone: accounts
written in `settings` fail evaluation, as a flat `settings.repositories`
of before does. A key of `settings` named
for a credential, `token`, `key`, `secret` or `password`, or one ending
in `_token`, `_key`, `_secret` or `_password`, fails evaluation, since
every user reads the store.

The runner keeps its clones, its lock and its report in its data
folder, which each module names as `services.boomdrive.dataDir`:
`~/.local/share/boomdrive` under the home of the runner's user, or
`$XDG_DATA_HOME/boomdrive` where the runner's job is given an absolute
`XDG_DATA_HOME`, in the agent's `EnvironmentVariables` on nix-darwin
and in the unit's `environment` on NixOS. The option is read-only: read
it wherever the host names the folder, a clone's folder in it being
`services.boomdrive.cloneDirs`'s, and set it nowhere, since evaluation
fails where a host sets it. An `XDG_DATA_HOME` the job takes from
elsewhere, launchd's user domain or the user manager's own environment,
is not seen, so a host that moves the runner's data gives the job its
own:

```nix
launchd.user.agents.boomdrive.serviceConfig.EnvironmentVariables.XDG_DATA_HOME = "/srv/data";  # nix-darwin
systemd.user.services.boomdrive.environment.XDG_DATA_HOME = "/srv/data";                       # NixOS
```

Each module states each kept repository's clone folder as
`services.boomdrive.cloneDirs`: one entry for each address of
`services.boomdrive.accounts`, every drive's of every account, another
machine's drives among them, keyed by the address, `<owner>/<name>`,
its folder `<dataDir>/repositories/<owner>/<name>`, the folder
`flume launch --repository` names. It holds only the repositories
`services.boomdrive.accounts` names, and none the runner's local layer
alone adds. The option is read-only: read it wherever the host names a
clone's folder, and set it nowhere, since evaluation fails where a host
sets it or an entry of it. git run in the folder finds the bare
repository by the folder's own `.git` file:

```nix
config.services.boomdrive.cloneDirs."agentplot/flume"  # ~/.local/share/boomdrive/repositories/agentplot/flume
```

The runner's module puts the runner in the system profile, and each
tool's module puts its tool there, the set's unless the host gives its
own. The runner's `PATH` is the user's profiles, then the system's, so
the runner, the agents Flume launches and your shell run one copy of
each tool. A host that installs its own Flume and sluice says so in
each tool's `package`, `null` for none:

```nix
services.flume.package = null;
services.sluice.package = null;
```

Each tool's `package`, `services.crew.package` among them, may name the
host's own package instead, and `services.boomdrive.package` replaces
the runner. A host that keeps a tool's file its own sets that tool's
`enable = false`, and installs the tool itself.

With `services.boomdrive.enable`, the runner's module also gives the
host a way to see its agents:

- It installs herdr, Ghostty and the Boomdrive launcher, `Boomdrive`
  in `/Applications/Nix Apps` on macOS or in the desktop's applications
  on Linux. A click on it runs `flume open`, which shows the default
  stream's own session, and shows Flume's words where Flume refuses.
- At each switch it links each plugin of
  `services.boomdrive.herdr.plugins`, by default the set's Flume and
  sluice plugins, into the runner's user's herdr with herdr's own
  `herdr plugin link`, which starts no server. A relink enables a plugin
  you disabled, so leave one you want off out of the list. A plugin
  left out stays linked until `herdr plugin unlink`. A herdr already
  running takes a new plugin at its next start.
- It sets Flume's `host.multiplexer` to `herdr` and `host.terminal_app`
  to `ghostty`, unless the host names its own in
  `services.flume.settings.host`. Where the host writes Flume's older
  `host.terminal`, it sets no `multiplexer`.

Each of the three is the set's by default, and each may name the host's
own package instead, or `null` for none:

```nix
services.boomdrive.herdr.package = null;
services.boomdrive.ghostty.package = null;
services.boomdrive.launcher.package = null;
```

With no herdr package, or no plugin, nothing is linked. A host with a
herdr of its own may give it as `services.boomdrive.herdr.package`, and
the plugins are linked with it. A host with no display sets Ghostty and
the launcher `null`, and Flume's `host.terminal_app` its own. A Mac that
keeps a Ghostty of its own in `/Applications` sets the set's `null`, so
macOS opens one Ghostty.

The flake also gives a shell, its `PATH` the set's runner, Flume,
sluice, crew and herdr, that starts herdr on entering and leaves you in
the shell when herdr ends:

    nix develop github:agentplot/boomdrive

A command run through it, `nix develop github:agentplot/boomdrive
--command <command>`, starts no herdr, and neither does a shell entered
from inside herdr. The shell links no plugin: the modules do. This
repository's own development shell is devenv's.

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
`sluice-herdr-plugin`, at its tool's tag, which the modules link. A host
without them links one with herdr's own command. Build it to a link of
your own, which keeps nix's garbage collector from it, and link that:

    nix build github:agentplot/boomdrive#flume-herdr-plugin --out-link ~/.local/state/herdr-plugins/flume
    herdr plugin link ~/.local/state/herdr-plugins/flume

A tool's version moves by its input, and an override is the host's
own: its paths are not the cache's, so the host builds them, and the
runner's floors judge it, not this repository's tests. On the command
line:

    nix profile add github:agentplot/boomdrive#flume --override-input flume github:agentplot/flume/v0.22.0

In a host's flake it is `inputs.boomdrive.inputs.flume.url`. Flume's
version and its git crates' hashes are Flume's own flake's, at the tag
it names. sluice's tagged tree says `0.0.0`, and crew's carries no
version, so the set gives each the version of the tag `flake.nix`
names, and an overridden sluice or crew is given its own version beside
it, in that tool's `package`:

```nix
# In the host's flake's inputs:
boomdrive.inputs.sluice.url = "github:agentplot/sluice/v0.77.0";
```

```nix
# In its configuration:
services.sluice.package = boomdrive.packages.aarch64-darwin.sluice.overrideAttrs (old: {
  __intentionallyOverridingVersion = true;
  version = "0.77.0";
  env = old.env // { SLUICE_VERSION = "0.77.0"; };
});
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
Each control pass ends with the settle of every place, on its part's
thread, and the first, which follows the first keeping pass at once,
settles for both, so no launch waits behind a settle. A control pass:

- reads `flume --json status`, and hands the control plane the
  host's tools' versions, the repositories kept with their clones made,
  apart from them those it drains, Flume's host part as Flume printed
  it, and the leases the runner holds;
- takes each launch offered under a lease, for a repository the runner
  keeps, and none in one it drains but a launch found by its mark, once
  each launch it comes after has landed and while its stream has room
  as Flume's status gives it. Each becomes one `flume launch`, naming its kind, stream, priority and intent only as
  the launch gives them, its recipients as the launch gives them with
  the runner one more after them, `--recipient boomdrive`, where it
  names any, marked `boomdrive:<id>`. A bolt is a launch
  like any other, of a kind that lands with no stages, whose branch
  sluice makes at Flume's asking, and its units are launches on that
  branch;
- carries out each retry, land, confirm, drop, priority or done as
  Flume's own command on the launch's place, once every launch the
  pull offers is taken, and, once the next pass is due, no further act
  after its first, so no launch waits behind an act. A place Flume's
  listing does not hold is first asked of
  `flume --json status <place>`, which settles nothing, and Flume's
  refusal of it is the act's answer. A land is
  `flume retry <place> --submission`, whatever the launch's kind, and
  a bolt is landed on a land, whose answer is Flume's. A confirm is
  `flume confirm --recipient=<recipient> -- <key>…`, of exactly the
  keys its control plane names, each the delivery of an end of the
  launch's place, as the reader it names and never as `boomdrive`; the
  runner confirms no end for another reader of its own accord;
- refuses every command, naming its tool, `this binary does not carry
  <tool>'s commands`, a branch command's being sluice, and runs nothing
  for it;
- brings up each preview the control plane asks for, of a branch at a
  revision, in a repository it keeps, as
  `sluice preview <branch> <revision>`, once every launch the pull
  offers is taken, and, once the next pass is due, no further preview
  after its first. It waits at
  sluice's bound no longer than 20 seconds, then interrupts sluice,
  which ends its wait and nothing else, and asks again at the next
  pass. It carries sluice's listing of the preview, `sluice previews`,
  until sluice lists it no more. On the control plane's done, it puts
  away only a preview it asked sluice for, as `sluice release <name>`,
  and refuses a done of any other, running nothing. It carries no
  output of the preview's command;
- carries back Flume's status of each place, sluice's entry of each
  submission, sluice's word of each preview and each inbox the control
  plane names, each as the tool gave it, and never a log; and relays a terminal on a launch's agent
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

No pane holds the name `boomdrive`, so Flume sends it no notice and
wakes nothing for it, and owes it each end of the place, a live place's
and a tidied one's alike, each owed until the runner confirms it. Once
the place is tidied, whatever tidied it, a landing, a drop or a done,
Flume keeps its record, `<location>@<at>`, while an end of it is owed.
The runner carries the place's status, that record among them, so its
control plane reads every end of the place from its report. Only once
its control plane has taken a report holding that status, and its report
file holds the answer, does the runner confirm the ends the status gives
pending for it, by `flume --json confirm --recipient=boomdrive -- <key>…`,
one call for each launch naming each end's delivery key, keeping nothing
of the answer. Until then each end stays owed in Flume, so losing the
runner's report loses no end. A confirmation Flume refuses, or could not
run, is said in one line and made again at a later pass whose report is
taken. A dropped repository's clone stays while a place's record is
pending for the runner. A launch that names no recipient is given none
of the runner's, so Flume's default is its recipient, and Flume owes the
runner nothing of it.

The runner runs with Flume 0.21.0 or later, the first whose ends a
recipient confirms by key. Where `flume --version` names an older
version, or none, no offer is launched and nothing is confirmed, and the
runner says so once until Flume's version changes; acts and statuses go
on.

Each launch, act and preview is carried out once, and each command
answered once, across a restart: the runner's report is kept at
`<data>/control/report.json`, and a launch whose answer was lost is
found by its mark. An offer, an act, a command or a preview whose
lease has lapsed by the runner's clock waits until a pull gives it under
a live lease.

crew's table names `host`, this host as crew's hosts file names it, and
each partition served: its `label`, and its `state_repository` as
`owner/name`, reached over https through the host's own git credential
helper. crew's implementation reads `requests.rec`, `roles.rec`,
`asks.rec` and `plan.rec` at `<label>/main`, keeping its caches under
`<data>/control/crew/`, and lists the runner branches,
`<label>/runner/*`, by `git ls-remote`. A listing still going at thirty
seconds is stopped with everything it started, as a fetch is, and the
partition is named unread and pulled again at the next pull. It answers
on the runner's branch, `<label>/runner/<host>`, in crew's formats:
`runner.rec`, `answers.rec` and `places.rec`, and, once a preview is
answered there, `previews.rec`,
under the descriptor this repository proposes for crew to hold. A
preview leaves the runner's branch once its done is answered, with what
the done put away; or once its work has left crew's plan and it is no
longer up, a bolt's work leaving with its `Bolt` record. A preview
request names the host that brings it up, and one naming no host is
given nothing. It pushes no other ref, and never with force. A
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

The runner speaks version 2 of the protocol, `boomdrive.v2`, its schema
under `crates/boomdrive-hosted/protocol/v2/`, which a control plane takes
beside the version before it, version 1. Each upgrade asks for
`boomdrive.v2` alone, and a control plane that answers without it is
said once, naming `boomdrive.v2`, and tried again at the backoff's cap.
Each end of a place goes up with its delivery: its key, its delivery's
key and expiry, the readers it is owed to and those that confirmed it,
and each hand-over, the end of the place's work by a drop or a done
among them. An end version 2 does not name goes up as its delivery
alone, or is left out where even that fails, the rest of its status
going up, with one line naming the launch and each end's path. Only a
status that went up with every end in a form version 2 names is one the
runner takes its control plane to hold, so it confirms none of the ends
of a place whose status went up with an end by its delivery alone. A
control plane's confirm of a reader's ends is an act, given to the
runner under the partition's prefix and carried out as crew's is,
by Flume's confirm of exactly its keys. A control plane's land and
preview, with its done, are given to the runner as crew's are, and what
the runner answered of each preview goes up with sluice's answer and
listing as their copies.

## Rules and checks

The rules every change keeps are `docs/architecture/core.md`; the
decisions behind them are `docs/adr/`. The checks are moon tasks, run
in the devenv shell: `devenv shell -- moon run :#check-fast`. Each crate
is a moon project of its own, `boomdrive`, `boomdrive-control`,
`boomdrive-crew`, `boomdrive-hosted` and `boomdrive-bounded`, beside
`nix`, which holds the flake, its modules and the cache's workflow,
`site`, which holds the site, `site/`, and the workflow that publishes
it, and `root`, which holds what spans the tree. The site's own rules are
`docs/architecture/site.md` and `site/CLAUDE.md`.
