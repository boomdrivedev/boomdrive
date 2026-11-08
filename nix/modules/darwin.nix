# The runner's nix-darwin module, in darwinModules.default: services.boomdrive
# as a launchd agent of the primary user's login (design Decision 10),
# since nix-darwin installs user agents for that user alone, with the
# runner in the system profile and the service's PATH the profiles' (design
# Decision 11). Each tool of the set is installed by its own module, which
# flake.nix imports beside this one, and tools.nix sets their options
# (record 0023). Once the Nix store is mounted, the agent runs
# `boomdrive run --config` with the file rendered from `settings` (design
# Decision 9), so nix-darwin reloads it when the package or the file
# changes, and leaves it running when neither does. The activation links the file at the runner's own path,
# as the primary user, for the commands a person runs by hand.
{ packages }:
{
  config,
  lib,
  pkgs,
  ...
}:
let
  cfg = config.services.boomdrive;
  render = import ./toml.nix { inherit lib; };
  file = pkgs.writeText "boomdrive-config.toml" (render cfg.settings);

  user = config.system.primaryUser;
  home = if user == null then null else config.users.users.${user}.home or null;

  # The profiles, then the operating system's folders: never a package's
  # own folder, so the runner, what Flume launches and a person's shell run
  # one copy of each tool.
  path = [
    "/etc/profiles/per-user/${user}/bin"
    "${home}/.nix-profile/bin"
    "${home}/.local/state/nix/profile/bin"
    "/run/current-system/sw/bin"
    "/nix/var/nix/profiles/default/bin"
    "/usr/local/bin"
    "/usr/bin"
    "/bin"
    "/usr/sbin"
    "/sbin"
  ];
in
{
  imports = [
    (import ./options.nix { inherit packages; })
    (import ./tools.nix { inherit packages; })
    (import ./way-in.nix { inherit packages; })
  ];

  options.services.boomdrive.logFile = lib.mkOption {
    type = lib.types.path;
    default = "${home}/Library/Logs/boomdrive.log";
    defaultText = lib.literalExpression ''"''${config.users.users.''${config.system.primaryUser}.home}/Library/Logs/boomdrive.log"'';
    description = "The file the agent's standard output and standard error go to.";
  };

  config = lib.mkIf cfg.enable {
    system.requiresPrimaryUser = [ "services.boomdrive.enable" ];

    assertions = [
      {
        assertion = user == null || home != null;
        message = "services.boomdrive runs in the login of nix-darwin's primary user ${toString user}, whose home users.users.${toString user}.home is not set: set it, as the agent's HOME and its log's folder.";
      }
    ];

    environment.systemPackages = [ cfg.package ];

    launchd.user.agents.boomdrive = {
      # nix-darwin runs a command as `/bin/sh -c '/bin/wait4path /nix/store
      # && exec <command>'`, so an agent loaded at boot before the store's
      # volume is mounted waits for it, and the runner then holds the job's
      # own process, which a stop's signal reaches.
      command = lib.escapeShellArgs [
        "${cfg.package}/bin/boomdrive"
        "run"
        "--config"
        "${file}"
      ];
      serviceConfig = {
        RunAtLoad = true;
        KeepAlive = true;
        # An exit 2 is tried again no more than twice a minute.
        ThrottleInterval = 30;
        ProcessType = "Interactive";
        # A stop ends the runner alone: a `flume launch` in flight runs to
        # its end.
        AbandonProcessGroup = true;
        EnvironmentVariables = {
          HOME = home;
          PATH = lib.concatStringsSep ":" path;
        };
        StandardOutPath = cfg.logFile;
        StandardErrorPath = cfg.logFile;
      };
    };

    # Activation runs as root, with root's HOME, so the link is made as the
    # primary user under their own home. A link that fails says why and
    # stops nothing else: the agent reads its own --config either way.
    system.activationScripts.postActivation.text = ''
      sudo --user=${lib.escapeShellArg user} -- /usr/bin/env HOME=${lib.escapeShellArg home} \
        /bin/sh ${./activate.sh} link ${file} || true
    '';
  };
}
