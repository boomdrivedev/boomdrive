# The runner's NixOS module, in nixosModules.default: services.boomdrive as
# a systemd user unit of services.boomdrive.user, who lingers, so the
# runner runs from boot on a host no one logs into (design Decision 10),
# with the runner in the system profile and the service's PATH the
# profiles' (design Decision 11). Each tool of the set is installed by its
# own module, which flake.nix imports beside this one; tools.nix sets
# their options, and each tool's user is the runner's unless the host
# names its own (record 0023). The unit runs `boomdrive run --config`
# with the file rendered from `settings` (design Decision 9). NixOS
# restarts no changed user unit, so the user's activation links the file
# at the runner's own path, then starts the unit where it is not running
# and restarts it where the running runner's command line is not the
# unit's; any other user's activation does nothing.
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
  file = pkgs.writeText "boomdrive-config.toml" (render cfg.settings);

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
  ];

  options.services.boomdrive.user = lib.mkOption {
    type = lib.types.str;
    example = "alice";
    description = "The user whose systemd user manager runs the runner, and who lingers.";
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
    # host names another.
    services = {
      flume.user = lib.mkDefault user;
      sluice.user = lib.mkDefault user;
      crew.user = lib.mkDefault user;
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
        PATH=${keepPath}:$PATH \
          ${pkgs.runtimeShell} ${./activate.sh} keep ${lib.escapeShellArgs ([ user ] ++ command)}
      fi
    '';
  };
}
