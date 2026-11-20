# crew's companions, which the Boomdrive module installs (record 0044,
# point 8; design Decisions 6 and 9 of the change
# crews-companions-come-with-the-flake): Pending You, Plannotator and
# plannotator-tui, none of which gives a nix-darwin or NixOS module. Each
# package is the set's for the host's system by default, so it is the path
# the family's cache holds whatever nixpkgs the host runs (core.47); a
# host's own replaces it, and `null` installs none. Each is turned on by a
# switch of its own, `services.boomdrive.<companion>.enable`, on by default
# where `services.boomdrive` and `services.crew` both are, and goes in the
# system profile wherever its switch is on, with the runner or without,
# with `pendingyou-signin` beside a Pending You (record 0044 as amended on
# 2026-11-18; record 0023 as amended that day; design Decisions 1 and 2 of
# the change boomdrives-tools-install-on-a-machine-without-the-runner;
# core.63). apps.nix links their herdr plugins, and darwin.nix and
# nixos.nix write Pending You's folder where its switch is on (record
# 0044, point 7). With the runner and Flume, darwin.nix and nixos.nix run
# activate.sh's `claude`, which installs the set's marketplace into every
# Claude Code home of Flume's accounts and the default home by Claude
# Code's own commands, Pending You's and Plannotator's plugins where each
# one's switch is on, with Pending You's server where its folder is
# written; sessions.nix puts the folder's wake mod on each login's
# `CLAUDE_CODE_PLUGIN_DIRS` (record 0044, points 3 and 5, and its
# amendments of 2026-11-18).
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

  # A companion the host installs where its switch is on: the set's `name`
  # by default.
  installed =
    name: description:
    mkOption {
      type = types.nullOr types.package;
      default = set.${name};
      defaultText = setText name;
      inherit description;
    };

  # The one command the person runs, once for the computer, to sign
  # Claude Code's connection and herdr's popup in to Pending You (design
  # Decision 9): Claude Code's with the folder's own Claude folder,
  # `signin`, which names Pending You's server with the folder's headers
  # helper, and herdr's with the same arguments but `--force`, which
  # `app login` does not take. Each approval is the person's, in the
  # browser or by a code on the phone. No step of the modules runs it.
  signinOf =
    package:
    pkgs.writeShellApplication {
      name = "pendingyou-signin";
      text = ''
        PENDINGYOU_CONFIG_DIR="$HOME/.config/pendingyou"
        export PENDINGYOU_CONFIG_DIR
        CLAUDE_CONFIG_DIR="$PENDINGYOU_CONFIG_DIR/signin" \
          ${lib.getExe' package "pendingyou"} login --app claude-code "$@"
        herdr=()
        for argument in "$@"; do
          if [ "$argument" != --force ]; then
            herdr+=("$argument")
          fi
        done
        ${lib.getExe' package "pendingyou"} app login herdr "''${herdr[@]}"
      '';
    };

  # A companion's switch, on by default with the runner and crew, a host's
  # own value winning, with the runner or without (design Decisions 1 and 2
  # of the change boomdrives-tools-install-on-a-machine-without-the-runner).
  switch =
    what:
    mkOption {
      type = types.bool;
      default = cfg.enable && config.services.crew.enable;
      defaultText = lib.literalExpression "config.services.boomdrive.enable && config.services.crew.enable";
      description = "Whether to install ${what}, its `package`, with the runner or without: on by default where `services.boomdrive.enable` and `services.crew.enable` both are, so a host without either turns it on here, and off installs none of it.";
    };

  # A companion's package in the system profile where its switch is on and
  # the package is not `null`, and `pendingyou-signin` beside a Pending You.
  installing =
    companion: extra:
    let
      inherit (cfg.${companion}) package;
    in
    lib.mkIf cfg.${companion}.enable {
      environment.systemPackages = lib.optionals (package != null) ([ package ] ++ extra package);
    };
in
{
  options.services.boomdrive = {
    pendingyou.enable = switch "Pending You, with `pendingyou-signin`, and write its folder";
    pendingyou.package = installed "pendingyou" "Pending You's command line, which posts the cards a crew session waits on a person for, installed where `services.boomdrive.pendingyou.enable` is on, with `pendingyou-signin`, and run by the launchers each activation writes into ~/.config/pendingyou; a host's own replaces it, and `null` installs none, writes no folder and installs no `pendingyou-signin`.";
    plannotator.enable = switch "Plannotator";
    plannotator.package = installed "plannotator" "Plannotator, which reviews a plan in the browser, installed where `services.boomdrive.plannotator.enable` is on; a host's own replaces it, and `null` installs none.";
    plannotator-tui.enable = switch "plannotator-tui";
    plannotator-tui.package = installed "plannotator-tui" "plannotator-tui, which reviews a document in a herdr pane beside the agent's, installed where `services.boomdrive.plannotator-tui.enable` is on; a host's own replaces it, and `null` installs none. Where its pane opens is the person's, in ~/.config/plannotator-tui/config.toml.";
  };

  # Each companion's install reads its own switch (design Decision 1 of the
  # change boomdrives-tools-install-on-a-machine-without-the-runner).
  config = lib.mkMerge [
    (installing "pendingyou" (package: [ (signinOf package) ]))
    (installing "plannotator" (_: [ ]))
    (installing "plannotator-tui" (_: [ ]))
  ];
}
