#!/bin/sh
# shellcheck disable=SC2016 # awk programs name their own $fields.
# Writes a release's version into every file of the tree that carries
# Boomdrive's version: the workspace's in Cargo.toml, which each crate
# takes (version.workspace = true), and Cargo.lock's entries for the
# crates, the entries with no source. main gives 0.0.0 in each
# (check-versions.sh). `release.sh commit` runs this on a copy of the
# commit it releases, and its tag names the release commit it makes of
# what this wrote (record 0024, core.48). It changes no other line, which
# root:releases proves on a scratch copy of the tree's files.
#
#   sh set-version.sh <major.minor.patch> [<root>]
#
# <root> is the folder this script is in where it is not given.
set -eu

[ $# -ge 1 ] && [ $# -le 2 ] || {
  echo "usage: set-version.sh <major.minor.patch> [<root>]" >&2
  exit 2
}
version=$1
root=${2:-$(dirname "$0")}
printf '%s\n' "$version" | grep -Eqx '[0-9]+\.[0-9]+\.[0-9]+' || {
  echo "set-version: $version is not a version major.minor.patch" >&2
  exit 2
}

# The file $1 under the root, rewritten through a file beside it by the
# awk program $2 with the version as `v`; the program counts what it
# wrote in `written`, and a file it wrote nothing in is refused and left
# as it was.
put() {
  file=$root/$1
  awk -v v="$version" "$2"'
    END { exit (written ? 0 : 1) }
  ' "$file" >"$file.set-version" || {
    rm -f "$file.set-version"
    echo "set-version: $1 gives no version where Boomdrive's is" >&2
    exit 1
  }
  mv "$file.set-version" "$file"
}

# The first version under [workspace.package].
put Cargo.toml '
  /^\[/ { section = $0 }
  section == "[workspace.package]" && /^version = "/ && !written {
    $0 = "version = \"" v "\""
    written++
  }
  { print }
'

# The crates are the entries of the lock with no source.
put Cargo.lock '
  function flush(i) {
    for (i = 1; i <= n; i++) {
      if (package && !sourced && lines[i] ~ /^version = "/) {
        lines[i] = "version = \"" v "\""
        written++
      }
      print lines[i]
    }
    n = 0
    sourced = 0
  }
  /^\[\[package\]\]$/ { flush(); package = 1 }
  /^source = / { sourced = 1 }
  { lines[++n] = $0 }
  END { flush() }
'
