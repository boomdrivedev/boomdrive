# Each kept repository's clone folder, which both platform modules import
# beside drives.nix (the change the-set-states-the-runners-folders, design
# Decisions 1 to 4; record 0036 as amended on 2026-11-11):
# `services.boomdrive.cloneDirs`, read-only and declared whether or not
# the runner is enabled, one entry for each address of every drive of
# every account in `services.boomdrive.accounts`, another machine's drives
# among them, keyed by the address as written. Its value is the runner's
# clone folder by the runner's own rule (clone-dir.nix) under
# `services.boomdrive.dataDir`: the folder `flume launch --repository`
# names, whose `.git` file names its bare repository. Its names are the
# accounts' alone, so reading them forces no folder, and each folder
# fails where `dataDir` does. Whatever names a clone's folder reads it,
# and no host sets it: a host's value beside the default fails
# evaluation, as nixpkgs fails a read-only option set twice.
{ config, lib, ... }:
let
  cfg = config.services.boomdrive;
  inherit (import ./accounts.nix { inherit lib; }) drivesOf;
  cloneDirOf = import ./clone-dir.nix;
  addresses = lib.concatMap (drive: drive.addresses) (drivesOf cfg.accounts);
in
{
  options.services.boomdrive.cloneDirs = lib.mkOption {
    type = lib.types.attrsOf lib.types.path;
    readOnly = true;
    default = lib.listToAttrs (
      map (
        address:
        lib.nameValuePair address (cloneDirOf {
          inherit (cfg) dataDir;
          inherit address;
        })
      ) addresses
    );
    defaultText = lib.literalMD ''
      each address of `services.boomdrive.accounts`, every drive's of every
      account, with its folder
      `<services.boomdrive.dataDir>/repositories/<owner>/<name>`
    '';
    description = ''
      The runner's clone of each repository the set names, keyed by its
      address as written, `<owner>/<name>`: the folder `flume launch
      --repository` names, `repositories/<owner>/<name>` under
      `services.boomdrive.dataDir`, by the runner's own rule. It holds every
      address of every drive of every account in `services.boomdrive.accounts`,
      another machine's drives among them, and no repository only the
      runner's local layer names. It is read-only: a host reads it wherever
      it names a clone's folder, and sets it nowhere. git run in the folder
      finds the bare repository by the folder's own `.git` file.
    '';
  };
}
