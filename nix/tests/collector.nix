# The service-modules spec's cases on the host's collector reaching Flume
# and sluice (design Decisions 2 to 7 of the change
# the-hosts-collector-reaches-sluice-and-flume): Flume's and sluice's
# `settings.telemetry`, read from each platform's system with its `null`
# keys dropped (Task notes 1.2 and 1.3). Each case runs on nix-darwin and
# on NixOS, unless it names a platform or the README.
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
in
onEach "The host's collector reaches Flume and sluice" reaches
// onEach "A collector's trailing slash" trailingSlash
// onEach "No tool's telemetry without a collector" noCollector
// onEach "No tool's telemetry without the runner" noRunner
// onEach "Each tool keeps its signals" keepsSignals
// onEach "A host turns a tool's sending off" sendingOff
