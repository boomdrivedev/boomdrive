# Flume at its tag, built as sluice's flake builds sluice, until Flume
# publishes a flake of its own (design Decision 3): Rust at the version
# its own .prototools names, its crates by its Cargo.lock's checksums and
# its git crates by flume-git-crates.nix's hashes, and the version its
# tagged tree's Cargo.toml carries, which Flume's release commit writes.
{
  lib,
  makeRustPlatform,
  rust-bin,
  src,
  platforms,
}:
let
  read = file: builtins.fromTOML (builtins.readFile "${src}/${file}");
  rust = rust-bin.stable.${(read ".prototools").rust}.minimal;
  rustPlatform = makeRustPlatform {
    cargo = rust;
    rustc = rust;
  };
in
rustPlatform.buildRustPackage {
  pname = "flume";
  inherit ((read "Cargo.toml").workspace.package) version;
  inherit src;
  cargoLock = {
    lockFile = "${src}/Cargo.lock";
    outputHashes = lib.mapAttrs (_: crate: crate.hash) (import ./flume-git-crates.nix);
  };
  cargoBuildFlags = [
    "-p"
    "flume"
    "--bin"
    "flume"
  ];
  # Flume's tests need git and herdr; its own workflow runs them.
  doCheck = false;
  meta = {
    description = "Places and their agents, for the Boomdrive family";
    homepage = "https://github.com/agentplot/flume";
    license = lib.licenses.mit;
    mainProgram = "flume";
    inherit platforms;
  };
}
