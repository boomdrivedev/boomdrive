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
# README's keys declared for their types (RFC 42). Under crew, with crew's
# module on, `control_plane.crew` is written from crew's drives whose
# `machine` is `services.crew.host`, in label order, a table written out
# winning whole (record 0023; record 0032; the change
# crews-partitions-give-the-runner-its-crew-table).
# Any other key left unset is left out of the file, so the runner's own
# default holds and the two never drift; a top-level key the module does
# not declare passes to the file, and the runner judges it, but `accounts`
# and the flat `repositories` of before each fail evaluation, naming
# services.boomdrive.accounts. A key named for a credential fails
# evaluation, since the file is in the store, which every user reads
# (design Decision 12, core.4). `settings.forge` names the Boomdrive App by
# its id and the path of its key, never the key, and a key file in the
# store fails evaluation (record 0007 as amended on 2026-11-11). Each tool of the set is installed by its
# own module, its package the set's unless the host gives its own
# (tools.nix; record 0023), so `tools` is gone, and setting it fails
# evaluation, naming what replaces it.
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
  # with that record's room and reserves by default. Each default label's
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
        machine = unset (types.strMatching "[a-z0-9][a-z0-9-]*") "The host the drive's work runs on, as crew's drives' `machine` names one; this host where unset. A drive of another host gives Flume nothing, and this host keeps its repositories to read.";
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

  # crew's module as the host sets it, off where the system declares none,
  # as options.nix evaluated alone does.
  crew = config.services.crew or { enable = false; };
  # The runner's crew table written from crew's drives (record 0023;
  # record 0032): crew's host, and each drive whose `machine` is that host,
  # in the order of their labels, as crew's file lists them, by its label
  # and its state alone, since crew's implementation refuses any other key.
  # The runner's own key stays `partitions`. None where crew's module is
  # off.
  written =
    if crew.enable then
      {
        inherit (crew) host;
        partitions = map (drive: {
          inherit (drive) label;
          state_repository = drive.state;
        }) (lib.filter (drive: drive.machine == crew.host) (lib.attrValues crew.settings.drives));
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
          defaultText = lib.literalMD "Under crew, with crew's module on, `host` from `services.crew.host` and each of `services.crew.settings.drives` whose `machine` is that host, in label order, by its `label` and its `state` as `state_repository`; else `null`.";
          description = "`[control_plane.crew]`, handed whole to crew's implementation, whose keys are its own. Under crew, with crew's module on, the module writes it from crew's drives on this host; a table written out wins, whole.";
        };
        hosted = unset (types.attrsOf toml) "`[control_plane.hosted]`, handed whole to the hosted implementation, whose keys are its own. It holds no token or key: a partition's key is made on the host by `boomdrive enroll`.";
      };
    }
  );

  # `[forge]`, the Boomdrive App whose key the runner holds (record 0007 as
  # amended on 2026-11-11): its id, and the path of its key, a string, so a
  # Nix path value fails the type and a key is never copied into the store.
  # It holds nothing else.
  forgeApp = types.submodule {
    options = {
      app_id = mkOption {
        type = types.ints.positive;
        description = "The Boomdrive App's id, which each JWT the runner mints a token with names.";
      };
      key_file = mkOption {
        type = types.str;
        description = "The path of the Boomdrive App's private key on the host, absolute or beginning with `~/`, outside the Nix store and readable by its owner alone (mode 0600); the runner reads it at each mint, and the file holds where the key is, never the key (record 0007 as amended on 2026-11-11).";
      };
    };
  };

  # The App's key file where `settings.forge` names one, and whether it
  # lies in the store, which every user reads (core.4): the store of the
  # nix evaluating it, `builtins.storeDir`, or `/nix/store`, which the
  # runner refuses too.
  keyFile = if cfg.settings.forge == null then null else cfg.settings.forge.key_file;
  inStore =
    file:
    file != null
    && lib.any (store: file == store || lib.hasPrefix "${store}/" file) [
      builtins.storeDir
      "/nix/store"
    ];

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

  # Each drive of the accounts, and each address of theirs that is no
  # GitHub repository's, as the runner reads one: an attribute set's names,
  # the owners, take no type.
  drives = (import ./accounts.nix { inherit lib; }).drivesOf cfg.accounts;
  addresses = lib.concatMap (drive: drive.addresses) drives;
  noRepository = lib.filter (
    address: builtins.match "[A-Za-z0-9-]+/[A-Za-z0-9._-]+" address == null
  ) addresses;

  # What is given more than once, each by what gives it (design Decision
  # 6): a drive's name by its accounts, and an address by its drives, a
  # drive named again for each time it keeps the address.
  twice =
    pairs:
    lib.filterAttrs (_: givers: lib.length givers > 1) (
      lib.mapAttrs (_: map (pair: pair.by)) (lib.groupBy (pair: pair.what) pairs)
    );
  said = givers: lib.concatStringsSep " and " givers;
  drivesTwice = twice (
    map (drive: {
      what = drive.name;
      by = drive.account;
    }) drives
  );
  addressesTwice = twice (
    lib.concatMap (
      drive:
      map (address: {
        what = address;
        by = drive.name;
      }) drive.addresses
    ) drives
  );

  # Whether the runner's control plane is crew.
  crewPlane = (cfg.settings.control_plane.type or null) == "crew";
  # The machines crew's drives name, each once, in label order, read only
  # where crew's module is on.
  machines = lib.concatStringsSep ", " (
    lib.unique (map (drive: drive.machine) (lib.attrValues crew.settings.drives))
  );
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
          forge = unset forgeApp "`[forge]`, the Boomdrive App whose key the runner holds, and mints every tool's forge token from, on a host whose control plane does not hold the App: its `app_id` and its `key_file`, the path of its key outside the Nix store; none where unset, and every fetch goes through the host's own credential helper (record 0007 as amended on 2026-11-11).";
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
        assertion = !(inStore keyFile);
        message = "services.boomdrive.settings.forge.key_file lies in the Nix store, which every user reads: name the App's key by a path outside it, absolute or under ~/ (record 0007 as amended on 2026-11-11).";
      }
      {
        assertion = !(cfg.settings ? repositories);
        message = "services.boomdrive.settings.repositories is the flat list of before: each repository is named under its drive and its owner, services.boomdrive.accounts.<account>.drives.<drive>.repositories.<owner> (record 0028).";
      }
      {
        assertion = !(cfg.settings ? accounts);
        message = "services.boomdrive.settings.accounts is set: the accounts are named in services.boomdrive.accounts alone, each drive's repositories by owner, services.boomdrive.accounts.<account>.drives.<drive>.repositories.<owner> (record 0028).";
      }
      {
        assertion = noRepository == [ ];
        message = "services.boomdrive.accounts keeps ${lib.concatStringsSep ", " noRepository}, which is no GitHub repository's address, `<owner>/<name>`: each drive's repositories are named by their owner, repositories.<owner> = [ <names> ].";
      }
      {
        assertion = drivesTwice == { };
        message = "services.boomdrive.accounts names ${
          lib.concatStringsSep "; " (
            lib.mapAttrsToList (
              drive: accounts: "the drive ${drive} under the accounts ${said accounts}"
            ) drivesTwice
          )
        }: a drive's name is given once on the host (record 0028).";
      }
      {
        assertion = addressesTwice == { };
        message = "services.boomdrive.accounts keeps ${
          lib.concatStringsSep "; " (
            lib.mapAttrsToList (address: drives: "${address} in the drives ${said drives}") addressesTwice
          )
        }: a repository belongs to one drive, which names it once (record 0028).";
      }
      {
        assertion = !(crewPlane && cfg.settings.control_plane.crew == null);
        message = "services.boomdrive.settings.control_plane.type is crew, and no services.boomdrive.settings.control_plane.crew is written: write that table, or turn on crew's module, services.crew, with its host, its drives and its hosts, and the module writes it from crew's drives on this host (record 0023).";
      }
      {
        assertion =
          !(
            crewPlane
            && written != null
            && cfg.settings.control_plane.crew == written
            && written.partitions == [ ]
          );
        message = lib.optionalString crew.enable "services.boomdrive.settings.control_plane.crew is written from crew's drives whose machine is services.crew.host, ${crew.host}, and no drive's is: they name ${machines}. Set services.crew.host to this host's machine, or give one of services.crew.settings.drives this host as its machine (record 0023).";
      }
    ];
  };
}
