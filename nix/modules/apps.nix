# The apps the Boomdrive module installs with the runner, which no tool's
# own module installs, and its own options for them (record 0023; design
# Decisions 5 and 7 of the change
# herdr-and-ghostty-come-with-the-flake-and-an-app-opens-the-agents):
# herdr and the plugins its activation links, Ghostty and the Boomdrive
# launcher, since neither herdr nor Ghostty gives a nix-darwin or NixOS
# module, and the launcher is this flake's. Each package is the set's for the
# host's system by default, so it is the path the family's cache holds
# whatever nixpkgs the host runs (core.47); a host's own replaces it, and
# `null` installs none. With the runner enabled they go in the system
# profile; without it, none does. The plugins are linked by the platform's
# activation, darwin.nix's and nixos.nix's, with herdr's own command. With
# the runner enabled, Flume's module is told herdr and Ghostty, its
# `host.multiplexer` and `host.terminal_app`, each by default, so a host's
# own wins, and whether or not the host keeps the set's herdr and Ghostty.
{ packages }:
{
  config,
  lib,
  pkgs,
  ...
}:
let
  inherit (lib) mkOption types;
  cfg = config.services.boomdrive;

  set = import ./set.nix {
    inherit lib packages;
    inherit (pkgs.stdenv.hostPlatform) system;
  };
  setText =
    name: lib.literalExpression "boomdrive.packages.\${pkgs.stdenv.hostPlatform.system}.${name}";

  # A package the runner's host installs: the set's `name` by default.
  installed =
    name: description:
    mkOption {
      type = types.nullOr types.package;
      default = set.${name};
      defaultText = setText name;
      inherit description;
    };
in
{
  options.services.boomdrive = {
    herdr = {
      package = installed "herdr" "herdr, the multiplexer Flume's terminal profile runs, installed with the runner and running each plugin's link; a host's own replaces it, and `null` installs none and links no plugin.";
      plugins = mkOption {
        type = types.listOf types.package;
        default = [
          set.flume-herdr-plugin
          set.sluice-herdr-plugin
        ];
        defaultText = lib.literalExpression ''
          [
            boomdrive.packages.''${pkgs.stdenv.hostPlatform.system}.flume-herdr-plugin
            boomdrive.packages.''${pkgs.stdenv.hostPlatform.system}.sluice-herdr-plugin
          ]
        '';
        description = "The herdr plugins each activation links, in order, with herdr's own `herdr plugin link`, as the runner's user. A relink enables a plugin the person disabled, so a host that wants one off leaves it out; one left out stays linked until `herdr plugin unlink`.";
      };
    };
    ghostty.package = installed "ghostty" "Ghostty, the terminal app Flume's window profile opens, installed with the runner; a host's own replaces it, and `null` installs none.";
    launcher.package = installed "boomdrive-launcher" "The Boomdrive launcher, which runs `flume open`: `Boomdrive` in /Applications/Nix Apps on macOS and the desktop's applications on Linux, installed with the runner; a host's own replaces it, and `null` installs none.";
  };

  config = lib.mkIf cfg.enable {
    environment.systemPackages = lib.filter (package: package != null) [
      cfg.herdr.package
      cfg.ghostty.package
      cfg.launcher.package
    ];

    # Flume's terminal and window (design Decision 7). `terminal` is the word
    # a host wrote before for `multiplexer`, which Flume reads as it and
    # refuses beside it (Flume's C.7), so where the host gives it no
    # `multiplexer` is set. Flume's module declares no `terminal`, so the
    # condition reads no definition it guards.
    services.flume.settings.host = {
      multiplexer = lib.mkIf (!(config.services.flume.settings.host ? terminal)) (lib.mkDefault "herdr");
      terminal_app = lib.mkDefault "ghostty";
    };
  };
}
