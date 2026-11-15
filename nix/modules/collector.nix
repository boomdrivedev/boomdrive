# What the Boomdrive module gives Flume's and sluice's own modules of the
# host's collector (the change the-hosts-collector-reaches-sluice-and-flume,
# design Decisions 1 to 5; core.57; record 0013 as amended on 2026-11-15):
# with the runner on and `services.boomdrive.claude-code.telemetry.collector`
# named, `services.flume.settings.telemetry` and
# `services.sluice.settings.telemetry` each `send = true` and `collector`,
# the host's collector with its one trailing `/` dropped (claude-code.nix's
# `baseOf`), so the sessions, Flume and sluice send to one
# `<base>/v1/<signal>`. Each key is given apart by `lib.mkDefault`, so a key
# a host sets in a tool's own `telemetry` merges beside them, and a host's
# `send = false` there turns that tool's telemetry off. No other key is
# given: which of its traces, metrics and logs each tool sends, and as
# what, stays that tool's own setting, and Claude Code's switches reach
# Claude Code's document alone. With the runner off or no collector named,
# neither tool's `telemetry` is given. Nothing else: the module writes no
# file of Flume's or sluice's, whose own modules write their `[telemetry]`
# from these keys (core.18, core.46).
{ config, lib, ... }:
let
  inherit (import ./claude-code.nix { inherit lib; }) baseOf;
  host = config.services.boomdrive.claude-code.telemetry.collector;

  # Each tool's keys, each apart at the module's default priority.
  telemetry = {
    send = lib.mkDefault true;
    collector = lib.mkDefault (baseOf host);
  };
in
{
  config = lib.mkIf (config.services.boomdrive.enable && host != null) {
    services.flume.settings.telemetry = telemetry;
    services.sluice.settings.telemetry = telemetry;
  };
}
