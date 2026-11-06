# What every case reads: flake.nix imported as an attribute set, and its
# outputs called with the lock's inputs and `self` as this tree, as
# flake-compat calls them (design Decision 14).
{ paths }:
let
  inherit ((import ../inputs.nix { inherit paths; })) inputs;
  flake = import ../../flake.nix;
  lock = builtins.fromJSON (builtins.readFile ../../flake.lock);
  outputs = flake.outputs (inputs // { inherit self; });
  # The tree by its path, never copied: the checks write nothing.
  self = outputs // {
    outPath = toString ../..;
    inherit inputs outputs;
    _type = "flake";
  };
in
{
  inherit
    flake
    inputs
    outputs
    self
    ;
  inherit (inputs.nixpkgs) lib;

  # flake.lock, which the inputs are read from.
  inherit lock;

  # The version of a tool of the set, its tag in the lock without the v.
  lockedVersion =
    name:
    inputs.nixpkgs.lib.removePrefix "v"
      lock.nodes.${lock.nodes.${lock.root}.inputs.${name}}.original.ref;

  # The systems the flake gives: sluice's flake's, less x86_64-darwin,
  # which nixpkgs 26.11 refuses.
  systems = [
    "aarch64-darwin"
    "aarch64-linux"
    "x86_64-linux"
  ];

  # A failure where `holds` is false, as a list of what fails.
  expect = holds: what: if holds then [ ] else [ what ];
}
