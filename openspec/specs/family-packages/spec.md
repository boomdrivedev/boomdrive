# family-packages Specification

## Purpose
The flake's package set: the runner at this tree, and Flume, sluice and their herdr plugins at release tags, each a derivation of the source the flake's lock names by its hash, for `aarch64-darwin`, `aarch64-linux` and `x86_64-linux`. It covers what the set holds, how each package is made and versioned, the runner's git crates' hashes, how a set is proven by the runner's tests with the set's own binaries, and how a host overrides a tool (core.44, core.45; record 0020).

## Requirements

### Requirement: The flake gives the family's host tools as packages

The flake SHALL give, for each of `aarch64-darwin`, `aarch64-linux` and `x86_64-linux`, the packages `boomdrive`, `flume`, `sluice`, `flume-herdr-plugin` and `sluice-herdr-plugin`, with `default` the runner's. It SHALL give no package of herdr or of git, which are the host's own, and none of a tool a repository pins.

#### Scenario: Every system's packages

- **WHEN** the flake's outputs are evaluated for each of the three systems
- **THEN** each gives the five packages and `default`, each a derivation, and `default` is `boomdrive`, and the flake gives no other system

#### Scenario: No tool of the host's own

- **WHEN** the flake's packages are listed for any system
- **THEN** none is herdr or git

### Requirement: The flake and its lock name one set

Each tool of the set but the runner SHALL be named in `flake.nix` by a release tag, and the lock SHALL lock that tag. A tag in `flake.nix` that the lock does not lock SHALL fail the flake's type check, naming the tool and both tags.

#### Scenario: A tag moved without the lock

- **WHEN** `flake.nix` names sluice's tag `v0.68.0` and the lock locks `v0.67.0`
- **THEN** the flake's type check fails, naming sluice, `v0.68.0` and `v0.67.0`

### Requirement: Each package is made from its release tag's source

Each package SHALL be a derivation of the source the lock names by its hash: the runner's of this tree's crates, `Cargo.toml` and `Cargo.lock`, and Flume's and sluice's of the tool's release tag, sluice's through sluice's own flake. Each tool's version SHALL be its tag's without the `v`, read from its tagged tree where the tree carries it, and from the flake's tag where it does not. The runner's SHALL be its tree's, `Cargo.toml`'s workspace version.

#### Scenario: Flume at its tag

- **WHEN** the lock names Flume's tag `v0.10.0`
- **THEN** the `flume` package's source is that tag's tree and its version is `0.10.0`

#### Scenario: sluice through its own flake

- **WHEN** the lock names sluice's tag `v0.68.0`, whose tree says `0.0.0`
- **THEN** the `sluice` package is sluice's flake's, its version `0.68.0`, and its build is given `0.68.0` as sluice's release build is

#### Scenario: The runner from this tree

- **WHEN** the flake is evaluated
- **THEN** the `boomdrive` package's source holds this tree's `Cargo.toml`, `Cargo.lock` and `crates`, and no other path of the tree

#### Scenario: The runner at its tree's version

- **WHEN** the flake is evaluated for each system it gives
- **THEN** the `boomdrive` package's version is the one `Cargo.toml` gives under `[workspace.package]`: `0.0.0` at a commit of `main`, and the release's at a release tag

#### Scenario: An overridden input

- **WHEN** the flake's outputs are evaluated with Flume's input replaced by a tree whose `Cargo.toml` says `9.9.9`
- **THEN** the `flume` package is made from that tree, its version `9.9.9`

### Requirement: The runner's git crates are hashed at the revision the lock names

Each git package of `Cargo.lock` SHALL have one entry among the runner's package's fixed hashes, holding the revision its hash was taken at. The flake's test SHALL fail where a git package has no entry, an entry names no git package, or an entry's revision is not the one `Cargo.lock` names.

#### Scenario: weir moved without its hash

- **WHEN** `Cargo.lock` names weir's git crates at a revision other than the one their entry holds
- **THEN** the flake's test fails, naming the crate and both revisions

### Requirement: A tool's herdr plugin is the tool's, at the tool's tag

Each herdr plugin package SHALL be its tool's `plugins/herdr` folder, taken from the source of the tool's own package, as the tool ships it, with the tool's version.

#### Scenario: Flume's plugin moves with Flume

- **WHEN** the lock names Flume's tag `v0.10.0`
- **THEN** `flume-herdr-plugin` is that tag's `plugins/herdr`, its version `0.10.0`

### Requirement: The set is the one the runner's tests pass with

The gate SHALL run the set's own Flume and sluice, the flake's packages of its lock, given to the runner's tests by their paths. Those tests SHALL check each program's version against its tag, and read the answers a scratch host gives with the runner's own readers. No test of the set SHALL run a Flume or a sluice found on the host's `PATH`.

#### Scenario: Each tool's version

- **WHEN** the runner's tests run in the gate
- **THEN** the set's Flume says `flume <Flume's tag without the v>` and its sluice says `sluice <sluice's tag without the v>`

#### Scenario: Flume's status of a scratch host

- **WHEN** the set's Flume is asked `--json status` on a scratch host that holds no place
- **THEN** the runner reads the answer as Flume's status, with no place and the host part as Flume printed it

#### Scenario: The set's Flume keeps the runner's watch

- **WHEN** the set's Flume is asked `--json settle --every 60s` on a scratch host that holds no place, as the runner asks it
- **THEN** the runner reads the answer as given, and the set's Flume's `--json status` then gives the host's watch with a bound of 60 seconds

#### Scenario: The set's Flume takes the runner as a recipient

- **WHEN** the set's Flume, on a scratch host, is asked `--json launch` with `--recipient scratch/planner --recipient boomdrive` in a folder that holds no repository, and `--json confirm --recipient=boomdrive -- <a key no end is under>`, each as the runner asks it
- **THEN** each exits 1 with Flume's refusal, which the runner reads as Flume's reason, and neither is a usage error; and the set's Flume is no older than the least the runner names

#### Scenario: A submission sluice has no record of

- **WHEN** the set's sluice is asked, in a scratch repository, for the entry of a submission it has no record of
- **THEN** the runner reads its answer as a missing entry

#### Scenario: Not the host's tools

- **WHEN** the runner's tests of the set run in a shell whose `PATH` holds another `flume` and `sluice`
- **THEN** they run the set's, by their paths
