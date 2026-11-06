# The flake's outputs, as flake.nix gives them called with the lock's
# inputs.
t:
let
  inherit (t) lib outputs systems;
  given = builtins.attrNames outputs.packages;
in
{
  "The flake's outputs" =
    t.expect (outputs ? packages) "the outputs give no packages"
    ++ t.expect (
      given == lib.sort lib.lessThan systems
    ) "packages gives ${toString given}, not ${toString systems}";
}
