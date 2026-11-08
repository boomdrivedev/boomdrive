# The service-modules spec's cases on each drive's stream in Flume's own
# module (the change each-repository-is-named-once-by-its-owner, design
# Decisions 3 to 6): what the Boomdrive module gives Flume's settings of
# each drive of this host, read from the system's
# `config.services.flume.settings`, whose keys the module does not give
# read null. Each case runs on nix-darwin and on NixOS, unless it names a
# platform.
t:
let
  inherit (t) lib outputs;
  render = import ../modules/toml.nix { inherit lib; };
  inherit (import ../modules/accounts.nix { inherit lib; }) fileOf;

  platforms = {
    "nix-darwin" = {
      system = t.darwinSystem;
      # The runner's module off, its accounts given.
      off =
        given:
        t.darwinOf [
          outputs.darwinModules.default
          given
        ];
    };
    NixOS = {
      system = t.nixosSystem;
      off =
        given:
        t.nixosOf [
          outputs.nixosModules.default
          given
        ];
    };
  };

  onEach =
    scenario: check:
    lib.mapAttrs' (platform: host: {
      name = "${scenario} on ${platform}";
      value = check platform host;
    }) platforms;

  # The drive `home` of the account `own`, keeping the owner agentplot's
  # flume, its stream `given`.
  home = stream: {
    services.boomdrive.accounts.own.drives.home = {
      inherit stream;
      repositories.agentplot = [ "flume" ];
    };
  };

  # Flume's settings of a system.
  flumeOf = evaluated: evaluated.config.services.flume.settings;
  # A stream of Flume's settings as the module gives it: its room, its
  # priority and each label's reserve; null where there is none.
  streamOf =
    evaluated: name:
    let
      inherit (flumeOf evaluated) streams;
      stream = if streams == null then null else streams.${name} or null;
    in
    if stream == null then
      null
    else
      {
        inherit (stream) agents priority;
        reserves = lib.mapAttrs (_: label: label.reserve) (
          if stream.labels == null then { } else stream.labels
        );
      };

  holds =
    platform: evaluated: name: expected:
    t.expect (
      t.failures evaluated == [ ]
    ) "the ${platform} system fails: ${toString (t.failures evaluated)}"
    ++
      t.expect (streamOf evaluated name == expected)
        "the ${platform} system's Flume stream ${name} is ${builtins.toJSON (streamOf evaluated name)}, not ${builtins.toJSON expected}";

  byDefault =
    platform:
    { system, ... }:
    holds platform (system (home { })) "home" {
      agents = 4;
      priority = null;
      reserves = {
        operation = 1;
        planning = 2;
      };
    };

  onlyChanges =
    platform:
    { system, ... }:
    holds platform
      (system (home {
        priority = 11;
        labels = {
          planning.reserve = 1;
          review.reserve = 1;
        };
      }))
      "home"
      {
        agents = 4;
        priority = 11;
        reserves = {
          operation = 1;
          planning = 1;
          review = 1;
        };
      };

  runnerOff =
    platform:
    { off, ... }:
    let
      evaluated = off (home { });
      settings = flumeOf evaluated;
    in
    t.expect (
      !evaluated.config.services.boomdrive.enable
    ) "the ${platform} system with the runner unset enables it"
    ++
      t.expect (settings.streams == null)
        "the ${platform} system with the runner off gives Flume the streams ${builtins.toJSON settings.streams}"
    ++
      t.expect (settings.repos == null)
        "the ${platform} system with the runner off gives Flume the repositories ${builtins.toJSON settings.repos}";

  # Two systems that differ only in the drive's room.
  nothingOfTheRunners =
    platform:
    { system, ... }:
    let
      four = system (home { });
      six = system (home {
        agents = 6;
      });
      fileText = evaluated: render (fileOf evaluated.config.services.boomdrive);
      plistOf = evaluated: evaluated.config.environment.userLaunchAgents."org.nixos.boomdrive.plist".text;
    in
    t.expect ((streamOf six "home").agents or null == 6)
      "the ${platform} system's Flume stream home is ${builtins.toJSON (streamOf six "home")}, not of 6 agents"
    ++
      t.expect (fileText four == fileText six)
        "the ${platform} runner's files differ with the drive's room: ${fileText four} and ${fileText six}"
    ++ lib.optionals (platform == "nix-darwin") (
      t.expect (plistOf four == plistOf six) "the nix-darwin agents' plists differ with the drive's room"
    );

  # Each repository's stream in Flume's settings, by its key; null where
  # there is none.
  reposOf =
    evaluated:
    let
      inherit (flumeOf evaluated) repos;
    in
    if repos == null then null else lib.mapAttrs (_: repo: repo.stream) repos;
  reposHold =
    platform: evaluated: expected:
    t.expect (
      t.failures evaluated == [ ]
    ) "the ${platform} system fails: ${toString (t.failures evaluated)}"
    ++
      t.expect (reposOf evaluated == expected)
        "the ${platform} system's Flume repositories are ${builtins.toJSON (reposOf evaluated)}, not ${builtins.toJSON expected}";

  eachAddress =
    platform:
    { system, ... }:
    reposHold platform
      (system {
        services.boomdrive.accounts = {
          own.drives.home.repositories.agentplot = [
            "flume"
            "sluice"
          ];
          work.drives.garden.repositories.example = [ "alpha" ];
        };
      })
      {
        "agentplot/flume" = "home";
        "agentplot/sluice" = "home";
        "example/alpha" = "garden";
      };

  namedOther =
    platform:
    { system, ... }:
    let
      evaluated = system (home {
        name = "garden";
      });
      inherit (flumeOf evaluated) streams;
      names = if streams == null then [ ] else builtins.attrNames streams;
    in
    reposHold platform evaluated { "agentplot/flume" = "garden"; }
    ++ t.expect (
      names == [
        "garden"
      ]
    ) "the ${platform} system's Flume streams are ${toString names}, not garden alone";
in
onEach "A drive's stream by default" byDefault
// onEach "A drive that writes only what it changes" onlyChanges
// onEach "With the runner off" runnerOff
// onEach "A drive's stream changes nothing of the runner's" nothingOfTheRunners
// onEach "Each address given its drive's stream" eachAddress
// onEach "A stream named other than its drive" namedOther
