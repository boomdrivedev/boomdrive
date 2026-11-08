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

  # Each tool's file, by the name its module writes it under.
  fileNames = {
    crew = "crew-teams.json";
    flume = "flume-config.toml";
    sluice = "sluice-config.toml";
  };

  # nix-darwin's activation, as text: a regular expression may not hold a
  # store path.
  darwinActivation =
    evaluated:
    builtins.unsafeDiscardStringContext evaluated.config.system.activationScripts.postActivation.text;
  # NixOS's user activation snippets, by name, as the modules define them:
  # the option's value is the one script they make.
  userScripts =
    evaluated: lib.foldl' (a: b: a // b) { } evaluated.options.system.userActivationScripts.definitions;

  platforms = {
    "nix-darwin" = {
      modules = outputs.darwinModules;
      set = outputs.packages.aarch64-darwin;
      system = t.darwinSystem;
      systemOf = t.darwinOf;
      # A tool's activation: the lines of the system's that name its file.
      activationOf =
        evaluated: tool:
        lib.concatStringsSep "\n" (
          lib.filter (lib.hasInfix fileNames.${tool}) (lib.splitString "\n" (darwinActivation evaluated))
        );
    };
    NixOS = {
      modules = outputs.nixosModules;
      set = outputs.packages.x86_64-linux;
      system = t.nixosSystem;
      systemOf = t.nixosOf;
      # A tool's activation: its user activation snippet's text.
      activationOf =
        evaluated: tool: builtins.unsafeDiscardStringContext ((userScripts evaluated).${tool}.text or "");
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

  # crew turned on, with its host `host-a` and crew's settings `teams`.
  crewOf = teams: {
    services.crew = {
      enable = true;
      host = "host-a";
      settings = teams;
    };
  };
  # crew turned on, with its host and one partition.
  crewOn = crewOf settings.crewModule;

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

  # A host's `enable` or `package` of a tool, over the Boomdrive module's
  # (design Decision 5).
  ownFlumeFile =
    platform:
    {
      system,
      set,
      activationOf,
      ...
    }:
    let
      evaluated = system { services.flume.enable = false; };
    in
    t.expect (
      t.failures evaluated == [ ]
    ) "a ${platform} system that keeps its own Flume file fails: ${toString (t.failures evaluated)}"
    ++
      t.expect (activationOf evaluated "flume" == "")
        "the ${platform} activation of a host that keeps its own Flume file links Flume's: ${activationOf evaluated "flume"}"
    ++ t.expect (
      !builtins.elem set.flume.outPath (profileOf evaluated)
    ) "the ${platform} system profile of a host that keeps its own Flume file holds the set's Flume";

  ownCrew =
    platform:
    {
      system,
      set,
      activationOf,
      ...
    }:
    let
      setOne = system crewOn;
      own = system (
        { pkgs, ... }:
        {
          imports = [ crewOn ];
          # Laid out as crew's flake lays its package out.
          services.crew.package = pkgs.runCommand "crew-own" { } ''
            mkdir -p "$out/bin" "$out/plugin"
            touch "$out/bin/crew"
          '';
        }
      );
      profile = profileOf own;
      linked = activationOf own "crew";
    in
    t.expect (builtins.elem own.config.services.crew.package.outPath profile) "the ${platform} system profile holds no host's own crew"
    ++ t.expect (
      !builtins.elem set.crew.outPath profile
    ) "the ${platform} system profile holds the set's crew beside the host's own"
    ++ t.expect (lib.hasInfix fileNames.crew linked) "the ${platform} activation with the host's own crew links no crew's file"
    ++
      t.expect (linked == activationOf setOne "crew")
        "the ${platform} activation of crew's file with the host's own crew is ${linked}, not ${activationOf setOne "crew"}";

  noSluice =
    platform:
    {
      system,
      set,
      activationOf,
      ...
    }:
    let
      evaluated = system { services.sluice.package = null; };
    in
    t.expect (
      !builtins.elem set.sluice.outPath (profileOf evaluated)
    ) "the ${platform} system profile with no sluice holds the set's sluice"
    ++ t.expect (lib.hasInfix fileNames.sluice (activationOf evaluated "sluice")) "the ${platform} activation with no sluice links no sluice's file";

  # The runner's system with crew on beside one of the tools' own modules
  # alone (design Decision 5): the Boomdrive module adds the runner's own
  # link and nothing else to the tools' activations.
  render = import ../modules/toml.nix { inherit lib; };
  linesOf = text: lib.filter (line: line != "") (lib.splitString "\n" text);

  darwinRunner = t.darwinSystem crewOn;
  runnerLink = builtins.unsafeDiscardStringContext ''
    sudo --user=alice -- /usr/bin/env HOME=/Users/alice \
      /bin/sh ${../modules/activate.sh} link ${darwinRunner.pkgs.writeText "boomdrive-config.toml" (render darwinRunner.config.services.boomdrive.settings)} || true
  '';
  darwinText = darwinActivation darwinRunner;
  darwinRest = linesOf (lib.replaceStrings [ runnerLink ] [ "" ] darwinText);
  darwinTools = linesOf (darwinActivation t.toolsDarwinSystem);

  # The runner's crew table, under crew with no table written, for crew's
  # partitions `garden` and `home` on host-a with `away` on host-b between
  # them (design Decision 3).
  crewOrder =
    platform:
    { system, ... }:
    let
      evaluated = system {
        imports = [ (crewOf settings.crewOrder) ];
        services.boomdrive.settings.control_plane.type = "crew";
      };
      table = evaluated.config.services.boomdrive.settings.control_plane.crew;
      ordered = {
        host = "host-a";
        partitions = [
          {
            label = "garden";
            state_repository = "example/garden";
          }
          {
            label = "home";
            state_repository = "example/state";
          }
        ];
      };
    in
    t.expect (
      t.failures evaluated == [ ]
    ) "a ${platform} system of crew's order fails: ${toString (t.failures evaluated)}"
    ++ t.expect (
      table == ordered
    ) "the ${platform} runner's crew table is ${builtins.toJSON table}, not ${builtins.toJSON ordered}";

  nixosRunner = userScripts (t.nixosSystem crewOn);
  nixosTools = userScripts t.toolsNixosSystem;
  nixosNames = builtins.attrNames (removeAttrs nixosRunner [ "boomdrive" ]);
  textOf = scripts: tool: builtins.unsafeDiscardStringContext (scripts.${tool}.text or "");
in
onEach "Each tool's options are its own flake's" ownOptions
// onEach "The tools' modules re-exported" reExported
// onEach "Imported twice" twice
// onEach "A host with the runner alone" runnerAlone
// onEach "The set's packages" setsPackages
// onEach "A host that keeps its own Flume file" ownFlumeFile
// onEach "A host's own crew" ownCrew
// onEach "No sluice of the set's" noSluice
// onEach "crew's order" crewOrder
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

  "The tools' activation is their own on nix-darwin" =
    t.expect (t.failures t.toolsDarwinSystem == [ ])
      "the nix-darwin system of the tools' own modules fails: ${toString (t.failures t.toolsDarwinSystem)}"
    ++ t.expect (lib.hasInfix runnerLink darwinText) "the nix-darwin activation runs no `${runnerLink}`: ${darwinText}"
    ++
      t.expect (darwinRest == darwinTools)
        "the nix-darwin activation less the runner's link is ${builtins.toJSON darwinRest}, not the tools' own ${builtins.toJSON darwinTools}";

  "The tools' activation is their own on NixOS" =
    t.expect (
      t.failures t.toolsNixosSystem == [ ]
    ) "the NixOS system of the tools' own modules fails: ${toString (t.failures t.toolsNixosSystem)}"
    ++ t.expect (nixosRunner ? boomdrive) "the NixOS user activation has no runner's snippet"
    ++
      t.expect (nixosNames == builtins.attrNames nixosTools)
        "the NixOS user activation's snippets less the runner's are ${toString nixosNames}, not the tools' own ${toString (builtins.attrNames nixosTools)}"
    ++
      lib.concatMap
        (
          tool:
          t.expect (textOf nixosRunner tool != "" && textOf nixosRunner tool == textOf nixosTools tool)
            "the NixOS user activation's ${tool} is ${textOf nixosRunner tool}, not the tool's own ${textOf nixosTools tool}"
        )
        [
          "crew"
          "flume"
          "sluice"
        ];
}
