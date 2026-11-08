# services.boomdrive, the options both modules share (design Decision 8),
# given the flake's own `packages`: the runner's package, the flake's own
# for the host's system, so it is the path the family's cache holds
# whatever nixpkgs the host runs (design Decision 4); and `settings`, the runner's file under its own names
# (core.46), free-form TOML with the README's keys declared for their types
# (RFC 42). Its `accounts` name the host's accounts, each account's drives
# and each drive's repositories (record 0028), every level always written;
# an account and a drive hold no key the runner does not read, so one fails
# evaluation, and a flat `repositories` of before fails it too. Under crew,
# with crew's module on, `control_plane.crew` is written from crew's
# partitions whose `machine` is `services.crew.host`, a table written out
# winning whole (record 0023; the change
# crews-partitions-give-the-runner-its-crew-table). Any other
# key left unset is left out of the file, so the runner's own default holds
# and the two never drift; a top-level key the module does not declare
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

  # A drive, `[accounts.<account>.drives.<drive>]`: its repositories and
  # nothing else, so a stream or a key the runner does not read fails
  # evaluation, naming its option (core.5).
  drive = types.submodule {
    options.repositories = mkOption {
      type = types.listOf (types.strMatching "[A-Za-z0-9-]+/[A-Za-z0-9._-]+");
      default = [ ];
      description = "Each repository the drive keeps, by its GitHub address, `owner/name`, in the order kept; a repository belongs to one drive. Always written.";
    };
  };

  # An account, `[accounts.<account>]`: its drives and nothing else.
  account = types.submodule {
    options.drives = mkOption {
      type = types.attrsOf drive;
      default = { };
      description = "Each of the account's drives by its name, which is given once on the host: one account's body of work. Always written.";
    };
  };

  # crew's module as the host sets it, off where the system declares none,
  # as options.nix evaluated alone does.
  crew = config.services.crew or { enable = false; };
  # The runner's crew table written from crew's partitions (record 0023):
  # crew's host, and each partition whose `machine` is that host, in crew's
  # order, by its label and its state alone, since crew's implementation
  # refuses any other key. None where crew's module is off.
  written =
    if crew.enable then
      {
        inherit (crew) host;
        partitions = map (p: {
          inherit (p) label;
          state_repository = p.state;
        }) (lib.filter (p: p.machine == crew.host) crew.settings.partitions);
      }
    else
      null;

  controlPlane = types.submodule (
    { config, ... }:
    {
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
        crew = mkOption {
          type = types.nullOr (types.attrsOf toml);
          default = if config.type == "crew" then written else null;
          defaultText = lib.literalMD "Under crew, with crew's module on, `host` from `services.crew.host` and each of `services.crew.settings.partitions` whose `machine` is that host, by its `label` and its `state` as `state_repository`; else `null`.";
          description = "`[control_plane.crew]`, handed whole to crew's implementation, whose keys are its own. Under crew, with crew's module on, the module writes it from crew's partitions on this host; a table written out wins, whole.";
        };
        hosted = unset (types.attrsOf toml) "`[control_plane.hosted]`, handed whole to the hosted implementation, whose keys are its own. It holds no token or key: a partition's key is made on the host by `boomdrive enroll`.";
      };
    }
  );

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

  # Whether the runner's control plane is crew.
  crewPlane = (cfg.settings.control_plane.type or null) == "crew";
  # The machines crew's partitions name, each once, read only where crew's
  # module is on.
  machines = lib.concatStringsSep ", " (lib.unique (map (p: p.machine) crew.settings.partitions));
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
          accounts = mkOption {
            type = types.attrsOf account;
            default = { };
            description = "Each account the host serves by its name, what a runner enrolls with a control plane, each naming its drives and each drive its repositories; always written.";
          };
          repository_root = unset types.str "The root each repository's view is kept under, an absolute path or one beginning with `~/`; the runner's `~/Code` where unset.";
          fetch_interval = unset interval "How often every repository is fetched; the runner's `2m` where unset.";
          code_workspaces = unset types.bool "Whether each drive has a VS Code workspace at the repository root; the runner's `false` where unset.";
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
      {
        assertion = !(cfg.settings ? repositories);
        message = "services.boomdrive.settings.repositories is the flat list of before: each repository is named under its drive, settings.accounts.<account>.drives.<drive>.repositories (record 0028).";
      }
      {
        assertion = !(crewPlane && cfg.settings.control_plane.crew == null);
        message = "services.boomdrive.settings.control_plane.type is crew, and no services.boomdrive.settings.control_plane.crew is written: write that table, or turn on crew's module, services.crew, with its host and its partitions, and the module writes it from crew's partitions on this host (record 0023).";
      }
      {
        assertion =
          !(
            crewPlane
            && written != null
            && cfg.settings.control_plane.crew == written
            && written.partitions == [ ]
          );
        message = lib.optionalString crew.enable "services.boomdrive.settings.control_plane.crew is written from crew's partitions whose machine is services.crew.host, ${crew.host}, and no partition's is: they name ${machines}. Set services.crew.host to this host's machine, or give one of services.crew.settings.partitions this host as its machine (record 0023).";
      }
    ];
  };
}
