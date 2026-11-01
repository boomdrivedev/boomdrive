# sluice 0.61.1's own answers

Each file here is what sluice 0.61.1 printed on standard output, byte for
byte, its trailing newline among it: nothing is stripped, reformatted or
rewritten (design Decision 21). treefmt excludes the directory so that no
formatter changes a byte (core.7). Each was read on deneb on 2026-11-01,
from `/etc/profiles/per-user/chuck/bin/sluice`, which says `sluice
0.61.1`, as `sluice status <n> --json` in a repository's bare container:
a read of a submission sluice had already recorded, which changes
nothing.

| file | where | what stood |
|---|---|---|
| `status-landed.json` | this repository, `sluice status 1 --json` | exit 0: the submission of `the-runner-keeps-its-repositories`, landed on `main`, with its `landed_as` |
| `status-failed.json` | agentplot/flume, `sluice status 45 --json` | exit 0: a fix failed, its one `failed` row holding its task's `log` and a `flaky` reading of the run it contradicts |
| `status-withdrawn.json` | agentplot/sluice, `sluice status 291 --json` | exit 0: a submission withdrawn by its resubmission, its one `failed` row holding its task's `log` and a `flaky` reading whose `other` names where that run's output is kept, a `log` too |
| `status-missing.json` | this repository, `sluice status 99 --json` | exit 1: no submission 99, as sluice's `error` answer, nothing on standard error |

The paths in them are deneb's: a `log` names where a task's output is
kept on that host, and the runner carries none of them (core.11).
