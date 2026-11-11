# The service-modules spec's cases on each kept repository's clone folder
# (the change the-set-states-the-runners-folders, design Decisions 1 to
# 6): the rule of nix/modules/clone-dir.nix held to each row of
# nix/tests/clone-dirs.json, which the runner's own test holds its
# `clone::container` to.
t:
let
  inherit (t) lib;
  cloneDirOf = import ../modules/clone-dir.nix;
  rows = builtins.fromJSON (builtins.readFile ./clone-dirs.json);
in
{
  "The runner's own clone rule" = lib.concatMap (
    row:
    let
      gave = cloneDirOf { inherit (row) dataDir address; };
    in
    t.expect (gave == row.folder)
      "dataDir ${builtins.toJSON row.dataDir} and address ${builtins.toJSON row.address} give ${builtins.toJSON gave}, not ${builtins.toJSON row.folder}"
  ) rows;
}
