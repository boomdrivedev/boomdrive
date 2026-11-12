#!/bin/sh
# Every commit since the newest release tag is a Conventional Commit
# (.versionrc), every commit where there is none, since git-cliff makes
# the next version and its notes from them (cliff.toml, record 0024,
# core.48): one it cannot read bumps nothing and is in no notes. Merge
# commits are not checked, nor one `git revert` wrote, whose subject opens
# `Revert "`, which git-cliff leaves out too.
#
# The newest release tag is the newest v<major>.<minor>.<patch> by
# version, wherever its commit is. A release's tag names its release
# commit, cut from a commit of main, off main's history, which
# `git describe` never reaches; <tag>..HEAD leaves out everything the
# release commit's parent reaches, so it is the commits main took since
# the one the release was cut from, the range git-cliff's --unreleased
# reads (release.sh). A tag made before the release commit, on a commit of
# main, is read as any tag.
set -eu

last=$(git tag --list 'v*' --sort=-v:refname |
  grep -E -x -m 1 'v[0-9]+\.[0-9]+\.[0-9]+' || true)
exec convco check --ignore-reverts --config .versionrc ${last:+"$last..HEAD"}
