#!/bin/sh
# The way in's cases (design Decision 8 of the change
# herdr-and-ghostty-come-with-the-flake-and-an-app-opens-the-agents),
# which nix/check.sh test runs beside the evaluated ones: each runs
# nix/packages/launcher.sh by sh in a scratch folder of its own, with a
# scratch HOME and stubs of flume, osascript, uname and id that record
# their calls. The launcher puts the host's profiles ahead of its PATH,
# where a host that takes the flake has a Flume of its own, so each stub is
# reached through a function of its name, which sh finds before PATH, and
# no real Flume or osascript runs. The stubs answer from the case's folder:
# id the user in `user`, alice unless it says another; uname the system in
# `system`; and flume the words in `words`, exiting with the status in
# `status`. It prints each failure as one line, nix: <case>: <what>, and
# nothing where every case holds.
set -eu

launcher=$(cd "$(dirname "$0")/../packages" && pwd)/launcher.sh
# The stubs run under the PATH a case gives the launcher, so each names
# cat by the path the test's own PATH finds it at.
cat=$(command -v cat)
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT
cases=0

# A case's folder: its home, the stubs and their calls.
fresh() {
  cases=$((cases + 1))
  dir=$scratch/$cases
  home=$dir/home/alice
  calls=$dir/calls
  mkdir -p "$home" "$dir/bin"
  echo alice >"$dir/user"
  echo Linux >"$dir/system"
  echo 0 >"$dir/status"
  : >"$dir/words"
  cat >"$dir/bin/flume" <<STUB
#!/bin/sh
echo "flume \$*" >>"$calls"
printf '%s\n' "\$PATH" >"$dir/flume-path"
"$cat" "$dir/words"
exit "\$("$cat" "$dir/status")"
STUB
  cat >"$dir/bin/osascript" <<STUB
#!/bin/sh
echo "osascript" >>"$calls"
printf '%s\n' "\$@" >"$dir/osascript-args"
STUB
  cat >"$dir/bin/uname" <<STUB
#!/bin/sh
"$cat" "$dir/system"
STUB
  cat >"$dir/bin/id" <<STUB
#!/bin/sh
"$cat" "$dir/user"
STUB
  chmod +x "$dir/bin/flume" "$dir/bin/osascript" "$dir/bin/uname" "$dir/bin/id"
  cat >"$dir/stubs" <<STUBS
flume() { "$dir/bin/flume" "\$@"; }
osascript() { "$dir/bin/osascript" "\$@"; }
uname() { "$dir/bin/uname" "\$@"; }
id() { "$dir/bin/id" "\$@"; }
STUBS
}

# The launcher in the case's folder, with the PATH `$1`: its status,
# output and error.
launch() {
  status=0
  HOME=$home PATH=$1 stubs=$dir/stubs launcher=$launcher \
    sh -c '. "$stubs"; . "$launcher"' >"$dir/out" 2>"$dir/err" || status=$?
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

# The stubs' calls, on one line.
called() {
  if [ -e "$calls" ]; then
    paste -s -d ';' "$calls"
  fi
}
holds() {
  [ "$(cat "$1" 2>/dev/null)" = "$2" ]
}
missing() {
  [ ! -e "$1" ]
}
# `$1` names `$2` before `$3`.
before() {
  case :$1: in *":$2:"*":$3:"*) ;; *) return 1 ;; esac
}
# `$1` holds `$2`.
has() {
  case $1 in *"$2"*) ;; *) return 1 ;; esac
}

refusal='no window program could be run'

named="Opened"
fresh
echo 'flume: opened the session' >"$dir/words"
launch /usr/bin:/bin
expect "$named" "the launcher exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "the launcher called $(called), not flume open once" holds "$calls" "flume open"
expect "$named" "the launcher showed \"$out$err\"" [ -z "$out$err" ]

named="Flume refuses on macOS"
fresh
echo Darwin >"$dir/system"
echo 1 >"$dir/status"
echo "$refusal" >"$dir/words"
launch /usr/bin:/bin
expect "$named" "the launcher exits $status, not 1" [ "$status" -eq 1 ]
expect "$named" "the launcher called $(called), not flume open then osascript" \
  holds "$calls" "flume open
osascript"
expect "$named" "osascript's last argument is \"$(tail -n 1 "$dir/osascript-args" 2>/dev/null)\", not Flume's words" \
  [ "$(tail -n 1 "$dir/osascript-args" 2>/dev/null)" = "$refusal" ]
expect "$named" "osascript's script holds no display alert \"Boomdrive\"" \
  grep -q 'display alert "Boomdrive"' "$dir/osascript-args"
expect "$named" "the launcher said \"$out$err\"" [ -z "$out$err" ]

named="Flume refuses elsewhere"
fresh
echo 1 >"$dir/status"
echo "$refusal" >"$dir/words"
launch /usr/bin:/bin
expect "$named" "the launcher exits $status, not 1" [ "$status" -eq 1 ]
expect "$named" "the launcher's standard error is \"$err\", not Flume's words" [ "$err" = "$refusal" ]
expect "$named" "the launcher asked for an alert" missing "$dir/osascript-args"

named="The profiles first"
fresh
launch /usr/bin:/bin
given=$(cat "$dir/flume-path" 2>/dev/null || true)
expect "$named" "the launcher exits $status: $err" [ "$status" -eq 0 ]
expect "$named" "Flume's PATH $given names /etc/profiles/per-user/alice/bin not before /usr/bin" \
  before "$given" /etc/profiles/per-user/alice/bin /usr/bin
expect "$named" "Flume's PATH $given names $home/.nix-profile/bin not before /usr/bin" \
  before "$given" "$home/.nix-profile/bin" /usr/bin
expect "$named" "Flume's PATH $given leaves out the PATH it was given" has "$given" :/usr/bin:/bin
