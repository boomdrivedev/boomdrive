#!/bin/sh
# nix:mac-sh: every script of the modules and the packages, which a Mac
# runs by its /bin/sh, is one macOS's /bin/sh reads (record 0020 as
# amended on 2026-11-15): nix/modules/activate.sh, which the nix-darwin
# module's activation runs as `/bin/sh activate.sh`, and
# nix/packages/launcher.sh, the launcher's script, run by its `#!/bin/sh`.
# The shell is devenv.nix's bash 3.2.57, the version macOS's /bin/sh is,
# given by BOOMDRIVE_MAC_SH and the same on every host that runs the gate
# (core.8). It reads each script in POSIX mode, as /bin/sh does, with
# `-n`, running none. Each failure is one line, nix: mac-sh: <script>:
# <what>.
set -eu

if [ -z "${BOOMDRIVE_MAC_SH:-}" ]; then
  echo "nix: mac-sh: BOOMDRIVE_MAC_SH is not set: the devenv shell sets it" >&2
  exit 1
fi

scripts=0
failed=0
for script in nix/modules/*.sh nix/packages/*.sh; do
  [ -f "$script" ] || continue
  scripts=$((scripts + 1))
  if ! words=$("$BOOMDRIVE_MAC_SH" --posix -n "$script" 2>&1); then
    words=$(printf '%s\n' "$words" | tr '\n' ' ')
    echo "nix: mac-sh: $script: macOS's /bin/sh cannot read it: ${words% }"
    failed=1
  fi
done
[ "$scripts" -gt 0 ] || {
  echo "nix: mac-sh: no script under nix/modules or nix/packages"
  exit 1
}
[ "$failed" = 0 ] || exit 1
echo "nix: mac-sh: macOS's /bin/sh reads the $scripts scripts a Mac runs by it"
