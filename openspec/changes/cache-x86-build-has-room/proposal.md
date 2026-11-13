# Proposal

## Why

The brief: "Boomdrive's cache workflow (.github/workflows/cache.yml) has failed at every release from v0.8.4 to v0.9.2, only on 'build (x86_64-linux)' on ubuntu-24.04: 'No space left on device' [...] Give the x86_64-linux build room: free the hosted runner's disk before Nix builds (remove the preinstalled toolchains it doesn't use, by a pinned maintained action or a step of its own) or build it on a larger runner, keeping the matrix's other systems as they are and the nix project's test of this file (nix/tests/workflow.nix) passing."

The runs fail in the build, five to eight minutes in, each with Nix's note that the build failure may have been caused by lack of free disk space, a note Nix gives only when the store's or the build's file system is out of bytes: Rust's `target` folders under `/build` (`plannotator-tui`, `herdr-annotate`) and a bun package's unpacking cannot write. Nix's store and its build folders are on `/`. An x64 `ubuntu-24.04` runner leaves about 88GiB of its 145GiB `/` free (nothing-but-nix's own test of `ubuntu-24.04`, 2026-11-12: 87875MB), and the arm64 runner, whose image holds fewer toolchains, about 106GiB, on which the same set builds and is pushed at every release. The x64 image's toolchains that no package's build uses, the Android SDK, .NET, GHC, Swift, Rust's own and the tool cache among them, hold about 30GiB more.

## What Changes

- The `x86_64-linux` job's first step removes, before Nix is installed, the hosted runner's preinstalled trees that no package's build uses and dpkg does not own: `~/.cargo`, `~/.rustup`, `/opt/ghc`, `/opt/hostedtoolcache`, `/usr/local/.ghcup`, `/usr/local/lib/android`, `/usr/share/dotnet` and `/usr/share/swift`, which nothing-but-nix's Carve measures at about 27GiB on x64, and `/usr/share/az_*`, `/usr/share/miniconda`, `/usr/share/gradle-*`, `/usr/share/kotlinc`, `/usr/share/sbt`, `/usr/lib/heroku` and `/opt/pipx*`. It says the free space of `/` before and after, so the run's log shows the room the build had. It is the job's only step with a condition, and no other system's job runs it.
- The requirement "The workflow builds and pushes every package of every system" says so, with a scenario, "The x86_64-linux job has room", which the nix project's test holds the workflow's text to.
- Record 0020 is amended, and core.47's details name the step as not ruled out.

### What stays

- Every system's runner, `x86_64-linux` on `ubuntu-24.04` among them; Nix's store and its build folders, on `/` as before; every other step, its order and its tokens; `aarch64-linux`'s and `aarch64-darwin`'s jobs, which run no new step.

### Not taken

- **nothing-but-nix.** A maintained action that pools the runner's free space into a compressed Btrfs `/nix` of up to about 129GiB on x64. It needs Nix's `build-dir` moved onto that volume, its newest release, v12, is a week old and mends v11's breaking of `cachix/install-nix-action`, and it runs a purge as root in the background of the job that holds the cache's push token. A step of the workflow's own gives the x64 job more room than the arm64 job builds the set in, with no new party and no change to Nix's settings. Should a run show the room is not enough, it is the next step.
- **A larger runner.** The spec names `ubuntu-24.04` for `x86_64-linux`, and a larger runner is a paid runner the organisation configures, which is the user's to decide.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `binary-cache`: the `x86_64-linux` job frees its runner's disk before Nix is installed.

## Impact

- `.github/workflows/cache.yml`: the step.
- `nix/tests/workflow.nix`: the scenario's case.
- `openspec/specs/binary-cache/spec.md`, by this change's delta once archived.
- `docs/adr/0020-*.md`: its amendment of 2026-11-13. `docs/architecture/core.md`: core.47's details.
- The proof is a run of `cache` whose `build (x86_64-linux)` passes. The environment `cachix` admits `main` and tags `v*` alone (core.47), so a run on this branch does not start its jobs; the first run on `main` after this lands, or a run by hand there, is the proof.
