# The flake's own packages for the host's system, which options.nix and
# tools.nix default to, so they are the paths the family's cache holds
# whatever nixpkgs the host runs (design Decision 4, core.47); a system
# the flake gives none for fails, naming those it gives.
{
  lib,
  packages,
  system,
}:
packages.${system}
  or (throw "the boomdrive flake gives no packages for ${system}: it gives ${lib.concatStringsSep ", " (builtins.attrNames packages)}")
