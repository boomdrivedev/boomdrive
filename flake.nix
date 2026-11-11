# The family's host tools, pinned together and built from source: the
# runner at this tree, Flume, sluice, crew and herdr each at a release tag
# through its own flake, and portless at its release tag through this
# flake's recipe (records 0020 and 0023, core.44). crew's companions come
# with them (record 0044): Pending You as it publishes it on npm,
# Plannotator and plannotator-tui through numtide/llm-agents.nix's flake
# at their release tags, their herdr plugins at the commits the lock
# names, and the set's marketplace of their Claude Code plugins. git is
# the host's own. A host's nix substitutes their binaries from the
# family's cache where it trusts it, and builds the same paths where it
# does not (core.47). Its shell, devShells.<system>.default, puts the set on PATH and
# starts herdr (record 0023); the repository's own development shell is
# devenv's (devenv.nix). Everything else of the flake's is under nix/, the
# moon project `nix`.
{
  description = "Boomdrive: the runner, Flume, sluice and crew, pinned together";

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
  # (core.45). git is the host's own (record 0023).
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    sluice = {
      url = "github:agentplot/sluice/v0.78.1";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        rust-overlay.follows = "rust-overlay";
        nix-darwin.follows = "nix-darwin";
      };
    };
    flume = {
      url = "github:agentplot/flume/v0.21.2";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        rust-overlay.follows = "rust-overlay";
        nix-darwin.follows = "nix-darwin";
      };
    };
    crew = {
      url = "github:agentplot/crew/v0.50.2";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        nix-darwin.follows = "nix-darwin";
      };
    };
    # The herdr Flume's tree pins at its tag, which Flume's terminal profile
    # was checked against (record 0023; design Decision 1).
    herdr = {
      url = "github:herdrdev/herdr/v0.9.5";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        rust-overlay.follows = "rust-overlay";
      };
    };
    # portless publishes no flake, so the set builds it from its source at
    # its release tag by this flake's recipe, nix/packages/portless.nix
    # (record 0023, core.44).
    portless = {
      url = "github:vercel-labs/portless/v0.17.0";
      flake = false;
    };
    # crew's companions (record 0044, point 6). Pending You's command line
    # as Pending You publishes it on npm, since its source tags no release
    # and locks no dependency.
    pendingyou = {
      url = "https://registry.npmjs.org/pendingyou/-/pendingyou-0.41.0.tgz";
      flake = false;
    };
    # Plannotator and plannotator-tui, which publish no flake, through
    # numtide/llm-agents.nix's, which builds each from its source at its
    # release tag, at a commit that carries Plannotator 0.28.61 and
    # plannotator-tui 0.9.9.
    llm-agents = {
      url = "github:numtide/llm-agents.nix/3c4f3e7ccf54c7279659536160f9c58de38073c1";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    # Their herdr plugins, which tag no release of their own, each at the
    # commit that names the set's Pending You and plannotator-tui.
    herdr-pendingyou = {
      url = "github:recordplane/herdr-pendingyou/d930a00715c2284254f8cb938fe683dca17163ac";
      flake = false;
    };
    herdr-annotate = {
      url = "github:plannotator/herdr-annotate/a9973b43c1fc90044a9f15755e310e12b8465abb";
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

      # The flake's own nixpkgs for one system, which the set and the shell
      # are made with, so their paths are the ones the family's cache holds
      # (design Decision 4).
      pkgsOf =
        system:
        import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

      # The set for one system (design Decisions 2 and 3), made with the
      # flake's own nixpkgs, so its paths are the ones the family's cache
      # holds (design Decision 4). git is the host's own (record 0023).
      packagesOf =
        system:
        let
          pkgs = pkgsOf system;
          tools = {
            boomdrive = pkgs.callPackage ./nix/packages/boomdrive.nix { platforms = systems; };
            flume = inputs.flume.packages.${system}.flume;
            sluice = import ./nix/packages/sluice.nix {
              inherit lib;
              package = inputs.sluice.packages.${system}.default;
              src = inputs.sluice;
              tag = tagOf (import ./flake.nix).inputs.sluice.url;
            };
            crew = import ./nix/packages/crew.nix {
              inherit lib;
              package = inputs.crew.packages.${system}.crew;
              tag = tagOf (import ./flake.nix).inputs.crew.url;
            };
            inherit (inputs.herdr.packages.${system}) herdr;
            # Ghostty from the flake's nixpkgs, at the release Flume's window
            # profile was checked against: built from source on Linux, and on
            # macOS, where nothing builds its app from source, Ghostty's own
            # release as `ghostty-bin` gives it (design Decision 2).
            ghostty = if pkgs.stdenv.hostPlatform.isDarwin then pkgs.ghostty-bin else pkgs.ghostty;
            # portless by this flake's recipe from its source at its release
            # tag, as it publishes no flake (core.44).
            portless = pkgs.callPackage ./nix/packages/portless.nix { src = inputs.portless; };
            # crew's companions (record 0044, point 6): Pending You as it
            # publishes it on npm, run on the flake's Node.js 24.
            pendingyou = pkgs.callPackage ./nix/packages/pendingyou.nix { src = inputs.pendingyou; };
          };
          pluginOf = tool: pkgs.callPackage ./nix/packages/herdr-plugin.nix { inherit tool; };
        in
        tools
        // {
          default = tools.boomdrive;
          # The way in: an app on macOS and a desktop entry on Linux that
          # runs `flume open` (design Decision 3).
          boomdrive-launcher = pkgs.callPackage ./nix/packages/launcher.nix { };
          flume-herdr-plugin = pluginOf tools.flume;
          sluice-herdr-plugin = pluginOf tools.sluice;
        };
      packages = lib.genAttrs systems packagesOf;

      # A platform's modules (design Decision 4): `boomdrive`, also
      # `default`, is the runner's service, its options the runner's file
      # (record 0020, core.46), with Flume's, sluice's and crew's own
      # modules, each as its tool's flake gives it at the tag the lock
      # names, none carried here (record 0023, core.18); and each tool's
      # module again, as its flake gives it. Keyed, as Flume's is, so a
      # system that imports `default` and `boomdrive` takes it once.
      tools = [
        "flume"
        "sluice"
        "crew"
      ];
      modulesOf =
        platform: file:
        let
          toolOf = tool: inputs.${tool}."${platform}Modules".${tool};
        in
        rec {
          boomdrive = {
            _file = ./flake.nix;
            key = "github:agentplot/boomdrive#${platform}Modules.boomdrive";
            imports = [ (import file { inherit packages; }) ] ++ map toolOf tools;
          };
          default = boomdrive;
        }
        // lib.genAttrs tools toolOf;
    in
    {
      inherit packages;

      # A host's way in: the set on PATH, starting herdr on entering
      # (record 0023). The repository's own development shell is devenv's.
      devShells = lib.genAttrs systems (system: {
        default = import ./nix/shell.nix {
          pkgs = pkgsOf system;
          set = packages.${system};
        };
      });

      darwinModules = modulesOf "darwin" ./nix/modules/darwin.nix;
      nixosModules = modulesOf "nixos" ./nix/modules/nixos.nix;
    };
}
