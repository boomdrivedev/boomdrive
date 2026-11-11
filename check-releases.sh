#!/bin/sh
# The releases spec (record 0024, core.48), proven in scratch repositories
# made in a temporary directory: release.sh's next tag and its notes under
# the tree's cliff.toml, release-preview beside them, check-commits.sh
# under the tree's .versionrc, set-version.sh on scratch copies of the
# tree's manifests, and the workflow release read line by line.
# The scratch repositories' git reads no configuration of the host's, and
# their names are neutral (core.41).
set -eu

tree=$(pwd)
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT

GIT_CONFIG_GLOBAL=/dev/null
GIT_CONFIG_NOSYSTEM=1
GIT_AUTHOR_NAME=Ada
GIT_AUTHOR_EMAIL=ada@example.com
GIT_COMMITTER_NAME=Ada
GIT_COMMITTER_EMAIL=ada@example.com
export GIT_CONFIG_GLOBAL GIT_CONFIG_NOSYSTEM GIT_AUTHOR_NAME GIT_AUTHOR_EMAIL \
  GIT_COMMITTER_NAME GIT_COMMITTER_EMAIL

status=0
fail() {
  echo "check-releases: $*" >&2
  status=1
}
pass() {
  echo "check-releases: $*"
}

# A scratch repository `$1` on main, holding the tree's cliff.toml,
# .versionrc and scripts, untracked, and no commit yet. It is `$dir`.
repo() {
  dir="$scratch/$1"
  git init -q -b main "$dir"
  cp "$tree/cliff.toml" "$tree/.versionrc" "$tree/release.sh" \
    "$tree/check-commits.sh" "$dir/"
  files=0
}

# A commit of a file of its own, its message the arguments, a paragraph
# each.
commit() {
  files=$((files + 1))
  echo "$1" >"$dir/file-$files"
  git -C "$dir" add "file-$files"
  message=""
  for paragraph in "$@"; do
    message="$message$paragraph

"
  done
  git -C "$dir" commit -q -m "$message"
}

# A commit of a file of its own under site/, and of one beside the root's
# files too where `$1` is `both`, its message `$2`.
site_commit() {
  files=$((files + 1))
  mkdir -p "$dir/site"
  echo "$2" >"$dir/site/page-$files"
  git -C "$dir" add "site/page-$files"
  if [ "$1" = both ]; then
    echo "$2" >"$dir/file-$files"
    git -C "$dir" add "file-$files"
  fi
  git -C "$dir" commit -q -m "$2"
}

# What `release.sh next` prints in `$dir`; git-cliff's own lines go to
# `$dir.log`.
next() {
  (cd "$dir" && sh release.sh next 2>>"$dir.log")
}

# The tag `release.sh next` prints after the commits `$@` follow v0.1.0.
after_v010() {
  repo "$1"
  shift
  commit "feat: a start"
  git -C "$dir" tag v0.1.0
  for message in "$@"; do
    commit "$message"
  done
  next
}

# Scenario: The first release.
repo first
commit "feat: a feature"
commit "fix: a fix"
got=$(next)
if [ "$got" = v0.1.0 ]; then
  pass "the first release is v0.1.0"
else
  fail "the first release is ${got:-nothing}, not v0.1.0"
fi

# Scenarios: A feature, A fix, A breaking change before 1.0.0.
for case in "feature|feat: a feature|v0.2.0" "fix|fix: a fix|v0.1.1" \
  "breaking|fix!: a breaking fix|v0.2.0"; do
  name=${case%%|*}
  rest=${case#*|}
  message=${rest%|*}
  want=${rest##*|}
  got=$(after_v010 "$name" "$message")
  if [ "$got" = "$want" ]; then
    pass "'$message' after v0.1.0 releases $want"
  else
    fail "'$message' after v0.1.0 releases ${got:-nothing}, not $want"
  fi
done

# Scenario: Nothing to release.
got=$(after_v010 nothing "chore: a chore" "test: a test")
if [ -z "$got" ] && grep -q 'v0.1.0 is released already' "$scratch/nothing.log"; then
  pass "a chore and a test after v0.1.0 release nothing"
else
  fail "a chore and a test after v0.1.0 release ${got:-nothing}, or say nothing of v0.1.0"
fi

# Scenario: A change to the site alone.
repo site
commit "feat: a start"
git -C "$dir" tag v0.1.0
site_commit alone "feat(site): a page"
got=$(next)
if [ -z "$got" ]; then
  pass "a feature that changes only site/ after v0.1.0 releases nothing"
else
  fail "a feature that changes only site/ after v0.1.0 releases $got"
fi

# Scenario: A change to the site and the runner.
repo both
commit "feat: a start"
git -C "$dir" tag v0.1.0
site_commit both "fix: both"
got=$(next)
if [ "$got" = v0.1.1 ]; then
  pass "a fix that changes site/ and a file outside it after v0.1.0 releases v0.1.1"
else
  fail "a fix that changes site/ and a file outside it after v0.1.0 releases ${got:-nothing}, not v0.1.1"
fi

# Scenario: A release's notes.
repo notes
commit "feat: a start"
git -C "$dir" tag v0.1.0
commit "feat(boomdrive): a feature" "Co-Authored-By: Someone <someone@example.com>"
commit "fix(nix): a fix"
commit "docs(adr, architecture): a record"
commit "chore: a chore"
notes=$(cd "$dir" && sh release.sh notes v0.2.0 2>>"$dir.log")
# The line of each, in the notes.
at() {
  printf '%s\n' "$notes" | grep -n -F -x -e "$1" | head -n 1 | cut -d : -f 1
}
features=$(at "### Features")
feature=$(printf '%s\n' "$notes" | grep -n '^- \*\*boomdrive\*\*: A feature (\[' | head -n 1 | cut -d : -f 1)
fixes=$(at "### Fixes")
fix=$(printf '%s\n' "$notes" | grep -n '^- \*\*nix\*\*: A fix (\[' | head -n 1 | cut -d : -f 1)
docs=$(at "### Documentation")
record=$(printf '%s\n' "$notes" | grep -n '^- \*\*adr, architecture\*\*: A record (\[' | head -n 1 | cut -d : -f 1)
case $(printf '%s\n' "$notes" | head -n 1) in
"## v0.2.0 ("*) ;;
*) fail "the notes are not headed v0.2.0: $(printf '%s\n' "$notes" | head -n 1)" ;;
esac
if [ -n "$features" ] && [ -n "$feature" ] && [ -n "$fixes" ] && [ -n "$fix" ] &&
  [ -n "$docs" ] && [ -n "$record" ] &&
  [ "$features" -lt "$feature" ] && [ "$feature" -lt "$fixes" ] &&
  [ "$fixes" -lt "$fix" ] && [ "$fix" -lt "$docs" ] && [ "$docs" -lt "$record" ]; then
  pass "the notes hold the feature, the fix and the record, each under its group, in order"
else
  fail "the notes do not hold the feature, the fix and the record, each under its group, in order:
$notes"
fi
if printf '%s\n' "$notes" | grep -q -i -e 'a chore' -e 'co-authored-by'; then
  fail "the notes hold the chore or the trailer:
$notes"
else
  pass "the notes hold nothing of the chore or the trailer"
fi

# release-preview, devenv.nix's, shows the notes the release would make,
# under the same heading but for its date.
if command -v release-preview >/dev/null; then
  preview=$(cd "$dir" && release-preview 2>>"$dir.log")
  if [ "$(printf '%s\n' "$preview" | head -n 1 | cut -d ' ' -f 1-2)" = "## v0.2.0" ] &&
    [ "$(printf '%s\n' "$preview" | sed 1d)" = "$(printf '%s\n' "$notes" | sed 1d)" ]; then
    pass "release-preview shows the notes release.sh makes for v0.2.0"
  else
    fail "release-preview shows other notes than release.sh makes for v0.2.0:
$preview"
  fi
else
  fail "release-preview is not on PATH: run this in the devenv shell"
fi

# Scenario: The site's commit in no notes.
repo sitenotes
commit "feat: a start"
git -C "$dir" tag v0.1.0
commit "fix(nix): a fix"
site_commit alone "feat(site): a page"
site_notes=$(cd "$dir" && sh release.sh notes v0.1.1 2>>"$dir.log")
fixes=$(printf '%s\n' "$site_notes" | grep -n -F -x -e "### Fixes" | head -n 1 | cut -d : -f 1)
fix=$(printf '%s\n' "$site_notes" | grep -n '^- \*\*nix\*\*: A fix (\[' | head -n 1 | cut -d : -f 1)
if [ -n "$fixes" ] && [ -n "$fix" ] && [ "$fixes" -lt "$fix" ] &&
  ! printf '%s\n' "$site_notes" | grep -q -i 'a page'; then
  pass "the notes hold the fix under Fixes and nothing of the site's page"
else
  fail "the notes do not hold the fix under Fixes, or hold the site's page:
$site_notes"
fi

# Scenario: A commit that is not conventional.
repo unconventional
commit "feat: a start"
git -C "$dir" tag v0.1.0
commit "Add a feature"
if said=$(cd "$dir" && sh check-commits.sh 2>&1); then
  fail "check-commits.sh passes 'Add a feature' after v0.1.0"
else
  case $said in
  *"Add a feature"*) pass "check-commits.sh fails 'Add a feature' after v0.1.0, naming it" ;;
  *) fail "check-commits.sh fails 'Add a feature' after v0.1.0 without naming it: $said" ;;
  esac
fi

# Scenario: Merges, reverts and several scopes.
repo allowed
commit "feat: a start"
git -C "$dir" tag v0.1.0
git -C "$dir" switch -q -c side
commit "feat: a side's feature"
git -C "$dir" switch -q main
commit "fix: a fix"
git -C "$dir" merge -q --no-ff --no-edit side
git -C "$dir" revert --no-edit HEAD~1 >/dev/null
commit "docs(adr, architecture): a record"
if said=$(cd "$dir" && sh check-commits.sh 2>&1); then
  pass "check-commits.sh passes a merge, a revert and several scopes"
else
  fail "check-commits.sh fails a merge, a revert or several scopes: $said"
fi

# Scenario: Before the newest release.
repo before
commit "Add a feature"
commit "feat: a start"
git -C "$dir" tag v0.1.0
commit "fix: a fix"
if said=$(cd "$dir" && sh check-commits.sh 2>&1); then
  pass "check-commits.sh passes 'Add a feature' before v0.1.0"
else
  fail "check-commits.sh fails 'Add a feature' before v0.1.0: $said"
fi

# A scratch copy `$1` of the tree's Cargo.toml, Cargo.lock and each
# crate's Cargo.toml, laid out as the tree lays them. It is `$copy`.
manifests() {
  copy="$scratch/$1"
  mkdir -p "$copy"
  cp "$tree/Cargo.toml" "$tree/Cargo.lock" "$copy/"
  for manifest in "$tree"/crates/*/Cargo.toml; do
    crate=${manifest%/Cargo.toml}
    crate=${crate##*/}
    mkdir -p "$copy/crates/$crate"
    cp "$manifest" "$copy/crates/$crate/"
  done
}
# The version Cargo.toml at `$1` gives under [workspace.package].
workspace_version() {
  awk '
    /^\[/ { section = $0 }
    section == "[workspace.package]" && /^version = "/ {
      gsub(/^version = "|"$/, "")
      print
      exit
    }
  ' "$1/Cargo.toml"
}
# Each entry with no source in Cargo.lock at `$1`, a line of its name and
# its version.
lock_crates() {
  awk '
    function flush() {
      if (package && !sourced) print name " " version
      sourced = 0
      name = ""
      version = ""
    }
    /^\[\[package\]\]$/ { flush(); package = 1; next }
    /^name = "/ { name = $0; gsub(/^name = "|"$/, "", name) }
    /^version = "/ { version = $0; gsub(/^version = "|"$/, "", version) }
    /^source = / { sourced = 1 }
    END { flush() }
  ' "$1/Cargo.lock"
}

# Scenario: A release's version and nothing else.
manifests written
if sh "$tree/set-version.sh" 98.76.54 "$copy" 2>>"$copy.log"; then
  others=$(lock_crates "$copy" | awk '$2 != "98.76.54"')
  if [ "$(workspace_version "$copy")" = 98.76.54 ] && [ -n "$(lock_crates "$copy")" ] &&
    [ -z "$others" ]; then
    pass "set-version.sh 98.76.54 writes Cargo.toml's version and each crate's in Cargo.lock"
  else
    fail "set-version.sh 98.76.54 leaves Cargo.toml giving $(workspace_version "$copy"), or a crate in Cargo.lock another: $others"
  fi
  for file in Cargo.toml Cargo.lock; do
    changes=$(diff "$tree/$file" "$copy/$file" || true)
    removed=$(printf '%s\n' "$changes" | grep -c '^<' || true)
    added=$(printf '%s\n' "$changes" | grep -c '^>' || true)
    stray=$(printf '%s\n' "$changes" | grep '^>' | grep -vF '"98.76.54"' || true)
    if [ "$removed" -gt 0 ] && [ "$removed" = "$added" ] && [ -z "$stray" ]; then
      pass "set-version.sh 98.76.54 changes no line of $file but its versions"
    else
      fail "set-version.sh 98.76.54 changes $file beyond its versions:
$changes"
    fi
  done
else
  fail "set-version.sh 98.76.54 fails on a copy of the tree's manifests: $(cat "$copy.log")"
fi

# Scenario: What is not a version.
manifests unversioned
code=0
sh "$tree/set-version.sh" 1.2 "$copy" 2>>"$copy.log" || code=$?
if [ "$code" -eq 2 ] && cmp -s "$tree/Cargo.toml" "$copy/Cargo.toml" &&
  cmp -s "$tree/Cargo.lock" "$copy/Cargo.lock"; then
  pass "set-version.sh 1.2 exits 2 and leaves both files as they were"
else
  fail "set-version.sh 1.2 exits $code, or changes Cargo.toml or Cargo.lock"
fi

# Scenario: Nowhere to write it.
manifests nowhere
awk '
  /^\[/ { section = $0 }
  section == "[workspace.package]" && /^version = "/ { next }
  { print }
' "$tree/Cargo.toml" >"$copy/Cargo.toml"
cp "$copy/Cargo.toml" "$scratch/nowhere.Cargo.toml"
code=0
said=$(sh "$tree/set-version.sh" 98.76.54 "$copy" 2>&1) || code=$?
if [ "$code" -eq 1 ] && cmp -s "$scratch/nowhere.Cargo.toml" "$copy/Cargo.toml" &&
  [ ! -e "$copy/Cargo.toml.set-version" ]; then
  case $said in
  *Cargo.toml*) pass "set-version.sh exits 1 on a Cargo.toml with no version, naming it and leaving it as it was" ;;
  *) fail "set-version.sh exits 1 on a Cargo.toml with no version without naming it: $said" ;;
  esac
else
  fail "set-version.sh exits $code on a Cargo.toml with no version, or changes it: $said"
fi

# Scenarios: The workflow's trigger, A release. The workflow as yamlfmt
# lays it out.
workflow="$tree/.github/workflows/release.yml"
# The lines of the top-level block `$1`, less the empty ones and comments.
block() {
  awk -v key="$1:" '
    $0 == key { inside = 1; next }
    inside && /^[^ #]/ { exit }
    inside && !/^ *(#|$)/ { print }
  ' "$workflow"
}
on=$(block on)
want_on="  push:
    branches:
      - main"
if [ "$on" = "$want_on" ]; then
  pass "the workflow release runs on a push to main alone"
else
  fail "the workflow release runs on more or less than a push to main:
$on"
fi
concurrency=$(block concurrency)
want_concurrency="  group: release
  cancel-in-progress: false"
if [ "$concurrency" = "$want_concurrency" ]; then
  pass "the workflow release runs one at a time, none cancelled"
else
  fail "the workflow release is not one run at a time, none cancelled:
$concurrency"
fi
# Each line `$1` matches, as a basic regular expression, once.
once() {
  [ "$(grep -c -e "$1" "$workflow")" -eq 1 ]
}
# The workflow's own lines, whose $ the shell of its step expands.
# shellcheck disable=SC2016
if once '^          tag=$(nix shell --inputs-from \. nixpkgs#git-cliff -c sh release\.sh next)$' &&
  once '^          nix shell --inputs-from \. nixpkgs#git-cliff -c sh release\.sh notes "\$TAG" >"\$RUNNER_TEMP/notes\.md"$' &&
  once '^          gh release create "\$TAG" --target "\$GITHUB_SHA" --title "\$TAG" --notes-file "\$RUNNER_TEMP/notes\.md"$' &&
  once '^          gh workflow run cache\.yml --ref "\$TAG"$' &&
  once "^        if: steps\.next\.outputs\.tag != ''$"; then
  pass "the workflow release publishes release.sh's tag with its notes and no file, then runs cache at it"
else
  fail "the workflow release does not publish release.sh's tag with its notes and no file, then run cache at it"
fi
if grep -n -E 'nix build|cargo|upload-artifact|git (push|commit|tag)' "$workflow"; then
  fail "the workflow release builds, attaches or commits something"
else
  pass "the workflow release builds, attaches and commits nothing"
fi

exit "$status"
