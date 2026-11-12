# The service-modules spec's cases on the apps the modules install with
# the runner (apps.nix; design Decisions 5 to 7 of the change
# herdr-and-ghostty-come-with-the-flake-and-an-app-opens-the-agents):
# herdr and the plugins the activation links, Ghostty and the launcher,
# and Flume's terminal and window; and the launcher made for each drive of
# this host (design Decisions 1 and 5 of the change
# each-drive-opens-from-the-launcher), whose accounts are those of
# drives.nix's cases. Each case runs on nix-darwin and on NixOS, unless it
# names a platform. portless's cases are design Decisions 5 and 6 of the
# change portless-comes-with-the-flake, worktrunk's design Decision 3 of
# the change worktrunk-comes-with-the-flake, and crew's companions' design
# Decisions 6 and 9 of the change crews-companions-come-with-the-flake.
t:
let
  inherit (t) lib outputs;

  # Each platform's module (design Decision 5).
  platforms = {
    "nix-darwin" = {
      system = t.darwinSystem;
      without = t.darwinOf [ outputs.darwinModules.default ];
      withCrew = t.darwinOf [
        outputs.darwinModules.default
        crewOn
      ];
      set = outputs.packages.aarch64-darwin;
    };
    NixOS = {
      system = t.nixosSystem;
      without = t.nixosOf [ outputs.nixosModules.default ];
      withCrew = t.nixosOf [
        outputs.nixosModules.default
        crewOn
        { services.crew.user = "alice"; }
      ];
      set = outputs.packages.x86_64-linux;
    };
  };
  onEach =
    scenario: check:
    lib.mapAttrs' (platform: host: {
      name = "${scenario} on ${platform}";
      value = check platform host;
    }) platforms;

  profileOf = evaluated: map (p: p.outPath) evaluated.config.environment.systemPackages;
  apps = [
    "herdr"
    "ghostty"
    "boomdrive-launcher"
  ];

  withRunner =
    platform:
    { system, set, ... }:
    let
      evaluated = system { };
      profile = profileOf evaluated;
    in
    t.expect (
      t.failures evaluated == [ ]
    ) "a ${platform} system with the runner fails: ${toString (t.failures evaluated)}"
    ++ lib.concatMap (
      name:
      t.expect (builtins.elem set.${name}.outPath
        profile
      ) "the ${platform} system profile holds no set's ${name}"
    ) apps;

  # A herdr of the host's own, laid out as a package with a bin.
  ownHerdr =
    { pkgs, ... }:
    {
      services.boomdrive.herdr.package = pkgs.runCommand "own-herdr" { } "mkdir -p $out/bin";
    };
  hostsHerdr =
    platform:
    { system, set, ... }:
    let
      evaluated = system ownHerdr;
      profile = profileOf evaluated;
      own = builtins.unsafeDiscardStringContext evaluated.config.services.boomdrive.herdr.package.outPath;
    in
    t.expect (builtins.elem evaluated.config.services.boomdrive.herdr.package.outPath profile) "the ${platform} system profile holds no host's own herdr"
    ++ t.expect (
      !builtins.elem set.herdr.outPath profile
    ) "the ${platform} system profile holds the set's herdr beside the host's own"
    ++ t.expect (lib.any (lib.hasInfix "-activate.sh plugins ${own}/bin/herdr ") (pluginSteps platform evaluated)) "the ${platform} activation links the plugins with no ${own}/bin/herdr: ${builtins.toJSON (pluginSteps platform evaluated)}";

  noGhostty =
    platform:
    { system, set, ... }:
    let
      evaluated = system { services.boomdrive.ghostty.package = null; };
      profile = profileOf evaluated;
    in
    t.expect (
      t.failures evaluated == [ ]
    ) "a ${platform} system with no Ghostty fails: ${toString (t.failures evaluated)}"
    ++ t.expect (
      !builtins.elem set.ghostty.outPath profile
    ) "the ${platform} system profile with no Ghostty holds the set's Ghostty"
    ++
      t.expect (hostOf evaluated ? terminal_app && (hostOf evaluated).terminal_app == "ghostty")
        "the ${platform} system with no Ghostty gives Flume's host ${builtins.toJSON (hostOf evaluated)}, its terminal_app no ghostty";

  # Flume's `[host]`, as Flume's module gives it (design Decision 7).
  hostOf = evaluated: evaluated.config.services.flume.settings.host or null;
  hostIs =
    platform: given: expected:
    let
      evaluated = platforms.${platform}.system given;
      host = hostOf evaluated;
      held = lib.mapAttrs (key: _: host.${key} or null) expected;
    in
    t.expect (
      t.failures evaluated == [ ]
    ) "the ${platform} system fails: ${toString (t.failures evaluated)}"
    ++
      t.expect (held == expected)
        "the ${platform} system gives Flume's host ${builtins.toJSON held}, not ${builtins.toJSON expected}";

  # NixOS's user activation snippets, by name, as the modules define them:
  # the option's value is the one script they make.
  userScripts =
    evaluated: lib.foldl' (a: b: a // b) { } evaluated.options.system.userActivationScripts.definitions;
  # The activation's text of the runner's user, as text: on nix-darwin the
  # system's, on NixOS the runner's own user snippet (design Decision 6).
  activationOf =
    platform: evaluated:
    builtins.unsafeDiscardStringContext (
      if platform == "nix-darwin" then
        evaluated.config.system.activationScripts.postActivation.text
      else
        (userScripts evaluated).boomdrive.text or ""
    );
  # Its lines that run activate.sh's `plugins`.
  pluginSteps =
    platform: evaluated:
    lib.filter (lib.hasInfix "-activate.sh plugins ") (
      lib.splitString "\n" (activationOf platform evaluated)
    );
  activate = builtins.unsafeDiscardStringContext "${../modules/activate.sh}";
  stepOf =
    set:
    builtins.unsafeDiscardStringContext "${activate} plugins ${set.herdr}/bin/herdr ${set.flume-herdr-plugin} ${set.sluice-herdr-plugin} || true";

  linkedOnDarwin =
    let
      inherit (platforms."nix-darwin") set;
      steps = pluginSteps "nix-darwin" (t.darwinSystem { });
      expected = "sudo --user=alice -- /usr/bin/env HOME=/Users/alice /bin/sh ${stepOf set}";
    in
    t.expect (
      steps == [ expected ]
    ) "the nix-darwin activation's plugin steps are ${builtins.toJSON steps}, not ${expected}";

  linkedOnNixOS =
    let
      inherit (platforms.NixOS) set;
      evaluated = t.nixosSystem { };
      text = activationOf "NixOS" evaluated;
      guard = "if [ \"$(id -un)\" = alice ]; then";
      guarded = lib.splitString guard text;
      expected = "${builtins.unsafeDiscardStringContext evaluated.pkgs.runtimeShell} ${stepOf set}";
      steps = map lib.trim (pluginSteps "NixOS" evaluated);
      scripts = userScripts evaluated;
      others = lib.filter (name: name != "boomdrive") (builtins.attrNames scripts);
    in
    t.expect (
      steps == [ expected ]
    ) "the NixOS user activation's plugin steps are ${builtins.toJSON steps}, not ${expected}"
    ++ t.expect (
      builtins.length guarded == 2 && lib.hasInfix expected (builtins.elemAt guarded 1)
    ) "the NixOS user activation links the plugins outside its test of alice: ${text}"
    ++ lib.concatMap (
      name:
      t.expect (
        !lib.hasInfix "-activate.sh plugins " (
          builtins.unsafeDiscardStringContext (scripts.${name}.text or "")
        )
      ) "the NixOS user activation's ${name} links the plugins"
    ) others;

  noSetsHerdr =
    platform:
    { system, ... }:
    lib.concatMap
      (
        given:
        let
          steps = pluginSteps platform (system given);
        in
        t.expect (
          steps == [ ]
        ) "the ${platform} activation with ${builtins.toJSON given} links plugins: ${builtins.toJSON steps}"
      )
      [
        { services.boomdrive.herdr.package = null; }
        { services.boomdrive.herdr.plugins = [ ]; }
      ];

  flumesTerminal =
    platform: _:
    hostIs platform { } {
      multiplexer = "herdr";
      terminal_app = "ghostty";
    };
  hostsWindow =
    platform: _:
    hostIs platform { services.flume.settings.host.terminal_app = "terminal"; } {
      multiplexer = "herdr";
      terminal_app = "terminal";
    };
  wordOfBefore =
    platform: _:
    hostIs platform { services.flume.settings.host.terminal = "herdr"; } {
      terminal = "herdr";
      multiplexer = null;
    };

  # portless, installed with the runner (design Decision 5 of the change
  # portless-comes-with-the-flake).
  portlessWithRunner =
    platform:
    { system, set, ... }:
    let
      evaluated = system { };
    in
    t.expect (
      t.failures evaluated == [ ]
    ) "a ${platform} system with the runner fails: ${toString (t.failures evaluated)}"
    ++ t.expect (builtins.elem set.portless.outPath (profileOf evaluated)) "the ${platform} system profile holds no set's portless";

  # A portless of the host's own, laid out as a package with a bin.
  ownPortless =
    { pkgs, ... }:
    {
      services.boomdrive.portless.package = pkgs.runCommand "own-portless" { } "mkdir -p $out/bin";
    };
  hostsPortless =
    platform:
    { system, set, ... }:
    let
      evaluated = system ownPortless;
      profile = profileOf evaluated;
    in
    t.expect (builtins.elem evaluated.config.services.boomdrive.portless.package.outPath profile) "the ${platform} system profile holds no host's own portless"
    ++ t.expect (
      !builtins.elem set.portless.outPath profile
    ) "the ${platform} system profile holds the set's portless beside the host's own";

  noPortless =
    platform:
    { system, set, ... }:
    let
      evaluated = system { services.boomdrive.portless.package = null; };
    in
    t.expect (
      t.failures evaluated == [ ]
    ) "a ${platform} system with no portless fails: ${toString (t.failures evaluated)}"
    ++ t.expect (
      !builtins.elem set.portless.outPath (profileOf evaluated)
    ) "the ${platform} system profile with no portless holds the set's portless";

  portlessWithoutRunner =
    platform:
    { without, set, ... }:
    t.expect (
      !builtins.elem set.portless.outPath (profileOf without)
    ) "the ${platform} system profile without the runner holds the set's portless";

  # No launchd job, systemd unit or activation step of the modules' runs
  # portless (design Decision 6 of the change
  # portless-comes-with-the-flake): each one's text, as text, holds no
  # path of the set's portless, and a failure names the one that does.
  textOf =
    value:
    builtins.unsafeDiscardStringContext (
      if builtins.isString value then value else toString (value.text or "")
    );
  launchdOf =
    kind: jobs:
    lib.mapAttrs' (name: job: {
      name = "launchd ${kind} ${name}";
      value = builtins.unsafeDiscardStringContext (
        toString (job.serviceConfig.Program or "")
        + " "
        + toString (job.serviceConfig.ProgramArguments or [ ])
      );
    }) jobs;
  systemdOf =
    kind: units:
    lib.mapAttrs' (name: unit: {
      name = "systemd ${kind} ${name}";
      value = builtins.unsafeDiscardStringContext (
        toString (unit.script or "") + " " + toString (unit.serviceConfig.ExecStart or "")
      );
    }) units;
  stepsOf =
    platform: evaluated:
    let
      inherit (evaluated) config;
    in
    if platform == "nix-darwin" then
      {
        "activation script" = textOf config.system.activationScripts.script;
      }
      // launchdOf "daemon" config.launchd.daemons
      // launchdOf "agent" config.launchd.agents
      // launchdOf "user agent" config.launchd.user.agents
    else
      lib.mapAttrs' (name: step: {
        name = "activation step ${name}";
        value = textOf step;
      }) config.system.activationScripts
      // lib.mapAttrs' (name: step: {
        name = "user activation step ${name}";
        value = textOf step;
      }) (userScripts evaluated)
      // systemdOf "service" config.systemd.services
      // systemdOf "user service" config.systemd.user.services;
  noProxy =
    platform:
    { system, set, ... }:
    let
      portless = builtins.unsafeDiscardStringContext set.portless.outPath;
    in
    lib.concatLists (
      lib.mapAttrsToList (
        name: text:
        t.expect (!lib.hasInfix portless text) "the ${platform} system's ${name} runs the set's portless"
      ) (stepsOf platform (system { }))
    );

  # worktrunk, installed with the runner (design Decision 3 of the change
  # worktrunk-comes-with-the-flake).
  worktrunkWithRunner =
    platform:
    { system, set, ... }:
    let
      evaluated = system { };
    in
    t.expect (
      t.failures evaluated == [ ]
    ) "a ${platform} system with the runner fails: ${toString (t.failures evaluated)}"
    ++ t.expect (builtins.elem set.worktrunk.outPath (profileOf evaluated)) "the ${platform} system profile holds no set's worktrunk";

  # A worktrunk of the host's own, laid out as a package with a bin.
  ownWorktrunk =
    { pkgs, ... }:
    {
      services.boomdrive.worktrunk.package = pkgs.runCommand "own-worktrunk" { } "mkdir -p $out/bin";
    };
  hostsWorktrunk =
    platform:
    { system, set, ... }:
    let
      evaluated = system ownWorktrunk;
      profile = profileOf evaluated;
    in
    t.expect (builtins.elem evaluated.config.services.boomdrive.worktrunk.package.outPath profile) "the ${platform} system profile holds no host's own worktrunk"
    ++ t.expect (
      !builtins.elem set.worktrunk.outPath profile
    ) "the ${platform} system profile holds the set's worktrunk beside the host's own";

  noWorktrunk =
    platform:
    { system, set, ... }:
    let
      evaluated = system { services.boomdrive.worktrunk.package = null; };
    in
    t.expect (
      t.failures evaluated == [ ]
    ) "a ${platform} system with no worktrunk fails: ${toString (t.failures evaluated)}"
    ++ t.expect (
      !builtins.elem set.worktrunk.outPath (profileOf evaluated)
    ) "the ${platform} system profile with no worktrunk holds the set's worktrunk";

  worktrunkWithoutRunner =
    platform:
    { without, set, ... }:
    t.expect (
      !builtins.elem set.worktrunk.outPath (profileOf without)
    ) "the ${platform} system profile without the runner holds the set's worktrunk";

  # crew's companions, installed with the runner where crew's module is on
  # (design Decisions 6 and 9 of the change
  # crews-companions-come-with-the-flake).
  crewOn = {
    services.crew = {
      enable = true;
      host = "host-a";
      settings = (import ./settings.nix).crewModule;
    };
  };
  companions = [
    "pendingyou"
    "plannotator"
    "plannotator-tui"
  ];
  signinsOf =
    evaluated:
    lib.filter (p: (p.name or "") == "pendingyou-signin") evaluated.config.environment.systemPackages;
  # The activation's lines that write Pending You's folder.
  folderSteps =
    platform: evaluated:
    lib.filter (lib.hasInfix "-activate.sh pendingyou ") (
      lib.splitString "\n" (activationOf platform evaluated)
    );

  companionsWithCrew =
    platform:
    { system, set, ... }:
    let
      evaluated = system crewOn;
      profile = profileOf evaluated;
    in
    t.expect (
      t.failures evaluated == [ ]
    ) "a ${platform} system with the runner and crew fails: ${toString (t.failures evaluated)}"
    ++ lib.concatMap (
      name:
      t.expect (builtins.elem set.${name}.outPath
        profile
      ) "the ${platform} system profile with the runner and crew holds no set's ${name}"
    ) companions
    ++ t.expect (
      builtins.length (signinsOf evaluated) == 1
    ) "the ${platform} system profile with the runner and crew holds no pendingyou-signin";

  # A Plannotator of the host's own, laid out as a package with a bin.
  ownPlannotator =
    { pkgs, ... }:
    {
      imports = [ crewOn ];
      services.boomdrive.plannotator.package = pkgs.runCommand "own-plannotator" { } "mkdir -p $out/bin";
    };
  hostsPlannotator =
    platform:
    { system, set, ... }:
    let
      evaluated = system ownPlannotator;
      profile = profileOf evaluated;
    in
    t.expect (builtins.elem evaluated.config.services.boomdrive.plannotator.package.outPath profile) "the ${platform} system profile holds no host's own Plannotator"
    ++ t.expect (
      !builtins.elem set.plannotator.outPath profile
    ) "the ${platform} system profile holds the set's Plannotator beside the host's own";

  # The set's companions and plugins, none in `evaluated`.
  noneOf =
    platform: what: set: evaluated:
    let
      profile = profileOf evaluated;
    in
    lib.concatMap (
      name:
      t.expect (
        !builtins.elem set.${name}.outPath profile
      ) "the ${platform} system profile ${what} holds the set's ${name}"
    ) companions
    ++ t.expect (
      signinsOf evaluated == [ ]
    ) "the ${platform} system profile ${what} holds pendingyou-signin";

  noCompanionsWithoutCrew =
    platform:
    { system, set, ... }:
    let
      evaluated = system { };
      plugins = map (p: p.outPath) evaluated.config.services.boomdrive.herdr.plugins;
      expected = map (name: set.${name}.outPath) [
        "flume-herdr-plugin"
        "sluice-herdr-plugin"
      ];
      steps = folderSteps platform evaluated;
    in
    noneOf platform "without crew" set evaluated
    ++ t.expect (
      steps == [ ]
    ) "the ${platform} activation without crew writes Pending You's folder: ${builtins.toJSON steps}"
    ++ t.expect (
      plugins == expected
    ) "the ${platform} plugins without crew are ${toString plugins}, not Flume's and sluice's alone";

  noCompanionsWithoutRunner =
    platform:
    { withCrew, set, ... }:
    noneOf platform "with crew and without the runner" set withCrew;

  companionsPlugins =
    let
      linked =
        platform:
        { system, set, ... }:
        let
          evaluated = system crewOn;
          steps = map lib.trim (pluginSteps platform evaluated);
          plugins = map (name: set.${name}) [
            "flume-herdr-plugin"
            "sluice-herdr-plugin"
            "pendingyou-herdr-plugin"
            "plannotator-herdr-plugin"
          ];
          step = builtins.unsafeDiscardStringContext "${activate} plugins ${set.herdr}/bin/herdr ${toString plugins} || true";
          expected =
            if platform == "nix-darwin" then
              "sudo --user=alice -- /usr/bin/env HOME=/Users/alice /bin/sh ${step}"
            else
              "${builtins.unsafeDiscardStringContext evaluated.pkgs.runtimeShell} ${step}";
        in
        t.expect (
          steps == [ expected ]
        ) "the ${platform} activation with crew links ${builtins.toJSON steps}, not ${expected}";
    in
    lib.concatLists (lib.mapAttrsToList linked platforms);

  # The sign-in's script, read (design Decision 9): given `--force`, it
  # runs Pending You's `login --app claude-code` with it and Claude Code's
  # folder the folder's `signin`, then `app login herdr` without it.
  signinRead =
    platform:
    { system, ... }:
    let
      evaluated = system crewOn;
      signins = signinsOf evaluated;
      pendingyou = lib.getExe' evaluated.config.services.boomdrive.pendingyou.package "pendingyou";
      text = builtins.unsafeDiscardStringContext (builtins.head signins).text;
      login = builtins.unsafeDiscardStringContext ''
        CLAUDE_CONFIG_DIR="$PENDINGYOU_CONFIG_DIR/signin" \
          ${pendingyou} login --app claude-code "$@"
      '';
      herdr = builtins.unsafeDiscardStringContext ''${pendingyou} app login herdr "''${herdr[@]}"'';
      parts = lib.splitString login text;
    in
    if signins == [ ] then
      [ "the ${platform} system profile with the runner and crew holds no pendingyou-signin" ]
    else
      t.expect (lib.hasInfix "PENDINGYOU_CONFIG_DIR=\"$HOME/.config/pendingyou\"\nexport PENDINGYOU_CONFIG_DIR\n" text) "the ${platform} pendingyou-signin exports no PENDINGYOU_CONFIG_DIR of $HOME/.config/pendingyou: ${text}"
      ++ t.expect (
        builtins.length parts == 2
      ) "the ${platform} pendingyou-signin runs no ${login}: ${text}"
      ++ t.expect (
        builtins.length parts == 2 && lib.hasInfix herdr (builtins.elemAt parts 1)
      ) "the ${platform} pendingyou-signin runs no ${herdr} after its login: ${text}"
      ++ t.expect (lib.hasInfix "  if [ \"$argument\" != --force ]; then\n    herdr+=(\"$argument\")\n" text) "the ${platform} pendingyou-signin gives herdr's login --force: ${text}";

  # Pending You's folder (design Decisions 7 and 8 of the change
  # crews-companions-come-with-the-flake): the store folder the modules
  # give for a home and a package, and the step that writes it.
  folderOf =
    evaluated: home: package:
    evaluated.pkgs.callPackage ../modules/pendingyou-folder.nix { inherit home package; };
  homeOf = platform: if platform == "nix-darwin" then "/Users/alice" else "/home/alice";
  folderStepOf =
    platform: evaluated: folder:
    let
      step = builtins.unsafeDiscardStringContext "${activate} pendingyou ${folder} ${homeOf platform}/.config/pendingyou || true";
    in
    if platform == "nix-darwin" then
      "sudo --user=alice -- /usr/bin/env HOME=/Users/alice /bin/sh ${step}"
    else
      "${builtins.unsafeDiscardStringContext evaluated.pkgs.runtimeShell} ${step}";
  # The index of the first of `lines` that `holds`, or null.
  indexOf =
    holds: lines:
    let
      found = lib.lists.findFirstIndex holds null lines;
    in
    found;
  fourFiles = [
    "bin/pendingyou-hook"
    "bin/pendingyou-mcp-headers"
    "claude-code.json"
    "signin/.claude.json"
  ];

  folderWritten =
    platform:
    let
      inherit (platforms.${platform}) system;
      evaluated = system crewOn;
      folder =
        folderOf evaluated (homeOf platform)
          evaluated.config.services.boomdrive.pendingyou.package;
      expected = folderStepOf platform evaluated folder;
      text = activationOf platform evaluated;
      lines = map lib.trim (lib.splitString "\n" text);
      steps = map lib.trim (folderSteps platform evaluated);
      folderAt = indexOf (line: line == expected) lines;
      pluginsAt = indexOf (lib.hasInfix "-activate.sh plugins ") lines;
    in
    t.expect (
      steps == [ expected ]
    ) "the ${platform} activation's folder steps are ${builtins.toJSON steps}, not ${expected}"
    ++ t.expect (
      folderAt != null && pluginsAt != null && folderAt < pluginsAt
    ) "the ${platform} activation writes Pending You's folder after it links the plugins: ${text}"
    ++ t.expect (
      folder.folder == "${homeOf platform}/.config/pendingyou"
    ) "the ${platform} folder is ${folder.folder}, not ${homeOf platform}/.config/pendingyou"
    ++
      t.expect (lib.sort lib.lessThan (builtins.attrNames folder.texts) == fourFiles)
        "the ${platform} folder holds ${toString (builtins.attrNames folder.texts)}, not ${toString fourFiles}";

  folderOnNixOS =
    let
      evaluated = t.nixosSystem crewOn;
      text = activationOf "NixOS" evaluated;
      guarded = lib.splitString "if [ \"$(id -un)\" = alice ]; then" text;
      scripts = userScripts evaluated;
      others = lib.filter (name: name != "boomdrive") (builtins.attrNames scripts);
    in
    folderWritten "NixOS"
    ++ t.expect (
      builtins.length guarded == 2 && lib.hasInfix "-activate.sh pendingyou " (builtins.elemAt guarded 1)
    ) "the NixOS user activation writes Pending You's folder outside its test of alice: ${text}"
    ++ lib.concatMap (
      name:
      t.expect (
        !lib.hasInfix "-activate.sh pendingyou " (
          builtins.unsafeDiscardStringContext (scripts.${name}.text or "")
        )
      ) "the NixOS user activation's ${name} writes Pending You's folder"
    ) others;

  # A Pending You of the host's own, laid out as a package with a bin.
  ownPendingyou =
    { pkgs, ... }:
    {
      imports = [ crewOn ];
      services.boomdrive.pendingyou.package = pkgs.runCommand "own-pendingyou" { } "mkdir -p $out/bin";
    };
  launchersRunGiven =
    platform:
    { system, ... }:
    let
      evaluated = system ownPendingyou;
      own = evaluated.config.services.boomdrive.pendingyou.package;
      folder = folderOf evaluated (homeOf platform) own;
      exe = builtins.unsafeDiscardStringContext "${own}/bin/pendingyou";
      exported = "PENDINGYOU_CONFIG_DIR=${homeOf platform}/.config/pendingyou\nexport PENDINGYOU_CONFIG_DIR\n";
      textOf = path: builtins.unsafeDiscardStringContext folder.texts.${path};
    in
    t.expect
      (
        folderSteps platform evaluated != [ ]
        && lib.trim (lib.head (folderSteps platform evaluated)) == folderStepOf platform evaluated folder
      )
      "the ${platform} activation with the host's own Pending You writes no folder of it: ${builtins.toJSON (folderSteps platform evaluated)}"
    ++
      lib.concatMap
        (
          path:
          t.expect (lib.hasInfix exported (textOf path)) "the ${platform} folder's ${path} exports no PENDINGYOU_CONFIG_DIR of the folder: ${textOf path}"
        )
        [
          "bin/pendingyou-hook"
          "bin/pendingyou-mcp-headers"
        ]
    ++ t.expect (lib.hasInfix "\nexec ${exe} \"$@\"\n" (textOf "bin/pendingyou-hook")) "the ${platform} folder's bin/pendingyou-hook runs no ${exe}: ${textOf "bin/pendingyou-hook"}"
    ++ t.expect (lib.hasInfix "\nexec ${exe} mcp-headers \"$@\"\n" (textOf "bin/pendingyou-mcp-headers")) "the ${platform} folder's bin/pendingyou-mcp-headers runs no ${exe} mcp-headers: ${textOf "bin/pendingyou-mcp-headers"}";

  stopCheck =
    let
      evaluated = t.darwinSystem crewOn;
      package = evaluated.config.services.boomdrive.pendingyou.package;
      folder = folderOf evaluated "/Users/alice" package;
      text = builtins.unsafeDiscardStringContext folder.texts."bin/pendingyou-hook";
      exe = builtins.unsafeDiscardStringContext (lib.getExe' package "pendingyou");
      expected = lib.concatStringsSep "\n" [
        "if [ \"$1\" = stopcheck ] && [ -n \"\${CREW_AGENT:-}\" ]; then"
        "  ${exe} \"$@\" >/dev/null"
        "  exit 0"
        "fi"
        "exec ${exe} \"$@\""
        ""
      ];
    in
    t.expect (lib.hasSuffix expected text) "the folder's bin/pendingyou-hook does not end in ${expected}: ${text}";

  # The wake mod in the folder (record 0044, point 5): the given Pending
  # You's `mod`, copied whole, the set's by its `mod` and a host's own by
  # where npm installs it.
  wakeModIn =
    platform:
    { system, ... }:
    lib.concatMap
      (
        given:
        let
          evaluated = system given;
          package = evaluated.config.services.boomdrive.pendingyou.package;
          folder = folderOf evaluated (homeOf platform) package;
          mod = builtins.unsafeDiscardStringContext (
            package.mod or "${package}/lib/node_modules/pendingyou/mod"
          );
          copies = builtins.unsafeDiscardStringContext "cp -RL ${mod} \"$out/mod\"";
          command = builtins.unsafeDiscardStringContext folder.buildCommand;
        in
        t.expect (builtins.unsafeDiscardStringContext folder.mod == mod)
          "the ${platform} folder's wake mod is ${builtins.unsafeDiscardStringContext folder.mod}, not ${mod}"
        ++ t.expect (lib.hasInfix copies command) "the ${platform} folder does not copy ${mod} whole: ${command}"
        ++ t.expect (lib.hasInfix "find \"$out/mod\" -type f -exec chmod 0644 {} +" command) "the ${platform} folder's wake mod is not 0644: ${command}"
      )
      [
        crewOn
        ownPendingyou
      ];

  noSetsPendingyou =
    platform:
    { system, set, ... }:
    let
      evaluated = system {
        imports = [ crewOn ];
        services.boomdrive.pendingyou.package = null;
      };
      steps = folderSteps platform evaluated;
    in
    t.expect (
      t.failures evaluated == [ ]
    ) "a ${platform} system with no Pending You fails: ${toString (t.failures evaluated)}"
    ++ t.expect (
      !builtins.elem set.pendingyou.outPath (profileOf evaluated)
    ) "the ${platform} system profile with no Pending You holds the set's Pending You"
    ++ t.expect (
      signinsOf evaluated == [ ]
    ) "the ${platform} system profile with no Pending You holds pendingyou-signin"
    ++ t.expect (
      steps == [ ]
    ) "the ${platform} activation with no Pending You writes its folder: ${builtins.toJSON steps}";

  # No activation step, launchd job or systemd unit of the modules' runs
  # `claude` or names a Claude Code home or the set's marketplace (record
  # 0044, point 3; design Decision 12 of the change
  # crews-companions-come-with-the-flake): each one's text, as text, with
  # the runner and crew on.
  claudeWords = [
    {
      what = "runs claude";
      holds = text: builtins.match "(.*[^A-Za-z0-9_.-])?claude([^A-Za-z0-9_.-].*)?" text != null;
    }
    {
      what = "names .claude";
      holds = text: builtins.match "(.*[^A-Za-z0-9_-])?\\.claude([^A-Za-z0-9_.-].*)?" text != null;
    }
    {
      what = "names CLAUDE_CONFIG_DIR";
      holds = lib.hasInfix "CLAUDE_CONFIG_DIR";
    }
  ];
  noClaudeCode =
    platform:
    { system, set, ... }:
    let
      marketplace = builtins.unsafeDiscardStringContext set.boomdrive-claude-plugins.outPath;
      steps = stepsOf platform (system crewOn);
      oneLine = text: lib.replaceStrings [ "\n" ] [ " " ] text;
    in
    lib.concatLists (
      lib.mapAttrsToList (
        name: text:
        lib.concatMap (
          word: t.expect (!word.holds (oneLine text)) "the ${platform} system's ${name} ${word.what}"
        ) claudeWords
        ++ t.expect (
          !lib.hasInfix marketplace text && !lib.hasInfix "boomdrive-claude-plugins" text
        ) "the ${platform} system's ${name} names the set's marketplace"
      ) steps
    )
    ++ t.expect (steps != { }) "the ${platform} system has no step, job or unit to read";

  # The launcher's cases (design Decisions 1 and 5). The account `own`
  # with the drive `home`, keeping agentplot's flume, and `work` with the
  # drive `garden`, keeping example's alpha, `garden` on `machine`.
  twoDrives = machine: {
    services.boomdrive.accounts = {
      own.drives.home.repositories.agentplot = [ "flume" ];
      work.drives.garden = {
        inherit machine;
        repositories.example = [ "alpha" ];
      };
    };
  };
  launcherOf = evaluated: evaluated.config.services.boomdrive.launcher.package;
  # A system of the modules `given`, whose launcher is made for `expected`.
  madeFor =
    platform: given: expected:
    let
      evaluated = platforms.${platform}.system { imports = given; };
      installed = launcherOf evaluated;
    in
    t.expect (
      t.failures evaluated == [ ]
    ) "the ${platform} system fails: ${toString (t.failures evaluated)}"
    ++
      t.expect (installed.drives or null == expected)
        "the ${platform} system's launcher is made for ${
          builtins.toJSON (installed.drives or null)
        }, not ${builtins.toJSON expected}"
    ++ t.expect (builtins.elem installed.outPath (profileOf evaluated)) "the ${platform} system profile holds no launcher it made";

  eachDrive =
    platform: _:
    madeFor platform
      [ (twoDrives null) ]
      [
        {
          drive = "garden";
          session = "garden";
        }
        {
          drive = "home";
          session = "home";
        }
      ];

  anotherMachine =
    platform: _:
    madeFor platform
      [
        (twoDrives "host-b")
        { services.crew.host = "host-a"; }
      ]
      [
        {
          drive = "home";
          session = "home";
        }
      ];

  sessionNamed =
    platform: _:
    madeFor platform
      [
        { services.boomdrive.accounts.own.drives.home.repositories.agentplot = [ "flume" ]; }
        { services.flume.settings.streams.home.session = "house"; }
      ]
      [
        {
          drive = "home";
          session = "house";
        }
      ];

  noDrive =
    platform:
    { system, set, ... }:
    let
      evaluated = system { };
    in
    t.expect (
      (launcherOf evaluated).outPath == set.boomdrive-launcher.outPath
    ) "the ${platform} system with no drive installs a launcher that is not the set's";

  hostsLauncher =
    platform:
    { system, set, ... }:
    let
      evaluated = system {
        imports = [
          (twoDrives null)
          (
            { pkgs, ... }:
            {
              services.boomdrive.launcher.package = pkgs.runCommand "own-launcher" { } "mkdir $out";
            }
          )
        ];
      };
      installed = launcherOf evaluated;
      profile = profileOf evaluated;
    in
    t.expect (
      t.failures evaluated == [ ]
    ) "the ${platform} system fails: ${toString (t.failures evaluated)}"
    ++ t.expect (
      installed.name == "own-launcher" && !(installed ? drives)
    ) "the ${platform} system's launcher is ${installed.name}, not the host's own as given"
    ++ t.expect (builtins.elem installed.outPath profile) "the ${platform} system profile holds no host's own launcher"
    ++ t.expect (
      !builtins.elem set.boomdrive-launcher.outPath profile
    ) "the ${platform} system profile holds the set's launcher beside the host's own";

  withoutRunner =
    platform:
    { without, set, ... }:
    let
      profile = profileOf without;
    in
    lib.concatMap (
      name:
      t.expect (
        !builtins.elem set.${name}.outPath profile
      ) "the ${platform} system profile without the runner holds the set's ${name}"
    ) apps;
in
onEach "The way in with the runner" withRunner
// onEach "A host's own herdr" hostsHerdr
// onEach "No Ghostty of the set's" noGhostty
// onEach "Not without the runner" withoutRunner
// onEach "Flume's terminal and window" flumesTerminal
// onEach "A host's own window" hostsWindow
// onEach "The word of before" wordOfBefore
// onEach "No herdr of the set's" noSetsHerdr
// onEach "portless with the runner" portlessWithRunner
// onEach "A host's own portless" hostsPortless
// onEach "No portless of the set's" noPortless
// onEach "No portless without the runner" portlessWithoutRunner
// onEach "No proxy of the modules'" noProxy
// onEach "worktrunk with the runner" worktrunkWithRunner
// onEach "A host's own worktrunk" hostsWorktrunk
// onEach "No worktrunk of the set's" noWorktrunk
// onEach "No worktrunk without the runner" worktrunkWithoutRunner
// onEach "The companions with the runner and crew" companionsWithCrew
// onEach "A host's own Plannotator" hostsPlannotator
// onEach "No companions without crew" noCompanionsWithoutCrew
// onEach "No companions without the runner" noCompanionsWithoutRunner
// onEach "The sign-in's two commands" signinRead
// onEach "The launchers run the given Pending You" launchersRunGiven
// onEach "No Pending You of the set's" noSetsPendingyou
// onEach "The wake mod in the folder" wakeModIn
// onEach "No Claude Code of the modules'" noClaudeCode
// onEach "Each drive of this host in the launcher" eachDrive
// onEach "A drive of another machine has no entry" anotherMachine
// onEach "A stream's own session named" sessionNamed
// onEach "No drive, the set's launcher" noDrive
// onEach "A host's own launcher" hostsLauncher
// {
  "The folder written on nix-darwin" = folderWritten "nix-darwin";
  "The folder written on NixOS" = folderOnNixOS;
  "A crew session's stop check" = stopCheck;
  "The companions' plugins with crew" = companionsPlugins;
  "The plugins linked on nix-darwin" = linkedOnDarwin;
  "The plugins linked on NixOS" = linkedOnNixOS;
}
