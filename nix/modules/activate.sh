#!/bin/sh
# The modules' activation, run as the service's user (design Decisions 9
# and 10):
#
#   activate.sh link <file>
#   activate.sh keep <user> <program> <argument>...
#   activate.sh plugins <herdr> <plugin>...
#   activate.sh pendingyou <source> <folder>
#   activate.sh managed <path> [<source>]
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
set -eu

usage() {
  echo "usage: activate.sh link <file>" >&2
  echo "       activate.sh keep <user> <program> <argument>..." >&2
  echo "       activate.sh plugins <herdr> <plugin>..." >&2
  echo "       activate.sh pendingyou <source> <folder>" >&2
  echo "       activate.sh managed <path> [<source>]" >&2
  exit 2
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

plugins() {
  herdr=$1
  shift
  for plugin in "$@"; do
    if ! words=$(
      names=$(env | while IFS= read -r line; do
        case $line in HERDR_*=*) echo "${line%%=*}" ;; esac
      done)
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
managed)
  [ $# -eq 2 ] || [ $# -eq 3 ] || usage
  shift
  managed "$@"
  ;;
*) usage ;;
esac
