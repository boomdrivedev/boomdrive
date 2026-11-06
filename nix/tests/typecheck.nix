# nix:typecheck: everything the flake gives evaluates (design Decision 14).
t:
let
  inherit (t) lib outputs systems;
  inherit (lib) concatLists mapAttrsToList;

  # A package's failures: none where it is a derivation whose path
  # evaluates.
  evaluates =
    system: name: package:
    let
      drvPath = builtins.tryEval (builtins.seq package.drvPath true);
    in
    if !lib.isDerivation package then
      [ "packages.${system}.${name} is not a derivation" ]
    else
      t.expect drvPath.success "packages.${system}.${name} does not evaluate to a derivation";
in
{
  "Every package evaluates" = concatLists (
    map (
      system:
      if outputs.packages ? ${system} then
        concatLists (mapAttrsToList (evaluates system) outputs.packages.${system})
      else
        [ "packages gives no ${system}" ]
    ) systems
  );
}
