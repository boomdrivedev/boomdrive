# The service-modules spec's cases on crew's drives in crew's own module
# (the change a-drives-machine-and-state-are-named-once, design Decisions 3,
# 4 and 7; the change crew-runs-only-the-drives-set-up-for-it, design
# Decisions 1 and 5): the Boomdrive module adds to the type of crew's
# drives, giving each drive crew's settings set up, under an account of
# the host's accounts that names its state repository, its machine and its
# account's state repository, with crew's module on, whether or not the
# runner is, and adding no drive to crew's; and a host's second write of
# either fails crew's own check, each given value placed at
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

  # Whether a definition of crew's drive `home`'s `key` is placed at
  # services.boomdrive.accounts, read from the submodules' `valueMeta`,
  # reading no definition's value (the change
  # crew-runs-only-the-drives-set-up-for-it, design Decisions 1 and 5).
  placedFor =
    key: evaluated:
    lib.any (definition: definition.file == "services.boomdrive.accounts")
      evaluated.options.services.crew.settings.valueMeta.configuration.options.drives.valueMeta.attrs.home.configuration.options.${key}.definitionsWithLocations;

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
    ++
      lib.concatMap
        (
          key:
          t.expect (placedFor key evaluated) "the ${platform} module's definition of home's ${key} is not placed at services.boomdrive.accounts"
        )
        [
          "machine"
          "state"
        ];

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

  # crewAccounts, whose drives `garden`, `away` and `field` crew's settings
  # do not set up, beside crewKeys, which sets up `home` alone: crew is
  # given `home` alone, and no drive of the others (the change
  # crew-runs-only-the-drives-set-up-for-it, design Decision 5).
  unconfigured =
    platform:
    { system, ... }:
    let
      evaluated = system (crewWith settings.crewAccounts settings.crewKeys);
    in
    holdsDrives platform evaluated { inherit (given) home; }
    ++ t.expect (loads evaluated) "crew's teams file of the ${platform} system does not evaluate"
    ++ t.expect (
      t.failures evaluated == [ ]
    ) "the ${platform} system fails: ${toString (t.failures evaluated)}";

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
    ++ t.expect (placedFor key evaluated) "the ${platform} module's definition of home's ${key} is not placed at services.boomdrive.accounts";
in
onEach "Each drive's machine and state repository, given to crew" givenToCrew
// onEach "Given without the runner" runnerOff
// onEach "An account naming no state repository gives crew nothing" stateless
// onEach "A drive crew's settings do not configure" unconfigured
// onEach "A machine written twice" (twice "machine" "host-a")
// onEach "A state written twice" (twice "state" "example/state")
