#!/bin/sh
# The workflow release's two asks of git-cliff, under cliff.toml, in the
# repository it runs in (record 0023, core.48), so root:releases tests what
# the workflow runs:
#
#   release.sh next          the tag of the next version, from the commits
#                            since the newest v* tag, or nothing where that
#                            version is tagged already
#   release.sh notes <tag>   the notes of the same commits, released as <tag>
#
# It writes nothing. git-cliff's own lines go to standard error.
set -eu

usage() {
  echo "usage: release.sh next | release.sh notes <tag>" >&2
  exit 2
}

case ${1:-} in
next)
  [ $# -eq 1 ] || usage
  tag=$(git-cliff --bumped-version)
  if git rev-parse -q --verify "refs/tags/$tag" >/dev/null; then
    echo "release: $tag is released already" >&2
  else
    printf '%s\n' "$tag"
  fi
  ;;
notes)
  [ $# -eq 2 ] || usage
  git-cliff --unreleased --tag "$2" --strip all
  ;;
*)
  usage
  ;;
esac
