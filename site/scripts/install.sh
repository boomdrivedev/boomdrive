#!/bin/sh
# site:install (design Decision 4): the packages bun.lock locks, installed
# with no network from bun's cache as flume's site/packages.nix builds it,
# which the devenv shell fetches outside the gate's sandbox and gives as
# BOOMDRIVE_SITE_PACKAGES (flume's record 0059). The store keeps it
# read-only, and bun copies a folder's mode with it, so a package that
# holds a nested node_modules could not take one: bun reads a writable
# copy, in a scratch folder removed after.
set -eu

if [ -z "${BOOMDRIVE_SITE_PACKAGES:-}" ]; then
  echo "site: install: BOOMDRIVE_SITE_PACKAGES is not set: the devenv shell sets it" >&2
  exit 1
fi

scratch=$(mktemp -d "${TMPDIR:-/tmp}/site-install.XXXXXX")
trap 'chmod -R u+w "$scratch" && rm -rf "$scratch"' EXIT
cp -R "$BOOMDRIVE_SITE_PACKAGES" "$scratch/cache"
chmod -R u+w "$scratch/cache"
bun install --frozen-lockfile --offline --cache-dir "$scratch/cache"
