# The quick-start spec's evaluated cases (record 0043; site.8; design
# Decisions 4 and 8 of the change
# boomdrives-quick-start-takes-a-stranger-to-a-working-drive): each step's
# file read by its path under site/src/start, as the page shows it, and
# evaluated as it stands, never built.
t:
let
  inherit (t) lib;
  start = ../../site/src/start;

  # The address README's Installing with nix names, its first `github:`
  # in backticks, read as cache.nix reads README.
  address = lib.findFirst (lib.hasPrefix "github:") null (
    lib.splitString "`" (toString t.installing)
  );

  # A shell file's words, split at every run of white space.
  wordsOf = text: lib.filter (word: word != "") (lib.flatten (builtins.split "[[:space:]]+" text));

  # The Mac's install: one `nix profile add` of the set's six and
  # nixpkgs' three, in this order (design Decision 4).
  macInstall =
    let
      words = wordsOf (builtins.readFile (start + "/mac/install.sh"));
      installables = lib.drop 3 words;
      set = map (name: if name == null then address else "${address}#${name}") [
        null
        "flume"
        "sluice"
        "herdr"
        "herdr-plugins"
        "crew"
      ];
      nixpkgs = map (name: "nixpkgs#${name}") [
        "git"
        "gh"
        "nodejs"
      ];
      want = set ++ nixpkgs;
      packageOf =
        installable:
        let
          parts = lib.splitString "#" installable;
        in
        if lib.length parts == 1 then "default" else lib.elemAt parts 1;
      isPackage =
        installable:
        if lib.hasPrefix "nixpkgs#" installable then
          t.inputs.nixpkgs.legacyPackages.aarch64-darwin ? ${packageOf installable}
        else
          lib.hasPrefix "${address}" installable
          && t.outputs.packages.aarch64-darwin ? ${packageOf installable};
    in
    t.expect (address != null) "README's Installing with nix names no `github:` address"
    ++ t.expect (
      lib.take 3 words == [
        "nix"
        "profile"
        "add"
      ]
    ) "mac/install.sh is no `nix profile add`: ${toString words}"
    ++ t.expect (
      installables == want
    ) "mac/install.sh adds ${toString installables}, not ${toString want}"
    ++ lib.concatMap (
      installable:
      t.expect (isPackage installable) "mac/install.sh's ${installable} is no package of aarch64-darwin"
    ) installables;
  # NixOS's flake, its outputs called with a nixpkgs whose nixosSystem
  # gives back its arguments and a set whose module is a marker (design
  # Decision 8).
  nixosFlake =
    let
      flake = import (start + "/nixos/flake.nix");
      marker = {
        quickStart = "the set's nixosModules.default";
      };
      called = flake.outputs {
        nixpkgs.lib.nixosSystem = arguments: arguments;
        boomdrive.nixosModules.default = marker;
      };
      modules = called.nixosConfigurations.your-host.modules or null;
      want = [
        (start + "/nixos/configuration.nix")
        marker
        (start + "/nixos/boomdrive.nix")
      ];
      describe =
        module:
        if builtins.isPath module then
          toString module
        else if module == marker then
          marker.quickStart
        else
          "a module";
    in
    t.expect ((flake.inputs.boomdrive.url or null) == address)
      "nixos/flake.nix's boomdrive is ${flake.inputs.boomdrive.url or "none"}, not ${toString address}"
    ++ t.expect (
      !(flake.inputs.boomdrive ? inputs)
    ) "nixos/flake.nix makes boomdrive's inputs follow its own"
    ++ t.expect (flake.inputs ? nixpkgs) "nixos/flake.nix has no input nixpkgs"
    ++
      t.expect (modules == want)
        "nixos/flake.nix's your-host is of ${
          if builtins.isList modules then lib.concatMapStringsSep ", " describe modules else "no modules"
        }, not ${lib.concatMapStringsSep ", " describe want}";
  # NixOS's drive: a system of x86_64-linux whose normal user is you, of
  # the set's module and the drive step's file alone, as the host's flake
  # gives them (design Decision 8).
  render = import ../modules/toml.nix { inherit lib; };
  inherit (import ../modules/accounts.nix { inherit lib; }) fileOf;
  drive = t.nixosOf [
    { users.users.you.isNormalUser = true; }
    t.outputs.nixosModules.default
    (start + "/nixos/boomdrive.nix")
  ];
  nixosDrive =
    let
      inherit (drive) config;
      file = builtins.fromTOML (render (fileOf config.services.boomdrive));
      crewTable = file.control_plane.crew or null;
      crewDrive = lib.mapAttrs (_: drive: {
        inherit (drive) state machine blueprints;
      }) config.services.crew.settings.drives;
      flume = config.services.flume.settings;
      repository = "your-github-name/your-repository";
      # The drive's step as its file stands, called with no packages: what
      # it sets itself, which gives Flume nothing (record 0034, point 2;
      # record 0028).
      written = import (start + "/nixos/boomdrive.nix") {
        pkgs = {
          git = null;
          gh = null;
          nodejs = null;
        };
      };
      holdsEach =
        setting: wanted:
        t.expect (lib.all (value: builtins.elem value (config.nix.settings.${setting} or [ ])) wanted)
          "the drive's system's nix.settings.${setting} is ${
            toString (config.nix.settings.${setting} or [ ])
          }, not nixConfig's ${toString wanted}";
    in
    t.expect (t.failures drive == [ ]) "the drive's system fails: ${toString (t.failures drive)}"
    ++ t.expect (
      config.services.boomdrive.user == "you"
    ) "the runner's user is ${config.services.boomdrive.user}, not you"
    ++ t.expect (
      file.accounts == {
        me.drives.home.repositories = [ "your-github-name/your-repository" ];
      }
    ) "the runner's file's accounts are ${builtins.toJSON file.accounts}"
    ++ t.expect (
      (file.control_plane.type or null) == "crew"
    ) "the runner's file's control plane is no crew"
    ++ t.expect (
      crewTable == {
        host = "your-host";
        partitions = [
          {
            label = "home";
            state_repository = "your-github-name/drive-state";
          }
        ];
      }
    ) "the runner's file's crew table is ${builtins.toJSON crewTable}"
    ++ t.expect (
      crewDrive == {
        home = {
          state = "your-github-name/drive-state";
          machine = "your-host";
          blueprints = [ repository ];
        };
      }
    ) "crew's drives are ${builtins.toJSON crewDrive}"
    ++
      t.expect (!(written.services ? flume))
        "nixos/boomdrive.nix writes Flume's settings itself: ${builtins.toJSON (builtins.attrNames written.services.flume)}"
    ++
      t.expect ((flume.streams or null) != null && flume.streams ? home)
        "the set gives Flume no stream home: ${
          builtins.toJSON (lib.attrNames (if flume.streams == null then { } else flume.streams))
        }"
    ++
      t.expect ((flume.repos.${repository}.stream or null) == "home")
        "the set gives Flume ${repository}'s stream as ${
          toString (flume.repos.${repository}.stream or "none")
        }, not home"
    ++ t.expect (
      (config.services.flume.sources.crew or null) != null
    ) "the set gives Flume no source crew"
    ++
      t.expect ((config.services.flume.settings.host.multiplexer or null) == "herdr")
        "Flume's host multiplexer is ${
          toString (config.services.flume.settings.host.multiplexer or "none")
        }, not herdr"
    ++ holdsEach "extra-substituters" t.flake.nixConfig.extra-substituters
    ++ holdsEach "extra-trusted-public-keys" t.flake.nixConfig.extra-trusted-public-keys;
  # The Mac's source of crew's: its keys, read as TOML, are those the set's
  # module gives Flume as the source `crew` on the NixOS drive's system,
  # so the Mac's file and the module never drift (design Decision 8;
  # record 0034, point 2).
  macCrewSource =
    let
      file = builtins.fromTOML (builtins.readFile (start + "/mac/crew-source.toml"));
      module = drive.config.services.flume.sources.crew or null;
    in
    t.expect (file == module)
      "mac/crew-source.toml is ${builtins.toJSON file}, not the set's source crew ${builtins.toJSON module}";
  # NixOS's switch: the folder the flake step writes to and the
  # configuration it gives, and the drive's system's derivation, which
  # evaluates and is never built (design Decision 8; Risk 6). nixosOf's
  # base gives the system its root file system and its boot loader.
  nixosSwitch =
    let
      words = wordsOf (builtins.readFile (start + "/nixos/switch.sh"));
      want = [
        "sudo"
        "nixos-rebuild"
        "switch"
        "--flake"
        "/etc/nixos#your-host"
      ];
      toplevel = builtins.tryEval (builtins.seq drive.config.system.build.toplevel.drvPath true);
    in
    t.expect (words == want) "nixos/switch.sh is ${toString words}, not ${toString want}"
    ++ t.expect toplevel.success "the drive's system's toplevel does not evaluate to a derivation";
in
{
  "quick start: the Mac's install" = macInstall;
  "quick start: NixOS's flake" = nixosFlake;
  "quick start: NixOS's drive" = nixosDrive;
  "quick start: NixOS's switch" = nixosSwitch;
  "quick start: the Mac's crew source" = macCrewSource;
}
