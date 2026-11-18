# The apps the Boomdrive module installs with the runner, which no tool's
# own module installs, and its own options for them (record 0023; design
# Decisions 5 and 7 of the change
# herdr-and-ghostty-come-with-the-flake-and-an-app-opens-the-agents):
# herdr and the plugins its activation links, Ghostty and the Boomdrive
# launcher, since neither herdr nor Ghostty gives a nix-darwin or NixOS
# module, and the launcher is this flake's. worktrunk's `wt`, which crew
# and sluice's worktree driver call, is installed the same way, since
# worktrunk gives no nix-darwin or NixOS module either (record 0023 as
# amended on 2026-11-08; design Decision 3 of the change
# worktrunk-comes-with-the-flake). Each package is the set's for the
# host's system by default, so it is the path the family's cache holds
# whatever nixpkgs the host runs (core.47); a host's own replaces it, and
# `null` installs none. Each is turned on by a switch of its own,
# `services.boomdrive.<tool>.enable`, which the runner's turns on by
# default, and goes in the system profile wherever its switch is on, with
# the runner or without, so a host without the runner takes the ones it
# turns on and nothing else (record 0023 as amended on 2026-11-18; design
# Decisions 1 and 2 of the change
# boomdrives-tools-install-on-a-machine-without-the-runner; core.62).
# The plugins are linked by the platform's
# activation, darwin.nix's and nixos.nix's, with herdr's own command. With
# the runner enabled, Flume's module is told herdr and Ghostty, its
# `host.multiplexer` and `host.terminal_app`, each by default, so a host's
# own wins, and whether or not the host keeps the set's herdr and Ghostty.
# portless, which gives each worktree's dev server a `.localhost` address
# and whose routes crew's board reads, is installed the same way, since it
# gives no nix-darwin or NixOS module; the modules start no proxy of it and
# trust no certificate authority of it, which are the person's (record
# 0023 as amended on 2026-11-11; design Decisions 5 and 6 of the change
# portless-comes-with-the-flake). Where crew's module is on, the plugins'
# default gains crew's companions' herdr plugins, Pending You for Herdr and
# Annotate, which companions.nix installs the tools of (record 0044, point
# 8). timber, which delivers what the merge queue lands, comes by its own
# module, which flake.nix imports and tools.nix turns on with the runner,
# whether or not crew is on; its herdr plugin is linked here with Flume's
# and sluice's, and the modules write no file of timber's and start no
# `timber serve`, which is timber's own (record 0023 as amended on
# 2026-11-13 and 2026-11-15; design Decisions 5 to 7 of the change
# the-set-takes-timbers-own-module).
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

  # A package the host installs where its switch is on: the set's `name`
  # by default.
  installed =
    name: description:
    mkOption {
      type = types.nullOr types.package;
      default = set.${name};
      defaultText = setText name;
      inherit description;
    };

  # A tool's switch, on by default with the runner, a host's own value
  # winning, with the runner or without (record 0023 as amended on
  # 2026-11-18; design Decisions 1 and 2 of the change
  # boomdrives-tools-install-on-a-machine-without-the-runner).
  switch =
    what:
    mkOption {
      type = types.bool;
      default = cfg.enable;
      defaultText = lib.literalExpression "config.services.boomdrive.enable";
      description = "Whether to install ${what}, its `package`, with the runner or without: on by default where `services.boomdrive.enable` is, so a host without the runner turns it on here, and off installs none of it.";
    };

  # A tool's package in the system profile where its switch is on and the
  # package is not `null`.
  installing =
    tool:
    lib.mkIf cfg.${tool}.enable {
      environment.systemPackages = lib.optional (cfg.${tool}.package != null) cfg.${tool}.package;
    };
in
{
  options.services.boomdrive = {
    herdr = {
      enable = switch "herdr and link its plugins";
      package = installed "herdr" "herdr, the multiplexer Flume's terminal profile runs, installed where `services.boomdrive.herdr.enable` is on and running each plugin's link; a host's own replaces it, and `null` installs none and links no plugin.";
      plugins = mkOption {
        type = types.listOf types.package;
        default = [
          set.flume-herdr-plugin
          set.sluice-herdr-plugin
          set.timber-herdr-plugin
        ]
        ++ lib.optionals config.services.crew.enable [
          set.pendingyou-herdr-plugin
          set.plannotator-herdr-plugin
        ];
        defaultText = lib.literalExpression ''
          [
            boomdrive.packages.''${pkgs.stdenv.hostPlatform.system}.flume-herdr-plugin
            boomdrive.packages.''${pkgs.stdenv.hostPlatform.system}.sluice-herdr-plugin
            boomdrive.packages.''${pkgs.stdenv.hostPlatform.system}.timber-herdr-plugin
          ]
          ++ lib.optionals config.services.crew.enable [
            boomdrive.packages.''${pkgs.stdenv.hostPlatform.system}.pendingyou-herdr-plugin
            boomdrive.packages.''${pkgs.stdenv.hostPlatform.system}.plannotator-herdr-plugin
          ]
        '';
        description = "The herdr plugins each activation links, in order, with herdr's own `herdr plugin link`, as the runner's user: Flume's, sluice's and timber's, then, where crew's module is on, crew's companions', Pending You for Herdr and Annotate. A relink enables a plugin the person disabled, so a host that wants one off leaves it out; one left out stays linked until `herdr plugin unlink`.";
      };
    };
    ghostty = {
      enable = switch "Ghostty";
      package = installed "ghostty" "Ghostty, the terminal app Flume's window profile opens, installed where `services.boomdrive.ghostty.enable` is on; a host's own replaces it, and `null` installs none.";
    };
    worktrunk = {
      enable = switch "worktrunk";
      package = installed "worktrunk" "worktrunk's `wt`, which crew and sluice's worktree driver call, installed where `services.boomdrive.worktrunk.enable` is on; a host's own replaces it, and `null` installs none.";
    };
    launcher.enable = switch "the Boomdrive launcher";
    launcher.package = mkOption {
      type = types.nullOr types.package;
      default = set.boomdrive-launcher.override { drives = entries; };
      defaultText = lib.literalExpression "boomdrive.packages.\${pkgs.stdenv.hostPlatform.system}.boomdrive-launcher.override { drives = <each drive of this host>; }";
      description = "The Boomdrive launcher, which runs `flume open`: `Boomdrive` in /Applications/Nix Apps on macOS, a menu-bar app, and the desktop's applications on Linux, installed where `services.boomdrive.launcher.enable` is on. By default it is the set's made for this host's drives: each drive of this host, in the order of the drives' names, is an entry opening its stream's own session, the stream's `session` in Flume's settings, else the stream's name, beside `Default stream`; a drive of another machine has none. A host's own replaces it, installed as given, and `null` installs none.";
    };
    portless.enable = switch "portless";
    portless.package = installed "portless" "portless, which gives each worktree's dev server a `.localhost` address and whose routes crew's board reads, installed where `services.boomdrive.portless.enable` is on; a host's own replaces it, and `null` installs none. Its proxy and the trust of its certificate authority are the person's: `portless service install` or a first `portless run` at a terminal, and `portless trust`.";
  };

  # Each tool's install reads its own switch (design Decision 1 of the
  # change boomdrives-tools-install-on-a-machine-without-the-runner).
  config = lib.mkMerge [
    (installing "herdr")
    (installing "ghostty")
    (installing "worktrunk")
    (installing "launcher")
    (installing "portless")

    # Flume's terminal and window (design Decision 7). `terminal` is the word
    # a host wrote before for `multiplexer`, which Flume reads as it and
    # refuses beside it (Flume's C.7), so where the host gives it no
    # `multiplexer` is set. Flume's module declares no `terminal`, so the
    # condition reads no definition it guards.
    (lib.mkIf cfg.enable {
      services.flume.settings.host = {
        multiplexer = lib.mkIf (!(config.services.flume.settings.host ? terminal)) (lib.mkDefault "herdr");
        terminal_app = lib.mkDefault "ghostty";
      };
    })
  ];
}
