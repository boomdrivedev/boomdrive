# The service-modules spec on nix-darwin (design Decisions 10 and 11): the
# module evaluated in a system of aarch64-darwin whose primary user is
# alice, at /Users/alice, its agent read key by key, and the system
# profile.
t:
let
  inherit (t) lib;
  settings = import ./settings.nix;
  set = t.outputs.packages.aarch64-darwin;

  base = t.darwinSystem { };
  agentOf = evaluated: evaluated.config.launchd.user.agents.boomdrive.serviceConfig;
  agent = agentOf base;
  fileOf = evaluated: lib.elemAt (agentOf evaluated).ProgramArguments 3;

  # The module's file holding `text`, byte for byte.
  writes =
    evaluated: text:
    fileOf evaluated == (evaluated.pkgs.writeText "boomdrive-config.toml" text).outPath;

  holds =
    key: value:
    t.expect (
      agent.${key} == value
    ) "the agent's ${key} is ${builtins.toJSON agent.${key}}, not ${builtins.toJSON value}";

  profileOf = evaluated: map (p: p.outPath) evaluated.config.environment.systemPackages;
  installs =
    evaluated: name: package:
    t.expect (builtins.elem package.outPath (profileOf evaluated)) "the system profile holds no ${name}";
  installsNo =
    evaluated: name: package:
    t.expect (
      !builtins.elem package.outPath (profileOf evaluated)
    ) "the system profile holds the set's ${name}";

  noHome = t.darwinSystem { users.users.alice.home = lib.mkForce null; };
  saysNoHome = words: lib.hasInfix words (toString (t.failures noHome));

  newFile = t.darwinSystem { services.boomdrive.settings.fetch_interval = "5m"; };
  arguments = agent.ProgramArguments;
  newArguments = (agentOf newFile).ProgramArguments;

  own = t.darwinSystem { services.boomdrive.tools = [ ]; };

  path = lib.splitString ":" agent.EnvironmentVariables.PATH;
  before =
    entry:
    let
      at = lib.lists.findFirstIndex (e: e == entry) null path;
      system = lib.lists.findFirstIndex (e: e == "/run/current-system/sw/bin") null path;
    in
    t.expect (at != null && system != null && at < system)
      "the agent's PATH does not name ${entry} before /run/current-system/sw/bin: ${agent.EnvironmentVariables.PATH}";

  crew = t.darwinSystem { services.boomdrive.settings = settings.crew; };

  # Compared as text: a regular expression may not hold a store path.
  activation = builtins.unsafeDiscardStringContext base.config.system.activationScripts.postActivation.text;
  # As the primary user, under their home, with the module's file.
  link = builtins.unsafeDiscardStringContext ''
    sudo --user=alice -- /usr/bin/env HOME=/Users/alice \
      /bin/sh ${../modules/activate.sh} link ${fileOf base} || true
  '';
in
{
  "The agent" =
    t.expect (t.failures base == [ ]) "the system fails: ${toString (t.failures base)}"
    ++ holds "Label" "org.nixos.boomdrive"
    ++ holds "ProgramArguments" [
      "${set.boomdrive}/bin/boomdrive"
      "run"
      "--config"
      (fileOf base)
    ]
    ++ t.expect (writes base (builtins.readFile ./files/empty.toml)) "the agent's --config is not the file of nix/tests/files/empty.toml"
    ++ holds "RunAtLoad" true
    ++ holds "KeepAlive" true
    ++ holds "ThrottleInterval" 30
    ++ holds "ProcessType" "Interactive"
    ++ holds "AbandonProcessGroup" true
    ++
      t.expect
        (
          builtins.attrNames agent.EnvironmentVariables == [
            "HOME"
            "PATH"
          ]
        )
        "the agent's environment is ${toString (builtins.attrNames agent.EnvironmentVariables)}, not HOME and PATH alone"
    ++ t.expect (
      agent.EnvironmentVariables.HOME == "/Users/alice"
    ) "the agent's HOME is ${agent.EnvironmentVariables.HOME}"
    ++ holds "StandardOutPath" "/Users/alice/Library/Logs/boomdrive.log"
    ++ holds "StandardErrorPath" "/Users/alice/Library/Logs/boomdrive.log";

  "No home for the primary user" =
    t.expect (t.failures noHome != [ ]) "the module evaluates for a primary user with no home"
    ++
      lib.concatMap
        (
          words: t.expect (saysNoHome words) "the failure names no ${words}: ${toString (t.failures noHome)}"
        )
        [
          "alice"
          "users.users.alice.home"
        ];

  "A new file on nix-darwin" =
    t.expect (
      lib.take 3 arguments == lib.take 3 newArguments && lib.drop 4 arguments == lib.drop 4 newArguments
    ) "the agents' arguments differ beside the file: ${toString arguments} and ${toString newArguments}"
    ++ t.expect (
      fileOf base != fileOf newFile
    ) "the agents' arguments name one file for two fetch_interval";

  "The set's tools on nix-darwin" =
    installs base "runner" set.boomdrive
    ++ installs base "Flume" set.flume
    ++ installs base "sluice" set.sluice;

  "A host's own tools on nix-darwin" =
    installs own "runner" set.boomdrive
    ++ installsNo own "Flume" set.flume
    ++ installsNo own "sluice" set.sluice;

  "The user's profile first on nix-darwin" =
    before "/etc/profiles/per-user/alice/bin"
    ++ before "/Users/alice/.nix-profile/bin"
    ++ t.expect (
      !lib.any (lib.hasPrefix builtins.storeDir) path
    ) "the agent's PATH names a package's own folder: ${agent.EnvironmentVariables.PATH}";

  "A host under crew on nix-darwin" =
    t.expect (t.failures crew == [ ]) "crew's settings fail: ${toString (t.failures crew)}"
    ++ t.expect (writes crew (builtins.readFile ./files/crew.toml)) "the agent's --config is not the file of nix/tests/files/crew.toml";

  "The activation links the file on nix-darwin" =
    t.expect (lib.hasInfix link activation) "the activation runs no `${link}`: ${activation}";
}
