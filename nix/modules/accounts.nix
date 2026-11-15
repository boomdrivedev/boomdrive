# What services.boomdrive.accounts gives the others (the change
# each-repository-is-named-once-by-its-owner, design Decision 1; record
# 0028), the one place that reads it, so both platforms render one text.
# `drivesOf` lists each drive, in the accounts' and then the drives' name
# order, as `{ account, name, machine, stream, state_repository, addresses }`,
# each carrying its account's crew state repository, null where the
# account names none (record 0032), its addresses
# `<owner>/<name>`, owner by owner in the order of the owners' names, each
# owner's names in the order given: nix keeps no order of an attribute
# set's names, so the owners' names give it. `hereOf host` keeps the
# drives of this host, those that name no machine or name `host`, which
# Flume's streams and the launcher's entries are both made from (design
# Decision 5 of the change each-drive-opens-from-the-launcher).
# `crewDrivesOf host drives` gives crew's drives, by the drive's name, each
# drive under an account that names its crew state repository as
# `{ machine, state }`: its machine, or `host` where it names none, and its
# account's state repository, which crew-drives.nix gives crew's module
# (record 0032). `fileOf` is
# the table the runner's file is rendered from: `settings`, with `accounts`
# from `accounts`, every account's `drives` and every drive's
# `repositories` always written. A drive's stream and machine are the modules' options,
# never keys of the runner's file (core.5).
{ lib }:
let
  addressesOf =
    drive:
    lib.concatLists (lib.mapAttrsToList (owner: map (name: "${owner}/${name}")) drive.repositories);
in
{
  drivesOf =
    accounts:
    lib.concatLists (
      lib.mapAttrsToList (
        account:
        {
          drives,
          state_repository ? null,
          ...
        }:
        lib.mapAttrsToList (name: drive: {
          inherit account name state_repository;
          inherit (drive) machine stream;
          addresses = addressesOf drive;
        }) drives
      ) accounts
    );

  # crew's drives by their names, each holding its machine, `host` where
  # the drive names none, and its account's crew state repository as its
  # `state`, of the drives under an account that names one (record 0032).
  crewDrivesOf =
    host: drives:
    lib.listToAttrs (
      map (
        drive:
        lib.nameValuePair drive.name {
          machine = if drive.machine == null then host else drive.machine;
          state = drive.state_repository;
        }
      ) (lib.filter (drive: drive.state_repository != null) drives)
    );

  # The drives of this host: those that name no machine, or `host`, this
  # host's name as services.crew.host gives it, null where it is not set.
  hereOf = host: lib.filter (drive: drive.machine == null || drive.machine == host);

  fileOf =
    cfg:
    cfg.settings
    // {
      accounts = lib.mapAttrs (_: account: {
        drives = lib.mapAttrs (_: drive: { repositories = addressesOf drive; }) account.drives;
      }) cfg.accounts;
    };
}
