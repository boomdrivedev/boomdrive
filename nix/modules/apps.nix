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
# activation, darwin.nix's and nixos.nix's, with herdr's own command.
# Where herdr is turned on, Flume's module is told herdr, its
# `host.multiplexer`, and where Ghostty is, Ghostty, its
# `host.terminal_app`, each by default, so a host's own wins, with the
# runner or without, and whether or not the host keeps the set's herdr and
# Ghostty (design Decision 5 of the change
# boomdrives-tools-install-on-a-machine-without-the-runner).
# portless, which gives each worktree's dev server a `.localhost` address
# and whose routes crew's board reads, is installed the same way, since it
# gives no nix-darwin or NixOS module; the modules start no proxy of it and
# trust no certificate authority of it, which are the person's (record
# 0023 as amended on 2026-11-11; design Decisions 5 and 6 of the change
# portless-comes-with-the-flake). The plugins' default is the set's plugin
# of each tool the set installs, each only beside the set's own build of
# it: Flume's, sluice's and timber's, then crew's companions' herdr
# plugins, Pending You for Herdr and Annotate, which companions.nix
# installs the tools of, each where its tool is on, with the runner or
# without (record 0044, point 8, and record 0023, each as amended on
# 2026-11-18; design Decision 3 of the change
# boomdrives-tools-install-on-a-machine-without-the-runner). timber, which
# delivers what the merge queue lands, comes by its own module, which
# flake.nix imports and tools.nix turns on with the runner, whether or not
# crew is on; its herdr plugin is linked here with Flume's and sluice's,
# and the modules write no file of timber's and start no
# `timber serve`, which is timber's own (record 0023 as amended on
# 2026-11-13 and 2026-11-15; design Decisions 5 to 7 of the change
# the-set-takes-timbers-own-module).
# With the runner, the launcher is the set's made, by its override, for
# each drive of this host, in the order of the drives' names, and without
# it the set's own, `Default stream` alone, since a drive gives Flume its
# stream only with the runner (record 0023 as amended on 2026-11-18;
# design Decision 6 of the change
# boomdrives-tools-install-on-a-machine-without-the-runner): each entry is the drive's name
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

  # Whether the set installs a tool of its own module, `name`, as its own
  # build: the tool's module on with the set's package; and a companion of
  # the Boomdrive module's, its switch on with the set's package (design
  # Decision 3 of the change
  # boomdrives-tools-install-on-a-machine-without-the-runner).
  tools = name: config.services.${name}.enable && config.services.${name}.package == set.${name};
  companions = name: package: cfg.${name}.enable && cfg.${name}.package == set.${package};

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
        # The set's plugin of each tool the set installs, beside the set's own
        # build of it, so the plugin linked is built from the same pin as the
        # command line installed (record 0023 as amended on 2026-11-18;
        # design Decision 3 of the change
        # boomdrives-tools-install-on-a-machine-without-the-runner;
        # blueprints principle 17). Two derivations are equal by their
        # `outPath`.
        default =
          lib.optional (tools "flume") set.flume-herdr-plugin
          ++ lib.optional (tools "sluice") set.sluice-herdr-plugin
          ++ lib.optional (tools "timber") set.timber-herdr-plugin
          ++ lib.optional (companions "pendingyou" "pendingyou") set.pendingyou-herdr-plugin
          ++ lib.optional (companions "plannotator-tui" "plannotator-tui") set.plannotator-herdr-plugin;
        defaultText = lib.literalExpression ''
          let
            set = boomdrive.packages.''${pkgs.stdenv.hostPlatform.system};
            tool = name: config.services.''${name}.enable && config.services.''${name}.package == set.''${name};
            companion = name: config.services.boomdrive.''${name}.enable && config.services.boomdrive.''${name}.package == set.''${name};
          in
          lib.optional (tool "flume") set.flume-herdr-plugin
          ++ lib.optional (tool "sluice") set.sluice-herdr-plugin
          ++ lib.optional (tool "timber") set.timber-herdr-plugin
          ++ lib.optional (companion "pendingyou") set.pendingyou-herdr-plugin
          ++ lib.optional (companion "plannotator-tui") set.plannotator-herdr-plugin
        '';
        description = "The herdr plugins each activation links, in order, with herdr's own `herdr plugin link`, as the set's user, wherever `services.boomdrive.herdr.enable` is on, with the runner or without. By default, the set's plugin of each tool the set installs, each only beside the set's own build of it: Flume's, sluice's and timber's where that tool's module is on with the set's package, then Pending You for Herdr where `services.boomdrive.pendingyou.enable` is on with the set's Pending You, and Annotate where `services.boomdrive.plannotator-tui.enable` is on with the set's plannotator-tui. A host that runs its own build of a tool and wants its pane lists its own build's plugin here, and a list given is linked as given. A relink enables a plugin the person disabled, so a host that wants one off leaves it out; one left out stays linked until `herdr plugin unlink`.";
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
      # A drive gives Flume its stream only with the runner (drives.nix;
      # record 0028), so without it the launcher holds no drive (design
      # Decision 6 of the change
      # boomdrives-tools-install-on-a-machine-without-the-runner).
      default = set.boomdrive-launcher.override { drives = if cfg.enable then entries else [ ]; };
      defaultText = lib.literalExpression "boomdrive.packages.\${pkgs.stdenv.hostPlatform.system}.boomdrive-launcher.override { drives = <each drive of this host where config.services.boomdrive.enable is on, else none>; }";
      description = "The Boomdrive launcher, which runs `flume open`: `Boomdrive` in /Applications/Nix Apps on macOS, a menu-bar app, and the desktop's applications on Linux, installed where `services.boomdrive.launcher.enable` is on. By default it is the set's made for this host's drives: each drive of this host, in the order of the drives' names, is an entry opening its stream's own session, the stream's `session` in Flume's settings, else the stream's name, beside `Default stream`; a drive of another machine has none. Without the runner, where no drive gives Flume a stream, it is the set's own, `Default stream` alone. A host's own replaces it, installed as given, and `null` installs none.";
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

    # Flume's terminal and window (design Decision 7), each where its tool is
    # turned on, with the runner or without (design Decision 5 of the change
    # boomdrives-tools-install-on-a-machine-without-the-runner). `terminal`
    # is the word a host wrote before for `multiplexer`, which Flume reads
    # as it and refuses beside it (Flume's C.7), so where the host gives it
    # no `multiplexer` is set. Flume's module declares no `terminal`, so the
    # condition reads no definition it guards.
    (lib.mkIf cfg.herdr.enable {
      services.flume.settings.host.multiplexer = lib.mkIf (
        !(config.services.flume.settings.host ? terminal)
      ) (lib.mkDefault "herdr");
    })
    (lib.mkIf cfg.ghostty.enable {
      services.flume.settings.host.terminal_app = lib.mkDefault "ghostty";
    })
  ];
}
