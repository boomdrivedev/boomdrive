# The family's host tools, pinned together and built from source: the
# runner at this tree, and Flume and sluice each at a release tag (record
# 0020, core.44). A host's nix substitutes their binaries from the family's
# cache where it trusts it, and builds the same paths where it does not
# (core.47). The development shell is devenv's (devenv.nix); everything
# else of the flake's is under nix/, the moon project `nix`.
{
  description = "Boomdrive: the runner, Flume and sluice, pinned together";

  # The family's cache, for a command run on this flake. A host's
  # configuration names the same two lines in its own nix settings, since
  # nix reads no input's nixConfig (design Decision 4).
  nixConfig = {
    extra-substituters = [ "https://boomdrive.cachix.org" ];
    extra-trusted-public-keys = [
      "boomdrive.cachix.org-1:vHLgmasPg2jKqkvPXWOOrYzspoGL2xxsW4fAuHBtiag="
    ];
  };

  # A tool's tag moves only with the lock, in a change the gate passes
  # (core.45).
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    sluice = {
      url = "github:agentplot/sluice/v0.68.0";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-overlay.follows = "rust-overlay";
    };
    # Flume publishes no flake yet: nix/packages/flume.nix builds its tag.
    flume = {
      url = "github:agentplot/flume/v0.10.0";
      flake = false;
    };
    # The tests' nix-darwin. A host's configuration follows its own, which
    # changes none of the packages.
    nix-darwin = {
      url = "github:nix-darwin/nix-darwin";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { nixpkgs, ... }:
    let
      # The systems sluice's flake builds, each with its job in the cache's
      # workflow.
      systems = [
        "aarch64-darwin"
        "x86_64-darwin"
        "aarch64-linux"
        "x86_64-linux"
      ];
      forAll = nixpkgs.lib.genAttrs systems;
    in
    {
      packages = forAll (_: { });
    };
}
