#!/bin/sh
# The activation's cases (design Decision 14, Task notes 5.2), which
# nix/check.sh test runs beside the evaluated ones: each runs
# nix/modules/activate.sh by sh in a scratch folder of its own, with a
# scratch HOME, the module's file in a store of its own, and stubs of
# systemctl and ps first on PATH that record their calls. It prints each
# failure as one line, nix: <case>: <what>, and nothing where every case
# holds.
set -eu

activate=$(cd "$(dirname "$0")/../modules" && pwd)/activate.sh
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT
cases=0

# A case's folder: its home, the module's file, the stubs and their calls.
fresh() {
  cases=$((cases + 1))
  dir=$scratch/$cases
  home=$dir/home
  file=$dir/store/boomdrive-config.toml
  target=$home/.config/boomdrive/config.toml
  aside=$target.before-nix
  calls=$dir/calls
  mkdir -p "$home" "$dir/store" "$dir/bin"
  echo 'repositories = []' >"$file"
  for tool in systemctl ps; do
    printf '#!/bin/sh\necho "%s $*" >>"%s"\n' "$tool" "$calls" >"$dir/bin/$tool"
    chmod +x "$dir/bin/$tool"
  done
}

# activate.sh with `$@` in the case's folder: its status, output and
# error.
run() {
  status=0
  HOME=$home PATH=$dir/bin:$PATH sh "$activate" "$@" >"$dir/out" 2>"$dir/err" || status=$?
  out=$(cat "$dir/out")
  err=$(cat "$dir/err")
}

# A failure of the case `$1`, saying `$2`, where the test after them fails.
expect() {
  named=$1
  what=$2
  shift 2
  if ! "$@"; then
    echo "nix: $named: $what"
  fi
}

linked() {
  [ -L "$target" ] && [ "$(readlink "$target")" = "$file" ]
}
holds() {
  [ "$(cat "$1" 2>/dev/null)" = "$2" ]
}
missing() {
  [ ! -e "$1" ] && [ ! -L "$1" ]
}
# `$1` names `$2`, then `$3`.
names() {
  case $1 in *"$2"*"$3"*) ;; *) return 1 ;; esac
}

# What every link holds to: it calls neither systemctl nor ps.
quiet() {
  expect "$1" "link called $(cat "$calls" 2>/dev/null)" missing "$calls"
}

named="An empty home"
fresh
run link "$file"
expect "$named" "link exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "$target is no link to $file" linked
expect "$named" "link says \"$out\", not that it linked $target" \
  [ "$out" = "boomdrive: linked $target to $file" ]
expect "$named" "link moved something aside" missing "$aside"
quiet "$named"

named="A person's own file"
fresh
mkdir -p "$(dirname "$target")"
echo 'repositories = ["example/own"]' >"$target"
run link "$file"
expect "$named" "link exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "$target is no link to $file" linked
expect "$named" "$aside does not hold the person's file" \
  holds "$aside" 'repositories = ["example/own"]'
expect "$named" "link says \"$out\", not both paths" \
  [ "$out" = "boomdrive: moved $target to $aside
boomdrive: linked $target to $file" ]
quiet "$named"

fresh
mkdir -p "$target"
echo 'kept' >"$target/inside"
run link "$file"
expect "$named" "link of a folder exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "$target is no link to $file, where a folder was" linked
expect "$named" "$aside does not hold the person's folder" holds "$aside/inside" kept
quiet "$named"

named="Already linked"
fresh
mkdir -p "$(dirname "$target")"
ln -s "$file" "$target"
run link "$file"
expect "$named" "link exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "$target is no link to $file" linked
expect "$named" "link says \"$out$err\"" [ -z "$out$err" ]
expect "$named" "link moved something aside" missing "$aside"
quiet "$named"

named="A link to another file"
fresh
mkdir -p "$(dirname "$target")"
ln -s "$dir/store/another.toml" "$target"
run link "$file"
expect "$named" "link exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "$target is no link to $file" linked
expect "$named" "link says \"$out\", not that it linked $target" \
  [ "$out" = "boomdrive: linked $target to $file" ]
expect "$named" "link moved the other link aside" missing "$aside"
quiet "$named"

named="A file moved aside before"
fresh
mkdir -p "$(dirname "$target")"
echo 'repositories = ["example/own"]' >"$target"
echo 'repositories = ["example/older"]' >"$aside"
run link "$file"
expect "$named" "link exits $status, not 1" [ "$status" -eq 1 ]
expect "$named" "link changed $target" holds "$target" 'repositories = ["example/own"]'
expect "$named" "link changed $aside" holds "$aside" 'repositories = ["example/older"]'
expect "$named" "link's refusal \"$err\" names not both paths" \
  names "$err" "$target" "$aside"
expect "$named" "link says \"$out\"" [ -z "$out" ]
quiet "$named"
