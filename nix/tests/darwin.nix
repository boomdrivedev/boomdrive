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

  # The agent's program, which launchd runs as the job's process.
  programOf = evaluated: (agentOf evaluated).ProgramArguments;
  # What the shell runs first: it waits for the store's volume, then hands
  # the job's process to the command, so a stop's signal reaches it.
  wait = "/bin/wait4path /nix/store && exec ";
  waits =
    evaluated:
    let
      program = programOf evaluated;
    in
    lib.length program == 3
    &&
      lib.take 2 program == [
        "/bin/sh"
        "-c"
      ]
    && lib.hasPrefix wait (lib.elemAt program 2);
  # The command's words: store paths and plain words, which the shell
  # reads unquoted.
  commandOf = evaluated: lib.splitString " " (lib.removePrefix wait (lib.last (programOf evaluated)));
  # The word after `--config`, or "" where there is none.
  fileOf =
    evaluated:
    let
      command = commandOf evaluated;
      at = lib.lists.findFirstIndex (word: word == "--config") null command;
    in
    if at == null || at + 1 >= lib.length command then "" else lib.elemAt command (at + 1);
  # The plist nix-darwin installs, which it reloads the agent for where it
  # differs from the one loaded.
  plistOf = evaluated: evaluated.config.environment.userLaunchAgents."org.nixos.boomdrive.plist".text;
  # The program of `evaluated` with `new` read as `old`.
  readAs =
    old: new: evaluated:
    map (lib.replaceStrings [ new ] [ old ]) (programOf evaluated);

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
  # Another build of the runner, in a folder of its own.
  newPackage = t.darwinSystem (
    { pkgs, ... }:
    {
      services.boomdrive.package = pkgs.writeTextDir "bin/boomdrive" "";
    }
  );
  otherPackage = newPackage.config.services.boomdrive.package.outPath;

  own = t.darwinSystem {
    services.flume.package = null;
    services.sluice.package = null;
  };
  gone = t.darwinSystem { services.boomdrive.tools = [ ]; };
  saysGone = words: lib.hasInfix words (toString (t.failures gone));
  ownFlume = t.darwinSystem { services.flume.package = null; };

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
    ++
      t.expect
        (
          commandOf base == [
            "${set.boomdrive}/bin/boomdrive"
            "run"
            "--config"
            (fileOf base)
          ]
        )
        "the agent runs ${builtins.toJSON (commandOf base)}, not the set's boomdrive run --config with the file"
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

  "The agent waits for the Nix store" =
    lib.concatMap
      (
        evaluated:
        t.expect (waits evaluated) "the agent's program does not wait for /nix/store, then exec its command: ${builtins.toJSON (programOf evaluated)}"
      )
      [
        base
        crew
      ];

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
    t.expect (readAs (fileOf base) (fileOf newFile) newFile == programOf base)
      "the agents' programs differ beside the file: ${builtins.toJSON (programOf base)} and ${builtins.toJSON (programOf newFile)}"
    ++ t.expect (
      fileOf base != fileOf newFile
    ) "the agents' programs name one file for two fetch_interval"
    ++ t.expect (plistOf base != plistOf newFile) "the agents' plists are one for two files";

  "A new package on nix-darwin" =
    t.expect (readAs set.boomdrive.outPath otherPackage newPackage == programOf base)
      "the agents' programs differ beside the package: ${builtins.toJSON (programOf base)} and ${builtins.toJSON (programOf newPackage)}"
    ++ t.expect (programOf base != programOf newPackage) "the agents' programs name one package for two"
    ++ t.expect (plistOf base != plistOf newPackage) "the agents' plists are one for two packages";

  "Nothing else changed on nix-darwin" = t.expect (
    plistOf base == plistOf ownFlume
  ) "the agent's plist changes with services.flume.package";

  "The set's tools on nix-darwin" =
    installs base "runner" set.boomdrive
    ++ installs base "Flume" set.flume
    ++ installs base "sluice" set.sluice;

  "A host's own tools on nix-darwin" =
    installs own "runner" set.boomdrive
    ++ installsNo own "Flume" set.flume
    ++ installsNo own "sluice" set.sluice;

  "The tools option is gone on nix-darwin" =
    t.expect (t.failures gone != [ ]) "services.boomdrive.tools = [ ] evaluates"
    ++
      lib.concatMap
        (
          words:
          t.expect (saysGone words) "the failure of services.boomdrive.tools names no ${words}: ${toString (t.failures gone)}"
        )
        [
          "services.flume.package"
          "services.sluice.package"
        ];

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
