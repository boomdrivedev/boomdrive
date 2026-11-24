# services.boomdrive, the options both modules share (design Decision 8),
# given the flake's own `packages`: the runner's package, the flake's own
# for the host's system, so it is the path the family's cache holds
# whatever nixpkgs the host runs (design Decision 4); and `accounts` and
# `settings`, the runner's file under its own names (core.46). `accounts`
# names the host's accounts, each account's crew state repository and its
# drives, each drive's machine and stream, and each drive's repositories
# by owner (records 0028 and 0032), and the module writes the runner's
# `accounts` from it, every level always written (accounts.nix); an
# account and a drive hold no key the module does not declare, so one
# fails evaluation. A drive's stream and machine are given to Flume's
# module (drives.nix), and with crew's module on its machine and its
# account's state repository to crew's drive of its name where crew's
# settings set it up (crew-drives.nix), never to the runner's file
# (core.5). `settings` holds the rest of the file, free-form TOML with the
# README's keys declared for their types (RFC 42). Under crew, with crew's
# module on, `control_plane.crew` is written from the accounts' drives on
# this host that crew's settings set up, under an account that names its
# state repository, in the accounts' order, a table written out winning
# whole (record 0023; record 0032 as amended on 2026-11-15 and 2026-11-18;
# the change crews-partitions-give-the-runner-its-crew-table). Under crew,
# an account with a drive crew's settings set up and no state repository,
# and a drive of crew's that no account names, each fail evaluation
# (record 0032 as amended on 2026-11-18); an account none of whose drives
# crew's settings set up needs no state repository.
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
# (tools.nix; record 0023), timber's among them (record 0023 as amended on
# 2026-11-15), so `tools` and `timber.package` are gone, and setting
# either fails evaluation, naming what replaces it (the change
# the-set-takes-timbers-own-module, design Decision 6). `enable` turns on
# the runner, and by default each tool's switch, so a host without it
# takes only the tools it turns on (record 0023 as amended on 2026-11-18;
# design Decisions 1 and 2 of the change
# boomdrives-tools-install-on-a-machine-without-the-runner).
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
  # its room, its priority and the agents each label is owed, record
  # 0028's keys, with that record's room and shares by default, each named
  # and ranged as Flume's own option is (flume record 0063). Each default
  # label's share is the module's own default, so a drive that sets
  # another label, or one of these, keeps the rest; a label's `owed` has
  # none of its own. Any other key of a stream, a label's `reserved` among
  # them, is Flume's own option, services.flume.settings.streams.<stream>.
  # A label's `reserve`, Flume's word before `owed`, is no option of the
  # drive's: setting it fails evaluation, naming `owed` (drives.nix).
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
          type = types.ints.positive;
          default = 4;
          description = "The stream's agents running at once, Flume's `agents`, a whole number from 1; it holds what its labels are owed together.";
        };
        priority = unset types.int "The stream's priority, Flume's `priority`; Flume's own where unset.";
        labels = mkOption {
          type = types.attrsOf (
            types.submodule {
              options = {
                owed = mkOption {
                  type = types.ints.unsigned;
                  description = "The stream's agents the label's places are first in line for, Flume's `owed`, a whole number of agents from 0, lent while the label has nothing waiting.";
                };
                reserve = mkOption {
                  type = types.nullOr types.unspecified;
                  default = null;
                  visible = false;
                  description = "Flume's word before `owed`, which fails evaluation, naming `owed`.";
                };
              };
            }
          );
          default = { };
          description = "The agents of the stream each label is owed; `operation` 1 and `planning` 2 unless the drive gives its own.";
        };
      };
      config.labels = {
        operation.owed = lib.mkDefault 1;
        planning.owed = lib.mkDefault 2;
      };
    };

  # A drive, `[accounts.<account>.drives.<drive>]`: its machine, its
  # stream and its repositories by owner, and nothing else, so a key the
  # module does not declare fails evaluation, naming its option (core.5).
  drive = types.submodule (
    { name, ... }:
    {
      options = {
        machine = unset (types.strMatching "[a-z0-9][a-z0-9-]*") "The host the drive's work runs on, as `hostname -s` names it; this host, `services.crew.host`, where unset. A drive of another host gives Flume nothing, and this host keeps its repositories to read. With crew's module on, crew's drive of this name, where crew's settings set it up, is given it as its `machine` (record 0032 as amended on 2026-11-18).";
        stream = mkOption {
          type = stream name;
          default = { };
          description = "The drive's stream in Flume.";
        };
        repositories = mkOption {
          type = types.attrsOf (types.listOf (types.strMatching "[A-Za-z0-9._-]+"));
          default = { };
          example = lib.literalExpression ''{ boomdrivedev = [ "flume" "sluice" ]; }'';
          description = "Each repository the drive keeps, by its owner, the names of the owner's repositories in the order kept; a repository belongs to one drive. Written as their addresses, `<owner>/<name>`, owner by owner in the order of the owners' names.";
        };
      };
    }
  );

  # An account, `[accounts.<account>]`: its drives and nothing else, and
  # beside them the account's crew state repository, which is no key of
  # the runner's file (core.5; record 0028; record 0032).
  account = types.submodule {
    options = {
      drives = mkOption {
        type = types.attrsOf drive;
        default = { };
        description = "Each of the account's drives by its name, which is given once on the host: one account's body of work. Always written.";
      };
      control_plane =
        unset
          (types.enum [
            "crew"
            "hosted"
          ])
          "The control plane that serves the account, `crew` or `hosted`, whose table `settings.control_plane` holds: the runner's `accounts.<account>.control_plane`, each account naming its own (record 0054); none where unset, and no control plane serves the account.";
      state_repository = unset (types.strMatching "[A-Za-z0-9-]+/[A-Za-z0-9._-]+") "The account's crew state repository, `<owner>/<name>`, whose branch `<drive>/main` holds each of its drives' plan and run record: under crew, named here once, and given to each drive of the account that crew's settings set up and to the runner's crew table; none where unset, and the account's drives give crew nothing.";
    };
  };

  # crew's module as the host sets it, off where the system declares none,
  # as options.nix evaluated alone does.
  crew = config.services.crew or { enable = false; };
  # The runner's crew table written from the accounts (record 0023; record
  # 0032 as amended on 2026-11-15 and 2026-11-18): crew's host, and each
  # drive of this host, naming no machine or that host, that crew's
  # settings set up, under an account that names crew as its control plane
  # and its state repository, in the accounts' order, as the runner's file
  # lists
  # them, by its name as its label and its account's state repository
  # alone, since crew's implementation refuses any other key and reads each
  # partition by its label. The runner's own key stays `partitions`. None
  # where crew's module is off.
  written =
    if crew.enable then
      {
        inherit (crew) host;
        partitions =
          map
            (drive: {
              label = drive.name;
              inherit (drive) state_repository;
            })
            (
              lib.filter (
                drive: drive.control_plane == "crew" && drive.state_repository != null && crewRuns drive
              ) (hereOf crew.host drives)
            );
      }
    else
      null;

  controlPlane = types.submodule (_: {
    freeformType = toml;
    options = {
      poll_interval = unset interval "How often a control pass is made; the runner's `1m` where unset.";
      crew = mkOption {
        type = types.nullOr (types.attrsOf toml);
        default = if crewPlane then written else null;
        defaultText = lib.literalMD "Where an account names crew, with crew's module on, `host` from `services.crew.host` and, for each drive of `services.boomdrive.accounts` naming no `machine` or that host that `services.crew.settings.drives` sets up, under an account that names crew and its `state_repository`, the drive's name as `label` and its account's `state_repository`, accounts and each account's drives in the order of their names; else `null`.";
        description = "`[control_plane.crew]`, handed whole to crew's implementation, whose keys are its own. Where an account names crew, with crew's module on, the module writes it from the drives on this host that crew's settings set up, under an account that names crew and its `state_repository`; a table written out wins, whole.";
      };
      hosted = unset (types.attrsOf toml) "`[control_plane.hosted]`, handed whole to the hosted implementation, whose keys are its own. It holds no token or key: a partition's key is made on the host by `boomdrive enroll`.";
    };
  });

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
  inherit (import ./accounts.nix { inherit lib; }) drivesOf hereOf;
  drives = drivesOf cfg.accounts;
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

  # The control planes the accounts name, each once, and whether crew is
  # among them (record 0054).
  planes = lib.unique (
    lib.filter (plane: plane != null) (
      lib.mapAttrsToList (_: account: account.control_plane) cfg.accounts
    )
  );
  crewPlane = lib.elem "crew" planes;
  # Whether crew's settings set up the drive, crew's drives' names being
  # the host's settings' alone (crew-drives.nix); read only where crew's
  # module is on (record 0032 as amended on 2026-11-18).
  crewRuns = drive: crew.settings.drives ? ${drive.name};
  # The machines the drives crew's settings set up, of the accounts that
  # name a state repository, name, each once, in the accounts' order, or
  # that no such drive is set up; read only where crew's module is on.
  crewMachines = lib.unique (
    map (drive: drive.machine) (
      lib.filter (drive: drive.state_repository != null && crewRuns drive) drives
    )
  );
  machines =
    if crewMachines == [ ] then
      "crew's settings set up no drive of the accounts under a state repository"
    else
      "they name ${lib.concatStringsSep ", " crewMachines}";

  # Under crew, the accounts that name no state repository and hold a
  # drive crew's settings set up, and the drives of crew's that no account
  # names, read only where crew's module is on (record 0032 as amended on
  # 2026-11-18).
  stateless = lib.attrNames (
    lib.filterAttrs (
      _: account:
      account.control_plane == "crew"
      && account.state_repository == null
      && lib.any (drive: crew.settings.drives ? ${drive}) (lib.attrNames account.drives)
    ) cfg.accounts
  );
  unnamed = lib.subtractLists (map (drive: drive.name) drives) (lib.attrNames crew.settings.drives);
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
    (lib.mkRemovedOptionModule
      [
        "services"
        "boomdrive"
        "timber"
        "package"
      ]
      "timber's own module installs timber, the set's unless the host gives its own: set services.timber.package to the host's own package, or to null for none, or set services.timber.enable to false to keep timber's file, ~/.config/timber/config.toml, the host's own."
    )
  ];

  options.services.boomdrive = {
    enable = lib.mkEnableOption "the runner, `boomdrive run`, as the host's service, and by default each tool of the set: Flume's, sluice's and timber's modules, and each switch of the Boomdrive module's, `herdr`, `ghostty`, `launcher`, `millpond`, `portless` and `worktrunk`, and, with crew's module on, `pendingyou`, `plannotator` and `plannotator-tui`; without it, a host turns on each tool it wants by its own switch";

    package = mkOption {
      type = types.package;
      default = set.boomdrive;
      defaultText = setText "boomdrive";
      description = "The runner.";
    };

    accounts = mkOption {
      type = types.attrsOf account;
      default = { };
      example = lib.literalExpression ''{ own.drives.home.repositories.boomdrivedev = [ "flume" "sluice" ]; }'';
      description = "Each account the host serves by its name, what a runner enrolls with a control plane, each naming its drives and each drive its repositories by owner: the runner's `accounts`, always written.";
    };

    settings = mkOption {
      type = types.submodule {
        freeformType = toml;
        options = {
          repository_root = unset types.str "The root each repository's view is kept under, an absolute path or one beginning with `~/`; the runner's `~/Code` where unset.";
          fetch_interval = unset interval "How often every repository is fetched; the runner's `2m` where unset.";
          code_workspaces = unset types.bool "Whether each drive has a VS Code workspace at the repository root; the runner's `false` where unset.";
          control_plane = mkOption {
            type = types.nullOr controlPlane;
            default = if planes == [ ] then null else { };
            defaultText = lib.literalMD "`{ }` where an account names a control plane, else `null`.";
            description = "`[control_plane]`, the table of each control plane the accounts name and how often each part makes a control pass; none where no account names one (record 0054).";
          };
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
        assertion = !((cfg.settings.control_plane or null) != null && cfg.settings.control_plane ? type);
        message = "services.boomdrive.settings.control_plane.type is set: each account names its control plane, services.boomdrive.accounts.<account>.control_plane, and settings.control_plane holds the table of each (record 0054).";
      }
      {
        assertion = !(crewPlane && cfg.settings.control_plane.crew == null);
        message = "an account of services.boomdrive.accounts names control_plane = crew, and no services.boomdrive.settings.control_plane.crew is written: write that table, or turn on crew's module, services.crew, with its host, its drives' own keys and its hosts, and the module writes it from the drives of services.boomdrive.accounts on this host (records 0023 and 0032).";
      }
      {
        assertion =
          !(
            crewPlane
            && written != null
            && cfg.settings.control_plane.crew == written
            && written.partitions == [ ]
          );
        message = lib.optionalString crew.enable "services.boomdrive.settings.control_plane.crew is written from the drives of services.boomdrive.accounts that services.crew.settings.drives sets up, whose machine is services.crew.host, ${crew.host}, or that name none, under an account that names its state_repository, and no drive is: ${machines}. Set services.crew.host to this host's machine, or give a drive crew's settings set up this host as its machine (record 0032).";
      }
      {
        assertion = !(crewPlane && crew.enable && stateless != [ ]);
        message = "an account of services.boomdrive.accounts names control_plane = crew, and ${
          lib.concatMapStringsSep ", " (
            account: "services.boomdrive.accounts.${account}.state_repository"
          ) stateless
        } ${
          if lib.length stateless == 1 then "is" else "are"
        } not set: under crew, an account with a drive crew's settings set up names its crew state repository, which the module gives that drive and the runner's crew table (records 0028 and 0032 as amended on 2026-11-18).";
      }
      {
        assertion = !(crewPlane && crew.enable && unnamed != [ ]);
        message = lib.optionalString crew.enable "services.crew.settings.drives names ${lib.concatStringsSep ", " unnamed}, which no account of services.boomdrive.accounts names: under crew, each drive is named once, under its account in services.boomdrive.accounts.<account>.drives.<drive>, with its machine where it runs on another host, and the module gives crew its machine and its account's state_repository (record 0032).";
      }
    ];
  };
}
