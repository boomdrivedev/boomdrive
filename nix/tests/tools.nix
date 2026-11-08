# The service-modules spec's cases on the tools' own modules (design
# Decisions 4 and 5): Flume's, sluice's and crew's modules, each as its
# flake gives it, imported by the platform's Boomdrive module and given
# again beside it. Each case runs on nix-darwin and on NixOS.
t:
let
  inherit (t) inputs lib outputs;

  tools = [
    "crew"
    "flume"
    "sluice"
  ];

  platforms = {
    "nix-darwin" = {
      modules = outputs.darwinModules;
      system = t.darwinSystem;
      systemOf = t.darwinOf;
    };
    NixOS = {
      modules = outputs.nixosModules;
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
in
onEach "Each tool's options are its own flake's" ownOptions
// onEach "The tools' modules re-exported" reExported
// onEach "Imported twice" twice
