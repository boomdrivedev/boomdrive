# services.boomdrive, the options both modules share (design Decision 8):
# `settings`, the runner's file under its own names (core.46), free-form
# TOML with the README's keys declared for their types (RFC 42). A key
# left unset is left out of the file, so the runner's own default holds
# and the two never drift; a key the module does not declare passes to the
# file, and the runner judges it.
{ lib, pkgs, ... }:
let
  inherit (lib) mkOption types;
  toml = (pkgs.formats.toml { }).type;

  # `<n>s`, `<n>m` or `<n>h`, n above zero, as the runner reads it.
  interval = types.strMatching "[0-9]*[1-9][0-9]*[smh]";

  unset =
    type: description:
    mkOption {
      type = types.nullOr type;
      default = null;
      inherit description;
    };

  controlPlane = types.submodule {
    freeformType = toml;
    options = {
      type = mkOption {
        type = types.enum [
          "crew"
          "hosted"
        ];
        description = "The control plane's implementation the runner opens.";
      };
      poll_interval = unset interval "How often a control pass is made; the runner's `1m` where unset.";
      crew = unset (types.attrsOf toml) "`[control_plane.crew]`, handed whole to crew's implementation, whose keys are its own.";
      hosted = unset (types.attrsOf toml) "`[control_plane.hosted]`, handed whole to the hosted implementation, whose keys are its own. It holds no token or key: a partition's key is made on the host by `boomdrive enroll`.";
    };
  };
in
{
  options.services.boomdrive.settings = mkOption {
    type = types.submodule {
      freeformType = toml;
      options = {
        repositories = mkOption {
          type = types.listOf (types.strMatching "[A-Za-z0-9-]+/[A-Za-z0-9._-]+");
          default = [ ];
          description = "Each repository the host keeps, by its GitHub address, `owner/name`; always written.";
        };
        repository_root = unset types.str "The root each repository's view is kept under, an absolute path or one beginning with `~/`; the runner's `~/Code` where unset.";
        fetch_interval = unset interval "How often every repository is fetched; the runner's `2m` where unset.";
        code_workspaces = unset types.bool "Whether each owner's views have a VS Code workspace; the runner's `false` where unset.";
        control_plane = unset controlPlane "`[control_plane]`, the control plane whose work the runner carries; none where unset.";
      };
    };
    default = { };
    description = "The runner's file, each key named as the file names it (README), rendered at evaluation.";
  };
}
