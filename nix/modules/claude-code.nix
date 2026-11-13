# Claude Code's sessions' telemetry (record 0013 as amended on 2026-11-12,
# the set's sessions in Claude Code's managed settings; design Decisions 1
# to 6 of the change every-claude-code-session-exports-to-the-hosts-collector):
# `services.boomdrive.claude-code.telemetry` names the collector on the
# host, after sluice's `[telemetry]`, and which of metrics, logs and traces
# Claude Code's sessions export to it. `documentOf` makes, from the option,
# the one document of Claude Code's managed settings the modules write,
# `10-boomdrive-telemetry.json` in the platform's `managed-settings.d`,
# which every session on the host reads whatever its account and home:
# an `env` of telemetry keys alone, each signal on sent to the collector by
# its own endpoint and each signal off pinned `none`, no general endpoint,
# header or resource attribute, every content setting pinned off, and
# Claude Code's own session, account and repository attributes pinned on.
# darwin.nix writes it by a root step of the activation and nixos.nix by
# `/etc`, each only with the runner on and a collector named; this file
# writes nothing.
{ lib }:
let
  inherit (lib) mkOption types;

  # The signals, each by the name OTLP's paths give it.
  signals = [
    "metrics"
    "logs"
    "traces"
  ];

  # `http://`, a host of the machine's own loopback, a port, and at most a
  # path: no user, query or fragment (Task notes 1.1).
  collector = types.strMatching "http://(localhost|127\\.0\\.0\\.1|\\[::1]):[0-9]{1,5}(/[A-Za-z0-9._~/-]*)?";

  # The document of `telemetry`, the option's value with a collector named.
  documentOf =
    telemetry:
    let
      base = lib.removeSuffix "/" telemetry.collector;
      upper = lib.toUpper;
      on = lib.filter (signal: telemetry.${signal}) signals;
    in
    {
      env = {
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
      }
      // lib.listToAttrs (
        map (signal: {
          name = "OTEL_${upper signal}_EXPORTER";
          value = if telemetry.${signal} then "otlp" else "none";
        }) signals
      )
      // lib.listToAttrs (
        lib.concatMap (signal: [
          {
            name = "OTEL_EXPORTER_OTLP_${upper signal}_ENDPOINT";
            value = "${base}/v1/${signal}";
          }
          {
            name = "OTEL_EXPORTER_OTLP_${upper signal}_PROTOCOL";
            value = "http/protobuf";
          }
        ]) on
      )
      // lib.optionalAttrs telemetry.traces { CLAUDE_CODE_ENHANCED_TELEMETRY_BETA = "1"; };
    };

  signal =
    default: description:
    mkOption {
      type = types.bool;
      inherit default description;
    };
in
{
  inherit documentOf;

  # The document's name, and the platforms' managed-settings folders.
  name = "10-boomdrive-telemetry.json";
  darwinFolder = "/Library/Application Support/ClaudeCode/managed-settings.d";
  nixosFolder = "claude-code/managed-settings.d";

  module =
    { config, ... }:
    let
      cfg = config.services.boomdrive.claude-code.telemetry;
    in
    {
      _file = ./claude-code.nix;

      options.services.boomdrive.claude-code.telemetry = {
        collector = mkOption {
          type = types.nullOr collector;
          default = null;
          example = "http://localhost:4318";
          description = ''
            The collector every Claude Code session on the host exports its
            telemetry to, as sluice's `[telemetry]` names its own: an
            `http://` address whose host is the machine's own loopback,
            `localhost`, `127.0.0.1` or `[::1]`, with a port and at most a
            path, such as `"http://localhost:4318"`. Each signal on goes to
            its `/v1/<signal>`, OTLP over HTTP, protobuf. With the runner on,
            the modules write it into Claude Code's managed settings,
            `10-boomdrive-telemetry.json` in
            `/Library/Application Support/ClaudeCode/managed-settings.d` on
            nix-darwin and `/etc/claude-code/managed-settings.d` on NixOS,
            which every session reads whatever its account and home, with
            every content setting off. Where `null`, nothing is written.
          '';
        };
        metrics = signal true "Whether Claude Code's sessions export their metrics to `collector`.";
        logs = signal true "Whether Claude Code's sessions export their logs, its events, to `collector`.";
        traces = signal false "Whether Claude Code's sessions export their traces to `collector`: Claude Code's beta, which the document turns on with `CLAUDE_CODE_ENHANCED_TELEMETRY_BETA`, off unless named.";
      };

      config.assertions = [
        {
          assertion = cfg.collector == null || lib.any (signal: cfg.${signal}) signals;
          message = "services.boomdrive.claude-code.telemetry names the collector ${toString cfg.collector} and turns metrics, logs and traces off: name metrics, logs or traces, or no collector.";
        }
      ];
    };
}
