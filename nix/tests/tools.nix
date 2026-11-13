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
  # crew turned on, with its host, one drive and its hosts.
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
  # link, and on nix-darwin Pending You's folder and herdr's plugins' links
  # (design Decisions 7 and 8 of the change
  # crews-companions-come-with-the-flake) and the managed telemetry
  # document's step (design Decision 10 of the change
  # every-claude-code-session-exports-to-the-hosts-collector), and nothing
  # else to the tools' activations.
  render = import ../modules/toml.nix { inherit lib; };
  inherit (import ../modules/accounts.nix { inherit lib; }) fileOf;
  linesOf = text: lib.filter (line: line != "") (lib.splitString "\n" text);

  darwinRunner = t.darwinSystem crewOn;
  runnerLink = builtins.unsafeDiscardStringContext ''
    sudo --user=alice -- /usr/bin/env HOME=/Users/alice \
      /bin/sh ${../modules/activate.sh} link ${darwinRunner.pkgs.writeText "boomdrive-config.toml" (render (fileOf darwinRunner.config.services.boomdrive))} || true
  '';
  darwinText = darwinActivation darwinRunner;
  pluginsLink = line: lib.hasInfix "-activate.sh plugins " line;
  folderWrite = line: lib.hasInfix "-activate.sh pendingyou " line;
  managedStep = line: lib.hasInfix "-activate.sh managed " line;
  darwinRest = lib.filter (line: !pluginsLink line && !folderWrite line && !managedStep line) (
    linesOf (lib.replaceStrings [ runnerLink ] [ "" ] darwinText)
  );
  darwinTools = linesOf (darwinActivation t.toolsDarwinSystem);

  # The runner's crew table, under crew with no table written, for crew's
  # drives `garden` and `home` on host-a with `away` on host-b written
  # between them: the table holds them in label order (design Decision 3).
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
    ) "a ${platform} system of crew's drives in label order fails: ${toString (t.failures evaluated)}"
    ++ t.expect (
      table == ordered
    ) "the ${platform} runner's crew table is ${builtins.toJSON table}, not ${builtins.toJSON ordered}";

  # Each of `words` in the failures of `evaluated`, which fails.
  failsNaming =
    what: evaluated: words:
    let
      failures = toString (t.failures evaluated);
    in
    t.expect (t.failures evaluated != [ ]) "${what} evaluates"
    ++ lib.concatMap (
      word: t.expect (lib.hasInfix word failures) "the failure of ${what} names no ${word}: ${failures}"
    ) words;

  # Under crew, with no crew table written and crew's module off (design
  # Decision 6).
  neither =
    platform:
    { system, ... }:
    failsNaming "a ${platform} system under crew with neither a table nor crew's module"
      (system { services.boomdrive = settings.crewUnwritten; })
      [
        "settings.control_plane.crew"
        "services.crew"
      ];

  # Under crew, with no crew table written and crew's one drive on host-b
  # while crew's host is host-a (design Decision 6).
  elsewhere =
    platform:
    { system, ... }:
    failsNaming "a ${platform} system under crew with no drive of crew's on its host"
      (system {
        imports = [ (crewOf settings.crewElsewhere) ];
        services.boomdrive = settings.crewUnwritten;
      })
      [
        "services.crew.host"
        "host-a"
        "host-b"
      ];

  # Whether crew's teams file, as crew's module renders it in `evaluated`,
  # evaluates whole: the module system throws for an option used with no
  # value and for a definition of an option a submodule does not declare,
  # and tryEval catches both, so success alone is read (design Decision 6).
  loads =
    evaluated:
    (builtins.tryEval (builtins.deepSeq evaluated.config.services.crew.settings true)).success;

  # A host on the set's crew loads crew's teams file at version 6, written
  # here as 6 and not read from crew, so a crew release that moves it fails
  # this case in the change that moves the pin (core.45; design Decision 6).
  setsCrew =
    platform:
    { system, ... }:
    let
      evaluated = system crewOn;
      inherit (evaluated.config.services.crew.settings) version;
    in
    t.expect (
      t.failures evaluated == [ ]
    ) "a ${platform} system on the set's crew fails: ${toString (t.failures evaluated)}"
    ++ t.expect (loads evaluated) "crew's teams file of a ${platform} system on the set's crew does not evaluate"
    ++
      t.expect (version == 6)
        "crew's teams file of a ${platform} system on the set's crew is version ${toString version}, not 6";

  # crew's `hosts`, and the drive `home`'s `runs` and `review`, are what
  # the host wrote: the module adds and changes none of them (design
  # Decision 4).
  crewsKeys =
    platform:
    { system, ... }:
    let
      given = settings.crewModule;
      written = (system crewOn).config.services.crew.settings;
    in
    t.expect (written.hosts == given.hosts)
      "crew's hosts on ${platform} are ${builtins.toJSON written.hosts}, not ${builtins.toJSON given.hosts}"
    ++
      t.expect (written.drives.home.runs == given.drives.home.runs)
        "the drive home's runs on ${platform} are ${builtins.toJSON written.drives.home.runs}, not ${builtins.toJSON given.drives.home.runs}"
    ++
      t.expect (written.drives.home.review == given.drives.home.review)
        "the drive home's review on ${platform} is ${builtins.toJSON written.drives.home.review}, not ${builtins.toJSON given.drives.home.review}";

  # crew's old names, its teams file's version 5's, fail evaluation (design
  # Decision 6).
  oldRuns =
    platform:
    { system, ... }:
    t.expect (
      !loads (system (crewOf settings.crewOldRuns))
    ) "a ${platform} system whose crew drive names flume in place of runs evaluates";
  oldReview =
    platform:
    { system, ... }:
    t.expect (
      !loads (system (crewOf settings.crewOldReview))
    ) "a ${platform} system whose crew drive's review is every and hours evaluates";

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
// onEach "crew's drives in label order" crewOrder
// onEach "Under crew with neither" neither
// onEach "No drive of crew's on this host" elsewhere
// onEach "A host on the set's crew" setsCrew
// onEach "crew's keys stay the host's" crewsKeys
// onEach "crew's old way of running" oldRuns
// onEach "crew's old batch names" oldReview
// {
  # The README's install with nix writes no crew table beside crew's
  # drives, and says the module writes it (design Decision 8).
  "The README's crew table" =
    t.expect (lib.any (lib.hasInfix ''type = "crew"'') t.nixBlocks) ''no nix block of the README's Installing with nix sets type = "crew"''
    ++ t.expect (
      !lib.any (lib.hasInfix "state_repository") t.nixBlocks
    ) "a nix block of the README's Installing with nix names state_repository"
    ++
      lib.concatMap
        (
          words:
          t.expect (lib.hasInfix words (toString t.installing)) "the README's Installing with nix says no ${words}"
        )
        [
          "services.crew.host"
          "written out wins"
        ];

  # The README's install with nix writes crew's settings by crew's names,
  # its teams file's version 6's, and none of version 5's. The leading
  # space of ` flume = {` tells a drive's key from `services.flume = {`,
  # and of ` every =` and ` hours =` review's old keys from
  # `after_hours =` (design Decisions 6 and 8).
  "The README's crew settings" =
    let
      crewBlock = lib.findFirst (lib.hasInfix "services.crew") null t.nixBlocks;
    in
    t.expect (crewBlock != null) "no nix block of the README's Installing with nix names services.crew"
    ++
      lib.concatMap
        (
          word:
          t.expect (
            crewBlock != null && lib.hasInfix word crewBlock
          ) "the README's nix block of services.crew names no ${word}"
        )
        [
          "drives"
          "runs"
          "after_changes"
          "after_hours"
          "hosts"
          "ssh ="
          "kind ="
        ]
    ++
      lib.concatMap
        (
          word:
          t.expect (
            !lib.any (lib.hasInfix word) t.nixBlocks
          ) "a nix block of the README's Installing with nix names ${word}"
        )
        [
          "partitions"
          " flume = {"
          " every ="
          " hours ="
        ];

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
    ++ t.expect (
      builtins.length (lib.filter folderWrite (linesOf darwinText)) == 1
    ) "the nix-darwin activation with crew writes Pending You's folder other than once: ${darwinText}"
    ++ t.expect (
      builtins.length (lib.filter pluginsLink (linesOf darwinText)) == 1
    ) "the nix-darwin activation with crew links herdr's plugins other than once: ${darwinText}"
    ++
      t.expect (builtins.length (lib.filter managedStep (linesOf darwinText)) == 1)
        "the nix-darwin activation with crew runs the managed telemetry document's step other than once: ${darwinText}"
    ++
      t.expect (darwinRest == darwinTools)
        "the nix-darwin activation less the runner's link, Pending You's folder, the plugins' and the managed telemetry document's step is ${builtins.toJSON darwinRest}, not the tools' own ${builtins.toJSON darwinTools}";

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
