#!/bin/sh
# The modules' activation, run as the service's user (design Decisions 9
# and 10):
#
#   activate.sh link <file>
#   activate.sh keep <user> <program> <argument>...
#   activate.sh plugins <herdr> <plugin>...
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
# sign is nix-darwin's step, run as root before nix-darwin loads its
# agents (record 0043; design Decisions 1 to 4 of the change
# a-new-build-keeps-the-runners-macos-access). It makes <folder> with mode
# 0755 where it is not there, and goes on only where <folder> is owned by
# the user it runs as and no other user can write it: otherwise it makes
# no identity, signs nothing, writes nothing there and fails. Each line of
# what it made goes to standard output, and a refusal to standard error.
set -eu

usage() {
  echo "usage: activate.sh link <file>" >&2
  echo "       activate.sh keep <user> <program> <argument>..." >&2
  echo "       activate.sh plugins <herdr> <plugin>..." >&2
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
sign)
  [ $# -eq 4 ] || usage
  shift
  sign "$@"
  ;;
*) usage ;;
esac
