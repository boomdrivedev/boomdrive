#!/bin/sh
# shellcheck disable=SC2016 # awk programs name their own $fields.
# main's tree names 0.0.0 (record 0024, core.48): Cargo.toml's
# [workspace.package] gives version 0.0.0, every crate its [workspace]
# members lists takes it (version.workspace = true), and Cargo.lock's
# entries with no source are exactly those crates, each giving it. A
# release's version is written into the tree only by its release commit,
# which `release.sh commit` cuts off main with set-version.sh, so this is
# a check of a candidate's tree, which the gate runs as root:versions and
# never at a tag. It names the file or the crate and both versions.
# root:releases runs it on scratch copies of the tree's manifests too.
#
#   sh check-versions.sh [<root>]
#
# <root> is the folder this script is in where it is not given.
set -u

[ $# -le 1 ] || {
  echo "usage: check-versions.sh [<root>]" >&2
  exit 2
}
root=${1:-$(dirname "$0")}
main=0.0.0

status=0
fail() {
  echo "versions: $*" >&2
  status=1
}

# The version Cargo.toml gives under [workspace.package], or nothing.
workspace=$(awk '
  /^\[/ { section = $0 }
  section == "[workspace.package]" && /^version = "/ {
    gsub(/^version = "|"$/, "")
    print
    exit
  }
' "$root/Cargo.toml") || exit 1
if [ -z "$workspace" ]; then
  fail "Cargo.toml gives no version under [workspace.package], and main's is $main"
elif [ "$workspace" != "$main" ]; then
  fail "Cargo.toml gives $workspace under [workspace.package], and main's is $main"
fi
# What the crates and the lock are held to.
want=${workspace:-$main}

# The paths [workspace] members lists, a line each.
members=$(awk '
  /^\[/ { section = $0 }
  section == "[workspace]" && /^members *= *\[/ { inside = 1 }
  inside {
    line = $0
    sub(/#.*/, "", line)
    while (match(line, /"[^"]*"/)) {
      print substr(line, RSTART + 1, RLENGTH - 2)
      line = substr(line, RSTART + RLENGTH)
    }
    if (line ~ /\]/) inside = 0
  }
' "$root/Cargo.toml")
[ -n "$members" ] || fail "Cargo.toml lists no members under [workspace]"

# Each entry of the lock with no source, a line of its name and version.
lock=$(awk '
  function flush() {
    if (package && !sourced) print name " " (version == "" ? "none" : version)
    sourced = 0
    name = ""
    version = ""
  }
  /^\[\[package\]\]$/ { flush(); package = 1; next }
  /^name = "/ { name = $0; gsub(/^name = "|"$/, "", name) }
  /^version = "/ { version = $0; gsub(/^version = "|"$/, "", version) }
  /^source = / { sourced = 1 }
  END { flush() }
' "$root/Cargo.lock") || exit 1

names=" "
for member in $members; do
  manifest=$member/Cargo.toml
  if [ ! -f "$root/$manifest" ]; then
    fail "$manifest, of a member Cargo.toml lists, is not there"
    continue
  fi
  # The crate's name, and `workspace` where it takes the workspace's
  # version, the version it gives of its own where it does not, or none.
  info=$(awk '
    /^\[/ { section = $0 }
    section == "[package]" && /^name = "/ { name = $0; gsub(/^name = "|"$/, "", name) }
    section == "[package]" && /^version\.workspace = true$/ { taken = "workspace" }
    section == "[package]" && /^version = "/ { taken = $0; gsub(/^version = "|"$/, "", taken) }
    END { print name " " (taken == "" ? "none" : taken) }
  ' "$root/$manifest")
  name=${info%% *}
  taken=${info#* }
  if [ "$taken" != workspace ]; then
    fail "$manifest gives $taken for the crate ${name:-unnamed}, not version.workspace = true, the workspace's $want"
  fi
  [ -n "$name" ] || continue
  names="$names$name "
  locked=$(printf '%s\n' "$lock" | awk -v name="$name" '$1 == name { print $2; exit }')
  if [ -z "$locked" ]; then
    fail "Cargo.lock gives no entry for the crate $name"
  elif [ "$locked" != "$want" ]; then
    fail "Cargo.lock gives $locked for the crate $name, and Cargo.toml's workspace $want"
  fi
done

# An entry with no source that is no member's.
strays=$(printf '%s\n' "$lock" | awk -v names="$names" 'NF && index(names, " " $1 " ") == 0 { print $1 " " $2 }')
if [ -n "$strays" ]; then
  printf '%s\n' "$strays" | while read -r name version; do
    echo "versions: Cargo.lock gives $version for $name, an entry with no source that is no crate of the workspace" >&2
  done
  status=1
fi

if [ "$status" -eq 0 ]; then
  echo "versions: Cargo.toml, its crates and Cargo.lock give $main"
fi
exit "$status"
