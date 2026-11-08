# The service-modules spec on NixOS (design Decisions 10 and 11): the
# module evaluated in a system of x86_64-linux whose normal users are
# alice, the service's, and bob, its unit read key by key, alice's
# lingering, the user activation, and the system profile.
t:
let
  inherit (t) lib;
  settings = import ./settings.nix;
  set = t.outputs.packages.x86_64-linux;

  base = t.nixosSystem { };
  serviceOf = evaluated: evaluated.config.systemd.user.services.boomdrive;
  service = serviceOf base;
  unit = base.config.systemd.user.units."boomdrive.service".text;
  execOf = evaluated: (serviceOf evaluated).serviceConfig.ExecStart;
  # ExecStart's arguments, each quoted as a JSON string.
  argumentsOf =
    evaluated:
    map builtins.fromJSON (
      lib.splitString " " (builtins.unsafeDiscardStringContext (execOf evaluated))
    );
  fileOf = evaluated: lib.elemAt (argumentsOf evaluated) 3;

  # The unit runs the set's runner with the module's file holding `text`,
  # byte for byte.
  runs =
    evaluated: text:
    execOf evaluated == lib.concatMapStringsSep " " builtins.toJSON [
      "${set.boomdrive}/bin/boomdrive"
      "run"
      "--config"
      (evaluated.pkgs.writeText "boomdrive-config.toml" text).outPath
    ];

  says = line: t.expect (lib.elem line (lib.splitString "\n" unit)) "the unit has no line ${line}";

  profileOf = evaluated: map (p: p.outPath) evaluated.config.environment.systemPackages;
  installs =
    evaluated: name: package:
    t.expect (builtins.elem package.outPath (profileOf evaluated)) "the system profile holds no ${name}";
  installsNo =
    evaluated: name: package:
    t.expect (
      !builtins.elem package.outPath (profileOf evaluated)
    ) "the system profile holds the set's ${name}";

  own = t.nixosSystem {
    services.flume.package = null;
    services.sluice.package = null;
  };
  gone = t.nixosSystem { services.boomdrive.tools = [ ]; };
  saysGone = words: lib.hasInfix words (toString (t.failures gone));
  crew = t.nixosSystem { services.boomdrive.settings = settings.crew; };
  hosted = t.nixosSystem { services.boomdrive.settings = settings.hosted; };
  # The runner's settings `given`, beside crew's module turned on with the
  # host `host-a` and crew's settings `teams`.
  crewWith =
    given: teams:
    t.nixosSystem {
      services.boomdrive.settings = given;
      services.crew = {
        enable = true;
        host = "host-a";
        settings = teams;
      };
    };
  fleet = crewWith settings.crewUnwritten settings.crewFleet;
  # "A host under crew" whose crew table, written out, is the partition
  # `garden` alone, beside crew's fleet.
  garden = {
    host = "host-a";
    partitions = [
      {
        label = "garden";
        state_repository = "example/garden";
      }
    ];
  };
  gardenSettings = settings.crew // {
    control_plane = settings.crew.control_plane // {
      crew = garden;
    };
  };
  writtenOut = crewWith gardenSettings settings.crewFleet;
  writtenTable = writtenOut.config.services.boomdrive.settings.control_plane.crew;
  render = import ../modules/toml.nix { inherit lib; };
  # Another control plane, or none, beside crew's module with one
  # partition on host-a.
  hostedBesideCrew = crewWith settings.hosted settings.crewModule;
  noneBesideCrew = crewWith settings.empty settings.crewModule;
  oldSystemd = t.nixosSystem (
    { pkgs, ... }:
    {
      services.boomdrive.settings = settings.hosted;
      systemd.package = pkgs.systemd.overrideAttrs {
        version = "255";
        __intentionallyOverridingVersion = true;
      };
    }
  );
  saysOldSystemd = words: lib.hasInfix words (toString (t.failures oldSystemd));
  noUser = t.nixosSystem { services.boomdrive.user = lib.mkForce "carol"; };
  saysNoUser = words: lib.hasInfix words (toString (t.failures noUser));

  path = lib.splitString ":" service.environment.PATH;
  # The module's own environment, beside what the system gives every unit.
  environment = builtins.attrNames (
    removeAttrs service.environment (builtins.attrNames base.config.systemd.globalEnvironment)
  );
  before =
    entry:
    let
      at = lib.lists.findFirstIndex (e: e == entry) null path;
      system = lib.lists.findFirstIndex (e: e == "/run/current-system/sw/bin") null path;
    in
    t.expect (at != null && system != null && at < system)
      "the unit's PATH does not name ${entry} before /run/current-system/sw/bin: ${service.environment.PATH}";

  # Compared as text: a regular expression may not hold a store path.
  activation = builtins.unsafeDiscardStringContext base.config.system.userActivationScripts.script;
  activate = ../modules/activate.sh;
  inherit (base.pkgs) runtimeShell;
  keepPath = lib.makeBinPath [
    base.pkgs.procps
    base.config.systemd.package
  ];
  # alice's alone: the link, then keep with the unit's own command line,
  # given ps and systemctl.
  keeps = builtins.unsafeDiscardStringContext ''
    if [ "$(id -un)" = alice ]; then
      ${runtimeShell} ${activate} link ${fileOf base} || true
      PATH=${keepPath}:$PATH \
        ${runtimeShell} ${activate} keep alice ${lib.escapeShellArgs (argumentsOf base)}
    fi
  '';
in
{
  "The unit" =
    t.expect (t.failures base == [ ]) "the system fails: ${toString (t.failures base)}"
    ++ t.expect (runs base (builtins.readFile ./files/empty.toml)) "the unit's ExecStart is ${execOf base}, not the set's runner with the file of nix/tests/files/empty.toml"
    ++ t.expect (
      service.wantedBy == [ "default.target" ]
    ) "the unit is wanted by ${toString service.wantedBy}"
    ++ says "ConditionUser=alice"
    ++ says "Restart=always"
    ++ says "RestartSec=30s"
    ++ says "StartLimitIntervalSec=0"
    ++ says "KillMode=process"
    ++ t.expect (base.config.users.users.alice.linger == true) "alice does not linger"
    ++ t.expect (base.config.users.users.bob.linger != true) "bob lingers";

  "No such user on NixOS" =
    t.expect (t.failures noUser != [ ]) "the module evaluates for a user users.users does not declare"
    ++
      lib.concatMap
        (
          words: t.expect (saysNoUser words) "the failure names no ${words}: ${toString (t.failures noUser)}"
        )
        [
          "services.boomdrive.user"
          "users.users.carol"
        ];

  "A hosted control plane on an old systemd" =
    t.expect (
      t.failures oldSystemd != [ ]
    ) "the module evaluates for a hosted control plane on systemd 255"
    ++
      lib.concatMap
        (
          words:
          t.expect (saysOldSystemd words) "the failure says no ${words}: ${toString (t.failures oldSystemd)}"
        )
        [
          "systemd-creds"
          "256"
          "255"
        ];

  "A host under a hosted control plane on NixOS" =
    t.expect (t.failures hosted == [ ])
      "hosted settings fail on systemd ${hosted.config.systemd.package.version}: ${toString (t.failures hosted)}"
    ++ t.expect (runs hosted (builtins.readFile ./files/hosted.toml)) "the unit's --config is not the file of nix/tests/files/hosted.toml";

  "The set's tools on NixOS" =
    installs base "runner" set.boomdrive
    ++ installs base "Flume" set.flume
    ++ installs base "sluice" set.sluice;

  "A host's own tools on NixOS" =
    installs own "runner" set.boomdrive
    ++ installsNo own "Flume" set.flume
    ++ installsNo own "sluice" set.sluice;

  "The tools option is gone on NixOS" =
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

  "The user's profile first on NixOS" =
    before "/etc/profiles/per-user/alice/bin"
    ++ before "/home/alice/.nix-profile/bin"
    ++ t.expect (
      !lib.any (lib.hasPrefix builtins.storeDir) path
    ) "the unit's PATH names a package's own folder: ${service.environment.PATH}"
    ++ t.expect (
      environment == [ "PATH" ]
    ) "the unit's own environment is ${toString environment}, not PATH alone";

  "A host under crew on NixOS" =
    t.expect (t.failures crew == [ ]) "crew's settings fail: ${toString (t.failures crew)}"
    ++ t.expect (runs crew (builtins.readFile ./files/crew.toml)) "the unit's --config is not the file of nix/tests/files/crew.toml";

  "A table written out wins on NixOS" =
    t.expect (
      t.failures writtenOut == [ ]
    ) "a table written out fails: ${toString (t.failures writtenOut)}"
    ++
      t.expect (writtenTable == garden)
        "settings.control_plane.crew is ${builtins.toJSON writtenTable}, not the table written, ${builtins.toJSON garden}"
    ++ t.expect (runs writtenOut (render gardenSettings)) "the unit's --config is not the rendering of the settings with the table written";

  "A hosted control plane beside crew on NixOS" =
    t.expect (
      t.failures hostedBesideCrew == [ ]
    ) "a hosted control plane beside crew fails: ${toString (t.failures hostedBesideCrew)}"
    ++ t.expect (runs hostedBesideCrew (builtins.readFile ./files/hosted.toml)) "the unit's --config beside crew is not the file of nix/tests/files/hosted.toml";

  "No control plane beside crew on NixOS" =
    t.expect (
      t.failures noneBesideCrew == [ ]
    ) "no control plane beside crew fails: ${toString (t.failures noneBesideCrew)}"
    ++ t.expect (runs noneBesideCrew (builtins.readFile ./files/empty.toml)) "the unit's --config beside crew is not the file of nix/tests/files/empty.toml";

  "crew's partitions on this host on NixOS" =
    t.expect (
      t.failures fleet == [ ]
    ) "crew's partitions on this host fail: ${toString (t.failures fleet)}"
    ++ t.expect (runs fleet (builtins.readFile ./files/crew.toml)) "the unit's --config is not the file of nix/tests/files/crew.toml";

  "The user activation on NixOS" =
    t.expect (lib.hasInfix keeps activation) "the user activation is not `${keeps}`: ${activation}";
}
