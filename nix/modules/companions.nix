# crew's companions, which the Boomdrive module installs with the runner
# where crew's module is on (record 0044, point 8; design Decisions 6 and
# 9 of the change crews-companions-come-with-the-flake): Pending You,
# Plannotator and plannotator-tui, none of which gives a nix-darwin or
# NixOS module. Each package is the set's for the host's system by
# default, so it is the path the family's cache holds whatever nixpkgs the
# host runs (core.47); a host's own replaces it, and `null` installs none.
# With `services.boomdrive` and `services.crew` both on they go in the
# system profile, with `pendingyou-signin` beside a Pending You; without
# either, none does. apps.nix links their herdr plugins, and darwin.nix
# and nixos.nix write Pending You's folder (record 0044, point 7). No
# module runs `claude` or writes into a Claude Code home: installing the
# set's marketplace there waits on the user's answer (record 0044, point
# 3).
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

  # A companion the runner's host installs where crew's module is on: the
  # set's `name` by default.
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

  installing = cfg.enable && config.services.crew.enable;
in
{
  options.services.boomdrive = {
    pendingyou.package = installed "pendingyou" "Pending You's command line, which posts the cards a crew session waits on a person for, installed with the runner where crew's module is on, with `pendingyou-signin`, and run by the launchers each activation writes into ~/.config/pendingyou; a host's own replaces it, and `null` installs none, writes no folder and installs no `pendingyou-signin`.";
    plannotator.package = installed "plannotator" "Plannotator, which reviews a plan in the browser, installed with the runner where crew's module is on; a host's own replaces it, and `null` installs none.";
    plannotator-tui.package = installed "plannotator-tui" "plannotator-tui, which reviews a document in a herdr pane beside the agent's, installed with the runner where crew's module is on; a host's own replaces it, and `null` installs none. Where its pane opens is the person's, in ~/.config/plannotator-tui/config.toml.";
  };

  config = lib.mkIf installing {
    environment.systemPackages =
      lib.filter (package: package != null) [
        cfg.pendingyou.package
        cfg.plannotator.package
        cfg.plannotator-tui.package
      ]
      ++ lib.optional (cfg.pendingyou.package != null) (signinOf cfg.pendingyou.package);
  };
}
