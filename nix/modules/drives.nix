# What the Boomdrive module gives Flume's own module of each drive of this
# host (the change each-repository-is-named-once-by-its-owner, design
# Decisions 3 and 4; record 0028): with the runner enabled, the drive's
# stream, `services.flume.settings.streams.<stream>`, its room, its
# priority where the drive sets one, and each label's reserve; and each
# address the drive keeps, `services.flume.settings.repos."<owner>/<name>"`,
# the drive's stream, by its address and never a folder (flume record
# 0055). Each value is given by `lib.mkDefault`, so a host's own value in
# services.flume.settings wins, and a key of a stream the module does not
# give, set there, sits beside the module's; where no drive of this host is
# named, Flume is given no `streams` and no `repos` at all. Nothing else:
# the module writes no file of Flume's, which Flume's own module renders
# from its `settings` (core.18, core.46), and a drive's stream is never a
# key of the runner's file (core.5).
{
  config,
  lib,
  ...
}:
let
  cfg = config.services.boomdrive;
  inherit (import ./accounts.nix { inherit lib; }) drivesOf;

  # The drives of this host.
  here = lib.filter (drive: drive.machine == null) (drivesOf cfg.accounts);

  # A drive's stream, as Flume's `[streams.<stream>]` names its keys.
  streamOf =
    { stream, ... }:
    {
      ${stream.name} = {
        agents = lib.mkDefault stream.agents;
        labels = lib.mapAttrs (_: label: { reserve = lib.mkDefault label.reserve; }) stream.labels;
      }
      // lib.optionalAttrs (stream.priority != null) { priority = lib.mkDefault stream.priority; };
    };

  # Each address a drive keeps, as Flume's `[repos."<owner>/<name>"]` names
  # it, given the drive's stream: by its address, never a folder (flume
  # record 0055).
  reposOf =
    { stream, addresses, ... }:
    lib.genAttrs addresses (_: {
      stream = lib.mkDefault stream.name;
    });
in
{
  config = lib.mkIf cfg.enable {
    services.flume.settings = {
      streams = lib.mkIf (here != [ ]) (lib.mkMerge (map streamOf here));
      repos = lib.mkIf (here != [ ]) (lib.mkMerge (map reposOf here));
    };
  };
}
