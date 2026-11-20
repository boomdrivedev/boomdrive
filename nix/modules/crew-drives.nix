# What the Boomdrive module gives crew's own module of each drive crew's
# settings set up (the change crew-runs-only-the-drives-set-up-for-it,
# design Decision 1; record 0032 as amended on 2026-11-18): with crew's
# module on, whether or not the runner is, each drive the host sets up in
# `services.crew.settings.drives.<drive>`, under an account of the host's
# accounts that names its crew state repository, this host's and others',
# is given its `machine`, the drive's, or services.crew.host where the
# drive names none, and its `state`, the account's `state_repository`, and
# no other key of crew's drive. A drive of the accounts that crew's
# settings do not set up is given nothing, so crew has no drive of its
# name; a drive set up under an account that names no state repository is
# given nothing, its machine and state crew's settings' own.
#
# The module adds to the type of crew's drives rather than defining
# crew's drives: it declares services.crew.settings again, whose `drives`
# is an attrsOf of a submodule holding the giving module, which the module
# system merges into crew's own declaration, so each drive crew's settings
# name is evaluated with it and no drive is named by the set. Defining
# crew's drives would name every drive the accounts give, and reading
# crew's drives' names to choose which to define would recurse. The giving
# module's `config` names `machine` and `state` itself, each under a
# `lib.mkIf`, since the module system reads a module's attribute names
# before its arguments, and `name` comes from them.
#
# crew's module takes each as written once, by the host's settings or by
# another module that gives it by the drive's label (crew's spec, "Another
# module gives a drive its machine and its state repository"), so both are
# given at the options' own priority, and a host that also writes either
# in services.crew.settings fails evaluation by crew's own check. `_file`
# names services.boomdrive.accounts, the option where the fix is, in place
# of this file in the store, so crew's failure lists it among the places
# that write the value. Nothing else: crew's own keys of a drive, its hosts
# and its runs stay the host's (core.57), and crew's module renders its
# teams file (core.18, core.46).
{ config, lib, ... }:
let
  inherit (import ./accounts.nix { inherit lib; }) drivesOf crewDrivesOf;
  crew = config.services.crew;
  given = crewDrivesOf crew.host (drivesOf config.services.boomdrive.accounts);
in
{
  _file = "services.boomdrive.accounts";
  options.services.crew.settings = lib.mkOption {
    type = lib.types.submodule {
      options.drives = lib.mkOption {
        type = lib.types.attrsOf (
          lib.types.submodule (
            { name, ... }:
            let
              on = crew.enable && given ? ${name};
            in
            {
              _file = "services.boomdrive.accounts";
              config = {
                machine = lib.mkIf on given.${name}.machine;
                state = lib.mkIf on given.${name}.state;
              };
            }
          )
        );
      };
    };
  };
}
