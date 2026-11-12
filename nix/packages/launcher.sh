#!/bin/sh
# The Boomdrive launcher (record 0023 as amended on 2026-11-11; design
# Decision 2 of the change each-drive-opens-from-the-launcher): run at a
# person's asking, by the menu of Boomdrive.app on macOS or the desktop
# entry boomdrive.desktop and its actions on Linux, it runs Flume once.
# Given a session, it runs `flume open -- <session>`, so Flume shows that
# session; given none, `flume open`, so Flume shows the default stream's
# own session; each in the view the host's configuration names (Flume's
# statement 186). Given more than one, it runs nothing and exits 2. It
# starts, keeps and names no multiplexer of its own (core.13, core.29).
#
# An app macOS opens is given /usr/bin:/bin:/usr/sbin:/sbin, so the running
# user's profiles and the system's go ahead of the PATH it is given, and it
# runs the host's one Flume, as the runner's service does. Flume reads the
# user's login environment for what it runs, so nothing else is passed.
#
# Where Flume exits 0 it shows nothing. Where Flume refuses, it shows
# Flume's words as Flume gave them, its standard output and error, and
# exits 1: in an alert titled Boomdrive on macOS, the words passed to
# osascript as an argument and never spliced into its script, and on
# standard error elsewhere, which the desktop session's log keeps.
set -u

case $# in
0) set -- open ;;
1) set -- open -- "$1" ;;
*)
  echo 'boomdrive-launcher: one session at most' >&2
  exit 2
  ;;
esac

user=$(id -un)
PATH=/etc/profiles/per-user/$user/bin:${HOME-}/.nix-profile/bin:${HOME-}/.local/state/nix/profile/bin:/run/wrappers/bin:/run/current-system/sw/bin:/nix/var/nix/profiles/default/bin:$PATH
export PATH

if words=$(flume "$@" 2>&1); then
  exit 0
fi
if [ "$(uname -s)" = Darwin ]; then
  osascript -e 'on run argv' -e 'display alert "Boomdrive" message (item 1 of argv)' -e 'end run' "$words" >/dev/null
else
  printf '%s\n' "$words" >&2
fi
exit 1
