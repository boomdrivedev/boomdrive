# nix:typecheck: everything the flake gives evaluates, each module in a
# system of its platform, and each tag in flake.nix is the lock's (design
# Decision 14).
t:
let
  inherit (t) lib outputs systems;
  inherit (lib) concatLists mapAttrsToList;
  tags = import ./tags.nix { inherit lib; };

  # The family-packages spec's set, for each system.
  set = [
    "boomdrive"
    "boomdrive-launcher"
    "crew"
    "default"
    "flume"
    "flume-herdr-plugin"
    "ghostty"
    "herdr"
    "sluice"
    "sluice-herdr-plugin"
  ];

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

  # A system's packages: the set, each evaluating, `default` the runner's.
  givesSet =
    system:
    let
      packages = outputs.packages.${system};
      given = builtins.attrNames packages;
      evaluated = concatLists (mapAttrsToList (evaluates system) packages);
    in
    t.expect (given == set) "packages.${system} gives ${toString given}, not ${toString set}"
    ++ evaluated
    ++ lib.optionals (evaluated == [ ] && packages ? default && packages ? boomdrive) (
      t.expect (
        packages.default.drvPath == packages.boomdrive.drvPath
      ) "packages.${system}.default is not packages.${system}.boomdrive"
    );

  # The host's own tools, which the set holds none of: git alone, since
  # herdr joined the set (records 0012 and 0023).
  hosts = [ "git" ];
  ownedBy =
    system: name: package:
    lib.concatMap (
      tool:
      t.expect (
        name != tool && (package.pname or null) != tool && (package.meta.mainProgram or null) != tool
      ) "packages.${system}.${name} is the host's own ${tool}"
    ) hosts;
in
{
  "Every system's packages" =
    t.expect (
      builtins.attrNames outputs.packages == lib.sort lib.lessThan systems
    ) "packages gives ${toString (builtins.attrNames outputs.packages)}, not ${toString systems}"
    ++ lib.concatMap (
      system: if outputs.packages ? ${system} then givesSet system else [ "packages gives no ${system}" ]
    ) systems;

  "No tool of the host's own" = lib.concatMap (
    system: concatLists (mapAttrsToList (ownedBy system) (outputs.packages.${system} or { }))
  ) systems;

  "Each tag is the lock's" = tags t.flake.inputs t.lock;

  # Each system's shell, whose derivation's path evaluates.
  "Every system's shell" = lib.concatMap (
    system:
    t.expect (builtins.tryEval (builtins.seq outputs.devShells.${system}.default.drvPath true)).success
      "devShells.${system}.default does not evaluate to a derivation"
  ) systems;

  # The nix-darwin module in a minimal system of its platform, whose
  # derivation throws on a failed assertion.
  "The nix-darwin module" =
    t.expect (builtins.tryEval (t.darwinSystem { }).system.drvPath).success
      "a nix-darwin system with services.boomdrive enabled does not evaluate";

  # The NixOS module in a minimal system of its platform, whose toplevel
  # throws on a failed assertion.
  "The NixOS module" =
    t.expect (builtins.tryEval (t.nixosSystem { }).config.system.build.toplevel.drvPath).success
      "a NixOS system with services.boomdrive enabled does not evaluate";
}
