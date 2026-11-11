# What the Boomdrive module gives Flume's own module of each drive of this
# host (the change each-repository-is-named-once-by-its-owner, design
# Decisions 3 to 6; record 0028): with the runner enabled, the drive's
# stream, `services.flume.settings.streams.<stream>`, its room, its
# priority where the drive sets one, and each label's reserve; and each
# address the drive keeps, `services.flume.settings.repos."<owner>/<name>"`,
# the drive's stream, by its address and never a folder (flume record
# 0055). A drive of this host names no `machine`, or names
# services.crew.host; one of another host gives Flume nothing. Each value
# is given by `lib.mkDefault`, so a host's own value in
# services.flume.settings wins, and a key of a stream the module does not
# give, set there, sits beside the module's; where no drive of this host is
# named, Flume is given no `streams` and no `repos` at all. Nothing else:
# the module writes no file of Flume's, which Flume's own module renders
# from its `settings` (core.18, core.46), and a drive's stream is never a
# key of the runner's file (core.5).
{
  config,
  lib,
  options,
  ...
}:
let
  cfg = config.services.boomdrive;
  inherit (import ./accounts.nix { inherit lib; }) drivesOf hereOf;
  drives = drivesOf cfg.accounts;

  # This host, as crew names it, whether or not crew is on; null where it
  # is not set (design Decision 5).
  host = if options.services.crew.host.isDefined then config.services.crew.host else null;

  # The drives of this host: those that name no machine, or this one. A
  # drive of another machine gives Flume nothing, and the runner's file
  # still keeps its repositories, to read.
  here = hereOf host drives;

  # The drives that name a machine where this host has no name to compare.
  unplaced = lib.filter (drive: drive.machine != null && host == null) drives;

  # Each stream given by more than one drive of this host, by its drives
  # (design Decision 6).
  streamsTwice = lib.filterAttrs (_: given: lib.length given > 1) (
    lib.mapAttrs (_: map (drive: drive.name)) (lib.groupBy (drive: drive.stream.name) here)
  );

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
    assertions = [
      {
        assertion = unplaced == [ ];
        message = "services.boomdrive.accounts names a machine at ${
          lib.concatMapStringsSep ", " (
            drive: "accounts.${drive.account}.drives.${drive.name}.machine"
          ) unplaced
        }, and services.crew.host, this host's name to compare it with, is not set: set services.crew.host, as `hostname -s` names this host, whether or not crew runs here.";
      }
      {
        assertion = streamsTwice == { };
        message = "services.boomdrive.accounts gives Flume ${
          lib.concatStringsSep "; " (
            lib.mapAttrsToList (
              stream: given: "the stream ${stream} from the drives ${lib.concatStringsSep " and " given}"
            ) streamsTwice
          )
        }: a stream is given by one drive of this host, so set another stream.name on all but one.";
      }
    ];

    services.flume.settings = {
      streams = lib.mkIf (here != [ ]) (lib.mkMerge (map streamOf here));
      repos = lib.mkIf (here != [ ]) (lib.mkMerge (map reposOf here));
    };
  };
}
