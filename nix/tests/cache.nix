# The binary-cache spec over the flake and the modules (design Decision 4):
# `nixConfig` names the family's cache alone, the example a host copies to
# trust it, examples/trust.nix, gives the same two lines (record 0049),
# and each module, in a system whose
# `pkgs` carries an overlay of the host's own and with crew on, installs
# the flake's own packages and adds nothing of the cache's to the host's
# nix settings. Given a drive, the launcher it installs is the flake's own
# by its override, its compiled menu the flake's own (design Decision 5 of
# the change each-drive-opens-from-the-launcher).
t:
let
  inherit (t) lib;
  settings = import ./settings.nix;

  # The family's cache, as design Decision 4 names it.
  substituter = "https://boomdrive.cachix.org";
  key = "boomdrive.cachix.org-1:vHLgmasPg2jKqkvPXWOOrYzspoGL2xxsW4fAuHBtiag=";

  # The nix settings README shows a host to trust the cache with, whose
  # example is its own file (record 0049).
  trusting = (import ./examples/trust.nix).nix.settings;

  # The host's own packages under the set's names, so a module that took
  # its packages from the host's `pkgs` would install these.
  names = [
    "boomdrive"
    "boomdrive-launcher"
    "crew"
    "flume"
    "ghostty"
    "herdr"
    "pendingyou"
    "plannotator"
    "plannotator-tui"
    "portless"
    "rex"
    "sluice"
    "timber"
    "worktrunk"
  ];
  # The herdr plugins the modules link, in the order linked, Flume's,
  # sluice's, timber's and, with crew on, crew's companions' (design
  # Decision 6 of the change crews-companions-come-with-the-flake; design
  # Decision 7 of the change timber-comes-with-boomdrives-flake).
  plugins = [
    "flume-herdr-plugin"
    "sluice-herdr-plugin"
    "timber-herdr-plugin"
    "pendingyou-herdr-plugin"
    "plannotator-herdr-plugin"
  ];
  overlay = final: _: lib.genAttrs (names ++ plugins) (name: final.writeShellScriptBin name "");
  # The host's overlay, and crew turned on with its host, its drives and
  # its hosts.
  host = {
    nixpkgs.overlays = [ overlay ];
    services.crew = {
      enable = true;
      host = "host-a";
      settings = settings.crewModule;
    };
  };
  hosts = {
    "nix-darwin" = {
      system = t.darwinSystem;
      evaluated = t.darwinSystem host;
      set = t.outputs.packages.aarch64-darwin;
    };
    NixOS = {
      system = t.nixosSystem;
      evaluated = t.nixosSystem host;
      set = t.outputs.packages.x86_64-linux;
    };
  };

  profileOf = evaluated: map (p: p.outPath) evaluated.config.environment.systemPackages;

  ownNixpkgs =
    platform:
    { evaluated, set, ... }:
    let
      profile = profileOf evaluated;
      linked = map (p: p.outPath) evaluated.config.services.boomdrive.herdr.plugins;
      own = map (name: set.${name}.outPath) plugins;
    in
    t.expect (
      evaluated.pkgs ? boomdrive
    ) "the ${platform} system's pkgs carries no overlay of the host's own"
    ++ lib.concatMap (
      name:
      t.expect (builtins.elem set.${name}.outPath
        profile
      ) "the system profile holds no flake's own ${name}"
      ++ t.expect (
        !builtins.elem evaluated.pkgs.${name}.outPath profile
      ) "the system profile holds the host's pkgs' ${name}"
    ) names
    ++
      t.expect (linked == own)
        "the ${platform} system links the plugins ${toString linked}, not the flake's own ${toString plugins}";

  # The same system with the drive `home`: its launcher is the flake's own
  # made for `home`, and on nix-darwin its menu is the flake's own.
  drivesInLauncher =
    platform:
    { system, set, ... }:
    let
      evaluated = system {
        imports = [
          host
          { services.boomdrive.accounts.own.drives.home.repositories.agentplot = [ "flume" ]; }
        ];
      };
      installed = evaluated.config.services.boomdrive.launcher.package;
      made = set.boomdrive-launcher.override {
        drives = [
          {
            drive = "home";
            session = "home";
          }
        ];
      };
    in
    t.expect (
      t.failures evaluated == [ ]
    ) "the ${platform} system fails: ${toString (t.failures evaluated)}"
    ++ t.expect (
      installed.drvPath == made.drvPath
    ) "the ${platform} system's launcher is not the flake's own made for home"
    ++ t.expect (
      platform != "nix-darwin"
      || installed.parts.menu.outPath == set.boomdrive-launcher.parts.menu.outPath
    ) "the ${platform} system's launcher compiles a menu that is not the flake's own";

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

  # The example a host copies to trust the cache names the substituter and
  # key nixConfig names, and nothing else.
  "The host's trust example" =
    t.expect
      (
        trusting == {
          inherit (t.flake.nixConfig) extra-substituters extra-trusted-public-keys;
        }
      )
      "examples/trust.nix trusts ${builtins.toJSON trusting}, not nixConfig's ${builtins.toJSON t.flake.nixConfig}";
}
// onEach "A host's own nixpkgs" ownNixpkgs
// onEach "A host's drives in the launcher" drivesInLauncher
// onEach "The host's trust" trust
