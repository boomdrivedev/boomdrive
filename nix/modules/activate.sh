#!/bin/sh
# The modules' activation, run as the service's user (design Decision 9):
#
#   activate.sh link <file>
#
# links ~/.config/boomdrive/config.toml to the module's file, so
# `boomdrive enroll` and `boomdrive run --once` run by hand read what the
# service reads. A link there is replaced. A file or folder there that is
# no link is moved once, to config.toml.before-nix, and both paths are
# named. Where config.toml.before-nix is there already, it changes neither,
# names both and fails: the service reads its own --config either way. It
# says a line only where it moved or linked something.
set -eu

usage() {
  echo "usage: activate.sh link <file>" >&2
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

case ${1:-} in
link)
  [ $# -eq 2 ] || usage
  link "$2"
  ;;
*) usage ;;
esac
