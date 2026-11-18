# What every case reads: flake.nix imported as an attribute set, and its
# outputs called with the lock's inputs and `self` as this tree, as
# flake-compat calls them (design Decision 14).
{ paths }:
let
  inherit ((import ../inputs.nix { inherit paths; })) inputs;
  inherit (inputs.nixpkgs) lib;
  flake = import ../../flake.nix;
  lock = builtins.fromJSON (builtins.readFile ../../flake.lock);
  outputs = flake.outputs (inputs // { inherit self; });
  # The tree by its path, never copied: the checks write nothing.
  self = outputs // {
    outPath = toString ../..;
    inherit inputs outputs;
    _type = "flake";
  };

  # A nix-darwin system of aarch64-darwin whose primary user is alice, at
  # /Users/alice (Task notes 5.1), of the modules `given`.
  darwinOf =
    given:
    inputs.nix-darwin.lib.darwinSystem {
      modules = [
        {
          nixpkgs.hostPlatform = "aarch64-darwin";
          system.primaryUser = "alice";
          users.users.alice.home = "/Users/alice";
          system.stateVersion = 6;
        }
      ]
      ++ given;
    };

  # A NixOS system of x86_64-linux whose normal users are alice and bob
  # (Task notes 6.1), of the modules `given`.
  nixosOf =
    given:
    inputs.nixpkgs.lib.nixosSystem {
      modules = [
        {
          nixpkgs.hostPlatform = "x86_64-linux";
          users.users.alice.isNormalUser = true;
          users.users.bob.isNormalUser = true;
          boot.loader.grub.enable = false;
          fileSystems."/" = {
            device = "/dev/disk/by-label/nixos";
            fsType = "ext4";
          };
          system.stateVersion = "26.11";
        }
      ]
      ++ given;
    };

  # Flume's, sluice's, crew's and timber's own modules alone, each from its
  # own flake, turned on with the set's packages, crew's settings for host-a,
  # Flume's terminal and window as the Boomdrive module gives them
  # (apps.nix), crew's source declared to Flume as sources.nix declares it
  # (the change the-set-declares-crews-source-to-flume, design Decision 6),
  # and on NixOS the user alice (design Decision 8; the change
  # the-set-takes-timbers-own-module, design Decision 8).
  settings = import ./settings.nix;
  toolsOf =
    platform: set: extra:
    map (tool: inputs.${tool}."${platform}Modules".${tool}) [
      "flume"
      "sluice"
      "crew"
      "timber"
    ]
    ++ [
      {
        services = {
          flume = {
            enable = true;
            package = set.flume;
            settings.host = {
              multiplexer = "herdr";
              terminal_app = "ghostty";
            };
            sources.crew = {
              kinds_dir = "~/.local/share/flume/kinds/crew";
              roles = "~/.local/state/crew/flume/roles.toml";
            };
          };
          sluice = {
            enable = true;
            package = set.sluice;
          };
          crew = {
            enable = true;
            package = set.crew;
            host = "host-a";
            settings = settings.crewModule;
          };
          timber = {
            enable = true;
            package = set.timber;
          };
        };
      }
      extra
    ];

  # A system of darwinOf's and one of nixosOf's of the tools' own modules
  # alone, each with the module `extra` of more settings beside them (the
  # change the-hosts-collector-reaches-sluice-and-flume, Task notes 1.5).
  toolsDarwinOf = extra: darwinOf (toolsOf "darwin" outputs.packages.aarch64-darwin extra);
  toolsNixosOf =
    extra:
    nixosOf (
      toolsOf "nixos" outputs.packages.x86_64-linux {
        imports = [ extra ];
        services = {
          flume.user = "alice";
          sluice.user = "alice";
          crew.user = "alice";
          timber.user = "alice";
        };
      }
    );
in
{
  inherit
    darwinOf
    flake
    inputs
    lib
    nixosOf
    outputs
    self
    ;

  # flake.lock, which the inputs are read from.
  inherit lock;

  # The version of a tool of the set, its tag in the lock without the v.
  lockedVersion =
    name:
    inputs.nixpkgs.lib.removePrefix "v"
      lock.nodes.${lock.nodes.${lock.root}.inputs.${name}}.original.ref;

  # The systems the flake gives: sluice's flake's, less x86_64-darwin,
  # which nixpkgs 26.11 refuses.
  systems = [
    "aarch64-darwin"
    "aarch64-linux"
    "x86_64-linux"
  ];

  # A failure where `holds` is false, as a list of what fails.
  expect = holds: what: if holds then [ ] else [ what ];

  # A system of darwinOf's with the flake's module enabled for alice, and
  # the module `given`.
  darwinSystem =
    given:
    darwinOf [
      outputs.darwinModules.default
      { services.boomdrive.enable = true; }
      given
    ];

  # A system of nixosOf's with the flake's module enabled for alice, and
  # the module `given`.
  nixosSystem =
    given:
    nixosOf [
      outputs.nixosModules.default
      {
        services.boomdrive = {
          enable = true;
          user = "alice";
        };
      }
      given
    ];

  # The tools' own systems, of more settings and of none, beside the
  # runner's systems above.
  inherit toolsDarwinOf toolsNixosOf;
  toolsDarwinSystem = toolsDarwinOf { };
  toolsNixosSystem = toolsNixosOf { };

  # The messages of the assertions a system fails.
  failures =
    evaluated: map (a: a.message) (builtins.filter (a: !a.assertion) evaluated.config.assertions);
}
