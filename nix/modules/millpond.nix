# millpond with the runner (record 0033): the Boomdrive module installs
# millpond, whose tick the runner's pass runs for each inbox the host
# keeps, writes millpond's host configuration from
# `services.boomdrive.millpond.settings`, and gives the runner's file the
# names of its inboxes, `inboxes`, so each inbox is named once, here.
# millpond gives no module of its own. Its package is the set's for the
# host's system by default, so it is the path the family's cache holds
# (core.47); a host's own replaces it, and `null` installs none, writes no
# configuration and names no inbox to the runner. The configuration file
# is rendered at evaluation, as the runner's is (toml.nix), and darwin.nix
# and nixos.nix link it at millpond's own path,
# `~/.config/millpond/config.toml`, at each activation (activate.sh
# `millpond`). It holds no secret: `secrets` names the folder millpond
# reads them from.
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
  toml = (pkgs.formats.toml { }).type;

  inbox = types.submodule {
    options = {
      name = mkOption {
        type = types.strMatching ".+";
        description = "The inbox's name, which millpond's commands and the runner's `inboxes` name it by.";
      };
      repository = mkOption {
        type = types.strMatching ".+";
        description = "The inbox's repository: a local repository's path, or a remote's address.";
      };
      branch = mkOption {
        type = types.strMatching ".+";
        default = "inbox";
        description = "The inbox's branch, millpond's `inbox` by default.";
      };
    };
  };

  render = import ./toml.nix {
    inherit lib;
    header = "# Written by the boomdrive flake's module from services.boomdrive.millpond.settings; a change goes there.";
  };

  names = map (entry: entry.name) cfg.millpond.settings.inbox;
  writing = cfg.enable && cfg.millpond.package != null;
in
{
  options.services.boomdrive.millpond = {
    package = mkOption {
      type = types.nullOr types.package;
      default = set.millpond;
      defaultText = lib.literalExpression "boomdrive.packages.\${pkgs.stdenv.hostPlatform.system}.millpond";
      description = "millpond, installed with the runner, whose tick the runner's pass runs for each inbox of `settings.inbox` (record 0033); a host's own replaces it, and `null` installs none, writes no configuration and names no inbox to the runner.";
    };

    settings = mkOption {
      type = types.submodule {
        freeformType = toml;
        options = {
          host = mkOption {
            type = types.nullOr types.str;
            default = null;
            description = "This machine's name in millpond's pointers and settings; its short hostname where unset.";
          };
          secrets = mkOption {
            type = types.nullOr types.str;
            default = null;
            description = "The folder holding one file per secret millpond's adapters read, outside the Nix store; none where unset.";
          };
          declarations = mkOption {
            type = types.listOf types.str;
            default = [ ];
            description = "The adapters' declaration files this host can run.";
          };
          inbox = mkOption {
            type = types.listOf inbox;
            default = [ ];
            description = "Each inbox this host keeps, in the order the runner ticks them: its name, repository and branch.";
          };
        };
      };
      default = { };
      description = "millpond's host configuration, `~/.config/millpond/config.toml`, each key named as millpond names it, rendered at evaluation; the runner's `inboxes` are its inboxes' names.";
    };

    text = mkOption {
      type = types.nullOr types.str;
      readOnly = true;
      internal = true;
      default =
        if writing && cfg.millpond.settings.inbox != [ ] then render cfg.millpond.settings else null;
      description = "millpond's host configuration's text as the module renders it at evaluation; none where millpond is not installed or no inbox is named.";
    };

    file = mkOption {
      type = types.nullOr types.path;
      readOnly = true;
      internal = true;
      default =
        if cfg.millpond.text == null then null else pkgs.writeText "millpond-config.toml" cfg.millpond.text;
      description = "millpond's host configuration as the module renders it, which each activation links at millpond's own path; none where millpond is not installed or no inbox is named.";
    };
  };

  config = lib.mkIf writing {
    assertions = [
      {
        assertion = lib.unique names == names;
        message = "services.boomdrive.millpond.settings.inbox names an inbox more than once: ${lib.concatStringsSep ", " names}.";
      }
      {
        assertion = !lib.hasPrefix builtins.storeDir (toString cfg.millpond.settings.secrets);
        message = "services.boomdrive.millpond.settings.secrets lies in the Nix store, which every user reads: name millpond's secrets by a folder outside it.";
      }
    ];
    environment.systemPackages = [ cfg.millpond.package ];
    services.boomdrive.settings.inboxes = lib.mkIf (names != [ ]) (lib.mkDefault names);
  };
}
