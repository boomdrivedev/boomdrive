#!/bin/sh
# Boomdrive's install (the install spec; record 0012 as amended on
# 2026-11-27). The workflow release attaches this file to each release,
# its version written in by the release commit (set-version.sh), and
# GitHub serves it at
#
#   https://github.com/boomdrivedev/boomdrive/releases/latest/download/install.sh
#   https://github.com/boomdrivedev/boomdrive/releases/download/v<version>/install.sh
#
# It adds the set at the release's tag, from Boomdrive's flake through
# the family's cache, to the person's Nix profile, and installs Nix first
# where the host has none (core.37, core.47). Then it runs the enroll
# command it was given, which reads the token on the same standard input:
#
#   echo <token> | sh -c "$(curl -fsSL <url>)" install [--yes] \
#     --control-plane <url> --drive <drive> --repository <owner/name>...
#
# It asks before it installs Nix or sets Nix to trust the cache, and
# --yes is the answer yes. Only the enroll command reads its standard
# input, so the token is in the arguments of no process (core.31). It
# runs on macOS on Apple silicon, the one Mac the flake gives.
set -eu

# The release's version, which set-version.sh writes; main gives 0.0.0.
version=0.36.10

# Nix's own installer, at the release this file pins and its digest,
# which Nix publishes beside it as install.sha256.
nix_version=2.36.0
nix_installer=https://releases.nixos.org/nix/nix-$nix_version/install
nix_installer_sha256=8388dfd4d3c3a0c2dc79f764d453eafcc244255f47c93c1958f4775af22f0353

# The family's cache and its key, as flake.nix's nixConfig names them.
cache=https://boomdrive.cachix.org
key=boomdrive.cachix.org-1:vHLgmasPg2jKqkvPXWOOrYzspoGL2xxsW4fAuHBtiag=

flake=github:boomdrivedev/boomdrive/v$version

say() {
  printf 'install: %s\n' "$*" >&2
}

# Stops with the exit code $1 and the words after it. Where the install
# was given an enroll command, it says that the token is not spent.
stop() {
  code=$1
  shift
  if [ "$enrolling" = yes ]; then
    say "$* The token is not spent."
  else
    say "$*"
  fi
  exit "$code"
}

usage() {
  say "$1"
  echo "usage: install.sh [--yes] [--control-plane <url> --drive <drive> --repository <owner/name>...]" >&2
  exit 2
}

have() {
  command -v "$1" >/dev/null 2>&1
}

nixx() {
  nix --extra-experimental-features 'nix-command flakes' "$@"
}

# Says $1, then gives yes where --yes was given, or where the person
# answers y on the terminal that is the standard input.
ask() {
  say "$1"
  [ "$yes" = yes ] && return 0
  [ -t 0 ] || return 1
  printf 'install: go on? [y/N] ' >&2
  read -r answer || answer=
  case $answer in
  y | Y | yes | Yes) return 0 ;;
  esac
  return 1
}

# The flags. The enroll command's flags stay in "$@", in their order.
yes=no
enrolling=no
plane=
drive=
repositories=0
left=$#
while [ "$left" -gt 0 ]; do
  flag=$1
  shift
  left=$((left - 1))
  case $flag in
  --yes) yes=yes ;;
  --control-plane | --drive | --repository)
    [ "$left" -gt 0 ] || usage "$flag takes a value"
    value=$1
    shift
    left=$((left - 1))
    case $value in
    '' | -*) usage "$flag takes a value" ;;
    esac
    case $flag in
    --control-plane)
      [ -z "$plane" ] || usage "--control-plane is given twice"
      plane=$value
      ;;
    --drive)
      [ -z "$drive" ] || usage "--drive is given twice"
      drive=$value
      ;;
    --repository) repositories=$((repositories + 1)) ;;
    esac
    set -- "$@" "$flag" "$value"
    ;;
  *) usage "$flag is no flag of the install" ;;
  esac
done
if [ -n "$plane" ] || [ -n "$drive" ] || [ "$repositories" -gt 0 ]; then
  [ -n "$plane" ] && [ -n "$drive" ] && [ "$repositories" -gt 0 ] ||
    usage "the enroll command takes --control-plane, --drive and one --repository or more"
  enrolling=yes
fi

case "$(uname -s) $(uname -m)" in
"Darwin arm64") ;;
"Darwin "*)
  stop 2 "This Mac is not Apple silicon, and the flake gives the set for Apple silicon alone."
  ;;
"Linux "*)
  stop 2 "On Linux, the runner runs from the flake's NixOS module, services.boomdrive; the quick start's NixOS steps set it up: https://boomdrivedev.github.io/boomdrive/start/."
  ;;
*)
  stop 2 "$(uname -s) is not macOS, and the install is for macOS alone."
  ;;
esac

# The runner's file, which Nix links into the store where the host's
# configuration sets up the runner.
file=${XDG_CONFIG_HOME:-$HOME/.config}/boomdrive/config.toml
if [ -L "$file" ]; then
  case $(readlink "$file") in
  /nix/store/*)
    if [ "$enrolling" = yes ]; then
      stop 2 "$file links into the Nix store, so the host's Nix configuration sets up the runner, and the install changes nothing. Give the token to the enroll command alone: echo <token> | boomdrive enroll $*. It names what to add to the configuration."
    fi
    stop 2 "$file links into the Nix store, so the host's Nix configuration sets up the runner, and the install changes nothing."
    ;;
  esac
fi

# A runner on the PATH already: this release's goes on to enroll, and any
# other stops, since it came from elsewhere.
installed=no
if have boomdrive; then
  said=$(boomdrive --version </dev/null 2>/dev/null || true)
  if [ "$said" = "boomdrive $version" ]; then
    installed=yes
    say "$(command -v boomdrive) is boomdrive $version already."
  else
    stop 2 "$(command -v boomdrive) is ${said:-a boomdrive that gives no version}, and this install is of $version. Upgrade it or remove it where it came from, then run the same command again."
  fi
fi

if [ "$installed" = no ] && ! have nix; then
  ask "Nix is not on this host. The install will install Nix $nix_version with Nix's own installer from $nix_installer, as root, and set Nix to use Boomdrive's cache, $cache, whose key is $key." ||
    stop 2 "Nothing is installed. Run the same command again with --yes to install Nix and use the cache."
  work=$(mktemp -d "${TMPDIR:-/tmp}/boomdrive-install.XXXXXX")
  trap 'rm -rf "$work"' EXIT
  curl -fsSL --proto '=https' --tlsv1.2 -o "$work/install" "$nix_installer" </dev/null ||
    stop 1 "Nix's installer was not fetched from $nix_installer."
  if have shasum; then
    digest=$(shasum -a 256 "$work/install" | awk '{ print $1 }')
  else
    digest=$(sha256sum "$work/install" | awk '{ print $1 }')
  fi
  [ "$digest" = "$nix_installer_sha256" ] ||
    stop 1 "Nix's installer from $nix_installer gives the digest $digest, not $nix_installer_sha256, and is not run."
  {
    echo "experimental-features = nix-command flakes"
    echo "extra-substituters = $cache"
    echo "extra-trusted-public-keys = $key"
  } >"$work/nix.conf"
  sh "$work/install" --daemon --yes --nix-extra-conf-file "$work/nix.conf" </dev/null ||
    stop 1 "Nix's installer failed."
  rm -rf "$work"
  trap - EXIT
  PATH=$PATH:/nix/var/nix/profiles/default/bin
  if [ -z "${NIX_SSL_CERT_FILE:-}" ] && [ -e /nix/var/nix/profiles/default/etc/ssl/certs/ca-bundle.crt ]; then
    NIX_SSL_CERT_FILE=/nix/var/nix/profiles/default/etc/ssl/certs/ca-bundle.crt
    export NIX_SSL_CERT_FILE
  fi
  have nix || stop 1 "Nix's installer ended, and no nix is at /nix/var/nix/profiles/default/bin."
elif [ "$installed" = no ]; then
  substituters=$(nixx config show substituters </dev/null 2>/dev/null || true)
  keys=$(nixx config show trusted-public-keys </dev/null 2>/dev/null || true)
  case " $substituters " in
  *" $cache "* | *" $cache/ "*) used=yes ;;
  *) used=no ;;
  esac
  case " $keys " in
  *" $key "*) ;;
  *) used=no ;;
  esac
  if [ "$used" = no ]; then
    if have determinate-nixd; then
      conf=/etc/nix/nix.custom.conf
      daemon=systems.determinate.nix-daemon
    else
      conf=/etc/nix/nix.conf
      daemon=org.nixos.nix-daemon
    fi
    lines="extra-substituters = $cache
extra-trusted-public-keys = $key"
    ask "Nix on this host does not use Boomdrive's cache, $cache. The install will add these lines to $conf, as root, and restart Nix's daemon:
$lines" ||
      stop 2 "Nothing is installed. Run the same command again with --yes, or add the lines to the host's Nix configuration yourself."
    printf '%s\n' "$lines" | sudo tee -a "$conf" >/dev/null ||
      stop 2 "$conf cannot be written. Add the lines where the host's Nix configuration writes it, then run the same command again."
    sudo launchctl kickstart -k "system/$daemon" </dev/null ||
      stop 1 "Nix's daemon, $daemon, did not restart."
  fi
fi

if [ "$installed" = no ]; then
  say "Adding the set at v$version to your Nix profile: the runner, flume, sluice, herdr with its plugins, and git."
  nixx profile add --inputs-from "$flake" "$flake#boomdrive" "$flake#flume" "$flake#sluice" \
    "$flake#herdr" "$flake#herdr-plugins" nixpkgs#git </dev/null ||
    stop 1 "nix profile add failed."
  PATH=$PATH:$HOME/.nix-profile/bin:${XDG_STATE_HOME:-$HOME/.local/state}/nix/profile/bin
  have boomdrive || stop 1 "nix profile add ended, and no boomdrive is in your Nix profile."
fi

if [ "$enrolling" = yes ]; then
  say "Enrolling the host: boomdrive enroll $*"
  exec boomdrive enroll "$@"
fi
say "The set at v$version is installed. Enroll the host with the command its control plane gives, or run boomdrive setup for a drive of crew's."
