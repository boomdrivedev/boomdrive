# What services.boomdrive.accounts gives the others (the change
# each-repository-is-named-once-by-its-owner, design Decision 1; record
# 0028), the one place that reads it, so both platforms render one text.
# `drivesOf` lists each drive, in the accounts' and then the drives' name
# order, as `{ account, name, machine, stream, addresses }`, its addresses
# `<owner>/<name>`, owner by owner in the order of the owners' names, each
# owner's names in the order given: nix keeps no order of an attribute
# set's names, so the owners' names give it. `hereOf host` keeps the
# drives of this host, those that name no machine or name `host`, which
# Flume's streams and the launcher's entries are both made from (design
# Decision 5 of the change each-drive-opens-from-the-launcher). `fileOf` is
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
        { drives, ... }:
        lib.mapAttrsToList (name: drive: {
          inherit account name;
          inherit (drive) machine stream;
          addresses = addressesOf drive;
        }) drives
      ) accounts
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
