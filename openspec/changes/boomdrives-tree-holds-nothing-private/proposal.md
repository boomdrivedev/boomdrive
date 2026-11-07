# Proposal

## Why

The repository is to open (record 0012), and the public repository starts from this tree with a fresh history (record 0014), before Boomdrive's site moves into it (record 0025). Record 0014's rule, core.41, says what the tree may hold: no text of a private repository, which it cites by its name and an id at most; no host, partition, state repository, person's path or account of anyone's in its code, tests, fixtures or docs; neutral names in tests; and every fixture of a tool's answer captured on a scratch host with its paths made neutral.

The tree as it stands holds what the rule rules out. crew's tests, the runner's tests and the README name the operator's hosts, partitions and state repositories, and a personal account as the owner of their example repositories. One of Flume 0.4.1's answers and four of sluice 0.61.1's were read on the operator's own host, under the operator's configuration, and hold its paths, its account, its pane addresses and its private checkouts; the rest were read on a scratch host whose directory holds the operator's home path. Records quote the cloud's records and ledger and afterthought/swancloud's rules, name the operator's state repository with a signal id, and name agents after the operator's partitions. A design sketch is one real host's whole configuration. The specs' scenarios and the archived changes carry the same names.

## What Changes

- **Neutral names, one vocabulary.** Every host, partition, state repository and account in code, tests, specs and docs takes the neutral names the hosted implementation's tests already use: partitions `home` and `work`, hosts `host-a` and `host-b`, the owner `example` for a repository no public owner holds, state repositories `example/home-state` and `example/work-state`, and a Flume pool `pool-a`.
- **Fixtures captured on a scratch host with neutral paths.** Flume 0.4.1's answers read on a scratch host keep every byte as Flume printed it but the scratch directory's prefix, which becomes `/private/tmp/scratch/host/` in every form the answer gives it. The answers read on the operator's own host are read again, at the same version, on a scratch host rooted there: Flume's status of a place whose agent is live in a herdr pane, and sluice's entries of a landed, a failed, a withdrawn and a missing submission. Each fixture README says where and how each was read, naming no machine.
- **Private repositories cited, never copied.** A record's or rule's text that quotes, or restates sentence by sentence, the cloud's records, ledger or domain model or afterthought/swancloud's rules becomes a citation by name and id, with the point said in this repository's own words where the record needs it. The user's own words stay. A state repository, its branches and its signal ids leave the records' sources, and an agent named after a partition is named by its role.
- **The design sketch of one host becomes a neutral one.** The Nix sketch in `design/the-flakes-configuration/`, named for the operator's host, becomes `host.nix`, the same shape with neutral owners, streams, pools, accounts, partitions and machine, and what cites it follows.
- **The archived changes** are scrubbed in place to the same names and citations, as the history they are.
- **The specs** take the neutral names through this change's deltas: each requirement whose scenarios name a private value is modified with the neutral one, and nothing else of it changes. No requirement is added.
- Nothing is added to the gate: no check, test or task. core.41 stays the rule a reviewer holds every diff to.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `crew-control-plane`: the scenarios of seventeen requirements name the partitions `home` and `work`, the hosts `host-a` and `host-b`, and the state repositories and example repositories under `example`.
- `work-into-flume`: the scenarios of eight requirements name the launches, streams, intents and recipients under `home`, the repository `example/crew`, and the pool `pool-a`.
- `views`: the workspace scenarios name the owner `example`.
- `branch-commands`: the branch command's scenario names `example/crew`.
- `tool-records`: the inbox's scenario names `example/crew`.

## Impact

- **Code and tests**, each crate's tests and the unit tests in its sources: `crates/boomdrive-crew`, `crates/boomdrive-control`, `crates/boomdrive`. `crates/boomdrive-hosted` reads the fixtures and changes only where they do; its protocol's version 1, spoken by a release (core.42), holds nothing private and is unchanged.
- **Fixtures**: `crates/boomdrive/tests/fixtures/flume-0.4.1/` and `crates/boomdrive/tests/fixtures/sluice-0.61.1/`, and their READMEs.
- **Docs**: `README.md`, records under `docs/adr/`, `design/`, and `openspec/changes/archive/`. `docs/architecture/core.md` cites private repositories by name and id alone, and no rule changes.
- **Specs**: the five capabilities above, through this change's deltas.
- No crate, tool, tracked file type, task or rule is added, and no behaviour of the runner changes.
- Out of scope: the fresh history and the opening itself, the rename to `agentplot/boomdrive-archive`, the site's move (record 0025), and crew's own scrub of its README and specs (record 0014's consequences).
