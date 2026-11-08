# services.boomdrive, the options both modules share (design Decision 8),
# given the flake's own `packages`: the runner's package, the flake's own
# for the host's system, so it is the path the family's cache holds
# whatever nixpkgs the host runs (design Decision 4); and `settings`, the runner's file under its own names
# (core.46), free-form TOML with the README's keys declared for their types
# (RFC 42). A key left unset is left out of the file, so the runner's own
# default holds and the two never drift; a key the module does not declare
# passes to the file, and the runner judges it. A key named for a
# credential fails evaluation, since the file is in the store, which every
# user reads (design Decision 12, core.4). Each tool of the set is
# installed by its own module, its package the set's unless the host gives
# its own (tools.nix; record 0023), so `tools` is gone, and setting it
# fails evaluation, naming what replaces it.
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
  toml = (pkgs.formats.toml { }).type;

  set = import ./set.nix {
    inherit lib packages;
    inherit (pkgs.stdenv.hostPlatform) system;
  };
  setText =
    name: lib.literalExpression "boomdrive.packages.\${pkgs.stdenv.hostPlatform.system}.${name}";

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

  # Each key of `value`, at any depth, named for a credential, by its path.
  words = [
    "token"
    "key"
    "secret"
    "password"
  ];
  credential = name: builtins.elem name words || lib.any (word: lib.hasSuffix "_${word}" name) words;
  credentials =
    named: value:
    if builtins.isAttrs value then
      lib.concatLists (
        lib.mapAttrsToList (
          name: inner:
          let
            at = if named == "" then name else "${named}.${name}";
          in
          lib.optional (credential name) at ++ credentials at inner
        ) value
      )
    else if builtins.isList value then
      lib.concatLists (lib.imap0 (i: credentials "${named}[${toString i}]") value)
    else
      [ ];
  found = credentials "" cfg.settings;
in
{
  imports = [
    (lib.mkRemovedOptionModule
      [
        "services"
        "boomdrive"
        "tools"
      ]
      "Each tool's own module installs its package, the set's unless the host gives its own: set services.flume.package or services.sluice.package to the host's own package, or to null for none, or set services.flume.enable or services.sluice.enable to false to keep that tool's file the host's own."
    )
  ];

  options.services.boomdrive = {
    enable = lib.mkEnableOption "the runner, `boomdrive run`, as the host's service";

    package = mkOption {
      type = types.package;
      default = set.boomdrive;
      defaultText = setText "boomdrive";
      description = "The runner.";
    };

    settings = mkOption {
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
      description = "The runner's file, each key named as the file names it (README), rendered at evaluation. It holds no token, key or credential.";
    };
  };

  config = lib.mkIf cfg.enable {
    assertions = [
      {
        assertion = found == [ ];
        message = "services.boomdrive.settings names a credential at ${lib.concatStringsSep ", " found}: the runner's file is in the store, which every user reads, so it holds no token or key. A partition's key is made on the host by `boomdrive enroll <label>`, its token on standard input.";
      }
    ];
  };
}
