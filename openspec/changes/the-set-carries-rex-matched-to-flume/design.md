# Design

## Context

Record 0023, as amended on 2026-11-18, decides that Rex is in the set beside herdr, from Rex's own flake (core.44), built into the family's cache (core.47), at the revision the pinned Flume's Rex profile is tested against, read from that Flume tag and moved whenever the set moves Flume's pin; and that a host that takes the set gets Rex installed while which terminal Flume opens stays its Flume settings. This design is how.

Flume v0.29.0, the tag the lock names for Flume, takes Rex in its `devenv.yaml` from Rex's flake by `https://pkgs.superlogical.com/rex/nix/preview/0.1.0-preview.14.tar.gz`, the URL Rex's release 0.1.0-preview.14 keeps for good, its `nixpkgs` following devenv's, and its `devenv.lock` locks that URL by `sha256-pmoFf2Wu9vB88SVgmOp+dN7e1frgEURcCBS1KYkNZ1E=` (Flume record 0068, point 5). Rex's flake packages Rex's release binary, as Rex publishes no source, for `x86_64-linux`, `aarch64-linux` and `aarch64-darwin`, the set's three systems; its package's version is the release's, from its `sources.json`; and it allows Rex, alone of unfree packages, in the nixpkgs it imports. It also gives a NixOS module, which runs Rex's server as a system service, and a home-manager module, and gives nix-darwin none.

Rules relied on: core.13, core.18, core.44, core.45, core.46 and core.47.

## Decisions

### Decision 1: Rex named by the URL Flume's tree locks, its nixpkgs the flake's

`flake.nix` names the input `rex` by the URL Flume's `devenv.yaml` names, Rex's release's own, with `inputs.nixpkgs.follows = "nixpkgs"`, as Flume's `devenv.yaml` makes it follow devenv's. The lock locks it as Flume's `devenv.lock` does, the same URL and hash. One string in `flake.nix` names the release, as a tag does for the other tools (core.45).

Not taken: Rex's preview channel's URL, which serves the newest preview and is locked to whatever it served at the lock, so `flake.nix` would not say which release the set holds; Rex's own nixpkgs, a second nixpkgs in the lock for one binary's `fetchurl` and `patchelf`.

### Decision 2: the package is Rex's flake's own, unchanged

The set's `rex` is `inputs.rex.packages.<system>.rex`, unchanged, on each of the three systems (core.44). Its version is its release's, `0.1.0-preview.14`, the last part of its URL without `.tar.gz`, and the flake's test holds the two the same, so the set says the release it names. Rex's flake allows Rex alone of unfree packages, in its own nixpkgs instance, so the set allows no other unfree package and asks no host to.

Not taken: a recipe of this flake's for Rex's binary, which core.44 rules out where the tool's own flake gives the package.

### Decision 3: held to Flume's `devenv.lock`

The flake's test reads Flume's `devenv.lock` at the tag the lock names for Flume, its root's input `rex`: `flake.nix` names its `original.url`, the URL Flume's `devenv.yaml` names, and the flake's lock locks its `locked.url` with its `narHash`. It fails where either does not hold, naming both, and where Flume's `devenv.lock` locks no `rex`, naming the file. A move of Flume's pin to a tag whose tree locks another Rex so fails the gate until the set's Rex moves with it, which is the amendment's "moved with Flume's pin".

Not taken: reading `devenv.yaml`'s lines, which nix reads no YAML of, and whose URL `devenv.lock`'s `original` holds; a version constant, as Ghostty's is, since Flume's tree pins its Rex where it pins no Ghostty.

### Decision 4: the Boomdrive module's own option, with the runner

`services.boomdrive.rex.package` is installed in the system profile with the runner: the set's `rex` by default, a host's own replacing it, and `null` installing none, as worktrunk's option is. The modules name Rex in no setting of Flume's, so `host.multiplexer` stays `herdr` by default, a host's own winning (the amendment's third point). They write no file of Rex's and start no server of it: Flume's Rex profile starts each server it uses (Flume record 0068, point 1), and the runner none (core.13).

Not taken: Rex's own NixOS module, which runs Rex's server as a system service, a process of the host's no tool of the set asks for, and which nix-darwin has no counterpart of; its home-manager module, which system modules cannot import, and whose terminfo and ssh guard it writes into a home (core.18).

### Decision 5: in the flake's shell

The flake's shell's `PATH` holds Rex beside herdr. Flume in the shell drives the terminal its settings name from the `PATH`, so a host whose Flume settings name Rex finds it there, as it finds herdr.

Not taken: leaving Rex out, as Ghostty is, which is an app the person opens from a terminal they are already in, not a command Flume runs.

### Decision 6: not in `herdr-plugins` or the quick start

Rex has no herdr plugin, and the quick start's steps run none of it. A Mac set up by `boomdrive setup` may add `github:boomdrivedev/boomdrive#rex` by its profile line, which the README names, as it names worktrunk's and timber's.

## Risks and trade-offs

- Rex is unfree and in tester beta, its preview releases weekly. The set holds the release Flume's Rex profile was checked against, never the channel's newest, and moves only with Flume's pin.
- The family's cache holds Rex's binary as Rex's flake packages it, as record 0023's amendment says, and serves it to every host that trusts the cache. A host that does not downloads the same binary from Superlogical's, the URL Rex's `sources.json` names: there is no source to build it from.
- Every host that switches to a set with this change gets `rex` in its system profile. A host that wants none sets `services.boomdrive.rex.package = null`.
