# Design

## Context

Record 0023, as amended on 2026-11-08 on the user's card ("worktrunk (wt), portless (the .localhost dev addresses)"), puts worktrunk in the set:

- **What it is for.** The flake installs worktrunk's `wt`, which crew and sluice's worktree driver call.
- **Overridable as herdr is.** It is the set's by default. A host replaces it through the modules' options, or drops it for its own.

Who calls `wt` on a host:

- **sluice.** Its package gives `wt-submit`, which worktrunk runs as `wt submit`, sluice's worktree driver (sluice's C.5).
- **crew.** It removes a worktree with the kit's own `wt remove`, and lands a bolt with `wt submit main`. Its README says a host whose path has no `wt` keeps the worktree, named.
- **Not the runner.** Its code calls no `wt`, and this change gives it none to call.

What each tool pins, as seen on 2026-11-11 at the tags the lock names:

- **sluice `v0.78.1`** pins worktrunk `0.80.0` in its `devenv.nix`. It is the `version` of a block `worktrunk = fromRelease { name = "worktrunk"; repo = "max-sixty/worktrunk"; version = "0.80.0"; … }`, whose comment calls it "the version they are tested against". sluice's tools tier and e2e tests drive that release archive.
- **crew `v0.50.2`** pins none: its `devenv.nix` says "herdr, wt and openspec come from the machine".
- **Flume `v0.21.2`** calls no `wt`.

What worktrunk gives, at its release tag `v0.80.0`:

- **A flake** with `packages.<system>.worktrunk`, also `default`, for `x86_64-linux`, `aarch64-linux` and `aarch64-darwin`, the three systems this flake gives. It also gives `worktrunk-with-git-wt`, `apps`, `checks`, a `devShells.default` and `homeModules.default`, a home-manager module, `programs.worktrunk`. It gives no nix-darwin or NixOS module.
- **Its inputs** are `nixpkgs`, `rust-overlay`, `crane` and `flake-utils`, which brings `systems`.
- **The package.** It is a crane build with tests off. Its `meta.mainProgram` is `wt`, and its version is its tagged `Cargo.toml`'s, `0.80.0`. Its toolchain is the channel its `rust-toolchain.toml` names, `1.97.0`, taken from `rust-overlay`.
- **Its `--version`.** `wt --version` prints vergen's `VERGEN_GIT_DESCRIBE` where it is set, and Cargo's version where it is not (`src/cli/mod.rs`, `version_str`). worktrunk's flake sets it to its source's short revision, `b49ca7e` at `v0.80.0`. A release build, made in a checkout at the tag, gets `v0.80.0` from `git describe`. nixpkgs' recipe sets none, so its `wt` says `0.80.0`.
- **Under the checks.** With the flake's `nixpkgs` and `rust-overlay` followed, the package evaluates on each of the three systems against the nix project's dummy store, offline and read-only. Its `Cargo.lock` holds no git source, so its vendored crates need no network at evaluation. The flake's locked `rust-overlay` carries `1.97.0`. Seen on 2026-11-11 with a scratch lock of this flake and worktrunk's input.

The flake today, after the bolt `the-boomdrive-flake-configures-the-whole-host`:

- **The set** is `boomdrive`, `boomdrive-launcher`, `crew`, `flume`, `flume-herdr-plugin`, `ghostty`, `herdr`, `sluice` and `sluice-herdr-plugin`, with `default` the runner's.
- **`nix/modules/apps.nix`** declares the Boomdrive module's own options for herdr, its plugins, Ghostty and the launcher, each `installed` from the set by default, a host's own replacing it and `null` dropping it. With the runner enabled, each non-null package goes in `environment.systemPackages`.
- **The shell**, `nix/shell.nix`, holds the set's runner, Flume, sluice, crew and herdr.
- **The cache's workflow** builds every package `nix eval` lists for each system, so a package the set gains is built and pushed with no change of the workflow's.

This change is the unit `worktrunk-comes-with-the-flake` of the bolt `the-flake-brings-its-companion-tools`.

## Goals / Non-Goals

**Goals:**

- The set gains worktrunk, through its own flake at the worktrunk sluice's tree pins, reporting its tag's version. The cache's workflow builds and pushes it.
- A host that turns on `services.boomdrive` gets `wt` in its system profile, and can replace it with its own or drop it.
- The flake's shell has `wt` beside the crew and sluice that call it.
- The runner's code does not change.

**Non-Goals:**

- worktrunk's own files:
  - the person's `~/.config/worktrunk/config.toml`, a repository's `.config/wt.toml`, and worktrunk's approvals of a repository's hooks;
  - worktrunk's shell integration, which lets `wt switch` change the shell's folder, and which the person installs with worktrunk's own `wt config shell install`.
- `git wt`, which worktrunk's `worktrunk-with-git-wt` gives for Windows, where `wt` is Windows Terminal's.
- Holding worktrunk to a floor of crew's, which pins none.
- portless, the bolt's other unit, `portless-comes-with-the-flake`.
- Any change to the runner's code, its file, its service, or what the runner's tests prove of the set.

## Decisions

### 1. worktrunk comes through its own flake, at the worktrunk sluice's tree pins

The flake takes:

```nix
worktrunk = {
  url = "github:max-sixty/worktrunk/v0.80.0";
  inputs = {
    nixpkgs.follows = "nixpkgs";
    rust-overlay.follows = "rust-overlay";
  };
};
```

worktrunk's `crane` and `flake-utils` stay as worktrunk's lock names them. This flake has no input of either name to follow, and neither carries a nixpkgs.

- **Pinned to sluice's.** "The version sluice's worktree driver was checked against" is the `version` of the `worktrunk` block of sluice's `devenv.nix`, at the tag the lock names for sluice. The test reads it from sluice's input and holds the set's worktrunk to it. A sluice tag that pins a newer worktrunk fails the gate until worktrunk's tag moves with it, as a Flume tag does for herdr.
- **Named by a release tag.** `nix/tests/tags.nix` holds worktrunk to one, as it holds Flume, sluice, crew and herdr (core.45).
- **A host's tool, not a repository's pin.** worktrunk is the host's `wt` that crew and sluice's driver call (record 0023 as amended on 2026-11-08). sluice's `devenv.nix` pins it for sluice's own tests, and the set reads that pin to choose the release, as it reads Flume's pin of herdr. It is no tool a kit repository pins in its `.prototools` and sluice equips a place with, which core.44 and core.37 keep out of the set (record 0009 as amended on 2026-11-05).

Alternatives:

- **nixpkgs' worktrunk `0.80.0`.** Not taken: core.44 takes a tool from its own flake where it publishes one, and nixpkgs' moves with nixpkgs, not with a tag the lock names.
- **The release archive sluice's `devenv.nix` fetches.** Not taken: core.44 rules out a prebuilt binary in place of a derivation the flake can make.
- **worktrunk's own `nixpkgs` and `rust-overlay`.** Not taken: they add a second nixpkgs to the lock and to every evaluation, for a build the flake's own nixpkgs makes.
- **`worktrunk-with-git-wt`.** Not taken: it adds `git-wt` for `git wt`, which no tool of the family calls.

### 2. Its build is given its tag as its `git describe`

`packages.<system>.worktrunk` is made by `nix/packages/worktrunk.nix`, given worktrunk's flake's `packages.<system>.worktrunk` and the tag `flake.nix` names, read by `tag.nix` as sluice's is. It is that package with `VERGEN_GIT_DESCRIBE` set to the tag, `v0.80.0`, and nothing else changed. Its source, its dependencies' derivation, its version and its `meta` are worktrunk's flake's.

So `wt --version` says `v0.80.0`, its tag's version as core.44 has it. That is what worktrunk's release build says, the archive sluice's tests drive, which gets it from `git describe` at the tag. sluice's package is given its tag the same way (`nix/packages/sluice.nix`, "as sluice's release build is"). worktrunk's flake gives its source's short revision, `b49ca7e`, in its place.

It is worktrunk's flake's package with one variable given, as sluice's is given its version, and no recipe of this flake's (core.44). worktrunk's tree carries its version, but its flake's build reports its commit, so core.44's details are amended to name this (Rules amended).

Alternatives:

- **worktrunk's flake's package unchanged.** Not taken: its `wt --version` names a commit, not its tag's version (core.44, core.45).
- **`VERGEN_GIT_DESCRIBE` removed, so `wt` says Cargo's `0.80.0`, as nixpkgs' does.** Not taken: the release sluice checked says `v0.80.0`, and the set says what that release says.

### 3. The modules' own option installs it

`nix/modules/apps.nix` declares `services.boomdrive.worktrunk.package`, `installed "worktrunk"` as herdr's and Ghostty's are:

| option | type | default |
|---|---|---|
| `services.boomdrive.worktrunk.package` | package or `null` | the set's `worktrunk` for the host's system |

With `services.boomdrive.enable`, `environment.systemPackages` holds it where it is not `null`, beside herdr, Ghostty and the launcher. A host's own package replaces the set's, and `null` drops it for the host's own. Without the runner none is installed. Its description says what calls it, and its `defaultText` names the set's package as its neighbours' do.

The option is the Boomdrive module's own because worktrunk gives no nix-darwin or NixOS module, and neither crew's nor sluice's module installs it. The service-modules requirement that the modules hold no option installing a tool its own module installs lets it, as it lets herdr's.

The modules write no file of worktrunk's (core.18): no configuration, no approval, no shell integration.

Alternatives:

- **worktrunk's `homeModules.default`.** Not taken: it is a home-manager module, which nix-darwin's and NixOS's system modules cannot import, and a host may run no home-manager. Its `programs.worktrunk` also writes the person's shell integration, which is their own.
- **sluice's module installing it.** Not taken: crew calls `wt` too, and sluice's module is sluice's repository's. It would ask another repository for what this flake can do with the worktrunk it pins.
- **`programs.worktrunk` as the option's name.** Not taken: a later nixpkgs or nix-darwin module of that name would collide with it, and the option sits under `services.boomdrive` beside herdr's, Ghostty's and the launcher's.
- **Installed whether or not the runner is enabled.** Not taken: a host that imports the module for one tool alone would get a `wt` it did not ask for.

### 4. The shell holds worktrunk

`nix/shell.nix`'s `packages` gain `set.worktrunk`, after `set.herdr`, and its header says so. crew and sluice are on the shell's `PATH` and call `wt`: `wt submit` runs sluice's `wt-submit` only with a `wt` beside it.

Alternative: **leaving it out, as Ghostty is.** Not taken: Ghostty is a window the person in the shell is already in, and `wt` is a command the shell's tools run.

### 5. The tests

Every evaluated case is under `nix/tests`, run by `nix:typecheck` or `nix:test` against no store (record 0020's Decision 14), with core.41's neutral names. No case builds or runs worktrunk. The lock's new nodes reach the checks through `nix/inputs.nix`, which reads every node of the lock, with no change of its own.

- **`typecheck.nix`.** The set gains `worktrunk`, so "Every system's packages" holds the ten and `default`.
- **`packages.nix`** holds the family-packages delta's cases:
  - "worktrunk through its own flake" and "worktrunk named by no release tag";
  - "worktrunk is sluice's" and "worktrunk other than sluice's", through a function called with the set's values and with given ones.
- **`tags.nix`'s** tools named by a release tag gain `worktrunk`.
- **`apps.nix`** holds the service-modules delta's added cases, each on nix-darwin and on NixOS. Each case's name is new to the tests, since `default.nix` merges every file's cases into one set.
- **`cache.nix`.** The host's overlay `names` gain `worktrunk`, so "A host's own nixpkgs" holds it to the flake's own. "The README's words" reads `services.boomdrive.worktrunk.package`.
- **`way-in.nix`.** "Every system's shell" holds the shell's six packages.
- **"The runner is pushed with its tools"** holds as it stands: `workflow.nix` holds the workflow to build every package `nix eval` lists, and `typecheck.nix` holds `worktrunk` among them.

### Rules relied on

- **core.44 and core.45.** worktrunk comes from its own flake at a release tag, moves only with the lock, is held to sluice's pin, and reports its tag's version (Decisions 1 and 2).
- **core.47.** The package is made with the flake's own nixpkgs and rust-overlay, which its input follows, and the workflow builds and pushes it with no change of its own.
- **core.37.** The set's worktrunk is a derivation from source, no release archive.
- **core.18 and core.46.** The modules write no file of worktrunk's, and their option holds no credential (core.4).
- **core.1 and core.13.** The runner calls no `wt` and starts nothing of worktrunk's. worktrunk runs only when crew, sluice or a person runs it.
- **core.12.** The option names what it installs, `worktrunk`, worktrunk's own name.
- **core.7 and core.8.** No crate, tool of the gate or tracked file type is added: `.nix` is formatted already, and the checks evaluate worktrunk without running it.
- **core.41.** Neutral names in every case.

### Rules amended

In the commit that writes this change:

- **core.44's details** do not rule out worktrunk through its own flake at its release tag, the worktrunk sluice's tree pins, its build given its tag as its `git describe`, replaceable or dropped through the modules' options. They say it is a host's tool, which crew and sluice's driver call, and not a tool a repository pins.
- **core.46's details** do not rule out the runner's module's own option for worktrunk, which gives no nix-darwin or NixOS module.
- **Record 0023** gains an amendment of 2026-11-11 holding Decisions 1 to 4 as the writing agent's choices, each cheap to change, with the path not taken.

### Rules departed from

None.

## Sequencing

The bolt's sibling unit `portless-comes-with-the-flake` adds another tool to the set the same way. It touches the same lines:

- `flake.nix`'s inputs and set, and `flake.lock`;
- `nix/modules/apps.nix`'s options and `environment.systemPackages`;
- `typecheck.nix`'s set, `tags.nix`'s list, and `cache.nix`'s `names` and README options;
- the README's install with nix, CLAUDE.md and `openspec/config.yaml`;
- core.44's and core.46's details, and record 0023;
- in the specs, family-packages' "The flake gives the family's host tools as packages", service-modules' "The modules hold no option that installs a tool its own module installs" and both binary-cache requirements, which each change modifies whole.

Whichever lands second merges those lines to hold both tools, locks the merged `flake.nix` with `nix flake lock`, and writes its modified requirements over the first's archived specs, naming both tools. Its `nix:typecheck` and `nix:test` hold both. Each change keeps its own options' cases under names of its own, so neither's cases replace the other's.

## Risks / Trade-offs

- **Two worktrunks on a host.** A host that keeps a `wt` of its own in a user's profile, home-manager's `programs.worktrunk` among them, has it ahead of the system profile's on the runner's `PATH` and the person's. The set's is shadowed and unused. Such a host sets `services.boomdrive.worktrunk.package` to `null`, or to its own, which the README says.
- **worktrunk's build in the cache.** Each system's job builds worktrunk's dependencies and then worktrunk with crane, a Rust build, until its paths are pushed.
- **worktrunk's toolchain follows the flake's rust-overlay.** A worktrunk tag whose `rust-toolchain.toml` names a channel the locked rust-overlay lacks fails `nix:typecheck` until rust-overlay moves. That holds back worktrunk's tag, never a host.
- **sluice's pin is read from its `devenv.nix`.** A sluice tag that writes the pin another way fails the test, naming the file, until the reading moves with it.
- **No shell integration.** A person's `wt switch` changes no folder until they run `wt config shell install`, worktrunk's own command. Agents and crew call `wt` without it.

## Migration Plan

A host takes the change by moving its Boomdrive input. With `services.boomdrive.enable`, its first switch installs the set's `wt` in the system profile.

- A host with a `wt` of its own sets `services.boomdrive.worktrunk.package` to its own, or to `null` to keep its own out of the modules.
- afterthought/swancloud, once it takes the flake, either drops its hosts' own worktrunk for the set's, or passes its own through this option (record 0023).

## Open Questions

None.

## Task notes

**1.1** In `flake.nix`:

- Add the `worktrunk` input of Decision 1, after `herdr`, with a comment saying it is the worktrunk sluice's tree pins, which sluice's worktree driver was checked against (record 0023; design Decision 1). Lock it with `nix flake lock --update-input worktrunk` in the devenv shell, outside the sandbox. The lock gains `worktrunk`, `crane`, `flake-utils` and `systems`, and no second `nixpkgs` or `rust-overlay`.
- `packagesOf`'s `tools` gains:

  ```nix
  worktrunk = import ./nix/packages/worktrunk.nix {
    package = inputs.worktrunk.packages.${system}.worktrunk;
    tag = tagOf (import ./flake.nix).inputs.worktrunk.url;
  };
  ```

- The header comment and the description name worktrunk among the tools at a release tag through its own flake.

`nix/packages/worktrunk.nix` is `{ package, tag }: package.overrideAttrs { VERGEN_GIT_DESCRIBE = tag; }`, its header comment saying Decision 2: worktrunk's release build gets its tag from `git describe`, and worktrunk's flake gives its build the commit's short revision, so the set gives it the tag flake.nix names.

`typecheck.nix`'s `set` gains `"worktrunk"` in name order, last.

**1.2** In `packages.nix`:

- "worktrunk through its own flake", for each system, with `own = inputs.worktrunk.packages.${system}.worktrunk`:
  - `toString worktrunk.src == toString own.src`, and `worktrunk.cargoArtifacts.drvPath == own.cargoArtifacts.drvPath`;
  - `worktrunk.meta.mainProgram` is `wt`, and `worktrunk.version` is `t.lockedVersion "worktrunk"`;
  - `worktrunk.drvAttrs.VERGEN_GIT_DESCRIBE` is `"v" + t.lockedVersion "worktrunk"`.
  
  Each failure names the system and what it found.
- `worktrunkHeld = { pinned, worktrunk }: [ failures ]`. It gives one failure where they differ, naming both: "the set's worktrunk is <worktrunk>, and sluice's tree pins <pinned>, the worktrunk its worktree driver was checked against".
- `sluicesWorktrunk` reads `${inputs.sluice.outPath}/devenv.nix`, split into lines:
  - It finds the one line matching `[[:space:]]*repo = "max-sixty/worktrunk";[[:space:]]*`.
  - It takes the lines after it up to the first matching `[[:space:]]*};[[:space:]]*`, and among them the one matching `[[:space:]]*version = "([^"]+)";[[:space:]]*`.
  - It is that version, or null where there is not exactly one such repo line, or not exactly one such version line in its block.
- "worktrunk is sluice's": where `sluicesWorktrunk` is null, one failure, "sluice's pin of worktrunk could not be read: <file> holds no one worktrunk block naming its version". Otherwise it is `each` system's `worktrunkHeld { pinned = sluicesWorktrunk; worktrunk = packages.worktrunk.version; }`.
- "worktrunk other than sluice's" calls `worktrunkHeld { pinned = "0.81.0"; worktrunk = "0.80.0"; }`. It holds that there is exactly one failure, naming both.
- "worktrunk named by no release tag" is built as "herdr named by no release tag" is, with worktrunk's URL `github:max-sixty/worktrunk/main` and the lock's node's `original.ref` `main`.

In `tags.nix`, the list of tools named by a release tag gains `"worktrunk"`, and its header names it.

**2.1** In `nix/modules/apps.nix`:

- `options.services.boomdrive.worktrunk.package = installed "worktrunk" "<description>"`. The description: "worktrunk's `wt`, which crew and sluice's worktree driver call, installed with the runner; a host's own replaces it, and `null` installs none."
- `environment.systemPackages`' list gains `cfg.worktrunk.package`, after `cfg.ghostty.package`.
- The header comment names worktrunk among the apps, which gives no nix-darwin or NixOS module, citing record 0023 as amended on 2026-11-08 and this design's Decision 3.

In `nix/tests/apps.nix`, each case `onEach`, as its neighbours are:

- "worktrunk with the runner": `system { }` evaluates with no failure, and its profile holds `set.worktrunk.outPath`.
- "A host's own worktrunk": with `services.boomdrive.worktrunk.package` a `pkgs.runCommand "own-worktrunk" { } "mkdir -p $out/bin"`, given as a module function as `ownHerdr` is, the profile holds the option's `outPath` and not `set.worktrunk.outPath`.
- "No worktrunk of the set's": with the option `null`, the system evaluates with no failure, and its profile holds no `set.worktrunk.outPath`.
- "No worktrunk without the runner": `without`'s profile holds no `set.worktrunk.outPath`.

The file's header names worktrunk and this change.

**2.2** In `nix/tests/cache.nix`, `names` gains `"worktrunk"` in name order, so the host's overlay gives a `worktrunk` of its own, and "A host's own nixpkgs" holds the profile to the set's.

**2.3** `nix/shell.nix`'s `packages` gain `set.worktrunk` after `set.herdr`. Its header names worktrunk among the set's tools on `PATH`, which crew and sluice call. In `nix/tests/way-in.nix`, `shellTools` gains `"worktrunk"`. `flake.nix`'s comment on `devShells` stays true as it stands.

**3.1** The README's **Installing with nix**:

- **The set.** worktrunk joins it: through its own flake at the release sluice's tag pins, which sluice's worktree driver was checked against. The profile line may add `github:agentplot/boomdrive#worktrunk`.
- **The modules.** With `services.boomdrive.enable` they install worktrunk's `wt`, which crew and sluice's driver call.
- **Replacing or dropping.** The block of `null`s gains `services.boomdrive.worktrunk.package = null;`. A host with a `wt` of its own, in a user's profile or home-manager's `programs.worktrunk`, sets it `null` or to its own, so one `wt` runs.
- **The shell.** It names worktrunk with the set's runner, Flume, sluice, crew and herdr.
- **Shell integration.** worktrunk's shell integration, which lets `wt switch` change the shell's folder, is the person's to install with `wt config shell install`. Neither the modules nor the shell install it.

In `nix/tests/cache.nix`, "The README's words"' list gains `"services.boomdrive.worktrunk.package"`, after Ghostty's.

**3.2** CLAUDE.md's paragraph on the flake reads "gives the runner, Flume, sluice and crew pinned together, with herdr, its plugins, Ghostty and worktrunk, …". Its modules "install herdr, Ghostty, worktrunk and the Boomdrive launcher". `openspec/config.yaml`'s stack line names the same.
