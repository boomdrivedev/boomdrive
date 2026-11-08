# sluice 0.74.3's own answers of a preview

Each file here is what sluice 0.74.3 printed on standard output on a
scratch host, byte for byte, its trailing newline among it: nothing is
stripped, reformatted or rewritten. treefmt excludes the directory so that
no formatter changes a byte (core.7).

## A preview, its listing and its release

The files were read on 2026-11-08, UTC, on a scratch host: a macOS
process tree under `sandbox-exec`, its profile denying every network
call but to the loopback address and Unix sockets, since sluice binds a
port on the loopback address to find it free, with `HOME`, XDG
directories and a git configuration of its own under
`/private/tmp/scratch/host/`. Its sluice says `sluice 0.74.3`: the
flake's `sluice` package, `nix build .#sluice`, run by its path.

Its forge was a bare `example/kit.git` on disk, and each read was made
in a clone of it at `/private/tmp/scratch/host/repo`. Its `main` held
three commits:

- R0, whose `.config/sluice.toml` gates `main` by the command runner,
  with `[checkout] ports = ["web"]` and `[tests] release` a script that
  exits 0, sluice 0.74.3 reading a preview's release command there, and
  no `[preview]` table;
- R1, which adds `[preview]`, its `command` a script that prints a line
  and exits 0, and `addresses = { web = "http://{name.web}.localhost:{port.web}" }`;
- R2, whose script prints a line and exits 1.

| file | the command | what stood |
|---|---|---|
| `preview-no-table.json` | `sluice preview main <R0> --json` | exit 2: R0's policy holds no `[preview]` table, as sluice's `error` answer, nothing on standard error |
| `preview-starting.json` | `sluice preview main <R1> --json` | exit 0: R1's preview made, `starting`, its addresses filled in |
| `previews.json` | `sluice previews --json` | exit 0: R1's preview `up`, once its command had exited 0, and R2's `failed`, asked for with `--wait` beforehand, its `output` holding what its command printed |
| `release.json` | `sluice release preview/<R1> --json` | exit 0: R1's preview released |
| `release-nothing.json` | `sluice release preview/<R1> --json` again | exit 0: nothing to release |

A preview's `checkout` names where sluice made its checkout on the
scratch host, and a failed preview's `output` is its command's log. The
runner carries neither (core.11).

sluice's wait at the host's bound is not captured: a scratch host would
have to hold its bound full. The stub `sluice` of `tests/fakes` stands in
for it, as it says sluice does: interrupted while it waits, it says so on
standard error, prints no document and exits 130.
