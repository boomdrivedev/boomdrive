#!/bin/sh
# The workflow release's asks of git-cliff, under cliff.toml, and its
# release commit, in the repository it runs in (record 0024, core.48), so
# root:releases tests what the workflow runs:
#
#   release.sh next          the tag of the next version, from the commits
#                            since the newest v* tag, or nothing where that
#                            version is tagged already
#   release.sh notes <tag>   the notes of the same commits, released as <tag>
#   release.sh commit <tag>  the release commit of <tag>, cut from HEAD,
#                            tagged <tag>
#
# next and notes ask with --unreleased. A release's tag names its release
# commit, cut from a commit of main, off main's history, so git-cliff's
# walk of main's history never reaches it, and would read every commit
# since the first as unreleased; --unreleased starts the range at the
# newest tag, wherever its commit is: the commits main took since the one
# the release was cut from. Ask for the notes before the release commit
# is tagged, since the range then starts at the new tag.
#
# commit writes the version into HEAD's committed Cargo.toml and
# Cargo.lock with set-version.sh, beside this script, and commits HEAD's
# tree with those two files as `chore(release): <tag>`, HEAD its one
# parent, by git's own identity, and tags it <tag>. It reads no file of
# the working tree, so nothing uncommitted reaches a release, and leaves
# the working tree, the index and every branch as they were. It refuses a
# tag that exists (exit 1) and one that is not v<major>.<minor>.<patch>
# (exit 2), making nothing. next and notes write nothing. git-cliff's own
# lines go to standard error.
set -eu

usage() {
  echo "usage: release.sh next | release.sh notes <tag> | release.sh commit <tag>" >&2
  exit 2
}

# The release commit of the tag $1, cut from HEAD.
release_commit() {
  tag=$1
  printf '%s\n' "$tag" | grep -Eqx 'v[0-9]+\.[0-9]+\.[0-9]+' || {
    echo "release: $tag is not a tag v<major>.<minor>.<patch>" >&2
    exit 2
  }
  if git rev-parse -q --verify "refs/tags/$tag" >/dev/null; then
    echo "release: $tag is a tag already" >&2
    exit 1
  fi
  head=$(git rev-parse --verify "HEAD^{commit}")
  work=$(mktemp -d)
  trap 'rm -rf "$work"' EXIT
  for file in Cargo.toml Cargo.lock; do
    git cat-file blob "$head:$file" >"$work/$file"
  done
  sh "$(dirname "$0")/set-version.sh" "${tag#v}" "$work"
  GIT_INDEX_FILE=$work/index git read-tree "$head"
  for file in Cargo.toml Cargo.lock; do
    mode=$(git ls-tree "$head" -- "$file" | awk '{ print $1 }')
    blob=$(git hash-object -w --no-filters "$work/$file")
    GIT_INDEX_FILE=$work/index git update-index --cacheinfo "$mode,$blob,$file"
  done
  tree=$(GIT_INDEX_FILE=$work/index git write-tree)
  commit=$(git commit-tree "$tree" -p "$head" -m "chore(release): $tag")
  git tag "$tag" "$commit"
  echo "release: $tag names $commit, cut from $head" >&2
}

case ${1:-} in
next)
  [ $# -eq 1 ] || usage
  tag=$(git-cliff --unreleased --bumped-version)
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
commit)
  [ $# -eq 2 ] || usage
  release_commit "$2"
  ;;
*)
  usage
  ;;
esac
