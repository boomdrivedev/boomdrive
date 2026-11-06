#!/bin/sh
# Every tool the gate runs is the version the tree pins, on every host
# (rule core.8): rustc, cargo, clippy and rustfmt the build .prototools
# pins, which rust-toolchain.toml names too, and git, recutils, OpenSpec
# and the nix project's nix, nixfmt, statix, deadnix and actionlint the
# releases devenv.nix pins. A tool found outside its pin, a
# runner's rustup default say, fails here rather than holding the tree to
# another version's rules.
set -eu

fail() {
  echo "check-toolchain: $*" >&2
  exit 1
}

pin=$(sed -n 's/^rust *= *"\(.*\)"$/\1/p' .prototools)
[ -n "$pin" ] || fail ".prototools pins no rust"
channel=$(sed -n 's/^channel *= *"\(.*\)"$/\1/p' rust-toolchain.toml)
[ "$channel" = "$pin" ] || fail "rust-toolchain.toml names ${channel:-no channel}, .prototools pins $pin"

rustc=$(rustc -vV)
[ "$(printf '%s\n' "$rustc" | sed -n 's/^release: //p')" = "$pin" ] ||
  fail "rustc is not $pin: $(rustc --version)"
[ "$(cargo -vV | sed -n 's/^release: //p')" = "$pin" ] ||
  fail "cargo is not $pin: $(cargo --version)"

# clippy and rustfmt carry the commit of the rustc they ship with.
commit=$(printf '%s\n' "$rustc" | sed -n 's/^commit-hash: \(.\{9\}\).*/\1/p')
same() {
  case $2 in
  *"($commit"*) ;;
  *) fail "$1 is not $pin's: $2" ;;
  esac
}
same clippy "$(cargo clippy --version)"
same rustfmt "$(rustfmt --version)"
echo "rustc, cargo, clippy and rustfmt are $pin"

# The releases devenv.nix pins, each by the version its derivation names.
pinned() {
  version=$(sed -n "s/^ *$1 = \"\(.*\)\";$/\1/p" devenv.nix)
  [ -n "$version" ] || fail "devenv.nix pins no $1"
  printf '%s\n' "$version"
}
release() {
  name=$1
  version=$2
  shift 2
  said=$("$@" 2>&1) || fail "$name did not answer $*: $said"
  case " $said " in
  *[!0-9.]"$version"[!0-9.]*) echo "$name is $version" ;;
  *) fail "$name is not $version: $said" ;;
  esac
}
release git "$(pinned gitVersion)" git --version
release recutils "$(pinned recutilsVersion)" recfix --version
release openspec "$(pinned openspecVersion)" openspec --version
release nix "$(pinned nixVersion)" nix --version
release nixfmt "$(pinned nixfmtVersion)" nixfmt --version
release deadnix "$(pinned deadnixVersion)" deadnix --version
release actionlint "$(pinned actionlintVersion)" actionlint --version

# statix answers no --version: its version is the name of the store path
# its program is in.
version=$(pinned statixVersion)
statix=$(command -v statix) || fail "statix is not on PATH"
case $(readlink -f "$statix") in
/nix/store/*-statix-"$version"/bin/statix) echo "statix is $version" ;;
*) fail "statix is not $version: $(readlink -f "$statix")" ;;
esac
