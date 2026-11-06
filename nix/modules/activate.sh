#!/bin/sh
# The modules' activation, run as the service's user (design Decisions 9
# and 10):
#
#   activate.sh link <file>
#   activate.sh keep <user> <program> <argument>...
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
set -eu

usage() {
  echo "usage: activate.sh link <file>" >&2
  echo "       activate.sh keep <user> <program> <argument>..." >&2
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
*) usage ;;
esac
