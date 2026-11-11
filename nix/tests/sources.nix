# The service-modules spec's cases on crew's source of kinds in Flume's own
# module (the change the-set-declares-crews-source-to-flume, design
# Decisions 1 to 6): what the Boomdrive module gives
# `config.services.flume.sources`, and whether Flume's activation links the
# source it renders, `flume-source-crew.toml`. Each case runs on nix-darwin
# and on NixOS, unless it names neither.
t:
let
  inherit (t) lib outputs;
  settings = import ./settings.nix;

  # NixOS's user activation snippets, by name, as the modules define them:
  # the option's value is the one script they make.
  userScripts =
    evaluated: lib.foldl' (a: b: a // b) { } evaluated.options.system.userActivationScripts.definitions;

  platforms = {
    "nix-darwin" = {
      system = t.darwinSystem;
      # The runner's module off, crew given.
      off =
        given:
        t.darwinOf [
          outputs.darwinModules.default
          given
        ];
      # Flume's activation, as tools.nix reads it: the system's, as text.
      activationOf =
        evaluated:
        builtins.unsafeDiscardStringContext evaluated.config.system.activationScripts.postActivation.text;
    };
    NixOS = {
      system = t.nixosSystem;
      off =
        given:
        t.nixosOf [
          outputs.nixosModules.default
          given
        ];
      # Flume's activation: its user activation snippet's text.
      activationOf =
        evaluated: builtins.unsafeDiscardStringContext ((userScripts evaluated).flume.text or "");
    };
  };

  onEach =
    scenario: check:
    lib.mapAttrs' (platform: host: {
      name = "${scenario} on ${platform}";
      value = check platform host;
    }) platforms;

  # crew turned on, with its host and one partition, as tools.nix's crewOn.
  crewOn = {
    services.crew = {
      enable = true;
      host = "host-a";
      settings = settings.crewModule;
    };
  };

  # crew's source as the module gives it (design Decision 1).
  crewSource = {
    kinds_dir = "~/.local/share/flume/kinds/crew";
    roles = "~/.local/state/crew/flume/roles.toml";
  };

  # The sources a system gives Flume.
  sourcesOf = evaluated: evaluated.config.services.flume.sources;
  rendered = "flume-source-crew.toml";

  # The system `evaluated` fails no assertion, and gives crew's source as
  # `expected`.
  holds =
    platform: evaluated: expected:
    t.expect (
      t.failures evaluated == [ ]
    ) "the ${platform} system fails: ${toString (t.failures evaluated)}"
    ++
      t.expect ((sourcesOf evaluated).crew or null == expected)
        "the ${platform} system's Flume source crew is ${
          builtins.toJSON ((sourcesOf evaluated).crew or null)
        }, not ${builtins.toJSON expected}";

  crewOnSource =
    platform:
    { system, activationOf, ... }:
    let
      evaluated = system crewOn;
    in
    holds platform evaluated crewSource
    ++ t.expect (lib.hasInfix rendered (activationOf evaluated)) "the ${platform} Flume activation links no ${rendered}: ${activationOf evaluated}";

  crewOff =
    platform:
    { system, activationOf, ... }:
    let
      evaluated = system { };
    in
    t.expect (!(sourcesOf evaluated ? crew))
      "the ${platform} system with crew off gives Flume the source crew ${
        builtins.toJSON ((sourcesOf evaluated).crew or null)
      }"
    ++ t.expect (
      !lib.hasInfix rendered (activationOf evaluated)
    ) "the ${platform} Flume activation with crew off links ${rendered}: ${activationOf evaluated}";

  runnerOff =
    platform:
    { off, ... }:
    let
      evaluated = off crewOn;
    in
    t.expect (
      !evaluated.config.services.boomdrive.enable
    ) "the ${platform} system with the runner unset enables it"
    ++
      t.expect ((sourcesOf evaluated).crew or null == crewSource)
        "the ${platform} system with the runner off gives Flume the source crew ${
          builtins.toJSON ((sourcesOf evaluated).crew or null)
        }, not ${builtins.toJSON crewSource}";
in
onEach "crew's source with crew on" crewOnSource
// onEach "No source of crew's with crew off" crewOff
// onEach "crew's source without the runner" runnerOff
