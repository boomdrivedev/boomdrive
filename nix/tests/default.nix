# The nix project's cases, which nix/check.sh evaluates against no store
# (design Decision 14): `typecheck` and `test`, one for each task, read
# the flake's inputs from BOOMDRIVE_FLAKE_INPUTS, the devenv shell's JSON
# of their store paths. Each case is a list of what fails, empty where it
# holds, so one failing case never stops the rest. A check's result is one
# line for each failure, `nix: <case>: <what>`.
let
  t = import ./lib.nix { paths = builtins.fromJSON (builtins.getEnv "BOOMDRIVE_FLAKE_INPUTS"); };
  inherit (t.lib) concatLists concatStringsSep mapAttrsToList;

  lines =
    cases:
    concatStringsSep "\n" (
      concatLists (mapAttrsToList (case: map (what: "nix: ${case}: ${what}")) cases)
    );
in
{
  typecheck = lines (import ./typecheck.nix t);
  test = lines (
    import ./outputs.nix t
    // import ./packages.nix t
    // import ./companions.nix t
    // import ./file.nix t
    // import ./darwin.nix t
    // import ./nixos.nix t
    // import ./tools.nix t
    // import ./drives.nix t
    // import ./cache.nix t
    // import ./workflow.nix t
    // import ./apps.nix t
    // import ./way-in.nix t
  );
}
