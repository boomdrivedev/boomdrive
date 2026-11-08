# services.boomdrive, the options both modules share (design Decision 8),
# given the flake's own `packages`: the runner's package, the flake's own
# for the host's system, so it is the path the family's cache holds
# whatever nixpkgs the host runs (design Decision 4); and `accounts` and
# `settings`, the runner's file under its own names (core.46). `accounts`
# names the host's accounts, each account's drives, each drive's machine
# and stream, and each drive's repositories by owner (record 0028), and the
# module writes the runner's `accounts` from it, every level always
# written (accounts.nix); an account and a drive hold no key the module
# does not declare, so one fails evaluation. A drive's stream and machine
# are given to Flume's module (drives.nix), never to the runner's file
# (core.5). `settings` holds the rest of the file, free-form TOML with the
# README's keys declared for their types (RFC 42); a key left unset is left
# out of the file, so the runner's own default holds and the two never
# drift; a top-level key the module does not declare passes to the file,
# and the runner judges it, but `accounts` and the flat `repositories` of
# before each fail evaluation, naming services.boomdrive.accounts. A key
# named for a credential fails evaluation, since the file is in the store,
# which every user reads (design Decision 12, core.4). Each tool of the set
# is installed by its own module, its package the set's unless the host
# gives its own (tools.nix; record 0023), so `tools` is gone, and setting
# it fails evaluation, naming what replaces it.
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

  # A drive's stream, which Flume's module is given (drives.nix): its name,
  # its room, its priority and each label's reserve, record 0028's keys,
  # with deneb's room and reserves by default. Each default label's
  # reserve is the module's own default, so a drive that sets another
  # label, or one of these, keeps the rest; a label's reserve has none of
  # its own. Any other key of a stream is Flume's own option,
  # services.flume.settings.streams.<stream>.
  stream =
    drive:
    types.submodule {
      options = {
        name = mkOption {
          type = types.nonEmptyStr;
          default = drive;
          defaultText = lib.literalMD "the drive's name";
          description = "The name of the drive's stream in Flume, which each of the drive's repositories runs in.";
        };
        agents = mkOption {
          type = types.ints.unsigned;
          default = 4;
          description = "The bound on the stream's agents running, Flume's `agents`; it holds each label's reserve.";
        };
        priority = unset types.int "The stream's priority, Flume's `priority`; Flume's own where unset.";
        labels = mkOption {
          type = types.attrsOf (
            types.submodule {
              options.reserve = mkOption {
                type = types.ints.unsigned;
                description = "The stream's agents the label is owed, Flume's `reserve`.";
              };
            }
          );
          default = { };
          description = "Each label's reserve of the stream's agents; `operation` 1 and `planning` 2 unless the drive gives its own.";
        };
      };
      config.labels = {
        operation.reserve = lib.mkDefault 1;
        planning.reserve = lib.mkDefault 2;
      };
    };

  # A drive, `[accounts.<account>.drives.<drive>]`: its machine, its
  # stream and its repositories by owner, and nothing else, so a key the
  # module does not declare fails evaluation, naming its option (core.5).
  drive = types.submodule (
    { name, ... }:
    {
      options = {
        machine = unset (types.strMatching "[a-z0-9][a-z0-9-]*") "The host the drive's work runs on, as crew's partitions' `machine` names one; this host where unset. A drive of another host gives Flume nothing, and this host keeps its repositories to read.";
        stream = mkOption {
          type = stream name;
          default = { };
          description = "The drive's stream in Flume.";
        };
        repositories = mkOption {
          type = types.attrsOf (types.listOf (types.strMatching "[A-Za-z0-9._-]+"));
          default = { };
          example = lib.literalExpression ''{ agentplot = [ "flume" "sluice" ]; }'';
          description = "Each repository the drive keeps, by its owner, the names of the owner's repositories in the order kept; a repository belongs to one drive. Written as their addresses, `<owner>/<name>`, owner by owner in the order of the owners' names.";
        };
      };
    }
  );

  # An account, `[accounts.<account>]`: its drives and nothing else.
  account = types.submodule {
    options.drives = mkOption {
      type = types.attrsOf drive;
      default = { };
      description = "Each of the account's drives by its name, which is given once on the host: one account's body of work. Always written.";
    };
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

    accounts = mkOption {
      type = types.attrsOf account;
      default = { };
      example = lib.literalExpression ''{ own.drives.home.repositories.agentplot = [ "flume" "sluice" ]; }'';
      description = "Each account the host serves by its name, what a runner enrolls with a control plane, each naming its drives and each drive its repositories by owner: the runner's `accounts`, always written.";
    };

    settings = mkOption {
      type = types.submodule {
        freeformType = toml;
        options = {
          repository_root = unset types.str "The root each repository's view is kept under, an absolute path or one beginning with `~/`; the runner's `~/Code` where unset.";
          fetch_interval = unset interval "How often every repository is fetched; the runner's `2m` where unset.";
          code_workspaces = unset types.bool "Whether each drive has a VS Code workspace at the repository root; the runner's `false` where unset.";
          control_plane = unset controlPlane "`[control_plane]`, the control plane whose work the runner carries; none where unset.";
        };
      };
      default = { };
      description = "The runner's file but its `accounts`, each key named as the file names it (README), rendered at evaluation. It holds no token, key or credential.";
    };
  };

  config = lib.mkIf cfg.enable {
    assertions = [
      {
        assertion = found == [ ];
        message = "services.boomdrive.settings names a credential at ${lib.concatStringsSep ", " found}: the runner's file is in the store, which every user reads, so it holds no token or key. A partition's key is made on the host by `boomdrive enroll <label>`, its token on standard input.";
      }
      {
        assertion = !(cfg.settings ? repositories);
        message = "services.boomdrive.settings.repositories is the flat list of before: each repository is named under its drive, settings.accounts.<account>.drives.<drive>.repositories (record 0028).";
      }
      {
        assertion = !(cfg.settings ? accounts);
        message = "services.boomdrive.settings.accounts is set: the accounts are named in services.boomdrive.accounts alone, each drive's repositories by owner, services.boomdrive.accounts.<account>.drives.<drive>.repositories.<owner> (record 0028).";
      }
    ];
  };
}
