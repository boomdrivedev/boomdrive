# Verify: boomdrives-tree-holds-nothing-private

The work on `change/boomdrives-tree-holds-nothing-private`, read at cfdd0df, meets its change, so the change is archived with this report.

- Every task is ticked.
- The gate run through sluice passes every task it ran: 21 `check-fast` tasks and 641 tests, none failed.
- The change modifies 28 requirements across five specs, with 53 scenarios, and adds or removes none. A trial archive changes only names in those requirements, and all 16 specs then pass `openspec validate --specs --strict`.
- Read for a missed name, the tree as the change leaves it holds no host, partition, state repository, person's path or account of anyone's, and no copied text of a private repository. The design's Risks leave this reading to this stage, since no check in the gate makes it.

## Checks

- **The gate.** `devenv shell -- sluice exec main` at cfdd0df, run once, exited 0. sluice ran the checks the commit affects against `main` at 3f428cf, which were 21 tasks:
  - `boomdrive`, `boomdrive-control`, `boomdrive-crew` and `boomdrive-hosted`: `format`, `lint`, `typecheck` and `test` each.
  - `nix:typecheck` and `nix:test`.
  - `root:adrs`, `root:changes` and `root:format`.
  - 641 tests passed, and none failed. sluice recorded the 21 results under their keys for cfdd0df.
  - Not run, because sluice found them unaffected: `nix:format`, `nix:lint`, `root:toolchain`, `root:specs`, `root:commits`, `root:releases` and `root:workflow`. The change touches no nix file, workflow or release script. The landing's own gate runs `root:specs` on the archived specs, and `root:commits` on these commits.
  - moon warned that it could not write its shared cache's blobs or archive its outputs. Neither failed a task.
- **The trial archive.** A copy of the tree at cfdd0df was made outside it. In that copy, the pinned openspec 1.14.3 archived the change: `~ 28 modified` and `+ 0, - 0`. Then `openspec validate --specs --strict` passed all 16 specs.
  - A word diff of the specs before and after holds only names swapped for design Decision 1's.
  - The archived specs hold none of the values replaced.
  - The one `/Users/` path left is the nix tests' neutral user `alice`.

## Tasks

All 21 tasks of `tasks.md`, groups 1 to 7, are ticked `[x]`, and none is open. Each task names a check, and the gate ran each one: `boomdrive-crew:test`, `boomdrive-control:test`, `boomdrive:test` (with its `--lib`, `readme` and `facts` tests), `boomdrive-hosted:test`, `root:adrs`, `root:format` and `root:changes`. What the tasks name is in the tree:

- **crew, the interface, the runner and the README** (groups 1 and 2).
  - The tests and unit tests use design Decision 1's vocabulary: partitions `home` and `work`, hosts `host-a` and `host-b`, `example/home-state`, `example/crew` and the owner `example`, and the pool `pool-a` with `pool-a-1`.
  - `offers.rs` holds that `pool-a` appears nowhere in the hello.
  - The views workspace test names `/h/Code/example/example.code-workspace`.
- **The fixtures** (group 3).
  - Each of the nineteen Flume 0.4.1 answers read on the scratch host that holds a path writes its prefix as `/private/tmp/scratch/host/`, in its `%`-joined form too. The two refusals hold no path.
  - `status-live-agent.json` is one place with no mark, showing its agent's herdr pane `w1:p1` at a socket under the scratch root. Its host part is unbounded, with the one stream `default` and the stand-in pool. `facts.rs`'s `a_live_place_shows_its_agents_pane` names that pane and socket.
  - sluice 0.61.1's four `status-*.json` are as design Decision 3 has them:
    - landed, with its `landed_as`;
    - failed, its row's `flaky.other` naming a worktree;
    - withdrawn by resubmission, its row's `flaky.other` naming a log;
    - missing, the `not_found` error.
  - Each sluice answer's peer is `host-a`.
  - Each fixture README says how its answers were read and names no machine.
  - Nothing under `crates/boomdrive-hosted/protocol/v1/` changed (core.42).
- **The records** (group 4).
  - Records 0002, 0006, 0015 and 0017 cite the cloud's records, rules, roadmap, research rows and ledger items, and afterthought/swancloud's herdr.14 and its record 0031, by id. They keep the user's own words.
  - Records 0009, 0017, 0022, 0023 and 0025 name agents by their role.
  - Record 0016's "What keeps its text" says what design Decision 6 asks.
  - `docs/architecture/` is unchanged.
- **The research and the flake's design** (group 5).
  - `design/research/` is gone from the tree. Records 0003 to 0014, 0016, 0023 and 0025, and the flake's design, cite its two transcripts as agentplot/boomdrive-archive's `design/research/<file>`.
  - `design/the-flakes-configuration/host.nix` is the neutral sketch of design Decision 7: the family's public repositories and `example`'s, streams `family` and `work`, `pool-a` of `pool-a-1` and `pool-a-2`, and crew's `home` and `work` on `host-a` with `example/crew-state`.
- **The archived changes** (group 6). The seven archived changes the tasks name use the neutral names. Each cites the private repositories by id.
- **The specs** (group 7). See Requirements.

The change's own files and its 29 commits' messages name none of the values they replace.

## Requirements

The deltas modify 28 requirements, with 53 scenarios:

| spec | requirements | scenarios |
|---|---|---|
| `crew-control-plane` | 17 | 33 |
| `work-into-flume` | 8 | 14 |
| `views` | 1 | 3 |
| `branch-commands` | 1 | 2 |
| `tool-records` | 1 | 1 |

Each requirement's text is unchanged, and only the names in its scenarios differ. Every neutral value a scenario names is in the tests that prove it under `crates/`, such as `home/runner/host-b`, `work/runner/host-a`, `boomdrive:home/12` and `example/home-blueprints`. All of those tests passed in the gate.

## For the landing

None of these is this change's to fix.

- **A conflict with `main`.** The branch merges into `bolt/boomdrive-is-ready-to-go-public` with no conflict, since that bolt still stands at the branch's base, 72e6882. Against `main` at 3f428cf, records 0002 and 0017 conflict. `main` amended both after the bolt was cut, 0017 to one workspace per drive.
- **What `main` has added since.** Its commits since 72e6882 add more of the private values this change replaces:
  - records 0002, 0010, 0026, 0027 and 0028;
  - the archived `2026-11-07-a-launch-waits-on-no-other-act` proposal;
  - the `work-into-flume` delta of the archived `2026-11-07-the-runner-reads-a-tidied-places-record`.

  When the bolt takes `main`, those need the same scrub before the tree opens (record 0014).
