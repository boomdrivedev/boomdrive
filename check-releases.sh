#!/bin/sh
# The releases spec (record 0024, core.48), proven in scratch repositories
# made in a temporary directory: release.sh's next tag and its notes under
# the tree's cliff.toml, after a tag on main and after one on a release
# commit off it, release-preview beside them, check-commits.sh under the
# tree's .versionrc, set-version.sh and check-versions.sh on scratch
# copies of the tree's manifests, release.sh's release commit of a main
# that commits them, and the workflow release read line by line.
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
    "$tree/set-version.sh" "$tree/check-commits.sh" "$dir/"
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

# A release commit tagged `$1`, cut from main's tip in `$dir` as the
# workflow cuts one: main's tree with a file of its own giving the
# version, main's tip its one parent, and main left where it was, so the
# tag is off main's history.
cut_release() {
  blob=$(printf '%s\n' "${1#v}" | git -C "$dir" hash-object -w --stdin)
  GIT_INDEX_FILE="$dir.index" git -C "$dir" read-tree HEAD
  GIT_INDEX_FILE="$dir.index" git -C "$dir" update-index --add --cacheinfo "100644,$blob,version"
  cut_tree=$(GIT_INDEX_FILE="$dir.index" git -C "$dir" write-tree)
  rm -f "$dir.index"
  cut_commit=$(git -C "$dir" commit-tree "$cut_tree" -p HEAD -m "chore(release): $1")
  git -C "$dir" tag "$1" "$cut_commit"
}

# Scenario: A fix after a release commit.
repo cutfix
commit "feat: a start"
cut_release v0.1.0
commit "fix: a fix"
got=$(next)
if [ "$got" = v0.1.1 ]; then
  pass "'fix: a fix' after v0.1.0's release commit releases v0.1.1"
else
  fail "'fix: a fix' after v0.1.0's release commit releases ${got:-nothing}, not v0.1.1"
fi

# Scenario: Nothing to release after a release commit.
repo cutnothing
commit "feat: a start"
cut_release v0.1.0
commit "chore: a chore"
got=$(next)
if [ -z "$got" ] && grep -q 'v0.1.0 is released already' "$scratch/cutnothing.log"; then
  pass "a chore after v0.1.0's release commit releases nothing"
else
  fail "a chore after v0.1.0's release commit releases ${got:-nothing}, or says nothing of v0.1.0"
fi

# Scenario: The notes after a release commit.
repo cutnotes
commit "feat: a start"
cut_release v0.1.0
commit "fix(nix): a fix"
cut_notes=$(cd "$dir" && sh release.sh notes v0.1.1 2>>"$dir.log")
fixes=$(printf '%s\n' "$cut_notes" | grep -n -F -x -e "### Fixes" | head -n 1 | cut -d : -f 1)
fix=$(printf '%s\n' "$cut_notes" | grep -n '^- \*\*nix\*\*: A fix (\[' | head -n 1 | cut -d : -f 1)
if [ -n "$fixes" ] && [ -n "$fix" ] && [ "$fixes" -lt "$fix" ] &&
  ! printf '%s\n' "$cut_notes" | grep -q -i 'a start'; then
  pass "the notes after v0.1.0's release commit hold the fix under Fixes and nothing of the start"
else
  fail "the notes after v0.1.0's release commit do not hold the fix under Fixes, or hold the start:
$cut_notes"
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

# Scenario: Before a release commit.
repo cutbefore
commit "Add a feature"
commit "feat: a start"
cut_release v0.1.0
commit "fix: a fix"
if said=$(cd "$dir" && sh check-commits.sh 2>&1); then
  pass "check-commits.sh passes 'Add a feature' before v0.1.0's release commit"
else
  fail "check-commits.sh fails 'Add a feature' before v0.1.0's release commit: $said"
fi

# Scenario: After a release commit.
repo cutafter
commit "feat: a start"
cut_release v0.1.0
commit "Add a feature"
if said=$(cd "$dir" && sh check-commits.sh 2>&1); then
  fail "check-commits.sh passes 'Add a feature' after v0.1.0's release commit"
else
  case $said in
  *"Add a feature"*) pass "check-commits.sh fails 'Add a feature' after v0.1.0's release commit, naming it" ;;
  *) fail "check-commits.sh fails 'Add a feature' after v0.1.0's release commit without naming it: $said" ;;
  esac
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

# check-versions.sh passes on a copy of the tree's manifests, which
# root:versions holds to main's.
manifests main
if said=$(sh "$tree/check-versions.sh" "$copy" 2>&1); then
  pass "check-versions.sh passes a copy of the tree's manifests"
else
  fail "check-versions.sh fails a copy of the tree's manifests: $said"
fi

# Scenario: A version where main's should be.
manifests released
sh "$tree/set-version.sh" 0.8.0 "$copy" 2>>"$copy.log"
if said=$(sh "$tree/check-versions.sh" "$copy" 2>&1); then
  fail "check-versions.sh passes a copy that set-version.sh 0.8.0 wrote"
else
  case $said in
  *Cargo.toml*0.8.0*) pass "check-versions.sh fails a copy that set-version.sh 0.8.0 wrote, naming Cargo.toml and 0.8.0" ;;
  *) fail "check-versions.sh fails a copy that set-version.sh 0.8.0 wrote without naming Cargo.toml and 0.8.0: $said" ;;
  esac
fi

# Scenario: A crate's entry that differs.
manifests differs
awk '
  /^name = "/ { entry = $0 }
  entry == "name = \"boomdrive-control\"" && /^version = "/ { $0 = "version = \"0.8.0\""; entry = "" }
  { print }
' "$tree/Cargo.lock" >"$copy/Cargo.lock"
if said=$(sh "$tree/check-versions.sh" "$copy" 2>&1); then
  fail "check-versions.sh passes a Cargo.lock that gives boomdrive-control 0.8.0"
else
  case $said in
  *0.8.0*boomdrive-control* | *boomdrive-control*0.8.0*) pass "check-versions.sh fails a Cargo.lock that gives boomdrive-control 0.8.0, naming the crate and 0.8.0" ;;
  *) fail "check-versions.sh fails a Cargo.lock that gives boomdrive-control 0.8.0 without naming the crate and 0.8.0: $said" ;;
  esac
fi

# Scenarios: The release commit, main is left as it was, A tag that
# exists, What is not a tag. A repository whose main commits the tree's
# manifests beside a file, and a change to that file left uncommitted.
repo releasecommit
manifests releasecommit
git -C "$dir" add Cargo.toml Cargo.lock crates
commit "feat: a start"
echo "an edit left uncommitted" >>"$dir/file-1"
# What a release commit leaves as it was: main's tip, the branch checked
# out, every branch, the index, the working tree and git status.
state() {
  git -C "$dir" rev-parse main
  git -C "$dir" symbolic-ref HEAD
  git -C "$dir" for-each-ref --format='%(refname) %(objectname)' refs/heads
  git -C "$dir" ls-files -s
  git -C "$dir" diff
  git -C "$dir" status --porcelain --untracked-files=all
}
# The objects git holds loose, which a commit made adds to.
objects() {
  git -C "$dir" count-objects | cut -d ' ' -f 1
}
main_tip=$(git -C "$dir" rev-parse main)
before=$(state)
if (cd "$dir" && sh release.sh commit v0.2.0 2>"$dir.log"); then
  released=$(git -C "$dir" rev-parse -q --verify "refs/tags/v0.2.0^{commit}" || true)
  mkdir -p "$scratch/released"
  if [ -n "$released" ]; then
    git -C "$dir" cat-file blob "$released:Cargo.toml" >"$scratch/released/Cargo.toml"
    git -C "$dir" cat-file blob "$released:Cargo.lock" >"$scratch/released/Cargo.lock"
  fi
  others=$(lock_crates "$scratch/released" | awk '$2 != "0.2.0"')
  if [ -n "$released" ] &&
    [ "$(git -C "$dir" rev-list --parents -n 1 "$released")" = "$released $main_tip" ] &&
    [ "$(git -C "$dir" log -1 --format=%s "$released")" = "chore(release): v0.2.0" ] &&
    [ "$(git -C "$dir" diff --name-only main "$released" | tr '\n' ' ')" = "Cargo.lock Cargo.toml " ] &&
    [ "$(workspace_version "$scratch/released")" = 0.2.0 ] &&
    [ -n "$(lock_crates "$scratch/released")" ] && [ -z "$others" ] &&
    grep -q "release: v0.2.0 names $released, cut from $main_tip" "$dir.log"; then
    pass "release.sh commit v0.2.0 tags a commit cut from main, chore(release): v0.2.0, changing Cargo.toml and Cargo.lock alone to give 0.2.0, and says so"
  else
    fail "release.sh commit v0.2.0 does not tag a commit cut from main, chore(release): v0.2.0, changing Cargo.toml and Cargo.lock alone to give 0.2.0, or does not say so: $(cat "$dir.log")"
  fi
  if [ "$(state)" = "$before" ]; then
    pass "release.sh commit v0.2.0 leaves main, the branch checked out, the index, the working tree and git status as they were"
  else
    fail "release.sh commit v0.2.0 changes main, the branch checked out, the index, the working tree or git status:
$(state)"
  fi
else
  fail "release.sh commit v0.2.0 fails on main: $(cat "$dir.log")"
fi
held=$(objects)
code=0
said=$(cd "$dir" && sh release.sh commit v0.2.0 2>&1) || code=$?
if [ "$code" -eq 1 ] && [ "$(objects)" = "$held" ] &&
  [ "$(git -C "$dir" rev-parse -q --verify "refs/tags/v0.2.0^{commit}")" = "$released" ]; then
  case $said in
  *v0.2.0*) pass "release.sh commit v0.2.0 exits 1 where v0.2.0 is a tag, naming it, and makes no commit" ;;
  *) fail "release.sh commit v0.2.0 exits 1 where v0.2.0 is a tag without naming it: $said" ;;
  esac
else
  fail "release.sh commit v0.2.0 exits $code where v0.2.0 is a tag, or makes a commit: $said"
fi
tags=$(git -C "$dir" tag --list)
code=0
said=$(cd "$dir" && sh release.sh commit 0.2.0 2>&1) || code=$?
if [ "$code" -eq 2 ] && [ "$(objects)" = "$held" ] && [ "$(git -C "$dir" tag --list)" = "$tags" ]; then
  pass "release.sh commit 0.2.0 exits 2 and makes no commit and no tag"
else
  fail "release.sh commit 0.2.0 exits $code, or makes a commit or a tag: $said"
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
# The line `$1` matches, as a basic regular expression, at.
line_of() {
  grep -n -e "$1" "$workflow" | head -n 1 | cut -d : -f 1
}
# The workflow's own lines, whose $ the shell of its step expands.
# shellcheck disable=SC2016
notes_line='^          nix shell --inputs-from \. nixpkgs#git-cliff -c sh release\.sh notes "\$TAG" >"\$RUNNER_TEMP/notes\.md"$'
# shellcheck disable=SC2016
commit_line='^          sh release\.sh commit "\$TAG"$'
# shellcheck disable=SC2016
push_line='^          git -c credential\.helper= -c '"'"'credential\.helper=!gh auth git-credential'"'"' push origin "refs/tags/\$TAG"$'
# shellcheck disable=SC2016
create_line='^          gh release create "\$TAG" --verify-tag --title "\$TAG" --notes-file "\$RUNNER_TEMP/notes\.md"$'
# shellcheck disable=SC2016
cache_line='^          gh workflow run cache\.yml --ref "\$TAG"$'
# shellcheck disable=SC2016
if once '^          tag=$(nix shell --inputs-from \. nixpkgs#git-cliff -c sh release\.sh next)$' &&
  once "^        if: steps\.next\.outputs\.tag != ''$" &&
  once "$notes_line" && once "$commit_line" && once "$push_line" &&
  once "$create_line" && once "$cache_line" &&
  [ "$(line_of "$notes_line")" -lt "$(line_of "$commit_line")" ] &&
  [ "$(line_of "$commit_line")" -lt "$(line_of "$push_line")" ] &&
  [ "$(line_of "$push_line")" -lt "$(line_of "$create_line")" ] &&
  [ "$(line_of "$create_line")" -lt "$(line_of "$cache_line")" ]; then
  pass "the workflow release writes release.sh's notes, makes the release commit of its tag, pushes the tag, publishes it with its notes and no file, then runs cache at it"
else
  fail "the workflow release does not write release.sh's notes, make the release commit of its tag, push the tag, publish it with its notes and no file, then run cache at it, in that order"
fi
if once '^          persist-credentials: false$' &&
  once '^          GIT_AUTHOR_NAME: github-actions\[bot\]$' &&
  once '^          GIT_AUTHOR_EMAIL: 41898282+github-actions\[bot\]@users\.noreply\.github\.com$' &&
  once '^          GIT_COMMITTER_NAME: github-actions\[bot\]$' &&
  once '^          GIT_COMMITTER_EMAIL: 41898282+github-actions\[bot\]@users\.noreply\.github\.com$'; then
  pass "the workflow release's checkout keeps no credentials, and its release commit is the bot's"
else
  fail "the workflow release's checkout keeps credentials, or its release commit is not the bot's"
fi
if grep -n -E 'nix build|cargo|upload-artifact|gh release upload|git (commit|tag)' "$workflow"; then
  fail "the workflow release builds, attaches or commits something"
else
  pass "the workflow release builds, attaches and commits nothing"
fi
pushes=$(grep -c -E 'git([[:space:]].*)?[[:space:]]push([[:space:]]|$)' "$workflow" || true)
if [ "$pushes" -eq 1 ] && once "$push_line"; then
  pass "the workflow release's one push pushes the release's tag alone, with no force"
else
  fail "the workflow release pushes more than the release's tag, or with force"
fi

exit "$status"
