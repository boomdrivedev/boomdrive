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
# The launcher is the set's made, by its override, for each drive of this
# host, in the order of the drives' names: each entry is the drive's name
# and its stream's own session, the `session` Flume's settings name for the
# stream, else the stream's name, so the override is made with the set's
# own nixpkgs and its compiled menu is the set's (design Decisions 1 and 5
# of the change each-drive-opens-from-the-launcher; core.47).
{ packages }:
{
  config,
  lib,
  options,
  pkgs,
  ...
}:
let
  inherit (lib) mkOption types;
  cfg = config.services.boomdrive;
  inherit (import ./accounts.nix { inherit lib; }) drivesOf hereOf;

  # This host, as crew names it, whether or not crew is on; null where it
  # is not set, as drives.nix reads it.
  host = if options.services.crew.host.isDefined then config.services.crew.host else null;

  # A drive's stream's own session, as Flume names it (Flume's 107 and
  # 186): its `session` in Flume's settings where that is set, else the
  # stream's name.
  streams = config.services.flume.settings.streams or null;
  sessionOf =
    drive:
    let
      named = if streams == null then null else streams.${drive.stream.name}.session or null;
    in
    if named == null then drive.stream.name else named;

  # The launcher's entries: each drive of this host, in the order of the
  # drives' names, which are given once on the host (design Decision 1).
  entries = lib.sort (a: b: a.drive < b.drive) (
    map (drive: {
      drive = drive.name;
      session = sessionOf drive;
    }) (hereOf host (drivesOf cfg.accounts))
  );

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
    launcher.package = mkOption {
      type = types.nullOr types.package;
      default = set.boomdrive-launcher.override { drives = entries; };
      defaultText = lib.literalExpression "boomdrive.packages.\${pkgs.stdenv.hostPlatform.system}.boomdrive-launcher.override { drives = <each drive of this host>; }";
      description = "The Boomdrive launcher, which runs `flume open`: `Boomdrive` in /Applications/Nix Apps on macOS, a menu-bar app, and the desktop's applications on Linux, installed with the runner. By default it is the set's made for this host's drives: each drive of this host, in the order of the drives' names, is an entry opening its stream's own session, the stream's `session` in Flume's settings, else the stream's name, beside `Default stream`; a drive of another machine has none. A host's own replaces it, installed as given, and `null` installs none.";
    };
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
