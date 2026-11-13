# The runner's nix-darwin module, in darwinModules.default: services.boomdrive
# as a launchd agent of the primary user's login (design Decision 10),
# since nix-darwin installs user agents for that user alone, with the
# runner in the system profile and the service's PATH the profiles' (design
# Decision 11). Each tool of the set is installed by its own module, which
# flake.nix imports beside this one, and tools.nix sets their options
# (record 0023); drives.nix gives Flume's module each drive's stream (record
# 0028), and sources.nix declares crew's source to Flume's module (record
# 0034). Once the Nix store is mounted, the agent runs
# `boomdrive run --config` with the file rendered from `accounts` and
# `settings` (design Decision 9), so nix-darwin reloads it when the package
# or the file changes, and leaves it running when neither does. The
# activation links the file at the runner's own path, as the primary user,
# for the commands a person runs by hand, writes Pending You's folder where
# crew's companions are installed (record 0044, point 7), and links the
# herdr plugins apps.nix lists with herdr's own command (record 0023).
# Beside `logFile`, the agent's log, the module declares `dataDir`, the
# runner's data folder by the runner's own rule (data-dir.nix) from the
# primary user's home and the agent's own `XDG_DATA_HOME`, and clones.nix
# declares `cloneDirs`, each kept repository's clone folder in it: each
# read-only, read wherever a host names the folder or a clone's, and set
# by none (record 0036 as amended on 2026-11-11). With crew's module on,
# the module gives crew's kept repositories,
# `services.crew.settings.repositories`, one entry for each address of
# `cloneDirs`, its `dir` the clone's folder and its `checkout` the
# runner's view under the primary user's home (kept.nix), at the module's
# own priority, so a host writes none (record 0036 as amended a third
# time on 2026-11-11). Each activation of a system that imports the
# module runs, as root and before the user's steps, activate.sh's
# `managed` on Claude Code's managed telemetry document, claude-code.nix's:
# where the runner is on and `services.boomdrive.claude-code.telemetry`
# names a collector, it writes the document every Claude Code session on
# the host reads, and otherwise it removes the set's own document, since
# nix-darwin removes no file it wrote outside `/etc` (record 0013 as
# amended on 2026-11-12).
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
  inherit (import ./accounts.nix { inherit lib; }) fileOf;
  dataDirOf = import ./data-dir.nix { inherit lib; };
  keptOf = import ./kept.nix { inherit lib; };
  claudeCode = import ./claude-code.nix { inherit lib; };
  file = pkgs.writeText "boomdrive-config.toml" (render (fileOf cfg));

  # Claude Code's managed telemetry document, written where the runner is
  # on and a collector is named, and removed otherwise (the change
  # every-claude-code-session-exports-to-the-hosts-collector, design
  # Decisions 6 and 8).
  telemetry = cfg.claude-code.telemetry;
  managed = lib.escapeShellArg "${claudeCode.darwinFolder}/${claudeCode.name}";
  document = pkgs.writeText claudeCode.name (builtins.toJSON (claudeCode.documentOf telemetry));
  managing =
    if cfg.enable && telemetry.collector != null then
      "/bin/sh ${./activate.sh} managed ${managed} ${document} || true\n"
    else
      "/bin/sh ${./activate.sh} managed ${managed} || true\n";

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

  user = config.system.primaryUser;
  home = if user == null then null else config.users.users.${user}.home or null;

  # Pending You's folder, where crew's companions are installed and
  # Pending You's package is given (companions.nix; record 0044, point 7).
  writingFolder = config.services.crew.enable && cfg.pendingyou.package != null;
  folder = pkgs.callPackage ./pendingyou-folder.nix {
    inherit home;
    inherit (cfg.pendingyou) package;
  };

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
    (import ./apps.nix { inherit packages; })
    (import ./companions.nix { inherit packages; })
    ./drives.nix
    ./clones.nix
    ./sources.nix
    claudeCode.module
  ];

  options.services.boomdrive.logFile = lib.mkOption {
    type = lib.types.path;
    default = "${home}/Library/Logs/boomdrive.log";
    defaultText = lib.literalExpression ''"''${config.users.users.''${config.system.primaryUser}.home}/Library/Logs/boomdrive.log"'';
    description = "The file the agent's standard output and standard error go to.";
  };

  # The agent's own XDG_DATA_HOME, or null where the agent has none or
  # the runner is not enabled (the change
  # the-set-names-the-runners-data-folder, design Decisions 1 and 3).
  options.services.boomdrive.dataDir = lib.mkOption {
    type = lib.types.path;
    readOnly = true;
    default = dataDirOf {
      inherit home;
      dataHome =
        config.launchd.user.agents.boomdrive.serviceConfig.EnvironmentVariables.XDG_DATA_HOME or null;
    };
    defaultText = lib.literalMD ''
      `$XDG_DATA_HOME/boomdrive` where the runner's agent is given an absolute
      `XDG_DATA_HOME` in `launchd.user.agents.boomdrive.serviceConfig.EnvironmentVariables`,
      else `.local/share/boomdrive` under the primary user's home,
      `users.users.''${config.system.primaryUser}.home`
    '';
    description = ''
      The runner's data folder, where it keeps its clones, its lock and its
      report, by the runner's own rule: `$XDG_DATA_HOME/boomdrive` where the
      agent's own environment gives an absolute `XDG_DATA_HOME`, else
      `~/.local/share/boomdrive` under the primary user's home. It is
      read-only: a host reads it wherever it names the folder, a clone's
      folder in it being `services.boomdrive.cloneDirs`'s, and sets it
      nowhere. An `XDG_DATA_HOME` the agent takes from
      outside its declaration, such as launchd's user domain, is not seen:
      a host that moves the runner's data gives the agent its own
      `XDG_DATA_HOME` in `launchd.user.agents.boomdrive.serviceConfig.EnvironmentVariables`.
    '';
  };

  config = lib.mkMerge [
    # Run as root, before the user's steps, whether or not the runner is
    # on: a failure says why and stops nothing else.
    { system.activationScripts.postActivation.text = lib.mkBefore managing; }

    (lib.mkIf cfg.enable {
      system.requiresPrimaryUser = [ "services.boomdrive.enable" ];

      assertions = [
        {
          assertion = user == null || home != null;
          message = "services.boomdrive runs in the login of nix-darwin's primary user ${toString user}, whose home users.users.${toString user}.home is not set: set it, as the agent's HOME and its log's folder.";
        }
      ];

      environment.systemPackages = [ cfg.package ];

      # crew's kept repositories, where crew's module is on and the runner
      # keeps a repository (design Decisions 1, 4 and 6).
      services.crew.settings.repositories =
        lib.mkIf (config.services.crew.enable && cfg.cloneDirs != { })
          (keptOf {
            inherit (cfg) cloneDirs;
            inherit home;
            root = cfg.settings.repository_root;
          });

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
      # Pending You's folder is written the same way, before the plugins, so
      # Pending You for Herdr finds its launcher at its first link; the
      # plugins are linked the same way, each refusal said and stopping none.
      system.activationScripts.postActivation.text = ''
        sudo --user=${lib.escapeShellArg user} -- /usr/bin/env HOME=${lib.escapeShellArg home} \
          /bin/sh ${./activate.sh} link ${file} || true
      ''
      + lib.optionalString writingFolder ''
        sudo --user=${lib.escapeShellArg user} -- /usr/bin/env HOME=${lib.escapeShellArg home} /bin/sh ${./activate.sh} pendingyou ${folder} ${lib.escapeShellArg folder.folder} || true
      ''
      + lib.optionalString linking ''
        sudo --user=${lib.escapeShellArg user} -- /usr/bin/env HOME=${lib.escapeShellArg home} /bin/sh ${./activate.sh} ${plugins} || true
      '';
    })
  ];
}
