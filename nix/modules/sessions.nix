# Every Claude Code session on the host loads the set's plugins (record
# 0044, points 1, 3 and 5, and its amendments of 2026-11-18; design
# Decisions 3, 5, 6 and 8 of the change
# every-claude-code-session-has-boomdrives-plugins): what darwin.nix and
# nixos.nix give the step `activate.sh claude`, which they run as the
# runner's user where the runner and Flume, with a package, are on.
#
# - `runs`: whether a system runs the step.
# - `pluginsOf`: the set's marketplace's plugins split into those the step
#   installs, each whose tool the host installs with a package, and those
#   it removes, the rest: `flume` by Flume's module, `sluice` and
#   `sluice-board` by sluice's, `crew` by crew's, `timber` by timber's, and
#   `worktrunk`, `pendingyou` and `plannotator` by their switches.
# - `serverOf`: a store file of Pending You's server, the one its folder's
#   signin/.claude.json names, where the modules write that folder, and
#   `-` elsewhere.
# - `argumentsOf`: the step's arguments after `claude`: the host's
#   `flume`, the system's `jq`, the set's marketplace, the server's file,
#   the plugins to install, then `--` and those to remove.
# - `module`: where the modules write Pending You's folder, the lines of
#   plugin-dirs.sh in `environment.extraInit`, on nix-darwin and on NixOS,
#   which put the folder's wake mod on each login's
#   `CLAUDE_CODE_PLUGIN_DIRS` (record 0044, point 5).
#
# This file writes nothing, and runs no `claude`.
{ lib, packages }:
let
  # A tool the host installs: its switch on, with a package.
  installs = tool: (tool.enable or false) && (tool.package or null) != null;
in
rec {
  runs = config: config.services.boomdrive.enable && installs config.services.flume;

  pluginsOf =
    config: market:
    let
      inherit (config) services;
      cfg = services.boomdrive;
      on = {
        flume = installs services.flume;
        sluice = installs services.sluice;
        sluice-board = installs services.sluice;
        crew = installs services.crew;
        timber = installs services.timber;
        worktrunk = installs cfg.worktrunk;
        pendingyou = installs cfg.pendingyou;
        plannotator = installs cfg.plannotator;
      };
      names = map (plugin: plugin.name) market.plugins;
    in
    {
      install = lib.filter (name: on.${name} or false) names;
      remove = lib.filter (name: !(on.${name} or false)) names;
    };

  serverOf =
    {
      writeText,
      folder,
      writing,
    }:
    if writing then writeText "pendingyou-server.json" (builtins.toJSON folder.server) else "-";

  argumentsOf =
    {
      config,
      pkgs,
      folder,
      writing,
    }:
    let
      market =
        (import ./set.nix {
          inherit lib packages;
          inherit (pkgs.stdenv.hostPlatform) system;
        }).boomdrive-claude-plugins;
      plugins = pluginsOf config market;
    in
    [
      (lib.getExe' config.services.flume.package "flume")
      (lib.getExe pkgs.jq)
      "${market}"
      "${serverOf {
        inherit (pkgs) writeText;
        inherit folder writing;
      }}"
    ]
    ++ plugins.install
    ++ [ "--" ]
    ++ plugins.remove;

  module =
    { config, ... }:
    {
      _file = ./sessions.nix;
      config = lib.mkIf (installs config.services.boomdrive.pendingyou) {
        environment.extraInit = builtins.readFile ./plugin-dirs.sh;
      };
    };
}
