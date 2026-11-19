#!/bin/sh
# The modules' activation, run as the service's user (design Decisions 9
# and 10):
#
#   activate.sh link <file>
#   activate.sh keep <user> <program> <argument>...
#   activate.sh plugins <herdr> <plugin>...
#   activate.sh pendingyou <source> <folder>
#   activate.sh claude <flume> <jq> <marketplace> <server> <plugin>... [-- <plugin>...]
#   activate.sh managed <path> [<source>]
#   activate.sh sign <keychain> <folder> <program>
#
# links ~/.config/boomdrive/config.toml to the module's file, so
# `boomdrive enroll` and `boomdrive run --once` run by hand read what the
# service reads. A link there is replaced. A file or folder there that is
# no link is moved once, to config.toml.before-nix, and both paths are
# named. Where config.toml.before-nix is there already, it changes neither,
# names both and fails: the service reads its own --config either way. It
# says a line only where it moved or linked something.
#
# keep is NixOS's step, since NixOS restarts no changed user unit: run by
# <user>, it starts boomdrive.service where it is not running, restarts it
# where the running runner's command line, by ps, is not <program>
# <argument>..., the unit's own, and does nothing where they are one. Run
# by any other user, it does nothing. It waits for no job, since it runs
# inside the user's nixos-activation.service, and says a line only where
# it started or restarted the unit.
#
# plugins links each herdr plugin, in order, with herdr's own command,
# `<herdr> plugin link <plugin>`, which records it in herdr's own file and
# starts no server (record 0023; design Decision 6 of the change
# herdr-and-ghostty-come-with-the-flake-and-an-app-opens-the-agents). herdr
# runs with standard input from /dev/null and every HERDR_* variable of
# its caller's unset, so it reaches the user's own herdr home and no
# server's. herdr's answer is left unsaid where the link holds; a link herdr
# refuses is said in one line naming the plugin and herdr's words, and
# stops no other. It exits 0.
#
# pendingyou writes Pending You's folder, <folder>, from the store folder
# <source> the module gives (record 0044, point 7; design Decision 8 of the
# change crews-companions-come-with-the-flake): each file of <source>, by
# its path under it, whole, as a regular file with the source's mode
# writable by its owner, 0755 where the source runs and 0644 where it does
# not, by a temporary file beside it renamed over its path, and only where
# no regular file there has the same bytes and mode. A link standing there
# is replaced. The wake mod's mod/hooks/wake.js, mod/hooks/register.js and
# mod/hooks/hooks.json go first, in that order, as Pending You's `init`
# writes them, so a session that reloads the mod finds the modules its
# hooks name (record 0044, point 5); then the other files by their paths.
# It makes <folder> with mode 0700 where it is missing, and
# each folder under it with 0755. It reads, writes and removes nothing else
# under <folder>: the sign-ins, the computer's key, the cards and the state
# are the command line's. It says one line naming each file it wrote, and
# nothing where none differed; a file it cannot write is said in one line
# naming it and stops the files after it. It exits 0.
#
# claude installs the set's marketplace of Claude Code plugins,
# <marketplace>, into every Claude Code home of Flume's accounts and
# Claude Code's default home, by Claude Code's own commands alone (record
# 0044, points 1 and 3, and its amendments of 2026-11-18; design
# Decisions 2 to 4 and 6 of the change
# every-claude-code-session-has-boomdrives-plugins). It finds `claude` on
# its PATH, and where it finds none it says so in one line and runs
# nothing. It reads the homes from `<flume> --json status`, with <jq>: the
# `home` of each account of `.status.host.pools[].accounts[]` whose
# `program` is `claude` and which gives one, each once, in the order status
# gives them, after the default home, with CLAUDE_CONFIG_DIR unset; where
# Flume's status cannot be read, it says so in one line and takes the
# default home alone. In each home, with CLAUDE_CONFIG_DIR set to it or
# unset, and standard input from /dev/null, it runs `claude plugin
# marketplace add <marketplace> --json`, which adds `boomdrive` or points it
# at this build, and `claude plugin list --json`. Then, each naming
# `<plugin>@boomdrive` with `--json`, it runs `install` for each plugin
# before `--` the list lacks, `update --scope user` for each the list holds
# at another version than the marketplace's, and `uninstall` for each
# after `--` the list holds. Where <server> is a file, not `-`, and the
# home's own .claude.json, `$CLAUDE_CONFIG_DIR/.claude.json` or else
# `$HOME/.claude.json`, names no `mcpServers.pendingyou`, it then runs
# `claude mcp add-json --scope user pendingyou` with the file's JSON. It
# names no other marketplace, plugin or server, and removes no marketplace
# or server. Each command Claude Code refuses or fails is said in one line,
# naming the home, the command and Claude Code's words, the `message` of
# its answer, and stops no other. It says nothing where every command held,
# and exits 0.
#
# managed is nix-darwin's root step for each of Claude Code's two managed
# documents, run as root once for each: the telemetry document (design
# Decisions 6, 8 and 9 of the change
# every-claude-code-session-exports-to-the-hosts-collector), whose <path>
# is 10-boomdrive-telemetry.json in Claude Code's managed-settings.d, and
# the transcripts document (design Decision 6 of the change
# the-set-keeps-claude-codes-transcripts-a-year), whose <path> is
# 10-boomdrive-transcripts.json there. Given <source>, it writes <source> to <path> whole,
# as a regular file of mode 0644, by a temporary file beside it renamed
# over the path, and only where no regular file there has the same bytes
# and mode. A link standing there is replaced. It makes the path's folder,
# and any folder above it, with mode 0755 where it is missing. Given none,
# it removes the file or link at <path> where one stands. It reads, writes
# and removes nothing else in the folder. It says one line naming the path
# where it wrote or removed it, and nothing where nothing differed or
# nothing stood; a write or a removal that fails is said in one line naming
# the path and why. It exits 0.
#
# sign is nix-darwin's step, run as root before nix-darwin loads its
# agents (record 0047; design Decisions 1 to 4 of the change
# the-runners-macos-grant-outlasts-a-new-build). It makes <folder> with
# mode 0755 where it is not there, and goes on only where <folder> is owned
# by the user it runs as and no other user can write it: otherwise it makes
# no identity, signs nothing, writes nothing there and fails. It finds the
# host's code-signing identity, Boomdrive runner, in <keychain>. Where the
# keychain holds none, it makes one: a key and a self-signed certificate
# for code signing, written in a folder only it reads, which goes whatever
# happens, and imported, the key non-extractable and usable by
# /usr/bin/codesign alone. Where the keychain holds two or more, it signs
# nothing and fails, naming each. It keeps <folder>/boomdrive a copy of
# <program> signed with the identity under the identifier boomdrive, and
# <folder>/boomdrive.source naming <program>. Where the copy is not there,
# the source names another program, or the copy is not signed with the
# identity, it copies <program> to boomdrive.new, signs that with no
# timestamp, removes boomdrive.source, renames the copy over boomdrive,
# then writes the source by a rename, so the source never names a program
# the copy was not made from; otherwise it changes nothing and says
# nothing. A copy it cannot make, sign or put in place leaves boomdrive as
# it was and no other file, and fails. Each line of what it made goes to
# standard output, and a refusal to standard error.
set -eu

usage() {
  echo "usage: activate.sh link <file>" >&2
  echo "       activate.sh keep <user> <program> <argument>..." >&2
  echo "       activate.sh plugins <herdr> <plugin>..." >&2
  echo "       activate.sh pendingyou <source> <folder>" >&2
  echo "       activate.sh claude <flume> <jq> <marketplace> <server> <plugin>... [-- <plugin>...]" >&2
  echo "       activate.sh managed <path> [<source>]" >&2
  echo "       activate.sh sign <keychain> <folder> <program>" >&2
  exit 2
}

# `$1`'s lines as one.
oneline() {
  words=$(printf '%s\n' "$1" | tr '\n' ' ')
  printf '%s' "${words% }"
}

link() {
  file=$1
  folder=$HOME/.config/boomdrive
  target=$folder/config.toml
  aside=$target.before-nix

  if [ -L "$target" ] && [ "$(readlink "$target")" = "$file" ]; then
    return
  fi
  if [ -e "$target" ] && [ ! -L "$target" ]; then
    if [ -e "$aside" ] || [ -L "$aside" ]; then
      echo "boomdrive: $target is no link and $aside is there already: left both, linked nothing; move one aside to link $file" >&2
      exit 1
    fi
    mv "$target" "$aside"
    echo "boomdrive: moved $target to $aside"
  fi
  mkdir -p "$folder"
  ln -sfn "$file" "$target"
  echo "boomdrive: linked $target to $file"
}

keep() {
  user=$1
  shift
  unit=boomdrive.service

  if [ "$(id -un)" != "$user" ]; then
    return
  fi
  pid=$(systemctl --user show --property=MainPID --value "$unit")
  if [ -z "$pid" ] || [ "$pid" = 0 ]; then
    systemctl --user --no-block start "$unit"
    echo "boomdrive: started $unit"
    return
  fi
  running=$(ps -ww -o args= -p "$pid")
  if [ "$running" != "$*" ]; then
    systemctl --user --no-block restart "$unit"
    echo "boomdrive: restarted $unit, which ran $running"
  fi
}

# The names of the environment's HERDR_* variables, one a line. A function
# of its own, called inside `$( )`, since no `case` may stand inside one:
# macOS's /bin/sh, bash 3.2.57, reads a pattern's bare `)` there as the
# substitution's end and so cannot read the file, and shfmt drops the
# POSIX leading `(` that would spare it.
herdrs() {
  env | while IFS= read -r line; do
    case $line in HERDR_*=*) echo "${line%%=*}" ;; esac
  done
}

plugins() {
  herdr=$1
  shift
  for plugin in "$@"; do
    if ! words=$(
      names=$(herdrs)
      for name in $names; do
        unset "$name"
      done
      "$herdr" plugin link "$plugin" </dev/null 2>&1
    ); then
      words=$(printf '%s\n' "$words" | tr '\n' ' ')
      echo "boomdrive: herdr did not link $plugin: ${words% }" >&2
    fi
  done
}

# One file of Pending You's folder: <from> written to <to> with <mode>,
# its folders under <folder> made 0755 where they are missing.
place() {
  from=$1
  to=$2
  mode=$3
  rest=${to#"$folder"/}
  under=$folder
  while case $rest in */*) true ;; *) false ;; esac do
    under=$under/${rest%%/*}
    rest=${rest#*/}
    if [ ! -d "$under" ]; then
      mkdir -m 0755 "$under" || return 1
    fi
  done
  if [ -d "$to" ] && [ ! -L "$to" ]; then
    echo "a folder is there"
    return 1
  fi
  temporary=$(mktemp "$to.XXXXXX") || return 1
  if ! cat "$from" >"$temporary" || ! chmod "$mode" "$temporary"; then
    rm -f "$temporary"
    return 1
  fi
  if [ -L "$to" ]; then
    rm -f "$to" || {
      rm -f "$temporary"
      return 1
    }
  fi
  mv -f "$temporary" "$to" || {
    rm -f "$temporary"
    return 1
  }
}

pendingyou() {
  source=$1
  folder=$2
  if [ ! -d "$folder" ]; then
    if ! words=$(mkdir -p "$(dirname "$folder")" 2>&1 && mkdir -m 0700 "$folder" 2>&1); then
      words=$(printf '%s\n' "$words" | tr '\n' ' ')
      echo "boomdrive: did not write Pending You's folder $folder: ${words% }" >&2
      return
    fi
  fi
  all=$(cd "$source" && find . -type f | sed 's|^\./||' | sort)
  paths=$(
    for first in mod/hooks/wake.js mod/hooks/register.js mod/hooks/hooks.json; do
      printf '%s\n' "$all" | grep -Fx -- "$first" || true
    done
    printf '%s\n' "$all" |
      grep -Fxv -e mod/hooks/wake.js -e mod/hooks/register.js -e mod/hooks/hooks.json || true
  )
  while IFS= read -r path; do
    [ -n "$path" ] || continue
    from=$source/$path
    to=$folder/$path
    if [ -x "$from" ]; then
      mode=0755
    else
      mode=0644
    fi
    if [ -f "$to" ] && [ ! -L "$to" ] && cmp -s "$from" "$to" &&
      [ -n "$(find "$to" -prune -perm "$mode")" ]; then
      continue
    fi
    if ! words=$(place "$from" "$to" "$mode" 2>&1); then
      words=$(printf '%s\n' "$words" | tr '\n' ' ')
      echo "boomdrive: did not write $to: ${words% }" >&2
      return
    fi
    echo "boomdrive: wrote $to"
  done <<PATHS
$paths
PATHS
}

# Claude Code's command `claude $@` in the home $home, its default where
# $home is empty: its answer in `answer`. Where Claude Code refuses or
# fails it, one line names $where, the command and the answer's `message`,
# or else Claude Code's words, and it returns 1.
ask() {
  if answer=$(
    if [ -n "$home" ]; then
      CLAUDE_CONFIG_DIR=$home
      export CLAUDE_CONFIG_DIR
    else
      unset CLAUDE_CONFIG_DIR
    fi
    "$claude" "$@" </dev/null 2>"$errors"
  ); then
    return 0
  fi
  message=$(printf '%s\n' "$answer" | "$jq" -r '.message // empty' 2>/dev/null) || message=
  if [ -z "$message" ]; then
    message=$(oneline "$(cat "$errors" 2>/dev/null)")
  fi
  if [ -z "$message" ]; then
    message=$(oneline "$answer")
  fi
  message=$(oneline "$message")
  echo "boomdrive: Claude Code refused \`claude $*\` in $where: $message" >&2
  return 1
}

# The second word of the line of `$2` whose first word is `$1`.
versionOf() {
  printf '%s\n' "$2" | awk -v name="$1" '$1 == name { print $2; exit }'
}

# Whether the home's list, $held, holds `$1`.
holding() {
  printf '%s\n' "$held" | awk -v name="$1" '$1 == name { found = 1 } END { exit !found }'
}

# The set's plugins in the home `$1`, Claude Code's default where empty.
inHome() {
  home=$1
  if [ -n "$home" ]; then
    where=$home
  else
    where="Claude Code's default home"
  fi
  ask plugin marketplace add "$marketplace" --json || true
  held=
  if ask plugin list --json; then
    held=$(printf '%s\n' "$answer" | "$jq" -r '
      .[]? | select((.id // "") | endswith("@boomdrive"))
      | "\(.id | rtrimstr("@boomdrive")) \(.version // "")"
    ' 2>/dev/null) || held=
  fi
  for plugin in $installing; do
    if ! holding "$plugin"; then
      ask plugin install "$plugin@boomdrive" --json || true
    elif [ "$(versionOf "$plugin" "$held")" != "$(versionOf "$plugin" "$offered")" ]; then
      ask plugin update "$plugin@boomdrive" --scope user --json || true
    fi
  done
  for plugin in $removing; do
    if holding "$plugin"; then
      ask plugin uninstall "$plugin@boomdrive" --json || true
    fi
  done
  if [ "$server" != - ] &&
    ! "$jq" -e '.mcpServers.pendingyou' "${home:-$HOME}/.claude.json" >/dev/null 2>&1; then
    ask mcp add-json --scope user pendingyou "$(cat "$server")" || true
  fi
}

sessions() {
  flume=$1
  jq=$2
  marketplace=$3
  server=$4
  shift 4
  installing=
  removing=
  side=installing
  for plugin in "$@"; do
    if [ "$plugin" = -- ]; then
      side=removing
    elif [ "$side" = installing ]; then
      installing="$installing $plugin"
    else
      removing="$removing $plugin"
    fi
  done

  if ! claude=$(command -v claude); then
    echo "boomdrive: found no claude on PATH, so no Claude Code home has the set's plugins" >&2
    return
  fi
  errors=$(mktemp "${TMPDIR:-/tmp}/boomdrive.XXXXXXXX") || errors=/dev/null
  if [ "$errors" != /dev/null ]; then
    trap 'rm -f "$errors"' EXIT
  fi
  offered=$("$jq" -r '.plugins[]? | "\(.name) \(.version // "")"' \
    "$marketplace/.claude-plugin/marketplace.json" 2>/dev/null) || offered=

  if status=$("$flume" --json status </dev/null 2>"$errors") &&
    homes=$(printf '%s\n' "$status" | "$jq" -r '
      .status.host.pools[]?.accounts[]?
      | select(.program == "claude") | .home
      | select(type == "string" and . != "")
    ' 2>/dev/null); then
    homes=$(printf '%s\n' "$homes" | awk 'NF && !seen[$0]++')
  else
    words=$(oneline "$(cat "$errors" 2>/dev/null)")
    if [ -z "$words" ]; then
      words=$(oneline "$status")
    fi
    echo "boomdrive: could not read Flume's status, so the set's Claude Code plugins go into Claude Code's default home alone: $words" >&2
    homes=
  fi

  inHome ""
  while IFS= read -r each; do
    [ -n "$each" ] || continue
    inHome "$each"
  done <<HOMES
$homes
HOMES
}

# <source> written to <path>, 0644, its folders made 0755 where they are
# missing.
document() {
  source=$1
  path=$2
  under=$(dirname "$path")
  if [ ! -d "$under" ]; then
    (umask 022 && mkdir -p "$under") || return 1
  fi
  if [ -d "$path" ] && [ ! -L "$path" ]; then
    echo "a folder is there"
    return 1
  fi
  temporary=$(mktemp "$path.XXXXXX") || return 1
  if ! cat "$source" >"$temporary" || ! chmod 0644 "$temporary"; then
    rm -f "$temporary"
    return 1
  fi
  if [ -L "$path" ]; then
    rm -f "$path" || {
      rm -f "$temporary"
      return 1
    }
  fi
  mv -f "$temporary" "$path" || {
    rm -f "$temporary"
    return 1
  }
}

managed() {
  path=$1
  if [ $# -eq 1 ]; then
    if [ ! -e "$path" ] && [ ! -L "$path" ]; then
      return
    fi
    if [ -d "$path" ] && [ ! -L "$path" ]; then
      echo "boomdrive: did not remove $path: a folder is there" >&2
      return
    fi
    if ! words=$(rm -f "$path" 2>&1); then
      words=$(printf '%s\n' "$words" | tr '\n' ' ')
      echo "boomdrive: did not remove $path: ${words% }" >&2
      return
    fi
    echo "boomdrive: removed $path"
    return
  fi
  source=$2
  if [ -f "$path" ] && [ ! -L "$path" ] && cmp -s "$source" "$path" &&
    [ -n "$(find "$path" -prune -perm 0644)" ]; then
    return
  fi
  if ! words=$(document "$source" "$path" 2>&1); then
    words=$(printf '%s\n' "$words" | tr '\n' ' ')
    echo "boomdrive: did not write $path: ${words% }" >&2
    return
  fi
  echo "boomdrive: wrote $path"
}

# The distinct SHA-1s of the identities for code signing named Boomdrive
# runner in the keychain `$1`, in `shas`, and how many there are, in
# `count` (design Decision 2).
lookup() {
  if ! listed=$(security find-identity -p codesigning "$1" 2>&1); then
    echo "boomdrive: could not read the identities in $1: $(oneline "$listed")" >&2
    exit 1
  fi
  shas=$(printf '%s\n' "$listed" | awk -v name='"Boomdrive runner"' 'index($0, name) { print $2 }' | sort -u)
  count=$(printf '%s\n' "$shas" | awk 'NF { n++ } END { print n + 0 }')
}

# A key and a self-signed certificate for code signing named Boomdrive
# runner, made in a folder of its own that only this user reads and that
# goes whatever happens, then imported into the keychain `$1`, the key
# non-extractable and usable by codesign alone (design Decision 2).
identity() (
  keychain=$1
  umask 077
  # Named under TMPDIR, which macOS's mktemp -d alone does not read.
  made=$(mktemp -d "${TMPDIR:-/tmp}/boomdrive.XXXXXXXX")
  trap 'rm -rf "$made"' EXIT
  trap 'exit 1' HUP INT TERM
  cat >"$made/openssl.cnf" <<'CONFIG'
[req]
prompt = no
distinguished_name = name
x509_extensions = extensions

[name]
CN = Boomdrive runner

[extensions]
basicConstraints = critical,CA:false
keyUsage = critical,digitalSignature
extendedKeyUsage = critical,codeSigning
CONFIG
  if ! words=$(openssl req -x509 -newkey rsa:2048 -nodes -days 7300 \
    -config "$made/openssl.cnf" -keyout "$made/key.pem" -out "$made/cert.pem" 2>&1) ||
    ! words=$(security import "$made/key.pem" -k "$keychain" -x -T /usr/bin/codesign 2>&1) ||
    ! words=$(security import "$made/cert.pem" -k "$keychain" 2>&1); then
    echo "boomdrive: could not make the code-signing identity in $keychain: $(oneline "$words")" >&2
    exit 1
  fi
)

sign() {
  keychain=$1
  folder=$2
  program=$3

  # The folder: made where it is not there, and trusted only where it is a
  # folder the step's own user owns and no other user can write (design
  # Decision 3). It changes no owner and no mode.
  if [ ! -e "$folder" ] && [ ! -L "$folder" ]; then
    if ! words=$(mkdir -m 0755 "$folder" 2>&1); then
      echo "boomdrive: could not sign $program into $folder: $(oneline "$words")" >&2
      exit 1
    fi
  fi
  listing=$(ls -ldn "$folder")
  mode=$(printf '%s\n' "$listing" | awk '{ print $1 }')
  owner=$(printf '%s\n' "$listing" | awk '{ print $3 }')
  case $mode in
  d????-??-*) shut=yes ;;
  *) shut=no ;;
  esac
  if [ "$shut" = no ] || [ "$owner" != "$(id -u)" ]; then
    echo "boomdrive: $folder is writable by a user other than $(id -un): signed nothing" >&2
    exit 1
  fi

  # The identity: the one the keychain holds, or one made where it holds
  # none. A listing it cannot read makes none, so a second is never made
  # beside one it could not see.
  lookup "$keychain"
  if [ "$count" -eq 0 ]; then
    identity "$keychain"
    lookup "$keychain"
    if [ "$count" -eq 0 ]; then
      echo "boomdrive: could not make the code-signing identity in $keychain: it lists none named Boomdrive runner after the import" >&2
      exit 1
    fi
    if [ "$count" -eq 1 ]; then
      echo "boomdrive: made the code-signing identity Boomdrive runner, $shas, in $keychain"
    fi
  fi
  if [ "$count" -gt 1 ]; then
    each=$(printf '%s\n' "$shas" | awk 'NR > 1 { printf ", " } { printf "%s", $0 }')
    echo "boomdrive: $keychain holds $count identities named Boomdrive runner, $each: signed nothing; remove all but one" >&2
    exit 1
  fi
  sha=$shas

  # The copy: made only where none is there, it was made from another
  # program, or it is not signed with the identity (design Decision 4).
  copy=$folder/boomdrive
  source=$folder/boomdrive.source
  if [ -e "$copy" ] && [ "$(cat "$source" 2>/dev/null)" = "$program" ] &&
    codesign --verify -R "=certificate leaf = H\"$sha\"" "$copy" >/dev/null 2>&1; then
    return
  fi
  # Made and signed beside the copy, which stays as it was where either
  # fails.
  if ! words=$({ cp "$program" "$copy.new" && chmod 0755 "$copy.new"; } 2>&1) ||
    ! words=$(codesign --force --sign "$sha" --keychain "$keychain" \
      --identifier boomdrive --timestamp=none "$copy.new" 2>&1); then
    rm -f "$copy.new"
    echo "boomdrive: could not sign $program into $folder: $(oneline "$words")" >&2
    exit 1
  fi
  # Put in place with its source removed first and written last, so the
  # source never names a program the copy was not made from.
  if ! words=$({
    rm -f "$source" &&
      mv -f "$copy.new" "$copy" &&
      printf '%s\n' "$program" >"$source.new" &&
      mv -f "$source.new" "$source"
  } 2>&1); then
    rm -f "$copy.new" "$source.new"
    echo "boomdrive: could not sign $program into $folder: $(oneline "$words")" >&2
    exit 1
  fi
  echo "boomdrive: signed $copy, made from $program, with Boomdrive runner, $sha"
}

case ${1:-} in
link)
  [ $# -eq 2 ] || usage
  link "$2"
  ;;
keep)
  [ $# -ge 3 ] || usage
  shift
  keep "$@"
  ;;
plugins)
  [ $# -ge 3 ] || usage
  shift
  plugins "$@"
  ;;
pendingyou)
  [ $# -eq 3 ] || usage
  pendingyou "$2" "$3"
  ;;
claude)
  [ $# -ge 6 ] || usage
  shift
  sessions "$@"
  ;;
managed)
  [ $# -eq 2 ] || [ $# -eq 3 ] || usage
  shift
  managed "$@"
  ;;
sign)
  [ $# -eq 4 ] || usage
  shift
  sign "$@"
  ;;
*) usage ;;
esac
