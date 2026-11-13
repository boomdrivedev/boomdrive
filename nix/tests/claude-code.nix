# The service-modules spec's cases on Claude Code's managed telemetry
# document (design Decisions 2 to 8 and 12 of the change
# every-claude-code-session-exports-to-the-hosts-collector):
# `services.boomdrive.claude-code.telemetry`, read from each platform's
# system. Each case runs on nix-darwin and on NixOS, unless it names a
# platform.
t:
let
  inherit (t) lib;

  # The option's value in `evaluated`, or null where reading it fails.
  telemetryOf =
    evaluated:
    let
      read = builtins.tryEval (
        builtins.deepSeq evaluated.config.services.boomdrive.claude-code.telemetry true
      );
    in
    if read.success then evaluated.config.services.boomdrive.claude-code.telemetry else null;

  platforms = {
    "nix-darwin" = {
      system = t.darwinSystem;
    };
    NixOS = {
      system = t.nixosSystem;
    };
  };
  onEach =
    scenario: check:
    lib.mapAttrs' (platform: host: {
      name = "${scenario} on ${platform}";
      value = check platform host;
    }) platforms;

  # A system naming the collector `collector`, with `given` beside it.
  named =
    system: collector: given:
    system {
      imports = [ given ];
      services.boomdrive.claude-code.telemetry.collector = collector;
    };

  # The collector's option is declared, and its type refuses `address`:
  # the module system's failure of a definition names the option it
  # defines.
  refuses =
    platform: evaluated: address:
    let
      option = evaluated.options.services.boomdrive.claude-code.telemetry.collector or null;
    in
    t.expect (
      telemetryOf evaluated == null
    ) "a ${platform} system naming the collector ${address} evaluates"
    ++ t.expect (
      option != null && lib.showOption option.loc == "services.boomdrive.claude-code.telemetry.collector"
    ) "the ${platform} system declares no services.boomdrive.claude-code.telemetry.collector"
    ++ t.expect (
      option != null && !option.type.check address
    ) "the type of services.boomdrive.claude-code.telemetry.collector takes ${address}";

  offHost =
    platform:
    { system, ... }:
    lib.concatMap (address: refuses platform (named system address { }) address) [
      "http://collector.example.com:4318"
      "https://localhost:4318"
      "http://user@localhost:4318"
      "http://localhost:4318/?x=1"
    ]
    ++
      lib.concatMap
        (
          address:
          let
            telemetry = telemetryOf (named system address { });
          in
          t.expect (telemetry != null && telemetry.collector == address)
            "a ${platform} system naming the collector ${address} reads ${
              builtins.toJSON (telemetry.collector or null)
            }"
        )
        [
          "http://localhost:4318"
          "http://127.0.0.1:4318/"
          "http://[::1]:4318/otlp"
        ];

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

  noSignal =
    platform:
    { system, ... }:
    failsNaming "a ${platform} system naming a collector with every signal off"
      (named system "http://localhost:4318" {
        services.boomdrive.claude-code.telemetry = {
          metrics = false;
          logs = false;
          traces = false;
        };
      })
      [
        "services.boomdrive.claude-code.telemetry"
        "metrics"
        "logs"
        "traces"
      ];

  # The document's module function, which the modules read (Task notes
  # 1.1).
  claudeCode = import ../modules/claude-code.nix { inherit lib; };
  collector = "http://localhost:4318";

  # NixOS's entries of `/etc` under claude-code, by name.
  etcName = "claude-code/managed-settings.d/10-boomdrive-telemetry.json";
  claudeEtcOf =
    evaluated:
    lib.filterAttrs (
      name: entry: lib.hasPrefix "claude-code" name || lib.hasPrefix "claude-code" entry.target
    ) evaluated.config.environment.etc;
  noEtc =
    what: evaluated:
    let
      entries = builtins.attrNames (claudeEtcOf evaluated);
    in
    t.expect (t.failures evaluated == [ ]) "${what} fails: ${toString (t.failures evaluated)}"
    ++ t.expect (entries == [ ]) "${what} holds ${toString entries} under claude-code in /etc";

  documentOnNixos =
    let
      evaluated = named t.nixosSystem collector { };
      entries = claudeEtcOf evaluated;
      entry = entries.${etcName} or null;
      expected = builtins.toJSON (
        claudeCode.documentOf evaluated.config.services.boomdrive.claude-code.telemetry
      );
    in
    t.expect (
      t.failures evaluated == [ ]
    ) "a NixOS system naming a collector fails: ${toString (t.failures evaluated)}"
    ++
      t.expect (builtins.attrNames entries == [ etcName ])
        "the NixOS system's /etc holds ${toString (builtins.attrNames entries)} under claude-code, not ${etcName} alone"
    ++ t.expect (entry != null && entry.enable) "the NixOS system's /etc does not write ${etcName}"
    ++ t.expect (
      entry != null && entry.target == etcName
    ) "the NixOS system's /etc writes ${etcName} at ${toString (entry.target or null)}"
    ++ t.expect (
      entry != null && entry.mode == "0644"
    ) "the NixOS system's ${etcName} has the mode ${toString (entry.mode or null)}, not 0644"
    ++ t.expect (
      entry != null && entry.text == expected
    ) "the NixOS system's ${etcName} holds ${toString (entry.text or null)}, not ${expected}";
in
onEach "A collector off the host" offHost
// onEach "A collector with no signal" noSignal
// {
  "The document on NixOS" = documentOnNixos;
  "No collector on NixOS" = noEtc "a NixOS system with the runner and no collector" (
    t.nixosSystem { }
  );
  "Not without the runner on NixOS" = noEtc "a NixOS system naming a collector without the runner" (
    t.nixosOf [
      t.outputs.nixosModules.default
      { services.boomdrive.claude-code.telemetry.collector = collector; }
    ]
  );
}
