#!/bin/sh
# Every commit since the newest release tag in the history is a
# Conventional Commit (.versionrc), every commit where there is none,
# since git-cliff makes the next version and its notes from them
# (cliff.toml, record 0024, core.48): one it cannot read bumps nothing and
# is in no notes. Merge commits are not checked, nor one `git revert`
# wrote, whose subject opens `Revert "`, which git-cliff leaves out too.
set -eu

last=$(git describe --tags --abbrev=0 --match 'v[0-9]*' 2>/dev/null || true)
exec convco check --ignore-reverts --config .versionrc ${last:+"$last..HEAD"}
