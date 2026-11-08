# Flume 0.4.1's own answers

Each file here is what Flume 0.4.1 printed on a scratch host, its
trailing newline among it, every byte as Flume printed it but the scratch
directory's prefix (the change boomdrives-tree-holds-nothing-private,
design Decision 2): nothing else is stripped, reformatted or rewritten.
treefmt excludes the directory so that no formatter changes a byte
(core.7). Each was read from a Flume that says `flume 0.4.1`.

## The scratch host

Every file but `status-live-agent.json` was read on 2026-11-01 from a
scratch host: Flume 0.4.1 run under an environment holding nothing but a
temporary directory's `HOME`, `XDG_CONFIG_HOME`, `XDG_STATE_HOME` and
`XDG_DATA_HOME`, a `PATH`, and a git configuration of its own, so that no
place, configuration or session of the user's was read or touched.

The temporary directory's path, up to and including its `scratchpad/host/`,
is written in each answer as `/private/tmp/scratch/host/`, in every form
Flume gave it: in a path, in a command Flume printed, and in a state
folder's name, where Flume joins the path's parts with `%`, as
`private%tmp%scratch%host%`. No other byte is changed: no field, time,
digest or newline. A digest Flume made of a path, such as the
`2867c6a5947d` in each place's path, keeps the value Flume printed.

- **Its Flume configuration** declared the printed command as its
  terminal, a pool of one account, `stand-in-1`, and two bounded streams
  beside `default`: `wide`, of three agents at priority 5, and `work`, of
  one. `[host] agents = 4` was added before `status-bounded.json` was
  read, and was absent before.
- **Its repository** was bound to `git-worktree`, `openspec` and `sluice`
  (`.config/flume.toml`), gated by `sh check.sh` under sluice 0.61.1
  (`.config/sluice.toml`), and landed on a bare repository in the same
  temporary directory as `origin`.
- **Each agent** was Flume's own stand-in for Claude Code,
  `flume-stand-in`, built from the tag `v0.4.1` (`78cf178`) of
  agentplot/flume, `crates/flume/tests/profiles/stand-in/claude.rs`. It
  was run as a person runs the printed command, first on `PATH` as
  `claude`, acting a script of tool calls in place of a model, as Flume's
  walkthroughs run it. It reaches no model, account or network.
- **The role's run** was launched with `--kinds` naming a declaration of
  `crew-planner` written as crew's `plugin/flume/kinds.toml` writes it:
  one stage, `turn`, landing nothing, its deliverable a report in the
  place's state folder.

The places, each launched with `--mark flywheel:scratch/<n>`, and what
was read of each:

| file | the command | what stood |
|---|---|---|
| `launch.json` | `flume --json launch --repository <repo> --kind fix --name parser --stream work --priority 4 --intent scratch/parser --recipient scratch/planner --brief-file <file> --mark flywheel:scratch/1` | exit 0: `fix--parser` launched, its code stage starting |
| `launch-refused.json` | `flume --json launch --repository <repo> --kind fix --name nowhere --stream nosuch --mark flywheel:scratch/9` | exit 1: the stream `nosuch` is not declared |
| `status.json` | `flume --json status` | every place: `change--parse-words` in code, `crew-planner--scratch` delivered, `fix--tabs` waiting on an answer, `fix--lexer` starting in `work`, `fix--indent` queued in `work`, and `fix--parser` and `fix--gate` tidied with an end pending; the host unbounded, `wide` with room 1, `default` unbounded, and `work` with no room and a queue |
| `status-bounded.json` | `flume --json status` | the same places, read again after `[host] agents = 4`: the host's `bound` 4 and `room` 1. Only the host part differs from `status.json` |
| `status-construct.json` | `flume --json status <change--parse-words>` | a change at construct, its stage begun and its agent working |
| `status-code.json` | `flume --json status <change--parse-words>` | the same change in code, construct delivered, one of its two tasks ticked and its agent working |
| `status-answer.json` | `flume --json status <fix--tabs>` | a fix whose agent ran `flume needs "Which width is a tab: 4 or 8?"`: code ended short, waiting on its caller with `why` `answer` |
| `status-on-hold.json` | `flume --json status <fix--held>` | a fix launched with `--hold "code=the user reads the brief first"`: waiting on its caller with `why` `on_hold` |
| `status-delivered.json` | `flume --json status <crew-planner--scratch>` | the role's run, its one stage delivered, waiting on its caller with `act` `done` and `why` `delivered` |
| `status-submitted.json` | `flume --json status <fix--slow-gate>` | a fix whose code delivered and whose submission is sluice's entry `3`, `under_test` while its gate waited |
| `status-landed-kept.json` | `flume --json status <fix--parser>` | `fix--parser` landed and over, kept from removal by a teardown step that failed: the scratch host's first environment named no user, so the login shell that ran `sluice release` found no sluice. The next settle, with the user named, tidied it |
| `status-tidied.json` | `flume --json status <fix--gate>` | a fix landed and tidied, its record kept as `<location>@<at>` for the end pending for `scratch/planner` |
| `status-gone.json` | `flume --json status <fix--slow-gate>` | exit 1: the same fix as `status-submitted.json`, landed and tidied with no end kept, so no place stands |
| `priority.json` | `flume --json priority <fix--indent> 7` | exit 0, on the queued place |
| `retry.json` | `flume --json retry <fix--tabs> --stage code --words Four.` | exit 0, on the place waiting on its answer |
| `done.json` | `flume --json done <crew-planner--scratch>` | exit 0, on the delivered role's run, which it tidied |
| `drop.json` | `flume --json drop <fix--indent>` | exit 0, on the queued place, which it tidied |
| `retry-refused.json` | `flume --json retry <fix--lexer>` | exit 1: its agent, printed and never run, is not quiet |
| `usage.stderr` | `flume --json 'done <place>'`, the act and its place passed as one word | exit 2, nothing on standard output; this file is its standard error |

## deneb's own Flume

`status-live-agent.json` is `flume --json status
/Users/chuck/Code/github_agentplot/flywheel-runner/places/change--the-runner-carries-crews-work-into-flume`,
read on deneb's own Flume under the user's configuration: the place this
change was coded in, while its code stage's agent worked in herdr. It is
the one answer here whose agent is live in a terminal that shows it, so
its `clues.shown_at.agent` names a pane, `herdr:<pane>@<socket>`. Its host
part is deneb's. Flume gives every value of an agent's environment as
`<set>`.
