# crew's kept repositories on a runner's host, `services.crew.settings.repositories`
# (the change the-set-gives-crew-each-kept-repositorys-folders, design
# Decision 2; record 0036, point 1; crew record 0061): one entry for each
# address of `cloneDirs`, every drive's of every account, another
# machine's drives among them, its `dir` the runner's clone, read from
# `cloneDirs` and never joined again, and its `checkout` the runner's
# view, by the runner's own rule (view-dir.nix) under `home` and `root`,
# the runner's `repository_root` or null. darwin.nix and nixos.nix take it
# as `keptOf`, each with its own user's home.
{ lib }:
let
  viewDirOf = import ./view-dir.nix { inherit lib; };
in
{
  cloneDirs,
  home,
  root,
}:
lib.mapAttrs (address: dir: {
  inherit dir;
  checkout = viewDirOf { inherit home root address; };
}) cloneDirs
