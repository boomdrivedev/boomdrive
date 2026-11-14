# The service-modules spec's cases on Claude Code's managed telemetry
# document (design Decisions 2 to 8 and 12 of the change
# every-claude-code-session-exports-to-the-hosts-collector):
# `services.boomdrive.claude-code.telemetry`, read from each platform's
# system, and the document each writes, held key by key to the documents
# written out here: NixOS's read from `/etc`'s text, and nix-darwin's the
# store path its root step names, held to `pkgs.writeText` of the
# expected document. Each case runs on nix-darwin and on NixOS, unless it
# names a platform.
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

  # The span's option is declared, a span below one day fails evaluation,
  # and its type refuses the span: the module system's failure of a
  # definition names the option it defines (the change
  # the-set-keeps-claude-codes-transcripts-a-year, design Decision 3).
  belowOneDay =
    platform:
    { system, ... }:
    lib.concatMap
      (
        days:
        let
          evaluated = system { services.boomdrive.claude-code.cleanupPeriodDays = days; };
          option = evaluated.options.services.boomdrive.claude-code.cleanupPeriodDays or null;
          read = builtins.tryEval (
            builtins.deepSeq evaluated.config.services.boomdrive.claude-code.cleanupPeriodDays true
          );
        in
        t.expect (!read.success) "a ${platform} system naming the span ${toString days} evaluates"
        ++ t.expect (
          option != null && lib.showOption option.loc == "services.boomdrive.claude-code.cleanupPeriodDays"
        ) "the ${platform} system declares no services.boomdrive.claude-code.cleanupPeriodDays"
        ++ t.expect (
          option != null && !option.type.check days
        ) "the type of services.boomdrive.claude-code.cleanupPeriodDays takes ${toString days}"
      )
      [
        0
        (-1)
      ];

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

  # NixOS's entries of `/etc` under claude-code, by name, and the
  # telemetry document's alone, which the telemetry cases read (the change
  # the-set-keeps-claude-codes-transcripts-a-year, Task notes 1.1).
  etcName = "claude-code/managed-settings.d/10-boomdrive-telemetry.json";
  transcriptsEtcName = "claude-code/managed-settings.d/10-boomdrive-transcripts.json";
  claudeEntriesOf =
    evaluated:
    lib.filterAttrs (
      name: entry: lib.hasPrefix "claude-code" name || lib.hasPrefix "claude-code" entry.target
    ) evaluated.config.environment.etc;
  claudeEtcOf =
    evaluated:
    lib.filterAttrs (
      name: entry: name == etcName || entry.target == etcName
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
      entry = (claudeEtcOf evaluated).${etcName} or null;
      # Under claude-code, the telemetry and the transcripts documents
      # alone.
      others = lib.filter (name: name != etcName && name != transcriptsEtcName) (
        builtins.attrNames (claudeEntriesOf evaluated)
      );
      expected = builtins.toJSON (
        claudeCode.documentOf evaluated.config.services.boomdrive.claude-code.telemetry
      );
    in
    t.expect (
      t.failures evaluated == [ ]
    ) "a NixOS system naming a collector fails: ${toString (t.failures evaluated)}"
    ++
      t.expect (others == [ ])
        "the NixOS system's /etc holds ${toString others} under claude-code, beside ${etcName} and ${transcriptsEtcName}"
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

  # The transcripts document on NixOS (the change
  # the-set-keeps-claude-codes-transcripts-a-year, design Decisions 2 and
  # 5): `/etc`'s entry, mode 0644, its text the document of the span,
  # wherever Flume is on, and none where Flume is off.
  transcriptsOnNixos =
    what: evaluated:
    let
      entry = evaluated.config.environment.etc.${transcriptsEtcName} or null;
      expected = builtins.toJSON (
        claudeCode.transcriptsOf evaluated.config.services.boomdrive.claude-code.cleanupPeriodDays
      );
    in
    t.expect (t.failures evaluated == [ ]) "${what} fails: ${toString (t.failures evaluated)}"
    ++ t.expect (entry != null && entry.enable) "${what}'s /etc does not write ${transcriptsEtcName}"
    ++ t.expect (
      entry != null && entry.target == transcriptsEtcName
    ) "${what}'s /etc writes ${transcriptsEtcName} at ${toString (entry.target or null)}"
    ++ t.expect (
      entry != null && entry.mode == "0644"
    ) "${what}'s ${transcriptsEtcName} has the mode ${toString (entry.mode or null)}, not 0644"
    ++ t.expect (
      entry != null && entry.text == expected
    ) "${what}'s ${transcriptsEtcName} holds ${toString (entry.text or null)}, not ${expected}";
  noTranscriptsOnNixos =
    what: evaluated:
    let
      entries = builtins.attrNames (
        lib.filterAttrs (
          name: entry: name == transcriptsEtcName || entry.target == transcriptsEtcName
        ) evaluated.config.environment.etc
      );
    in
    t.expect (t.failures evaluated == [ ]) "${what} fails: ${toString (t.failures evaluated)}"
    ++ t.expect (entries == [ ]) "${what}'s /etc holds ${toString entries}";

  # nix-darwin's activation, as text, and its lines.
  darwinLines =
    evaluated:
    lib.filter (line: line != "") (
      lib.splitString "\n" (
        builtins.unsafeDiscardStringContext evaluated.config.system.activationScripts.postActivation.text
      )
    );
  darwinPath = "/Library/Application Support/ClaudeCode/managed-settings.d/10-boomdrive-telemetry.json";
  # The root step's line, writing `document` where it is given and
  # removing the document where it is null.
  managedLine =
    document:
    builtins.unsafeDiscardStringContext (
      "/bin/sh ${../modules/activate.sh} managed ${lib.escapeShellArg darwinPath}"
      + lib.optionalString (document != null) " ${document}"
      + " || true"
    );
  # The root steps, each document's, and the telemetry document's alone,
  # by its path (Task notes 1.1).
  allManagedLines =
    evaluated: lib.filter (lib.hasInfix "-activate.sh managed ") (darwinLines evaluated);
  managedLines = evaluated: lib.filter (lib.hasInfix darwinPath) (allManagedLines evaluated);
  # The line `evaluated`'s activation runs for a document, among its
  # lines `stepsOf` gives, which runs once, as root, before every step of
  # the runner's user.
  runsOnceAmong =
    stepsOf: what: evaluated: line:
    let
      lines = darwinLines evaluated;
      steps = stepsOf evaluated;
      at = lib.lists.findFirstIndex (l: l == line) null lines;
      users = lib.lists.findFirstIndex (lib.hasPrefix "sudo --user=") null lines;
    in
    t.expect (steps == [ line ]) "${what} runs ${builtins.toJSON steps}, not ${line} once"
    ++ t.expect (
      at != null && (users == null || at < users)
    ) "${what} runs ${line} after a step of the runner's user"
    ++ t.expect (
      !lib.any (lib.hasInfix "managed-settings.json") lines
    ) "${what} names managed-settings.json";
  runsOnce = runsOnceAmong managedLines;

  # The transcripts document's root step on nix-darwin, writing `document`
  # where it is given and removing the document where it is null, and the
  # lines naming its path (the change
  # the-set-keeps-claude-codes-transcripts-a-year, design Decisions 2 and
  # 6).
  transcriptsPath = "/Library/Application Support/ClaudeCode/managed-settings.d/10-boomdrive-transcripts.json";
  transcriptsLine =
    document:
    builtins.unsafeDiscardStringContext (
      "/bin/sh ${../modules/activate.sh} managed ${lib.escapeShellArg transcriptsPath}"
      + lib.optionalString (document != null) " ${document}"
      + " || true"
    );
  transcriptsLines = evaluated: lib.filter (lib.hasInfix transcriptsPath) (allManagedLines evaluated);
  transcriptsOnDarwin =
    what: evaluated:
    let
      document = evaluated.pkgs.writeText "10-boomdrive-transcripts.json" (
        builtins.toJSON (
          claudeCode.transcriptsOf evaluated.config.services.boomdrive.claude-code.cleanupPeriodDays
        )
      );
    in
    t.expect (t.failures evaluated == [ ]) "${what} fails: ${toString (t.failures evaluated)}"
    ++ runsOnceAmong transcriptsLines "the activation of ${what}" evaluated (transcriptsLine document);
  noTranscriptsOnDarwin =
    what: evaluated:
    t.expect (t.failures evaluated == [ ]) "${what} fails: ${toString (t.failures evaluated)}"
    ++ runsOnceAmong transcriptsLines "the activation of ${what}" evaluated (transcriptsLine null);

  documentOnDarwin =
    let
      evaluated = named t.darwinSystem collector { };
      document = evaluated.pkgs.writeText "10-boomdrive-telemetry.json" (
        builtins.toJSON (claudeCode.documentOf evaluated.config.services.boomdrive.claude-code.telemetry)
      );
    in
    t.expect (
      t.failures evaluated == [ ]
    ) "a nix-darwin system naming a collector fails: ${toString (t.failures evaluated)}"
    ++ runsOnce "the activation of a nix-darwin system naming a collector" evaluated (
      managedLine document
    );

  removesOnDarwin =
    what: evaluated:
    t.expect (t.failures evaluated == [ ]) "${what} fails: ${toString (t.failures evaluated)}"
    ++ runsOnce "the activation of ${what}" evaluated (managedLine null);

  # The documents the cases expect, written out key by key (design
  # Decisions 3 to 5): the keys every document holds, then each signal's.
  fixed = {
    CLAUDE_CODE_ENABLE_TELEMETRY = "1";
    OTEL_METRICS_INCLUDE_SESSION_ID = "1";
    OTEL_METRICS_INCLUDE_ACCOUNT_UUID = "1";
    OTEL_METRICS_INCLUDE_REPOSITORY = "1";
    OTEL_LOG_USER_PROMPTS = "0";
    OTEL_LOG_ASSISTANT_RESPONSES = "0";
    OTEL_LOG_TOOL_DETAILS = "0";
    OTEL_LOG_TOOL_CONTENT = "0";
    OTEL_LOG_RAW_API_BODIES = "0";
    ENABLE_BETA_TRACING_DETAILED = "0";
  };
  # A collector named and no signal: metrics and logs.
  byDefault = {
    env = fixed // {
      OTEL_METRICS_EXPORTER = "otlp";
      OTEL_LOGS_EXPORTER = "otlp";
      OTEL_TRACES_EXPORTER = "none";
      OTEL_EXPORTER_OTLP_METRICS_ENDPOINT = "http://localhost:4318/v1/metrics";
      OTEL_EXPORTER_OTLP_METRICS_PROTOCOL = "http/protobuf";
      OTEL_EXPORTER_OTLP_LOGS_ENDPOINT = "http://localhost:4318/v1/logs";
      OTEL_EXPORTER_OTLP_LOGS_PROTOCOL = "http/protobuf";
    };
  };
  # Traces on and logs off, the collector `http://127.0.0.1:4318/`.
  tracesNamed = {
    env = fixed // {
      OTEL_METRICS_EXPORTER = "otlp";
      OTEL_LOGS_EXPORTER = "none";
      OTEL_TRACES_EXPORTER = "otlp";
      OTEL_EXPORTER_OTLP_METRICS_ENDPOINT = "http://127.0.0.1:4318/v1/metrics";
      OTEL_EXPORTER_OTLP_METRICS_PROTOCOL = "http/protobuf";
      OTEL_EXPORTER_OTLP_TRACES_ENDPOINT = "http://127.0.0.1:4318/v1/traces";
      OTEL_EXPORTER_OTLP_TRACES_PROTOCOL = "http/protobuf";
      CLAUDE_CODE_ENHANCED_TELEMETRY_BETA = "1";
    };
  };
  # Every signal on.
  everySignal = {
    env = fixed // {
      OTEL_METRICS_EXPORTER = "otlp";
      OTEL_LOGS_EXPORTER = "otlp";
      OTEL_TRACES_EXPORTER = "otlp";
      OTEL_EXPORTER_OTLP_METRICS_ENDPOINT = "http://localhost:4318/v1/metrics";
      OTEL_EXPORTER_OTLP_METRICS_PROTOCOL = "http/protobuf";
      OTEL_EXPORTER_OTLP_LOGS_ENDPOINT = "http://localhost:4318/v1/logs";
      OTEL_EXPORTER_OTLP_LOGS_PROTOCOL = "http/protobuf";
      OTEL_EXPORTER_OTLP_TRACES_ENDPOINT = "http://localhost:4318/v1/traces";
      OTEL_EXPORTER_OTLP_TRACES_PROTOCOL = "http/protobuf";
      CLAUDE_CODE_ENHANCED_TELEMETRY_BETA = "1";
    };
  };

  # The systems each case reads: the collector and signals `telemetry`.
  givesTraces = {
    collector = "http://127.0.0.1:4318/";
    traces = true;
    logs = false;
  };
  givesEvery = {
    inherit collector;
    traces = true;
  };
  givesDefault = { inherit collector; };
  crewOn = {
    services.crew = {
      enable = true;
      host = "host-a";
      settings = (import ./settings.nix).crewModule;
    };
  };

  # The document a platform's system of `telemetry` and `given` writes, or
  # null with what fails: on NixOS `/etc`'s text, read; on nix-darwin the
  # store path its root step names, which holds `expected` only where it is
  # `pkgs.writeText` of it (design Decision 12).
  writtenOf =
    platform: telemetry: given: expected:
    if platform == "NixOS" then
      let
        evaluated = t.nixosSystem {
          imports = [ given ];
          services.boomdrive.claude-code.telemetry = telemetry;
        };
        entry = evaluated.config.environment.etc.${etcName} or null;
        document = if entry == null then null else builtins.fromJSON entry.text;
      in
      {
        inherit document;
        failing =
          t.expect (
            t.failures evaluated == [ ]
          ) "the NixOS system of ${builtins.toJSON telemetry} fails: ${toString (t.failures evaluated)}"
          ++ t.expect (entry != null) "the NixOS system of ${builtins.toJSON telemetry} writes no ${etcName}"
          ++
            t.expect (document == expected)
              "the NixOS system of ${builtins.toJSON telemetry} writes ${builtins.toJSON document}, not ${builtins.toJSON expected}";
      }
    else
      let
        evaluated = t.darwinSystem {
          imports = [ given ];
          services.boomdrive.claude-code.telemetry = telemetry;
        };
        line = managedLine (
          evaluated.pkgs.writeText "10-boomdrive-telemetry.json" (builtins.toJSON expected)
        );
        holds = managedLines evaluated == [ line ];
      in
      {
        document = if holds then expected else null;
        failing =
          t.expect (
            t.failures evaluated == [ ]
          ) "the nix-darwin system of ${builtins.toJSON telemetry} fails: ${toString (t.failures evaluated)}"
          ++ t.expect holds "the nix-darwin system of ${builtins.toJSON telemetry} runs ${builtins.toJSON (managedLines evaluated)}, not the step that writes ${builtins.toJSON expected}";
      };

  # A case on each platform: the document written, then `check` of it,
  # each of whose items is `{ holds; what; }`.
  ofDocument =
    telemetry: given: expected: check: platform: _:
    let
      written = writtenOf platform telemetry given expected;
    in
    written.failing
    ++ lib.optionals (written.document != null) (
      lib.concatMap (item: t.expect item.holds "the ${platform} document ${item.what}") (
        check written.document
      )
    );
  # `key` of the document's env is `value`, or is not there where `value`
  # is null.
  envIs = document: key: value: {
    holds = (document.env.${key} or null) == value;
    what = "has ${key} ${builtins.toJSON (document.env.${key} or null)}, not ${builtins.toJSON value}";
  };

  # Every key Claude Code's telemetry takes, which the document may hold.
  telemetryKeys = [
    "CLAUDE_CODE_ENABLE_TELEMETRY"
    "CLAUDE_CODE_ENHANCED_TELEMETRY_BETA"
    "ENABLE_BETA_TRACING_DETAILED"
    "OTEL_METRICS_EXPORTER"
    "OTEL_LOGS_EXPORTER"
    "OTEL_TRACES_EXPORTER"
    "OTEL_EXPORTER_OTLP_METRICS_ENDPOINT"
    "OTEL_EXPORTER_OTLP_LOGS_ENDPOINT"
    "OTEL_EXPORTER_OTLP_TRACES_ENDPOINT"
    "OTEL_EXPORTER_OTLP_METRICS_PROTOCOL"
    "OTEL_EXPORTER_OTLP_LOGS_PROTOCOL"
    "OTEL_EXPORTER_OTLP_TRACES_PROTOCOL"
    "OTEL_METRICS_INCLUDE_SESSION_ID"
    "OTEL_METRICS_INCLUDE_ACCOUNT_UUID"
    "OTEL_METRICS_INCLUDE_REPOSITORY"
    "OTEL_LOG_USER_PROMPTS"
    "OTEL_LOG_ASSISTANT_RESPONSES"
    "OTEL_LOG_TOOL_DETAILS"
    "OTEL_LOG_TOOL_CONTENT"
    "OTEL_LOG_RAW_API_BODIES"
  ];
  contentKeys = [
    "OTEL_LOG_USER_PROMPTS"
    "OTEL_LOG_ASSISTANT_RESPONSES"
    "OTEL_LOG_TOOL_DETAILS"
    "OTEL_LOG_TOOL_CONTENT"
    "OTEL_LOG_RAW_API_BODIES"
    "ENABLE_BETA_TRACING_DETAILED"
  ];
  attributeKeys = [
    "OTEL_METRICS_INCLUDE_SESSION_ID"
    "OTEL_METRICS_INCLUDE_ACCOUNT_UUID"
    "OTEL_METRICS_INCLUDE_REPOSITORY"
  ];

  keysAlone =
    document:
    let
      env = document.env or { };
      strange = lib.filter (key: !builtins.elem key telemetryKeys) (builtins.attrNames env);
      unstrung = lib.filter (key: !builtins.isString env.${key}) (builtins.attrNames env);
    in
    [
      {
        holds = builtins.attrNames document == [ "env" ];
        what = "holds ${toString (builtins.attrNames document)}, not env alone";
      }
      {
        holds = strange == [ ];
        what = "holds ${toString strange}, none of them Claude Code's telemetry keys";
      }
      {
        holds = unstrung == [ ];
        what = "holds ${toString unstrung} as no string";
      }
    ];

  metricsAndLogs = document: [
    (envIs document "OTEL_METRICS_EXPORTER" "otlp")
    (envIs document "OTEL_LOGS_EXPORTER" "otlp")
    (envIs document "OTEL_TRACES_EXPORTER" "none")
    (envIs document "CLAUDE_CODE_ENHANCED_TELEMETRY_BETA" null)
  ];

  tracesOn = document: [
    (envIs document "OTEL_TRACES_EXPORTER" "otlp")
    (envIs document "CLAUDE_CODE_ENHANCED_TELEMETRY_BETA" "1")
    (envIs document "OTEL_LOGS_EXPORTER" "none")
  ];

  endpoints = document: [
    (envIs document "OTEL_EXPORTER_OTLP_METRICS_ENDPOINT" "http://127.0.0.1:4318/v1/metrics")
    (envIs document "OTEL_EXPORTER_OTLP_TRACES_ENDPOINT" "http://127.0.0.1:4318/v1/traces")
    (envIs document "OTEL_EXPORTER_OTLP_METRICS_PROTOCOL" "http/protobuf")
    (envIs document "OTEL_EXPORTER_OTLP_TRACES_PROTOCOL" "http/protobuf")
    (envIs document "OTEL_EXPORTER_OTLP_LOGS_ENDPOINT" null)
    (envIs document "OTEL_EXPORTER_OTLP_LOGS_PROTOCOL" null)
  ];

  noGeneral =
    document:
    let
      keys = builtins.attrNames (document.env or { }) ++ builtins.attrNames document;
      general = lib.filter (
        key:
        builtins.elem key [
          "OTEL_EXPORTER_OTLP_ENDPOINT"
          "OTEL_EXPORTER_OTLP_PROTOCOL"
          "otelHeadersHelper"
          "BETA_TRACING_ENDPOINT"
          "OTEL_RESOURCE_ATTRIBUTES"
        ]
        || lib.hasSuffix "HEADERS" key
      ) keys;
    in
    [
      {
        holds = general == [ ];
        what = "holds ${toString general}";
      }
    ];

  contentOff = document: map (key: envIs document key "0") contentKeys;
  attributesOn = document: map (key: envIs document key "1") attributeKeys;

  # Each document's path, on each platform, with crew on (design Decision
  # 1): the platform's managed-settings folder, in no home, the telemetry
  # document's among them (the change
  # the-set-keeps-claude-codes-transcripts-a-year, Task notes 1.1).
  inNoHome =
    platform: _:
    let
      path =
        if platform == "NixOS" then
          let
            evaluated = named t.nixosSystem collector crewOn;
          in
          map (entry: "/etc/${entry.target}") (builtins.attrValues (claudeEntriesOf evaluated))
        else
          map (line: builtins.head (builtins.match ".*-activate.sh managed '([^']*)'.*" line ++ [ line ])) (
            allManagedLines (named t.darwinSystem collector crewOn)
          );
      expected = if platform == "NixOS" then "/etc/${etcName}" else darwinPath;
      folder =
        if platform == "NixOS" then
          "/etc/claude-code/managed-settings.d"
        else
          "/Library/Application Support/ClaudeCode/managed-settings.d";
      outside = lib.filter (p: dirOf p != folder) path;
      homely = lib.filter (
        p:
        lib.any (word: lib.hasInfix word p) [
          "/home/"
          "/Users/"
          "/root"
          ".claude"
          "CLAUDE_CONFIG_DIR"
        ]
      ) path;
    in
    t.expect (builtins.elem expected path) "the ${platform} documents' paths are ${builtins.toJSON path}, none of them ${expected}"
    ++ t.expect (
      outside == [ ]
    ) "the ${platform} documents' paths ${builtins.toJSON outside} are not in ${folder}"
    ++ t.expect (
      homely == [ ]
    ) "the ${platform} documents' paths name a home: ${builtins.toJSON homely}";

  # The transcripts document a platform's system of the runner and `given`
  # writes, held byte for byte to `expected`, its text, or null with what
  # fails: on NixOS `/etc`'s text, read; on nix-darwin the store path its
  # root step names, which holds `expected` only where it is
  # `pkgs.writeText` of it (the change
  # the-set-keeps-claude-codes-transcripts-a-year, design Decisions 4 and
  # 8).
  transcriptsWrittenOf =
    platform: given: expected:
    let
      of = "the ${platform} system of ${builtins.toJSON given}";
    in
    if platform == "NixOS" then
      let
        evaluated = t.nixosSystem given;
        text = evaluated.config.environment.etc.${transcriptsEtcName}.text or null;
      in
      {
        document = if text == null then null else builtins.fromJSON text;
        failing =
          t.expect (t.failures evaluated == [ ]) "${of} fails: ${toString (t.failures evaluated)}"
          ++ t.expect (text != null) "${of} writes no ${transcriptsEtcName}"
          ++ t.expect (text == expected) "${of} writes ${builtins.toJSON text}, not ${expected}";
      }
    else
      let
        evaluated = t.darwinSystem given;
        line = transcriptsLine (evaluated.pkgs.writeText "10-boomdrive-transcripts.json" expected);
        holds = transcriptsLines evaluated == [ line ];
      in
      {
        document = if holds then builtins.fromJSON expected else null;
        failing =
          t.expect (t.failures evaluated == [ ]) "${of} fails: ${toString (t.failures evaluated)}"
          ++ t.expect holds "${of} runs ${builtins.toJSON (transcriptsLines evaluated)}, not the step that writes ${expected}";
      };

  # A case on each platform: the transcripts document written, then each
  # of `check`'s items, `{ holds; what; }`, of it.
  ofTranscripts =
    given: expected: check: platform: _:
    let
      written = transcriptsWrittenOf platform given expected;
    in
    written.failing
    ++ lib.optionals (written.document != null) (
      lib.concatMap (item: t.expect item.holds "the ${platform} transcripts document ${item.what}") (
        check written.document
      )
    );
  spanAlone = document: [
    {
      holds = builtins.attrNames document == [ "cleanupPeriodDays" ];
      what = "holds ${toString (builtins.attrNames document)}, not cleanupPeriodDays alone";
    }
    {
      holds = builtins.isInt (document.cleanupPeriodDays or null);
      what = "holds cleanupPeriodDays ${builtins.toJSON (document.cleanupPeriodDays or null)}, no number";
    }
  ];
  # The span alone, and the telemetry document of every signal beside it
  # holding no cleanupPeriodDays.
  spanAloneOn =
    platform: host:
    let
      telemetry = writtenOf platform givesEvery { } everySignal;
      document = telemetry.document or { };
    in
    ofTranscripts {
      services.boomdrive.claude-code.telemetry = givesEvery;
    } ''{"cleanupPeriodDays":365}'' spanAlone platform host
    ++ telemetry.failing
    ++ t.expect (
      !(document ? cleanupPeriodDays) && !((document.env or { }) ? cleanupPeriodDays)
    ) "the ${platform} telemetry document holds cleanupPeriodDays";

  # The README's install with nix on the sessions' telemetry (design
  # Decision 11).
  readmeTelemetry =
    let
      block = lib.findFirst (lib.hasInfix "services.boomdrive.claude-code.telemetry") null t.nixBlocks;
      words = toString t.installing;
    in
    t.expect (
      block != null
    ) "no nix block of the README's Installing with nix names services.boomdrive.claude-code.telemetry"
    ++
      t.expect
        (
          block != null
          && lib.hasInfix ''services.boomdrive.claude-code.telemetry.collector = "http://localhost:4318";'' block
        )
        "the README's nix block of services.boomdrive.claude-code.telemetry names no collector http://localhost:4318"
    ++
      lib.concatMap
        (word: t.expect (lib.hasInfix word words) "the README's Installing with nix says no ${word}")
        [
          "`metrics`"
          "`logs`"
          "`traces`, Claude Code's beta, is off\nunless named"
          "10-boomdrive-telemetry.json"
          "`/Library/Application Support/ClaudeCode/managed-settings.d`"
          "`/etc/claude-code/managed-settings.d`"
          "Every Claude Code session on the host"
          "whatever\nits account and its configuration home"
          "a session exports none of its words"
          "Without a collector, or without the runner, nothing is written"
          "named after the set's in Claude Code's order"
          "launch environment"
          "`workspace.host_paths`"
        ];

  # The README's install with nix on the transcripts' span (the change
  # the-set-keeps-claude-codes-transcripts-a-year, design Decision 9), its
  # words read with each line's end a space.
  readmeTranscripts =
    let
      block =
        lib.findFirst (lib.hasInfix "services.boomdrive.claude-code.cleanupPeriodDays") null
          t.nixBlocks;
      words = lib.replaceStrings [ "\n" ] [ " " ] (toString t.installing);
    in
    t.expect (block != null)
      "no nix block of the README's Installing with nix names services.boomdrive.claude-code.cleanupPeriodDays"
    ++ t.expect (
      block != null && lib.hasInfix "services.boomdrive.claude-code.cleanupPeriodDays = 365;" block
    ) "the README's nix block of services.boomdrive.claude-code.cleanupPeriodDays names no 365"
    ++
      lib.concatMap
        (word: t.expect (lib.hasInfix word words) "the README's Installing with nix says no ${word}")
        [
          "`services.boomdrive.claude-code.cleanupPeriodDays`"
          "365 by default"
          "`10-boomdrive-transcripts.json`"
          "into `/Library/Application Support/ClaudeCode/managed-settings.d` on nix-darwin"
          "and `/etc/claude-code/managed-settings.d` on NixOS, whether or not a collector is named"
          "Wherever Flume is on, with the runner or without"
          "every Claude Code session on the host keeps its transcript a year, whatever its account and its configuration home"
          "a stage Flume hibernated can be resumed"
          "named after the set's in Claude Code's order, such as `20-transcripts.json`, sets the key over it"
          "a shorter span, or turning Flume off, deletes every older transcript at the next session's start"
        ];
in
onEach "A collector off the host" offHost
// onEach "A collector with no signal" noSignal
// onEach "A span below one day" belowOneDay
// onEach "A year by default" (ofTranscripts { } ''{"cleanupPeriodDays":365}'' (_: [ ]))
// onEach "A span named" (
  ofTranscripts {
    services.boomdrive.claude-code.cleanupPeriodDays = 90;
  } ''{"cleanupPeriodDays":90}'' (_: [ ])
)
// onEach "The span alone" spanAloneOn
// onEach "In no home" inNoHome
// onEach "The telemetry keys alone" (ofDocument givesEvery { } everySignal keysAlone)
// onEach "Metrics and logs by default" (ofDocument givesDefault { } byDefault metricsAndLogs)
// onEach "Traces named" (ofDocument givesTraces { } tracesNamed tracesOn)
// onEach "Each signal's endpoint" (ofDocument givesTraces { } tracesNamed endpoints)
// onEach "No general endpoint" (ofDocument givesEvery { } everySignal noGeneral)
// onEach "The content settings off" (ofDocument givesEvery { } everySignal contentOff)
// onEach "The account and repository attributes on" (
  ofDocument givesDefault { } byDefault attributesOn
)
// {
  "The README's telemetry" = readmeTelemetry;
  "The README's transcripts" = readmeTranscripts;
  "The document on NixOS" = documentOnNixos;
  "The document on nix-darwin" = documentOnDarwin;
  "The transcripts document on nix-darwin" =
    transcriptsOnDarwin "a nix-darwin system with the runner, Flume on by default and no collector"
      (t.darwinSystem { });
  "No Flume on nix-darwin" =
    noTranscriptsOnDarwin "a nix-darwin system with the runner and Flume off"
      (t.darwinSystem { services.flume.enable = false; });
  "Flume without the runner on nix-darwin" =
    transcriptsOnDarwin "a nix-darwin system with Flume on and the runner off"
      (
        t.darwinOf [
          t.outputs.darwinModules.default
          { services.flume.enable = true; }
        ]
      );
  "No collector on nix-darwin" =
    removesOnDarwin "a nix-darwin system with the runner and no collector"
      (t.darwinSystem { });
  "Not without the runner on nix-darwin" =
    removesOnDarwin "a nix-darwin system naming a collector without the runner"
      (
        t.darwinOf [
          t.outputs.darwinModules.default
          { services.boomdrive.claude-code.telemetry.collector = collector; }
        ]
      );
  "The transcripts document on NixOS" =
    transcriptsOnNixos "a NixOS system with the runner, Flume on by default and no collector"
      (t.nixosSystem { });
  "No Flume on NixOS" = noTranscriptsOnNixos "a NixOS system with the runner and Flume off" (
    t.nixosSystem { services.flume.enable = false; }
  );
  "Flume without the runner on NixOS" =
    transcriptsOnNixos "a NixOS system with Flume on and the runner off"
      (
        t.nixosOf [
          t.outputs.nixosModules.default
          {
            services.flume = {
              enable = true;
              user = "alice";
            };
          }
        ]
      );
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
