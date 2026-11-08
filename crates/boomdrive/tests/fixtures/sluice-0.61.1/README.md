# sluice 0.61.1's own answers

Each file here is what sluice 0.61.1 printed on standard output on a
scratch host, byte for byte, its trailing newline among it: nothing is
stripped, reformatted or rewritten. treefmt excludes the directory so that
no formatter changes a byte (core.7).

## A submission's entry

`status-*.json` were read on 2026-11-08, UTC, on a scratch host named
`host-a` (the change boomdrives-tree-holds-nothing-private, design
Decision 3): a Linux container with no network whose hostname, `host-a`,
is what sluice names as the host that ran a check, with `HOME`, XDG
directories and a git configuration of its own under
`/private/tmp/scratch/host/`. Its sluice says `sluice 0.61.1`: built
outside this tree from the tag `v0.61.1` (`074b0f5`) of agentplot/sluice
with `SLUICE_VERSION=0.61.1`, as its release is, and run by its path.

Its forge was a bare `example/crew.git` on disk, whose `main` held
`.config/sluice.toml`: `main` gated by the command runner, `sh check.sh`,
and landed on `origin`. The check failed while the file
`/private/tmp/scratch/host/outcome`, outside the tree, said `fail`, so it
passed and failed on the same inputs, as a flaky test does. Each
submission came from a worktree of a clone of the forge, and each read
was `sluice status <n> --json` in that clone: a read of a submission
sluice had already recorded, which changes nothing.

| file | the command | what stood |
|---|---|---|
| `status-landed.json` | `sluice status 1 --json` | exit 0: `change/parse-words` landed on `main`, with its `landed_as` |
| `status-failed.json` | `sluice status 2 --json` | exit 0: `fix/tabs` failed, its one `failed` row holding its task's `log` and a `flaky` reading: the queue's run of its combination passed, where `sluice exec main` had failed on the same key in the worktree the reading's `other` names |
| `status-withdrawn.json` | `sluice status 4 --json` | exit 0: `fix/indent` submitted a second time, the queue's run of its combination passing where the first submission's had failed; its one `failed` row holds its task's `log` and a `flaky` reading whose `other` names where the first run's output is kept, a `log` too; then withdrawn by its resubmission |
| `status-missing.json` | `sluice status 99 --json` | exit 1: no submission 99, as sluice's `error` answer, nothing on standard error |

sluice's command runner keys a check by the commit it ran on, and tests
a submission on a combination it forms as a commit of its own, its author
and committer `sluice` at time 0. So the worktree of `status-failed.json`'s
`other` was checked out at that combination, made beforehand with `git
commit-tree` from the same tree, parents and words, for `sluice exec` to
run the check on the key the queue's run then read.

A `log` names where a task's output is kept on the scratch host, and the
runner carries none of them (core.11).

## A bolt's branch

`sluice branch create` changes a repository, so its answers were read on
a scratch host too: on 2026-11-01, from sluice's release 0.61.1, under
`env -i` with a temporary `HOME`, XDG directories and git configuration.
Its forge was a bare `example/crew.git` on disk, which the git
configuration put in place of GitHub's https address, its `main` holding
`.config/sluice.toml`. Its clone was laid out as the runner lays one out,
a bare `crew.git` in its container beside a `.git` file naming it, and
each command ran in that container.

| file | what stood | what sluice did |
|---|---|---|
| `branch-made.json` | `[branches.main]` and `[branches."bolt/*"]`, each with `remote = "origin"`; no `bolt/x` | `sluice branch create bolt/x --from main --json`, exit 0: made `bolt/x` on the forge at `main`'s commit, nothing on standard error |
| `branch-there.json` | the same, once `branch-made.json` was read | the same command, exit 2: `already there`, as sluice's `error` answer, nothing on standard error |
| `branch-ungated.json` | `[branches.main]` alone; no `bolt/x` | the same command, exit 2: no `[branches]` table names `bolt/x`, as sluice's `error` answer, nothing on standard error |

The commits they name are the scratch forge's.
