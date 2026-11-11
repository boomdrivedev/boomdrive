# The service-modules spec's cases on crew's kept repositories (the change
# the-set-gives-crew-each-kept-repositorys-folders, design Decisions 1 to
# 7): the rule of nix/modules/view-dir.nix held to each row of
# nix/tests/view-dirs.json, which the runner's own test holds its
# `config::view_path` to.
t:
let
  inherit (t) lib;
  viewDirOf = import ../modules/view-dir.nix { inherit lib; };
  rows = builtins.fromJSON (builtins.readFile ./view-dirs.json);

  # A row as its failure names it.
  named =
    row:
    "home ${builtins.toJSON row.home}, repository_root ${builtins.toJSON row.repository_root} and address ${builtins.toJSON row.address}";
  # The row's rule, read: `success` and `value` as builtins.tryEval gives
  # them.
  readOf =
    row:
    builtins.tryEval (viewDirOf {
      inherit (row) home address;
      root = row.repository_root;
    });
in
{
  "The runner's own view rule" = lib.concatMap (
    row:
    let
      read = readOf row;
      gave = if read.success then read.value else null;
    in
    t.expect (gave == row.folder) "${named row} give ${builtins.toJSON gave}, not ${row.folder}"
  ) (lib.filter (row: row.folder != null) rows);

  "A repository root the runner refuses" = lib.concatMap (
    row:
    let
      read = readOf row;
    in
    t.expect (
      !read.success
    ) "${named row} give ${builtins.toJSON read.value}, where the runner refuses the root"
  ) (lib.filter (row: row.folder == null) rows);
}
