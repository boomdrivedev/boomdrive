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
    inputs@{ nixpkgs, rust-overlay, ... }:
    let
      inherit (nixpkgs) lib;
      # The systems sluice's flake builds that nixpkgs builds for, each with
      # its job in the cache's workflow: nixpkgs 26.11 refuses
      # x86_64-darwin (design Decision 2).
      systems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-linux"
      ];
      tagOf = import ./nix/packages/tag.nix { inherit lib; };

      # The set for one system (design Decisions 2 and 3), made with the
      # flake's own nixpkgs, so its paths are the ones the family's cache
      # holds (design Decision 4). herdr and git are the host's own.
      packagesOf =
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ rust-overlay.overlays.default ];
          };
          tools = {
            boomdrive = pkgs.callPackage ./nix/packages/boomdrive.nix { platforms = systems; };
            flume = pkgs.callPackage ./nix/packages/flume.nix {
              src = inputs.flume;
              platforms = systems;
            };
            sluice = import ./nix/packages/sluice.nix {
              inherit lib;
              package = inputs.sluice.packages.${system}.default;
              src = inputs.sluice;
              tag = tagOf (import ./flake.nix).inputs.sluice.url;
            };
          };
          pluginOf = tool: pkgs.callPackage ./nix/packages/herdr-plugin.nix { inherit tool; };
        in
        tools
        // {
          default = tools.boomdrive;
          flume-herdr-plugin = pluginOf tools.flume;
          sluice-herdr-plugin = pluginOf tools.sluice;
        };
      packages = lib.genAttrs systems packagesOf;
    in
    {
      inherit packages;

      # The runner's service, its options the runner's file (record 0020,
      # core.46), its packages the flake's own above.
      darwinModules = rec {
        boomdrive = import ./nix/modules/darwin.nix { inherit packages; };
        default = boomdrive;
      };
      nixosModules = rec {
        boomdrive = import ./nix/modules/nixos.nix { inherit packages; };
        default = boomdrive;
      };
    };
}
