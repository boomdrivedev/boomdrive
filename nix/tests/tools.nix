# The service-modules spec's cases on the tools' own modules (design
# Decisions 4 and 5): Flume's, sluice's and crew's modules, each as its
# flake gives it, imported by the platform's Boomdrive module and given
# again beside it, and what the Boomdrive module sets of their options.
# Each case runs on nix-darwin and on NixOS, unless it names a platform.
t:
let
  inherit (t) inputs lib outputs;
  settings = import ./settings.nix;

  tools = [
    "crew"
    "flume"
    "sluice"
  ];

  platforms = {
    "nix-darwin" = {
      modules = outputs.darwinModules;
      set = outputs.packages.aarch64-darwin;
      system = t.darwinSystem;
      systemOf = t.darwinOf;
    };
    NixOS = {
      modules = outputs.nixosModules;
      set = outputs.packages.x86_64-linux;
      system = t.nixosSystem;
      systemOf = t.nixosOf;
    };
  };

  onEach =
    scenario: check:
    lib.mapAttrs' (platform: host: {
      name = "${scenario} on ${platform}";
      value = check platform host;
    }) platforms;

  # The files an option is written in, as nix records each declaration's
  # position whatever `_file` its module carries: sluice's and crew's
  # modules carry none, so their `declarations` name the file that imports
  # them.
  filesOf = option: map (position: toString position.file) option.declarationPositions;
  # Each option of `services.<tool>` written outside the tool's input, by
  # its name and its files.
  outsideOf =
    tool: options:
    let
      source = "${toString inputs.${tool}.outPath}/";
    in
    lib.concatMap (
      option:
      let
        files = filesOf option;
      in
      lib.optional (
        files == [ ] || !lib.all (lib.hasPrefix source) files
      ) "${lib.showOption option.loc} in ${toString files}"
    ) (lib.collect lib.isOption options);

  ownOptions =
    platform:
    { system, ... }:
    let
      inherit (system { }) options;
    in
    lib.concatMap (
      tool:
      let
        outside = outsideOf tool options.services.${tool};
      in
      t.expect (options.services ? ${tool}) "the ${platform} system declares no services.${tool}"
      ++ t.expect (
        outside == [ ]
      ) "the ${platform} system declares ${lib.concatStringsSep ", " outside}, outside ${tool}'s input"
    ) tools;

  reExported =
    platform:
    { modules, systemOf, ... }:
    let
      names = [
        "boomdrive"
        "crew"
        "default"
        "flume"
        "sluice"
      ];
      inherit (systemOf [ modules.sluice ]) options;
      outside = outsideOf "sluice" options.services.sluice;
    in
    t.expect (
      builtins.attrNames modules == names
    ) "the ${platform} modules are ${toString (builtins.attrNames modules)}, not ${toString names}"
    ++ t.expect (
      options.services ? sluice.enable
    ) "a ${platform} system of sluice's module alone declares no services.sluice.enable"
    ++
      t.expect (outside == [ ])
        "a ${platform} system of sluice's module alone declares ${lib.concatStringsSep ", " outside}, outside sluice's input"
    ++ t.expect (
      !(options.services ? boomdrive)
    ) "a ${platform} system of sluice's module alone declares services.boomdrive";

  # A system of `default` and `boomdrive` both: nix refuses an option
  # declared twice, so it evaluates only where the two are taken once.
  twice =
    platform:
    { modules, system, ... }:
    let
      failures = builtins.tryEval (
        builtins.deepSeq (t.failures (system modules.boomdrive)) (t.failures (system modules.boomdrive))
      );
    in
    t.expect failures.success "a ${platform} system that imports default and boomdrive does not evaluate"
    ++ t.expect (
      !failures.success || failures.value == [ ]
    ) "a ${platform} system that imports default and boomdrive fails: ${toString failures.value}";

  # crew turned on, with its host and one partition.
  crewOn = {
    services.crew = {
      enable = true;
      host = "host-a";
      settings = settings.crewModule;
    };
  };

  profileOf = evaluated: map (p: p.outPath) evaluated.config.environment.systemPackages;

  runnerAlone =
    platform:
    { system, ... }:
    let
      evaluated = system { };
      inherit (evaluated.config.services) crew flume sluice;
    in
    t.expect flume.enable "a ${platform} system of the runner alone leaves Flume's module off"
    ++ t.expect sluice.enable "a ${platform} system of the runner alone leaves sluice's module off"
    ++ t.expect (!crew.enable) "a ${platform} system of the runner alone turns crew's module on"
    ++ t.expect (
      t.failures evaluated == [ ]
    ) "a ${platform} system of the runner alone fails: ${toString (t.failures evaluated)}";

  setsPackages =
    platform:
    { system, set, ... }:
    let
      evaluated = system crewOn;
      profile = profileOf evaluated;
    in
    t.expect (
      t.failures evaluated == [ ]
    ) "a ${platform} system with crew on fails: ${toString (t.failures evaluated)}"
    ++
      lib.concatMap
        (
          name:
          t.expect (builtins.elem set.${name}.outPath
            profile
          ) "the ${platform} system profile holds no set's ${name}"
        )
        [
          "boomdrive"
          "crew"
          "flume"
          "sluice"
        ];

  # The three tools' users on NixOS, crew on, and again with sluice's own.
  usersOf =
    given:
    let
      inherit ((t.nixosSystem given).config.services) crew flume sluice;
    in
    {
      crew = crew.user;
      flume = flume.user;
      sluice = sluice.user;
    };
  runnersUsers = usersOf crewOn;
  sluicesUser = usersOf {
    imports = [
      crewOn
      { services.sluice.user = "bob"; }
    ];
  };
in
onEach "Each tool's options are its own flake's" ownOptions
// onEach "The tools' modules re-exported" reExported
// onEach "Imported twice" twice
// onEach "A host with the runner alone" runnerAlone
// onEach "The set's packages" setsPackages
// {
  "The tools' user on NixOS" =
    t.expect (
      runnersUsers == {
        crew = "alice";
        flume = "alice";
        sluice = "alice";
      }
    ) "the tools' users of alice's runner are ${builtins.toJSON runnersUsers}, not alice's"
    ++ t.expect (sluicesUser.sluice == "bob") "sluice's user, set to bob, is ${sluicesUser.sluice}"
    ++ t.expect (
      sluicesUser.flume == "alice" && sluicesUser.crew == "alice"
    ) "Flume's and crew's users, beside sluice's bob, are ${builtins.toJSON sluicesUser}";
}
