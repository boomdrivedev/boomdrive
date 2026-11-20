# millpond as the set gives it (record 0033): millpond's own flake's
# package at the set's tag, unchanged but for how its git crates are
# fetched: by the hashes millpond-git-crates.nix holds, where millpond's
# flake fetches them at evaluation, which the gate's evaluation against
# no store cannot (core.44, core.45).
{
  lib,
  rustPlatform,
  package,
  src,
}:
package.overrideAttrs (_: {
  cargoDeps = rustPlatform.importCargoLock {
    lockFile = "${src}/Cargo.lock";
    outputHashes = lib.mapAttrs (_: crate: crate.hash) (import ./millpond-git-crates.nix);
  };
})
