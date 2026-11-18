# What the Boomdrive module gives crew's own module of each drive of the
# host's accounts (the change a-drives-machine-and-state-are-named-once,
# design Decisions 3 and 4; record 0032 as amended on 2026-11-15): with
# crew's module on, whether or not the runner is, each drive of every
# account that names its crew state repository, this host's and others',
# `services.crew.settings.drives.<drive>`, its `machine` the drive's, or
# services.crew.host where the drive names none, and its `state` the
# account's `state_repository`, and no other key of crew's drive; an
# account that names no state repository gives crew nothing. crew's module
# takes each as written once, by the host's settings or by another module
# that gives it by the drive's label (crew's spec, "Another module gives a
# drive its machine and its state repository"), so both are given at the
# options' own priority, and a host that also writes either in
# services.crew.settings fails evaluation by crew's own check. `_file`
# names services.boomdrive.accounts, the option where the fix is, in place
# of this file in the store, so crew's failure lists it among the places
# that write the value. Nothing else: crew's own keys of a drive, its hosts
# and its runs stay the host's (core.57), and crew's module renders its
# teams file (core.18, core.46).
{ config, lib, ... }:
let
  inherit (import ./accounts.nix { inherit lib; }) drivesOf crewDrivesOf;
  crew = config.services.crew;
in
{
  _file = "services.boomdrive.accounts";
  config = lib.mkIf crew.enable {
    services.crew.settings.drives = crewDrivesOf crew.host (
      drivesOf config.services.boomdrive.accounts
    );
  };
}
