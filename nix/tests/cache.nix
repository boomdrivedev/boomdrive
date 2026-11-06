# The binary-cache spec over the flake and the modules (design Decision 4):
# `nixConfig` names the family's cache alone, the README's install with
# nix gives a host the same two lines, and each module, in a system whose
# `pkgs` carries an overlay of the host's own, installs the flake's own
# packages and adds nothing of the cache's to the host's nix settings.
t:
let
  inherit (t) lib;

  # The family's cache, as design Decision 4 names it.
  substituter = "https://boomdrive.cachix.org";
  key = "boomdrive.cachix.org-1:vHLgmasPg2jKqkvPXWOOrYzspoGL2xxsW4fAuHBtiag=";

  # The README's **Installing with nix**, up to the next section, and each
  # nix block in it. A host copies the block that trusts the cache, so one
  # block holds each substituter and key nixConfig names, as nix strings.
  installing = lib.findFirst (lib.hasPrefix "Installing with nix\n") null (
    lib.splitString "\n## " (builtins.readFile ../../README.md)
  );
  nixBlocks = map (block: lib.head (lib.splitString "```" block)) (
    lib.drop 1 (lib.splitString "```nix\n" (toString installing))
  );
  named = t.flake.nixConfig.extra-substituters ++ t.flake.nixConfig.extra-trusted-public-keys;
  trusts = block: lib.all (value: lib.hasInfix ''"${value}"'' block) named;

  # The host's own packages under the set's names, so a module that took
  # its packages from the host's `pkgs` would install these.
  overlay =
    final: _: lib.genAttrs [ "boomdrive" "flume" "sluice" ] (name: final.writeShellScriptBin name "");
  hosts = {
    "nix-darwin" = {
      evaluated = t.darwinSystem { nixpkgs.overlays = [ overlay ]; };
      set = t.outputs.packages.aarch64-darwin;
    };
    NixOS = {
      evaluated = t.nixosSystem { nixpkgs.overlays = [ overlay ]; };
      set = t.outputs.packages.x86_64-linux;
    };
  };

  profileOf = evaluated: map (p: p.outPath) evaluated.config.environment.systemPackages;

  ownNixpkgs =
    platform:
    { evaluated, set }:
    let
      profile = profileOf evaluated;
    in
    t.expect (
      evaluated.pkgs ? boomdrive
    ) "the ${platform} system's pkgs carries no overlay of the host's own"
    ++
      lib.concatMap
        (
          name:
          t.expect (builtins.elem set.${name}.outPath
            profile
          ) "the system profile holds no flake's own ${name}"
          ++ t.expect (
            !builtins.elem evaluated.pkgs.${name}.outPath profile
          ) "the system profile holds the host's pkgs' ${name}"
        )
        [
          "boomdrive"
          "flume"
          "sluice"
        ];

  # Each of the host's nix settings that names a substituter or a key.
  trusted = [
    "substituters"
    "extra-substituters"
    "trusted-substituters"
    "trusted-public-keys"
    "extra-trusted-public-keys"
  ];
  trust =
    platform:
    { evaluated, ... }:
    let
      inherit (evaluated.config) nix;
      names = setting: lib.hasInfix "boomdrive.cachix.org" (toString setting);
    in
    lib.concatMap (
      setting:
      t.expect (!names (nix.settings.${setting} or [ ]))
        "the ${platform} system's nix.settings.${setting} names the family's cache: ${
          toString nix.settings.${setting}
        }"
    ) trusted
    ++ t.expect (
      !names (nix.extraOptions or "")
    ) "the ${platform} system's nix.extraOptions names the family's cache";

  onEach =
    scenario: check:
    lib.mapAttrs' (platform: host: {
      name = "${scenario} on ${platform}";
      value = check platform host;
    }) hosts;
in
{
  "The flake's nixConfig" = t.expect (
    t.flake.nixConfig == {
      extra-substituters = [ substituter ];
      extra-trusted-public-keys = [ key ];
    }
  ) "nixConfig is ${builtins.toJSON t.flake.nixConfig}, not ${substituter} and ${key} alone";

  "The README's lines" =
    t.expect (installing != null) "the README has no section ## Installing with nix"
    ++
      t.expect (installing == null || lib.any trusts nixBlocks)
        "no nix block of the README's Installing with nix names ${lib.concatStringsSep " and " named}, as nixConfig does";
}
// onEach "A host's own nixpkgs" ownNixpkgs
// onEach "The host's trust" trust
