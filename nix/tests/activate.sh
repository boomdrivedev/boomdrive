#!/bin/sh
# The activation's cases (design Decision 14, Task notes 5.2), which
# nix/check.sh test runs beside the evaluated ones: each runs
# nix/modules/activate.sh by sh in a scratch folder of its own, with a
# scratch HOME, the module's file in a store of its own, and stubs of
# systemctl and ps first on PATH that record their calls, and of id. The
# stubs answer from the case's folder: id the user running it, alice
# unless `user` says another; systemctl's show the unit's MainPID, 0
# unless `MainPID` says another; and ps the command line in `args`. It
# prints each failure as one line, nix: <case>: <what>, and nothing where
# every case holds.
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
  program=$dir/store/boomdrive/bin/boomdrive
  mkdir -p "$home" "$dir/store" "$dir/bin"
  echo 'repositories = []' >"$file"
  echo alice >"$dir/user"
  echo 0 >"$dir/MainPID"
  : >"$dir/args"
  cat >"$dir/bin/systemctl" <<STUB
#!/bin/sh
echo "systemctl \$*" >>"$calls"
case " \$* " in *" show "*) cat "$dir/MainPID" ;; esac
STUB
  cat >"$dir/bin/ps" <<STUB
#!/bin/sh
echo "ps \$*" >>"$calls"
cat "$dir/args"
STUB
  cat >"$dir/bin/id" <<STUB
#!/bin/sh
cat "$dir/user"
STUB
  chmod +x "$dir/bin/systemctl" "$dir/bin/ps" "$dir/bin/id"
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

# The stubs' calls, on one line.
called() {
  if [ -e "$calls" ]; then
    paste -s -d ';' "$calls"
  fi
}

# What every link holds to: it calls neither systemctl nor ps.
quiet() {
  expect "$1" "link called $(called)" missing "$calls"
}

# systemctl was asked to `$1` the unit, without waiting for the job.
asked() {
  grep -qx "systemctl --user --no-block $1 boomdrive.service" "$calls" 2>/dev/null
}
unasked() {
  ! asked "$1"
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

named="A local layer beside it"
fresh
local=$home/.config/boomdrive/config.local.toml
mkdir -p "$(dirname "$local")"
echo 'repositories = ["example/local"]' >"$local"
run link "$file"
expect "$named" "link exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "$target is no link to $file" linked
expect "$named" "link changed $local" holds "$local" 'repositories = ["example/local"]'
expect "$named" "link made $local a link" [ ! -L "$local" ]
expect "$named" "link moved something aside" missing "$aside"
expect "$named" "link moved $local aside" missing "$local.before-nix"
quiet "$named"

named="Not running on NixOS"
fresh
run keep alice "$program" run --config "$file"
expect "$named" "keep exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "keep did not start the unit: $(called)" asked start
expect "$named" "keep restarted the unit" unasked restart
expect "$named" "keep says \"$out\", not that it started the unit" \
  [ "$out" = "boomdrive: started boomdrive.service" ]

named="Nothing changed on NixOS"
fresh
echo 4242 >"$dir/MainPID"
echo "$program run --config $file" >"$dir/args"
run keep alice "$program" run --config "$file"
expect "$named" "keep exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "keep read no command line of the unit's process 4242: $(called)" \
  grep -q '^ps .*-p 4242$' "$calls"
expect "$named" "keep started the unit" unasked start
expect "$named" "keep restarted the unit" unasked restart
expect "$named" "keep says \"$out$err\"" [ -z "$out$err" ]

named="A new file on NixOS"
fresh
echo 4242 >"$dir/MainPID"
echo "$program run --config $dir/store/another.toml" >"$dir/args"
run keep alice "$program" run --config "$file"
expect "$named" "keep exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "keep did not restart the unit: $(called)" asked restart
expect "$named" "keep started the unit" unasked start
expect "$named" "keep says \"$out\", not that it restarted the unit and what it ran" \
  [ "$out" = "boomdrive: restarted boomdrive.service, which ran $program run --config $dir/store/another.toml" ]

named="A new package on NixOS"
fresh
echo 4242 >"$dir/MainPID"
echo "$dir/store/older/bin/boomdrive run --config $file" >"$dir/args"
run keep alice "$program" run --config "$file"
expect "$named" "keep exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "keep did not restart the unit: $(called)" asked restart
expect "$named" "keep started the unit" unasked start

named="Another user's activation"
fresh
echo bob >"$dir/user"
run keep alice "$program" run --config "$file"
expect "$named" "keep exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "keep called $(called)" missing "$calls"
expect "$named" "keep says \"$out$err\"" [ -z "$out$err" ]
expect "$named" "keep made something in the home" missing "$home/.config"

named="keep's usage"
fresh
run keep alice
expect "$named" "keep with no program exits $status, not 2" [ "$status" -eq 2 ]
expect "$named" "keep with no program called $(called)" missing "$calls"
