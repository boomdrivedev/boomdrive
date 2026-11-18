# The service-modules spec's cases on crew's drives in crew's own module
# (the change a-drives-machine-and-state-are-named-once, design Decisions 3,
# 4 and 7): what the Boomdrive module gives
# `config.services.crew.settings.drives` of the host's accounts, each
# drive's machine and its account's state repository, with crew's module
# on, whether or not the runner is; and that a host's second write of
# either fails crew's own check, the module's definition placed at
# services.boomdrive.accounts. Each case runs on nix-darwin and on NixOS.
t:
let
  inherit (t) lib outputs;
  settings = import ./settings.nix;

  platforms = {
    "nix-darwin" = {
      system = t.darwinSystem;
      # The runner's module off.
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

  # crew turned on, with its host `host-a` and crew's settings `teams`,
  # beside the accounts `accounts`.
  crewWith = accounts: teams: {
    services = {
      boomdrive.accounts = accounts;
      crew = {
        enable = true;
        host = "host-a";
        settings = teams;
      };
    };
  };

  # Whether crew's teams file, as crew's module renders it in `evaluated`,
  # evaluates whole, as tools.nix's `loads` reads it.
  loads =
    evaluated:
    (builtins.tryEval (builtins.deepSeq evaluated.config.services.crew.settings true)).success;

  # Whether the module's definition of services.crew.settings is placed at
  # services.boomdrive.accounts, reading no definition's value (design
  # Decision 4).
  placed =
    evaluated:
    lib.any (
      definition: definition.file == "services.boomdrive.accounts"
    ) evaluated.options.services.crew.settings.definitionsWithLocations;

  # crew's drives' machine and state alone.
  machinesAndStates =
    evaluated:
    lib.mapAttrs (_: drive: {
      inherit (drive) machine state;
    }) evaluated.config.services.crew.settings.drives;

  # What the accounts of crewAccounts give crew (design Decision 3).
  given = {
    garden = {
      machine = "host-a";
      state = "example/state";
    };
    home = {
      machine = "host-a";
      state = "example/state";
    };
    away = {
      machine = "host-b";
      state = "example/work";
    };
    field = {
      machine = "host-a";
      state = "example/work";
    };
  };

  # crew's drives in `evaluated` hold `expected`'s machine and state.
  holdsDrives =
    platform: evaluated: expected:
    let
      read = machinesAndStates evaluated;
    in
    t.expect (read == expected)
      "crew's drives' machines and states on ${platform} are ${builtins.toJSON read}, not ${builtins.toJSON expected}";

  givenToCrew =
    platform:
    { system, ... }:
    let
      evaluated = system (crewWith settings.crewAccounts settings.crewDrives);
    in
    holdsDrives platform evaluated given
    ++ t.expect (loads evaluated) "crew's teams file of the ${platform} system does not evaluate"
    ++ t.expect (
      t.failures evaluated == [ ]
    ) "the ${platform} system fails: ${toString (t.failures evaluated)}"
    ++ t.expect (placed evaluated) "the ${platform} module's definition of services.crew.settings is not placed at services.boomdrive.accounts";

  runnerOff =
    platform:
    { off, ... }:
    let
      evaluated = off (crewWith settings.crewAccounts settings.crewDrives);
    in
    t.expect (
      !evaluated.config.services.boomdrive.enable
    ) "the ${platform} system with the runner unset enables it"
    ++ holdsDrives platform evaluated given;

  # The account `own`, naming no state repository, whose drive `home`
  # keeps agentplot/flume, beside crew's settings that write `home`'s
  # machine and state.
  stateless =
    platform:
    { system, ... }:
    let
      evaluated = system (
        crewWith { own.drives.home.repositories.agentplot = [ "flume" ]; } settings.crewModule
      );
    in
    holdsDrives platform evaluated {
      home = {
        machine = "host-a";
        state = "example/state";
      };
    }
    ++ t.expect (loads evaluated) "crew's teams file of the ${platform} system does not evaluate"
    ++ t.expect (
      t.failures evaluated == [ ]
    ) "the ${platform} system fails: ${toString (t.failures evaluated)}";

  unconfigured =
    platform:
    { system, ... }:
    t.expect (!loads (system (crewWith settings.crewAccounts settings.crewKeys)))
      "crew's teams file of a ${platform} system whose accounts give crew drives its settings do not configure evaluates";

  # crewDrives with the drive `home`'s `key` written in crew's settings too
  # (design Decision 4).
  twice =
    key: value: platform:
    { system, ... }:
    let
      evaluated = system (
        crewWith settings.crewAccounts (
          lib.recursiveUpdate settings.crewDrives { drives.home.${key} = value; }
        )
      );
    in
    t.expect (!loads evaluated)
      "crew's teams file of a ${platform} system whose settings write home's ${key} beside the accounts evaluates"
    ++ t.expect (placed evaluated) "the ${platform} module's definition of services.crew.settings is not placed at services.boomdrive.accounts";
in
onEach "Each drive's machine and state repository, given to crew" givenToCrew
// onEach "Given without the runner" runnerOff
// onEach "An account naming no state repository gives crew nothing" stateless
// onEach "A drive crew's settings do not configure" unconfigured
// onEach "A machine written twice" (twice "machine" "host-a")
// onEach "A state written twice" (twice "state" "example/state")
