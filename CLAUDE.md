# Boomdrive

The thin runner on each host, one binary named `boomdrive`. It keeps the
clones and views of the repositories its configuration names, and turns
the control plane's work into `flume launch` calls, carrying their
results back. It reaches the control plane through one interface,
`crates/boomdrive-control`, and carries two implementations of it:
crew's, `crates/boomdrive-crew`, which reads and writes records in crew's
state repository, and a hosted control plane's, `crates/boomdrive-hosted`,
which keeps one outgoing WebSocket for each partition the host is
enrolled in, and holds the runner's protocol with it, one JSON Schema per
version under `protocol/`. The hosted table, `[control_plane.hosted]`,
names the control plane's `url` and its partitions, each a `label` and
the `repositories` it is served, and holds no token or key.
`boomdrive enroll <label>` enrolls a partition once: it reads the
one-time token from standard input, makes the partition's key on the
host and keeps it only in the host's credential store, the macOS
keychain or `systemd-creds`.
`boomdrive run` keeps the repositories its configuration names: a bare
clone of each in its data directory, fetched on a timer, and a view of
each under the repository root. Where the configuration names a control
plane, it makes control passes on a timer of their own: it takes each
launch offered under a lease while its Flume stream has room, carries
out each act and command once, and carries Flume's, sluice's and the
inbox's records back as the tools gave them.

The flake at the root, `flake.nix` and `flake.lock`, with everything
else of it under `nix/`, gives the runner, Flume and sluice pinned
together, and the runner's service as nix-darwin and NixOS modules,
`services.boomdrive`, whose `settings` are the runner's file. The
workflow `.github/workflows/cache.yml` builds every package of every
system the flake gives and pushes it to the family's cache, `boomdrive`
on Cachix, at each push to `main` and each tag `v*` (record 0020,
core.44 to core.47).

## The family

**The family.** This tool works with the others of the Boomdrive family:
sluice (the merge queue), Flume (places and their agents), crew (teams),
Boomdrive (the runner), Boomdrive Cloud (the control plane), timber,
weir and boom. Each tool's statement is in agentplot/blueprints,
`design/<tool>/requirements.md`. Before designing or fixing a behavior
here, look at how the related tools do the same thing, in their code and
their statements: readiness, configuration, credentials, retries, what a
place is. Where two tools do one thing, reconcile them into one design
with one owner the others call or cite, and say so in the design. Never
fix one tool alone when the cause is that the tools differ.

**Specs.** `openspec/specs/` changes only through an OpenSpec change: a
proposal, a delta spec, and tasks, validated with `openspec validate
--strict` and archived with `openspec archive`, so the spec reconciles.
This holds for a fix or a chore too. Where the work changes a
requirement, it carries that change in the same branch. Never edit a
spec file directly.

## Rules

The rules are in `docs/architecture/`: `core.md` binds wherever a change
is made, and an area page, when one exists, binds the paths its
frontmatter names. The decisions behind them are `docs/adr/`, one MADR
record each. A design or a commit that relies on a rule cites its id;
one that departs from a rule says which and why. A rule is sent to
`core.md` or its area page with an id, the next free number in its area,
amended in the same commit as the decision that changes it.

## Checks

Every tool runs through the shell, one command at a time:
`devenv shell -- <command>`. moon runs the checks, tagged by layer
(`.moon/workspace.yml`); `.config/sluice.toml` says which layers each
branch requires:

| layer | what | who requires it |
|---|---|---|
| `check-fast` | the crate's format (rustfmt), lint (clippy -D warnings), type check and tests; the flake's format (nixfmt), lint (statix, deadnix, actionlint), type check and tests, evaluated and never built; the root's format (treefmt), toolchain, records, specs and open changes | main |

Run suites through sluice, so the agents on this host take turns under
its bound: once your work is committed, `sluice exec main` for the
gate's checks, and while iterating `sluice exec main -- moon run
<project>:<task>`. The projects are those `.moon/workspace.yml` lists:
`boomdrive` (`crates/boomdrive`), `boomdrive-control`
(`crates/boomdrive-control`), `boomdrive-crew` (`crates/boomdrive-crew`),
`boomdrive-hosted` (`crates/boomdrive-hosted`), `nix` (`nix/`, which
owns the flake and `.github/workflows/cache.yml`) and `root`. A quick,
narrow call, `cargo check` or one unit test, is fine to run directly.
`devenv shell -- moon run :#check-fast` runs the fast layer.

## Commits

Stage by pathspec and commit the paths you wrote: `git add <paths>`, then
`git commit -m "<subject>" -- <paths>`. Never `-a`, never `add .`.
Conventional Commits subjects, scope the part of the tree touched.

@docs/architecture/core.md
