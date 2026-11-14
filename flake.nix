# The family's host tools, pinned together and built from source: the
# runner at this tree, Flume, sluice, crew, timber, herdr and worktrunk
# each at a release tag through its own flake, timber's herdr and Claude
# Code plugins from its same tag, and portless at its release tag through
# this flake's recipe (records 0020 and 0023, core.44). crew's
# companions come with them (record 0044): Pending You as it publishes it
# on npm, Plannotator and plannotator-tui through numtide/llm-agents.nix's
# flake at their release tags, their herdr plugins at the commits the lock
# names, and the set's marketplace of their Claude Code plugins. git is
# the host's own. A host's nix substitutes their binaries from the
# family's cache where it trusts it, and builds the same paths where it
# does not (core.47). Its shell, devShells.<system>.default, puts the set on PATH and
# starts herdr (record 0023); the repository's own development shell is
# devenv's (devenv.nix). Everything else of the flake's is under nix/, the
# moon project `nix`.
{
  description = "Boomdrive: the runner, Flume, sluice, crew and timber, pinned together, with herdr, Ghostty, portless and worktrunk";

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
      url = "github:boomdrivedev/sluice/v0.94.8";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        rust-overlay.follows = "rust-overlay";
        nix-darwin.follows = "nix-darwin";
      };
    };
    flume = {
      url = "github:boomdrivedev/flume/v0.21.2";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        rust-overlay.follows = "rust-overlay";
        nix-darwin.follows = "nix-darwin";
      };
    };
    crew = {
      url = "github:boomdrivedev/crew/v0.59.0";
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
    # The worktrunk sluice's tree pins at its tag, which sluice's worktree
    # driver was checked against (record 0023; design Decision 1).
    worktrunk = {
      url = "github:max-sixty/worktrunk/v0.80.0";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        rust-overlay.follows = "rust-overlay";
      };
    };
    # timber through its own flake at its release tag, its CLI and both its
    # plugins, herdr's and Claude Code's, from this one pin (record 0023 as
    # amended on 2026-11-13; design Decision 1).
    timber = {
      url = "github:boomdrivedev/timber/v0.6.0";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        rust-overlay.follows = "rust-overlay";
      };
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
            # Flume's and crew's own flakes' packages, unchanged: each tagged
            # tree carries its version (core.44).
            flume = inputs.flume.packages.${system}.flume;
            crew = inputs.crew.packages.${system}.crew;
            # timber's own flake's package, unchanged: its tagged tree
            # carries its version (timber's record 0031; core.44).
            timber = inputs.timber.packages.${system}.default;
            # sluice's own flake's package, unchanged: its tagged tree
            # carries its version (sluice's record 0111; core.44).
            sluice = inputs.sluice.packages.${system}.default;
            inherit (inputs.herdr.packages.${system}) herdr;
            worktrunk = import ./nix/packages/worktrunk.nix {
              package = inputs.worktrunk.packages.${system}.worktrunk;
              tag = tagOf (import ./flake.nix).inputs.worktrunk.url;
            };
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
            # Plannotator and plannotator-tui, unchanged, through
            # numtide/llm-agents.nix's flake, which builds each from its
            # source at its release tag, since neither publishes a flake.
            inherit (inputs.llm-agents.packages.${system}) plannotator plannotator-tui;
          };
          pluginOf = tool: pkgs.callPackage ./nix/packages/herdr-plugin.nix { inherit tool; };
          plugins = {
            flume-herdr-plugin = pluginOf tools.flume;
            sluice-herdr-plugin = pluginOf tools.sluice;
          };
        in
        tools
        // plugins
        // {
          default = tools.boomdrive;
          # The way in: an app on macOS and a desktop entry on Linux that
          # runs `flume open` (design Decision 3).
          boomdrive-launcher = pkgs.callPackage ./nix/packages/launcher.nix { };
          # The set's marketplace of crew's companions' Claude Code plugins
          # and timber's (record 0044, point 2, and its amendment of
          # 2026-11-13), which no module installs yet.
          boomdrive-claude-plugins = pkgs.callPackage ./nix/packages/claude-plugins.nix {
            inherit (tools) pendingyou plannotator timber;
            skill = ./nix/packages/pendingyou-skill.md;
          };
          # The companions' herdr plugins, each at the commit the lock names
          # (record 0044, point 6): Annotate's bin holds its runtime and the
          # set's plannotator-tui, so herdr's link fetches nothing.
          pendingyou-herdr-plugin = pkgs.callPackage ./nix/packages/pendingyou-herdr-plugin.nix {
            src = inputs.herdr-pendingyou;
          };
          plannotator-herdr-plugin = pkgs.callPackage ./nix/packages/plannotator-herdr-plugin.nix {
            src = inputs.herdr-annotate;
            inherit (tools) plannotator-tui;
          };
          # timber's plugins/herdr at its tag, as Flume's and sluice's are,
          # outside `plugins`, so `herdr-plugins` stays theirs (design
          # Decisions 3 and 9).
          timber-herdr-plugin = pluginOf tools.timber;
          # The two plugins in one package a profile holds, which
          # `boomdrive setup` links from it (record 0009 as amended
          # 2026-11-11).
          herdr-plugins = pkgs.callPackage ./nix/packages/herdr-plugins.nix (
            plugins // { inherit (tools) boomdrive; }
          );
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
      nixosModules = modulesOf "nixos" ./nix/modules/nixos.nix;
    in
    {
      inherit packages nixosModules;

      # The boot: a NixOS system with the module, booted on aarch64-linux,
      # the system the boxes run, and on no other system (record 0020 as
      # amended on 2026-11-14; nix/boot.nix).
      checks.aarch64-linux.boot = import ./nix/boot.nix {
        pkgs = pkgsOf "aarch64-linux";
        module = nixosModules.default;
        set = packages.aarch64-linux;
      };

      # A host's way in: the set on PATH, starting herdr on entering
      # (record 0023). The repository's own development shell is devenv's.
      devShells = lib.genAttrs systems (system: {
        default = import ./nix/shell.nix {
          pkgs = pkgsOf system;
          set = packages.${system};
        };
      });

      darwinModules = modulesOf "darwin" ./nix/modules/darwin.nix;
    };
}
