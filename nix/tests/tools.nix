# The service-modules spec's cases on the tools' own modules (design
# Decisions 4 and 5): Flume's, sluice's, crew's and timber's modules, each
# as its flake gives it, imported by the platform's Boomdrive module and
# given again beside it, and what the Boomdrive module sets of their
# options (the change the-set-takes-timbers-own-module, design Decisions 1
# to 5).
# Each case runs on nix-darwin and on NixOS, unless it names a platform.
t:
let
  inherit (t) inputs lib outputs;
  settings = import ./settings.nix;

  tools = [
    "crew"
    "flume"
    "sluice"
    "timber"
  ];

  # Each tool's file, by the name its module writes it under.
  fileNames = {
    crew = "crew-teams.json";
    flume = "flume-config.toml";
    sluice = "sluice-config.toml";
    timber = "timber-config.toml";
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
      # timber's flake's own module for the platform.
      timber = inputs.timber.darwinModules.timber;
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
      timber = inputs.timber.nixosModules.timber;
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
  # The declarations of a tool's option this tree makes again, by the
  # option's name, each with its file: crew-drives.nix's of
  # services.crew.settings and its drives, adding to crew's drive type the
  # module that gives each drive crew's settings set up its machine and
  # state (the change crew-runs-only-the-drives-set-up-for-it, design
  # Decisions 1 and 5).
  again = lib.genAttrs [
    "services.crew.settings"
    "services.crew.settings.drives"
  ] (_: toString ../modules/crew-drives.nix);
  # The options of an option's submodule type, less the module system's own.
  subOptionsOf =
    option: lib.collect lib.isOption (removeAttrs (option.type.getSubOptions option.loc) [ "_module" ]);
  # Each option of `services.<tool>` with no declaration in the tool's
  # input, or declared again in a file outside it but `again`'s file for
  # its name, by its name and its files; the options of each declaration
  # `again` names checked the same way, so it declares no option the tool's
  # input does not.
  outsideOf =
    tool: options:
    let
      source = "${toString inputs.${tool}.outPath}/";
      check =
        option:
        let
          name = lib.showOption option.loc;
          files = filesOf option;
          admitted = again ? ${name};
          stray = lib.filter (
            file: !(lib.hasPrefix source file) && !(admitted && file == again.${name})
          ) files;
        in
        lib.optional (!lib.any (lib.hasPrefix source) files || stray != [ ]) "${name} in ${toString files}"
        ++ lib.optionals admitted (lib.concatMap check (subOptionsOf option));
    in
    lib.concatMap check (lib.collect lib.isOption options);

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
        "timber"
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

  # A system of the runner's, of the platform's `default` and timber's
  # flake's own module both: timber's key takes it once (the change
  # the-set-takes-timbers-own-module, design Decision 1; Task notes 1.1).
  timberBeside =
    platform:
    { system, timber, ... }:
    let
      failures = builtins.tryEval (
        builtins.deepSeq (t.failures (system timber)) (t.failures (system timber))
      );
    in
    t.expect failures.success "a ${platform} system that imports default and timber's own module does not evaluate"
    ++
      t.expect (!failures.success || failures.value == [ ])
        "a ${platform} system that imports default and timber's own module fails: ${toString failures.value}";

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
      inherit (evaluated.config.services)
        crew
        flume
        sluice
        timber
        ;
    in
    t.expect flume.enable "a ${platform} system of the runner alone leaves Flume's module off"
    ++ t.expect sluice.enable "a ${platform} system of the runner alone leaves sluice's module off"
    ++ t.expect timber.enable "a ${platform} system of the runner alone leaves timber's module off"
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
          "timber"
        ];

  # The four tools' users on NixOS, crew on, and again with sluice's own.
  usersOf =
    given:
    let
      inherit ((t.nixosSystem given).config.services)
        crew
        flume
        sluice
        timber
        ;
    in
    {
      crew = crew.user;
      flume = flume.user;
      sluice = sluice.user;
      timber = timber.user;
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
  # crews-companions-come-with-the-flake), the managed telemetry
  # document's step (design Decision 10 of the change
  # every-claude-code-session-exports-to-the-hosts-collector) and the
  # managed transcripts document's (design Decision 7 of the change
  # the-set-keeps-claude-codes-transcripts-a-year), each by its path, and
  # the install into Claude Code's homes, activate.sh's `claude` (design
  # Decision 10 of the change
  # every-claude-code-session-has-boomdrives-plugins), and nothing else to
  # the tools' activations.
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
  claudeStep = line: lib.hasInfix "-activate.sh claude " line;
  folderWrite = line: lib.hasInfix "-activate.sh pendingyou " line;
  managedStep =
    name: line:
    lib.hasInfix "-activate.sh managed " line
    && lib.hasInfix "/Library/Application Support/ClaudeCode/managed-settings.d/${name}" line;
  telemetryStep = managedStep "10-boomdrive-telemetry.json";
  transcriptsStep = managedStep "10-boomdrive-transcripts.json";
  darwinRest = lib.filter (
    line:
    !pluginsLink line
    && !folderWrite line
    && !claudeStep line
    && !telemetryStep line
    && !transcriptsStep line
  ) (linesOf (lib.replaceStrings [ runnerLink ] [ "" ] darwinText));
  darwinTools = linesOf (darwinActivation t.toolsDarwinSystem);

  # The runner's crew table, under crew with no table written, for the
  # accounts `own`, with the drives `garden` and `home` on host-a, and
  # `work`, with `away` on host-b and `field` on host-a: the table holds
  # this host's in the accounts' order, each account's drives in the order
  # of their names (the change a-drives-machine-and-state-are-named-once,
  # design Decision 5).
  accountsOrder =
    platform:
    { system, ... }:
    let
      evaluated = system {
        imports = [ (crewOf settings.crewDrives) ];
        services.boomdrive = {
          accounts = settings.crewAccounts;
          settings.control_plane.type = "crew";
        };
      };
      table = evaluated.config.services.boomdrive.settings.control_plane.crew;
      ordered = {
        host = "host-a";
        partitions = [
          {
            label = "garden";
            state_repository = "example/state";
          }
          {
            label = "home";
            state_repository = "example/state";
          }
          {
            label = "field";
            state_repository = "example/work";
          }
        ];
      };
    in
    t.expect (t.failures evaluated == [ ])
      "a ${platform} system of the accounts' drives in their order fails: ${toString (t.failures evaluated)}"
    ++ t.expect (
      table == ordered
    ) "the ${platform} runner's crew table is ${builtins.toJSON table}, not ${builtins.toJSON ordered}";

  # crewAccounts beside crewKeys, which sets up `home` alone: the table
  # holds `home` alone of this host's drives (the change
  # crew-runs-only-the-drives-set-up-for-it, design Decision 5).
  setUp =
    platform:
    { system, ... }:
    let
      evaluated = system {
        imports = [ (crewOf settings.crewKeys) ];
        services.boomdrive = {
          accounts = settings.crewAccounts;
          settings.control_plane.type = "crew";
        };
      };
      table = evaluated.config.services.boomdrive.settings.control_plane.crew;
      expected = {
        host = "host-a";
        partitions = [
          {
            label = "home";
            state_repository = "example/state";
          }
        ];
      };
    in
    t.expect (t.failures evaluated == [ ])
      "a ${platform} system of the drives crew's settings set up fails: ${toString (t.failures evaluated)}"
    ++
      t.expect (table == expected)
        "the ${platform} runner's crew table is ${builtins.toJSON table}, not ${builtins.toJSON expected}";

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

  # Under crew, with no crew table written and the accounts' one drive on
  # host-b while crew's host is host-a (design Decision 6).
  elsewhere =
    platform:
    { system, ... }:
    failsNaming "a ${platform} system under crew with no drive of the accounts on its host"
      (system {
        imports = [ (crewOf settings.crewElsewhere) ];
        services.boomdrive = settings.elsewhere;
      })
      [
        "services.crew.host"
        "host-a"
        "host-b"
      ];

  # Under crew, with no crew table written, the drive `home` on this host,
  # which crew's settings do not set up, beside `away` on host-b, which
  # they do (the change crew-runs-only-the-drives-set-up-for-it, design
  # Decision 5).
  elsewhereHere =
    platform:
    { system, ... }:
    failsNaming "a ${platform} system under crew with no drive crew runs on its host"
      (system {
        imports = [ (crewOf settings.crewElsewhere) ];
        services.boomdrive = settings.elsewhereHome;
      })
      [
        "services.crew.host"
        "host-a"
        "host-b"
      ];

  # Under crew, with no crew table written, an account with a drive and no
  # state repository, beside crew's settings that write the drive's machine
  # and state (the change a-drives-machine-and-state-are-named-once, design
  # Decision 6).
  stateless =
    platform:
    { system, ... }:
    failsNaming "a ${platform} system under crew whose account names no state repository" (system {
      imports = [ (crewOf settings.crewModule) ];
      services.boomdrive = settings.stateless;
    }) [ "services.boomdrive.accounts.own.state_repository" ];

  # Under crew, with no crew table written, the account `work`, naming no
  # state repository, whose drive `field` crew's settings do not set up,
  # beside `own`'s `home`, which they do: crew has `home` alone, the table
  # holds it, and Flume is given both drives' streams (the change
  # crew-runs-only-the-drives-set-up-for-it, design Decision 5).
  reading =
    platform:
    { system, ... }:
    let
      evaluated = system {
        imports = [ (crewOf settings.crewKeys) ];
        services.boomdrive = settings.reading;
      };
      names = lib.attrNames evaluated.config.services.crew.settings.drives;
      table = evaluated.config.services.boomdrive.settings.control_plane.crew;
      expected = {
        host = "host-a";
        partitions = [
          {
            label = "home";
            state_repository = "example/state";
          }
        ];
      };
      streams = evaluated.config.services.flume.settings.streams;
    in
    t.expect (t.failures evaluated == [ ])
      "a ${platform} system whose account's drives crew's settings do not set up fails: ${toString (t.failures evaluated)}"
    ++ t.expect (
      names == [ "home" ]
    ) "the ${platform} system's crew drives are ${toString names}, not home"
    ++ t.expect (
      table == expected
    ) "the ${platform} runner's crew table is ${builtins.toJSON table}, not ${builtins.toJSON expected}"
    ++
      t.expect (streams ? home && streams ? field)
        "the ${platform} system's Flume streams are ${toString (lib.attrNames streams)}, not home and field";

  # Under crew, with no crew table written, a drive of crew's, `stray`,
  # that no account names (design Decision 6).
  stray =
    platform:
    { system, ... }:
    failsNaming "a ${platform} system under crew with a drive of crew's no account names"
      (system {
        imports = [ (crewOf settings.crewStray) ];
        services.boomdrive = settings.crewUnwritten;
      })
      [
        "stray"
        "services.boomdrive.accounts"
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

  # The configuration's own module of host-a in the Mac's flake README
  # shows, examples/host-flake.nix (record 0049): its outputs called with a
  # nix-darwin whose darwinSystem gives back its arguments and a set whose
  # module is a marker.
  hostModules =
    ((import ./examples/host-flake.nix).outputs {
      nix-darwin.lib.darwinSystem = arguments: arguments;
      boomdrive.darwinModules.default = "the set's module";
    }).darwinConfigurations.host-a.modules;
  hostModule = lib.elemAt hostModules 1;
in
onEach "Each tool's options are its own flake's" ownOptions
// onEach "The tools' modules re-exported" reExported
// onEach "Imported twice" twice
// onEach "timber's own module beside the set's" timberBeside
// onEach "A host with the runner alone" runnerAlone
// onEach "The set's packages" setsPackages
// onEach "A host that keeps its own Flume file" ownFlumeFile
// onEach "A host's own crew" ownCrew
// onEach "No sluice of the set's" noSluice
// onEach "The accounts' drives in their order" accountsOrder
// onEach "Only the drives crew's settings set up" setUp
// onEach "Under crew with neither" neither
// onEach "No drive of the accounts on this host" elsewhere
// onEach "No drive crew runs on this host" elsewhereHere
// onEach "An account under crew with no state repository" stateless
// onEach "An account whose drives crew's settings do not set up" reading
// onEach "A drive of crew's no account names" stray
// onEach "A host on the set's crew" setsCrew
// onEach "crew's keys stay the host's" crewsKeys
// onEach "crew's old way of running" oldRuns
// onEach "crew's old batch names" oldReview
// {
  # The Mac's flake README shows sets the runner's control plane to crew's
  # and writes no crew table beside crew's drives, which the module writes
  # (design Decision 8; the change a-drives-machine-and-state-are-named-once,
  # design Decision 9), and declares no source of crew's to Flume, which
  # the module declares (Task notes 2.1 of the change
  # the-set-declares-crews-source-to-flume).
  "The host flake's crew table" =
    let
      controlPlane = hostModule.services.boomdrive.settings.control_plane;
    in
    t.expect (
      lib.head hostModules == "the set's module"
    ) "examples/host-flake.nix's host-a takes no darwinModules.default first"
    ++
      t.expect
        (
          controlPlane == {
            type = "crew";
          }
        )
        "examples/host-flake.nix's control_plane is ${builtins.toJSON controlPlane}, not crew's type alone"
    ++ t.expect (
      !(hostModule.services.flume or { } ? sources)
    ) "examples/host-flake.nix sets services.flume.sources";

  # The Mac's flake README shows writes crew's settings by crew's names,
  # its teams file's version 6's, and none of version 5's (design
  # Decisions 6 and 8).
  "The host flake's crew settings" =
    let
      crew = hostModule.services.crew.settings;
      home = crew.drives.home;
      host = crew.hosts.host-a;
    in
    t.expect (home ? runs) "examples/host-flake.nix's drive home has no runs"
    ++
      t.expect (home.review ? after_changes && home.review ? after_hours)
        "examples/host-flake.nix's review is ${builtins.toJSON home.review}, not after_changes and after_hours"
    ++ t.expect (
      host ? ssh && host ? kind
    ) "examples/host-flake.nix's host host-a is ${builtins.toJSON host}, not its ssh and kind"
    ++ t.expect (!(crew ? partitions)) "examples/host-flake.nix's crew settings name partitions"
    ++ t.expect (!(home ? flume)) "examples/host-flake.nix's drive home names flume"
    ++ t.expect (
      !(home.review ? every) && !(home.review ? hours)
    ) "examples/host-flake.nix's review names every or hours";

  # The Mac's flake README shows names the account's crew state repository
  # in services.boomdrive.accounts, and neither a drive's state nor its
  # machine in crew's settings (the change
  # a-drives-machine-and-state-are-named-once, design Decision 9).
  "The host flake's machine and state" =
    let
      account = hostModule.services.boomdrive.accounts.own;
      home = hostModule.services.crew.settings.drives.home;
    in
    t.expect (
      account.state_repository or null == "example/state"
    ) ''examples/host-flake.nix's account own names no state_repository = "example/state"''
    ++ t.expect (!(home ? state)) "examples/host-flake.nix's crew drive home names its state"
    ++ t.expect (!(home ? machine)) "examples/host-flake.nix's crew drive home names its machine";

  # The set's user given to the tools a host without the runner turns on
  # (design Decision 4 of the change
  # boomdrives-tools-install-on-a-machine-without-the-runner).
  "The tools' user without the runner" =
    let
      evaluated = t.nixosOf [
        outputs.nixosModules.default
        {
          services = {
            boomdrive.user = "alice";
            flume.enable = true;
            sluice.enable = true;
          };
        }
      ];
      users = {
        flume = evaluated.config.services.flume.user;
        sluice = evaluated.config.services.sluice.user;
      };
    in
    t.expect (
      t.failures evaluated == [ ]
    ) "the NixOS system without the runner fails: ${toString (t.failures evaluated)}"
    ++ t.expect (
      users == {
        flume = "alice";
        sluice = "alice";
      }
    ) "Flume's and sluice's users without the runner are ${builtins.toJSON users}, not alice's";

  "The tools' user on NixOS" =
    t.expect (
      runnersUsers == {
        crew = "alice";
        flume = "alice";
        sluice = "alice";
        timber = "alice";
      }
    ) "the tools' users of alice's runner are ${builtins.toJSON runnersUsers}, not alice's"
    ++ t.expect (sluicesUser.sluice == "bob") "sluice's user, set to bob, is ${sluicesUser.sluice}"
    ++ t.expect (
      sluicesUser.flume == "alice" && sluicesUser.crew == "alice" && sluicesUser.timber == "alice"
    ) "Flume's, crew's and timber's users, beside sluice's bob, are ${builtins.toJSON sluicesUser}";

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
    ++ t.expect (
      builtins.length (lib.filter claudeStep (linesOf darwinText)) == 1
    ) "the nix-darwin activation with crew runs activate.sh claude other than once: ${darwinText}"
    ++
      t.expect (builtins.length (lib.filter telemetryStep (linesOf darwinText)) == 1)
        "the nix-darwin activation with crew runs the managed telemetry document's step other than once: ${darwinText}"
    ++
      t.expect (builtins.length (lib.filter transcriptsStep (linesOf darwinText)) == 1)
        "the nix-darwin activation with crew runs the managed transcripts document's step other than once: ${darwinText}"
    ++
      t.expect (darwinRest == darwinTools)
        "the nix-darwin activation less the runner's link, Pending You's folder, the plugins', activate.sh claude's and the managed telemetry and transcripts documents' steps is ${builtins.toJSON darwinRest}, not the tools' own ${builtins.toJSON darwinTools}";

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
          "timber"
        ];
}
