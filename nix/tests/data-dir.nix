# The service-modules spec's cases on the runner's data folder (the change
# the-set-names-the-runners-data-folder, design Decisions 1 to 5): the
# rule of nix/modules/data-dir.nix held to each row of
# nix/tests/data-dirs.json, which the runner's own test holds its
# `data_dir` to.
t:
let
  inherit (t) lib;
  dataDirOf = import ../modules/data-dir.nix { inherit (t) lib; };
  rows = builtins.fromJSON (builtins.readFile ./data-dirs.json);
in
{
  "The runner's own rule" = lib.concatMap (
    row:
    let
      gave = dataDirOf {
        home = row.HOME;
        dataHome = row.XDG_DATA_HOME;
      };
    in
    t.expect (gave == row.folder)
      "HOME ${builtins.toJSON row.HOME} and XDG_DATA_HOME ${builtins.toJSON row.XDG_DATA_HOME} give ${builtins.toJSON gave}, not ${builtins.toJSON row.folder}"
  ) rows;
}
