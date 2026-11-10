# Flume 0.21.0's own answers

Each file here is what Flume 0.21.0 printed on a scratch host, its
trailing newline among it: nothing is stripped, reformatted or rewritten
but the scratch directory's prefix, as the 0.4.1 fixtures' README says
of its own. treefmt excludes the directory so that no formatter changes
a byte (core.7). Each was read on 2026-11-09 from the set's Flume, the
flake's package of its lock at the tag `v0.21.0`, which says
`flume 0.21.0`: the program the dev shell names `BOOMDRIVE_SET_FLUME`.

## The scratch host

Each was read from a scratch host: Flume 0.21.0 run with `env -i` under
an environment holding nothing but a temporary directory's `HOME`,
`XDG_CONFIG_HOME`, `XDG_STATE_HOME`, `XDG_DATA_HOME`, `XDG_CACHE_HOME`
and `TMPDIR`, and a `PATH` of the set's Flume and sluice, then
`/usr/bin:/bin`, so that no place, configuration, watch or session of a
person's was read or touched. Each XDG folder was a short one, `d` for
data and `s` for state, since Flume names a place's state file by the
place's whole path joined by `%`, and a deeper folder took that name
past the 255 bytes a name may take, so no tidied record could be kept.

The temporary directory's path, up to and including its folder `x/`, is
written in each answer as `/private/tmp/scratch/host/`, wherever Flume
gave it. No other byte is changed: no field, time, digest, key or
newline. A digest Flume made of a path, such as the `b85c8b7b7ac5` in
each place's path, keeps the value Flume printed.

- **Its Flume configuration** held no `config.toml`, and one source,
  `scratch`, whose roles file declared one role, `scratch/planner`,
  carried to another host, so that a launch could name it and no run of
  it was woken on the scratch host.
- **Its repository** was a scratch git repository cloned from a bare
  repository in the same temporary directory as `origin`, holding one
  empty commit on `main`.
- **Each place** was launched with `--kinds` naming a declaration of its
  own: the kind `note`, which does not land, of one stage, `code`. Each
  launch held `code` with `--hold "code=held for the fixture"`, so no
  agent was started and no stage ended.

The places, each launched as
`flume --json launch --kinds ../kinds.toml --kind note --name <name> --hold "code=held for the fixture" --recipient scratch/planner --recipient boomdrive --mark boomdrive:scratch/<name>`,
then given up by `flume --json drop note--drop` and declared complete by
`flume --json done note--done`, each act tidying its place at once and
keeping its record for the end pending; then `flume --json settle --every
60s`, which settled nothing and tidied nothing more. What was read of
each:

| file | the command | what stood |
|---|---|---|
| `status-dropped-held.json` | `flume --json status <location>@<at>` of `note--drop`'s record | exit 0: the place tidied, `over` `dropped`, its one end the drop's, `{"of":"over","over":"dropped",…}`, keyed `over/dropped`, with its delivery's key and expiry, pending for and owed to `boomdrive` alone |
| `status-done-held.json` | `flume --json status <location>@<at>` of `note--done`'s record | exit 0: the same of a place done, its one end `over` `done`, keyed `over/done` |

Each place names `scratch/planner` and `boomdrive` as its recipients. A
drop's or a done's end stands pending only for the place's recipients
that are no role's address, so each end is owed to `boomdrive` alone,
and `scratch/planner`, a role's address, is owed none.
