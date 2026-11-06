# The runner, built from this tree as sluice's flake builds sluice (design
# Decision 3): Rust at the version .prototools names, from rust-overlay;
# the crates by Cargo.lock's own checksums, and its git crates by the
# hashes git-crates.nix holds beside their revisions. Its source is
# Cargo.toml, Cargo.lock and crates/ alone, so no other file of the tree
# changes its path. The gate runs its tests, so the build runs none.
{
  lib,
  makeRustPlatform,
  rust-bin,
  platforms,
}:
let
  rust = rust-bin.stable.${(builtins.fromTOML (builtins.readFile ../../.prototools)).rust}.minimal;
  rustPlatform = makeRustPlatform {
    cargo = rust;
    rustc = rust;
  };
in
rustPlatform.buildRustPackage {
  pname = "boomdrive";
  inherit ((builtins.fromTOML (builtins.readFile ../../Cargo.toml)).workspace.package) version;
  src = lib.fileset.toSource {
    root = ../..;
    fileset = lib.fileset.unions [
      ../../Cargo.toml
      ../../Cargo.lock
      ../../crates
    ];
  };
  cargoLock = {
    lockFile = ../../Cargo.lock;
    outputHashes = lib.mapAttrs (_: crate: crate.hash) (import ./git-crates.nix);
  };
  cargoBuildFlags = [
    "-p"
    "boomdrive"
  ];
  doCheck = false;
  meta = {
    description = "The runner on each host of the Boomdrive family";
    homepage = "https://github.com/agentplot/boomdrive";
    mainProgram = "boomdrive";
    inherit platforms;
  };
}
