# The service-modules spec's cases on the host's collector reaching Flume
# and sluice (design Decisions 2 to 7 of the change
# the-hosts-collector-reaches-sluice-and-flume): Flume's and sluice's
# `settings.telemetry`, read from each platform's system with its `null`
# keys dropped (Task notes 1.2 and 1.3). Each case runs on nix-darwin and
# on NixOS, unless it names a platform.
t:
let
  inherit (t) lib;

  collector = "http://localhost:4318";

  tools = [
    "flume"
    "sluice"
  ];

  # A tool's `settings.telemetry` in `evaluated`, less its `null` keys:
  # null where the tool has none, and the string "unread" where reading it
  # fails.
  telemetryOf =
    evaluated: tool:
    let
      read = builtins.tryEval (
        builtins.deepSeq evaluated.config.services.${tool}.settings.telemetry
          evaluated.config.services.${tool}.settings.telemetry
      );
    in
    if !read.success then
      "unread"
    else if read.value == null then
      null
    else
      lib.filterAttrs (_: value: value != null) read.value;

  # The messages of the assertions `evaluated` fails, or one saying it does
  # not evaluate.
  failuresOf =
    evaluated:
    let
      read = builtins.tryEval (builtins.deepSeq (t.failures evaluated) (t.failures evaluated));
    in
    if read.success then read.value else [ "the system does not evaluate" ];

  platforms = {
    "nix-darwin" = {
      system = t.darwinSystem;
      # The set's modules with Flume's and sluice's on and the runner off,
      # and the module `given`.
      without =
        given:
        t.darwinOf [
          t.outputs.darwinModules.default
          {
            services = {
              flume.enable = true;
              sluice.enable = true;
            };
          }
          given
        ];
    };
    NixOS = {
      system = t.nixosSystem;
      without =
        given:
        t.nixosOf [
          t.outputs.nixosModules.default
          {
            services = {
              flume = {
                enable = true;
                user = "alice";
              };
              sluice = {
                enable = true;
                user = "alice";
              };
            };
          }
          given
        ];
    };
  };
  onEach =
    scenario: check:
    lib.mapAttrs' (platform: host: {
      name = "${scenario} on ${platform}";
      value = check platform host;
    }) platforms;

  # A system naming the collector `address`, with `given` beside it.
  named =
    system: address: given:
    system {
      imports = [ given ];
      services.boomdrive.claude-code.telemetry.collector = address;
    };

  # `evaluated` fails no assertion.
  failsNone =
    what: evaluated:
    t.expect (failuresOf evaluated == [ ]) "${what} fails: ${toString (failuresOf evaluated)}";

  # Each tool's telemetry in `evaluated`, less its `null` keys, is
  # `expected`'s.
  gives =
    what: evaluated: expected:
    lib.concatMap (
      tool:
      let
        read = telemetryOf evaluated tool;
      in
      t.expect (read == expected.${tool})
        "${what} gives ${tool}'s settings.telemetry ${builtins.toJSON read}, not ${
          builtins.toJSON expected.${tool}
        }"
    ) tools;

  # Each tool sending to `address`, as the module gives it.
  sending = address: {
    flume = {
      send = true;
      collector = address;
    };
    sluice = {
      send = true;
      collector = address;
    };
  };
  none = {
    flume = null;
    sluice = null;
  };

  reaches =
    platform:
    { system, ... }:
    let
      what = "a ${platform} system naming the collector ${collector}";
      evaluated = named system collector { };
    in
    failsNone what evaluated ++ gives what evaluated (sending collector);

  trailingSlash =
    platform:
    { system, ... }:
    let
      what = "a ${platform} system naming the collector http://127.0.0.1:4318/";
      evaluated = named system "http://127.0.0.1:4318/" { };
    in
    failsNone what evaluated ++ gives what evaluated (sending "http://127.0.0.1:4318");

  noCollector =
    platform:
    { system, ... }:
    gives "a ${platform} system with the runner and no collector" (system { }) none;

  noRunner =
    platform:
    { without, ... }:
    gives "a ${platform} system naming the collector ${collector} without the runner" (without {
      services.boomdrive.claude-code.telemetry.collector = collector;
    }) none;

  # Claude Code's traces on and its logs off, beside a key of each tool's
  # own: each tool keeps its own, and is given Claude Code's switches none
  # (design Decision 5).
  keepsSignals =
    platform:
    { system, ... }:
    let
      what = "a ${platform} system naming a collector, with Flume's traces and sluice's logs its own";
      evaluated = named system collector {
        services = {
          boomdrive.claude-code.telemetry = {
            traces = true;
            logs = false;
          };
          flume.settings.telemetry.traces = false;
          sluice.settings.telemetry.logs = "tail";
        };
      };
      sent = sending collector;
    in
    failsNone what evaluated
    ++ gives what evaluated {
      flume = sent.flume // {
        traces = false;
      };
      sluice = sent.sluice // {
        logs = "tail";
      };
    };

  # A host's `send = false` in sluice's own settings wins, and leaves
  # Flume's sending (design Decision 3).
  sendingOff =
    platform:
    { system, ... }:
    let
      what = "a ${platform} system naming a collector, with sluice's send false";
      evaluated = named system collector { services.sluice.settings.telemetry.send = false; };
      sent = sending collector;
    in
    failsNone what evaluated
    ++ gives what evaluated {
      inherit (sent) flume;
      sluice = sent.sluice // {
        send = false;
      };
    };

  # Each of `words` in the failures of `evaluated`, which fails.
  failsNaming =
    what: evaluated: words:
    let
      failures = toString (failuresOf evaluated);
    in
    t.expect (failuresOf evaluated != [ ]) "${what} evaluates"
    ++ lib.concatMap (
      word: t.expect (lib.hasInfix word failures) "the failure of ${what} names no ${word}: ${failures}"
    ) words;

  # Flume's own collector, the host's with a trailing `/` (design Decision
  # 6).
  namedAgain =
    platform:
    { system, ... }:
    failsNone
      "a ${platform} system naming the collector ${collector} again in Flume's own settings, with a trailing slash"
      (named system collector { services.flume.settings.telemetry.collector = "${collector}/"; });

  # Each of `tools`' own collector apart from the host's, given as `give`
  # does (design Decision 6).
  apart =
    give: tools: platform:
    { system, ... }:
    lib.concatMap (
      tool:
      let
        option = "services.${tool}.settings.telemetry.collector";
      in
      failsNaming
        "a ${platform} system naming the collector ${collector} and ${option} http://localhost:4317"
        (named system collector {
          services.${tool}.settings.telemetry.collector = give "http://localhost:4317";
        })
        [
          option
          "services.boomdrive.claude-code.telemetry.collector"
        ]
    ) tools;

  # The runner's system with crew and the collector, beside the tools' own
  # modules alone given the same `telemetry` keys, as *The tools'
  # activation is their own* sets them (design Decision 7). Each tool's
  # activation links the file its module rendered from its settings, so
  # equal lines show each tool's module wrote the same `[telemetry]`.
  settings = import ./settings.nix;
  render = import ../modules/toml.nix { inherit lib; };
  inherit (import ../modules/accounts.nix { inherit lib; }) fileOf;
  crewOn.services.crew = {
    enable = true;
    host = "host-a";
    settings = settings.crewModule;
  };
  toolsTelemetry.services = lib.mapAttrs (_: telemetry: { settings = { inherit telemetry; }; }) (
    sending collector
  );
  linesOf = text: lib.filter (line: line != "") (lib.splitString "\n" text);

  # nix-darwin's activation, as text, less the runner's link, Pending You's
  # folder, the plugins' links and the managed telemetry and transcripts
  # documents' steps.
  darwinActivation =
    evaluated:
    builtins.unsafeDiscardStringContext evaluated.config.system.activationScripts.postActivation.text;
  darwinRunner = named t.darwinSystem collector crewOn;
  darwinOwn = t.toolsDarwinOf toolsTelemetry;
  runnerLink = builtins.unsafeDiscardStringContext ''
    sudo --user=alice -- /usr/bin/env HOME=/Users/alice \
      /bin/sh ${../modules/activate.sh} link ${darwinRunner.pkgs.writeText "boomdrive-config.toml" (render (fileOf darwinRunner.config.services.boomdrive))} || true
  '';
  managedStep =
    name: line:
    lib.hasInfix "-activate.sh managed " line
    && lib.hasInfix "/Library/Application Support/ClaudeCode/managed-settings.d/${name}" line;
  dropped =
    line:
    lib.hasInfix "-activate.sh plugins " line
    || lib.hasInfix "-activate.sh pendingyou " line
    || managedStep "10-boomdrive-telemetry.json" line
    || managedStep "10-boomdrive-transcripts.json" line;
  darwinRest = lib.filter (line: !dropped line) (
    linesOf (lib.replaceStrings [ runnerLink ] [ "" ] (darwinActivation darwinRunner))
  );
  darwinTools = linesOf (darwinActivation darwinOwn);

  # NixOS's user activation snippets, by name, as the modules define them.
  userScripts =
    evaluated: lib.foldl' (a: b: a // b) { } evaluated.options.system.userActivationScripts.definitions;
  nixosRunner = named t.nixosSystem collector crewOn;
  nixosOwn = t.toolsNixosOf toolsTelemetry;
  textOf =
    evaluated: tool: builtins.unsafeDiscardStringContext ((userScripts evaluated).${tool}.text or "");
in
onEach "The host's collector reaches Flume and sluice" reaches
// onEach "A collector's trailing slash" trailingSlash
// onEach "No tool's telemetry without a collector" noCollector
// onEach "No tool's telemetry without the runner" noRunner
// onEach "Each tool keeps its signals" keepsSignals
// onEach "A host turns a tool's sending off" sendingOff
// onEach "The same collector named again" namedAgain
// onEach "A collector apart from the host's" (
  apart (address: address) [
    "flume"
    "sluice"
  ]
)
// onEach "A forced collector apart from the host's" (apart lib.mkForce [ "sluice" ])
// {
  "The tools' telemetry is their own on nix-darwin" =
    failsNone "the nix-darwin system of the runner with crew and a collector" darwinRunner
    ++ failsNone "the nix-darwin system of the tools' own modules with their telemetry" darwinOwn
    ++ t.expect (lib.hasInfix runnerLink (darwinActivation darwinRunner)) "the nix-darwin activation runs no `${runnerLink}`"
    ++
      t.expect (darwinRest == darwinTools)
        "the nix-darwin activation with a collector, less the runner's link, Pending You's folder, the plugins' and the managed telemetry and transcripts documents' steps, is ${builtins.toJSON darwinRest}, not the tools' own given the same telemetry ${builtins.toJSON darwinTools}";

  "The tools' telemetry is their own on NixOS" =
    failsNone "the NixOS system of the runner with crew and a collector" nixosRunner
    ++ failsNone "the NixOS system of the tools' own modules with their telemetry" nixosOwn
    ++ lib.concatMap (
      tool:
      t.expect (textOf nixosRunner tool != "" && textOf nixosRunner tool == textOf nixosOwn tool)
        "the NixOS user activation's ${tool} with a collector is ${textOf nixosRunner tool}, not the tool's own given the same telemetry ${textOf nixosOwn tool}"
    ) tools;
}
