# The service-modules spec's cases on the apps the modules install with
# the runner (apps.nix; design Decisions 5 to 7 of the change
# herdr-and-ghostty-come-with-the-flake-and-an-app-opens-the-agents):
# herdr and the plugins the activation links, Ghostty and the launcher,
# and Flume's terminal and window. Each case runs on nix-darwin and on
# NixOS, unless it names a platform. portless's cases are design Decisions
# 5 and 6 of the change portless-comes-with-the-flake, and crew's
# companions' design Decisions 6 and 9 of the change
# crews-companions-come-with-the-flake.
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
// onEach "The companions with the runner and crew" companionsWithCrew
// onEach "A host's own Plannotator" hostsPlannotator
// onEach "No companions without crew" noCompanionsWithoutCrew
// onEach "No companions without the runner" noCompanionsWithoutRunner
// onEach "The sign-in's two commands" signinRead
// {
  "The companions' plugins with crew" = companionsPlugins;
  "The plugins linked on nix-darwin" = linkedOnDarwin;
  "The plugins linked on NixOS" = linkedOnNixOS;
}
