# The service-modules and family-packages specs' cases on millpond with
# the runner (record 0033; the change the-set-installs-millpond): the set
# gives millpond through its own flake at its release tag; the module
# installs it with the runner, renders millpond's host configuration from
# services.boomdrive.millpond.settings and links it at millpond's own path
# at each activation, and gives the runner's file the names of its inboxes,
# `inboxes`, on nix-darwin and on NixOS; with no inbox it writes no
# configuration and names none, and with no package it installs nothing.
t:
let
  inherit (t) inputs lib outputs;

  each = check: lib.concatMap (system: check system outputs.packages.${system}) t.systems;

  # Two inboxes, `home` and `work`, with the host and its secrets.
  settings = {
    host = "deneb";
    secrets = "/Users/alice/.local/share/millpond/secrets";
    inbox = [
      {
        name = "home";
        repository = "boomdrivedev/strategy";
      }
      {
        name = "work";
        repository = "/Users/alice/Code/work";
        branch = "signals";
      }
    ];
  };

  # The text the module renders for `settings`.
  expected = ''
    # Written by the boomdrive flake's module from services.boomdrive.millpond.settings; a change goes there.
    declarations = []
    host = "deneb"
    secrets = "/Users/alice/.local/share/millpond/secrets"

    [[inbox]]
    branch = "inbox"
    name = "home"
    repository = "boomdrivedev/strategy"

    [[inbox]]
    branch = "signals"
    name = "work"
    repository = "/Users/alice/Code/work"
  '';

  darwin = t.darwinSystem { services.boomdrive.millpond.settings = settings; };
  nixos = t.nixosSystem { services.boomdrive.millpond.settings = settings; };
  bare = t.darwinSystem { };
  none = t.darwinSystem {
    services.boomdrive.millpond = {
      package = null;
      inherit settings;
    };
  };

  cfgOf = system: system.config.services.boomdrive;
  textOf = system: (cfgOf system).millpond.text;
  darwinSteps =
    system:
    builtins.unsafeDiscardStringContext system.config.system.activationScripts.postActivation.text;
  nixosSteps =
    system: builtins.unsafeDiscardStringContext system.config.system.userActivationScripts.script;
  installs = system: package: builtins.elem package system.config.environment.systemPackages;
  millpondOf = system: outputs.packages.${system}.millpond;
in
{
  # millpond's own flake's package, its git crates fetched by the hashes
  # nix/packages/millpond-git-crates.nix holds, each at the revision
  # millpond's Cargo.lock names at the set's tag.
  "millpond through its own flake" =
    each (
      system: packages:
      let
        inherit (packages) millpond;
        own = inputs.millpond.packages.${system}.millpond;
        version = t.lockedVersion "millpond";
      in
      t.expect (
        millpond.pname == own.pname && toString millpond.src == toString own.src
      ) "packages.${system}.millpond is not built from millpond's flake's package"
      ++ t.expect (
        millpond.version == version
      ) "packages.${system}.millpond says ${millpond.version}, not its tag's ${version}"
    )
    ++ (
      let
        lock = builtins.fromTOML (builtins.readFile "${inputs.millpond}/Cargo.lock");
        git = builtins.filter (package: lib.hasPrefix "git+" (package.source or "")) lock.package;
        held = import ../packages/millpond-git-crates.nix;
        revOf = package: lib.last (lib.splitString "#" package.source);
        wrong = builtins.filter (
          package: (held."${package.name}-${package.version}".rev or null) != revOf package
        ) git;
      in
      t.expect (wrong == [ ] && builtins.length git == builtins.length (builtins.attrNames held))
        "millpond's git crates are not held at its Cargo.lock's revisions: ${
          toString (map (package: package.name) wrong)
        }"
    );

  "millpond's host configuration" =
    t.expect (textOf darwin == expected) "nix-darwin renders ${builtins.toJSON (textOf darwin)}"
    ++ t.expect (textOf nixos == expected) "NixOS renders ${builtins.toJSON (textOf nixos)}"
    ++ t.expect (
      (cfgOf darwin).settings.inboxes == [
        "home"
        "work"
      ]
    ) "nix-darwin gives the runner's inboxes ${builtins.toJSON (cfgOf darwin).settings.inboxes}"
    ++ t.expect (
      (cfgOf nixos).settings.inboxes == [
        "home"
        "work"
      ]
    ) "NixOS gives the runner's inboxes ${builtins.toJSON (cfgOf nixos).settings.inboxes}"
    ++ t.expect (installs darwin (millpondOf "aarch64-darwin")) "nix-darwin does not install the set's millpond"
    ++ t.expect (installs nixos (millpondOf "x86_64-linux")) "NixOS does not install the set's millpond"
    ++ t.expect (t.failures darwin == [ ]) "nix-darwin fails: ${toString (t.failures darwin)}"
    ++ t.expect (t.failures nixos == [ ]) "NixOS fails: ${toString (t.failures nixos)}";

  "millpond's host configuration linked at each activation" =
    let
      named = system: builtins.unsafeDiscardStringContext (toString (cfgOf system).millpond.file);
    in
    t.expect (lib.hasInfix "activate.sh millpond ${named darwin}" (darwinSteps darwin)) "nix-darwin's activation links no millpond configuration: ${darwinSteps darwin}"
    ++ t.expect (lib.hasInfix "activate.sh millpond ${named nixos}" (nixosSteps nixos)) "NixOS's activation links no millpond configuration: ${nixosSteps nixos}";

  "No inbox, no configuration" =
    t.expect (
      (cfgOf bare).millpond.file == null
    ) "a system naming no inbox renders millpond's configuration"
    ++ t.expect (
      !(lib.hasInfix "activate.sh millpond" (darwinSteps bare))
    ) "a system naming no inbox links millpond's configuration"
    ++ t.expect (
      !((cfgOf bare).settings ? inboxes) || (cfgOf bare).settings.inboxes == null
    ) "a system naming no inbox gives the runner inboxes"
    ++ t.expect (installs bare (millpondOf "aarch64-darwin")) "the runner's system does not install the set's millpond";

  "No millpond, nothing of it" =
    t.expect ((cfgOf none).millpond.file == null) "a system with no millpond renders its configuration"
    ++ t.expect (
      !(installs none (millpondOf "aarch64-darwin"))
    ) "a system with no millpond installs the set's"
    ++ t.expect (
      !((cfgOf none).settings ? inboxes) || (cfgOf none).settings.inboxes == null
    ) "a system with no millpond gives the runner inboxes";

  "An inbox named twice" =
    let
      twice = t.darwinSystem {
        services.boomdrive.millpond.settings.inbox = [
          {
            name = "home";
            repository = "a/b";
          }
          {
            name = "home";
            repository = "c/d";
          }
        ];
      };
    in
    t.expect (lib.any (lib.hasInfix "names an inbox more than once") (t.failures twice)) "an inbox named twice fails nothing: ${toString (t.failures twice)}";
}
