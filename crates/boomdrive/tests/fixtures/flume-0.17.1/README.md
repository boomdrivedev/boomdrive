# Flume 0.17.1's own answers

Each file here is what Flume 0.17.1 printed, byte for byte, its trailing
newline among it: nothing is stripped, reformatted or rewritten. treefmt
excludes the directory so that no formatter changes a byte (core.7). Each
was read on 2026-11-07 from the set's Flume, the flake's package of its
lock at the tag `v0.17.1`, which says `flume 0.17.1`: the program the dev
shell names `BOOMDRIVE_SET_FLUME`.

## The scratch host

Each was read from a scratch host: Flume 0.17.1 run with `env -i` under an
environment holding nothing but a temporary directory's `HOME`,
`XDG_CONFIG_HOME`, `XDG_STATE_HOME` and `XDG_DATA_HOME`, and a `PATH` of
`/usr/bin:/bin`, in an empty folder of the same temporary directory, so
that no place, configuration, watch or session of a person's was read or
touched. The scratch host held no Flume configuration, no repository and
no place.

| file | the command | what stood |
|---|---|---|
| `settled.json` | `flume --json settle --every 60s` | exit 0, nothing on standard error: no place settled and nothing tidied. Flume wrote the host's watch, and its `--json status` read next gave the host's `watch` as `{"bound":{"seconds":60}}` |
