# The service-modules spec's cases on the install into every Claude Code
# home (design Decisions 3, 5 and 6 of the change
# every-claude-code-session-has-boomdrives-plugins): the step
# `activate.sh claude` each platform's activation runs, where and in which
# order, the plugins it is given to install and to remove, and Pending
# You's server's file; and the login environment's lines, plugin-dirs.sh,
# where Pending You's folder is written (design Decision 8). Each case runs
# on nix-darwin and on NixOS, unless it names a platform.
t:
let
  inherit (t) lib outputs;

  crewOn = {
    services.crew = {
      enable = true;
      host = "host-a";
      settings = (import ./settings.nix).crewModule;
    };
  };

  platforms = {
    "nix-darwin" = {
      system = t.darwinSystem;
      withoutOf =
        given:
        t.darwinOf [
          outputs.darwinModules.default
          given
        ];
      home = "/Users/alice";
      set = outputs.packages.aarch64-darwin;
    };
    NixOS = {
      system = t.nixosSystem;
      withoutOf =
        given:
        t.nixosOf [
          outputs.nixosModules.default
          {
            services = {
              boomdrive.user = "alice";
              flume.user = "alice";
            };
          }
          given
        ];
      home = "/home/alice";
      set = outputs.packages.x86_64-linux;
    };
  };
  onEach =
    scenario: check:
    lib.mapAttrs' (platform: host: {
      name = "${scenario} on ${platform}";
      value = check platform host;
    }) platforms;

  activate = builtins.unsafeDiscardStringContext "${../modules/activate.sh}";
  sessions = import ../modules/sessions.nix {
    inherit lib;
    inherit (outputs) packages;
  };
  plain = builtins.unsafeDiscardStringContext;

  # NixOS's user activation snippets, by name, as the modules define them.
  userScripts =
    evaluated: lib.foldl' (a: b: a // b) { } evaluated.options.system.userActivationScripts.definitions;
  # The text the runner's user's steps are in: on nix-darwin the system's
  # postActivation, on NixOS the runner's own user snippet.
  activationOf =
    platform: evaluated:
    plain (
      if platform == "nix-darwin" then
        evaluated.config.system.activationScripts.postActivation.text
      else
        (userScripts evaluated).boomdrive.text or ""
    );
  linesOf =
    platform: evaluated: map lib.trim (lib.splitString "\n" (activationOf platform evaluated));
  # Every text of a system's activation that a step could stand in: on
  # nix-darwin its activation script, on NixOS each activation and user
  # activation snippet.
  everyText =
    platform: evaluated:
    if platform == "nix-darwin" then
      [ (plain evaluated.config.system.activationScripts.script.text) ]
    else
      map (step: plain (if builtins.isString step then step else toString (step.text or ""))) (
        builtins.attrValues evaluated.config.system.activationScripts
        ++ builtins.attrValues (userScripts evaluated)
      );
  isStep = lib.hasInfix "-activate.sh claude ";
  stepsOf = platform: evaluated: lib.filter isStep (linesOf platform evaluated);
  indexOf = holds: lines: lib.lists.findFirstIndex holds null lines;

  # Pending You's server's file, as the modules make it for alice.
  serverOf =
    platform: evaluated:
    sessions.serverOf {
      inherit (evaluated.pkgs) writeText;
      folder = evaluated.pkgs.callPackage ../modules/pendingyou-folder.nix {
        inherit (platforms.${platform}) home;
        inherit (evaluated.config.services.boomdrive.pendingyou) package;
      };
      writing = true;
    };

  # The step the platform's activation runs, with the server `server` and
  # the plugins `install` and `remove`.
  stepOf =
    platform: evaluated: server: install: remove:
    let
      inherit (platforms.${platform}) home set;
      inherit (evaluated) config;
      path =
        if platform == "nix-darwin" then
          config.launchd.user.agents.boomdrive.serviceConfig.EnvironmentVariables.PATH
        else
          config.systemd.user.services.boomdrive.environment.PATH;
      words = lib.concatStringsSep " " (
        [
          "claude"
          (lib.getExe' config.services.flume.package "flume")
          (lib.getExe evaluated.pkgs.jq)
          "${set.boomdrive-claude-plugins}"
          "${server}"
        ]
        ++ install
        ++ [ "--" ]
        ++ remove
      );
      step = "/bin/sh ${activate} ${words} || true";
    in
    plain (
      if platform == "nix-darwin" then
        "sudo --user=alice -- /usr/bin/env HOME=${home} PATH=${path}:${home}/.local/bin ${step}"
      else
        "PATH=${path}:${home}/.local/bin ${evaluated.pkgs.runtimeShell} ${activate} ${words} || true"
    );

  every = [
    "pendingyou"
    "plannotator"
    "timber"
    "flume"
    "sluice"
    "sluice-board"
    "crew"
    "worktrunk"
  ];

  # The step's plugins: the words between the server's file and `--`, and
  # after `--`.
  pluginsOf =
    step:
    let
      words = lib.filter (word: word != "") (
        lib.splitString " " (
          lib.removeSuffix " || true" (lib.last (lib.splitString "-activate.sh claude " step))
        )
      );
      after = lib.drop 4 words;
      cut = indexOf (word: word == "--") after;
    in
    if cut == null then
      null
    else
      {
        install = lib.take cut after;
        remove = lib.drop (cut + 1) after;
      };
  pluginsAre =
    platform: what: evaluated: install: remove:
    let
      steps = stepsOf platform evaluated;
      given = if steps == [ ] then null else pluginsOf (builtins.head steps);
    in
    t.expect (
      t.failures evaluated == [ ]
    ) "a ${platform} system ${what} fails: ${toString (t.failures evaluated)}"
    ++
      t.expect (builtins.length steps == 1)
        "the ${platform} system ${what} runs ${toString (builtins.length steps)} steps of activate.sh claude, not one"
    ++
      t.expect (given == { inherit install remove; })
        "the ${platform} step ${what} installs and removes ${builtins.toJSON given}, not ${
          builtins.toJSON { inherit install remove; }
        }";

  stepOnDarwin =
    let
      platform = "nix-darwin";
      evaluated = t.darwinSystem crewOn;
      lines = linesOf platform evaluated;
      expected = stepOf platform evaluated (serverOf platform evaluated) every [ ];
      steps = stepsOf platform evaluated;
      at = indexOf (line: line == expected) lines;
      before = [
        "flume-config.toml"
        "-activate.sh pendingyou "
        "-activate.sh plugins "
      ];
    in
    t.expect (
      t.failures evaluated == [ ]
    ) "the nix-darwin system fails: ${toString (t.failures evaluated)}"
    ++
      t.expect (steps == [ expected ])
        "the nix-darwin activation's steps of activate.sh claude are ${builtins.toJSON steps}, not ${expected}"
    ++ lib.concatMap (
      word:
      let
        found = lib.filter (lib.hasInfix word) lines;
        index = indexOf (lib.hasInfix word) lines;
      in
      t.expect (
        builtins.length found == 1
      ) "the nix-darwin activation runs ${toString (builtins.length found)} lines naming ${word}, not one"
      ++ t.expect (
        at != null && index != null && index < at
      ) "the nix-darwin activation runs activate.sh claude before its line naming ${word}"
    ) before;

  stepOnNixOS =
    let
      platform = "NixOS";
      evaluated = t.nixosSystem crewOn;
      lines = linesOf platform evaluated;
      text = activationOf platform evaluated;
      expected = stepOf platform evaluated (serverOf platform evaluated) every [ ];
      steps = stepsOf platform evaluated;
      at = indexOf (line: line == expected) lines;
      after = index: index != null && at != null && index < at;
      folderAt = indexOf (lib.hasInfix "-activate.sh pendingyou ") lines;
      pluginsAt = indexOf (lib.hasInfix "-activate.sh plugins ") lines;
      keepAt = indexOf (lib.hasInfix "-activate.sh keep ") lines;
      guarded = lib.splitString "if [ \"$(id -un)\" = alice ]; then" text;
      scripts = userScripts evaluated;
      others = lib.filter (name: name != "boomdrive") (builtins.attrNames scripts);
      deps = scripts.boomdrive.deps or [ ];
    in
    t.expect (t.failures evaluated == [ ]) "the NixOS system fails: ${toString (t.failures evaluated)}"
    ++
      t.expect (steps == [ expected ])
        "the NixOS user activation's steps of activate.sh claude are ${builtins.toJSON steps}, not ${expected}"
    ++
      t.expect (after folderAt && after pluginsAt)
        "the NixOS user activation runs activate.sh claude before Pending You's folder or herdr's links: ${text}"
    ++ t.expect (
      keepAt != null && at != null && at < keepAt
    ) "the NixOS user activation runs activate.sh claude after keep: ${text}"
    ++ t.expect (
      builtins.length guarded == 2 && lib.hasInfix "-activate.sh claude " (builtins.elemAt guarded 1)
    ) "the NixOS user activation runs activate.sh claude outside its test of alice: ${text}"
    ++ t.expect (builtins.elem "flume" deps) "the NixOS user activation of the runner's user depends on ${builtins.toJSON deps}, not Flume's"
    ++ lib.concatMap (
      name:
      t.expect (
        !isStep (plain (scripts.${name}.text or ""))
      ) "the NixOS user activation's ${name} runs activate.sh claude"
    ) others;

  # No step anywhere in the system's activation.
  noStep =
    platform: what: evaluated:
    t.expect (
      t.failures evaluated == [ ]
    ) "a ${platform} system ${what} fails: ${toString (t.failures evaluated)}"
    ++ t.expect (
      !lib.any isStep (everyText platform evaluated)
    ) "a ${platform} system ${what} runs activate.sh claude";

  noStepWithout =
    platform:
    { withoutOf, ... }:
    noStep platform "with Flume and no runner" (withoutOf {
      services.flume.enable = true;
    });

  noStepWithoutFlume =
    platform:
    { system, ... }:
    noStep platform "with no Flume package" (system {
      services.flume.package = null;
    });

  everyPlugin =
    platform:
    { system, ... }:
    pluginsAre platform "with the runner and crew" (system crewOn) every [ ];

  noCompanions =
    platform:
    { system, ... }:
    let
      evaluated = system { };
      steps = stepsOf platform evaluated;
    in
    pluginsAre platform "without crew" evaluated
      [
        "timber"
        "flume"
        "sluice"
        "sluice-board"
        "worktrunk"
      ]
      [
        "pendingyou"
        "plannotator"
        "crew"
      ]
    ++ t.expect (
      steps != [ ] && lib.hasInfix "-boomdrive-claude-plugins - " (builtins.head steps)
    ) "the ${platform} step without crew is given a server: ${builtins.toJSON steps}";

  toolOff =
    platform:
    { system, ... }:
    pluginsAre platform "with timber off" (system {
      imports = [ crewOn ];
      services.timber.enable = false;
    }) (lib.filter (name: name != "timber") every) [ "timber" ];

  serversFile =
    platform:
    { system, home, ... }:
    let
      evaluated = system crewOn;
      server = serverOf platform evaluated;
      steps = stepsOf platform evaluated;
      named = builtins.fromJSON server.text;
      expected = {
        type = "http";
        url = "https://www.pendingyou.com/mcp";
        headersHelper = "${home}/.config/pendingyou/bin/pendingyou-mcp-headers";
      };
    in
    t.expect (
      steps != [ ] && lib.hasInfix " ${plain "${server}"} " (builtins.head steps)
    ) "the ${platform} step is given no ${plain "${server}"}: ${builtins.toJSON steps}"
    ++ t.expect (
      named == expected
    ) "the ${platform} server's file names ${builtins.toJSON named}, not ${builtins.toJSON expected}";

  # The lines of plugin-dirs.sh in the login environment.
  lines = lib.trim (builtins.readFile ../modules/plugin-dirs.sh);
  holdsLines = evaluated: lib.hasInfix lines (plain evaluated.config.environment.extraInit);

  linesWith =
    platform:
    { system, ... }:
    let
      evaluated = system crewOn;
    in
    t.expect (
      t.failures evaluated == [ ]
    ) "the ${platform} system with crew fails: ${toString (t.failures evaluated)}"
    ++ t.expect (holdsLines evaluated) "the ${platform} system with crew's environment.extraInit holds no lines of plugin-dirs.sh";

  noLinesWithout =
    platform:
    { system, ... }:
    lib.concatLists (
      lib.mapAttrsToList
        (
          what: evaluated:
          t.expect (
            t.failures evaluated == [ ]
          ) "the ${platform} system ${what} fails: ${toString (t.failures evaluated)}"
          ++ t.expect (
            !holdsLines evaluated
          ) "the ${platform} system ${what}'s environment.extraInit holds the lines of plugin-dirs.sh"
        )
        {
          "without crew" = system { };
          "with no Pending You" = system {
            imports = [ crewOn ];
            services.boomdrive.pendingyou.package = null;
          };
        }
    );
in
{
  "The step on nix-darwin" = stepOnDarwin;
  "The step on NixOS" = stepOnNixOS;
}
// onEach "No step without the runner" noStepWithout
// onEach "No step without Flume's package" noStepWithoutFlume
// onEach "Every plugin with the runner and crew" everyPlugin
// onEach "No companions' plugins without crew" noCompanions
// onEach "A tool turned off" toolOff
// onEach "The server's file" serversFile
// onEach "The lines with the folder" linesWith
// onEach "No lines without the folder" noLinesWithout
