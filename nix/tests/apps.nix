# The service-modules spec's cases on the apps the modules install with
# the runner (apps.nix; design Decisions 5 to 7 of the change
# herdr-and-ghostty-come-with-the-flake-and-an-app-opens-the-agents):
# herdr and the plugins the activation links, Ghostty and the launcher,
# and Flume's terminal and window; and worktrunk (design Decision 3 of the
# change worktrunk-comes-with-the-flake). Each case runs on nix-darwin and
# on NixOS, unless it names a platform.
t:
let
  inherit (t) lib outputs;

  # Each platform's module (design Decision 5).
  platforms = {
    "nix-darwin" = {
      system = t.darwinSystem;
      without = t.darwinOf [ outputs.darwinModules.default ];
      set = outputs.packages.aarch64-darwin;
    };
    NixOS = {
      system = t.nixosSystem;
      without = t.nixosOf [ outputs.nixosModules.default ];
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
// onEach "worktrunk with the runner" worktrunkWithRunner
// onEach "A host's own worktrunk" hostsWorktrunk
// onEach "No worktrunk of the set's" noWorktrunk
// onEach "No worktrunk without the runner" worktrunkWithoutRunner
// {
  "The plugins linked on nix-darwin" = linkedOnDarwin;
  "The plugins linked on NixOS" = linkedOnNixOS;
}
