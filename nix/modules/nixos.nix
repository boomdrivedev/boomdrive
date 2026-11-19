# The runner's NixOS module, in nixosModules.default: services.boomdrive as
# a systemd user unit of services.boomdrive.user, who lingers, so the
# runner runs from boot on a host no one logs into (design Decision 10),
# with the runner in the system profile and the service's PATH the
# profiles' (design Decision 11). Each tool of the set is installed by its
# own module, which flake.nix imports beside this one; tools.nix sets
# their options, and each tool's user, Flume's, sluice's, crew's and
# timber's, is the set's user, `services.boomdrive.user`, wherever the
# host names it, with the runner or without, unless the host names the
# tool's own (record 0023 and its amendments of 2026-11-15 and
# 2026-11-18); drives.nix gives Flume's module each drive's
# stream (record 0028), sources.nix declares crew's source to Flume's
# module (record 0034), crew-drives.nix gives crew's module each drive's
# machine and its account's state repository (record 0032), and
# collector.nix turns Flume's and sluice's telemetry on to the collector
# the host names for its Claude Code sessions, with the runner on (record
# 0013 as amended on 2026-11-15). The unit runs `boomdrive run --config`
# with the file rendered from `accounts` and `settings` (design Decision
# 9). NixOS restarts no changed user unit, so the user's activation links
# the file at the runner's own path, writes Pending You's folder where
# Pending You is turned on (record 0044, point 7), links the herdr
# plugins apps.nix lists with herdr's own command where herdr is turned on
# (record 0023), runs activate.sh's `claude` with what sessions.nix gives
# it where Flume, with a package, is on, after Flume's own user snippet,
# which installs the set's marketplace and the plugins of the tools the
# host installs, with Pending You's server, into every Claude Code home of
# Flume's accounts and the default home by Claude Code's own commands
# (record 0044, point 3, and its amendments of 2026-11-18; design Decision
# 3 of the change every-claude-code-session-has-boomdrives-plugins), then
# starts the unit where it is not running and restarts it where the
# running runner's command line is not the unit's; any other user's
# activation does nothing. Without the runner, the same
# user's activation writes the folder and links the plugins alone: the
# user is the set's, `services.boomdrive.user`, who does not linger, and
# no unit is declared; where such a step runs and no user is named,
# evaluation fails, naming it (record 0023 as amended on 2026-11-18;
# design Decision 4 of the change
# boomdrives-tools-install-on-a-machine-without-the-runner). Beside `user`, the module declares `dataDir`,
# the runner's data folder by the runner's own rule (data-dir.nix) from
# that user's home and the unit's own `XDG_DATA_HOME`, and clones.nix
# declares `cloneDirs`, each kept repository's clone folder in it: each
# read-only, read wherever a host names the folder or a clone's, and set
# by none (record 0036 as amended on 2026-11-11). With crew's module on,
# the module gives crew's kept repositories,
# `services.crew.settings.repositories`, one entry for each address of
# `cloneDirs`, its `dir` the clone's folder and its `checkout` the
# runner's view under that user's home (kept.nix), at the module's own
# priority, so a host writes none (record 0036 as amended a third time on
# 2026-11-11). Where `services.boomdrive.claude-code.telemetry` names a
# collector, `/etc` holds Claude Code's managed telemetry document,
# claude-code.nix's, which every Claude Code session on the host reads
# (record 0013 as amended on 2026-11-12). Wherever Flume is on, with the
# runner or without, `/etc` holds Claude Code's managed transcripts
# document beside it, claude-code.nix's, which keeps every session's
# transcript `services.boomdrive.claude-code.cleanupPeriodDays` days
# (record 0046).
{ packages }:
{
  config,
  lib,
  options,
  pkgs,
  utils,
  ...
}:
let
  cfg = config.services.boomdrive;
  render = import ./toml.nix { inherit lib; };
  inherit (import ./accounts.nix { inherit lib; }) fileOf;
  dataDirOf = import ./data-dir.nix { inherit lib; };
  keptOf = import ./kept.nix { inherit lib; };
  claudeCode = import ./claude-code.nix { inherit lib; };
  sessions = import ./sessions.nix { inherit lib packages; };
  telemetry = cfg.claude-code.telemetry;
  file = pkgs.writeText "boomdrive-config.toml" (render (fileOf cfg));

  # herdr's own link of each plugin, where herdr is turned on and a herdr
  # and a plugin are given (apps.nix; record 0023 as amended on
  # 2026-11-18).
  linking = cfg.herdr.enable && cfg.herdr.package != null && cfg.herdr.plugins != [ ];
  plugins = lib.escapeShellArgs (
    [
      "plugins"
      (lib.getExe' cfg.herdr.package "herdr")
    ]
    ++ map toString cfg.herdr.plugins
  );

  inherit (cfg) user;
  named = options.services.boomdrive.user.isDefined;
  account = config.users.users.${user};
  inherit (account) home;

  # Pending You's folder, where Pending You is turned on and its package is
  # given (companions.nix; record 0044, point 7, as amended on
  # 2026-11-18), written before the plugins are linked.
  writingFolder = cfg.pendingyou.enable && cfg.pendingyou.package != null;
  folder = pkgs.callPackage ./pendingyou-folder.nix {
    inherit home;
    inherit (cfg.pendingyou) package;
  };

  # The set's user's steps before `keep`, in order: the runner's link with
  # the runner, Pending You's folder, then herdr's links of the plugins,
  # each wherever its tool is on, with the runner or without (record 0023
  # as amended on 2026-11-18; design Decision 4 of the change
  # boomdrives-tools-install-on-a-machine-without-the-runner); then, with
  # the runner and Flume, the install into Claude Code's homes (design
  # Decision 3 of the change every-claude-code-session-has-boomdrives-plugins).
  steps =
    lib.optional cfg.enable "${pkgs.runtimeShell} ${./activate.sh} link ${file} || true"
    ++ lib.optional writingFolder "${pkgs.runtimeShell} ${./activate.sh} pendingyou ${folder} ${lib.escapeShellArg folder.folder} || true"
    ++ lib.optional linking "${pkgs.runtimeShell} ${./activate.sh} ${plugins} || true"
    ++ lib.optional installing "PATH=${lib.escapeShellArg claudePath} ${pkgs.runtimeShell} ${./activate.sh} ${claude} || true";

  # The install into Claude Code's homes, where the runner and Flume, with
  # a package, are on, on the service's PATH and then ~/.local/bin, where
  # Claude Code's own installer puts `claude`.
  installing = sessions.runs config;
  claude = lib.escapeShellArgs (
    [ "claude" ]
    ++ sessions.argumentsOf {
      inherit config pkgs folder;
      writing = writingFolder;
    }
  );
  claudePath = lib.concatStringsSep ":" (path ++ [ "${home}/.local/bin" ]);

  # The switches whose steps run as the set's user, which a failure names
  # where no user is.
  needing =
    lib.optional cfg.enable "services.boomdrive.enable"
    ++ lib.optional writingFolder "services.boomdrive.pendingyou.enable"
    ++ lib.optional linking "services.boomdrive.herdr.enable";

  # `keep`, after the steps, where the runner is on.
  keeping = lib.optionalString cfg.enable (
    "  PATH=${keepPath}:$PATH \\\n"
    + "    ${pkgs.runtimeShell} ${./activate.sh} keep ${lib.escapeShellArgs ([ user ] ++ command)}\n"
  );

  # The unit's command line, which the activation compares the running
  # runner's with.
  command = [
    "${cfg.package}/bin/boomdrive"
    "run"
    "--config"
    "${file}"
  ];

  # The profiles: never a package's own folder, so the runner, what Flume
  # launches and a person's shell run one copy of each tool.
  path = [
    "/etc/profiles/per-user/${user}/bin"
    "${home}/.nix-profile/bin"
    "${home}/.local/state/nix/profile/bin"
    "/run/wrappers/bin"
    "/run/current-system/sw/bin"
    "/nix/var/nix/profiles/default/bin"
  ];

  # What `keep` runs, which the user's activation does not have.
  keepPath = lib.makeBinPath [
    pkgs.procps
    config.systemd.package
  ];

  hosted = (cfg.settings.control_plane.type or null) == "hosted";
  systemd = config.systemd.package.version;
in
{
  imports = [
    (import ./options.nix { inherit packages; })
    (import ./tools.nix { inherit packages; })
    (import ./apps.nix { inherit packages; })
    (import ./companions.nix { inherit packages; })
    ./drives.nix
    ./clones.nix
    ./sources.nix
    ./crew-drives.nix
    ./collector.nix
    claudeCode.module
    sessions.module
  ];

  options.services.boomdrive.user = lib.mkOption {
    type = lib.types.str;
    example = "alice";
    description = "The user the set is set up for, with the runner or without: the one whose systemd user manager runs the runner, and who lingers, where `services.boomdrive.enable` is on, and as whom herdr's plugins are linked and Pending You's folder written wherever `services.boomdrive.herdr.enable` or `services.boomdrive.pendingyou.enable` is on. Without the runner, nothing else runs as that user.";
  };

  # The unit's own XDG_DATA_HOME, or null where the unit has none or the
  # runner is not enabled; the user's manager gives the unit HOME, the
  # user's home (the change the-set-names-the-runners-data-folder, design
  # Decisions 1 and 3).
  options.services.boomdrive.dataDir = lib.mkOption {
    type = lib.types.path;
    readOnly = true;
    default = dataDirOf {
      inherit home;
      dataHome = config.systemd.user.services.boomdrive.environment.XDG_DATA_HOME or null;
    };
    defaultText = lib.literalMD ''
      `$XDG_DATA_HOME/boomdrive` where the runner's unit is given an absolute
      `XDG_DATA_HOME` in `systemd.user.services.boomdrive.environment`, else
      `.local/share/boomdrive` under the home of `services.boomdrive.user`,
      `users.users.''${config.services.boomdrive.user}.home`
    '';
    description = ''
      The runner's data folder, where it keeps its clones, its lock and its
      report, by the runner's own rule: `$XDG_DATA_HOME/boomdrive` where the
      unit's own environment gives an absolute `XDG_DATA_HOME`, else
      `~/.local/share/boomdrive` under the home of `services.boomdrive.user`.
      It is read-only: a host reads it wherever it names the folder, a
      clone's folder in it being `services.boomdrive.cloneDirs`'s, and sets
      it nowhere. An `XDG_DATA_HOME` the unit takes
      from outside its declaration, such as the user manager's
      `environment.d`, is not seen: a host that moves the runner's data
      gives the unit its own `XDG_DATA_HOME` in
      `systemd.user.services.boomdrive.environment`.
    '';
  };

  config = lib.mkMerge [
    # Claude Code's managed transcripts document, wherever Flume is on,
    # whether or not the runner is: copied, not linked into the store, as
    # the telemetry document is (the change
    # the-set-keeps-claude-codes-transcripts-a-year, design Decisions 2
    # and 5). Where Flume is off, no entry is declared, and NixOS removes
    # the file it wrote.
    {
      environment.etc = lib.mkIf config.services.flume.enable {
        "${claudeCode.nixosFolder}/${claudeCode.transcriptsName}" = {
          text = builtins.toJSON (claudeCode.transcriptsOf cfg.claude-code.cleanupPeriodDays);
          mode = "0644";
        };
      };
    }

    # The set's user's steps, in one user activation script, wherever one of
    # them runs, with the runner or without (design Decision 4 of the change
    # boomdrives-tools-install-on-a-machine-without-the-runner): run by
    # every user's nixos-activation.service, inside which a job is never
    # waited for, with coreutils on its PATH and neither systemctl nor ps,
    # and by that user's alone. A link that fails says why and stops
    # nothing else: the unit reads its own --config either way.
    (lib.mkIf (steps != [ ]) {
      assertions = [
        # Whether the user is named is read before users.users, so a host
        # that names none is told which option to set. users.users.<user>
        # is there either way where the runner is on, since this module
        # makes the user linger: a user only the module names is neither
        # kind.
        {
          assertion = named && (account.isNormalUser || account.isSystemUser);
          message =
            if named then
              "services.boomdrive.user is ${user}, whom users.users.${user} does not declare: declare the user the set is set up for."
            else
              "services.boomdrive.user is not set, and ${lib.concatStringsSep " and " needing} run steps as the set's user: set services.boomdrive.user to the user the set is set up for.";
        }
      ];

      # After Flume's own snippet where the install into Claude Code's
      # homes runs, so Flume's status reads the switch's file.
      system.userActivationScripts.boomdrive = {
        text = ''
          if [ "$(id -un)" = ${lib.escapeShellArg user} ]; then
          ${lib.concatMapStrings (step: "  ${step}\n") steps}${keeping}fi
        '';
        deps = lib.optional installing "flume";
      };
    })

    # Each tool's module links its file for the set's user, unless the host
    # names another, wherever the host names the set's user, with the runner
    # or without; where it names none, each tool's module asks for its own
    # (record 0023 as amended on 2026-11-18; design Decision 4 of the change
    # boomdrives-tools-install-on-a-machine-without-the-runner).
    (lib.mkIf named {
      services = {
        flume.user = lib.mkDefault user;
        sluice.user = lib.mkDefault user;
        crew.user = lib.mkDefault user;
        timber.user = lib.mkDefault user;
      };
    })

    (lib.mkIf cfg.enable {
      assertions = [
        {
          assertion = !hosted || lib.versionAtLeast systemd "256";
          message = "services.boomdrive.settings names a hosted control plane, whose partition's key systemd-creds of systemd 256 or later keeps for the user, and this system's systemd is ${systemd}.";
        }
      ];

      environment = {
        systemPackages = [ cfg.package ];

        # Claude Code's managed telemetry document, where a collector is
        # named: copied, not linked into the store, as Claude Code's files
        # are (the change every-claude-code-session-exports-to-the-hosts-collector,
        # design Decisions 6 and 7). With none, no entry is declared, and
        # NixOS removes the file it wrote.
        etc = lib.mkIf (telemetry.collector != null) {
          "${claudeCode.nixosFolder}/${claudeCode.name}" = {
            text = builtins.toJSON (claudeCode.documentOf telemetry);
            mode = "0644";
          };
        };
      };

      # crew's kept repositories, where crew's module is on and the runner
      # keeps a repository (design Decisions 1, 4 and 6).
      services.crew.settings.repositories =
        lib.mkIf (config.services.crew.enable && cfg.cloneDirs != { })
          (keptOf {
            inherit (cfg) cloneDirs;
            inherit home;
            root = cfg.settings.repository_root;
          });

      users.users.${user}.linger = true;

      systemd.user.services.boomdrive = {
        description = "Boomdrive, the runner";
        wantedBy = [ "default.target" ];
        unitConfig.ConditionUser = user;
        # The profiles' PATH alone; the user's manager gives HOME and
        # XDG_RUNTIME_DIR.
        enableDefaultPath = false;
        environment.PATH = lib.concatStringsSep ":" path;
        # Restarted by the activation below, which compares command lines.
        restartIfChanged = false;
        serviceConfig = {
          ExecStart = utils.escapeSystemdExecArgs command;
          # An exit 2 is tried again no more than twice a minute, and never
          # given up on.
          Restart = "always";
          RestartSec = "30s";
          # A stop ends the runner alone: a `flume launch` in flight runs to
          # its end.
          KillMode = "process";
        };
        startLimitIntervalSec = 0;
      };
    })
  ];
}
