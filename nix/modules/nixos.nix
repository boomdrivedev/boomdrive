# The runner's NixOS module, in nixosModules.default: services.boomdrive as
# a systemd user unit of services.boomdrive.user, who lingers, so the
# runner runs from boot on a host no one logs into (design Decision 10),
# with the runner in the system profile and the service's PATH the
# profiles' (design Decision 11). Each tool of the set is installed by its
# own module, which flake.nix imports beside this one; tools.nix sets
# their options, and each tool's user is the runner's unless the host
# names its own (record 0023); drives.nix gives Flume's module each drive's
# stream (record 0028). The unit runs `boomdrive run --config`
# with the file rendered from `accounts` and `settings` (design Decision
# 9). NixOS restarts no changed user unit, so the user's activation links
# the file at the runner's own path, links the herdr plugins apps.nix
# lists with herdr's own command (record 0023), then starts the unit where
# it is not running and restarts it where the running runner's command
# line is not the unit's; any other user's activation does nothing. Beside
# `user`, the module declares `dataDir`, the runner's data folder by the
# runner's own rule (data-dir.nix) from that user's home and the unit's
# own `XDG_DATA_HOME`, and clones.nix declares `cloneDirs`, each kept
# repository's clone folder in it: each read-only, read wherever a host
# names the folder or a clone's, and set by none (record 0036 as amended
# on 2026-11-11). With crew's module on, the module gives crew's kept
# repositories, `services.crew.settings.repositories`, one entry for each
# address of `cloneDirs`, its `dir` the clone's folder and its `checkout`
# the runner's view under that user's home (kept.nix), at the module's
# own priority, so a host writes none (record 0036 as amended a third
# time on 2026-11-11).
{ packages }:
{
  config,
  lib,
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
  file = pkgs.writeText "boomdrive-config.toml" (render (fileOf cfg));

  # herdr's own link of each plugin, where a herdr and a plugin are given
  # (apps.nix).
  linking = cfg.herdr.package != null && cfg.herdr.plugins != [ ];
  plugins = lib.escapeShellArgs (
    [
      "plugins"
      (lib.getExe' cfg.herdr.package "herdr")
    ]
    ++ map toString cfg.herdr.plugins
  );
  pluginsStep = lib.optionalString linking "${pkgs.runtimeShell} ${./activate.sh} ${plugins} || true";

  inherit (cfg) user;
  account = config.users.users.${user};
  inherit (account) home;

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
    ./drives.nix
    ./clones.nix
  ];

  options.services.boomdrive.user = lib.mkOption {
    type = lib.types.str;
    example = "alice";
    description = "The user whose systemd user manager runs the runner, and who lingers.";
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

  config = lib.mkIf cfg.enable {
    assertions = [
      # users.users.<user> is there either way, since this module makes
      # the user linger: a user only the module names is neither kind.
      {
        assertion = account.isNormalUser || account.isSystemUser;
        message = "services.boomdrive.user is ${user}, whom users.users.${user} does not declare: declare the user whose manager runs the runner.";
      }
      {
        assertion = !hosted || lib.versionAtLeast systemd "256";
        message = "services.boomdrive.settings names a hosted control plane, whose partition's key systemd-creds of systemd 256 or later keeps for the user, and this system's systemd is ${systemd}.";
      }
    ];

    environment.systemPackages = [ cfg.package ];

    # Each tool's module links its file for the runner's user, unless the
    # host names another; and crew's kept repositories, where crew's
    # module is on and the runner keeps a repository (design Decisions 1,
    # 4 and 6).
    services = {
      flume.user = lib.mkDefault user;
      sluice.user = lib.mkDefault user;
      crew = {
        user = lib.mkDefault user;
        settings.repositories = lib.mkIf (config.services.crew.enable && cfg.cloneDirs != { }) (keptOf {
          inherit (cfg) cloneDirs;
          inherit home;
          root = cfg.settings.repository_root;
        });
      };
    };

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

    # Run by every user's nixos-activation.service, inside which a job is
    # never waited for, with coreutils on its PATH and neither systemctl
    # nor ps. A link that fails says why and stops nothing else: the unit
    # reads its own --config either way.
    system.userActivationScripts.boomdrive.text = ''
      if [ "$(id -un)" = ${lib.escapeShellArg user} ]; then
        ${pkgs.runtimeShell} ${./activate.sh} link ${file} || true
        ${pluginsStep}
        PATH=${keepPath}:$PATH \
          ${pkgs.runtimeShell} ${./activate.sh} keep ${lib.escapeShellArgs ([ user ] ++ command)}
      fi
    '';
  };
}
